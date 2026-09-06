import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/TextField` — the box a value is typed into.
class TextFieldStories {
  TextFieldStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('TextField', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Default', (context) => const _Field(text: 'DazzUI')),
    Story('Placeholder', (context) => const _Field()),
    Story(
      'Disabled',
      (context) => const _Field(text: 'DazzUI', enabled: false),
    ),
    Story(
      'Read-only',
      (context) => const _Field(text: '785f949', readOnly: true),
    ),
    Story(
      'Sizes',
      (context) => Col(
        width: 320,
        children: [
          for (final size in sizes)
            _Field(text: label(size.namedSize.name), size: size),
        ],
      ),
    ),
    Story(
      'Beside a button',
      (context) => RowWrap(
        children: [
          const _Field(),
          Button(
            variant: ButtonVariant.filled,
            onPressed: () {},
            child: const Text('Add'),
          ),
        ],
      ),
    ),
    Story('Multiline', (context) => const _Multiline()),
    Story(
      'In a form field',
      (context) => const Col(
        width: 380,
        children: [
          FormField(
            label: 'Workspace name',
            hint: 'Shown to everyone in the workspace.',
            child: _Field(text: 'DazzUI'),
          ),
        ],
      ),
    ),
  ]);
}

class _Field extends StatefulWidget {
  const _Field({
    this.text = '',
    this.size = WidgetSize.medium,
    this.enabled = true,
    this.readOnly = false,
  });

  final String text;
  final WidgetSize size;
  final bool enabled;
  final bool readOnly;

  @override
  State<_Field> createState() => _FieldState();
}

class _FieldState extends State<_Field> {
  late final TextEditingController _controller = TextEditingController(
    text: widget.text,
  );

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 240,
      child: TextField(
        controller: _controller,
        placeholder: 'Acme',
        size: widget.size,
        enabled: widget.enabled,
        readOnly: widget.readOnly,
      ),
    );
  }
}

class _Multiline extends StatefulWidget {
  const _Multiline();

  @override
  State<_Multiline> createState() => _MultilineState();
}

class _MultilineState extends State<_Multiline> {
  final TextEditingController _controller = TextEditingController();

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 320,
      child: TextField(
        controller: _controller,
        placeholder: 'Anything the team should know…',
        maxLines: 4,
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'TextField',
      description:
          'The box a value is typed into, on the control heights everything '
          'else uses. The focus ring is the system accent in every tint — it '
          'never follows the control’s own colour, because a ring that changed '
          'colour per field would stop meaning focus.',
      children: [
        DocSection(
          'States',
          description:
              'Default, empty with a placeholder, read-only and disabled. An '
              'invalid field is the `FormField` around it turning, not a state '
              'the box draws twice.',
          children: [
            Col(
              width: 420,
              children: [
                _Field(text: 'DazzUI'),
                _Field(),
                _Field(text: '785f949', readOnly: true),
                _Field(text: 'DazzUI', enabled: false),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes',
          description:
              'The three field sizes are the control profile’s, so a field and '
              'a button on the same row are the same height.',
          children: [
            Col(
              width: 420,
              children: [
                _Field(text: 'Small', size: WidgetSize.small),
                _Field(text: 'Medium'),
                _Field(text: 'Large', size: WidgetSize.large),
              ],
            ),
          ],
        ),
        DocSection(
          'Multiline',
          description:
              '`maxLines` grows the same box into a text area — the same '
              'chrome, the same states, more room.',
          children: [_Multiline()],
        ),
      ],
    );
  }
}
