# WinUI 3 port coverage

The example draws every story in the React example: `pnpm codegen` compiles
them into `example/Stories/`, and `example/Lib/Mapping.cs` maps each React
component and prop to this package. A story that sets a prop the mapping does
not list fails generation, naming it.

Implemented areas:

- Generated themes, per-subtree inheritance, native control skins and presentation.
- Selection, text/numeric/OTP fields, forms, progress and feedback.
- Menus, dialogs/drawers, popovers, navigation, tree/table, splitter and shortcuts.
- Sidebars that animate their width, on both sides of the example window.
- Window chrome: the titlebar band with its slots, caption control sets and
  inactive ink, with minimize/maximize/close/drag requests left to the host.
- The React example's window: component list, overview, component pages,
  theme panel, resizable columns.
- Real-window all-theme rendering and native automation/interaction checks.

The WinUI collector reads every scene against React's. The last pass aligned,
component by component: icon sizing from the theme's `icon.medium`; the
sidebar's row inset, group heading, icon elements and count badge; card and
stage leading; slider height and reading; field block padding; the menubar's
control height; and disabled fields, whose native template now takes the
selected theme's disabled recipe. Graphite and Bright were measured with no
non-width difference outside the platform's text measurement.

Intrinsic widths must be compared with React on the same Windows host, not
with a reference collected in another system font. Avatar rings are measured
from their actual outset borders. Empty ToastViewports are mounted in the
real window overlay and measured there. The four Studio Light viewport
specimens were collected at 1024 × 32, with 16px padding and an 8px gap,
matching the same-host React reference.

The detail fixtures additionally open the native Menu, Popover and Tooltip
stories, and exercise search clear/Escape, selection state, button shortcuts,
copy feedback, resizer keys and animation lifecycles. They complement the
initial readings; they do not establish complete pixel or interaction parity.

Select and suggestion lists now use the shared popup recipe over the native
controls. Combobox separates its selected value from its query and maps empty
results, clearing, initial selection and unmatched-query blur. Native fixtures
cover menu traversal, live popup themes, placement and edge collisions.

This is an implementation checkpoint, not a declaration of complete parity.
The latest Select disabled-navigation adapter and typeahead assertions still
need a fresh native run. Query fixtures reached a grouped-popup corner mismatch;
the native joined-corner policy has since been disabled but not revalidated.
Toast hover tests now use actual injected pointer movement and await hover
events; their full timing run remains pending. The latest live prose-theme
update also needs a rerun. Full all-theme collection and shadow/corner raster
comparisons remain separate verification requirements. A long smoke run rendered
691 of 708 pages before screenshot readback timed out; a separate process passed
the remaining 17 overview renders and native behavior checks.

Validation outputs belong under ignored `artifacts/`, not in the source tree.
Keep design decisions in ds. Keep editing, selection, focus and popup behavior
in the Windows primitives.
