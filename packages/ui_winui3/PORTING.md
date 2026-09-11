# WinUI 3 port coverage

The port covers the native playground's 59 component pages and 7 token pages,
including Flutter's Aside, NavItem and Rail. `scripts/sync-stories.mjs` owns the
489 story registrations; actual rendered registry coverage is checked in both
directions by the smoke-test entry point.

Implemented areas:

- Generated themes, per-subtree inheritance, native control skins and presentation.
- Selection, text/numeric/OTP fields, forms, progress and feedback.
- Menus, dialogs/drawers, popovers, navigation, tree/table, splitter and shortcuts.
- Theme toolbar, search and grouped navigation, breadcrumbs, Overview sections
  and centered/page/fullscreen specimen layouts.
- Real-window all-theme rendering and native automation/interaction checks.

Validation outputs belong under ignored `artifacts/`, not in the source tree.
The React reference measurement and pixel-parity viewer integration remain a
separate task. Matching story names is not a pixel-parity assertion.

Keep design decisions in ds. Keep editing, selection, focus and popup behavior
in the Windows primitives. Never replace a working specimen with a placeholder
merely to pass the registry check.
