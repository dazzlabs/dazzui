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
contains the searchable story directory and family/appearance selection.
`GeneratedStories.swift` is generated from the same scene declarations as React
under `packages/js/dazzui/example/stories/`; never edit it by hand. Run
`pnpm codegen` from the repository root after source changes.

The catalog currently includes all 143 declared scenes. The remaining 345 React
stories have not been converted to shared data yet; `codegen:complete` remains
red until every story and every target is covered.

Content, bindings, conditional text, action slots, button assignments and timer
updates come from the declarations. Controls use native SwiftUI: Toggle for both
switch and checkbox semantics, GroupBox for cards/callouts, ProgressView for
progress/spinners, SF Symbols for icons, and a system sheet for Dialog. Mobile
uses native control geometry, Dynamic Type and system styles. Desktop sizes,
radii, surface recipes and minimum grid widths do not override native controls;
the mobile grid has an adaptive screen layout. Badge and key-cap content use
native Text rather than custom drawing. The DS palette is installed by
`mobileTheme`.

`PLAYGROUND_STORY=<stable story ID>` launches an isolated scene for automation.
Navigating away cancels its timers and revisiting creates fresh state.

Run the native suites with a simulator UUID from `xcrun simctl list devices`:

```sh
xcodebuild -project Playground.xcodeproj -scheme Playground \
  -destination 'platform=iOS Simulator,id=<UUID>' \
  -derivedDataPath .build/xcode CODE_SIGNING_ALLOWED=NO test
```

The tests check the generated registry, render all declared stories in all twelve
themes, and execute non-timed input/toggle/dialog checkpoints from the shared JSON.
Text entry uses native paste so invisible Unicode characters survive unchanged.
The UI driver currently requires at most one fill per checkpoint and fails
explicitly if that changes; every declared state expectation must be observed.
Timed checkpoints still need a deterministic native clock; screenshots are smoke
coverage, not cross-platform pixel goldens.

`project.yml` is the project source. After changing project configuration,
regenerate the checked-in project with `xcodegen generate` (XcodeGen 2.46+).
XcodeGen is not needed to open, build or run the checked-in project.
