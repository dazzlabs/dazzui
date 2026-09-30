import FluentIcons
import SwiftUI

/// Two densities, both under control height: a tree is a column of names rather
/// than a column of controls, and a row at control height would fit six fewer
/// files on the screen for nothing.
public enum TreeSize: String, CaseIterable, Sendable {
  case small
  case medium
}

/// One entry in a ``Tree``.
public struct TreeNode: Identifiable {
  public let id: String
  public var label: String
  public var icon: FluentIcon?
  /// A trailing aside: a count, a change marker.
  public var meta: String?
  /// Non-nil, empty included, marks the node as a branch. An empty array is how
  /// an empty folder still draws a twisty and still opens, which is the
  /// difference between "nothing in here" and "not a folder".
  public var children: [TreeNode]?
  public var enabled: Bool

  public init(
    id: String,
    label: String,
    icon: FluentIcon? = nil,
    meta: String? = nil,
    children: [TreeNode]? = nil,
    enabled: Bool = true
  ) {
    self.id = id
    self.label = label
    self.icon = icon
    self.meta = meta
    self.children = children
    self.enabled = enabled
  }

  public var isBranch: Bool { children != nil }
}

/// One visible row: a node, and how deep it sits.
private struct TreeRow: Identifiable {
  let id: String
  let node: TreeNode
  let level: Int
  let open: Bool
}

/// A hierarchy the reader walks.
///
/// React's is the one component in that package with no Base UI part under it,
/// because a tree's keyboard is not a list's. SwiftUI hands focus and the arrow
/// keys to the platform, so what is here is the part the design decides — the
/// indent, the twisty's column, the selection's wash and weight.
///
/// A branch and a leaf answer a click differently on purpose. Clicking a folder
/// opens it *and* selects it, because in a file tree those are one gesture;
/// clicking the twisty alone opens it and leaves the selection where it was,
/// which is how a reader looks inside a folder without losing the file they
/// were reading.
public struct Tree: View {
  public var items: [TreeNode]
  public var expanded: [String]
  public var selected: String?
  public var size: TreeSize
  /// Which ramp the selected row is washed with. The hover comes off the
  /// `normal` recipe either way — that one reads the theme's surface roles
  /// rather than a ramp, which is what keeps a row's hover neutral while its
  /// selection carries the accent.
  public var tint: Tint
  public var onSelect: ((String) -> Void)?
  public var onToggle: ((String) -> Void)?

  @Environment(\.theme) private var theme

  public init(
    items: [TreeNode],
    expanded: [String] = [],
    selected: String? = nil,
    size: TreeSize = .medium,
    tint: Tint = .primary,
    onSelect: ((String) -> Void)? = nil,
    onToggle: ((String) -> Void)? = nil
  ) {
    self.items = items
    self.expanded = expanded
    self.selected = selected
    self.size = size
    self.tint = tint
    self.onSelect = onSelect
    self.onToggle = onToggle
  }

  public var body: some View {
    VStack(alignment: .leading, spacing: 0) {
      ForEach(rows) { row in
        Row(
          node: row.node,
          level: row.level,
          open: row.open,
          selected: selected == row.id,
          size: size,
          tint: tint,
          onSelect: { onSelect?(row.id) },
          onToggle: { onToggle?(row.id) }
        )
      }
    }
    .frame(maxWidth: .infinity, alignment: .leading)
  }

  /// The visible rows, in the order the eye reads them.
  private var rows: [TreeRow] {
    var out: [TreeRow] = []
    func walk(_ nodes: [TreeNode], _ level: Int) {
      for node in nodes {
        let open = node.isBranch && expanded.contains(node.id)
        out.append(TreeRow(id: node.id, node: node, level: level, open: open))
        if open, let children = node.children {
          walk(children, level + 1)
        }
      }
    }
    walk(items, 1)
    return out
  }
}

private struct Row: View {
  let node: TreeNode
  let level: Int
  let open: Bool
  let selected: Bool
  let size: TreeSize
  let tint: Tint
  let onSelect: () -> Void
  let onToggle: () -> Void

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Pressable(
      action: {
        onSelect()
        if node.isBranch { onToggle() }
      },
      enabled: node.enabled,
      cornerRadius: vars.radiusSmall
    ) { state in
      HStack(spacing: 0) {
        Twisty(branch: node.isBranch, open: open, enabled: node.enabled, action: onToggle)
        if let icon = node.icon {
          Icon(icon, size: vars.iconMedium)
            .foregroundStyle(selected ? content(state) : vars.colorContentSubtle)
            .padding(.leading, vars.spacing1)
        }
        Text(node.label)
          .lineLimit(1)
          .truncationMode(.tail)
          .padding(.leading, vars.spacing1)
          .frame(maxWidth: .infinity, alignment: .leading)
        if let meta = node.meta {
          Text(meta)
            .font(vars.captionSmall.font)
            .foregroundStyle(selected ? content(state) : vars.colorContentFaint)
            .padding(.leading, vars.spacing2)
        }
      }
      .font(face)
      .foregroundStyle(content(state))
      // One step per level, plus the row's own leading inset. The twisty column
      // is inside the indent, which is what keeps the glyphs of one level in a
      // straight line.
      .padding(.leading, vars.spacing1 + CGFloat(level - 1) * vars.spacing4)
      .padding(.trailing, vars.spacing2)
      .frame(height: height)
      .background(surface(state))
      .clipShape(RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular))
      .animation(vars.motion, value: state.controlState)
    }
    .accessibilityAddTraits(selected ? [.isButton, .isSelected] : .isButton)
  }

  private var height: CGFloat {
    size == .small ? theme.vars.spacing6 : theme.vars.spacing7
  }

  private var face: Font {
    let vars = theme.vars
    let step = size == .small ? vars.labelSmall.size : vars.labelQuiet.size
    // Selection shifts weight as well as colour, the way every selected thing
    // in this system does. On a column of forty rows the weight is doing most
    // of the work — one grade of colour is hardest to find in exactly this
    // shape.
    return .system(
      size: step,
      weight: selected ? vars.labelMedium.weight : vars.labelQuiet.weight
    )
  }

  private func surface(_ state: PressableState) -> Color {
    let vars = theme.vars
    if selected {
      return vars.resolve(vars.descriptor(.tinted, .surface), tint: tint, state: .normal)
    }
    guard state.enabled, state.hovered || state.pressed else { return .clear }
    // The neutral control's wash — the card step — which is what a menu item, a
    // table row and a tree row all want. `plain` would come off the tinted ramp
    // and sit heavier than the selection one rung below it.
    return vars.resolve(vars.descriptor(.normal, .surface), tint: .neutral, state: .hovered)
  }

  private func content(_ state: PressableState) -> Color {
    let vars = theme.vars
    if selected {
      return vars.resolve(vars.descriptor(.tinted, .content), tint: tint, state: .normal)
    }
    if !node.enabled { return vars.colorContentFaint }
    return state.hovered ? vars.colorContent : vars.colorContentNav
  }
}

/// The twisty keeps its column on a leaf as well as a branch, empty.
///
/// Files and folders at one level start at one x, which is what makes a deep
/// tree scannable — a leaf that reclaimed the space would put every filename at
/// a different indent from its siblings.
private struct Twisty: View {
  let branch: Bool
  let open: Bool
  let enabled: Bool
  let action: () -> Void

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Group {
      if branch {
        Icon(.chevronRight12Regular, size: vars.iconSmall)
          .rotationEffect(.degrees(open ? 90 : 0))
          .foregroundStyle(vars.colorContentFaint)
          .animation(vars.motion, value: open)
          .contentShape(Rectangle())
          // The twisty is the one target that opens without selecting, so its
          // click must not reach the row behind it.
          .onTapGesture { if enabled { action() } }
      } else {
        Color.clear
      }
    }
    .frame(width: vars.spacing4, height: vars.spacing4)
  }
}
