import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Combobox` — a choice out of a list too long to scroll.
class ComboboxStories {
  ComboboxStories._();

  static const List<ComboboxOption<String>> zones = [
    ComboboxOption(value: 'utc', label: 'UTC'),
    ComboboxOption(value: 'lisbon', label: 'Europe/Lisbon'),
    ComboboxOption(value: 'london', label: 'Europe/London'),
    ComboboxOption(value: 'berlin', label: 'Europe/Berlin'),
    ComboboxOption(value: 'shanghai', label: 'Asia/Shanghai'),
    ComboboxOption(value: 'tokyo', label: 'Asia/Tokyo'),
    ComboboxOption(value: 'sydney', label: 'Australia/Sydney'),
    ComboboxOption(value: 'new_york', label: 'America/New_York'),
    ComboboxOption(value: 'sao_paulo', label: 'America/Sao_Paulo'),
  ];

  static const List<ComboboxGroup<String>> grouped = [
    ComboboxGroup(
      label: 'Europe',
      options: [
        ComboboxOption(value: 'lisbon', label: 'Europe/Lisbon'),
        ComboboxOption(value: 'london', label: 'Europe/London'),
        ComboboxOption(value: 'berlin', label: 'Europe/Berlin'),
      ],
    ),
    ComboboxGroup(
      label: 'Asia',
      options: [
        ComboboxOption(value: 'shanghai', label: 'Asia/Shanghai'),
        ComboboxOption(value: 'tokyo', label: 'Asia/Tokyo'),
      ],
    ),
    ComboboxGroup(
      label: 'Americas',
      options: [
        ComboboxOption(value: 'new_york', label: 'America/New_York'),
        ComboboxOption(value: 'sao_paulo', label: 'America/Sao_Paulo'),
      ],
    ),
  ];

  static StoryPage get page => StoryPage('Combobox', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Bare', (context) => const _Live(width: 256)),
    Story('Grouped', (context) => const _Live(width: 256, group: true)),
    Story(
      'Sizes',
      (context) => const Col(
        width: 256,
        children: [
          _Live(start: 'utc', size: WidgetSize.small),
          _Live(start: 'utc', size: WidgetSize.medium),
          _Live(start: 'utc', size: WidgetSize.large),
        ],
      ),
    ),
    Story(
      'States',
      (context) => const Col(
        width: 256,
        children: [
          _Live(start: 'utc'),
          _Live(state: TextFieldState.error),
          Combobox<String>(options: ComboboxStories.zones, value: 'utc'),
        ],
      ),
    ),
    Story('In a form', (context) => const _InAForm()),
  ]);
}

class _Live extends StatefulWidget {
  const _Live({
    this.width,
    this.start,
    this.group = false,
    this.size = WidgetSize.medium,
    this.state = TextFieldState.normal,
  });

  final double? width;
  final String? start;
  final bool group;
  final WidgetSize size;
  final TextFieldState state;

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  late String? _value = widget.start;

  @override
  Widget build(BuildContext context) {
    final Widget field = Combobox<String>(
      options: widget.group ? const [] : ComboboxStories.zones,
      groups: widget.group ? ComboboxStories.grouped : const [],
      value: _value,
      placeholder: 'Search time zones…',
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
  String? _zone = 'london';

  @override
  Widget build(BuildContext context) {
    final bool missing = _zone == null;

    return SizedBox(
      width: 288,
      child: FormField(
        label: 'Time zone',
        hint: missing ? 'Pick one' : 'Used for every schedule',
        invalid: missing,
        child: Combobox<String>(
          groups: ComboboxStories.grouped,
          value: _zone,
          state: missing ? TextFieldState.error : TextFieldState.normal,
          onChanged: (value) => setState(() => _zone = value),
        ),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Combobox',
      description:
          'A choice out of a list too long to scroll: the reader types to '
          'narrow it, and picks one of what is left. It is a Select with a '
          'query in front of it, and that is the only question worth asking '
          'when choosing between them — a list of six is a select, a list of '
          'six hundred is this. Both end in one of the options; neither lets a '
          'reader enter something that is not on the list, which is what '
          'separates them from an Autocomplete.',
      children: [
        DocSection(
          'Sizes',
          description:
              'The box is the TextField’s and the list is the MenuPanel, '
              'so a combobox in a form is the same box as the field beside it '
              'and the same panel a Menu opens.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'small',
                  children: [
                    _Live(width: 224, start: 'utc', size: WidgetSize.small),
                  ],
                ),
                Specimen('medium', children: [_Live(width: 224, start: 'utc')]),
                Specimen(
                  'large',
                  children: [
                    _Live(width: 224, start: 'utc', size: WidgetSize.large),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Grouped',
          description:
              'Grouped or flat, never both: a list read one way would silently '
              'lose the other. The filtering runs inside the groups and drops '
              'the ones it leaves empty — a heading over nothing is worse than '
              'no heading.',
          children: [_Live(width: 256, group: true)],
        ),
        DocSection(
          'States',
          description:
              'Erroring re-tints the control to danger, exactly as a TextField '
              'does. The empty line is prose in a panel of rows, so it takes '
              'the row’s inset and the faint ink rather than looking like '
              'a row that cannot be picked.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'error',
                  children: [_Live(width: 224, state: TextFieldState.error)],
                ),
                Specimen(
                  'disabled',
                  children: [
                    SizedBox(
                      width: 224,
                      child: Combobox<String>(
                        options: ComboboxStories.zones,
                        value: 'utc',
                      ),
                    ),
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
