import SwiftUI

/// A ds colour ramp, indexed by the role's shade.
public struct ColorSwatch: Sendable, Equatable {
  public let shades: [Int: Color]

  public init(_ shades: [Int: Color]) {
    self.shades = shades
  }

  public subscript(shade: Int) -> Color {
    shades[shade] ?? .clear
  }
}

extension Color {
  // Internal so importing both DazzUI packages does not add competing APIs.
  @usableFromInline
  init(argb: UInt32) {
    self.init(
      .sRGB,
      red: Double((argb >> 16) & 0xff) / 255,
      green: Double((argb >> 8) & 0xff) / 255,
      blue: Double(argb & 0xff) / 255,
      opacity: Double((argb >> 24) & 0xff) / 255
    )
  }
}
