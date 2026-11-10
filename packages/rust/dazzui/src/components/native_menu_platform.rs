//! The `native-menus` backends, one per desktop platform, behind the
//! parent module's `PlatformBackend` and `window_ptr`.
//!
//! macOS and Windows call nativeapi directly: AppKit pops up inside its own
//! tracking loop, `TrackPopupMenu` inside its own modal pump, and both
//! position the menu against the window's content bounds. Linux has no GTK
//! in a gpui app, so a pump thread owns the default GLib context — the one
//! thing a GTK menu cannot live without — and every menu operation is sent
//! to it; nativeapi's own calls marshal onto the context they need. Linux
//! also positions from the pointer, since a Wayland compositor declines to
//! give out global window coordinates.

use gpui::Window;
use nativeapi::menu::{
    ListenerId, Menu, MenuEvent, MenuItem, MenuItemId, MenuItemState, MenuItemType,
};

use crate::components::native_menu::{EventQueue, NativeMenuEntry, NativeMenuEvent};

#[cfg(target_os = "linux")]
pub(crate) use linux::PlatformBackend;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub(crate) use windowed::PlatformBackend;
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub(crate) type PlatformBackend = crate::components::native_menu::UnsupportedBackend;

/// The platform window behind gpui's, resolved while `&Window` is in hand —
/// the NSWindow on macOS, the HWND on Windows, nothing on Linux, which
/// positions from the pointer.
#[allow(unused_variables)]
pub(crate) fn window_ptr(window: &Window) -> usize {
    #[cfg(target_os = "macos")]
    {
        use objc::{msg_send, runtime::Object, sel, sel_impl};
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let Some(handle) = HasWindowHandle::window_handle(window).ok() else {
            return 0;
        };
        let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
            return 0;
        };
        let view = appkit.ns_view.as_ptr() as *mut Object;
        if view.is_null() {
            return 0;
        }
        unsafe {
            let ns_window: *mut Object = msg_send![view, window];
            ns_window as usize
        }
    }
    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let Some(handle) = HasWindowHandle::window_handle(window).ok() else {
            return 0;
        };
        match handle.as_raw() {
            RawWindowHandle::Win32(win32) => win32.hwnd.get() as usize,
            _ => 0,
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        0
    }
}

/// One row as the OS will wear it: the label with the hint appended for
/// display — never an accelerator registered with the OS — and the state
/// the row's checkbox column shows.
pub(crate) struct Row {
    label: String,
    enabled: bool,
    checked: Option<bool>,
}

pub(crate) fn rows_of(entries: &[NativeMenuEntry]) -> Vec<Row> {
    entries
        .iter()
        .map(|entry| {
            let mut label = entry.label.clone();
            if let Some(hint) = &entry.hint {
                label.push('\t');
                label.push_str(hint);
            }
            Row {
                label,
                enabled: entry.enabled && !entry.heading,
                checked: entry.checked,
            }
        })
        .collect()
}

/// The menu those rows become. Headings present disabled; a row with a
/// check state presents as a checkbox even when unchecked, so the column
/// stays aligned. `None` when the OS refused to build it.
pub(crate) fn build_menu(rows: &[Row]) -> Option<(Menu, Vec<MenuItem>, Vec<MenuItemId>)> {
    let menu = Menu::new()?;
    let mut items = Vec::new();
    let mut ids = Vec::new();
    for row in rows {
        let item_type = if row.checked.is_some() {
            MenuItemType::Checkbox
        } else {
            MenuItemType::Normal
        };
        let item = MenuItem::with_label_and_type(&row.label, item_type)?;
        item.set_enabled(row.enabled);
        if let Some(checked) = row.checked {
            item.set_state(if checked {
                MenuItemState::Checked
            } else {
                MenuItemState::Unchecked
            });
        }
        ids.push(item.id());
        menu.add_item(Some(&item));
        items.push(item);
    }
    Some((menu, items, ids))
}

/// One listener for the menu's own events: a choice becomes the row's index,
/// a close becomes the dismissal.
pub(crate) fn install_listener(menu: &Menu, events: &EventQueue, ids: &[MenuItemId]) -> ListenerId {
    let ids = ids.to_vec();
    let events = events.clone();
    menu.add_listener(move |event| match event {
        MenuEvent::ItemClicked { item_id } => {
            if let Some(index) = ids.iter().position(|id| *id == *item_id) {
                events.push(NativeMenuEvent::Clicked(index));
            }
        }
        MenuEvent::Closed { .. } => events.push(NativeMenuEvent::Dismissed),
        _ => {}
    })
}

/// The menu while it is out; dropping it unregisters the listener and asks
/// the OS to stop tracking — the teardown a replaced or dismissed session
/// runs.
pub(crate) struct LiveMenu {
    menu: Menu,
    _items: Vec<MenuItem>,
    listener: ListenerId,
}

impl Drop for LiveMenu {
    fn drop(&mut self) {
        let _ = self.menu.remove_listener(self.listener);
        let _ = self.menu.close();
    }
}

/// macOS and Windows: nativeapi called in place. The menu positions against
/// the window's content bounds — window coordinates are content
/// coordinates.
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod windowed {
    use std::cell::RefCell;

    use gpui::{Pixels, Point, point, px};
    use nativeapi::display_manager::DisplayManager;
    use nativeapi::geometry::Point as NativePoint;
    use nativeapi::menu::{Menu, MenuBackend};
    use nativeapi::placement::Placement;
    use nativeapi::positioning_strategy::PositioningStrategy;
    use nativeapi::window::Window as NativeWindow;

    use super::{LiveMenu, build_menu, install_listener, rows_of};
    use crate::components::native_menu::{EventQueue, NativeMenuBackend, NativeMenuRequest};

    pub struct PlatformBackend {
        live: RefCell<Option<LiveMenu>>,
    }

    impl Default for PlatformBackend {
        fn default() -> Self {
            PlatformBackend {
                live: RefCell::new(None),
            }
        }
    }

    fn native_window(ptr: usize) -> Option<NativeWindow> {
        if ptr == 0 {
            return None;
        }
        // SAFETY: the pointer is the platform window `window_ptr` resolved
        // from a live gpui window, alive while the request is.
        unsafe { NativeWindow::with_native_window(ptr as *mut std::ffi::c_void) }
    }

    impl NativeMenuBackend for PlatformBackend {
        fn supported(&self) -> bool {
            Menu::is_backend_supported(MenuBackend::Native)
        }

        fn present(&self, request: &NativeMenuRequest, events: &EventQueue) -> bool {
            self.close();
            let Some(window) = native_window(request.window_ptr) else {
                return false;
            };
            let Some((menu, items, ids)) = build_menu(&rows_of(&request.entries)) else {
                return false;
            };
            let listener = install_listener(&menu, events, &ids);
            let anchor = request.anchor;
            let x = if request.align_end {
                anchor.right()
            } else {
                anchor.left()
            };
            let offset = NativePoint {
                x: x.to_f64(),
                y: anchor.bottom().to_f64(),
            };
            // Anchored to the content bounds rather than the window object:
            // that is what the window form resolves to, the rect form has kept
            // its signature across nativeapi revisions, and a GPUI window is
            // not the GTK widget the window form looks for on Linux.
            let bounds = window.content_bounds();
            let Some(strategy) =
                PositioningStrategy::relative_with_rect_and_offset(&bounds, &offset)
            else {
                return false;
            };
            let placement = if request.align_end {
                Placement::BottomEnd
            } else {
                Placement::BottomStart
            };
            let presented = menu.open(&strategy, placement);
            if presented {
                *self.live.borrow_mut() = Some(LiveMenu {
                    menu,
                    _items: items,
                    listener,
                });
            }
            presented
        }

        fn close(&self) {
            *self.live.borrow_mut() = None;
        }

        fn pointer_position(&self, request: &NativeMenuRequest) -> Option<Point<Pixels>> {
            let window = native_window(request.window_ptr)?;
            let bounds = window.content_bounds();
            let cursor = DisplayManager::get_cursor_position();
            Some(point(
                px((cursor.x - bounds.x) as f32),
                px((cursor.y - bounds.y) as f32),
            ))
        }
    }
}

/// Linux: a pump thread owns the default GLib context — the thing GTK menus
/// need iterating that a gpui app never iterates — and every menu operation
/// is a command to it. Its events arrive on that thread, straight into the
/// queue.
#[cfg(target_os = "linux")]
mod linux {
    use std::ffi::{c_char, c_int};
    use std::sync::OnceLock;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::{Receiver, Sender, channel};
    use std::time::Duration;

    use gpui::Point;
    use nativeapi::menu::{Menu, MenuBackend};
    use nativeapi::placement::Placement;
    use nativeapi::positioning_strategy::PositioningStrategy;

    use super::{LiveMenu, build_menu, install_listener, rows_of};
    use crate::components::native_menu::{EventQueue, NativeMenuBackend, NativeMenuRequest};

    type gboolean = i32;

    #[repr(C)]
    struct GMainContext {
        _opaque: [u8; 0],
    }

    unsafe extern "C" {
        fn gtk_init_check(argc: *mut c_int, argv: *mut *mut *mut c_char) -> gboolean;
        fn g_main_context_default() -> *mut GMainContext;
        fn g_main_context_acquire(context: *mut GMainContext) -> gboolean;
        fn g_main_context_iteration(context: *mut GMainContext, may_block: gboolean) -> gboolean;
    }

    /// What the pump thread is asked to do. Rows are plain data — the row
    /// actions stay on the main thread, in the session.
    enum Command {
        Present {
            rows: Vec<super::Row>,
            align_end: bool,
            events: EventQueue,
        },
        Close,
    }

    pub struct PlatformBackend {
        pump: OnceLock<Sender<Command>>,
        starting: AtomicBool,
        ready: AtomicBool,
    }

    impl Default for PlatformBackend {
        fn default() -> Self {
            PlatformBackend {
                pump: OnceLock::new(),
                starting: AtomicBool::new(false),
                ready: AtomicBool::new(false),
            }
        }
    }

    impl PlatformBackend {
        /// The pump, started if it can be. GTK has to initialize before any
        /// of it runs and the initialization opens the display connection:
        /// once, on the calling thread.
        fn pump(&self) -> Option<&Sender<Command>> {
            if !self.ready.load(Ordering::Relaxed)
                && self
                    .starting
                    .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
            {
                // SAFETY: plain C entry points; the gtk+-3 the bindings link
                // defines them.
                let started =
                    unsafe { gtk_init_check(std::ptr::null_mut(), std::ptr::null_mut()) } != 0;
                if started {
                    let (sender, receiver) = channel();
                    let _ = self.pump.set(sender);
                    spawn_pump(receiver);
                    self.ready.store(true, Ordering::Relaxed);
                }
            }
            if self.ready.load(Ordering::Relaxed) {
                self.pump.get()
            } else {
                None
            }
        }
    }

    fn spawn_pump(receiver: Receiver<Command>) {
        std::thread::spawn(move || {
            // SAFETY: same three entry points, run by the thread that owns
            // the context from here on.
            unsafe {
                let context = g_main_context_default();
                if g_main_context_acquire(context) == 0 {
                    return;
                }
                let mut live: Option<LiveMenu> = None;
                loop {
                    while let Ok(command) = receiver.try_recv() {
                        match command {
                            Command::Close => live = None,
                            Command::Present {
                                rows,
                                align_end,
                                events,
                            } => {
                                live = None;
                                live = present(rows, align_end, events);
                            }
                        }
                    }
                    // GTK menus move and choose on the GLib sources. A gpui
                    // app never runs this context, so this thread is what
                    // iterates it.
                    g_main_context_iteration(context, 0);
                    std::thread::sleep(Duration::from_millis(4));
                }
            }
        });
    }

    /// Builds and pops up the menu, on the pump thread — the context owner.
    /// Positioned at the pointer: on Wayland no global window coordinates
    /// exist to anchor against, and a click-opened menu's pointer is on its
    /// trigger anyway.
    fn present(rows: Vec<super::Row>, align_end: bool, events: EventQueue) -> Option<LiveMenu> {
        let (menu, items, ids) = build_menu(&rows)?;
        let listener = install_listener(&menu, &events, &ids);
        let strategy = PositioningStrategy::cursor_position()?;
        let placement = if align_end {
            Placement::BottomEnd
        } else {
            Placement::BottomStart
        };
        menu.open(&strategy, placement).then_some(LiveMenu {
            menu,
            _items: items,
            listener,
        })
    }

    impl NativeMenuBackend for PlatformBackend {
        fn supported(&self) -> bool {
            self.pump().is_some() && Menu::is_backend_supported(MenuBackend::Native)
        }

        fn present(&self, request: &NativeMenuRequest, events: &EventQueue) -> bool {
            let Some(sender) = self.pump() else {
                return false;
            };
            sender
                .send(Command::Present {
                    rows: rows_of(&request.entries),
                    align_end: request.align_end,
                    events: events.clone(),
                })
                .is_ok()
        }

        fn close(&self) {
            if let Some(sender) = self.pump.get() {
                let _ = sender.send(Command::Close);
            }
        }

        fn pointer_position(&self, _: &NativeMenuRequest) -> Option<Point> {
            // No nativeapi window wraps a gpui one on Linux, and the content
            // bounds the conversion needs do not exist on Wayland.
            None
        }
    }
}
