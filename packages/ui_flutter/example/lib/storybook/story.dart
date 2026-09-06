import 'package:dazzui/dazzui.dart';

/// The playground is organised the way `apps/storybook` is, and for the same
/// reason: a gallery of six themed pages tells you what the system looks like,
/// but not where a widget's states are written down. One page per widget, an
/// `Overview` that argues for it, and every other story the widget and nothing
/// else.
///
/// The three types below are what Storybook's file conventions are here: a
/// [Story] is one export, a [StoryPage] is one `*.stories.tsx` file, and a
/// [StoryGroup] is the `Tokens/` or `Components/` half of the sidebar.

/// How the canvas presents a story.
///
/// The counterpart of Storybook's `parameters.layout`, and it carries the same
/// three cases: a bare specimen sits in the middle of the canvas, a page brings
/// its own padding, and a story whose subject is fixed to the viewport — a
/// scrim, a stack of toasts — takes the canvas whole.
enum StoryLayout {
  /// A bare specimen, centred. The default, as it is on a component file.
  centered,

  /// A documented page. It scrolls, and pads itself.
  page,

  /// The canvas, filled and unscrolled — what a route overlay would cover.
  fullscreen,
}

/// A story's Storybook id — `components-button--all-variants` — derived the
/// way Storybook derives it from a title and an export name: lower-cased, with
/// every run of anything but a letter or a digit collapsed to one dash.
///
/// The id is the key the React reference is written under, so it is what
/// makes a story here and the story in `apps/storybook` the same story. A
/// name that differs only in case or spacing (`All variants`, `All Variants`)
/// lands on the same id; a name that differs in a word does not.
String storyId(String group, String page, String story) {
  return '${_sanitize('$group/$page')}--${_sanitize(story)}';
}

String _sanitize(String text) {
  return text
      .toLowerCase()
      .replaceAll(RegExp('[^a-z0-9]+'), '-')
      .replaceAll(RegExp(r'^-+|-+$'), '');
}

/// One story: a name in the sidebar and the widget it draws.
class Story {
  const Story(this.name, this.builder, {this.layout = StoryLayout.centered});

  final String name;
  final WidgetBuilder builder;
  final StoryLayout layout;
}

/// One widget's stories — the `Overview` first, then the axes, then the
/// compositions, in the order they are written.
class StoryPage {
  const StoryPage(this.name, this.stories);

  final String name;
  final List<Story> stories;
}

/// A half of the sidebar: `Tokens` or `Components`.
class StoryGroup {
  const StoryGroup(this.name, this.pages);

  final String name;
  final List<StoryPage> pages;
}

/// Where the canvas is pointed: a page and one of its stories.
///
/// A page is addressed by name rather than by index, because the sidebar
/// filters and a filtered list would renumber underneath the selection.
@immutable
class StoryRef {
  const StoryRef(this.page, this.story);

  final String page;
  final String story;

  @override
  bool operator ==(Object other) =>
      other is StoryRef && other.page == page && other.story == story;

  @override
  int get hashCode => Object.hash(page, story);
}
