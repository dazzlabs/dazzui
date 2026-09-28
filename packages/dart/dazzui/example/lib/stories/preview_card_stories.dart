import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/PreviewCard` — what is behind a link, without following it.
class PreviewCardStories {
  PreviewCardStories._();

  static const List<PopoverSide> sides = [
    PopoverSide.top,
    PopoverSide.bottom,
    PopoverSide.left,
    PopoverSide.right,
  ];

  static StoryPage get page => StoryPage('PreviewCard', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('On a link', (context) => const _OnALink()),
    Story('A repository', (context) => const _ARepository()),
    Story(
      'Sides',
      (context) => RowWrap(
        children: [
          for (final side in sides)
            PreviewCard(
              side: side,
              trigger: _Link(side.name),
              child: const _Person(),
            ),
        ],
      ),
    ),
    Story(
      'Instant',
      (context) => const PreviewCard(
        delay: Duration.zero,
        trigger: _Link('@ada'),
        child: _Person(),
      ),
    ),
  ]);
}

/// A link, drawn the way the `plain` recipe draws one: the accent one grade
/// darker than the fill, underlined.
class _Link extends StatelessWidget {
  const _Link(this.label);

  final String label;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Text(
      label,
      style: vars.labelQuiet.copyWith(
        color: vars.colorPrimary[vars.controlColorPlainContent.normalShade!]!,
        decoration: TextDecoration.underline,
      ),
    );
  }
}

class _Person extends StatelessWidget {
  const _Person();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Col(
      spacing: vars.spacing3,
      children: [
        Row(
          mainAxisSize: MainAxisSize.min,
          spacing: vars.spacing3,
          children: [
            const Avatar(name: 'Ada Lovelace', size: WidgetSize.large),
            Col(
              spacing: vars.spacing05,
              children: [
                Text(
                  'Ada Lovelace',
                  style: vars.labelStrong.copyWith(color: vars.colorContent),
                ),
                Text(
                  '@ada',
                  style: vars.labelQuiet.copyWith(
                    color: vars.colorContentSubtle,
                  ),
                ),
              ],
            ),
          ],
        ),
        Text(
          'Writes the notes that turn a machine into a program. 42 commits '
          'this month.',
          style: vars.bodySmall.copyWith(color: vars.colorContentSecondary),
        ),
      ],
    );
  }
}

class _OnALink extends StatelessWidget {
  const _OnALink();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Row(
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing1,
      children: [
        Text(
          'Reviewed by',
          style: vars.labelQuiet.copyWith(color: vars.colorContent),
        ),
        const PreviewCard(trigger: _Link('@ada'), child: _Person()),
        Text(
          'three days ago.',
          style: vars.labelQuiet.copyWith(color: vars.colorContent),
        ),
      ],
    );
  }
}

class _ARepository extends StatelessWidget {
  const _ARepository();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return PreviewCard(
      trigger: const _Link('dazzlabs/dazzui'),
      child: Col(
        spacing: vars.spacing2,
        children: [
          Row(
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing2,
            children: [
              const Avatar(
                name: 'DazzUI',
                shape: AvatarShape.square,
                size: WidgetSize.small,
              ),
              Text(
                'dazzlabs/dazzui',
                style: vars.labelStrong.copyWith(color: vars.colorContent),
              ),
              const Badge(
                size: WidgetSize.small,
                tint: BadgeTint.neutral,
                child: Text('public'),
              ),
            ],
          ),
          Text(
            'The design system, its tokens, and four platform '
            'implementations of it.',
            style: vars.bodySmall.copyWith(color: vars.colorContentSecondary),
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
    return const DocPage(
      'PreviewCard',
      description:
          'What is behind a link, shown without following it — a repository, a '
          'person, an issue. It is a Tooltip that holds a card rather than a '
          'label, and the difference is what may go in it: a tooltip must '
          'carry nothing a reader needs, because a pointer never reaches it; a '
          'preview card stays open while the pointer travels, so it can hold a '
          'link of its own. It is never the only way to reach what it shows.',
      children: [
        DocSection(
          'A link, previewed',
          description:
              'It opens on hover and on focus, after a rest — never on a '
              'click, which belongs to the link. The sheet is the '
              'Popover’s, with a ceiling on its width: a popover is as '
              'wide as what a caller put in it, a preview card is filled from '
              'whatever it previews.',
          children: [_OnALink()],
        ),
        DocSection(
          'Against a Tooltip',
          description:
              'If the panel has anything in it a reader might want to reach — '
              'a link, a button, a value to select — it is a preview card. If '
              'it is a phrase naming what an icon does, it is a tooltip, and '
              'it must stay one.',
          children: [_ARepository()],
        ),
      ],
    );
  }
}
