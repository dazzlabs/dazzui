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

Controls are drawn to React's geometry and recipes — a Tree, a
SegmentedControl, PillTabs or a Collapsible draws its own rows and segments
rather than restyling a native list — while native WinUI primitives own
editing, selection, focus and popup behavior.
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

`DAZZUI_SMOKE_SUITES=system-sizing` runs only system-size fixtures. They drive
the same text-scale notification and dispatcher path at 100%, 150% and 225%
without modifying the user's OS settings, across drawn and Windows themes.
They check label fit, native selection preservation, independent nested design
scopes, host-panel notifications and unchanged palette/stroke values, then render
all gallery pages at 225%.

Reports, the runtime story list, and PNG captures are written to a unique
ignored `artifacts/smoke-*` directory.

Story/catalog alignment and native rendering tests complement the React
conformance collector described above. A successful smoke run verifies the
native controls and interactions; the conformance verdict separately reports
the remaining differences from React.
