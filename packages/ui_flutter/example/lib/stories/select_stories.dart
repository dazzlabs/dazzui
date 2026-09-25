import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Select` — a choice out of a list too long to lay out.
class SelectStories {
  SelectStories._();

  /// The same four options the React page draws, so the two ports are
  /// measuring one specimen rather than two that resemble each other.
  static const List<SelectOption<String>> languages = [
    SelectOption(value: 'zh', label: 'Chinese (Simplified)'),
    SelectOption(value: 'ja', label: 'Japanese'),
    SelectOption(value: 'de', label: 'German'),
    SelectOption(value: 'fr', label: 'French'),
  ];

  static const List<SelectGroup<String>> grouped = [
    SelectGroup(
      label: 'Recently used',
      options: [
        SelectOption(value: 'zh', label: 'Chinese (Simplified)'),
        SelectOption(value: 'ja', label: 'Japanese'),
      ],
    ),
    SelectGroup(
      label: 'All languages',
      options: [
        SelectOption(value: 'de', label: 'German'),
        SelectOption(value: 'fr', label: 'French'),
        SelectOption(value: 'es', label: 'Spanish'),
        SelectOption(value: 'pt', label: 'Portuguese'),
      ],
    ),
  ];

  static const double width = 352;

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Select', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Default', (context) => const _Live(start: 'zh')),
    Story('Disabled', (context) => const _Live(start: 'zh', live: false)),
    // `Error state` rather than `Error`: the story id is the stand-in key
    // every platform is held to, and React's export is named `ErrorState`
    // with its display name shortened — so the id React wrote is this one.
    Story('Error state', (context) => const _Live(start: 'zh', error: true)),
    Story(
      'Placeholder',
      (context) => const _Live(placeholder: 'Choose a language…'),
    ),
    Story(
      'Sizes',
      (context) => Col(
        width: width,
        children: [
          for (final WidgetSize size in sizes) _Live(start: 'zh', size: size),
        ],
      ),
    ),
    Story('Beside a button', (context) => const _BesideAButton()),
    Story(
      'Tints',
      (context) => Col(
        width: width,
        children: [
          for (final TextFieldTint tint in TextFieldTint.values)
            _Live(start: 'zh', tint: tint),
        ],
      ),
    ),
    Story('Grouped', (context) => const _Live(start: 'zh', groups: true)),
    Story('In a form field', (context) => const _InAFormField()),
    Story('In a settings row', (context) => const _InASettingsRow()),
  ]);
}

/// A select whose value the story owns, so choosing a row moves the trigger.
class _Live extends StatefulWidget {
  const _Live({
    this.start,
    this.placeholder = 'Choose…',
    this.size = WidgetSize.medium,
    this.error = false,
    this.tint = TextFieldTint.primary,
    this.groups = false,
    this.live = true,
  });

  final String? start;
  final String placeholder;
  final WidgetSize size;
  final bool error;
  final TextFieldTint tint;
  final bool groups;

  /// A select with no handler is a disabled one, so the two live on one
  /// widget: the difference between the default story and the disabled one is
  /// this flag rather than a second class.
  final bool live;

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  late String? _value = widget.start;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: SelectStories.width,
      child: Select<String>(
        options: widget.groups ? const [] : SelectStories.languages,
        groups: widget.groups ? SelectStories.grouped : const [],
        value: _value,
        placeholder: widget.placeholder,
        size: widget.size,
        state: widget.error ? TextFieldState.error : TextFieldState.normal,
        tint: widget.tint,
        onChanged: widget.live
            ? (value) => setState(() => _value = value)
            : null,
      ),
    );
  }
}

class _BesideAButton extends StatelessWidget {
  const _BesideAButton();

  @override
  Widget build(BuildContext context) {
    return Col(
      width: SelectStories.width,
      children: [
        for (final WidgetSize size in SelectStories.sizes)
          ActionBar(
            children: [
              SizedBox(
                width: 240,
                child: Select<String>(
                  options: SelectStories.languages,
                  value: 'zh',
                  size: size,
                  onChanged: (_) {},
                ),
              ),
              Button(
                onPressed: () {},
                size: size,
                variant: ButtonVariant.filled,
                child: const Text('Apply'),
              ),
            ],
          ),
      ],
    );
  }
}

class _InAFormField extends StatefulWidget {
  const _InAFormField();

  @override
  State<_InAFormField> createState() => _InAFormFieldState();
}

class _InAFormFieldState extends State<_InAFormField> {
  String? _language = 'zh';

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: SelectStories.width,
      child: FormField(
        label: 'Target language',
        hint: 'Used when a document does not name one.',
        child: Select<String>(
          options: SelectStories.languages,
          value: _language,
          onChanged: (value) => setState(() => _language = value),
        ),
      ),
    );
  }
}

class _InASettingsRow extends StatefulWidget {
  const _InASettingsRow();

  @override
  State<_InASettingsRow> createState() => _InASettingsRowState();
}

class _InASettingsRowState extends State<_InASettingsRow> {
  String? _language = 'zh';

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: SelectStories.width,
      child: PreferenceSection(
        children: [
          PreferenceRow(
            title: 'Target language',
            subtitle: 'Used when a document does not name one.',
            trailing: Select<String>(
              options: SelectStories.languages,
              value: _language,
              size: WidgetSize.small,
              onChanged: (value) => setState(() => _language = value),
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
    return DocPage(
      'Select',
      description:
          'A choice out of a list too long to lay out. The trigger reads the '
          'TextField namespace rather than owning one — looking exactly like an '
          'input is the design, and a second set of numbers could only drift '
          'from it — and the list is the same floating sheet a Menu opens, so '
          'the two match. It takes its options as data rather than as children, '
          'because the rows are drawn here rather than by the platform.',
      children: [
        DocSection(
          'States',
          description:
              'The same four a TextField has, drawn the same way. An erroring '
              'select is re-tinted onto the danger ramp rather than given a '
              'second palette.',
          children: [
            Specimens(
              children: [
                const Specimen('default', children: [_Live(start: 'zh')]),
                const Specimen(
                  'disabled',
                  children: [_Live(start: 'zh', live: false)],
                ),
                const Specimen(
                  'error',
                  children: [_Live(start: 'zh', error: true)],
                ),
                const Specimen(
                  'placeholder',
                  children: [_Live(placeholder: 'Choose a language…')],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes',
          description:
              'The chevron keeps its distance from the trailing edge, and the '
              'value keeps its distance from the chevron, at every size. The '
              'list is at least as wide as the trigger it dropped from, so a '
              'long label never has to be read in a narrower column than it was '
              'chosen in.',
          children: [
            Col(
              width: SelectStories.width,
              children: [
                for (final WidgetSize size in SelectStories.sizes)
                  _Live(start: 'zh', size: size),
              ],
            ),
          ],
        ),
        const DocSection(
          'Groups',
          description:
              'A group becomes a heading and the rows under it, which is what '
              '`<optgroup>` used to do. The heading takes the caption voice on '
              'the row’s own inset, so it reads as a divider with a name rather '
              'than as an option nobody can pick.',
          children: [_Live(start: 'zh', groups: true)],
        ),
      ],
    );
  }
}
