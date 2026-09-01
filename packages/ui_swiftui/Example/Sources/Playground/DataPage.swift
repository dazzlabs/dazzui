import DazzUI
import SwiftUI

/// Surfaces and the things that sit on them: cards, a table, progress, and
/// the step list that reports on a long-running job.
struct DataPage: View {
  @Environment(\.theme) private var theme
  @State private var selected = "aurora"

  private let builds = [
    Build(id: "aurora", name: "aurora", branch: "main", status: "Passed", tint: Tint.success),
    Build(id: "beacon", name: "beacon", branch: "release/2.1", status: "Running", tint: .info),
    Build(id: "cinder", name: "cinder", branch: "fix/tokens", status: "Failed", tint: .danger),
  ]

  var body: some View {
    let vars = theme.vars

    PageTitle(
      "Data",
      "A card is a surface; a row is a surface that answers the pointer."
    )

    Specimen("Cards", note: "raised · sunken · outlined · tinted") {
      Row(spacing: vars.spacing3) {
        ForEach(CardVariant.allCases, id: \.self) { variant in
          Card(variant: variant) {
            VStack(alignment: .leading, spacing: vars.spacing1) {
              Text(variant.rawValue.capitalized)
                .font(vars.labelMedium.font)
                .foregroundStyle(vars.colorContent)
              Text("A surface.")
                .font(vars.captionSmall.font)
                .foregroundStyle(vars.colorContentMuted)
            }
          }
        }
      }
    }

    Specimen("Table", note: "hover outranks selection, exactly as the stylesheet has it") {
      Table {
        TableHead {
          TableCell(head: true) { Text("Build") }
          TableCell(head: true) { Text("Branch") }
          TableCell(head: true, align: .end) { Text("Status") }
        }

        ForEach(builds) { build in
          TableRow(
            active: selected == build.id,
            interactive: true,
            action: { selected = build.id }
          ) {
            TableCell { Text(build.name) }
            TableCell { Text(build.branch) }
            TableCell(align: .end) {
              Badge(build.status, size: .small, variant: .tinted, tint: build.tint)
            }
          }
        }
      }
    }

    Specimen("Progress") {
      VStack(alignment: .leading, spacing: vars.spacing3) {
        Progress(value: 0.35)
        Progress(value: 0.7, size: .medium, gradient: true)
        // No value at all is the indeterminate sweep.
        Progress(size: .large, tint: .info)
      }
      .frame(maxWidth: vars.dialogWidth)
    }

    Specimen("Spinners") {
      Row(spacing: vars.spacing4) {
        Spinner(size: .small)
        Spinner()
        Spinner(size: .large, tint: .neutral)
      }
    }

    Specimen("Steps") {
      StepList {
        Step(status: .done) { Text("Resolve the token graph") }
        Step(status: .done) { Text("Write the platform sources") }
        Step(status: .active) { Text("Compile DazzUI") }
        Step(status: .idle) { Text("Publish the package") }
      }
    }

    Spacer(minLength: vars.spacing8)
  }
}

private struct Build: Identifiable {
  let id: String
  let name: String
  let branch: String
  let status: String
  let tint: Tint
}
