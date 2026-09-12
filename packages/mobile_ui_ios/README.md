# DazzUI Mobile

Native SwiftUI controls with the `packages/ds` colour schemes. iOS 16+ (iPhone
and iPad), with macOS 13+ support for package tests and previews. The Swift
module and library product are both `DazzMobileUI`; there are no external
dependencies.

Add this directory as a local Swift package in Xcode and link its library to
your app target:

```swift
import SwiftUI
import DazzMobileUI

struct ContentView: View {
  var body: some View {
    NavigationStack {
      Form {
        Button("Continue") { }
          .buttonStyle(.borderedProminent)
      }
      .navigationTitle("Welcome")
    }
    .mobileTheme(.studio)
  }
}
```

`mobileTheme` applies the ds action colour through SwiftUI's `.tint`, and
publishes the palette as `@Environment(\.mobilePalette)`. Studio, Bright,
Frost, Graphite, Ember and Nocturne each have light and dark palettes. The
modifier follows the system by default; use
`.mobileTheme(.bright, colorScheme: .dark)` to choose an appearance. Place it
on the scene's root view so navigation and presentations inherit it.

Buttons, toggles, pickers, lists, forms, navigation, menus and sheets are
SwiftUI components. Use their native styles, roles and accessibility APIs.
System typography, spacing, geometry, materials, disabled states and destructive
colours stay system-owned. Tint support varies by native component and style.
There are no replacement controls, custom control styles or icon dependencies;
use SF Symbols through `Image(systemName:)`.

For app-owned content, read semantic colours such as
`palette.colorContent` and `palette.colorSurfaceMuted` and apply them with
standard modifiers (`foregroundStyle`, `background`, `listRowBackground`).
System backgrounds and text colours are preserved unless the host explicitly
sets them. No global UIKit appearance proxy is changed.

## Generation and validation

`pnpm tokens` at the repository root regenerates the committed `Generated/`
sources. The SwiftUI generator shares colour conversion, ramp resolution and
mode composition with the desktop port; mobile receives only colours and the
ds filled action's shade. It does not import `DazzUI` or desktop geometry.
Never edit generated files directly.

```sh
cd packages/mobile_ui_ios
swift test
xcodebuild -scheme DazzMobileUI -destination 'generic/platform=iOS Simulator' CODE_SIGNING_ALLOWED=NO build
```

React geometry parity does not apply: native component rendering is deliberately
owned by the OS.

## Example

`Example/Playground.xcodeproj` is a runnable iPhone and iPad app, already linked
to this local package. From the repository root:

```sh
open packages/mobile_ui_ios/Example/Playground.xcodeproj
```

Select the **Playground** scheme and an iPhone or iPad simulator, then press
**⌘R**. No package setup or signing team is needed for the simulator.

The interactive gallery covers all families, system/light/dark appearance,
controls, navigation, menus, an alert and a sheet. It uses system layout and
SF Symbols throughout. See [Example/README.md](Example/README.md) for CLI build
and launch commands.
