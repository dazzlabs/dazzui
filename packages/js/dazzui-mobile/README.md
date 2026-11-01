# @dazzlabs/dazzui-mobile

React components drawn as iOS and Android draw theirs, in the DazzUI
colours. It is the web counterpart of `DazzMobileUI` and `dazzui-mobile`:
those two hand the OS a ds family and let it draw the controls; this one
has no OS to hand anything to, so it draws the controls itself — Liquid
Glass as iOS 27 draws it, Material 3 Expressive as Android does — and
takes only colour from ds.

Every value it draws with is decided in one of two places. The colours are
`ds`'s, generated into `src/generated/mobile.css` and committed, so the
package builds and installs on its own. The geometry, type and motion are
the platforms' own, transcribed once each into `src/styles/ios.css` and
`src/styles/android.css` from the Human Interface Guidelines and
m3.material.io; nothing in either file is a design decision of DazzUI's.

## Using it

Import both stylesheets — the colour sheet is separate on purpose, because
it is the contract, not an implementation detail:

```ts
import '@dazzlabs/dazzui-mobile/mobile.css'
import '@dazzlabs/dazzui-mobile/components.css'
```

Install a theme at the root of the screen, as `mobileTheme` and
`DazzTheme` are installed in the native packages:

```tsx
import {
  Button,
  List,
  ListItem,
  ListSection,
  MobileTheme,
  NavigationBar,
  Screen,
  Switch,
} from '@dazzlabs/dazzui-mobile'

<MobileTheme family="frost">
  <Screen bar={<NavigationBar title="Settings" large />}>
    <List>
      <ListSection header="General">
        <ListItem title="Wi-Fi" trailing={<Switch defaultChecked />} />
        <ListItem title="Bluetooth" value="On" onClick={open} />
      </ListSection>
    </List>
    <Button variant="filled" size="large" fullWidth>
      Continue
    </Button>
  </Screen>
</MobileTheme>
```

`MobileTheme` takes a `family` — Bright (the default), Studio, Frost,
Graphite, Ember, Nocturne, and the imported ones — and an `appearance` that
follows the OS unless it is set to `light` or `dark`. Its `platform` is read off the user
agent by default; pass `ios` or `android` to choose. Every component reads
the platform from the theme around it and carries it as `data-platform`,
which is all the stylesheets select on, so one screen written once is
drawn as either OS. A `contained` theme keeps sheets, alerts, menus and
toasts inside its own element rather than over the viewport, which is what
a device frame in a gallery wants.

Behaviour is [Base UI](https://base-ui.com)'s, as it is in the desktop
package: the switch, the slider, the sheet's focus trap, the menu's
type-ahead, the select's hidden input. What this package adds is the
drawing. Content glyphs come from whatever set the host uses — SF Symbols
and Material Symbols are the platforms' and cannot ship here — and the few
glyphs the kit draws itself (a chevron, a check, a clear button) are
strokes drawn to read as either.

## Building

```bash
pnpm build
```

`dist/` is not committed; `prepare` runs the same build on install. After
editing a token, regenerate first — from the workspace root, `pnpm packages`
builds the tokens and both React packages in order.

## Example

`example/` is the mobile example: the desktop example's window — the story
list, the toolbar, the theme panel on the right — drawn with
`@dazzlabs/dazzui`, with an iPhone and a Pixel standing on the desk
in the middle, running the same story under the family the panel chose.
The panel picks the mode (System, Light or Dark — System being the half a
phone picks on its own), which devices show, and the family; the window
follows the same choice, so the whole page redraws in the theme. From the
workspace root:

```bash
pnpm mobile
```

It serves on `:6007`. The address carries the whole state — `?family=`,
`?mode=`, `?platforms=` and `#/story` — so a link is a complete
description of what it shows.

The two token sheets share the page without meeting: the desktop sheet
owns `:root` and `data-theme`, and the mobile sheet writes nothing there —
every value it sets lives on the element `MobileTheme` renders, under
`data-family`.

## Conventions

- One `<name>.tsx` / `<name>.css` pair per component, both listed in their
  registries (`src/components/index.ts`, `src/styles/components.css`).
- A component's root carries `data-platform`, and its stylesheet is two
  blocks: what iOS draws, what Android draws. Shared layout comes first
  and reads only `--dzm-<component>-*` variables the two blocks set.
- A colour is one of `--color-*` (the ds roles and accents) or `--m3-*`
  (the Material roles the Compose plugin derives). A length, a face or a
  curve is one of `--ios-*` or `--m3-*` from the platform sheets. Never a
  literal in a component stylesheet, except the specular white and the
  shadow black that are the glass itself.
- Never hand-edit `src/generated/`; the next generator run overwrites it.
