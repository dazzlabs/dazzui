// swift-tools-version: 5.9
//
// The Apple host for the settings example: the SwiftUI that `dazzui-compiler
// generate --target swiftui` wrote into `generated/apple`, compiled against
// the real DazzUI and linked to the Rust app core, and a window to run it
// in. It sits beside Cargo.toml because a SwiftPM target cannot reach
// outside its package.
//
// The Rust core is a static library Cargo builds; SwiftPM does not build it.
// Build it first, then the host, as DazzUI itself is built (see its README):
//
//   cargo build -p settings-example
//   xcodebuild -scheme SettingsExample -destination 'platform=macOS' build
import PackageDescription

/// What `cargo build -p settings-example` writes. Named by its full path
/// rather than with `-l`, which would pick the `cdylib` beside it.
let rustCore = Context.packageDirectory + "/../../target/debug/libsettings_example.a"

let package = Package(
  name: "SettingsExample",
  platforms: [.macOS(.v13)],
  dependencies: [
    .package(path: "../../../../packages/swift/DazzUI")
  ],
  targets: [
    .systemLibrary(
      name: "CSettings",
      path: "generated/apple/CSettings"
    ),
    .target(
      name: "SettingsUI",
      dependencies: ["CSettings", .product(name: "DazzUI", package: "DazzUI")],
      path: "generated/apple",
      exclude: ["CSettings"],
      linkerSettings: [.unsafeFlags([rustCore])]
    ),
    .executableTarget(
      name: "SettingsHost",
      dependencies: ["SettingsUI", .product(name: "DazzUI", package: "DazzUI")],
      path: "apple/SettingsHost"
    ),
    .testTarget(
      name: "SettingsUITests",
      dependencies: ["SettingsUI"],
      path: "apple/SettingsUITests"
    ),
  ]
)
