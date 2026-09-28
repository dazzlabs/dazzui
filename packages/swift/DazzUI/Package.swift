// swift-tools-version: 5.9
//
// The DazzUI design system for SwiftUI — the platform sibling of
// `packages/js/dazzui-react` and `packages/dart/dazzui`. Same tokens, same four
// themes, same components.
//
// The icon set is Microsoft's official Swift package, which is the same set
// `packages/js/dazzui-react` pulls from `@fluentui/react-icons`: the React
// components name specific glyphs, so a port that drew its own would be a
// different mark at a different weight in the same box.
//
// It is pinned by revision because the repository's tags belong to the npm
// packages rather than to the Swift one, so a host carrying both dependencies
// resolves a single copy.
//
// **Build with `xcodebuild`, not `swift build`.** The icon package ships a
// 20,000-glyph asset catalog its own manifest never declares as a resource,
// so SwiftPM alone leaves `Bundle.module` unsynthesised and the dependency
// fails to compile. Xcode's build system processes the catalog and the
// package builds. See the README.
import PackageDescription

let package = Package(
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
