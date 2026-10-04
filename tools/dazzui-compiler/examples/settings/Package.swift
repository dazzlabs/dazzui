// swift-tools-version: 5.9
//
// The Apple host for the settings example: the SwiftUI that `dazzui-compiler
// generate --target swiftui` wrote into `generated/apple`, compiled against
// the real DazzUI, and a window to run it in. It sits beside Cargo.toml
// because a SwiftPM target cannot reach outside its package.
//
// Build with xcodebuild, as DazzUI itself is (see its README):
//
//   xcodebuild -scheme SettingsExample -destination 'platform=macOS' build
import PackageDescription

let package = Package(
  name: "SettingsExample",
  platforms: [.macOS(.v13)],
  dependencies: [
    .package(path: "../../../../packages/swift/DazzUI")
  ],
  targets: [
    .target(
      name: "SettingsUI",
      dependencies: [.product(name: "DazzUI", package: "DazzUI")],
      path: "generated/apple"
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
