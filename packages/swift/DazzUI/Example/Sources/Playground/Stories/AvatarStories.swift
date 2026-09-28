import DazzUI
import SwiftUI

/// `Components/Avatar` — the picture that stands for a person or a project.
enum AvatarStories {
  static let people: [(String, Tint)] = [
    ("Ada Lovelace", .primary),
    ("Grace Hopper", .info),
    ("Alan Turing", .success),
    ("Katherine Johnson", .warning),
    ("Barbara Liskov", .danger),
  ]

  static var page: StoryPage {
    StoryPage("Avatar") {
      Story("Overview", layout: .page) { Overview() }
      Story("Initials") {
        Row {
          ForEach(people.prefix(4), id: \.0) { name, tint in
            Avatar(name, tint: tint)
          }
        }
      }
      Story("Image") {
        // Nothing here reaches a network, so what an image would fall back to
        // is what is drawn. It is the same view either way.
        Row {
          ForEach(AvatarSize.allCases, id: \.self) { size in
            Avatar("Ada Lovelace", size: size)
          }
        }
      }
      Story("Square") {
        Row {
          Avatar("DazzUI", shape: .square)
          Avatar("Acme Web", shape: .square, tint: .info)
          Avatar("Design Tokens", shape: .square, tint: .neutral)
        }
      }
      Story("Sizes") {
        Col {
          ForEach(AvatarSize.allCases, id: \.self) { size in
            Row {
              Avatar("Ada Lovelace", size: size)
              Avatar("DazzUI", size: size, shape: .square)
            }
          }
        }
      }
      Story("Tints") {
        Row {
          ForEach(Tint.allCases, id: \.self) { tint in
            Avatar(tint.rawValue, tint: tint)
          }
        }
      }
      Story("Groups") {
        Col {
          ForEach(AvatarSize.allCases, id: \.self) { size in
            group(size: size)
          }
        }
      }
    }
  }

  static func group(size: AvatarSize, ring: Color? = nil) -> some View {
    AvatarGroup(size: size, overflow: people.count - 3, ring: ring) {
      ForEach(people.prefix(3), id: \.0) { name, tint in
        Avatar(name, size: size, tint: tint)
      }
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Avatar",
        description:
          "The picture that stands for a person or a project. What it is really for is the "
          + "case where there is no picture: the initials are drawn until one has loaded and "
          + "instead of it when it never does. It is not interactive and stays off the shared "
          + "state machine — an avatar that lit up under the pointer would read as something "
          + "you can press."
      ) {
        Section(
          "Sizes",
          description:
            "Three steps off the 4px grid rather than off the control profile — an avatar is "
            + "not a control, and these are the sizes a row, a header and a card actually ask "
            + "for. The type moves one step of the scale per size rather than tracking the box, "
            + "which grows nearly twice as fast."
        ) {
          Stack {
            ForEach(AvatarSize.allCases, id: \.self) { size in
              Specimen(label(size)) {
                Avatar("Ada Lovelace", size: size)
                Avatar("DazzUI", size: size, shape: .square)
              }
            }
          }
        }
        Section(
          "Circle and square",
          description:
            "A person is a circle and a thing is a square — a project, a repository. The square "
            + "takes the container corner rather than the control corner: under Bright, where "
            + "controls are pills, a control-cornered square would go round and stop being the "
            + "shape that says “project”."
        ) {
          Stack {
            Specimen("circle") {
              ForEach(AvatarStories.people.prefix(3), id: \.0) { name, tint in
                Avatar(name, tint: tint)
              }
            }
            Specimen("square") {
              Avatar("DazzUI", shape: .square)
              Avatar("Acme Web", shape: .square, tint: .info)
            }
          }
        }
        Section(
          "Groups",
          description:
            "Overlapped by a third of the box and cut out of each other by a ring in the ground "
            + "colour. The ring is drawn outside the box rather than as a border on it, so it "
            + "costs the run nothing — a border would grow every avatar by three pixels."
        ) {
          Stack {
            Specimen("on paper") { AvatarStories.group(size: .medium) }
            Specimen("on a card") {
              Card {
                AvatarStories.group(size: .medium, ring: theme.vars.colorSurfaceMuted)
              }
            }
          }
        }
      }
    }
  }
}
