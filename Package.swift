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
// npm packages. On Apple platforms, use `xcodebuild`; the icon package's
// asset catalog needs Xcode's build system (see packages/swift/DazzUI/README.md).

import Foundation

import PackageDescription

let buildForWeb = ProcessInfo.processInfo.environment["CROSS_SWIFTUI_WEB"] == "1"
#if os(Windows) || os(Linux)
  let useCrossSwiftUI = true
#else
  let useCrossSwiftUI = buildForWeb
#endif

let nativeAPIPath = ProcessInfo.processInfo.environment["NATIVEAPI_SWIFT_PATH"]
  ?? "../../libnativeapi/nativeapi/bindings/swift"
let package: Package
if useCrossSwiftUI {
  let crossSwiftPath =
    ProcessInfo.processInfo.environment["CROSS_SWIFTUI_PATH"] ?? "../../libnativeapi/crossswift"
  var dependencies: [Package.Dependency] = [.package(name: "CrossSwiftUI", path: crossSwiftPath)]
  var dazzDependencies: [Target.Dependency] = [
    "FluentIcons", .product(name: "SwiftUI", package: "CrossSwiftUI"),
  ]
  if !buildForWeb {
    dependencies.append(.package(name: "NativeAPI", path: nativeAPIPath))
    dazzDependencies.append(.product(name: "NativeAPI", package: "NativeAPI"))
  }
  if buildForWeb {
    dependencies.append(.package(url: "https://github.com/swiftwasm/JavaScriptKit.git", exact: "0.63.0"))
    dazzDependencies.append(.product(name: "JavaScriptKit", package: "JavaScriptKit"))
  }
  var targets: [Target] = [
    .target(
      name: "FluentIcons", dependencies: [.product(name: "SwiftUI", package: "CrossSwiftUI")],
      path: "packages/swift/DazzUI/Windows/FluentIcons", exclude: buildForWeb ? ["Resources"] : [],
      resources: buildForWeb ? [] : [.process("Resources")]),
    .target(
      name: "DazzUI",
      dependencies: dazzDependencies,
      path: "packages/swift/DazzUI/Sources/DazzUI"),
  ]
  if !buildForWeb {
    targets.append(.testTarget(
      name: "DazzUINativeMenuTests", dependencies: ["DazzUI", .product(name: "NativeAPI", package: "NativeAPI")],
      path: "packages/swift/DazzUI/Tests/NativeMenus"))
  }
  #if os(Windows)
    targets.append(.testTarget(
      name: "DazzUIWindowsTests", dependencies: ["DazzUI"],
      path: "packages/swift/DazzUI/Tests/Windows"))
  #endif
  package = Package(
    name: "DazzUI",
    products: [
      .library(name: "DazzUI", targets: ["DazzUI"]),
      .library(name: "FluentIcons", targets: ["FluentIcons"]),
    ],
    dependencies: dependencies,
    targets: targets
  )
} else {
  package = Package(
    name: "DazzUI",
    platforms: [.macOS(.v13), .iOS(.v16)],
    products: [
      .library(name: "DazzUI", targets: ["DazzUI"]),
      .library(name: "DazzMobileUI", targets: ["DazzMobileUI"]),
    ],
    dependencies: [
      .package(name: "NativeAPI", path: nativeAPIPath),
      .package(
        url: "https://github.com/microsoft/fluentui-system-icons.git",
        revision: "21d5d02f724be2aaf586564775fff73a18a76eb6"
      )
    ],
    targets: [
      .target(
        name: "DazzUI",
        dependencies: [
          .product(name: "FluentIcons", package: "fluentui-system-icons"),
          .product(name: "NativeAPI", package: "NativeAPI", condition: .when(platforms: [.macOS]))
        ],
        path: "packages/swift/DazzUI/Sources/DazzUI"
      ),
      .target(
        name: "DazzMobileUI",
        path: "packages/swift/DazzMobileUI/Sources/DazzMobileUI"
      ),
    ]
  )
}
