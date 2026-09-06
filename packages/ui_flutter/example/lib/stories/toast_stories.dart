import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Toast` — what the app says in passing.
class ToastStories {
  ToastStories._();

  static StoryPage get page => StoryPage('Toast', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Default',
      (context) => const SizedBox(
        width: 380,
        child: Toast(message: 'Tokens rebuilt — four packages refreshed.'),
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        width: 380,
        children: [
          for (final tint in ToastTint.values)
            Toast(message: label(tint.name), tint: tint),
        ],
      ),
    ),
    Story(
      'Shapes',
      (context) => Col(
        width: 380,
        children: [
          const Toast(message: 'Message only.'),
          Toast(
            message: 'With a dismiss.',
            tint: ToastTint.info,
            onDismiss: () {},
          ),
          const _Detailed(),
        ],
      ),
    ),
    Story(
      'Truncation',
      (context) => SizedBox(
        width: 420,
        child: Toast(
          message:
              'A message long enough to reach the toast’s own maximum width '
              'and wrap inside it rather than stretching the viewport.',
          tint: ToastTint.warning,
          onDismiss: () {},
        ),
      ),
    ),
    Story('A stack', (context) => const _Stack()),
    Story('Posting one', (context) => const _Posting()),
  ]);
}

class _Detailed extends StatelessWidget {
  const _Detailed();

  @override
  Widget build(BuildContext context) {
    return Toast(
      message: 'The build failed — 4 errors in sidebar.dart.',
      icon: FluentIcons.error_circle_16_regular,
      tint: ToastTint.danger,
      onDismiss: () {},
      action: Button(
        variant: ButtonVariant.plain,
        tint: ButtonTint.danger,
        onPressed: () {},
        child: const Text('Show'),
      ),
    );
  }
}

class _Stack extends StatelessWidget {
  const _Stack();

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 420,
      spacing: Theme.of(context).vars.spacing2,
      children: [
        Toast(
          message: 'Tokens rebuilt.',
          tint: ToastTint.success,
          onDismiss: () {},
        ),
        Toast(
          message: 'Two packages still building…',
          tint: ToastTint.info,
          onDismiss: () {},
        ),
      ],
    );
  }
}

/// A toast as an app posts one: something happens, the toast appears over the
/// surface it happened on, and it goes when it is dismissed.
class _Posting extends StatefulWidget {
  const _Posting();

  @override
  State<_Posting> createState() => _PostingState();
}

class _PostingState extends State<_Posting> {
  bool _posted = false;

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 420,
      children: [
        Button(
          variant: ButtonVariant.filled,
          onPressed: () => setState(() => _posted = true),
          child: const Text('Rebuild tokens'),
        ),
        if (_posted)
          Toast(
            message: 'Tokens rebuilt — four packages refreshed.',
            tint: ToastTint.success,
            onDismiss: () => setState(() => _posted = false),
          ),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Toast',
      description:
          'A message that passes, where a callout stays. The tint carries the '
          'meaning and the shape carries how long it is around — which is the '
          'whole of the difference between the two widgets.',
      children: [
        DocSection(
          'Shapes',
          description:
              'The message alone; the message with a dismiss; or the full '
              'shape — a leading glyph, the message, and one action. More than '
              'one action belongs in a dialog: a toast the reader has to make '
              'a decision in is a toast that will disappear while they are '
              'deciding.',
          children: [
            Col(
              width: proseMeasure,
              children: [
                const Toast(message: 'Message only.'),
                Toast(
                  message: 'With a dismiss.',
                  tint: ToastTint.info,
                  onDismiss: () {},
                ),
                const _Detailed(),
              ],
            ),
          ],
        ),
        DocSection(
          'Tints',
          description:
              'The status ramps, read as washes — so a toast sits correctly '
              'over paper and over a near-black canvas without a per-theme '
              'value.',
          children: [
            Col(
              width: proseMeasure,
              children: [
                for (final tint in ToastTint.values)
                  Toast(
                    message: label(tint.name),
                    tint: tint,
                    onDismiss: () {},
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Tokens',
          description: 'The one the toast owns.',
          children: [
            TokenTable([
              TokenRow.length('toast.max-width', vars.toastMaxWidth),
            ]),
          ],
        ),
      ],
    );
  }
}
