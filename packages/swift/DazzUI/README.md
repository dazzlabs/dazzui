# DazzUI

The DazzUI design system for SwiftUI — the platform sibling of
`packages/js/dazzui` and `packages/dart/dazzui`. Same tokens, same themes,
same components.

**`packages/js/dazzui` is the standard.** Every name, prop and metric here is
read off the React source; where the two disagree, the React file wins.

## Using it

A host depends on it by the repository's URL, pinned to a commit — the root
`Package.swift` re-exports this package (and `DazzMobileUI`), since SwiftPM
reads a manifest only at a repository's root:

```swift
.package(url: "https://github.com/dazzlabs/dazzui.git", revision: "<commit>")
```

```swift
import DazzUI

struct ContentView: View {
  var body: some View {
    Button("Translate", variant: .filled) {}
      .theme(.studioLight)
  }
}
```

`theme(_:)` scopes a token set to a subtree, the way React's `data-theme`
attribute does. Themes are per-subtree, so several can sit on screen at once.

## Naming

Names match the React export **exactly**, with no prefix — `Button` is
`Button`, `Divider` is `Divider`, `Table` is `Table`. Where a name is also
SwiftUI's, the module qualifies it at the use site (`SwiftUI.Button`) rather
than the package renaming its own component: a design system whose button is
called `DesignButton` teaches the wrong name for the thing.

The colour axis is `tint` (`Tint.primary`), never `kind` or `secondary` — the
ds vocabulary is the API here as much as it is in Flutter.

## Tokens

`Generated/` holds what `ds` wrote and is committed, so the package
builds and ships on its own — there is no dependency edge to the design
system and so no build order to get wrong. **Never hand-edit it**: a value a
component needs belongs in `ds`, and anything written here is
overwritten on the next `pnpm tokens`.

Two files come out of the generator:

- `Generated/Colors.swift` — the base ramps as `ColorSwatch` constants
- `Generated/ThemeVariables.swift` — the semantic and component layers as one
  struct, plus one instance per theme

Swift has no cascade, so a theme is not a block of overrides layered on at
render time: `plugins/swiftui.mjs` collapses each theme's mode chain into a
whole `ThemeVariables` value — `.studioDark`, `.brightLight`, `.brightDark` —
beside the `.studioLight` default. It reads the themes back out of the same
`modeSelectors` the stylesheets use, so adding a theme reaches Swift without
the plugin being touched.

`ThemeName` names every theme the token graph carries — the twelve drawn
ones below and the imported ones — and is generated with the variables, so
a theme added to ds reaches it without an edit here.

| `ThemeName`    | Feel |
| -------------- | ---- |
| `.studioLight` | White card, light grey canvas, violet accent. **The baseline** |
| `.studioDark`  | The same skeleton over a near-black canvas |
| `.brightLight` | Warm paper, ink navy, acid-green marker, pill controls |
| `.brightDark`  | Ink-navy canvas, with acid green as the accent itself |

## Layout

```
Sources/DazzUI/
  Views/       the components
  Foundation/  the types they are written against
  Theme/       the theme value and its environment key
  Generated/   what ds wrote
```

`Views/` rather than `Components/` because each platform package speaks its
own dialect: React has `components/`, Flutter has `widgets/`, and the SwiftUI
word for a piece of interface is `View`. The
*names* inside follow React exactly; the folder follows the platform.

## Foundation

The hand-written half of the vocabulary — the types the generated file is
written against, and the interaction primitives CSS gets from pseudo-classes:

- `ColorSwatch`, `ColorDescriptor`, `TypeStyle`, `TokenShadow`, `BezierCurve`
- `Tint`, `ControlVariant`, `ControlSize`, `ControlState`, and the
  `controlColors(variant:tint:state:)` resolution every interactive component
  reads — the Swift half of `data-control` plus `data-variant`
- `Pressable`, `HoverRegion`, `focusRing(visible:color:cornerRadius:width:)`

Interactive colours are stored as *recipes* rather than finished colours: a
shade to pick off a ramp plus an alpha to apply, resolved against whichever
ramp the control is tinted with. A component names a state, never a shade.

## Known divergences

Shared token values do not imply identical rendering. The current alignment
pass uses circular corners to match CSS, includes border widths in intrinsic
Button, Badge and Toast sizes, and gives Badge, KeyCap and a Toast's message
explicit single-line boxes. Dialog header and Toast minimum heights include
their padding, as they do with CSS `border-box`, and a Toast is `width:
max-content` capped at its maximum — a `maxWidth` frame alone would take
whatever width the viewport offered. Text controls are edged with the same
hairline as separators.

Remaining visual work includes multiline text metrics, independent shadow
layers, and shared popup presentation: Menu and Tooltip use overlays without
collision avoidance. These need side-by-side rendering checks across every
theme; a successful build is not a visual conformance check.

| What | Why |
| --- | --- |
| A per-theme *shade* on a recipe naming a semantic role | Such a recipe is a computed property reading this instance's roles, so the descriptor itself has no initialiser parameter. The shades a theme re-picks on it (`tinted` reads 700 on paper and 300 on a dark canvas) are stored members of their own — `controlColorTintedContentNormalShade` — which the theme re-points and the computed descriptor reads. |
| Hairlines | A separator is one point at every display scale: `@Environment(\.hairline)` is `strokeHairline` drawn as written, the way `base.css` reads `--stroke-hairline`. Drawn control outlines read `strokeControl` and stay put. |
| The mono face resolves rather than being named | `text-field.css` reaches for `--base-font-code`, whose stack leads with `ui-monospace` — on this platform a request for the system monospaced face rather than a family to look up, so a mono field takes `Font.Design.monospaced`. The same argument as `-apple-system` resolving to `.system` in `TypeStyle`. (`base.font.code` has no semantic token in front of it; a component reaching a `base.*` primitive is the graph's own rule being bent, and it is a `ds` decision to unbend.) |

## Icons

The set is Fluent — Microsoft's official Swift package, the same set
`packages/js/dazzui` pulls from `@fluentui/react-icons`. That sameness is
the point: the React components name specific glyphs, so a port drawing its own
checkmark would be a different mark at a different weight in the same box.

| React component | Glyph |
| --- | --- |
| `checkbox.tsx`, `menu.tsx`, `step-list.tsx` | `Checkmark12Filled` |
| `search-field.tsx`, `toast.tsx` | `Dismiss12Regular` |
| `search-field.tsx` | `Search16Regular` |
| `text-field.tsx`, `date-range-picker.tsx` | `ChevronDown12Regular` |
| `shortcut-recorder.tsx` | `DismissCircle12Filled` |
| `calendar.tsx` | `ChevronLeft12Regular`, `ChevronRight12Regular` |
| `date-range-picker.tsx` | `Calendar16Regular` — the web names `CalendarLtr16Regular`, which the Swift package does not carry; the two differ only in the dots' direction |

`Icon` takes a `FluentIcon`, and the components that carry one — `IconButton`,
`NavItem`, `MenuItem` — take one too. It also keeps an `Image` initialiser, so
a host can draw its own artwork at icon size the way React's `IconGlyph`
accepts any component that renders an `<svg>`.

## Building

**Use `xcodebuild`, not `swift build`:**

```bash
xcodebuild -scheme DazzUI -destination 'platform=macOS' build
```

The icon package ships a 20,000-glyph asset catalog that its own manifest
never declares as a resource. SwiftPM therefore leaves `Bundle.module`
unsynthesised and the dependency fails to compile with
`type 'Bundle' has no member 'module'`; Xcode's build system processes the
catalog into a resource bundle and the package builds. This is an upstream
manifest bug rather than anything this package can configure around, and it is
why the first build after a clean checkout spends several minutes compiling
the catalog.

## Playground

`Example/` holds the gallery app — six pages of components, and a Themes page
that renders the same view under every theme at once:

```bash
open Example/Example.xcodeproj
```

⌘R runs it. The project builds a real `.app`, so it debugs and profiles like
any other Mac app; its README carries the command-line equivalent and what
each page is for.
