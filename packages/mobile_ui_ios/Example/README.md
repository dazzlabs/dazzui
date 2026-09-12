# iOS playground

A runnable iPhone and iPad app using the adjacent `DazzMobileUI` package.
The Xcode project and shared **Playground** scheme are committed, so Xcode is
all you need to run it.

From this directory:

```sh
open Playground.xcodeproj
```

Select the **Playground** scheme, choose an iPhone or iPad simulator and press
**⌘R**. To run on a physical device, select your development team in the app
target's Signing & Capabilities settings.

## Command line

Build for the simulator:

```sh
xcodebuild -project Playground.xcodeproj -scheme Playground \
  -destination 'generic/platform=iOS Simulator' \
  -derivedDataPath .build CODE_SIGNING_ALLOWED=NO build
```

With one simulator booted, install and launch the app:

```sh
open -a Simulator
xcrun simctl install booted .build/Build/Products/Debug-iphonesimulator/DazzMobilePlayground.app
xcrun simctl launch booted dev.dazzlabs.dazzui.mobile.playground
```

If no simulator is booted, choose a device from Simulator's **File → Open
Simulator** menu first. If several are booted, replace `booted` with the desired
device UUID from `xcrun simctl list devices booted`.

## Contents

`PlaygroundApp.swift` provides the App entry point. `NativeControls.swift`
contains the interactive gallery and an Xcode preview. The family and
appearance pickers exercise all twelve ds palettes; controls, navigation,
menus and presentations use native SwiftUI.

`project.yml` is the project source. After changing project configuration,
regenerate the checked-in project with `xcodegen generate` (XcodeGen 2.46+).
XcodeGen is not needed to open, build or run the checked-in project.
