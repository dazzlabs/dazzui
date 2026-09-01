import DazzUI
import FluentIcons
import SwiftUI

/// What the app says back: banners that stay, toasts that pass, a dialog that
/// blocks, and the empty state that stands in for content.
struct FeedbackPage: View {
  /// Owned by the window rather than by this page: the scrim has to cover the
  /// window, not the scrolled column it was triggered from.
  @Binding var dialog: Bool
  @Binding var toast: Bool

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    PageTitle(
      "Feedback",
      "The tint carries the meaning; the shape carries how long it stays."
    )

    Specimen("Alert banners") {
      VStack(alignment: .leading, spacing: vars.spacing2) {
        AlertBanner(
          "Restart to finish updating to 2.1.0.",
          tint: .info,
          title: "A new version is available"
        )
        AlertBanner(
          "Three platform packages were refreshed.",
          tint: .success,
          title: "Tokens rebuilt"
        )
        AlertBanner(
          "color.warning cannot move whole under bright-dark.",
          tint: .warning,
          title: "Ramp moves only part-way"
        )
        AlertBanner(
          "DazzUI: 4 errors in Sidebar.swift.",
          tint: .danger,
          title: "The build failed"
        )
      }
      .frame(maxWidth: vars.toastMaxWidth * 1.5)
    }

    Specimen("Dialog", note: "the scrim is the content colour at its own alpha") {
      Button("Show a dialog", variant: .filled) { dialog = true }
    }

    Specimen("Toast") {
      Button("Post a toast") { toast = true }
    }

    Specimen("Menu", note: "Esc or a selection closes it") {
      Menu(
        items: [
          MenuItem("Rename", icon: .rename16Regular, shortcut: "⌘R") {},
          MenuItem("Duplicate", icon: .squareMultiple16Regular) {},
          MenuItem("Show hidden files", checked: true) {},
          MenuItem("Delete", icon: .delete16Regular, disabled: true) {},
        ]
      ) { state in
        Button("Actions", shortcut: "▾") { state.toggle() }
      }
    }

    Specimen("Empty state") {
      Card(variant: .sunken) {
        EmptyState(title: "No components match this filter") {
          Button("Clear the filter", variant: .tinted) {}
        }
        .frame(maxWidth: .infinity)
      }
    }

    Spacer(minLength: vars.spacing8)
  }
}

/// The overlays, hoisted to the window so the scrim covers it rather than
/// covering the scrolled page inside it — where a portal target would sit in
/// React, and where a sheet would sit in an app.
struct FeedbackOverlays: View {
  @Binding var dialog: Bool
  @Binding var toast: Bool

  var body: some View {
    ZStack {
      if dialog {
        DialogScrim {
          Dialog {
            DialogHeader(
              "Discard these changes?",
              subtitle: "The three files you edited will go back to their last saved state."
            )
            DialogBody {
              Text("This cannot be undone.")
            }
            DialogFooter {
              Button("Keep editing") { dialog = false }
              Button("Discard", variant: .filled, tint: .danger) { dialog = false }
            }
          }
        }
      }

      if toast {
        ToastViewport {
          Toast(tint: .success, onDismiss: { toast = false }) {
            Text("Tokens rebuilt — three packages refreshed.")
          }
        }
      }
    }
  }
}
