import 'package:dazzui/dazzui.dart';

import 'host.dart';
import 'playground_theme.dart';
import 'storybook/navigator.dart';
import 'storybook/story.dart';
import 'storybook/story_index.dart';

void main() {
  runApp(const PlaygroundApp());
}

class PlaygroundApp extends StatelessWidget {
  const PlaygroundApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const Host(
      title: 'DazzUI Playground',
      home: PlaygroundWindow(),
    );
  }
}

/// The playground: the story tree on the left, one story on the canvas, and
/// the theme picker over both.
///
/// The theme is applied with the package's own [Theme] widget on one subtree
/// rather than to the process, which is the point worth demonstrating — themes
/// are per-subtree here, so all twelve can sit on screen at once (see
/// `Tokens/Themes`).
class PlaygroundWindow extends StatefulWidget {
  const PlaygroundWindow({super.key});

  @override
  State<PlaygroundWindow> createState() => _PlaygroundWindowState();
}

class _PlaygroundWindowState extends State<PlaygroundWindow> {
  StoryRef _selection = StoryIndex.first;
  PlaygroundStyle _style = playgroundStyleGroups.first.styles.first;
  bool _dark = false;

  @override
  Widget build(BuildContext context) {
    final ThemeData data = playgroundThemeData(_style, dark: _dark);

    return Theme(
      data: data,
      child: ColoredBox(
        color: data.vars.colorCanvas,
        child: Column(
          children: [
            _Toolbar(
              style: _style,
              dark: _dark,
              onStyleChanged: (style) => setState(() => _style = style),
              onDarkChanged: (dark) => setState(() => _dark = dark),
            ),
            const Divider(height: 1),
            Expanded(
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  StoryNavigator(
                    groups: StoryIndex.groups,
                    selection: _selection,
                    onSelected: (ref) => setState(() => _selection = ref),
                  ),
                  const VerticalDivider(width: 1),
                  Expanded(child: _Canvas(selection: _selection)),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _Toolbar extends StatelessWidget {
  const _Toolbar({
    required this.style,
    required this.dark,
    required this.onStyleChanged,
    required this.onDarkChanged,
  });

  final PlaygroundStyle style;
  final bool dark;
  final ValueChanged<PlaygroundStyle> onStyleChanged;
  final ValueChanged<bool> onDarkChanged;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      height: vars.frameTitlebarSize,
      color: vars.colorSurfaceMuted,
      padding: EdgeInsets.symmetric(horizontal: vars.spacing4),
      child: Row(
        spacing: vars.spacing3,
        children: [
          // The title is what gives way in a narrow window: a style and a
          // brightness are the controls, and a name that truncates
          // costs less than a control that overflows.
          Expanded(
            child: Row(
              spacing: vars.spacing3,
              children: [
                Flexible(
                  child: Text(
                    'DazzUI',
                    overflow: TextOverflow.ellipsis,
                    style: vars.titleSmall.copyWith(color: vars.colorContent),
                  ),
                ),
                const Badge(
                  size: WidgetSize.small,
                  variant: BadgeVariant.tinted,
                  child: Text('Flutter'),
                ),
              ],
            ),
          ),
          // A style and a brightness rather than seventy names: that is the
          // shape of the graph — a family is a mode, `dark` is a mode — and
          // the styles, thirty-five of them in three groups, are a list to
          // choose from rather than a row of segments.
          SizedBox(
            width: vars.frameAsideWidth,
            child: Select<PlaygroundStyle>(
              value: style,
              onChanged: (next) {
                if (next != null) onStyleChanged(next);
              },
              groups: [
                for (final group in playgroundStyleGroups)
                  SelectGroup(
                    label: group.title,
                    options: [
                      for (final candidate in group.styles)
                        SelectOption(value: candidate, label: candidate.title),
                    ],
                  ),
              ],
            ),
          ),
          SegmentedControl<bool>(
            value: dark,
            onChanged: onDarkChanged,
            items: const [
              SegmentedItem(value: false, label: 'Light'),
              SegmentedItem(value: true, label: 'Dark'),
            ],
          ),
        ],
      ),
    );
  }
}

/// One story, presented the way its layout asks for.
class _Canvas extends StatelessWidget {
  const _Canvas({required this.selection});

  final StoryRef selection;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return ColoredBox(
      color: vars.colorSurface,
      child: Column(
        children: [
          _Breadcrumb(selection: selection),
          const Divider(height: 1),
          Expanded(child: StoryStage(selection: selection)),
        ],
      ),
    );
  }
}

/// The canvas body. It is public because the widget test drives it directly:
/// pumping one story is what says a story draws, and walking the sidebar to
/// reach it would only be testing the sidebar.
class StoryStage extends StatelessWidget {
  const StoryStage({super.key, required this.selection});

  final StoryRef selection;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final Story? story = StoryIndex.story(selection);

    if (story == null) {
      return const Center(child: EmptyState(title: 'No story selected'));
    }

    return switch (story.layout) {
      // A bare specimen sits in the middle of the canvas rather than in its
      // corner, which is what `layout: 'centered'` buys a component file.
      StoryLayout.centered => SingleChildScrollView(
        padding: EdgeInsets.all(vars.spacing8),
        child: Center(child: story.builder(context)),
      ),
      // The page brings its own padding, as `layout: 'fullscreen'` does on an
      // `Overview`.
      StoryLayout.page => SingleChildScrollView(
        child: Align(
          alignment: Alignment.topLeft,
          child: story.builder(context),
        ),
      ),
      // A scrim or a stack of toasts is fixed to the canvas the way it would
      // be to the viewport, so it takes the whole of it, unscrolled.
      StoryLayout.fullscreen => story.builder(context),
    };
  }
}

/// Where the canvas is pointed, in words.
class _Breadcrumb extends StatelessWidget {
  const _Breadcrumb({required this.selection});

  final StoryRef selection;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      height: vars.frameTitlebarSize,
      color: vars.colorSurfaceMuted,
      padding: EdgeInsets.symmetric(horizontal: vars.spacing4),
      child: Row(
        spacing: vars.spacing2,
        children: [
          Text(
            selection.page,
            style: vars.labelMedium.copyWith(color: vars.colorContent),
          ),
          Text(
            '/',
            style: vars.captionMedium.copyWith(color: vars.colorContentFaint),
          ),
          Text(
            selection.story,
            style: vars.captionMedium.copyWith(color: vars.colorContentMuted),
          ),
        ],
      ),
    );
  }
}
