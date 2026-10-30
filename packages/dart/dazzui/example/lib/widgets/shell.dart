import 'dart:math' as math;

import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/lib/scene.dart';
import 'package:dazzui_example/lib/story.dart';
import 'package:dazzui_example/lib/themes.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/services.dart';

import '../stories/index.dart';
import 'brand.dart';
import 'layout.dart';
import 'resizer.dart';
import 'theme_panel.dart';

const ColumnWidth _list = ColumnWidth(initial: 272, min: 200, max: 420);
const ColumnWidth _themes = ColumnWidth(initial: 336, min: 280, max: 560);

double _previewWidth(ThemeVariables vars) =>
    256 *
    math.max(1, vars.controlMediumSize / themeVariables.controlMediumSize);

/// The example, as a desktop application's window: the component list on
/// the left, the document under its toolbar, and the theme panel on the
/// right — the same window the React example draws, column for column.
///
/// It is handed the page and the theme and owns only the window's own
/// state: the search, which columns are open, and how wide.
class Shell extends StatefulWidget {
  const Shell({
    super.key,
    required this.page,
    required this.open,
    required this.theme,
    required this.setTheme,
    this.omarchy,
    this.followingOmarchy = false,
    this.setFollowingOmarchy,
  });

  /// The component being read, or null on the overview.
  final String? page;
  final ValueChanged<String?> open;
  final ThemeChoice theme;
  final ValueChanged<ThemeChoice> setTheme;

  /// The theme the Omarchy machine this runs on is wearing, if it is one;
  /// and whether the window is drawn in it rather than in [theme].
  final OmarchyPalette? omarchy;
  final bool followingOmarchy;
  final ValueChanged<bool>? setFollowingOmarchy;

  @override
  State<Shell> createState() => _ShellState();
}

class _ShellState extends State<Shell> {
  String _query = '';
  bool _listOpen = true;
  bool _themesOpen = true;
  double _listWidth = _list.initial;
  double _themesWidth = _themes.initial;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final Story? story = storyOf(widget.page);

    return CallbackShortcuts(
      bindings: {
        const SingleActivator(LogicalKeyboardKey.keyB, meta: true): () =>
            setState(() => _listOpen = !_listOpen),
        const SingleActivator(LogicalKeyboardKey.keyB, control: true): () =>
            setState(() => _listOpen = !_listOpen),
      },
      child: Focus(
        autofocus: true,
        child: ColoredBox(
          color: vars.colorSurface,
          child: DefaultTextStyle(
            style: vars.bodyMedium.copyWith(color: vars.colorContent),
            child: IconTheme(
              data: IconThemeData(
                color: vars.colorContent,
                size: vars.iconMedium,
              ),
              child: LayoutBuilder(
                builder: (context, constraints) {
                  final double document =
                      _previewWidth(vars) +
                      vars.spacing6 * 2 +
                      vars.spacing4 * 2;
                  final bool showList =
                      _listOpen &&
                      constraints.maxWidth >= _listWidth + document;
                  final double list = showList ? _listWidth : 0;
                  // The panel's room is measured against the window with the
                  // list open, not with the list's animated width: a sidebar
                  // the reader puts away must not make a second panel appear
                  // behind it mid-sweep.
                  final bool showThemes =
                      _themesOpen &&
                      constraints.maxWidth >=
                          _listWidth + _themesWidth + document;
                  final double themes = showThemes ? _themesWidth : 0;
                  return Stack(
                    children: [
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          _List(
                            width: _listWidth,
                            collapsed: !showList,
                            query: _query,
                            page: widget.page,
                            onQuery: (query) => setState(() => _query = query),
                            onOpen: widget.open,
                          ),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.stretch,
                              children: [
                                _Toolbar(
                                  story: story,
                                  listOpen: showList,
                                  themesOpen: showThemes,
                                  onToggleList: () => setState(() {
                                    _listOpen = !showList;
                                    if (_listOpen && !showThemes) {
                                      _themesOpen = false;
                                    }
                                  }),
                                  onToggleThemes: () => setState(() {
                                    _themesOpen = !showThemes;
                                    if (_themesOpen &&
                                        constraints.maxWidth <
                                            _listWidth +
                                                _themesWidth +
                                                document) {
                                      _listOpen = false;
                                    }
                                  }),
                                ),
                                Expanded(
                                  child: story == null
                                      ? _Overview(
                                          stories: matching(_query),
                                          onOpen: widget.open,
                                        )
                                      : _Document(
                                          key: ValueKey(story.id),
                                          story: story,
                                        ),
                                ),
                              ],
                            ),
                          ),
                          if (showThemes)
                            ThemePanel(
                              theme: widget.theme,
                              width: themes,
                              onChanged: widget.setTheme,
                              omarchy: widget.omarchy,
                              followingOmarchy: widget.followingOmarchy,
                              onFollowOmarchy: widget.setFollowingOmarchy,
                            ),
                        ],
                      ),
                      if (showList)
                        Positioned(
                          top: 0,
                          bottom: 0,
                          left: list - context.hairlineWidth,
                          child: Resizer(
                            outward: 1,
                            column: _list,
                            width: _listWidth,
                            lineAtStart: true,
                            label: 'Resize the component list',
                            onChanged: (width) =>
                                setState(() => _listWidth = width),
                          ),
                        ),
                      if (showThemes)
                        Positioned(
                          top: 0,
                          bottom: 0,
                          left:
                              constraints.maxWidth -
                              themes -
                              vars.spacing1 -
                              context.hairlineWidth,
                          child: Resizer(
                            outward: -1,
                            column: _themes,
                            width: _themesWidth,
                            label: 'Resize the theme panel',
                            onChanged: (width) =>
                                setState(() => _themesWidth = width),
                          ),
                        ),
                    ],
                  );
                },
              ),
            ),
          ),
        ),
      ),
    );
  }
}

/* The component list ----------------------------------------------------- */

class _List extends StatelessWidget {
  const _List({
    required this.width,
    required this.collapsed,
    required this.query,
    required this.page,
    required this.onQuery,
    required this.onOpen,
  });

  /// The width the column has when open. Put away, it keeps this one, so
  /// opening it again lands where the divider was dragged to.
  final double width;
  final bool collapsed;
  final String query;
  final String? page;
  final ValueChanged<String> onQuery;
  final ValueChanged<String?> onOpen;

  @override
  Widget build(BuildContext context) {
    return Sidebar(
      width: width,
      collapsed: collapsed,
      header: Brand(onPressed: () => onOpen(null)),
      children: [
        SidebarGroup(
          children: [
            SearchField(
              value: query,
              onChanged: onQuery,
              placeholder: 'Search',
              size: WidgetSize.small,
            ),
          ],
        ),
        SidebarGroup(
          children: [
            NavItem(
              label: 'Overview',
              icon: FluentIcons.home_20_regular,
              size: WidgetSize.small,
              current: page == null,
              trailing: Text('${stories.length}'),
              onPressed: () => onOpen(null),
            ),
          ],
        ),
        SidebarGroup(
          label: 'Components',
          children: [
            for (final story in matching(query))
              NavItem(
                label: story.name,
                size: WidgetSize.small,
                current: story.id == page,
                onPressed: () => onOpen(story.id),
              ),
          ],
        ),
      ],
    );
  }
}

/* The toolbar ---------------------------------------------------------- */

class _Toolbar extends StatelessWidget {
  const _Toolbar({
    required this.story,
    required this.listOpen,
    required this.themesOpen,
    required this.onToggleList,
    required this.onToggleThemes,
  });

  final Story? story;
  final bool listOpen;
  final bool themesOpen;
  final VoidCallback onToggleList;
  final VoidCallback onToggleThemes;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final Story? story = this.story;
    return Container(
      height: vars.frameTitlebarSize,
      padding: EdgeInsets.symmetric(horizontal: vars.spacing4),
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: vars.colorBorder,
            width: context.hairlineWidth,
          ),
        ),
      ),
      child: Row(
        spacing: vars.spacing2,
        children: [
          Tooltip(
            label: 'Toggle sidebar  ⌘B',
            side: PopoverSide.bottom,
            child: IconButton(
              icon: const Icon(FluentIcons.panel_left_20_regular),
              semanticsLabel: 'Toggle sidebar',
              onPressed: onToggleList,
            ),
          ),
          Padding(
            padding: EdgeInsetsDirectional.only(start: vars.spacing1),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.baseline,
              textBaseline: TextBaseline.alphabetic,
              spacing: vars.spacing2,
              children: [
                Text(
                  story?.name ?? 'Overview',
                  style: TextStyle(fontWeight: vars.labelStrong.fontWeight),
                ),
                Text(
                  story == null
                      ? '${stories.length} components'
                      : '${story.examples.length} examples',
                  style: TextStyle(
                    color: vars.colorContentSecondary,
                    fontSize: vars.captionLarge.fontSize,
                  ),
                ),
              ],
            ),
          ),
          const Spacer(),
          Tooltip(
            label: 'Toggle theme panel',
            side: PopoverSide.bottom,
            child: IconButton(
              icon: const Icon(FluentIcons.panel_right_20_regular),
              semanticsLabel: 'Toggle theme panel',
              onPressed: onToggleThemes,
            ),
          ),
        ],
      ),
    );
  }
}

/* The document ---------------------------------------------------------- */

EdgeInsets _pagePadding(ThemeVariables vars) => EdgeInsets.only(
  top: vars.spacing5,
  left: vars.spacing6,
  right: vars.spacing6,
  bottom: vars.spacing10,
);

/// Every component at once, each by the one scene that stands for it.
class _Overview extends StatelessWidget {
  const _Overview({required this.stories, required this.onOpen});

  final List<Story> stories;
  final ValueChanged<String> onOpen;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final double gap = vars.spacing3;
    return SingleChildScrollView(
      padding: _pagePadding(vars),
      child: LayoutBuilder(
        builder: (context, constraints) {
          // Roomier theme profiles need roomier previews as well. Keep the
          // default 16rem track, scaling it with the regular control height.
          final double track = _previewWidth(vars);
          final int columns = ((constraints.maxWidth + gap) / (track + gap))
              .floor()
              .clamp(1, 99);
          return Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            spacing: gap,
            children: [
              for (int start = 0; start < stories.length; start += columns)
                StretchRow(
                  gap: gap,
                  columns: columns,
                  children: [
                    for (
                      int i = start;
                      i < math.min(start + columns, stories.length);
                      i++
                    )
                      _Tile(story: stories[i], onOpen: onOpen),
                  ],
                ),
            ],
          );
        },
      ),
    );
  }
}

class _Tile extends StatelessWidget {
  const _Tile({required this.story, required this.onOpen});

  final Story story;
  final ValueChanged<String> onOpen;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final Widget stage = ConstrainedBox(
      // The web's `min-height: 8rem`.
      constraints: const BoxConstraints(minHeight: 8 * 16),
      child: Center(child: SceneView(scene: story.preview)),
    );
    final Widget name = Button(
      variant: ButtonVariant.plain,
      expand: true,
      onPressed: () => onOpen(story.id),
      child: Text(story.name),
    );
    return Card(
      variant: CardVariant.outlined,
      // Laid out once to learn its height, then again at its row's: the
      // stage takes whatever the row adds, as `flex: 1` does on the web.
      child: LayoutBuilder(
        builder: (context, constraints) => Column(
          mainAxisSize: constraints.hasBoundedHeight
              ? MainAxisSize.max
              : MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          // A medium card's own gap, which on the web wins over the tile's.
          spacing: vars.spacing3,
          children: [
            if (constraints.hasBoundedHeight) Expanded(child: stage) else stage,
            name,
          ],
        ),
      ),
    );
  }
}

/// One component: what it is for, then each example on a stage of its own.
class _Document extends StatelessWidget {
  const _Document({super.key, required this.story});

  final Story story;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    return SingleChildScrollView(
      padding: _pagePadding(vars),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        spacing: vars.spacing6,
        children: [
          Align(
            alignment: AlignmentDirectional.centerStart,
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 640),
              child: Text(
                story.summary,
                style: TextStyle(color: vars.colorContentSecondary),
              ),
            ),
          ),
          for (final example in story.examples)
            Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              spacing: vars.spacing2,
              children: [
                SectionLabel(example.title),
                Card(
                  variant: CardVariant.outlined,
                  size: WidgetSize.large,
                  child: ConstrainedBox(
                    // The web's `min-height: 10rem`.
                    constraints: const BoxConstraints(minHeight: 10 * 16),
                    child: Center(child: SceneView(scene: example.demo)),
                  ),
                ),
                if (example.note != null)
                  Text(
                    example.note!,
                    style: TextStyle(
                      color: vars.colorContentSecondary,
                      fontSize: vars.captionLarge.fontSize,
                    ),
                  ),
              ],
            ),
        ],
      ),
    );
  }
}
