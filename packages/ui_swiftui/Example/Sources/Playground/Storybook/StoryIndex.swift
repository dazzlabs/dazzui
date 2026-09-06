import Foundation

/// The registry, the way `styles/components.css` is for the React package and
/// `components/mod.rs` is for the Rust one: a story page ships once it is
/// named here.
///
/// Tokens before Components, because the components are drawn out of them; and
/// within Components, alphabetical — there is one page per component, so the
/// group is an index you look a name up in.
enum StoryIndex {
  static let groups: [StoryGroup] = [
    StoryGroup("Tokens", pages: [
      ColorStories.page,
      TypographyStories.page,
      SpacingStories.page,
      RadiusStories.page,
      EffectStories.page,
      ControlStories.page,
      ThemeStories.page,
    ]),
    StoryGroup("Components", pages: [
      ActionBarStories.page,
      AsideStories.page,
      BadgeStories.page,
      ButtonStories.page,
      CalloutStories.page,
      CardStories.page,
      CheckboxStories.page,
      DialogStories.page,
      DividerStories.page,
      EmptyStateStories.page,
      FormFieldStories.page,
      IconStories.page,
      IconButtonStories.page,
      KeyCapStories.page,
      MenuStories.page,
      NavItemStories.page,
      OptionCardStories.page,
      PillTabsStories.page,
      PreferenceStories.page,
      ProgressStories.page,
      RadioStories.page,
      RailStories.page,
      SearchFieldStories.page,
      SectionLabelStories.page,
      SegmentedControlStories.page,
      SelectStories.page,
      ShortcutRecorderStories.page,
      SidebarStories.page,
      SpinnerStories.page,
      StepListStories.page,
      SwitchStories.page,
      TableStories.page,
      TextFieldStories.page,
      ToastStories.page,
      TooltipStories.page,
    ])
  ]

  /// Where the canvas points when the window opens.
  static let first = StoryRef(page: "Button", story: "Overview")

  /// The component pages by name. A story that needs a long column of rows
  /// takes the playground's own index rather than inventing a list — an
  /// invented one is the first thing that reads as fake.
  static var componentNames: [String] {
    groups.first { $0.name == "Components" }?.pages.map(\.name) ?? []
  }

  /// Every story's id, in sidebar order — what the parity test holds against
  /// the React list.
  static var ids: [String] {
    groups.flatMap { group in
      group.pages.flatMap { page in
        page.stories.map { storyId(group: group.name, page: page.name, story: $0.name) }
      }
    }
  }

  static func story(_ ref: StoryRef) -> Story? {
    groups
      .flatMap(\.pages)
      .first { $0.name == ref.page }?
      .stories
      .first { $0.name == ref.story }
  }
}
