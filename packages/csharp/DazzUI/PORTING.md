# WinUI 3 port coverage

The example draws every story in the React example: `pnpm codegen` compiles
them into `example/Stories/`, and `example/Lib/Mapping.cs` maps each React
component and prop to this package. A story that sets a prop the mapping does
not list fails generation, naming it.

Implemented areas:

- Generated themes, per-subtree inheritance, native control skins and presentation.
- Selection, text/numeric/OTP fields, forms, progress and feedback.
- Menus, dialogs/drawers, popovers, navigation, tree/table, splitter and shortcuts.
- The React example's window: component list, overview, component pages,
  theme panel, resizable columns.
- Real-window all-theme rendering and native automation/interaction checks.

Where the kit has no counterpart for a React prop, the mapping composes one
from kit parts (a meter's label, a sidebar's collapsing column) or leaves it
to the kit's native look (a tree row's glyph, a splitter panel's percentage
limits). Matching stories is not a conformance assertion; the WinUI
conformance collector remains a separate task.

Validation outputs belong under ignored `artifacts/`, not in the source tree.
Keep design decisions in ds. Keep editing, selection, focus and popup behavior
in the Windows primitives.
