import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/story.dart';

/// `Components/Code` — a run of code inside a sentence.
class CodeStories {
  CodeStories._();

  static StoryPage get page => StoryPage('Code', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Inline', (context) => const Code('pnpm parity')),
    Story('In prose', (context) => const _InProse()),
  ]);
}

class _InProse extends StatelessWidget {
  const _InProse();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 416,
      child: Text.rich(
        const TextSpan(
          children: <InlineSpan>[
            TextSpan(text: 'Pass '),
            WidgetSpan(
              alignment: PlaceholderAlignment.middle,
              child: Code('--features runtime-shaders'),
            ),
            TextSpan(
              text: ' on a Mac without the Metal toolchain, then run ',
            ),
            WidgetSpan(
              alignment: PlaceholderAlignment.middle,
              child: Code('cargo test'),
            ),
            TextSpan(text: ' from '),
            WidgetSpan(
              alignment: PlaceholderAlignment.middle,
              child: Code('packages/ui_gpui'),
            ),
            TextSpan(text: '.'),
          ],
        ),
        style: vars.bodySmall.copyWith(color: vars.colorContent),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Code',
      description:
          'A run of code inside a sentence — a flag, a path, an identifier. It '
          'is a wash and a face, not a box: the chip corner and the half-step '
          'inset keep it separable from the prose around it without breaking '
          'the line’s rhythm, which is what a border at this size always does.',
      children: [
        DocSection(
          'In a sentence',
          description:
              'The face is the code face read straight off base.font.code — '
              'the one place in this package a base primitive is named, '
              'because the code face is not a role. The size steps down one '
              'from the prose it sits in: a monospace face at the same nominal '
              'size always reads larger, and the run would ride above the line.',
          children: [_InProse()],
        ),
        DocSection(
          'What it is not',
          description:
              'A run of code is not a KeyCap and not a Badge. A KeyCap is a '
              'key you press and is drawn as one; a Badge is an annotation '
              'about something else. This is the thing itself, quoted.',
          children: [
            Specimens(
              children: [
                Specimen('an identifier', children: [Code('ThemeVariables')]),
                Specimen(
                  'a path',
                  children: [Code('packages/ds/tokens/global/_base.yaml')],
                ),
                Specimen('a flag', children: [Code('--no-verify')]),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
