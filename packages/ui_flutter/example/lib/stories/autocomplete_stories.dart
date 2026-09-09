import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Autocomplete` — a text field that suggests as you type.
class AutocompleteStories {
  AutocompleteStories._();

  static const List<String> branches = [
    'main',
    'release/4.2',
    'release/4.1',
    'fix/token-drift',
    'fix/popup-layer',
    'feat/slider',
    'feat/drawer',
  ];

  static const List<String> tags = [
    'design-system',
    'tokens',
    'flutter',
    'gpui',
    'swiftui',
    'parity',
  ];

  static StoryPage get page => StoryPage('Autocomplete', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Bare',
      (context) =>
          const _Live(width: 256, items: tags, placeholder: 'Add a tag'),
    ),
    Story(
      'Mono',
      (context) =>
          const _Live(width: 256, items: branches, placeholder: 'Branch'),
    ),
    Story(
      'Sizes',
      (context) => const Col(
        width: 256,
        children: [
          _Live(items: tags, size: WidgetSize.small),
          _Live(items: tags, size: WidgetSize.medium),
          _Live(items: tags, size: WidgetSize.large),
        ],
      ),
    ),
    Story(
      'States',
      (context) => const Col(
        width: 256,
        children: [
          _Live(items: tags, start: 'tokens'),
          _Live(items: tags, start: 'tok', state: TextFieldState.error),
          Autocomplete(items: tags, value: 'tokens'),
        ],
      ),
    ),
    Story('In a form', (context) => const _InAForm()),
  ]);
}

class _Live extends StatefulWidget {
  const _Live({
    this.width,
    required this.items,
    this.start = '',
    this.placeholder,
    this.size = WidgetSize.medium,
    this.state = TextFieldState.normal,
  });

  final double? width;
  final List<String> items;
  final String start;
  final String? placeholder;
  final WidgetSize size;
  final TextFieldState state;

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  late String _value = widget.start;

  @override
  Widget build(BuildContext context) {
    final Widget field = Autocomplete(
      items: widget.items,
      value: _value,
      placeholder: widget.placeholder,
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
  String _branch = 'feat/';

  @override
  Widget build(BuildContext context) {
    final bool known = AutocompleteStories.branches.contains(_branch);

    return SizedBox(
      width: 320,
      child: FormField(
        label: 'Branch',
        hint: known ? 'Checks out the existing branch' : 'Creates a new branch',
        child: Autocomplete(
          items: AutocompleteStories.branches,
          value: _branch,
          onChanged: (value) => setState(() => _branch = value),
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
      'Autocomplete',
      description:
          'A text field that suggests as you type. The difference from a '
          'Combobox is what the value is: a combobox ends in one of its '
          'options and nothing else; an autocomplete’s value is whatever '
          'was typed, and the list is a shortcut to a likely answer rather '
          'than the set of allowed ones. A branch name, a search query, a tag '
          'that may not exist yet.',
      children: [
        DocSection(
          'Sizes',
          description:
              'It is a TextField wearing the shared popup sheet, and it is '
              'literally that: the same box, the same panel, the same rows. '
              'There is no chevron and no clear — nothing here is opened or '
              'chosen, and the field is emptied the way any text field is.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'small',
                  children: [
                    _Live(
                      width: 224,
                      items: AutocompleteStories.branches,
                      size: WidgetSize.small,
                    ),
                  ],
                ),
                Specimen(
                  'medium',
                  children: [
                    _Live(width: 224, items: AutocompleteStories.branches),
                  ],
                ),
                Specimen(
                  'large',
                  children: [
                    _Live(
                      width: 224,
                      items: AutocompleteStories.branches,
                      size: WidgetSize.large,
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Against a Combobox',
          description:
              'Ask what happens when the reader types something that is not on '
              'the list. If that is an answer, this is the widget. If it is a '
              'mistake, it is a Combobox.',
          children: [
            _Live(
              width: 256,
              items: AutocompleteStories.branches,
              placeholder: 'Branch or new name',
            ),
          ],
        ),
        DocSection(
          'States',
          description:
              'Erroring re-tints to danger like every other text control, so '
              'the border, the wash and the value’s ink move together.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'error',
                  children: [
                    _Live(
                      width: 224,
                      items: AutocompleteStories.tags,
                      start: 'tok',
                      state: TextFieldState.error,
                    ),
                  ],
                ),
                Specimen(
                  'disabled',
                  children: [
                    SizedBox(
                      width: 224,
                      child: Autocomplete(
                        items: AutocompleteStories.tags,
                        value: 'tokens',
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
