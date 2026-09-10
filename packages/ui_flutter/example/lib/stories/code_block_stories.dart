import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/CodeBlock` — a block of code, read rather than edited.
class CodeBlockStories {
  CodeBlockStories._();

  static const String sample =
      "export function Badge({ size = 'medium', tint = 'primary' }: BadgeProps) {\n"
      '  return <span data-size={size} data-tint={tint} className="dz-badge" />\n'
      '}';

  static const String shell = 'pnpm install\npnpm tokens\npnpm parity';

  static const String wide =
      "const themes = ['studio-dark', 'bright-light', 'bright-dark', "
      "'frost-light', 'frost-dark', 'graphite-light']";

  static StoryPage get page => StoryPage('CodeBlock', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Plain',
      (context) => const Col(width: 480, children: [CodeBlock(code: shell)]),
    ),
    Story(
      'With head',
      (context) => const Col(
        width: 480,
        children: [
          CodeBlock(
            code: sample,
            language: 'tsx',
            title: 'src/components/badge.tsx',
          ),
        ],
      ),
    ),
    Story(
      'Line numbers',
      (context) => const Col(
        width: 480,
        children: [CodeBlock(code: sample, lineNumbers: true)],
      ),
    ),
    Story(
      'Copyable',
      (context) => const Col(
        width: 480,
        children: [CodeBlock(code: shell, copyable: true, language: 'bash')],
      ),
    ),
    Story(
      'Wrapped',
      (context) => const Col(
        width: 384,
        children: [CodeBlock(code: wide, language: 'ts', wrap: true)],
      ),
    ),
    Story('With actions', (context) => const _WithActions()),
  ]);
}

class _WithActions extends StatelessWidget {
  const _WithActions();

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 480,
      children: <Widget>[
        CodeBlock(
          code: CodeBlockStories.shell,
          copyable: true,
          title: 'install.sh',
          actions: <Widget>[
            Button(
              size: WidgetSize.tiny,
              variant: ButtonVariant.plain,
              onPressed: () {},
              child: const Text('Run'),
            ),
          ],
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
      'CodeBlock',
      description:
          'A block of code, read rather than edited. It takes the card’s '
          'materials rather than the popup’s — this sits in the page, not over '
          'it — and its head is drawn only where there is something to put in '
          'it. A bar holding one label is furniture that says less than the '
          'space it takes.',
      children: [
        DocSection(
          'The gutter is a column',
          description:
              'Line numbers live in their own column rather than in the same '
              'run as the code, so a selection that crosses ten lines takes '
              'ten lines of code and not ten line numbers — the single thing a '
              'reader does with one of these. It sits outside the scroller, so '
              'it stays put while the code slides under it.',
          children: [
            Col(
              width: 480,
              children: [
                CodeBlock(
                  code: CodeBlockStories.sample,
                  lineNumbers: true,
                  title: 'src/components/badge.tsx',
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'No highlighter',
          description:
              'language names the grammar at the trailing edge of the head and '
              'does nothing else. This package ships no highlighter, because a '
              'grammar per language is a dependency a design system has no '
              'opinion about.',
          children: [
            Col(
              width: 480,
              children: [
                CodeBlock(
                  code: CodeBlockStories.shell,
                  copyable: true,
                  language: 'bash',
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Long lines',
          description:
              'Scrolling is the default: a wrapped line of code has lost the '
              'one thing its indentation was carrying. wrap is there for a '
              'block in a narrow column, where a sideways scroller would be '
              'missed entirely.',
          children: [
            Col(
              width: 480,
              children: [
                CodeBlock(code: CodeBlockStories.wide, language: 'ts'),
                CodeBlock(
                  code: CodeBlockStories.wide,
                  language: 'ts',
                  wrap: true,
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
