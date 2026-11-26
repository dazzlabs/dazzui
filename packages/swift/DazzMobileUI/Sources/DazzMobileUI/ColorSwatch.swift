import SwiftUI

/// A ds colour ramp, indexed by the role's shade.
public struct ColorSwatch: Sendable, Equatable {
  private final class Storage: Sendable {
    let shades: [Int: Color]
    let s50: Color?
    let s100: Color?
    let s200: Color?
    let s300: Color?
    let s400: Color?
    let s500: Color?
    let s600: Color?
    let s700: Color?
    let s800: Color?
    let s900: Color?
    let s950: Color?

    init(_ shades: [Int: Color]) {
      self.shades = shades
      self.s50 = shades[50]
      self.s100 = shades[100]
      self.s200 = shades[200]
      self.s300 = shades[300]
      self.s400 = shades[400]
      self.s500 = shades[500]
      self.s600 = shades[600]
      self.s700 = shades[700]
      self.s800 = shades[800]
      self.s900 = shades[900]
      self.s950 = shades[950]
    }
  }

  private let storage: Storage

  public var shades: [Int: Color] { storage.shades }

  public init(_ shades: [Int: Color]) {
    self.storage = Storage(shades)
  }

  public subscript(shade: Int) -> Color {
    switch shade {
    case 50: return storage.s50 ?? .clear
    case 100: return storage.s100 ?? .clear
    case 200: return storage.s200 ?? .clear
    case 300: return storage.s300 ?? .clear
    case 400: return storage.s400 ?? .clear
    case 500: return storage.s500 ?? .clear
    case 600: return storage.s600 ?? .clear
    case 700: return storage.s700 ?? .clear
    case 800: return storage.s800 ?? .clear
    case 900: return storage.s900 ?? .clear
    case 950: return storage.s950 ?? .clear
    default: return storage.shades[shade] ?? .clear
    }
  }

  public static func == (lhs: ColorSwatch, rhs: ColorSwatch) -> Bool {
    lhs.storage === rhs.storage || lhs.storage.shades == rhs.storage.shades
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
