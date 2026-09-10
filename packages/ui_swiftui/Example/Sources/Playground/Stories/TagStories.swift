import DazzUI
import SwiftUI

/// `Components/Tag` — a Badge you can act on.
enum TagStories {
  static let labels = ["bug", "good first issue", "needs design", "blocked"]

  static var page: StoryPage {
    StoryPage("Tag") {
      Story("Overview", layout: .page) { Overview() }
      Story("Selectable") { Filters() }
      Story("Removable") { Removable() }
      Story("Sizes") {
        HStack(spacing: 6) {
          ForEach(BadgeSize.allCases, id: \.self) { size in
            Tag(size.rawValue, size: size, onRemove: {})
          }
        }
      }
      Story("Variants") {
        HStack(spacing: 6) {
          ForEach(TagVariant.allCases, id: \.self) { variant in
            Tag(variant.rawValue, variant: variant)
          }
        }
      }
      Story("Tints") {
        HStack(spacing: 6) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Tag(tint.rawValue, tint: tint, onRemove: {})
          }
        }
      }
      Story("Disabled") {
        HStack(spacing: 6) {
          Tag("unavailable", selected: false, enabled: false, onSelect: { _ in })
          Tag("pinned", enabled: false, onRemove: {})
        }
      }
      Story("A filter bar") {
        VStack(alignment: .leading, spacing: 8) {
          Filters()
          Removable()
        }
        .frame(width: 416, alignment: .leading)
      }
    }
  }

  /// Split out of the Overview: the type checker times out on a `Tag` with
  /// four labelled arguments inside two result builders.
  static var selectedSpecimen: some View {
    Tag("bug", variant: .outlined, selected: true, onSelect: { _ in })
  }

  static var removableSpecimen: some View {
    Tag("needs design", onRemove: {})
  }

  struct Filters: View {
    @State private var on: Set<String> = ["bug"]

    var body: some View {
      HStack(spacing: 6) {
        ForEach(TagStories.labels, id: \.self) { label in
          Tag(
            label,
            variant: .outlined,
            selected: on.contains(label),
            onSelect: { next in
              if next { on.insert(label) } else { on.remove(label) }
            }
          )
        }
      }
    }
  }

  struct Removable: View {
    @State private var labels = TagStories.labels

    var body: some View {
      HStack(spacing: 6) {
        ForEach(labels, id: \.self) { label in
          Tag(label, removeLabel: "Remove \(label)", onRemove: {
            labels.removeAll { $0 == label }
          })
        }
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Tag",
        description:
          "A Badge you can act on. The two are one drawing and two views rather than one view "
          + "with an interactive flag, because the difference is not cosmetic: a badge reads "
          + "its recipes at rest and stays out of the shared state machine, since a label that "
          + "moves under the pointer reads as something you can press. A tag is something you "
          + "can press — and taking the state machine is the whole reason it exists."
      ) {
        Section(
          "Beside a Badge",
          description:
            "They share the capsule — the same box at the same three densities — and nothing "
            + "else. A tag that is neither selectable nor removable should be a Badge."
        ) {
          Stack {
            Specimen("Badge") { Badge("v1.7.0") }
            Specimen("Tag, selectable") { TagStories.selectedSpecimen }
            Specimen("Tag, removable") { TagStories.removableSpecimen }
          }
        }
        Section(
          "Selected",
          description:
            "Drawn twice over: the tinted fill the view swaps the variant to, and the tint's "
            + "edge. One alone is not enough — a tinted tag that was already tinted would look "
            + "untouched, and an edge alone is a pixel to spot in a row of forty."
        ) {
          TagStories.Filters()
        }
        Section(
          "Dismissing",
          description:
            "The dismiss button is its own control rather than a second job for the label: a "
            + "chip that removes itself when clicked has no way to be selected, and one that "
            + "does both from the same target removes things by accident. It is faint at rest "
            + "and only resolves under the pointer."
        ) {
          TagStories.Removable()
        }
      }
    }
  }
}
