import 'package:dazzui/dazzui.dart';

import 'story.dart';

/// The story tree: the two groups, the pages inside them, and a page's own
/// stories under it once it is open.
///
/// Storybook's sidebar is one tree with a filter over it, and this is the same
/// tree drawn with the package's own [Sidebar], [SidebarGroup], [NavItem] and
/// [SearchField] — the playground is built out of the system it documents, so
/// its furniture answers the theme menu like everything else.
class StoryNavigator extends StatefulWidget {
  const StoryNavigator({
    super.key,
    required this.groups,
    required this.selection,
    required this.onSelected,
  });

  final List<StoryGroup> groups;
  final StoryRef selection;
  final ValueChanged<StoryRef> onSelected;

  @override
  State<StoryNavigator> createState() => _StoryNavigatorState();
}

class _StoryNavigatorState extends State<StoryNavigator> {
  final TextEditingController _query = TextEditingController();

  /// Which pages are open. Storybook opens the page you select and leaves the
  /// rest as they were, so this is a set rather than one name.
  final Set<String> _opened = <String>{};

  @override
  void dispose() {
    _query.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Sidebar(
      header: SearchField(
        controller: _query,
        placeholder: 'Find a widget',
        size: WidgetSize.small,
        onChanged: (_) => setState(() {}),
      ),
      children: [
        for (final StoryGroup group in _matches)
          SidebarGroup(
            label: group.name,
            children: [
              for (final StoryPage page in group.pages) ...[
                NavItem(
                  label: page.name,
                  current: widget.selection.page == page.name,
                  onPressed: () => _open(page),
                ),
                if (_isOpen(page))
                  for (final Story story in page.stories)
                    // The story rows hang off their page rather than sitting
                    // level with it, which is the whole of what makes the list
                    // a tree.
                    Padding(
                      padding: EdgeInsets.only(left: vars.spacing4),
                      child: NavItem(
                        label: story.name,
                        size: WidgetSize.small,
                        current:
                            widget.selection == StoryRef(page.name, story.name),
                        onPressed: () =>
                            widget.onSelected(StoryRef(page.name, story.name)),
                      ),
                    ),
              ],
            ],
          ),
      ],
    );
  }

  /// The groups a query leaves standing.
  ///
  /// A page matches by its own name — and then keeps all of its stories, since
  /// the reader was looking for the widget — or by any story's, in which case
  /// only the stories that matched are worth showing.
  List<StoryGroup> get _matches {
    final String needle = _query.text.trim().toLowerCase();
    if (needle.isEmpty) return widget.groups;

    final List<StoryGroup> groups = [];
    for (final StoryGroup group in widget.groups) {
      final List<StoryPage> pages = [];
      for (final StoryPage page in group.pages) {
        if (page.name.toLowerCase().contains(needle)) {
          pages.add(page);
          continue;
        }
        final List<Story> stories = page.stories
            .where((story) => story.name.toLowerCase().contains(needle))
            .toList();
        if (stories.isNotEmpty) pages.add(StoryPage(page.name, stories));
      }
      if (pages.isNotEmpty) groups.add(StoryGroup(group.name, pages));
    }
    return groups;
  }

  /// A filtered list is already the answer to the question, so it opens
  /// itself; without a query a page is open because it was asked for.
  bool _isOpen(StoryPage page) =>
      _query.text.trim().isNotEmpty || _opened.contains(page.name);

  /// Selecting a page opens it on its `Overview`, which is the one story that
  /// says what the widget is. Selecting it again folds it away.
  void _open(StoryPage page) {
    if (_opened.contains(page.name) && widget.selection.page == page.name) {
      setState(() => _opened.remove(page.name));
      return;
    }

    setState(() => _opened.add(page.name));
    if (page.stories.isNotEmpty) {
      widget.onSelected(StoryRef(page.name, page.stories.first.name));
    }
  }
}
