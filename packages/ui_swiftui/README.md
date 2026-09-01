# DazzUI

The DazzUI design system for SwiftUI — the platform sibling of
`packages/ui_react` and `packages/ui_flutter`. Same tokens, same four themes,
same components.

**`packages/ui_react` is the standard.** Every name, prop and metric here is
read off the React source; where the two disagree, the React file wins.

## Using it

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

`Generated/` holds what `packages/ds` wrote and is committed, so the package
builds and ships on its own — there is no dependency edge to the design
system and so no build order to get wrong. **Never hand-edit it**: a value a
component needs belongs in `packages/ds`, and anything written here is
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

| `ThemeName`    | Feel |
| -------------- | ---- |
| `.studioLight` | White card, light grey canvas, violet accent. **The baseline** |
| `.studioDark`  | The same skeleton over a near-black canvas |
| `.brightLight` | Warm paper, ink navy, acid-green marker, pill controls |
| `.brightDark`  | Ink-navy canvas, with acid green as the accent itself |

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

Everything not listed here is a value-for-value port.

| What | Why |
| --- | --- |
| A per-theme *shade* on a recipe naming a semantic role | Such a recipe is a computed property reading this instance's roles, so it has no initialiser parameter to re-point. The same limit the Flutter port carries. |
| Hairlines | A separator is one *device* pixel: `@Environment(\.hairline)` halves on Retina, the way `base.css` does with `min-resolution: 2dppx`. Drawn control outlines read `strokeControl` and stay put. |
| The mono face resolves rather than being named | `input.css` reaches for `--base-font-code`, whose stack leads with `ui-monospace` — on this platform a request for the system monospaced face rather than a family to look up, so a mono field takes `Font.Design.monospaced`. The same argument as `-apple-system` resolving to `.system` in `TypeStyle`. (`base.font.code` has no semantic token in front of it; a component reaching a `base.*` primitive is the graph's own rule being bent, and it is a `packages/ds` decision to unbend.) |

## Icons

The set is Fluent — Microsoft's official Swift package, the same set
`packages/ui_react` pulls from `@fluentui/react-icons`. That parity is the
point: the React components name specific glyphs, so a port drawing its own
checkmark would be a different mark at a different weight in the same box.

| React component | Glyph |
| --- | --- |
| `checkbox.tsx`, `menu.tsx`, `step-list.tsx` | `Checkmark12Filled` |
| `search-field.tsx`, `toast.tsx` | `Dismiss12Regular` |
| `search-field.tsx` | `Search16Regular` |
| `input.tsx` | `ChevronDown12Regular` |
| `shortcut-recorder.tsx` | `DismissCircle12Filled` |

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
that renders the same view under all four themes at once:

```bash
xcodebuild -scheme Playground -destination 'platform=macOS' -derivedDataPath .build/xcode build
./.build/xcode/Build/Products/Debug/Playground
```

Both from `Example/`. See its own README for what each page is for.
