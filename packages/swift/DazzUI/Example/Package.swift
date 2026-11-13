// swift-tools-version: 5.9
//
// The SwiftUI example, the counterpart of `packages/dart/dazzui/example`.
//
// It is a package of its own rather than a target of the library, for the same
// reason the Flutter example is a separate app: the library must not carry a
// dependency, an entry point, or a window that only the example needs.

import Foundation
import PackageDescription

let buildForWeb = ProcessInfo.processInfo.environment["SWIFTUI_PORT_WEB"] == "1"
var dependencies: [Package.Dependency] = [.package(name: "DazzUI", path: "..")]
var exampleDependencies: [Target.Dependency] = ["DazzUI"]
if buildForWeb {
  dependencies.append(.package(url: "https://github.com/swiftwasm/JavaScriptKit.git", exact: "0.63.0"))
  exampleDependencies.append(.product(name: "JavaScriptKit", package: "JavaScriptKit"))
}
var targets: [Target] = [
  .executableTarget(
    name: "Example", dependencies: exampleDependencies,
    linkerSettings: [
      .unsafeFlags(["-Xlinker", "-z", "-Xlinker", "stack-size=16777216"], .when(platforms: [.wasi]))
    ])
]

#if os(Windows) || os(Linux)
  #if os(Linux)
    let platformExcludes = ["NativeGalleryTests.swift"]
  #else
    let platformExcludes: [String] = []
  #endif
  targets.append(.testTarget(
    name: "GalleryTests", dependencies: ["Example"], path: "Tests/Windows",
    exclude: platformExcludes))
#endif
let package = Package(
  name: "Example",
  platforms: [.macOS(.v13)],
  dependencies: dependencies,
  targets: targets
)
