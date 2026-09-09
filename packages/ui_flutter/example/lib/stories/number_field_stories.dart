import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/NumberField` — a number, with a stepper.
class NumberFieldStories {
  NumberFieldStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('NumberField', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Bare', (context) => const _Live(width: 160)),
    Story(
      'With unit',
      (context) =>
          const _Live(width: 160, unit: 'px', min: 8, max: 24, start: 13),
    ),
    Story(
      'Sizes',
      (context) => const Col(
        width: 160,
        children: [
          _Live(size: WidgetSize.small),
          _Live(size: WidgetSize.medium),
          _Live(size: WidgetSize.large),
        ],
      ),
    ),
    Story(
      'States',
      (context) => const Col(
        width: 160,
        children: [
          _Live(),
          _Live(state: TextFieldState.error),
          SizedBox(child: NumberField(value: 12)),
        ],
      ),
    ),
    Story('In a form', (context) => const _InAForm()),
    Story('In settings', (context) => const _InSettings()),
  ]);
}

class _Live extends StatefulWidget {
  const _Live({
    this.width,
    this.start = 12,
    this.min,
    this.max,
    this.unit,
    this.size = WidgetSize.medium,
    this.state = TextFieldState.normal,
  });

  final double? width;
  final double start;
  final double? min;
  final double? max;
  final String? unit;
  final WidgetSize size;
  final TextFieldState state;

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  late double? _value = widget.start;

  @override
  Widget build(BuildContext context) {
    final Widget field = NumberField(
      value: _value,
      min: widget.min,
      max: widget.max,
      unit: widget.unit,
      size: widget.size,
      state: widget.state,
      onChanged: (value) => setState(() => _value = value),
    );

    return widget.width == null
        ? field
        : SizedBox(width: widget.width, child: field);
  }
}

class _InAForm extends StatefulWidget {
  const _InAForm();

  @override
  State<_InAForm> createState() => _InAFormState();
}

class _InAFormState extends State<_InAForm> {
  double? _port = 5173;

  @override
  Widget build(BuildContext context) {
    final bool missing = _port == null;

    return SizedBox(
      width: 288,
      child: FormField(
        label: 'Dev server port',
        hint: missing ? 'Required' : 'Restart to apply',
        invalid: missing,
        child: NumberField(
          value: _port,
          min: 1024,
          max: 65535,
          state: missing ? TextFieldState.error : TextFieldState.normal,
          onChanged: (value) => setState(() => _port = value),
        ),
      ),
    );
  }
}

class _InSettings extends StatelessWidget {
  const _InSettings();

  @override
  Widget build(BuildContext context) {
    return const SizedBox(
      width: 480,
      child: PreferenceSection(
        label: 'Editor',
        children: [
          PreferenceRow(
            title: 'Font size',
            subtitle: 'Applies to every project.',
            trailing: SizedBox(
              width: 112,
              child: _Live(
                size: WidgetSize.small,
                unit: 'px',
                start: 13,
                min: 8,
                max: 24,
              ),
            ),
          ),
          PreferenceRow(
            title: 'Tab width',
            trailing: SizedBox(
              width: 112,
              child: _Live(size: WidgetSize.small, start: 2, min: 1, max: 8),
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
      'NumberField',
      description:
          'A number, with the two things a plain text field never gets right: '
          'a stepper you can hold down, and a value that stays a number. The '
          'box is the TextField’s — field_box.dart draws it for both — so '
          'a form mixing the two does not step. What is this widget’s own '
          'is the stepper at the trailing edge, and the clamp that keeps the '
          'value inside its bounds.',
      children: [
        DocSection(
          'Sizes',
          description:
              'The stepper takes its room out of the trailing padding rather '
              'than out of the value, and it is stacked rather than a minus '
              'and a plus on either side: a value with a control on both sides '
              'of it reads as three things.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'small',
                  children: [_Live(width: 128, size: WidgetSize.small)],
                ),
                Specimen('medium', children: [_Live(width: 128)]),
                Specimen(
                  'large',
                  children: [_Live(width: 128, size: WidgetSize.large)],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Bounds and units',
          description:
              '`min` and `max` clamp. A unit inside the box is a hint rather '
              'than part of the value — it is the faint ink, and it never '
              'enters what a form submits.',
          children: [
            Col(
              width: 192,
              children: [
                _Live(start: 13, min: 8, max: 24, unit: 'px'),
                _Live(start: 50, unit: '%'),
              ],
            ),
          ],
        ),
        DocSection(
          'States',
          description:
              'Erroring re-tints the control to danger rather than bolting on '
              'a second palette, exactly as a TextField does — the border, the '
              'wash and the value’s ink all move together.',
          children: [
            Specimens(
              children: [
                Specimen('default', children: [_Live(width: 128)]),
                Specimen(
                  'error',
                  children: [_Live(width: 128, state: TextFieldState.error)],
                ),
                Specimen(
                  'disabled',
                  children: [
                    SizedBox(width: 128, child: NumberField(value: 12)),
                  ],
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
