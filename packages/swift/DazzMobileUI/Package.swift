// swift-tools-version: 5.9
import PackageDescription

let package = Package(
  name: "DazzMobileUI",
  platforms: [.iOS(.v16), .macOS(.v13)],
  products: [.library(name: "DazzMobileUI", targets: ["DazzMobileUI"])],
  targets: [
    .target(name: "DazzMobileUI"),
    .testTarget(name: "DazzMobileUITests", dependencies: ["DazzMobileUI"]),
  ]
)
