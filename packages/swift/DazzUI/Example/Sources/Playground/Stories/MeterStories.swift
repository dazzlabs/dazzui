import DazzUI
import SwiftUI

/// `Components/Meter` — a reading inside a known range.
enum MeterStories {
  static let quotas: [(String, Double, Tint)] = [
    ("Storage", 62, .primary),
    ("Bandwidth", 41, .info),
    ("Build minutes", 88, .warning),
    ("Seats", 100, .danger),
  ]

  static var page: StoryPage {
    StoryPage("Meter") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Meter(value: 62).frame(width: 288) }
      Story("With value") {
        Meter(value: 62, label: "Storage", valueLabel: "62%").frame(width: 320)
      }
      Story("Sizes") {
        Col(width: 320) {
          ForEach(MeterSize.allCases, id: \.self) { size in
            Meter(value: 62, size: size)
          }
        }
      }
      Story("Tints") {
        Col(width: 320) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Meter(value: 62, tint: tint)
          }
        }
      }
      Story("In a quota list") { quotaList }
    }
  }

  static var quotaList: some View {
    Col(spacing: 16, width: 352) {
      ForEach(quotas, id: \.0) { label, value, tint in
        Meter(value: value, tint: tint, label: label, valueLabel: "\(Int(value))%")
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Meter",
        description:
          "A reading inside a known range — disk used of disk there, spend against a budget, an "
          + "API quota. It is not a Progress, and the difference is not how they look: progress "
          + "reports on something running, so it moves on its own, it can be indeterminate, and "
          + "it ends. A meter measures something that already has a value, so it never animates "
          + "and never has none — which is why it prints its number and progress does not."
      ) {
        Section(
          "Sizes",
          description:
            "The same groove a Progress bar is cut into — the sunken surface, the soft square "
            + "corner — because they are the same hole in the page. What differs is everything "
            + "above it."
        ) {
          Stack {
            ForEach(MeterSize.allCases, id: \.self) { size in
              Specimen(label(size)) { Meter(value: 62, size: size).frame(width: 240) }
            }
          }
        }
        Section(
          "Label and value",
          description:
            "The number takes the mono face, so a column of meters keeps its numbers on one "
            + "edge. The label is the meter’s heading and its accessible name."
        ) {
          MeterStories.quotaList
        }
        Section(
          "Against a Progress",
          description:
            "Set side by side the rule is easy to keep: if the number would change on its own "
            + "while the reader watched, it is progress. If it is a fact about right now, it is "
            + "a meter."
        ) {
          Col(width: 320) {
            Meter(value: 62, label: "Disk used", valueLabel: "62%")
            Progress(value: 0.62)
          }
        }
      }
    }
  }
}
