# DazzUI for WinUI 3

A native Windows implementation of the DazzUI design system, with every
shared theme and an example: the React example's window — the component list,
the overview of every component, a page per component with its examples, and
the theme panel — drawn with this package. Its stories are compiled from the
React example's by `pnpm codegen`.

## Run

Requires Windows 10 build 19041 or newer and the .NET 8 SDK or later. NuGet
access is required for the first restore. Visual Studio is optional; for IDE
work, open `DazzUI.sln` with the WinUI development workload installed.

From the repository root:

```powershell
pnpm winui3
pnpm winui3:build
pnpm winui3:test
```

From this directory, without Node or pnpm:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/run.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/run.ps1 -SmokeTest
dotnet build src/DazzUI/DazzUI.csproj
```

`run.ps1` accepts `-Configuration Release`, `-Platform ARM64` and `-NoBuild`.
`--page button --theme ember-dark` opens the example on a page and a theme,
as the React example's address does. The example is unpackaged and self-contained: it bundles .NET and Windows
App SDK dependencies, so no MSIX registration, signing certificate or separate
Windows App Runtime installation is needed. Keep the entire output directory
when copying the app. ARM64 is configured but requires validation on ARM64 hardware.

## Conformance

`scripts/conformance.sh --out <dir>` is the WinUI collector `pnpm conformance`
runs: from a Mac it builds and runs `example.exe --conformance` on a Windows
machine over SSH and copies back the readings of every scene in
`tools/conformance/scenes.json` — see
[`specs/conformance.md`](../../../specs/conformance.md).

For intrinsic text widths, collect the React reference on the same Windows
machine: `system-ui` resolves that machine's message font. From the repository
root, after installing dependencies and building the React package:

```powershell
$env:DAZZUI_CHROMIUM_EXECUTABLE = 'C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe'
node packages/js/dazzui/example/scripts/conformance.mjs --out packages/csharp/DazzUI/artifacts/react-windows --themes drawn --no-images
```

The browser collector fixes `en-US`, matching the examples' English labels
and date strings. Keep this reference separate from a macOS collection;
different system fonts have different advances. Both runs retain the usual
geometry tolerances and read actual rendered bounds.

## Components

- Actions and presentation: Button, IconButton, Toggle, ToggleGroup, Link,
  Badge, Card, Avatar, AvatarGroup, Callout, Code, CodeBlock, Divider, EmptyState,
  Icon, KeyCap and SectionLabel.
- Fields and selection: TextField, SecureTextField, SearchField, FieldValue, Select,
  Autocomplete, Combobox, NumberField, OtpField, Form, FormField, Fieldset,
  Checkbox, CheckboxGroup, Radio, RadioGroup, Switch, OptionCard,
  SegmentedControl, PillTabs, Tag, ShortcutRecorder, Calendar and
  DateRangePicker.
- Feedback: Progress, Meter, Slider, RangeSlider, Spinner, Skeleton,
  SkeletonText, Step, StepList, Toast and ToastViewport.
- Layout and navigation: ActionBar, Accordion, Collapsible, Preferences,
  PreferenceSection, PreferenceGroup, PreferenceRow, Sidebar, SidebarGroup,
  NavItem, Aside, Rail, Splitter, Tree, Table, TableHead, TableRow and TableCell.
- Overlays: Menu, ContextMenu, Menubar, Popover, PreviewCard, Tooltip,
  Dialog and Drawer.
- Window chrome: WindowTitlebar with its leading, title, toolbar, trailing and
  accessory slots, and WindowControls — the 46px Windows caption strip, with
  inactive ink, close-only and all-hidden sets. The host owns AppWindow and
  connects the minimize, maximize, close and drag requests.

Controls are drawn to React's geometry and recipes — a Tree, a
SegmentedControl, PillTabs or a Collapsible draws its own rows and segments
rather than restyling a native list — while native WinUI primitives own
editing, selection, focus and popup behavior. The collector has been used to
check that scene by scene: an Icon takes the theme's `icon.medium` unless a
size is named, a navigation row and its badge use the sidebar's own inset and
ink, card and stage text carry their declared leading, sliders and fields use
the design's block padding, and a disabled field repaints its native template
with the theme's disabled recipe instead of keeping the light one. Intrinsic
text widths use the host's system font; compare them with a browser reference
from the same host. A glyph drawn from a `Glyph` value still has no separate
specimen address.
Sidebar and Aside animate their width over `motion.duration`, and the
example's component list and theme panel do the same. The example lays the
page into its final centre once, then slides the pane over (or into) it, so
the document is not reflowed on every frame; the rail steps out of the
pointer's way when the column reaches zero.
Toast uses a compact horizontal layout; timed notices dismiss themselves,
while persistent notices expose a dismiss button whose accessible label can
be localized with `ToastOptions.DismissLabel`.
The system UI face comes from Windows' message-font setting, as the browser's
`system-ui` does. `Theme.CodeFont` exposes the Windows `ui-monospace` face used
by code, mono fields and numeric readings. Layout retains fractional logical
sizes, and annotation labels use React's declared leading.
`new Theme(name, ThemeSizing.System)` independently selects runtime system
sizing. It uses WinUI's `TextControlThemeFontSize` and
`TextControlThemeMinHeight` resources, with the committed Windows 11 metrics
as a fallback, and follows Windows Accessibility → Text size through
`UISettings.TextScaleFactor`. Typography, line heights, spacing, icons and
control boxes grow together. Corners, strokes, focus rings and animation
timings retain their design values; colors still come from the selected
family. XAML handles monitor DPI separately. Resolved text disables XAML's
additional text scaling so the accessibility factor is applied once.

The default `ThemeSizing.Design` retains shared geometry for conformance.
The interactive example starts in system sizing; **Follow system sizing** in
the theme panel switches it without rebuilding the current story. Pass
`--design-sizing` to start with design metrics. Smoke and conformance runs
use design sizing, with separate native system-sizing fixtures.

Loaded components follow text-size changes on their UI dispatcher and retain
native editing, selection and checked state. Host-owned panels can subscribe
through `Appearance.Follow(panel, theme => { /* update layout */ });`; the
subscription follows the panel's loaded/unloaded lifetime. `Theme.Text`
returns a styled snapshot; keep host-owned text current with that callback.

String annotation labels match missing font weights as CSS does and align
their glyph baselines using the font's ascent, descent and declared leading.
Text uses WinUI's OpenType formatter, including kerning and live text updates.
Keyboard symbols use the Windows math fallback; embedded bitmap strike
advances come from the installed font rather than theme-specific corrections.
Pill counts have their own token gap and tabular figures. Avatar rings are
outsets and do not enlarge their group's layout box.
Interactive preference rows use a native button for keyboard
and automation activation; adding or removing their action updates that role.
Controls that WinUI does not support subclassing are composed and expose their
native input through `Input`, `Indicator`, `View` or `Native`. RangeSlider and
Splitter implement their extra drag and keyboard behavior with native Thumb
controls and range-value automation peers. Icons are the Fluent System Icons
React draws, by name, from [FluentIcons.WinUI](https://github.com/davidxuang/FluentIcons):
`new Icon { Glyph = FluentIcons.Common.Icon.Settings }`, as SwiftUI names them
from its FluentIcons package. The one glyph the package spells differently is
the picker's: the web draws `CalendarLtr16Regular`, which this set carries
only as `Calendar`; the two differ in the dots' direction alone.

## Consume and theme

Reference `src/DazzUI/DazzUI.csproj` from a WinUI 3 app. Merge
`XamlControlsResources` in the host application resources, as `example/App.xaml`
does. The library loads its component templates from `Themes/Generic.xaml`.

```csharp
var input = new DazzUI.TextField { PlaceholderText = "Project name" };
var button = new DazzUI.Button
{
    Content = "Save",
    Tint = DazzUI.WidgetTint.Primary,
    Variant = DazzUI.ControlVariant.Filled,
    Size = DazzUI.ControlSize.Medium,
};
button.Click += (_, _) => Save(input.Text);

var scope = new DazzUI.ThemeScope
{
    Theme = new DazzUI.Theme("studio-light"),
    Content = new StackPanel { Children = { input, button } },
};

// A theme change preserves editing and selection state.
scope.Theme = new DazzUI.Theme("bright-dark", DazzUI.ThemeSizing.System);
```

`Appearance.SetTheme(root, theme)` also themes an existing subtree. An explicit
component theme overrides its ancestor. Theme, tint and size are dependency
properties; Button's variant and Badge/Card variants support bindings too.
Set composition properties and item collections before attaching a component.
After changing a composition or collection on a loaded component, call
`Appearance.Refresh(component)`. Native value and selection properties update
through their native controls and events.

## Tokens and stories

`ds/plugins/winui3.mjs` emits `Generated/tokens.json` through the
existing Terrazzo pipeline. `pnpm tokens` distributes the snapshot; commit it
and never edit it by hand. Palette, geometry and recipe decisions stay in ds.
The .NET library embeds the snapshot and builds independently of ds and Node.

- `example/Stories/`: a file per React story, generated by `pnpm codegen`;
  never edit them. `example/Lib/` holds the story type, the scene interpreter
  (`Scene.cs`), its component mapping (`Mapping.cs`), and the generated themes
  and glyph names. `example/Controls/` is the window, written by hand.
  See [`specs/playgrounds.md`](../../../specs/playgrounds.md).

## Validation

Tables use the React component hierarchy. Add bands to `Table.Children` and
cells to each band's `Children`; a cell's `Content` accepts text or a component.
`TableCell.Head` gives a cell the heading style and column-header role, and
`Align = TextAlignment.Right` corresponds to React's `align="end"`.
Cells share the available row width equally, with no column gap. Other child
elements retain their natural width. The bands have square corners in every
theme.

`TableRow.Interactive` enables keyboard focus and native button activation;
`Click` is the handler. `Active` remains caller-owned and takes its selection
wash from `Table.Tint`. Changing the theme or adding another row keeps existing
cell controls and their state. The table and cells expose native grid and table
automation patterns, including column headers and cell coordinates.
Only interactive rows expose selectable state to automation. Rich inline
cells align their content with the cell font's line box; block panels retain
their own height.

The former `Columns`/data-record `Rows` API and automatic list selection are
replaced by these independently composable controls.

```powershell
node --test ds/plugins/winui3.test.mjs
pnpm winui3:test
```

The smoke suite needs an interactive Windows desktop. It runs in a temporary
real WinUI window, renders every component's page in each of the twelve
themes DazzUI draws and the overview in every imported theme, and checks button geometry, state recipes and a scene's live state.
`DAZZUI_SMOKE_THEMES=studio-light,studio-dark` narrows the themes, and
`DAZZUI_SMOKE_SHOTS=all` keeps a picture of every page,
`DAZZUI_SMOKE_EXAMPLES=1` a picture of each example on its own, beside which
React's can be set, `DAZZUI_SMOKE_PAGES=a,b` narrows the pages and
`DAZZUI_SMOKE_EVERY_PAGE=1` renders every page in every theme it runs. Interaction checks cover
checkbox/radio automation, form validation and submission, OTP paste and
masking, range constraints, inherited themes without resetting input, menu
actions, popovers, dialog results and all four drawer edges.

`DAZZUI_SMOKE_SUITES=system-sizing` runs system-size fixtures. They drive
the same text-scale notification and dispatcher path at 100%, 150% and 225%
without modifying the user's OS settings, across drawn and Windows themes.
They check label fit, native selection preservation, independent nested design
scopes, host-panel notifications and unchanged palette/stroke values, then render
all gallery pages at 225%.

`DAZZUI_SMOKE_SUITES=geometry` runs the layout regressions on their own:
OpenType text, disabled ink, avatar outsets, inline link boxes, pill counts,
native shadow drawings and initial sidebar widths. The default suite includes
these fixtures too. The shadow fixture also reads pixels from small and large
shadows and checks CSS Gaussian decay at the display's rasterization scale.
Composition's radius is converted from CSS blur at the rendering boundary;
shadow offsets are snapped to physical pixels. Filled button backgrounds
cover the outer border edge as CSS backgrounds do.

Reports, the runtime story list, and PNG captures are written to a unique
ignored `artifacts/smoke-*` directory.

The default smoke run also keeps controls alive through theme and sizing
changes, checking link and annotation typography, native editor state colours,
and selection preservation. `DAZZUI_SMOKE_SUITES=appearance` additionally
compares every gallery page after live theme/sizing changes with a freshly
opened page in the same configuration, including visible text, glyphs and
their ink. `DAZZUI_SMOKE_SUITES=behavior` runs the live-scene, native interaction
and corner fixtures.

Story/catalog alignment and native rendering tests complement the React
conformance collector described above. A successful smoke run verifies the
native controls and interactions; the conformance verdict separately reports
the remaining differences from React.

`DAZZUI_SMOKE_SUITES=parity` exercises details outside the initial-scene
collector: search clearing and Escape, copy feedback, resizer keys, disabled
selection, button shortcuts, menu keyboard navigation and controlled checks,
tooltip/prose sheets, and feedback animation through resizing and remounting.
The same fixtures are included in the default smoke run. Use `parity-search`,
`parity-select`, `parity-query`, `parity-presentation`, `parity-selection`,
`parity-buttons`, `parity-overlays`, `parity-placement`, `parity-menus`,
`parity-open-scenes` or `parity-feedback` to isolate a family.
Keyboard fixtures verify their native foreground window before sending input.
Open-scene captures run the generated Menu, Popover and Tooltip stories in
three palettes; native popup screenshots use the actual screen region because
render-target readback does not cover every popup island.

Menus retain native flyouts and menu items, with the shared quiet labels,
per-item icon/check columns, transparent resting rows and neutral highlight.
Disabled rows can receive focus while confirmation remains inert. Home/End
visit the endpoints. Checked items use React's controlled radio-group rule:
the first true item is selected, and activation proposes a different item
without replacing the caller's model. Menubar keeps its trigger instances and
keyboard destination through theme changes; arrows and pointer movement also
switch between open menus without a second activation. Tooltip uses the raised surface
and small label; prose sheets include their padding and border in an explicit
width. Both draw shared shadows separately from their content.

Menu, Popover and PreviewCard expose `Side` and `Align` through native placement
modes. Menu defaults to bottom/end, a menubar to bottom/start, and prose sheets
to bottom/center. The placement fixture checks actual popup and trigger bounds,
including native edge collision handling.

Select keeps its native ComboBox navigation and scrolling while drawing the
shared popup sheet, option rows, group labels and checks. Autocomplete retains
free-form text; Combobox separates its query from its selected option value,
commits on confirmation, restores the selected label after an unmatched query
loses focus, and supports a clear action and an empty-results message. Query
fixtures type into the native editor and wait for its debounced input event.

Search's native editor has a separately focusable clear button whose accessible
label is `ClearLabel`. It remains visible for a
nonempty query after blur. Escape clears a query first and requests dismissal
only when already empty. CodeBlock copying is opt-in and reports successful
copying for the shared feedback duration. Progress and Skeleton follow Windows'
animation preference and stop their clocks when unloaded; Spinner retains the
reference's slower reduced-motion turn.
