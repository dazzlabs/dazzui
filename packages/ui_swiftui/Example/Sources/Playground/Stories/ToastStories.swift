import DazzUI
import SwiftUI

/// `Components/Toast` — what the app says in passing.
enum ToastStories {
  static var page: StoryPage {
    StoryPage("Toast") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") {
        Toast("Tokens rebuilt — three packages refreshed.")
      }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            Tinted(tint: tint)
          }
        }
      }
      Story("Shapes") {
        Col {
          Toast("Message only.")
          Toast("With a dismiss.", tint: .info, onDismiss: {})
          Detailed()
        }
      }
      Story("Truncation") {
        Toast(
          "A message long enough to run past the toast's own maximum width, where it truncates rather than pushing the box wider.",
          tint: .warning,
          onDismiss: {}
        )
      }
      Story("A stack", layout: .fullscreen) {
        ToastViewport {
          Toast("Tokens rebuilt.", tint: .success, onDismiss: {})
          Toast("Two packages still building…", tint: .info, onDismiss: {})
        }
      }
      Story("In a top viewport", layout: .fullscreen) {
        ToastViewport(placement: .top) {
          Toast("Tokens rebuilt.", tint: .success, onDismiss: {})
        }
      }
      Story("Posting one", layout: .fullscreen) { Posting() }
    }
  }

  /// The tint reaches the leading glyph and nothing else, so a story that
  /// leaves the glyph out would be six identical toasts.
  private struct Tinted: View {
    let tint: Tint

    @Environment(\.theme) private var theme

    var body: some View {
      Toast(
        tint: tint,
        onDismiss: {},
        leading: { Icon(.circle16Filled, size: theme.vars.spacing4) },
        message: { Text(label(tint)) },
        trailing: { EmptyView() }
      )
    }
  }

  private struct Detailed: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Toast(
        tint: .danger,
        onDismiss: {},
        leading: { Icon(.errorCircle16Regular, size: theme.vars.spacing4) },
        message: { Text("The build failed — 4 errors in Sidebar.swift.") },
        trailing: { Button("Show", variant: .plain, tint: .danger) {} }
      )
    }
  }

  /// A toast as an app posts one: something happens, the toast appears over
  /// the surface it happened on, and it goes when it is dismissed.
  private struct Posting: View {
    @State private var posted = false

    var body: some View {
      ZStack {
        Button("Rebuild tokens", variant: .filled) { posted = true }
          .frame(maxWidth: .infinity, maxHeight: .infinity)

        if posted {
          ToastViewport {
            Toast(
              "Tokens rebuilt — three packages refreshed.",
              tint: .success,
              onDismiss: { posted = false }
            )
          }
        }
      }
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Toast",
        description: """
          A message that passes, where a callout stays. The tint carries the \
          meaning and the shape carries how long it is around — which is the \
          whole of the difference between the two components.
          """
      ) {
        Section(
          "Shapes",
          description: """
            The message alone; the message with a dismiss; or the full shape — \
            a leading glyph, the message, and one action. Each is as wide as \
            what it holds, capped at the toast's own maximum: a long message \
            truncates rather than pushing the box wider. More than one action \
            belongs in a dialog — a toast the reader has to make a decision in \
            is a toast that will disappear while they are deciding.
            """
        ) {
          Col {
            Toast("Message only.")
            Toast("With a dismiss.", tint: .info, onDismiss: {})
            Detailed()
          }
        }

        Section(
          "Tints",
          description: """
            Neutral by default, because most toasts report that something \
            ordinary worked; reach for a ramp where the outcome itself is the \
            news. The tint reaches the leading glyph — and whatever the \
            trailing action is tinted with — and stops there: the box stays \
            paper, because a toast is read over whatever happens to be under \
            it.
            """
        ) {
          Col {
            ForEach(Tint.allCases, id: \.self) { tint in
              Tinted(tint: tint)
            }
          }
        }

        Section(
          "The viewport",
          description: """
            `ToastViewport` is the stack the toasts hang in, from the bottom by \
            default. It is transparent to the pointer between its toasts, so \
            the page underneath stays usable while one is on screen.
            """
        ) {
          Frame(width: 460, height: 220) {
            ToastViewport {
              Toast("Tokens rebuilt.", tint: .success, onDismiss: {})
              Toast("Two packages still building…", tint: .info, onDismiss: {})
            }
          }
        }

        Section("Tokens", description: "The one the toast owns.") {
          TokenTable([
            TokenRow("toast.max-width", theme.vars.toastMaxWidth)
          ])
        }
      }
    }
  }
}
