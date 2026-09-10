import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/ToggleGroup` — several toggles joined into one control.
class ToggleGroupStories {
  ToggleGroupStories._();

  static const List<ToggleItem<String>> styles = <ToggleItem<String>>[
    ToggleItem<String>(
      value: 'bold',
      semanticsLabel: 'Bold',
      label: Icon(FluentIcons.text_bold_24_regular),
    ),
    ToggleItem<String>(
      value: 'italic',
      semanticsLabel: 'Italic',
      label: Icon(FluentIcons.text_italic_24_regular),
    ),
    ToggleItem<String>(
      value: 'underline',
      semanticsLabel: 'Underline',
      label: Icon(FluentIcons.text_underline_24_regular),
    ),
  ];

  static const List<ToggleItem<String>> show = <ToggleItem<String>>[
    ToggleItem<String>(value: 'whitespace', label: Text('Whitespace')),
    ToggleItem<String>(value: 'minimap', label: Text('Minimap')),
    ToggleItem<String>(value: 'blame', label: Text('Blame')),
  ];

  static StoryPage get page => StoryPage('ToggleGroup', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Glyphs', (context) => const _Styles()),
    Story('Words', (context) => const _Show()),
    Story('Single', (context) => const _Show(multiple: false)),
    Story(
      'Variants',
      (context) => Col(
        children: <Widget>[
          for (final ToggleVariant variant in <ToggleVariant>[
            ToggleVariant.recessed,
            ToggleVariant.normal,
            ToggleVariant.outlined,
            ToggleVariant.plain,
          ])
            ToggleGroup<String>(
              items: show,
              value: const <String>['whitespace'],
              variant: variant,
              onChanged: (_) {},
            ),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        children: <Widget>[
          for (final ToggleTint tint in ToggleTint.values)
            ToggleGroup<String>(
              items: show,
              value: const <String>['whitespace'],
              tint: tint,
              onChanged: (_) {},
            ),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => ToggleGroup<String>(
        items: const <ToggleItem<String>>[
          ToggleItem<String>(value: 'whitespace', label: Text('Whitespace')),
          ToggleItem<String>(value: 'minimap', label: Text('Minimap')),
          ToggleItem<String>(
            value: 'blame',
            label: Text('Blame'),
            enabled: false,
          ),
        ],
        value: const <String>['whitespace'],
        onChanged: (_) {},
      ),
    ),
  ]);
}

/// A specimen that is shown rather than driven still needs a callback: a
/// control with none draws as disabled.
void _noop(String _) {}

class _Styles extends StatefulWidget {
  const _Styles();

  @override
  State<_Styles> createState() => _StylesState();
}

class _StylesState extends State<_Styles> {
  List<String> _on = <String>['bold'];

  @override
  Widget build(BuildContext context) {
    return ToggleGroup<String>(
      items: ToggleGroupStories.styles,
      value: _on,
      onChanged: (List<String> next) => setState(() => _on = next),
    );
  }
}

class _Show extends StatefulWidget {
  const _Show({this.multiple = true});

  final bool multiple;

  @override
  State<_Show> createState() => _ShowState();
}

class _ShowState extends State<_Show> {
  List<String> _on = <String>['whitespace'];

  @override
  Widget build(BuildContext context) {
    return ToggleGroup<String>(
      items: ToggleGroupStories.show,
      multiple: widget.multiple,
      value: _on,
      onChanged: (List<String> next) => setState(() => _on = next),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'ToggleGroup',
      description:
          'Several toggles that belong together, joined into one control. '
          'Joined rather than merely adjacent: the corners are rounded at the '
          'ends of the run only and the shared edges collapse to one, which is '
          'what says the group is one thing with several states rather than '
          'three buttons that happen to touch.',
      children: [
        DocSection(
          'Beside a SegmentedControl',
          description:
              'This is where the two part company. A segmented control is a '
              'choice between its segments and always has exactly one; a '
              'toggle group can have several on at once, or none. Drawing a '
              'mandatory single choice with this one would let the reader '
              'clear a choice that cannot be cleared.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'SegmentedControl — one, always',
                  children: [
                    SegmentedControl<String>(
                      items: [
                        SegmentedItem<String>(value: 'source', label: 'Source'),
                        SegmentedItem<String>(
                          value: 'preview',
                          label: 'Preview',
                        ),
                      ],
                      value: 'source',
                      onChanged: _noop,
                    ),
                  ],
                ),
                Specimen(
                  'ToggleGroup — any number, including none',
                  children: [_Styles()],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'The collapsed edge',
          description:
              'A member that is not first drops its leading side, so the rule '
              'between two members is one line rather than two. That is '
              'border-collapse done the only way a row of boxes can do it, and '
              'it is what makes the run read as one control.',
          children: [_Show()],
        ),
      ],
    );
  }
}
