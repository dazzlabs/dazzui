import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/OtpField` — the one-time code a reader is copying.
class OtpFieldStories {
  OtpFieldStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('OtpField', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Bare', (context) => const _Live()),
    Story('Filled', (context) => const _Live(start: '204815')),
    Story('Grouped', (context) => const _Live(start: '204815', groupAfter: 3)),
    Story(
      'Sizes',
      (context) => const Col(
        children: [
          _Live(start: '204', size: WidgetSize.small),
          _Live(start: '204', size: WidgetSize.medium),
          _Live(start: '204', size: WidgetSize.large),
        ],
      ),
    ),
    Story(
      'Masked',
      (context) => const _Live(start: '2048', length: 4, mask: true),
    ),
    Story(
      'Disabled',
      (context) => const OtpField(length: 6, value: '204815'),
    ),
    Story('In a form', (context) => const _InAForm()),
  ]);
}

class _Live extends StatefulWidget {
  const _Live({
    this.start = '',
    this.length = 6,
    this.groupAfter,
    this.mask = false,
    this.size = WidgetSize.medium,
  });

  final String start;
  final int length;
  final int? groupAfter;
  final bool mask;
  final WidgetSize size;

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  late String _code = widget.start;

  @override
  Widget build(BuildContext context) {
    return OtpField(
      length: widget.length,
      value: _code,
      groupAfter: widget.groupAfter,
      mask: widget.mask,
      size: widget.size,
      onChanged: (value) => setState(() => _code = value),
    );
  }
}

class _InAForm extends StatefulWidget {
  const _InAForm();

  @override
  State<_InAForm> createState() => _InAFormState();
}

class _InAFormState extends State<_InAForm> {
  String _code = '';

  bool get _wrong => _code.length == 6 && _code != '204815';

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 320,
      spacing: 16,
      children: [
        const Callout(
          tint: CalloutTint.info,
          message: Text('We sent a six-digit code to ada@example.com.'),
        ),
        FormField(
          label: 'Verification code',
          hint: _wrong ? 'That code did not match' : 'It expires in 10 minutes',
          invalid: _wrong,
          child: OtpField(
            length: 6,
            groupAfter: 3,
            value: _code,
            state: _wrong ? TextFieldState.error : TextFieldState.normal,
            onChanged: (value) => setState(() => _code = value),
          ),
        ),
        Button(
          variant: ButtonVariant.filled,
          onPressed: _code.length < 6 ? null : () {},
          child: const Text('Verify'),
        ),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'OtpField',
      description:
          'The one-time code a reader is copying out of a message. It is drawn '
          'as a run of cells but it is one field underneath, and that is '
          'deliberate: a paste of the whole code fills every cell at once, the '
          'platform’s own SMS autofill has somewhere to put it, and the '
          'caret only ever has one place to be. A run of separate inputs gets '
          'all three wrong and has to reinvent the moves between them.',
      children: [
        DocSection(
          'Sizes',
          description:
              'Each cell is the text controls’ box at one character wide, '
              'so a code sits on the same rhythm as the fields around it — a '
              'code entered in a box of its own design would be the only '
              'control on the page that was.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'small',
                  children: [_Live(start: '204', size: WidgetSize.small)],
                ),
                Specimen('medium', children: [_Live(start: '204')]),
                Specimen(
                  'large',
                  children: [_Live(start: '204', size: WidgetSize.large)],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Grouping',
          description:
              '`groupAfter` breaks the run where a printed code breaks — three '
              'and three is how a six-digit code is read aloud. It is a step '
              'of air rather than a rule, because a rule would read as two '
              'fields.',
          children: [
            Specimens(
              children: [
                Specimen('ungrouped', children: [_Live(start: '204815')]),
                Specimen(
                  'groupAfter 3',
                  children: [_Live(start: '204815', groupAfter: 3)],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Masked and disabled',
          description:
              '`mask` is for a code that is a secret rather than a one-time '
              'token — a PIN. A filled cell lifts to the paper and keeps it, '
              'so the run says how far through the code the reader is without '
              'the caret having to be the only thing that does.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'masked',
                  children: [_Live(start: '20', length: 4, mask: true)],
                ),
                Specimen(
                  'disabled',
                  children: [OtpField(length: 6, value: '204815')],
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
