// swift-tools-version: 5.9
//
// The repository's Swift packages, for a host that depends on DazzUI by its
// GitHub URL: SwiftPM reads a manifest only at a repository's root, so the
// packages under `packages/swift/` are re-exported here with their own
// sources. Each package's own manifest is still the one to work in — its
// playground, its tests and its README — and this file follows it: a target,
// a platform or a dependency changed there is changed here too.
//
//     .package(url: "https://github.com/dazzlabs/dazzui.git", revision: "<commit>")
//
// Pin a revision rather than a version: the repository's tags belong to the
// npm packages. Build with `xcodebuild`, not `swift build` — the icon package's
// asset catalog needs Xcode's build system (see packages/swift/DazzUI/README.md).

import Foundation

import PackageDescription

#if os(Windows)
  let portPath =
    ProcessInfo.processInfo.environment["SWIFTUI_PORT_PATH"] ?? "../../libnativeapi/swiftui-port"
  let package = Package(
    name: "DazzUI",
    products: [
      .library(name: "DazzUI", targets: ["DazzUI"]),
      .library(name: "FluentIcons", targets: ["FluentIcons"]),
    ],
    dependencies: [.package(name: "SwiftUIPort", path: portPath)],
    targets: [
      .target(
        name: "FluentIcons", dependencies: [.product(name: "SwiftUI", package: "SwiftUIPort")],
        path: "packages/swift/DazzUI/Windows/FluentIcons", resources: [.process("Resources")]),
      .target(
        name: "DazzUI",
        dependencies: ["FluentIcons", .product(name: "SwiftUI", package: "SwiftUIPort")],
        path: "packages/swift/DazzUI/Sources/DazzUI"),
      .testTarget(
        name: "DazzUIWindowsTests", dependencies: ["DazzUI"],
        path: "packages/swift/DazzUI/Tests/Windows"),
    ]
  )
#else
  let package = Package(
    name: "DazzUI",
    platforms: [.macOS(.v13), .iOS(.v16)],
    products: [
      .library(name: "DazzUI", targets: ["DazzUI"]),
      .library(name: "DazzMobileUI", targets: ["DazzMobileUI"]),
    ],
    dependencies: [
      .package(
        url: "https://github.com/microsoft/fluentui-system-icons.git",
        revision: "21d5d02f724be2aaf586564775fff73a18a76eb6"
      )
    ],
    targets: [
      .target(
        name: "DazzUI",
        dependencies: [
          .product(name: "FluentIcons", package: "fluentui-system-icons")
        ],
        path: "packages/swift/DazzUI/Sources/DazzUI"
      ),
      .target(
        name: "DazzMobileUI",
        path: "packages/swift/DazzMobileUI/Sources/DazzMobileUI"
      ),
    ]
  )

#endif
