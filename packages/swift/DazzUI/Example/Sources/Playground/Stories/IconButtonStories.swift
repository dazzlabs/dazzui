import DazzUI
import SwiftUI

/// `Components/IconButton` — the square button a toolbar is made of.
enum IconButtonStories {
  static var page: StoryPage {
    StoryPage("IconButton") {
      Story("Overview", layout: .page) { Overview() }
      Story("Plain") { IconButton(label: "Pin", icon: .pin16Regular) {} }
      Story("Filled") {
        IconButton(label: "Add", icon: .add16Regular, variant: .filled) {}
      }
      Story("Variants") {
        Row {
          ForEach(DazzUI.ControlVariant.allCases, id: \.self) { variant in
            IconButton(label: label(variant), icon: .star16Regular, variant: variant) {}
          }
        }
      }
      Story("Disabled") {
        Row {
          ForEach(DazzUI.ControlVariant.allCases, id: \.self) { variant in
            IconButton(
              label: label(variant),
              icon: .star16Regular,
              variant: variant,
              enabled: false
            ) {}
          }
        }
      }
      Story("Sizes") {
        Row {
          ForEach(DazzUI.ControlSize.allCases, id: \.self) { size in
            IconButton(label: label(size), icon: .star16Regular, size: size) {}
          }
        }
      }
      Story("Tints") {
        Row {
          ForEach(Tint.allCases, id: \.self) { tint in
            IconButton(
              label: label(tint),
              icon: .star16Regular,
              variant: .tinted,
              tint: tint
            ) {}
          }
        }
      }
      Story("A toolbar") { Toolbar() }
      Story("In a list header") {
        Card {
          HStack {
            Text("Builds")
            Spacer()
            IconButton(label: "Filter", icon: .filter16Regular) {}
            IconButton(label: "Add", icon: .add16Regular) {}
          }
          .frame(width: 280)
        }
      }
    }
  }

  /// An icon button beside the thing that does latch, which is a ``Toggle``.
  private struct NotALatch: View {
    @State private var pinned = true

    var body: some View {
      Row {
        IconButton(label: "Copy", icon: .copy16Regular) {}
        Toggle(.pin16Regular, label: "Pin window", pressed: pinned) {
          pinned.toggle()
        }
        Toggle(.pin16Regular, label: "Pinned", pressed: true) {}
      }
    }
  }

  private struct Toolbar: View {
    var body: some View {
      TooltipProvider {
        ActionBar {
          IconButton(label: "Back", icon: .chevronLeft16Regular) {}
          IconButton(label: "Forward", icon: .chevronRight16Regular) {}
          ToolbarSeparator()
          IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
          IconButton(label: "Share", icon: .share16Regular) {}
          ToolbarSeparator()
          IconButton(label: "Delete", icon: .delete16Regular, tint: .danger) {}
        }
        .frame(width: 300)
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "IconButton",
        description: """
          A button whose label is a glyph. It is the same control as `Button` — \
          the same recipes, the same states — squared to the control height, \
          with the label moved into the accessible name. It defaults to `plain` \
          at the `tiny` step, because a toolbar of these should read as chrome \
          rather than as a row of accented actions.
          """
      ) {
        Section(
          "Variants",
          description: """
            Every control recipe, on any ramp. Pass a tint where one button \
            genuinely leads — a destructive action in a toolbar of neutral \
            ones.
            """
        ) {
          Stack {
            ForEach(DazzUI.ControlVariant.allCases, id: \.self) { variant in
              Specimen(variant.rawValue) {
                ForEach(Tint.allCases, id: \.self) { tint in
                  IconButton(
                    label: label(tint),
                    icon: .star16Regular,
                    variant: variant,
                    tint: tint
                  ) {}
                }
              }
            }
          }
        }

        Section(
          "Sizes",
          description: """
            The control profile again, and `tiny` is the one this component was \
            drawn for: a square on the toolbar step.
            """
        ) {
          Row {
            ForEach(DazzUI.ControlSize.allCases, id: \.self) { size in
              IconButton(label: label(size), icon: .star16Regular, size: size) {}
            }
          }
        }

        Section(
          "It does not latch",
          description: """
            A glyph button that stays down is a Toggle, which is square at this \
            same control height when its label is a lone glyph — so a latching \
            glyph and a plain one sit in one toolbar without a seam. Keeping an \
            on-state here as well would be a second drawing for one state, and \
            the system draws a selection one way: a tinted wash and a heavier \
            label.
            """
        ) {
          NotALatch()
        }

        Section(
          "In a toolbar",
          description: """
            Runs of buttons separated by a vertical rule, a tooltip provider \
            around the lot so the second label opens without its own delay, and \
            at most one tinted button.
            """
        ) {
          Toolbar()
        }
      }
    }
  }
}
