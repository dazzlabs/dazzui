# DazzUI · GTK4

The Rust/gtk4-rs desktop implementation. GTK owns input, focus, selection,
keyboard navigation, accessibility and popup positioning; DazzUI draws the
native CSS nodes with the shared ds tokens. This crate is a separate Cargo
workspace beside the GPUI implementation and is invisible to pnpm.

Requires Rust and GTK4 **4.16 or later** with development headers and
`pkg-config` and `glib-compile-resources`. The minimum follows GTK's
introduction of CSS custom properties.
On macOS: `brew install gtk4 pkg-config`. On Linux, install the corresponding
GTK4 development package; Ubuntu 24.04's stock GTK 4.14 needs an upgrade.

```sh
# From this directory
cargo build --workspace
cargo run -p dazzui-gtk4-example
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Tests open native windows and must run with a display. On a Linux CI machine,
use `xvfb-run -a env G_DEBUG=fatal-warnings cargo test --workspace`. GTK runs
on the main process thread; the native test binaries deliberately do not use
the threaded Rust test harness.

```rust,no_run
use dazzui_gtk4::{gtk, prelude::*, *};

gtk::init().unwrap();
let window = gtk::Window::new();
Theme::new("bright-light").unwrap().apply(&window);
let column = box_(gtk::Orientation::Vertical, "3");
let email = text_field("you@example.com", Size::Medium);
column.append(&form_field("Email", &email, Some("Used for receipts")));
let save = button("Save", Variant::Filled, Tint::Primary, Size::Medium);
save.connect_clicked(|_| println!("saved"));
column.append(&save);
window.set_child(Some(&column));
```

Constructors expose their actual GTK widgets. Connect native signals and
read/write state with gtk4-rs, rather than learning another state API. Use
`set_dazz_size`, `set_dazz_tint`, `set_dazz_variant` and `set_dazz_invalid`
for drawing axes. `icon` loads the embedded Fluent glyph by its React export
name. Icon buttons also accept host icon-theme names and require an
accessible label. `glyph_button` puts an icon-theme glyph before or
after a label, sized, spaced and padded as React's Button does;
`glyph_segmented_control` does the same for segments, or draws each glyph
alone with the label as its accessible name. The legacy `table(model)` and
`tree(model, factory)` constructors expose GTK's selection models and factories;
rich compositions use `table_content` and `tree_content`.
`Combobox` is a searchable model selector; `autocomplete` retains a free-form
native entry with suggestions. `tag` is a native selectable toggle.

`Theme::apply` installs the embedded sheets once per display and selects a
complete token snapshot on a subtree. All ds themes are included. Nested
scopes reset every token, so an inner light scope does not inherit a dark
palette. Changing themes preserves the widgets and their state. Modal windows
carry their parent's theme when created; apply a new theme to them separately
when changing the parent while they are open. Application CSS is scoped to
DazzUI classes. The host chooses its theme; GTK's global theme is not replaced.

The token generator translates `-apple-system` to CoreText's
`.AppleSystemUIFont` and `system-ui` to Pango's `sans-serif`. Otherwise Pango
can use the CJK fallback for Latin text as well. Line heights are explicit ds
lengths: GTK's unitless leading
uses native ascent/descent rather than CSS's font size. `Theme::dimension`
exposes resolved token lengths for native layout properties that cannot read
CSS variables.

`src/generated/tokens.css` and `themes.rs` are committed output of
`ds/plugins/gtk4.mjs`. Run `pnpm tokens` at the repository root to regenerate;
never edit these files. `src/generated/icons/` is emitted by `pnpm codegen`
from the same Fluent SVG paths React imports and embedded during the Rust
build. The crate builds without Node or ds installed.

The library includes buttons, toggles, checkboxes, radios, switches, text,
password, multiline, search and number fields, selectors, sliders, progress,
meters, spinners, badges, tags, cards, callouts, typography, code, key caps,
avatars and groups, dividers, links, forms, preferences, collapsibles,
accordions, splitters, tabs, segmented controls, toggle groups, steps, option
cards, popovers, hover cards, tooltips, menus, dialogs, tables, trees and
action bars.

The example follows React's organization:

```text
example/src/
  main.rs          application / collector entry point
  app.rs           window lifecycle
  components/      shell, brand, stage, theme panel and theme preview
  lib/             story model, scene interpreter and generated theme families
  stories/         one generated Rust file per React story, plus registry
  conformance/     native readings and snapshots
```

The three columns are the searchable component list, the document under its
toolbar, and the theme panel. Overview contains Preview cards; a component
document contains its summary and examples (with notes). Native splitters
resize the columns, and toolbar buttons toggle the two side panels.
Theme families and their group headings come from React's `FAMILY_GROUPS`;
the panel switches appearance and displays a miniature of every family.
Use `pnpm gtk4 -- --story button --theme bright-dark` to open a specific page.
Pass `--new-instance` after Cargo's `--` to open the rebuilt executable in a
separate gallery process while another gallery is running.

The example's story pages are generated from the React stories by
`pnpm codegen`; it currently renders **188 of 188 scenes across 53 component
pages**. Unmapped scenes show their exact missing components or props. Mapped
scenes evaluate shared state expressions and connect native widget signals
without replacing edited fields. `select_options` accepts flat and grouped text
options, placeholders and disabled choices, and `select_values` maps native
positions to domain values. Advanced props still have explicit mapping gaps.
Scene coverage describes the authored gallery. Advanced mapping gaps and
unread paint properties remain explicit; dimension comparisons do not establish
complete API or image conformance. Imported themes are rendered by the native
smoke suite as well.

Story rows wrap at each child's own width through a GTK layout manager;
explicit story widths are capped by the available column. Preference groups,
sections and rows use the same hierarchy and spacing roles as React, and
multiline fields use a scrolling viewport sized by their row count and
minimum height, including when their initial contents wrap. Native tests
exercise these allocations as well as editing and selection state.
Document stages measure their scrollable content at the available width so
wrapped rows increase the card height. Shell checks cover all 53 documents,
button rows at three window widths, and native keyboard focus entry.

Accordion coordinates native Expanders and supports single or multiple open
sections, divided rows, leading glyphs and trailing metadata. ToggleGroup
supports empty and multiple selections, one tab stop and arrow navigation.
AvatarGroup overlaps opaque-backed avatars and shows the remaining count.
Popup scenes keep their original triggers and edited native fields. GTK
positions popovers and dismisses them on outside clicks. Modal windows use
native modality and focus containment; clicking their scrim does not yet
dismiss them. Rich hover cards retain interactive content, while tooltips
remain passive. Shared ds hover timing applies to pointer entry, keyboard
hints open immediately, and unmapping cancels pending presentation. Native
interaction tests check opening, closing, reopening, focus movement and modal
scrim removal. Controlled popup props that are not implemented remain explicit
mapping gaps.

`Drawer` keeps its native controls in a modal GtkPopover covering the owning
window. Its four edges, default/fixed size, scrim, header, scrolling body and
footer use the same ds roles as React. `set_open` prepares the window anchor
before presenting; resizing updates the same popup. `DrawerClose` maps to
native popup dismissal. Tests cover all four edges, RTL, resizing, retained
edits, focus entry/return, and real X11 Escape and scrim clicks on Omarchy's
isolated display. Source title, subtitle, header, body and footer bounds also
match the open 800×600 reference in Studio Light. Swipe dismissal, transition
motion, controlled open state, rich headings and relative sizing remain gaps;
the mapped stories do not establish those behaviours or screen-reader parity.

`ToastQueue` keeps receipts in the owning window's native overlay. Install
`toast_layer(content)` before presenting a host window; the gallery does this
at startup. Queues can also install the layer around an existing window child,
retaining that child and its focus. The viewport participates in pointer picking
only inside eligible receipts, so clicks elsewhere reach the original controls.
Showing a receipt preserves the input's focus;
F6 enters the notification region, Tab visits receipts and their actions, and
Escape dismisses the focused receipt. The last Tab restores the previous
control. The queue preserves the source's newest-first order and reverses its
visual order at the top. Limited receipts remain drawn but cannot receive
input, as in the current React stylesheet. Same-ID replacement keeps position
and resets its timer without calling the previous close callback. It retains
the actual action button and its focus. A priority change replaces only the
outer container because GTK fixes its accessible role when realized.

Notice/action durations and message line height come from ds. Danger defaults
to persistent; focus and pointer hover pause all timers. The scene interpreter
routes notifications through their provider, retaining the originating state
for action and close callbacks without a queue reference cycle. Native checks
cover click-through, real keyboard traversal including the empty viewport,
timeout/pause/resume, replacement, truncation, top placement and callback state.
The service schedules a single expiry timeout and does not poll every frame.
Open receipts match 336 source bounds checks on Omarchy across the twelve drawn
themes; the empty viewport also keeps its actual native allocation. Swipe
dismissal, transition motion, nested modal handoff and full paint/accessibility
parity remain unverified. Direct GTK announcements are used on Linux; GTK's
current macOS accessibility backend crashes on that call, so macOS retains the
native roles and labels without calling it. Initial scene coverage does not
establish those remaining behaviours.
The scene mapping requires one authored flow child and a direct viewport under
each provider; multiple flow siblings need a Row or Col to preserve their
source layout rather than acquire a new native column.

Skeleton draws the shared shapes and sweep, with proportional bars measured
from the current allocation and a final paragraph line at the ds proportion.
NumberField uses two native SpinButtons sharing one adjustment: the editable
input and the compact stacked stepper. GTK keeps numeric parsing, bounds,
precision and held-button repeats; the gallery connects numeric state in both
directions.
Both `number_field` and `number_field_with_unit` return `(field, input)`:
style or parent the first widget and connect native numeric signals on the
second. This replaces the initial port's standalone horizontal SpinButton.

`Form` coordinates registered `FormField`s over native Entry/TextView
controls. `FieldRules` supplies a name, required state, text/email/URL
constraint and an optional validation boundary; the form defaults to
`OnSubmit`, including for a field outside a form. Submission validates every
eligible field, focuses and selects the first invalid input in tree order,
then publishes the current named values. Read-only email type mismatches
still count; disabled inputs do not block submission. `set_errors` supplies
server errors, clears each named error on editing and focuses returned errors
after submission. GTK still owns editing, focus and selection.
`submit_button`, `reset_button` and `capture_defaults` wire native actions;
Enter activates the first submit button, while reset restores captured text
without fabricating edits or clearing validation state. Capturing defaults
again replaces those bindings rather than adding another implicit submit.
The example maps text forms and exposes unsupported control families,
portaled fields, custom validators and browser association props as gaps.
The older `form_field` helper remains available for presentation alone.

Buttons retain native GTK activation while measuring fractional caption
advances once and centering their contents, including full-width actions.
Hints round their complete wrapped line box, and native placeholder opacity
is reset so it does not fade the ds faint ink a second time.

`Combobox::new(ComboboxOptions)` is an editable choice with flat or grouped
options, filtering, selected checks and an optional clear action.
`autocomplete(AutocompleteOptions)` keeps arbitrary text and offers suggestions
without a trigger or clear button. Both return a `Combobox` coordinator exposing
its native `entry()`, `list()` and `popup()`. The input remains the one Tab stop;
arrow keys move the active suggestion, Enter chooses it, and Escape dismisses
or clears. Combobox restores its chosen label when a query is dismissed.
Native Popover positioning supplies edge collision and the list scrolls its
active row into view. Controlled values, read-only/disabled states and model
updates retain the input. Search uses ICU's locale-aware search collation and
Base UI's query-sized UTF-16 windows, including its accent, punctuation and
ligature boundaries. Native tests compare those cases with Base UI's actual
filter. Small imported control profiles allow the 20px actions to overflow
without enlarging the field, as React does. Rich empty content and browser
form/name/id association remain mapping gaps. The older `combobox` helper
still returns a searchable native DropDown. Actual pointer delivery on Omarchy
remains to be verified after its SSH address is reachable.

`slider_field(SliderOptions)` returns a `SliderField`: one native GTK Scale
per value, a shared track and an optional trailing reading. `scales()` exposes
the native controls; `values()` and `set_values()` retain existing thumbs when
the value or count changes. Pointer movement pushes neighbours or stops at
an explicit gap, keyboard movement respects neighbours, and each thumb keeps
its native focus and accessibility. Value changes can be cancelled; controlled
fields wait for their host to accept the value. Size, tint, RTL, coincident
thumb picking and the Omarchy ink knob use the shared recipes. The original
single-value `slider` helper still returns a native Scale. Vertical/inset
alignment, swap collision, custom/locale formatting and web form association
remain unmapped. Interaction tests exercise native gesture signals; actual
pointer delivery on Omarchy still needs verification after it reconnects.

`code_block` builds a native selectable listing with an optional header,
separate line-number gutter, scrolling and timed clipboard feedback. The
gutter stays outside the horizontal scroller and the copied source.
`code_block_source` exposes the native label; source updates keep the gutter
and clipboard payload in sync, including compiled story state.
`inline_code` returns an `InlineCode` widget whose `label()` exposes native
selection; its painted line box does not grow to GTK's selection bounds.
The convenience `code(text, block)` returns a `gtk::Widget` for either form.
The code face and copy-feedback duration come from ds. The clipboard
interaction test runs on an empty Linux clipboard or a locally owned provider;
run under Xvfb to exercise it without replacing external clipboard content.
Preference rows with `onOpen` use GTK buttons with the same text column and
the shared interactive wash.

`menu_popup` and `menubar` keep GTK's model rows, actions, native navigation,
radio indicators and dismissal. `menu_row_content` adds optional presentation
slots without registering shortcut hints as accelerators. The example maps
button, context and menubar menus, including disabled actions and controlled
radio state; repeated-letter and incremental search use the same session
duration as React. Context menus keep their original region and open at the
pointer, on touch long press or with Menu / Shift+F10. Dynamic item arrays,
controlled popup visibility and vertical menubars remain explicit gaps.

`Table` / `table_content` composes rich cells in native list rows. Use
`table_head`, `table_row` and `table_cell`, connect `rows().row-activated`,
and update `set_table_row_active` from host state. Headers are outside the
tab sequence; interactive rows and their embedded fields keep native focus
and activation. The composition exposes table, row and column-header roles,
retains edits on selection changes and propagates its own RTL column layout.
`table(model)` remains the ColumnView entry point for host-owned factories.
Badge line boxes round to the nearest GTK logical pixel while their labels
retain fractional leading. Table columns retain fractional positions and round
each cell once; the table rounds its accumulated text heights once. Its native
ListBox allocates the rows and owns hit testing. Table traversal skips static
headings between actionable rows and preserves the controls' own focus behavior.

`split_view` coordinates any number of `SplitPane`s over native Paned widgets;
`splitter_panel` supplies each scrolling region. Shares are normalized once,
the seam changes only its neighbouring pair, and `connect_sizes_changed`
reports user changes without treating window allocation as a drag. Native
pointer capture remains GTK's; arrow keys use the shared percentage steps,
Home/End respect both panels' bounds and double-click evens the pair within
those bounds. Tab walks panel controls and native seams. Resize, orientation
changes and RTL retain the proportions and native fields. Discrete changes
currently apply without React's panel interpolation. The existing `splitter`
constructor remains the direct two-child Paned API.

`Tree` / `tree_content` builds rich `TreeNode` rows over GTK's TreeListModel,
ListBox and TreeExpander. Children present as an empty array remain a branch.
Native twisties expand without selecting; row activation requests selection
and toggles a branch. Selection stays host-controlled through
`connect_selected_changed` / `set_selected`; expansion can be internal or
controlled with `TreeOptions.expanded`, `connect_expanded_changed` and
`set_expanded`. The tree has one Tab entry and React's physical arrow keys,
Home/End and ds-timed incremental name search. Disabled-row navigation follows
the shared source. Rich labels, icons and metadata retain their widgets when
collapsed; nested expansion is restored on reopening. Dynamic node arrays
remain an explicit gallery mapping gap. Twelve-theme comparisons match the
row geometry, icons and embedded badges within the shared tolerances.

`shortcut_recorder` returns a `ShortcutRecorder` with native recording and
clear buttons. Read `value`, connect `connect_value_changed` and feed the host's
accepted value back through `set_value`. A `connect_clear` callback replaces
the default empty-value request. Recording shows held modifiers, accepts
modified physical keys or F1–F20, cancels on Escape or blur, and clears on bare
Backspace/Delete. Invalid presses use the shared rejection duration; repeated
presses keep the original timeout. Unmapping cancels the session and timer.
The Linux Wayland / standard X11 evdev and macOS Carbon key positions preserve
letters under Alt/Option and keyboard layout changes. Other GTK backends
currently reject hardware positions they cannot translate; position-free
synthetic events retain React's ASCII fallback. No global shortcuts are
registered. Root sensitivity, prompts, clear label, error state and tint can
be updated without replacing the native controls.

`otp_field` takes a positive `OtpOptions.length` and returns an `OtpField`
with native Entry/Text cells. A code remains
contiguous across insertion, replacement, paste and deletion. Native input
methods, selection, copying, cutting and clipboard reads remain GTK's; the
coordinator distributes characters, filters validation modes and moves focus.
Navigation, grouping and the single Tab entry follow the shared primitive,
including RTL and modifier-assisted boundary movement. `OtpOptions.value`
selects host control; `connect_value_changed` receives a cancellable
`OtpChange`, and `set_value` accepts a later host update. Completion runs after
the accepted value, including complete pastes that do not change the value.
Masking, read-only and disabled states preserve the cells. State `error` in
the gallery selects the danger tint as React does; field validation separately
uses `set_invalid`. Native Unicode text uses code points. Custom normalization,
web form association, automatic submission and SMS autofill are not mapped;
unsupported scene props remain explicit gaps.

A collector already reads the generated specimen addresses and writes native
bounds, ink, font size, all four painted corners and 2× scene/specimen images
in the shared format:

```sh
example/tool/conformance.sh --out /tmp/dazzui-gtk4-readings \
  --themes studio-light,bright-dark --scenes button,checkbox,switch
```

Use `--themes drawn|all|<names>`, `--scenes <story ids>`, and `--no-images`.
`gaps.json` records skipped scenes; unread drawing properties are `null`.
The collector uses GTK's actual renderer and the platform's font fallback
(`font: null`), so image comparisons must not claim a shared test face. GTK
has not joined the default conformance run or its shrinking expectation
baselines until the remaining mappings and a measured baseline are reviewed.
An inactive display can leave a mapped widget unallocated; the collector
reports that as an error rather than writing an empty successful run. On a
locked or powered-down Linux desktop use a separate `xvfb-run` display.

`SidebarProvider` owns controlled or uncontrolled disclosure and the window's
Ctrl/Meta+B shortcut. `Sidebar` retains native controls while sliding offcanvas
or narrowing to icons; `none` columns remain visible. The gallery maps header,
scrolling content, groups, buttons, counts, trigger and inset/floating surfaces.
Native row activation updates the host's selection, and icon-only hints follow
the provider state. Rich row captions, custom trigger glyphs, action buttons,
rail handles and submenus remain explicit codegen gaps.

`Calendar::new(CalendarOptions)` composes one or two six-week month grids
from native GTK buttons. `CalendarDate` keeps Gregorian days independent of
time zones; ICU supplies localized month, weekday and accessible date names.
Single and range values and the first visible month can be controlled. Native
arrow/week/month/year navigation, Home/End and one day Tab stop follow React,
including focusable but unselectable bounded dates and pending-range previews.
`day_buttons`, `day_button`, `previous_button` and `next_button` expose the
native controls. Date cells remain in place across month changes; focus on an
obsolete date is released before a keyboard move restores its requested day.
Dynamic month counts, mode, week starts, navigation and locales remain explicit
codegen gaps.

`DateRangePicker::new(DateRangePickerOptions)` composes the native Calendar,
popover, preset menu and time selectors. Opening starts a fresh draft; only
Done emits a value change. Cancel, Escape, outside press and focus leaving
discard edits. Controlled hosts may refuse a commit. Calendar selection
preserves the clock times, moving the start carries the elapsed duration,
and an earlier end pulls the start back one hour. All-day mode retains the
clocks while disabling their selectors. Durations, relative-day presets,
time steps, labels and locale are configurable; off-step current times remain
in the native selector. The six generated picker scenes are mapped. Dynamic
structural configuration remains an explicit gallery gap.

The current port has been built and tested on Omarchy with GTK 4.22.4.
The Linux Xvfb run exercises all 72 themes, native interactions including
clipboard, and 376 light/dark scene renders. The complete twelve-theme gallery
comparison covers 8,292 instances and 27,684 readable properties, with zero
out-of-tolerance or missing readings. Unread paint properties, every open-popup
internal metric and full image conformance remain outside that result.

`text_area_with_rows(rows)` returns a native ScrolledWindow field and its
TextView. It retains the row-sized box while long text scrolls inside it;
`text_area()` remains the direct TextView constructor. The gallery uses the
viewport so an unconstrained first measure cannot turn a filled five-row
field into a 1601px document. Native regression checks use the collector's
frame and verify that later edits also scroll without growing the field.

`badge`, `key_cap` and `section_label` return a `TextBox`, whose `label()`
exposes the actual GTK Label. `set_text` retains that label and updates its
measurement. These passive boxes round the shaped advance once and keep the
CSS line box even when a fallback glyph extends beyond it. A nested key hint
retains its own type inside a button. `sidebar_menu_badge` uses the same text
measurement inside its fixed-height count box. These replace the initial
port's direct Label return types for those constructors.

Checkboxes and radios retain GTK's native indicators, activation and grouping,
while rounding the complete indicator-and-caption width once.
`checkbox_with_note` keeps the note in the label's native paragraph, including
its leading space and wrapping. Native text attributes take the note colour
from the current ds theme and follow changes to the surrounding theme scope.

Option cards keep their native CheckButton input and radio grouping. Their
paragraph height is carried in fractional CSS padding, retaining a 64.5px
painted box when the native content allocation is integral. A one-shot idle
updates that measured value between layout passes; wrapping, window resizing,
theme changes and disposal retain native focus and input behavior.

`pill_tabs`, `segmented_control` and `glyph_segmented_control` return a
`ButtonGroup` rather than the initial port's Box. `buttons()` exposes their
native GTK buttons; `set_stretch(true)` gives each an equal share of the
available width. The group rounds its combined fractional width once and
places members at fractional positions, including in RTL. It has one Tab
entry, remembers the last focused member, and moves focus with arrows and
Home/End without changing selection. Native GTK activation selects a member.
Ordinary buttons and toggles keep their CSS control height even when a glyph
extends beyond the line box; tags retain their automatic badge height.

On an isolated X11 display, `DAZZUI_X11_INPUT=1 GDK_BACKEND=x11 cargo test
--workspace` also verifies real arrow-key delivery and native Space activation
through XTEST. This optional checkpoint needs Python 3 and the X11/Xtst
runtime libraries. It finds its uniquely named test window and directs input
only there; the default test run does not require those tools.
