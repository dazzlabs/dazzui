import '../generated/scenes.dart';
import '../stories/accordion_stories.dart';
import '../stories/action_bar_stories.dart';
import '../stories/aside_stories.dart';
import '../stories/autocomplete_stories.dart';
import '../stories/avatar_stories.dart';
import '../stories/checkbox_stories.dart';
import '../stories/code_block_stories.dart';
import '../stories/code_stories.dart';
import '../stories/collapsible_stories.dart';
import '../stories/combobox_stories.dart';
import '../stories/context_menu_stories.dart';
import '../stories/dialog_stories.dart';
import '../stories/drawer_stories.dart';
import '../stories/empty_state_stories.dart';
import '../stories/form_field_stories.dart';
import '../stories/form_stories.dart';
import '../stories/icon_button_stories.dart';
import '../stories/key_cap_stories.dart';
import '../stories/link_stories.dart';
import '../stories/menu_stories.dart';
import '../stories/menubar_stories.dart';
import '../stories/meter_stories.dart';
import '../stories/nav_item_stories.dart';
import '../stories/number_field_stories.dart';
import '../stories/option_card_stories.dart';
import '../stories/otp_field_stories.dart';
import '../stories/pill_tabs_stories.dart';
import '../stories/popover_stories.dart';
import '../stories/preferences_stories.dart';
import '../stories/preview_card_stories.dart';
import '../stories/progress_stories.dart';
import '../stories/radio_stories.dart';
import '../stories/rail_stories.dart';
import '../stories/search_field_stories.dart';
import '../stories/section_label_stories.dart';
import '../stories/segmented_control_stories.dart';
import '../stories/sidebar_stories.dart';
import '../stories/skeleton_stories.dart';
import '../stories/slider_stories.dart';
import '../stories/spinner_stories.dart';
import '../stories/splitter_stories.dart';
import '../stories/switch_stories.dart';
import '../stories/table_stories.dart';
import '../stories/tag_stories.dart';
import '../stories/text_field_stories.dart';
import '../stories/toast_stories.dart';
import '../stories/toggle_group_stories.dart';
import '../stories/toggle_stories.dart';
import '../stories/tokens/color_stories.dart';
import '../stories/tokens/control_stories.dart';
import '../stories/tokens/effect_stories.dart';
import '../stories/tokens/radius_stories.dart';
import '../stories/tokens/spacing_stories.dart';
import '../stories/tokens/theme_stories.dart';
import '../stories/tokens/typography_stories.dart';
import '../stories/tree_stories.dart';
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

  static final List<StoryGroup> groups = generatedGroups([
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
      AccordionStories.page,
      ActionBarStories.page,
      AsideStories.page,
      AutocompleteStories.page,
      AvatarStories.page,
      CheckboxStories.page,
      CodeStories.page,
      CodeBlockStories.page,
      CollapsibleStories.page,
      ComboboxStories.page,
      ContextMenuStories.page,
      TableStories.page,
      DialogStories.page,
      DrawerStories.page,
      EmptyStateStories.page,
      FormStories.page,
      FormFieldStories.page,
      IconButtonStories.page,
      KeyCapStories.page,
      SpinnerStories.page,
      LinkStories.page,
      MenuStories.page,
      MenubarStories.page,
      MeterStories.page,
      NavItemStories.page,
      NumberFieldStories.page,
      OptionCardStories.page,
      OtpFieldStories.page,
      PillTabsStories.page,
      PopoverStories.page,
      PreferencesStories.page,
      PreviewCardStories.page,
      ProgressStories.page,
      RadioStories.page,
      RailStories.page,
      SearchFieldStories.page,
      SectionLabelStories.page,
      SegmentedControlStories.page,
      SidebarStories.page,
      SkeletonStories.page,
      SliderStories.page,
      SplitterStories.page,
      SwitchStories.page,
      TagStories.page,
      TextFieldStories.page,
      ToastStories.page,
      ToggleStories.page,
      ToggleGroupStories.page,
      TreeStories.page,
    ]),
  ]);

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
          story.sourceId ?? storyId(group.name, page.name, story.name),
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
