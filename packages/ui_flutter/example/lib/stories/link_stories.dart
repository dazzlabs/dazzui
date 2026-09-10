import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Link` — text that goes somewhere.
class LinkStories {
  LinkStories._();

  static const List<LinkTint> _tints = LinkTint.values;

  static StoryPage get page => StoryPage('Link', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Underlines',
      (context) => const Col(
        children: [
          Link(label: 'Open the changelog', underline: LinkUnderline.hover),
          Link(label: 'Open the changelog', underline: LinkUnderline.always),
          Link(label: 'Open the changelog', underline: LinkUnderline.none),
        ],
      ),
    ),
    Story(
      'External',
      (context) => const Link(label: 'Base UI', external: true),
    ),
    Story(
      'Tints',
      (context) => Col(
        children: [
          for (final LinkTint tint in _tints)
            Link(label: 'Continue', tint: tint, onPressed: _noop),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => const Link(label: 'Not available yet', enabled: false),
    ),
    Story('In prose', (context) => const _InProse()),
  ]);

  static void _noop() {}
}

class _InProse extends StatelessWidget {
  const _InProse();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 416,
      child: Text.rich(
        TextSpan(
          children: const <InlineSpan>[
            TextSpan(text: 'Behaviour comes from '),
            WidgetSpan(
              alignment: PlaceholderAlignment.baseline,
              baseline: TextBaseline.alphabetic,
              child: Link(label: 'Base UI'),
            ),
            TextSpan(
              text:
                  ', and the drawing is ours. Where a control has to reach '
                  'a form, read ',
            ),
            WidgetSpan(
              alignment: PlaceholderAlignment.baseline,
              baseline: TextBaseline.alphabetic,
              child: Link(label: 'the field conventions'),
            ),
            TextSpan(text: ' first — or open the '),
            WidgetSpan(
              alignment: PlaceholderAlignment.baseline,
              baseline: TextBaseline.alphabetic,
              child: Link(label: 'upstream docs', external: true),
            ),
            TextSpan(
              text: ' for the parts this package does not wrap.',
            ),
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
      'Link',
      description:
          'Text that goes somewhere. It is the one interactive thing in this '
          'package that is genuinely inline, which is why it exists beside '
          'Button(variant: plain) rather than inside it: a button keeps its '
          'box at every variant, and a box is what a link in a sentence must '
          'not have. It takes the shared state machine for its colour — the '
          'plain recipe, a grade darker on hover — and none of its geometry.',
      children: [
        DocSection(
          'Underline',
          description:
              'hover is the default, because a paragraph with a rule under '
              'every link reads as struck through. always is for a link that '
              'has to be findable without a pointer — a list of references, a '
              'footer. none is for one whose surroundings already say it is a '
              'link: a breadcrumb, a row that is entirely a target.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'hover',
                  children: [
                    Link(
                      label: 'Open the changelog',
                      underline: LinkUnderline.hover,
                    ),
                  ],
                ),
                Specimen(
                  'always',
                  children: [
                    Link(
                      label: 'Open the changelog',
                      underline: LinkUnderline.always,
                    ),
                  ],
                ),
                Specimen(
                  'none',
                  children: [
                    Link(
                      label: 'Open the changelog',
                      underline: LinkUnderline.none,
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'External',
          description:
              'Goes somewhere outside this window, and draws the arrow that '
              'says so before the reader commits. The arrow rides in the text '
              'run, so it wraps with the last word rather than stranding '
              'itself on a line of its own.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'same window',
                  children: [Link(label: 'Release notes')],
                ),
                Specimen(
                  'external',
                  children: [Link(label: 'Base UI', external: true)],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'In a paragraph',
          description:
              'Flutter has no inline widgets in a paragraph without saying so, '
              'so a link inside prose is a WidgetSpan on the alphabetic '
              'baseline. That is the one place this port differs from the '
              'React one in how it is written rather than in what it draws.',
          children: [_InProse()],
        ),
      ],
    );
  }
}
