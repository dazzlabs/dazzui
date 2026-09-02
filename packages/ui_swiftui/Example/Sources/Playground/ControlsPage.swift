import DazzUI
import FluentIcons
import SwiftUI

/// The control vocabulary: every recipe, every size, and the annotations that
/// sit beside them.
struct ControlsPage: View {
  @Environment(\.theme) private var theme
  @State private var pinned = true

  var body: some View {
    let vars = theme.vars

    PageTitle(
      "Controls",
      "A variant picks the recipe, a tint picks the ramp it resolves against."
    )

    Specimen("Variants", note: "recessed · normal · filled · tinted · outlined · plain") {
      Row {
        ForEach(ControlVariant.allCases, id: \.self) { variant in
          Button(variant.rawValue.capitalized, variant: variant) {}
        }
      }
    }

    Specimen("Sizes", note: "geometry and type both come off the size profile") {
      Row {
        ForEach(ControlSize.allCases, id: \.self) { size in
          Button(size.rawValue.capitalized, size: size, variant: .filled) {}
        }
      }
    }

    Specimen("Tints", note: "one recipe table, six ramps") {
      Row {
        ForEach(Tint.allCases, id: \.self) { tint in
          Button(tint.rawValue.capitalized, variant: .tinted, tint: tint) {}
        }
      }
    }

    Specimen("Disabled", note: "every variant flattens to the same groove") {
      Row {
        ForEach(ControlVariant.allCases, id: \.self) { variant in
          Button(variant.rawValue.capitalized, variant: variant, enabled: false) {}
        }
      }
    }

    Specimen("With a shortcut") {
      Row {
        Button("Save", variant: .filled, shortcut: "⌘S") {}
        Button("Cancel", shortcut: "esc") {}
      }
    }

    Specimen("Icon buttons", note: "quiet chrome until touched; the pin holds an on-state") {
      Row {
        IconButton(label: "Back", icon: .chevronLeft16Regular) {}
        IconButton(label: "Forward", icon: .chevronRight16Regular) {}
        IconButton(label: "Pin", icon: .pin16Regular, active: pinned) { pinned.toggle() }
        IconButton(label: "Delete", icon: .delete16Regular, variant: .tinted, tint: .danger) {}
        IconButton(label: "Disabled", icon: .star16Regular, enabled: false) {}
      }
    }

    Specimen("Badges", note: "annotations, so they never react to the pointer") {
      Row {
        Badge("Filled", variant: .filled)
        Badge("Tinted", variant: .tinted)
        Badge("Outlined", variant: .outlined)
        Badge("Raised", variant: .raised)
        Badge("Beta", variant: .tinted, tint: .warning)
        Badge("Failed", variant: .tinted, tint: .danger)
      }
    }

    Specimen("Keys") {
      Row {
        KeyCap("⌘K")
        KeyCap("⇧⌘P", variant: .strong)
        KeyCap("Esc", variant: .key)
      }
    }

    Specimen("Pill tabs") {
      PillTabsDemo()
    }

    Specimen("Segmented control") {
      SegmentedDemo()
    }

    Spacer(minLength: vars.spacing8)
  }
}

private struct PillTabsDemo: View {
  @State private var tab = "all"

  var body: some View {
    PillTabs(
      items: [
        PillTabItem(value: "all", label: "All", count: 24),
        PillTabItem(value: "open", label: "Open", count: 3),
        PillTabItem(value: "done", label: "Done", count: 21),
        PillTabItem(value: "archived", label: "Archived", disabled: true),
      ],
      value: tab,
      onValueChange: { tab = $0 }
    )
  }
}

private struct SegmentedDemo: View {
  @State private var mode = "design"

  var body: some View {
    SegmentedControl(
      items: [
        SegmentedItem(value: "design", label: "Design"),
        SegmentedItem(value: "code", label: "Code"),
        SegmentedItem(value: "preview", label: "Preview"),
      ],
      value: mode,
      onValueChange: { mode = $0 }
    )
  }
}
