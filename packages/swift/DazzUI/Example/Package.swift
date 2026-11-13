// swift-tools-version: 5.9
//
// The SwiftUI example, the counterpart of `packages/dart/dazzui/example`.
//
// It is a package of its own rather than a target of the library, for the same
// reason the Flutter example is a separate app: the library must not carry a
// dependency, an entry point, or a window that only the example needs.

import PackageDescription

var targets: [Target] = [.executableTarget(name: "Example", dependencies: ["DazzUI"])]

#if os(Windows)
  targets.append(
    .testTarget(name: "GalleryTests", dependencies: ["Example"], path: "Tests/Windows"))
#endif
let package = Package(
  name: "Example",
  platforms: [.macOS(.v13)],
  dependencies: [
    .package(name: "DazzUI", path: "..")
  ],
  targets: targets
)
