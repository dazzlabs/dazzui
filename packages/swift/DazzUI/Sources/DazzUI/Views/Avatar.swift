import SwiftUI

/// The three avatar sizes.
///
/// Off the 4px grid rather than off the control profile: an avatar is not a
/// control, and these are the steps a row, a header and a card actually ask
/// for.
public enum AvatarSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A person is a circle and a thing is a square — a project, a repository.
public enum AvatarShape: String, CaseIterable, Sendable {
  case circle
  case square
}

/// The picture that stands for a person or a project.
///
/// What it is really for is the case where there is no picture: the initials
/// are drawn until one has loaded and instead of it when it never does, so a
/// slow or missing image never flashes an empty box.
///
/// It is not interactive and stays off ``Pressable`` — it reads the `tinted`
/// recipe's resting colours directly. An avatar that lit up under the pointer
/// would read as something you can press.
public struct Avatar<Fallback: View>: View {
  /// Who or what this is. It names the avatar to an assistive technology and,
  /// unless `fallback` says otherwise, it is where the initials come from.
  public var name: String
  /// The picture. Until it loads, and if it never does, the fallback shows.
  public var image: Image?
  public var size: AvatarSize
  public var shape: AvatarShape
  public var tint: Tint
  @ViewBuilder public var fallback: () -> Fallback

  @Environment(\.theme) private var theme
  @Environment(\.avatarRing) private var ring

  public init(
    name: String,
    image: Image? = nil,
    size: AvatarSize = .medium,
    shape: AvatarShape = .circle,
    tint: Tint = .primary,
    @ViewBuilder fallback: @escaping () -> Fallback
  ) {
    self.name = name
    self.image = image
    self.size = size
    self.shape = shape
    self.tint = tint
    self.fallback = fallback
  }

  /// Initials: the first letter of the first word and of the last, which is
  /// the pair a reader actually recognises. One word gives one letter rather
  /// than two off the same word, since "Ac" for Acme reads as a truncation
  /// rather than as a monogram.
  public static func initials(of name: String) -> String {
    let words = name.split(whereSeparator: \.isWhitespace)
    guard let first = words.first?.first else { return "" }
    let last = words.count > 1 ? words.last?.first : nil
    return String([first, last].compactMap { $0 }).uppercased()
  }

  /// The box, per size. ``AvatarGroup`` measures its overlap off the same
  /// three steps, and has no other way to know them.
  static func dimension(_ vars: ThemeVariables, _ size: AvatarSize) -> CGFloat {
    switch size {
    case .small: return vars.spacing5
    case .medium: return vars.spacing7
    case .large: return vars.spacing9
    }
  }

  public var body: some View {
    let vars = theme.vars
    let side = Self.dimension(vars, size)

    return fallback()
      // The fallback fills the whole box rather than sitting behind the
      // picture, so a transparent image lands on the tint rather than on
      // whatever is behind the avatar.
      .frame(width: side, height: side)
      .background(recipe(.surface))
      .foregroundStyle(recipe(.content))
      .overlay {
        if let image {
          image.resizable().scaledToFill()
        }
      }
      .clipShape(RoundedRectangle(cornerRadius: corner, style: .circular))
      // The cut a group makes between neighbours, drawn *outside* the box: a
      // background takes no part in layout, so the negative padding grows the
      // ring past the avatar's edge without growing the avatar. A border
      // would eat into the picture instead.
      .background {
        if let ring {
          RoundedRectangle(cornerRadius: corner + vars.strokeControl, style: .circular)
            .fill(ring)
            .padding(-vars.strokeControl)
        }
      }
      .accessibilityElement()
      .accessibilityLabel(name)
      .accessibilityAddTraits(.isImage)
  }

  /// It is not interactive, so it reads the resting recipe directly rather
  /// than joining the shared state machine.
  private func recipe(_ part: ControlPart) -> Color {
    theme.vars.resolve(theme.vars.descriptor(.tinted, part), tint: tint, state: .normal)
  }

  /// The square takes the container corner rather than the control corner. An
  /// avatar is a thing rather than something you press, and under Bright —
  /// where the control corner is a pill — a control-cornered square would go
  /// round and stop being the shape that says "project".
  private var corner: CGFloat {
    switch shape {
    case .circle: return theme.vars.radiusFull
    case .square: return theme.vars.radiusMedium
    }
  }
}

public extension Avatar where Fallback == AvatarInitials {
  /// The common avatar: initials drawn from the name.
  init(
    _ name: String,
    image: Image? = nil,
    size: AvatarSize = .medium,
    shape: AvatarShape = .circle,
    tint: Tint = .primary
  ) {
    self.init(
      name: name,
      image: image,
      size: size,
      shape: shape,
      tint: tint,
      fallback: { AvatarInitials(name: name, size: size) }
    )
  }
}

/// The two letters an avatar falls back to.
///
/// The type moves one step of the scale per size — 11, 12, 13 — rather than
/// tracking the box, which grows nearly twice as fast: two letters at a third
/// of a 36px circle would fill it edge to edge.
public struct AvatarInitials: View {
  public var name: String
  public var size: AvatarSize

  @Environment(\.theme) private var theme

  public init(name: String, size: AvatarSize = .medium) {
    self.name = name
    self.size = size
  }

  public var body: some View {
    let vars = theme.vars
    let face: TypeStyle = {
      switch size {
      case .small: return vars.captionSmall
      case .medium: return vars.labelMedium
      case .large: return vars.labelLarge
      }
    }()

    return Text(Avatar<AvatarInitials>.initials(of: name))
      .font(.system(size: face.size, weight: vars.labelStrong.weight))
  }
}

/// A run of avatars, overlapped.
///
/// Each one is cut out of the one before it, so the stack reads as a group
/// rather than as a smear. The cut is drawn as a ring the avatar sits inside
/// rather than a border on it, so the box each avatar measures is unchanged —
/// the same thing the stylesheet's `box-shadow: 0 0 0 1.5px` does.
public struct AvatarGroup<Content: View>: View {
  /// The size the avatars inside are drawn at. The group does not set it on
  /// them — each one carries its own — but the overlap, the ring and the count
  /// chip are measured from it, and they have no other way to know.
  public var size: AvatarSize
  public var shape: AvatarShape
  /// How many were left out, drawn as a count at the end of the run.
  public var overflow: Int
  /// The colour each avatar is cut out of its neighbour with. Defaults to the
  /// paper a group usually sits on; a group on a card passes the card's own
  /// surface.
  public var ring: Color?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    size: AvatarSize = .medium,
    shape: AvatarShape = .circle,
    overflow: Int = 0,
    ring: Color? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.size = size
    self.shape = shape
    self.overflow = overflow
    self.ring = ring
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    // A third of the box, on every size — a fixed overlap would swallow a
    // small avatar and barely touch a large one. Negative spacing is what
    // SwiftUI gives instead of a negative margin.
    let overlap: CGFloat = {
      switch size {
      case .small: return vars.spacing15
      case .medium: return vars.spacing2
      case .large: return vars.spacing3
      }
    }()

    return HStack(spacing: -overlap) {
      content()
      if overflow > 0 {
        // The overflow count wears the same box in the neutral groove rather
        // than a tint: it is a number, not a person.
        Text("+\(overflow)")
          .font(.system(size: countSize, weight: vars.labelStrong.weight))
          .foregroundStyle(vars.colorContentMuted)
          .frame(width: side, height: side)
          .background(vars.colorSurfaceSunken)
          .clipShape(RoundedRectangle(cornerRadius: corner, style: .circular))
      }
    }
    .environment(\.avatarRing, ring ?? vars.colorSurface)
  }

  private var side: CGFloat { Avatar<AvatarInitials>.dimension(theme.vars, size) }

  private var corner: CGFloat {
    shape == .circle ? theme.vars.radiusFull : theme.vars.radiusMedium
  }

  private var countSize: CGFloat {
    size == .large ? theme.vars.labelMedium.size : theme.vars.captionSmall.size
  }
}

private struct AvatarRingKey: EnvironmentKey {
  static let defaultValue: Color? = nil
}

public extension EnvironmentValues {
  /// The colour a grouped avatar is cut out of its neighbour with. The group
  /// sets it; an avatar outside one has none and draws no ring.
  var avatarRing: Color? {
    get { self[AvatarRingKey.self] }
    set { self[AvatarRingKey.self] = newValue }
  }
}
