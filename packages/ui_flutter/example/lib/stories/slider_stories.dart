import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Slider` — a value picked along a range.
class SliderStories {
  SliderStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Slider', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Bare', (context) => const _Live(width: 288)),
    Story('With value', (context) => const _Live(width: 320, printed: true)),
    Story(
      'Range',
      (context) => const _Live(width: 320, values: [20, 70], printed: true),
    ),
    Story(
      'Sizes',
      (context) => const Col(
        width: 320,
        children: [
          _Live(size: WidgetSize.small),
          _Live(size: WidgetSize.medium),
          _Live(size: WidgetSize.large),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        width: 320,
        children: [
          for (final tint in SliderTint.values)
            _Live(values: const [60], tint: tint),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => const SizedBox(
        width: 320,
        child: Slider(values: [40], valueLabel: '40'),
      ),
    ),
    Story('Interactive', (context) => const _FontSize()),
    Story('In settings', (context) => const _InSettings()),
  ]);
}

/// The bare slider is stateful everywhere it appears: a control that did not
/// move under the pointer would be a picture of one.
class _Live extends StatefulWidget {
  const _Live({
    this.width,
    this.values = const [40],
    this.size = WidgetSize.medium,
    this.tint = SliderTint.primary,
    this.printed = false,
  });

  final double? width;
  final List<double> values;
  final WidgetSize size;
  final SliderTint tint;
  final bool printed;

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  late List<double> _values = widget.values;

  @override
  Widget build(BuildContext context) {
    final Widget slider = Slider(
      values: _values,
      size: widget.size,
      tint: widget.tint,
      valueLabel: widget.printed
          ? _values.map((value) => value.round().toString()).join(' – ')
          : null,
      onChanged: (value) => setState(() => _values = value),
    );

    return widget.width == null
        ? slider
        : SizedBox(width: widget.width, child: slider);
  }
}

class _FontSize extends StatefulWidget {
  const _FontSize();

  @override
  State<_FontSize> createState() => _FontSizeState();
}

class _FontSizeState extends State<_FontSize> {
  double _size = 13;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 352,
      child: FormField(
        label: 'Editor font size',
        hint: '${_size.round()}px',
        child: Slider(
          values: [_size],
          min: 8,
          max: 24,
          valueLabel: '${_size.round()}px',
          onChanged: (value) => setState(() => _size = value.first),
        ),
      ),
    );
  }
}

class _InSettings extends StatefulWidget {
  const _InSettings();

  @override
  State<_InSettings> createState() => _InSettingsState();
}

class _InSettingsState extends State<_InSettings> {
  double _font = 13;
  double _width = 240;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 480,
      child: PreferenceSection(
        label: 'Appearance',
        children: [
          PreferenceRow(
            title: 'Editor font size',
            subtitle: 'Applies to every open project.',
            trailing: SizedBox(
              width: 160,
              child: Slider(
                values: [_font],
                min: 8,
                max: 24,
                size: WidgetSize.small,
                onChanged: (value) => setState(() => _font = value.first),
              ),
            ),
          ),
          PreferenceRow(
            title: 'Sidebar width',
            trailing: SizedBox(
              width: 160,
              child: Slider(
                values: [_width],
                min: 180,
                max: 360,
                size: WidgetSize.small,
                onChanged: (value) => setState(() => _width = value.first),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Slider',
      description:
          'A value picked along a range. It owns the parts that make one '
          'usable rather than merely draggable — the arrow keys and their '
          'large step, Home and End, the press on the track that jumps the '
          'nearest thumb, and, on a range, which thumb the pointer actually '
          'grabbed when the two are on top of each other. Reach for it where '
          'the reader is feeling for a value rather than stating one; a number '
          'they know is a NumberField.',
      children: [
        DocSection(
          'Sizes',
          description:
              'The row’s height comes off the control profile, so a '
              'slider in a form sits on the same rhythm as the fields around '
              'it. The groove and the thumb are the widget’s own tokens, '
              'and both are even on every size for the reason the radio’s '
              'ring and dot are.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'small',
                  children: [_Live(width: 240, size: WidgetSize.small)],
                ),
                Specimen('medium', children: [_Live(width: 240)]),
                Specimen(
                  'large',
                  children: [_Live(width: 240, size: WidgetSize.large)],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Range',
          description:
              'A range is the same widget: pass more than one value and it '
              'draws a thumb per value. There is no second widget, because '
              'everything but the number of thumbs is identical.',
          children: [
            Col(
              width: 320,
              children: [
                _Live(printed: true),
                _Live(values: [20, 70], printed: true),
              ],
            ),
          ],
        ),
        DocSection(
          'Disabled',
          description:
              'It flattens to the groove like every other control — no lift, '
              'no accent, one dead look — rather than becoming a translucent '
              'ghost of the live one.',
          children: [
            SizedBox(
              width: 288,
              child: Slider(values: [40], valueLabel: '40'),
            ),
          ],
        ),
      ],
    );
  }
}
