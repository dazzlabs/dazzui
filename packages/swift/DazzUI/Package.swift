// swift-tools-version: 5.9
//
// The DazzUI design system for SwiftUI — the platform sibling of
// `packages/js/dazzui` and `packages/dart/dazzui`. Same tokens, same four
// themes, same components.
//
// The icon set is Microsoft's official Swift package, which is the same set
// `packages/js/dazzui` pulls from `@fluentui/react-icons`: the React
// components name specific glyphs, so a port that drew its own would be a
// different mark at a different weight in the same box.
//
// It is pinned by revision because the repository's tags belong to the npm
// packages rather than to the Swift one, so a host carrying both dependencies
// resolves a single copy.
//
// On Apple platforms, build with `xcodebuild`. The icon package ships a
// 20,000-glyph asset catalog its own manifest never declares as a resource,
// so SwiftPM alone leaves `Bundle.module` unsynthesised and the dependency
// fails to compile. Xcode's build system processes the catalog and the
// package builds. See the README.

import Foundation

import PackageDescription

let buildForWeb = ProcessInfo.processInfo.environment["SWIFTUI_PORT_WEB"] == "1"
#if os(Windows) || os(Linux)
  let useSwiftUIPort = true
#else
  let useSwiftUIPort = buildForWeb
#endif

let package: Package
if useSwiftUIPort {
  let portPath =
    ProcessInfo.processInfo.environment["SWIFTUI_PORT_PATH"]
    ?? "../../../../../libnativeapi/swiftui-port"
  var dependencies: [Package.Dependency] = [.package(name: "SwiftUIPort", path: portPath)]
  var dazzDependencies: [Target.Dependency] = [
    "FluentIcons", .product(name: "SwiftUI", package: "SwiftUIPort"),
  ]
  if buildForWeb {
    dependencies.append(.package(url: "https://github.com/swiftwasm/JavaScriptKit.git", exact: "0.63.0"))
    dazzDependencies.append(.product(name: "JavaScriptKit", package: "JavaScriptKit"))
  }
  var targets: [Target] = [
    .target(
      name: "FluentIcons", dependencies: [.product(name: "SwiftUI", package: "SwiftUIPort")],
      path: "Windows/FluentIcons", exclude: buildForWeb ? ["Resources"] : [],
      resources: buildForWeb ? [] : [.process("Resources")]),
    .target(
      name: "DazzUI",
      dependencies: dazzDependencies,
      path: "Sources/DazzUI"),
  ]
  #if os(Windows)
    targets.append(.testTarget(
      name: "DazzUIWindowsTests", dependencies: ["DazzUI"], path: "Tests/Windows"))
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
      .library(name: "DazzUI", targets: ["DazzUI"])
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
        ]
      )
    ]
  )
}
