//! Native dropdowns: the preference that hands a menu's list to the OS.
//!
//! The sheet every dropdown draws is the kit's own;
//! [`MenuPresentation::Native`] keeps the trigger's DazzUI drawing and lets
//! the desktop OS draw the list instead — AppKit on macOS, Win32 on Windows,
//! GTK on Linux — through nativeapi. The preference scopes like a CSS custom
//! property, [`provide_menu_presentation`]; a nearer scope overrides the
//! root; changing it dismisses an open menu. The feature off, an unsupported
//! platform, or a backend that refuses to present all fall back to the drawn
//! sheet.
//!
//! The OS popup runs outside gpui's frames — AppKit inside its tracking
//! loop, Win32 inside `TrackPopupMenu`'s modal pump, GTK on a thread that
//! owns the GLib context no gpui app iterates. Its events therefore land in
//! a queue any thread may push to, and a drain task re-enters the window
//! through its handle to run them. AppKit and Win32 both report the close
//! before the chosen item's action, so a batch dispatches its clicks first
//! and the listeners outlive the close by a grace period — the same
//! accommodation the Flutter backend makes.
//!
//! Shortcut strings stay hints — display text in the native row, never an
//! accelerator registered with the OS. An item's element slots (its icon, a
//! `KeyCap`) are dropped: gpui cannot rasterize an element into the bitmap a
//! native row wears. A glyph it can: an item's glyph is rasterized to a PNG
//! as the menu is presented, in the theme's content ink, and the row wears
//! it beside the label, the check, the disabled state and the plain-text
//! hint. AppKit presents it as a template image and inks it itself.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use gpui::{
    AnyWindowHandle, App, AsyncApp, Bounds, Entity, IntoElement, Pixels, Point, SharedString, Task,
    Window,
};

use crate::components::popup::{Anchor, OnOpenChange, PopupState, Press, part_id};
use crate::foundation::scope::{Provide, provide, scoped};

/// How long the listeners stay registered after the OS reports the menu
/// closed, for the platforms that deliver the chosen item's action after it.
const DISMISS_GRACE: Duration = Duration::from_millis(50);

/// How often the drain task looks at the queue.
const POLL_INTERVAL: Duration = Duration::from_millis(20);

/// How a scope's dropdowns present themselves — the sheet the kit draws, or
/// the OS's own popup.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuPresentation {
    /// The kit draws the sheet. The default.
    #[default]
    Custom,
    /// The trigger keeps its DazzUI drawing; the OS draws the list.
    Native,
}

/// The innermost [`MenuPresentation`] a [`provide_menu_presentation`] put in
/// force — [`MenuPresentation::Custom`] where none has.
pub fn menu_presentation(cx: &App) -> MenuPresentation {
    scoped::<MenuPresentation>(cx).unwrap_or(MenuPresentation::Custom)
}

/// `child`, with `presentation` in force for every dropdown inside it —
/// Flutter's `MenuConfiguration`, SwiftUI's `.menuPresentation(_:)`.
pub fn provide_menu_presentation(
    presentation: MenuPresentation,
    child: impl IntoElement,
) -> Provide {
    provide(presentation, child)
}

/// One row of a menu, as the OS popup wears it. A heading is a group's
/// caption; it presents disabled. A row holding [`NativeMenuEntry::children`]
/// presents as a submenu — the nesting is the OS's own. This is the seam a
/// [`NativeMenuBackend`] reads and a test drives.
#[derive(Clone)]
pub struct NativeMenuEntry {
    pub label: String,
    pub enabled: bool,
    /// Radio-style check state; `Some` puts a checkbox column on the row.
    pub checked: Option<bool>,
    /// The plain-text form of the shortcut slot — a hint appended to the
    /// label for display, never an accelerator.
    pub hint: Option<String>,
    /// The row's icon, a glyph's asset path.
    pub icon: Option<SharedString>,
    /// That glyph as the backend hands it to the OS: a PNG, base64, written
    /// as the menu is presented when the `native-menus` feature is on.
    pub icon_image: Option<String>,
    pub heading: bool,
    /// Run on the main thread when the row is chosen.
    pub on_select: Option<Press>,
    /// The rows a right arrow opens beside this one.
    pub children: Vec<NativeMenuEntry>,
}

impl NativeMenuEntry {
    pub fn new(label: impl Into<String>) -> Self {
        NativeMenuEntry {
            label: label.into(),
            enabled: true,
            checked: None,
            hint: None,
            icon: None,
            icon_image: None,
            heading: false,
            on_select: None,
            children: Vec::new(),
        }
    }

    pub fn heading(label: impl Into<String>) -> Self {
        NativeMenuEntry {
            heading: true,
            enabled: false,
            ..NativeMenuEntry::new(label)
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn checked(mut self, checked: Option<bool>) -> Self {
        self.checked = checked;
        self
    }

    pub fn hint(mut self, hint: Option<String>) -> Self {
        self.hint = hint;
        self
    }

    pub fn icon(mut self, icon: Option<SharedString>) -> Self {
        self.icon = icon;
        self
    }

    pub fn on_select(mut self, on_select: Option<Press>) -> Self {
        self.on_select = on_select;
        self
    }

    pub fn children(mut self, children: Vec<NativeMenuEntry>) -> Self {
        self.children = children;
        self
    }
}

impl NativeMenuEntry {
    /// Every row, depth-first: the walk the OS build and the click dispatch
    /// must agree on, a heading taking its slot like any other row.
    pub fn walk(&self, visit: &mut impl FnMut(&NativeMenuEntry)) {
        visit(self);
        for child in &self.children {
            child.walk(visit);
        }
    }
}

/// One report from a native popup: a row was chosen, or the menu closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeMenuEvent {
    Clicked(usize),
    Dismissed,
}

/// Where a backend puts its events. Any thread the OS calls back on may
/// push; the drain task on the main thread takes them out.
#[derive(Clone, Default)]
pub struct EventQueue(Arc<Mutex<Vec<NativeMenuEvent>>>);

impl EventQueue {
    pub fn new() -> Self {
        EventQueue(Arc::new(Mutex::new(Vec::new())))
    }

    pub fn push(&self, event: NativeMenuEvent) {
        self.0.lock().unwrap().push(event);
    }

    fn drain(&self) -> Vec<NativeMenuEvent> {
        std::mem::take(&mut self.0.lock().unwrap())
    }
}

/// What a component asks a backend to present.
pub struct NativeMenuRequest {
    pub entries: Vec<NativeMenuEntry>,
    /// The trigger, or the right-click point, in window coordinates.
    pub anchor: Bounds<Pixels>,
    /// The menu's right edge at the anchor's right, not its left at the
    /// anchor's left.
    pub align_end: bool,
    /// The window to present over.
    pub window: AnyWindowHandle,
    /// The platform window behind it, resolved while `&Window` was in hand.
    pub window_ptr: usize,
}

/// The OS popup behind a scope's dropdowns. One method set, three
/// implementations: the platform backend the `native-menus` feature compiles,
/// and a stand-in for tests. Presenting is synchronous; events arrive later
/// through the queue.
pub trait NativeMenuBackend: 'static {
    /// Whether this platform can present at all.
    fn supported(&self) -> bool;

    /// Presents the menu anchored as `request` asks. `false` means it did
    /// not present and the caller falls back to the drawn sheet. Events go
    /// to `events` from whatever thread the OS calls back on.
    fn present(&self, request: &NativeMenuRequest, events: &EventQueue) -> bool;

    /// Dismisses the live menu, if there is one.
    fn close(&self);

    /// The pointer, in `window`'s coordinates — for a bar that follows the
    /// pointer to the next menu once one closes.
    fn pointer_position(&self, request: &NativeMenuRequest) -> Option<Point<Pixels>>;
}

thread_local! {
    static DEBUG_BACKEND: RefCell<Option<Rc<dyn NativeMenuBackend>>> =
        const { RefCell::new(None) };
    static PLATFORM_BACKEND: RefCell<Option<Rc<dyn NativeMenuBackend>>> =
        const { RefCell::new(None) };
}

/// Puts a stand-in backend in force — the seam the behavior tests drive,
/// the way Flutter's `debugNativeMenuBackend` does. `None` restores the
/// platform's.
#[doc(hidden)]
pub fn set_debug_native_menu_backend(backend: Option<Rc<dyn NativeMenuBackend>>) {
    DEBUG_BACKEND.with_borrow_mut(|slot| *slot = backend);
}

fn backend() -> Rc<dyn NativeMenuBackend> {
    if let Some(backend) = DEBUG_BACKEND.with_borrow(|backend| backend.clone()) {
        return backend;
    }
    #[cfg(feature = "native-menus")]
    {
        if let Some(backend) = PLATFORM_BACKEND.with_borrow(|backend| backend.clone()) {
            return backend;
        }
        let backend: Rc<dyn NativeMenuBackend> = Rc::new(platform::PlatformBackend::default());
        PLATFORM_BACKEND.with_borrow_mut(|slot| *slot = Some(backend.clone()));
        return backend;
    }
    #[cfg(not(feature = "native-menus"))]
    Rc::new(UnsupportedBackend)
}

/// The backend a build without `native-menus` carries: never supported, so
/// every dropdown keeps its drawn sheet.
#[allow(dead_code)]
pub(crate) struct UnsupportedBackend;

impl NativeMenuBackend for UnsupportedBackend {
    fn supported(&self) -> bool {
        false
    }

    fn present(&self, _: &NativeMenuRequest, _: &EventQueue) -> bool {
        false
    }

    fn close(&self) {}

    fn pointer_position(&self, _: &NativeMenuRequest) -> Option<Point<Pixels>> {
        None
    }
}

/// The native half of a dropdown's bookkeeping, keyed beside its
/// [`PopupState`]: the live OS popup, whether one is on its way, and whether
/// the last present was refused — that open falls back to the sheet.
#[derive(Default)]
pub(crate) struct NativeMenuState {
    live: Option<Live>,
    pending: bool,
    /// Identifies the Windows popup scheduled outside the current GPUI update.
    #[cfg(all(target_os = "windows", feature = "native-menus"))]
    opening: Option<Rc<()>>,
    /// The last present was refused: that open falls back to the sheet.
    pub(crate) failed: bool,
    /// What a dismissal from the OS runs — the owner's `on_open_change`, or
    /// the bar's follow-the-pointer onto the next menu. Weakly tied back to
    /// this state, so the closure and the state do not pin each other.
    on_dismiss: Option<OnOpenChange>,
}

struct Live {
    queue: EventQueue,
    /// Row actions parallel to the entries the backend was given.
    selects: Rc<Vec<Option<Press>>>,
    dismissed: bool,
    notified: bool,
    deadline: Option<Instant>,
    task: Task<()>,
}

impl NativeMenuState {
    /// Whether an OS popup is out there right now.
    fn presented(&self) -> bool {
        self.live.is_some()
    }

    /// Forgets the live session, reporting whether there was one.
    fn take(&mut self) -> bool {
        self.pending = false;
        self.failed = false;
        self.on_dismiss = None;
        let opened = self.live.take().is_some();
        #[cfg(all(target_os = "windows", feature = "native-menus"))]
        let opened = self.opening.take().is_some() || opened;
        opened
    }

    fn select_of(&self, index: usize) -> Option<Press> {
        self.live
            .as_ref()
            .and_then(|live| live.selects.get(index))
            .and_then(|select| select.clone())
    }

    /// Drains one batch, its clicks ordered before its dismissal — AppKit
    /// and Win32 report the close before the chosen item's action — and
    /// says whether the session is finished.
    fn poll(&mut self) -> (Vec<NativeMenuEvent>, bool) {
        let Some(live) = self.live.as_mut() else {
            return (Vec::new(), true);
        };
        let mut events = Vec::new();
        let mut dismissed = false;
        for event in live.queue.drain() {
            match event {
                NativeMenuEvent::Clicked(_) => events.push(event),
                NativeMenuEvent::Dismissed => dismissed = true,
            }
        }
        if dismissed && !live.dismissed {
            live.dismissed = true;
            live.deadline = Some(Instant::now() + DISMISS_GRACE);
        }
        if dismissed && !live.notified {
            live.notified = true;
            events.push(NativeMenuEvent::Dismissed);
        }
        let done = live.dismissed && live.deadline.is_some_and(|at| Instant::now() >= at);
        (events, done)
    }
}

/// The native-menu state keyed beside the popup `id` names.
pub(crate) fn native_state_for(
    id: &gpui::ElementId,
    window: &mut Window,
    cx: &mut App,
) -> Entity<NativeMenuState> {
    window.use_keyed_state(part_id(id, "native"), cx, |_, _| NativeMenuState::default())
}

/// Whether the scope asks for native dropdowns and this build can present
/// them. A refused present falls back to the sheet through `failed`, not
/// through here.
pub(crate) fn native_mode_active(cx: &App) -> bool {
    menu_presentation(cx) == MenuPresentation::Native && backend().supported()
}

/// Opens without the drawn sheet's focus handoff — nothing is drawn for the
/// OS popup to take the keyboard from.
pub(crate) fn set_open(state: &Entity<PopupState>, open: bool, cx: &mut App) {
    state.update(cx, |state, cx| {
        state.open = open;
        if !open {
            state.highlighted = None;
        }
        cx.notify();
    });
}

/// Dismisses the OS popup a dropdown holds, if it holds one — a close, or a
/// scope that changed its mind about native.
pub(crate) fn close_native_menu(native: &Entity<NativeMenuState>, cx: &mut App) {
    if native.update(cx, |state, _| state.take()) {
        backend().close();
        cx.refresh_windows();
    }
}

/// Installs what a dismissal from the OS runs — the owner sets it while it
/// holds an open native popup.
pub(crate) fn set_on_dismiss(
    native: &Entity<NativeMenuState>,
    on_dismiss: Option<OnOpenChange>,
    cx: &mut App,
) {
    native.update(cx, |state, _| state.on_dismiss = on_dismiss);
}

/// The press a native dropdown's trigger gets: it presents the OS popup —
/// whose anchor is where the last frame drew the trigger — or dismisses it,
/// and reports the change either way, the way the drawn toggle does. Run
/// from a press handler, outside any frame.
pub(crate) fn native_press(
    state: &Entity<PopupState>,
    native: &Entity<NativeMenuState>,
    entries: Vec<NativeMenuEntry>,
    anchor: Anchor,
    align_end: bool,
    on_change: Option<OnOpenChange>,
) -> Press {
    let state = state.clone();
    let native = native.clone();
    Rc::new(move |window, cx| {
        let open = !state.read(cx).open;
        if open {
            set_open(&state, true, cx);
            if let Some(on_change) = &on_change {
                on_change(true, window, cx);
            }
            present_native_menu(
                entries.clone(),
                &anchor,
                align_end,
                &state,
                &native,
                window,
                cx,
            );
        } else {
            close_native_menu(&native, cx);
            set_open(&state, false, cx);
            if let Some(on_change) = &on_change {
                on_change(false, window, cx);
            }
        }
    })
}

/// Arms a present for the next frame — the path of an open set
/// programmatically, a `default_open` or a host's controlled `open`, where
/// no press handler is running. A press-driven open never gets here.
pub(crate) fn arm_native_menu(
    state: &Entity<PopupState>,
    native: &Entity<NativeMenuState>,
    entries: Vec<NativeMenuEntry>,
    anchor: Anchor,
    align_end: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let armed = {
        let state = native.read(cx);
        state.pending || state.presented()
    };
    if armed {
        return;
    }
    native.update(cx, |state, _| state.pending = true);
    let native = native.clone();
    let state = state.clone();
    window.on_next_frame(move |window, cx| {
        present_native_menu(entries, &anchor, align_end, &state, &native, window, cx);
    });
}

/// Presents the OS popup now — the frame the anchor was written by has
/// already painted. Refused, the open falls back to the sheet; accepted, a
/// drain task carries the popup's events into the window.
pub(crate) fn present_native_menu(
    entries: Vec<NativeMenuEntry>,
    anchor: &Anchor,
    align_end: bool,
    state: &Entity<PopupState>,
    native: &Entity<NativeMenuState>,
    window: &mut Window,
    cx: &mut App,
) {
    native.update(cx, |state, _| state.pending = false);
    if !state.read(cx).open {
        return;
    }
    #[cfg(feature = "native-menus")]
    let entries = {
        let mut entries = entries;
        let ink = crate::theme::ActiveTheme::theme(cx)
            .color_content
            .hsla()
            .to_rgb();
        icon_images(&mut entries, ink, cx);
        entries
    };
    let request = NativeMenuRequest {
        anchor: anchor.bounds().unwrap_or_default(),
        align_end,
        window: window.window_handle(),
        window_ptr: native_window_ptr(window),
        entries,
    };
    // The click dispatch reads the same depth-first walk the OS build made,
    // a heading taking its slot like any other row.
    let mut slots = Vec::new();
    for entry in &request.entries {
        entry.walk(&mut |row| {
            slots.push((!row.heading).then(|| row.on_select.clone()).flatten());
        });
    }
    let selects: Rc<Vec<Option<Press>>> = Rc::new(slots);
    let queue = EventQueue::new();
    let backend = backend();

    #[cfg(all(target_os = "windows", feature = "native-menus"))]
    if DEBUG_BACKEND.with_borrow(|backend| backend.is_none()) {
        let opening = Rc::new(());
        native.update(cx, |state, _| {
            state.pending = true;
            state.opening = Some(opening.clone());
        });
        let native = native.clone();
        let state = state.clone();
        let window_handle = request.window;
        cx.spawn(async move |cx: &mut AsyncApp| {
            let current = |cx: &App| {
                native
                    .read(cx)
                    .opening
                    .as_ref()
                    .is_some_and(|current| Rc::ptr_eq(current, &opening))
            };
            if !window_handle
                .update(cx, |_, _, cx| current(cx) && state.read(cx).open)
                .unwrap_or(false)
            {
                return;
            }
            // TrackPopupMenu pumps Windows messages, including GPUI tasks.
            // Release the app borrow before entering it so those tasks can
            // update the app, dismiss the menu, or shut down the window.
            let presented = backend.present(&request, &queue);
            let _ = window_handle.update(cx, |_, _, cx| {
                if !current(cx) {
                    return;
                }
                native.update(cx, |state, _| {
                    state.pending = false;
                    state.opening = None;
                });
                finish_present(
                    presented,
                    queue,
                    selects,
                    window_handle,
                    &state,
                    &native,
                    cx,
                );
            });
        })
        .detach();
        return;
    }

    let presented = backend.present(&request, &queue);
    finish_present(presented, queue, selects, request.window, state, native, cx);
}

fn finish_present(
    presented: bool,
    queue: EventQueue,
    selects: Rc<Vec<Option<Press>>>,
    window_handle: AnyWindowHandle,
    state: &Entity<PopupState>,
    native: &Entity<NativeMenuState>,
    cx: &mut App,
) {
    if !presented {
        native.update(cx, |state, _| state.failed = true);
        cx.refresh_windows();
        return;
    }
    native.update(cx, |state, _| {
        state.failed = false;
        state.live = Some(Live {
            queue: queue.clone(),
            selects,
            dismissed: false,
            notified: false,
            deadline: None,
            task: Task::ready(()),
        });
    });
    let drain_native = native.clone();
    let drain_state = state.clone();
    let task = cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            cx.background_executor().timer(POLL_INTERVAL).await;
            let Ok((events, done)) = drain_native.update(cx, |state, _| state.poll()) else {
                break;
            };
            for event in events {
                let dispatched = window_handle.update(cx, |_, window, cx| {
                    dispatch_native_event(event, &drain_native, &drain_state, window, cx);
                });
                if dispatched.is_err() {
                    return;
                }
            }
            if done {
                let _ = window_handle.update(cx, |_, _, cx| {
                    drain_native.update(cx, |state, _| {
                        state.live = None;
                    });
                    cx.refresh_windows();
                });
                break;
            }
        }
    });
    native.update(cx, |state, _| {
        if let Some(live) = state.live.as_mut() {
            live.task = task;
        }
    });
}

/// Runs one popup event inside the window: a choice runs its row's press; a
/// dismissal closes the kit's bookkeeping and then hears the owner's
/// `on_dismiss` — which may keep the bar going onto the next menu.
fn dispatch_native_event(
    event: NativeMenuEvent,
    native: &Entity<NativeMenuState>,
    state: &Entity<PopupState>,
    window: &mut Window,
    cx: &mut App,
) {
    match event {
        NativeMenuEvent::Clicked(index) => {
            if let Some(select) = native.read(cx).select_of(index) {
                select(window, cx);
            }
        }
        NativeMenuEvent::Dismissed => {
            // An uncontrolled popup flips its own state; a controlled one
            // writes through its host on the next render.
            state.update(cx, |state, cx| {
                state.open = false;
                state.highlighted = None;
                cx.notify();
            });
            let on_dismiss = native.read(cx).on_dismiss.clone();
            if let Some(on_dismiss) = on_dismiss {
                on_dismiss(false, window, cx);
            }
        }
    }
}

/// The platform window behind `window`, resolved while `&Window` is in
/// hand. Linux positions from the pointer and never asks. The stand-in
/// backend never asks either — and a test window has no platform behind it
/// to ask.
#[allow(unused_variables)]
pub(crate) fn native_window_ptr(window: &Window) -> usize {
    if DEBUG_BACKEND.with_borrow(|backend| backend.is_some()) {
        return 0;
    }
    #[cfg(all(
        feature = "native-menus",
        any(target_os = "macos", target_os = "windows")
    ))]
    {
        platform::window_ptr(window)
    }
    #[cfg(not(all(
        feature = "native-menus",
        any(target_os = "macos", target_os = "windows")
    )))]
    {
        0
    }
}

/// Where the pointer sits, in `window`'s coordinates.
pub(crate) fn native_pointer(window: &Window) -> Option<Point<Pixels>> {
    let request = NativeMenuRequest {
        entries: Vec::new(),
        anchor: Bounds::default(),
        align_end: false,
        window: window.window_handle(),
        window_ptr: native_window_ptr(window),
    };
    backend().pointer_position(&request)
}

/// The edge of the bitmap a glyph is rasterized to: twice the 16pt AppKit
/// sets a menu image to, and the 32px nativeapi's Win32 backend draws.
#[cfg(feature = "native-menus")]
const ICON_EDGE: u32 = 32;

/// Writes each entry's `icon_image` from its glyph, through the app's asset
/// source — the one gpui draws the same glyph from.
#[cfg(feature = "native-menus")]
fn icon_images(entries: &mut [NativeMenuEntry], ink: gpui::Rgba, cx: &App) {
    for entry in entries {
        entry.icon_image = entry.icon.as_ref().and_then(|path| {
            let svg = cx.asset_source().load(path).ok()??;
            rasterize_glyph(&svg, ink)
        });
        icon_images(&mut entry.children, ink, cx);
    }
}

/// A glyph's SVG as a native row wears it: a square PNG, base64, the glyph
/// centred in it and inked like a drawn one — its coverage in `ink`,
/// whatever fill the SVG names. `None` when the SVG does not parse.
#[cfg(feature = "native-menus")]
pub(crate) fn rasterize_glyph(svg: &[u8], ink: gpui::Rgba) -> Option<String> {
    use base64::Engine as _;
    use resvg::{tiny_skia, usvg};

    let tree = usvg::Tree::from_data(svg, &usvg::Options::default()).ok()?;
    let size = tree.size();
    let edge = ICON_EDGE as f32;
    let scale = edge / size.width().max(size.height());
    let mut pixmap = tiny_skia::Pixmap::new(ICON_EDGE, ICON_EDGE)?;
    let transform = tiny_skia::Transform::from_scale(scale, scale).post_translate(
        (edge - size.width() * scale) / 2.,
        (edge - size.height() * scale) / 2.,
    );
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let [r, g, b] = [ink.r, ink.g, ink.b].map(|c| (c.clamp(0., 1.) * 255.).round() as u32);
    for pixel in pixmap.pixels_mut() {
        let a = pixel.alpha() as u32;
        // Premultiplied, as the pixmap stores it.
        let channel = |c: u32| ((c * a + 127) / 255) as u8;
        *pixel = tiny_skia::PremultipliedColorU8::from_rgba(
            channel(r),
            channel(g),
            channel(b),
            a as u8,
        )?;
    }
    let png = pixmap.encode_png().ok()?;
    Some(base64::engine::general_purpose::STANDARD.encode(png))
}

#[cfg(feature = "native-menus")]
#[path = "native_menu_platform.rs"]
mod platform;

#[cfg(all(test, feature = "native-menus"))]
mod tests {
    use base64::Engine as _;
    use resvg::tiny_skia::Pixmap;

    use super::{ICON_EDGE, rasterize_glyph};

    /// A glyph whose left half is filled, in a fill the ink overrides.
    const HALF: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><rect width="10" height="20" fill="red"/></svg>"#;

    #[test]
    fn a_glyph_rasterizes_to_a_square_png_in_the_ink() {
        let ink = gpui::Rgba {
            r: 0.,
            g: 0.,
            b: 1.,
            a: 1.,
        };
        let png = base64::engine::general_purpose::STANDARD
            .decode(rasterize_glyph(HALF, ink).expect("the svg parses"))
            .unwrap();
        let pixmap = Pixmap::decode_png(&png).unwrap();
        assert_eq!((pixmap.width(), pixmap.height()), (ICON_EDGE, ICON_EDGE));
        let at = |x: u32, y: u32| pixmap.pixel(x, y).unwrap().demultiply();
        let covered = at(4, 16);
        assert_eq!(
            (
                covered.red(),
                covered.green(),
                covered.blue(),
                covered.alpha()
            ),
            (0, 0, 255, 255),
            "covered pixels wear the ink, not the svg's fill"
        );
        assert_eq!(at(28, 16).alpha(), 0, "uncovered pixels stay clear");
    }

    #[test]
    fn an_svg_that_does_not_parse_has_no_image() {
        assert!(rasterize_glyph(b"not svg", gpui::Rgba::default()).is_none());
    }
}
