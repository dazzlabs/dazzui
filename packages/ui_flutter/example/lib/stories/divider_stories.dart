import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Divider` — one device pixel, in two orientations.
class DividerStories {
  DividerStories._();

  static StoryPage get page => StoryPage('Divider', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Horizontal',
      (context) => const SizedBox(width: 320, child: Divider()),
    ),
    Story(
      'Muted',
      (context) => SizedBox(
        width: 320,
        child: Divider(color: Theme.of(context).vars.colorBorderStrong),
      ),
    ),
    Story(
      'Vertical',
      (context) => const RowWrap(
        children: [
          Text('Left'),
          ToolbarSeparator(size: WidgetSize.small),
          Text('Right'),
        ],
      ),
    ),
    Story(
      'In a toolbar',
      (context) => SizedBox(
        width: 260,
        child: ActionBar(
          children: [
            IconButton(
              icon: Icon(FluentIcons.chevron_left_16_regular),
              onPressed: () {},
            ),
            IconButton(
              icon: Icon(FluentIcons.chevron_right_16_regular),
              onPressed: () {},
            ),
            const ToolbarSeparator(),
            IconButton(
              icon: Icon(FluentIcons.arrow_clockwise_16_regular),
              onPressed: () {},
            ),
          ],
        ),
      ),
    ),
    Story(
      'In a card',
      (context) => const SizedBox(
        width: 260,
        child: Card(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [Text('Tokens'), Divider(), Text('Components')],
          ),
        ),
      ),
    ),
    Story('Between rows', (context) => const _Rows()),
  ]);
}

class _Rows extends StatelessWidget {
  const _Rows();

  static const List<String> entries = ['aurora', 'beacon', 'cinder'];

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      width: 260,
      decoration: BoxDecoration(
        color: vars.colorSurface,
        borderRadius: BorderRadius.circular(vars.radiusMedium),
        border: Border.all(color: vars.colorBorder, width: vars.spacingPx),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          for (final (index, name) in entries.indexed) ...[
            if (index > 0) const Divider(height: 1),
            Padding(
              padding: EdgeInsets.symmetric(
                vertical: vars.spacing2,
                horizontal: vars.spacing3,
              ),
              child: SizedBox(
                width: double.infinity,
                child: Text(
                  name,
                  style: vars.labelQuiet.copyWith(color: vars.colorContent),
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Divider',
      description:
          'A rule. It is one *device* pixel rather than one logical one: the '
          'hairline halves on a Retina display, the way `base.css` does with '
          'its `min-resolution` query, so a separator stays as thin as the '
          'screen can draw it. A drawn control outline is a different thing '
          'and reads `stroke.control` instead.',
      children: [
        DocSection(
          'Tone',
          description:
              'The ordinary border is the default; the stronger one is for the '
              'rule that has to survive on a busier surface. Both are semantic '
              'roles rather than steps off a ramp, so a theme moves them '
              'together.',
          children: [
            Col(
              width: 420,
              children: [
                const Divider(),
                Divider(color: vars.colorBorderStrong),
              ],
            ),
          ],
        ),
        DocSection(
          'Orientation',
          description:
              'A horizontal rule fills its parent. A vertical one stretches to '
              'the row it sits in under CSS; here a row that sizes itself to '
              'its children has to give the rule the height of the controls '
              'beside it.',
          children: [
            SizedBox(
              width: 220,
              child: ActionBar(
                children: [
                  IconButton(
                    icon: Icon(FluentIcons.chevron_left_16_regular),
                    onPressed: () {},
                  ),
                  IconButton(
                    icon: Icon(FluentIcons.chevron_right_16_regular),
                    onPressed: () {},
                  ),
                  const ToolbarSeparator(),
                  IconButton(
                    icon: Icon(FluentIcons.arrow_clockwise_16_regular),
                    onPressed: () {},
                  ),
                ],
              ),
            ),
          ],
        ),
        const DocSection(
          'Between rows',
          description:
              'A list separates its rows with a rule between them rather than '
              'under each, so the run does not end in a line hanging off the '
              'last entry.',
          children: [_Rows()],
        ),
      ],
    );
  }
}
