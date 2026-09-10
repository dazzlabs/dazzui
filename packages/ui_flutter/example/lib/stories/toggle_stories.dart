import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Toggle` — a button that stays down.
class ToggleStories {
  ToggleStories._();

  static StoryPage get page => StoryPage('Toggle', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Pressed',
      (context) => const RowWrap(
        children: [
          Toggle(child: Text('Wrap')),
          Toggle(initiallyPressed: true, child: Text('Wrap')),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => RowWrap(
        children: <Widget>[
          for (final WidgetSize size in <WidgetSize>[
            WidgetSize.tiny,
            WidgetSize.small,
            WidgetSize.medium,
            WidgetSize.large,
          ])
            Toggle(
              size: size,
              initiallyPressed: true,
              variant: ToggleVariant.recessed,
              child: Text(size.namedSize.name),
            ),
        ],
      ),
    ),
    Story(
      'Variants',
      (context) => Col(
        children: <Widget>[
          RowWrap(
            children: <Widget>[
              for (final ToggleVariant variant in ToggleVariant.values)
                Toggle(variant: variant, child: Text(variant.name)),
            ],
          ),
          RowWrap(
            children: <Widget>[
              for (final ToggleVariant variant in ToggleVariant.values)
                Toggle(
                  variant: variant,
                  initiallyPressed: true,
                  child: Text(variant.name),
                ),
            ],
          ),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => RowWrap(
        children: <Widget>[
          for (final ToggleTint tint in ToggleTint.values)
            Toggle(
              tint: tint,
              initiallyPressed: true,
              child: Text(tint.name),
            ),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => const RowWrap(
        children: [
          Toggle(enabled: false, child: Text('Wrap')),
          Toggle(enabled: false, initiallyPressed: true, child: Text('Wrap')),
        ],
      ),
    ),
    Story('A toolbar', (context) => const _Toolbar()),
  ]);
}

class _Toolbar extends StatelessWidget {
  const _Toolbar();

  @override
  Widget build(BuildContext context) {
    return const RowWrap(
      spacing: 4,
      children: <Widget>[
        Toggle(
          semanticsLabel: 'Bold',
          initiallyPressed: true,
          child: Icon(FluentIcons.text_bold_24_regular),
        ),
        Toggle(
          semanticsLabel: 'Italic',
          child: Icon(FluentIcons.text_italic_24_regular),
        ),
        Toggle(
          semanticsLabel: 'Underline',
          child: Icon(FluentIcons.text_underline_24_regular),
        ),
      ],
    );
  }
}

/// The Overview's specimens are shown rather than driven, but a control with
/// no callback draws as disabled — so they are handed one that does nothing.
void _noop(bool _) {}
void _noopString(String _) {}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Toggle',
      description:
          'A button that stays down. Three widgets in this package are one '
          'press away from each other: a Switch commits immediately and reads '
          'as a setting; a SegmentedControl is a choice between its segments '
          'and always has one; a Toggle is neither — a single property that is '
          'on or off, in a row where none of them may be on.',
      children: [
        DocSection(
          'Beside its neighbours',
          description:
              'Reach for a Switch where the thing is a setting in a list and '
              'the change takes effect on the spot. Reach for a '
              'SegmentedControl where the reader is choosing one of several '
              'alternatives and cannot choose none. Reach for this where the '
              'answer is yes or no and the control lives in a toolbar rather '
              'than a form.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'Switch',
                  children: [Switch(value: true, onChanged: _noop)],
                ),
                Specimen(
                  'SegmentedControl',
                  children: [
                    SegmentedControl<String>(
                      items: [
                        SegmentedItem<String>(value: 'left', label: 'Left'),
                        SegmentedItem<String>(value: 'center', label: 'Center'),
                      ],
                      value: 'left',
                      onChanged: _noopString,
                    ),
                  ],
                ),
                Specimen(
                  'Toggle',
                  children: [
                    Toggle(
                      semanticsLabel: 'Bold',
                      initiallyPressed: true,
                      child: Icon(FluentIcons.text_bold_24_regular),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Pressed',
          description:
              'Drawn the way the system draws every selection: the tinted '
              'wash, and the label a weight heavier. Colour alone would be one '
              'grade in a toolbar of glyphs, which is exactly where it is '
              'hardest to see — and the weight is the half that survives for a '
              'reader who cannot tell the two grades apart.',
          children: [
            Specimens(
              children: [
                Specimen('up', children: [Toggle(child: Text('Wrap'))]),
                Specimen(
                  'down',
                  children: [
                    Toggle(initiallyPressed: true, child: Text('Wrap')),
                  ],
                ),
                Specimen(
                  'disabled, down',
                  children: [
                    Toggle(
                      enabled: false,
                      initiallyPressed: true,
                      child: Text('Wrap'),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Glyphs',
          description:
              'Most toggles live in toolbars and most toolbar labels are '
              'glyphs, so semanticsLabel is what a screen reader is given. The '
              'glyph takes the control type’s size, so a row of these lines up '
              'with the buttons beside it rather than with whatever the icon '
              'set ships at.',
          children: [_Toolbar()],
        ),
      ],
    );
  }
}
