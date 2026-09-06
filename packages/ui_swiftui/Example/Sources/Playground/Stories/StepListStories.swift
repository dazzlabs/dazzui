import DazzUI
import SwiftUI

/// `Components/StepList` — a job, reported.
enum StepListStories {
  static var page: StoryPage {
    StoryPage("StepList") {
      Story("Overview", layout: .page) { Overview() }
      Story("In flight") {
        StepList {
          Step("Resolve the token graph", status: .done)
          Step("Write the platform sources", status: .done)
          Step("Compile DazzUI", status: .active)
          Step("Publish the package", status: .idle)
        }
        .frame(width: 320)
      }
      Story("Statuses") {
        StepList {
          ForEach(StepStatus.allCases, id: \.self) { status in
            Step(label(status), status: status)
          }
        }
        .frame(width: 320)
      }
      Story("Finished") {
        StepList {
          Step("Resolve the token graph", status: .done)
          Step("Write the platform sources", status: .done)
          Step("Compile DazzUI", status: .done)
        }
        .frame(width: 320)
      }
      Story("Timed detail") {
        StepList {
          Step("Resolve the token graph", meta: "0.4s", status: .done)
          Step("Write the platform sources", meta: "1.2s", status: .done)
          Step("Compile DazzUI", meta: "running", status: .active)
          Step("Publish the package", meta: "—", status: .idle)
        }
        .frame(width: 360)
      }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            StepList {
              Step(label(tint), status: .active, tint: tint)
            }
          }
        }
      }
      Story("Advancing") { Advancing() }
      Story("In a card") {
        Card {
          Text("Rebuilding tokens")
          StepList {
            Step("Resolve the token graph", meta: "0.4s", status: .done)
            Step("Compile DazzUI", meta: "running", status: .active)
            Step("Publish the package", status: .idle)
          }
        }
        .frame(width: 360)
      }
    }
  }

  private struct Advancing: View {
    @State private var reached = 1

    private let steps = [
      "Resolve the token graph",
      "Write the platform sources",
      "Compile DazzUI",
      "Publish the package",
    ]

    var body: some View {
      Col(width: 360) {
        StepList {
          ForEach(Array(steps.enumerated()), id: \.element) { index, step in
            Step(step, status: status(at: index))
          }
        }
        ActionBar {
          Button("Advance", variant: .recessed) {
            reached = reached >= steps.count ? 0 : reached + 1
          }
        }
      }
    }

    private func status(at index: Int) -> StepStatus {
      if index < reached { return .done }
      return index == reached ? .active : .idle
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "StepList",
        description: """
          What a long job says while it runs: a line per stage, a mark saying \
          where it got to, and an optional trailing detail. It reports rather \
          than navigates — nothing in it is clickable.
          """
      ) {
        Section(
          "Statuses",
          description: """
            `done` is the tick; `active` is the stage in flight and the only \
            one that carries the tint; `idle` is the stage not reached yet, in \
            faint ink. Exactly one step should be active.
            """
        ) {
          StepList {
            ForEach(StepStatus.allCases, id: \.self) { status in
              Step(label(status), status: status)
            }
          }
          .frame(width: 360)
        }

        Section(
          "Detail",
          description: """
            The trailing slot is for the number the stage produced — a \
            duration, a count, a hyphen where there is nothing yet. It is not a \
            second label: a step whose detail explains what it does is a step \
            whose label is wrong.
            """
        ) {
          StepList {
            Step("Resolve the token graph", meta: "0.4s", status: .done)
            Step("Compile DazzUI", meta: "running", status: .active)
            Step("Publish the package", meta: "—", status: .idle)
          }
          .frame(width: 400)
        }

        Section(
          "Advancing",
          description: "The list as a job actually moves through it."
        ) {
          Advancing()
        }
      }
    }
  }
}
