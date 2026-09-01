// swift-tools-version: 5.9
//
// The playground app, the SwiftUI counterpart of `packages/ui_flutter/example`.
//
// It is a package of its own rather than a target of the library, for the same
// reason the Flutter example is a separate app: the library must not carry a
// dependency, an entry point, or a window that only the playground needs.
import PackageDescription

let package = Package(
  name: "Playground",
  platforms: [.macOS(.v13)],
  dependencies: [
    .package(name: "DazzUI", path: "..")
  ],
  targets: [
    .executableTarget(
      name: "Playground",
      dependencies: ["DazzUI"]
    )
  ]
)
