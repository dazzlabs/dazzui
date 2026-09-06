import '../stories/action_bar_stories.dart';
import '../stories/aside_stories.dart';
import '../stories/badge_stories.dart';
import '../stories/button_stories.dart';
import '../stories/callout_stories.dart';
import '../stories/card_stories.dart';
import '../stories/checkbox_stories.dart';
import '../stories/data_table_stories.dart';
import '../stories/dialog_stories.dart';
import '../stories/divider_stories.dart';
import '../stories/empty_state_stories.dart';
import '../stories/form_field_stories.dart';
import '../stories/icon_button_stories.dart';
import '../stories/key_cap_stories.dart';
import '../stories/loader_stories.dart';
import '../stories/menu_stories.dart';
import '../stories/nav_item_stories.dart';
import '../stories/option_card_stories.dart';
import '../stories/pill_tabs_stories.dart';
import '../stories/preference_stories.dart';
import '../stories/progress_stories.dart';
import '../stories/radio_stories.dart';
import '../stories/rail_stories.dart';
import '../stories/search_field_stories.dart';
import '../stories/section_label_stories.dart';
import '../stories/segmented_control_stories.dart';
import '../stories/sidebar_stories.dart';
import '../stories/step_list_stories.dart';
import '../stories/switch_stories.dart';
import '../stories/text_field_stories.dart';
import '../stories/toast_stories.dart';
import '../stories/tokens/color_stories.dart';
import '../stories/tokens/control_stories.dart';
import '../stories/tokens/effect_stories.dart';
import '../stories/tokens/radius_stories.dart';
import '../stories/tokens/spacing_stories.dart';
import '../stories/tokens/theme_stories.dart';
import '../stories/tokens/typography_stories.dart';
import 'story.dart';

/// The registry, the way `styles/components.css` is for the React package and
/// `components/mod.rs` is for the Rust one: a story page ships once it is named
/// here.
///
/// Tokens before Components, because the widgets are drawn out of them; and
/// within Components, alphabetical — there is one page per widget, so the group
/// is an index you look a name up in.
class StoryIndex {
  StoryIndex._();

  static final List<StoryGroup> groups = [
    StoryGroup('Tokens', [
      ColorStories.page,
      TypographyStories.page,
      SpacingStories.page,
      RadiusStories.page,
      EffectStories.page,
      ControlStories.page,
      ThemeStories.page,
    ]),
    StoryGroup('Components', [
      ActionBarStories.page,
      AsideStories.page,
      BadgeStories.page,
      ButtonStories.page,
      CalloutStories.page,
      CardStories.page,
      CheckboxStories.page,
      DataTableStories.page,
      DialogStories.page,
      DividerStories.page,
      EmptyStateStories.page,
      FormFieldStories.page,
      IconButtonStories.page,
      KeyCapStories.page,
      LoaderStories.page,
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
      SidebarStories.page,
      StepListStories.page,
      SwitchStories.page,
      TextFieldStories.page,
      ToastStories.page,
    ]),
  ];

  /// Where the canvas points when the window opens.
  static const StoryRef first = StoryRef('Button', 'Overview');

  /// The component pages by name. A story that needs a long column of rows
  /// takes the playground's own index rather than inventing a list — an
  /// invented one is the first thing that reads as fake.
  static List<String> get componentNames => groups
      .firstWhere((group) => group.name == 'Components')
      .pages
      .map((page) => page.name)
      .toList();

  /// Every story's id, in sidebar order — what the parity test holds against
  /// the React list.
  static List<String> get ids => [
    for (final StoryGroup group in groups)
      for (final StoryPage page in group.pages)
        for (final Story story in page.stories)
          storyId(group.name, page.name, story.name),
  ];

  static Story? story(StoryRef ref) {
    for (final StoryGroup group in groups) {
      for (final StoryPage page in group.pages) {
        if (page.name != ref.page) continue;
        for (final Story story in page.stories) {
          if (story.name == ref.story) return story;
        }
      }
    }
    return null;
  }
}
