import 'package:dazzui/dazzui.dart';

import '../../storybook/gallery.dart';
import '../../storybook/story.dart';

/// `Tokens/Typography` — the faces, and what each is for.
class TypographyStories {
  TypographyStories._();

  static const String sample = 'Sharp, considered interfaces';

  static StoryPage get page => StoryPage('Typography', [
    Story('Scale', (context) => const _Scale(), layout: StoryLayout.page),
  ]);
}

class _Scale extends StatelessWidget {
  const _Scale();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Typography',
      description:
          'Four families of face — headline, title, body, label — plus the '
          'caption. Leading is stated absolutely rather than as a multiple, '
          'because that is how the design states it: a label face sets its '
          'line height to its own size so the text sits on the centre of a '
          '26px box, and a multiple would round that away.',
      children: [
        DocSection(
          'Headline',
          description: 'The page’s own name, used once.',
          children: [
            _Faces([
              ('headline.small', vars.headlineSmall),
              ('headline.medium', vars.headlineMedium),
              ('headline.large', vars.headlineLarge),
            ]),
          ],
        ),
        DocSection(
          'Title',
          description: 'A section’s heading.',
          children: [
            _Faces([
              ('title.small', vars.titleSmall),
              ('title.medium', vars.titleMedium),
              ('title.large', vars.titleLarge),
            ]),
          ],
        ),
        DocSection(
          'Body',
          description: 'Prose, and anything a reader reads.',
          children: [
            _Faces([
              ('body.small', vars.bodySmall),
              ('body.medium', vars.bodyMedium),
              ('body.large', vars.bodyLarge),
            ]),
          ],
        ),
        DocSection(
          'Label',
          description:
              'The display cut a control is set in. `quiet` and `strong` are '
              'the two weights selection moves between: a row rests at `quiet` '
              'and the selected entry takes the label weight, while a pill '
              'tab’s active label and an option card’s title go to `strong`.',
          children: [
            _Faces([
              ('label.quiet', vars.labelQuiet),
              ('label.small', vars.labelSmall),
              ('label.medium', vars.labelMedium),
              ('label.large', vars.labelLarge),
              ('label.strong', vars.labelStrong),
            ]),
          ],
        ),
        DocSection(
          'Caption',
          description: 'The line under something else.',
          children: [
            _Faces([
              ('caption.small', vars.captionSmall),
              ('caption.medium', vars.captionMedium),
              ('caption.large', vars.captionLarge),
            ]),
          ],
        ),
      ],
    );
  }
}

class _Faces extends StatelessWidget {
  const _Faces(this.entries);

  final List<(String, TextStyle)> entries;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing4,
      children: [
        for (final (name, style) in entries)
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing1,
            children: [
              Text(
                TypographyStories.sample,
                style: style.copyWith(color: vars.colorContent),
              ),
              Text(
                '$name · ${px(style.fontSize ?? 0)}',
                style: mono(
                  vars.captionSmall.copyWith(color: vars.colorContentMuted),
                ),
              ),
            ],
          ),
      ],
    );
  }
}
