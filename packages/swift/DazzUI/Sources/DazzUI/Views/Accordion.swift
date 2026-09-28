import FluentIcons
import SwiftUI

/// One section of an ``Accordion``: the header that opens it, and everything
/// it discloses.
public struct AccordionItem: Identifiable {
  /// Identifies the section. It is what ``Accordion/open`` holds, so it
  /// survives a re-order where an index would not.
  public var id: String
  public var title: String
  public var icon: FluentIcon?
  public var meta: AnyView?
  public var disabled: Bool
  public var content: AnyView

  public init<Content: View>(
    _ id: String,
    title: String,
    icon: FluentIcon? = nil,
    meta: AnyView? = nil,
    disabled: Bool = false,
    @ViewBuilder content: () -> Content
  ) {
    self.id = id
    self.title = title
    self.icon = icon
    self.meta = meta
    self.disabled = disabled
    self.content = AnyView(content())
  }
}

/// Several disclosures that know about each other.
///
/// That knowledge is the whole difference from a run of ``Collapsible``s: by
/// default opening one shuts the last, so a reader is always looking at one
/// section rather than at a page that grew while they were reading it.
/// `multiple` is the deliberate opt-in for the case where they are comparing
/// rather than choosing.
///
/// The rows and the panels are the same drawing a collapsible wears. What is
/// the accordion's own is the run between the sections — the thing a single
/// disclosure has no opinion about.
public struct Accordion: View {
  public var items: [AccordionItem]
  /// Drive it from outside. Left `nil` the view holds its own state.
  public var open: Set<String>?
  public var initiallyOpen: Set<String>
  public var onOpenChange: ((Set<String>) -> Void)?
  public var multiple: Bool
  /// Draws a hairline between the sections, for a run that has to read as one
  /// list rather than as a stack of separate disclosures.
  public var divided: Bool
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @State private var latched: Set<String>

  public init(
    items: [AccordionItem],
    open: Set<String>? = nil,
    initiallyOpen: Set<String> = [],
    onOpenChange: ((Set<String>) -> Void)? = nil,
    multiple: Bool = false,
    divided: Bool = false,
    enabled: Bool = true
  ) {
    self.items = items
    self.open = open
    self.initiallyOpen = initiallyOpen
    self.onOpenChange = onOpenChange
    self.multiple = multiple
    self.divided = divided
    self.enabled = enabled
    self._latched = State(initialValue: initiallyOpen)
  }

  private var value: Set<String> { open ?? latched }

  public var body: some View {
    let vars = theme.vars

    VStack(spacing: 0) {
      ForEach(Array(items.enumerated()), id: \.element.id) { index, item in
        VStack(spacing: 0) {
          DisclosureHeader(
            title: item.title,
            open: value.contains(item.id),
            icon: item.icon,
            enabled: enabled && !item.disabled,
            toggle: { toggle(item.id) },
            meta: { item.meta }
          )
          if value.contains(item.id) {
            DisclosurePanel { item.content }
          }
        }
        // Undivided, the sections sit on one run with nothing between them:
        // the rows are already separated by their own hover wash, and a rule
        // under every one would draw a table. `divided` is for the case where
        // they are genuinely a list.
        .rule(divided && index > 0 ? vars.colorBorder : .clear, edge: .top, width: hairline)
      }
    }
    .animation(vars.motion, value: value)
  }

  private func toggle(_ id: String) {
    let current = value
    let next: Set<String>
    if current.contains(id) {
      next = current.subtracting([id])
    } else if multiple {
      next = current.union([id])
    } else {
      next = [id]
    }
    if open == nil { latched = next }
    onOpenChange?(next)
  }
}
