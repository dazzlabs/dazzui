import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/FormField` — the label over a control and the hint under it.
class FormFieldStories {
  FormFieldStories._();

  static StoryPage get page => StoryPage('FormField', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'With a hint',
      (context) => const _Field(hint: 'Shown to everyone in the workspace.'),
    ),
    Story('Without a hint', (context) => const _Field()),
    Story('Error state', (context) => const _Invalid()),
    Story('Over a multiline field', (context) => const _Multiline()),
    Story(
      'Over any control',
      (context) => Col(
        width: 320,
        children: [
          FormField(
            label: 'Notifications',
            child: Switch(value: true, onChanged: (_) {}),
          ),
          const FormField(label: 'Shortcut', child: KeyCap('⇧⌘B')),
        ],
      ),
    ),
    Story('A full form', (context) => const _FullForm()),
  ]);
}

class _Field extends StatefulWidget {
  const _Field({this.hint});

  final String? hint;

  @override
  State<_Field> createState() => _FieldState();
}

class _FieldState extends State<_Field> {
  final TextEditingController _name = TextEditingController(text: 'DazzUI');

  @override
  void dispose() {
    _name.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 320,
      child: FormField(
        label: 'Workspace name',
        hint: widget.hint,
        child: TextField(controller: _name, placeholder: 'Acme'),
      ),
    );
  }
}

class _Invalid extends StatefulWidget {
  const _Invalid();

  @override
  State<_Invalid> createState() => _InvalidState();
}

class _InvalidState extends State<_Invalid> {
  final TextEditingController _email = TextEditingController(text: 'you@');

  @override
  void dispose() {
    _email.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 320,
      child: FormField(
        label: 'Email',
        hint: 'We could not reach this address.',
        invalid: true,
        child: TextField(
          controller: _email,
          placeholder: 'you@example.com',
        ),
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
  final TextEditingController _notes = TextEditingController();

  @override
  void dispose() {
    _notes.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 320,
      child: FormField(
        label: 'Notes',
        hint: 'Markdown is supported.',
        child: TextField(
          controller: _notes,
          placeholder: 'Anything the team should know…',
          maxLines: 4,
        ),
      ),
    );
  }
}

class _FullForm extends StatelessWidget {
  const _FullForm();

  @override
  Widget build(BuildContext context) {
    return const Col(
      width: 380,
      children: [_Field(), _Invalid(), _Multiline()],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'FormField',
      description:
          'The chrome around a control: a label above it, a hint below, and '
          'the error state that turns both. It takes any control as its child '
          'rather than knowing about text fields, so a switch or a key cap '
          'gets the same label treatment as an input.',
      children: [
        DocSection(
          'Label and hint',
          description:
              'The hint is the field’s ordinary explanation. Marked invalid it '
              'becomes the message, in the tint’s ink — one line rather than a '
              'hint and an error stacked, because a field that explains itself '
              'twice is a field the reader stops reading.',
          children: [
            RowWrap(
              alignment: WrapCrossAlignment.start,
              children: [
                _Field(hint: 'Shown to everyone in the workspace.'),
                _Field(),
                _Invalid(),
              ],
            ),
          ],
        ),
        DocSection(
          'Over any control',
          description:
              'A multiline field, a switch, a key cap — the label is a label '
              'for whatever is underneath it.',
          children: [_Multiline()],
        ),
      ],
    );
  }
}
