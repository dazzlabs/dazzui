import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/OptionCard` — a card you can choose.
class OptionCardStories {
  OptionCardStories._();

  static StoryPage get page => StoryPage('OptionCard', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Selected',
      (context) => SizedBox(
        width: 240,
        child: OptionCard(
          title: 'Automatic',
          description: 'Follow the system appearance.',
          selected: true,
          onPressed: () {},
        ),
      ),
    ),
    Story(
      'Unselected',
      (context) => SizedBox(
        width: 240,
        child: OptionCard(
          title: 'Manual',
          description: 'Pick a theme and keep it.',
          selected: false,
          onPressed: () {},
        ),
      ),
    ),
    Story('A group', (context) => const _Appearance()),
    Story(
      'Without a description',
      (context) => RowWrap(
        children: [
          SizedBox(
            width: 140,
            child: OptionCard(
              title: 'Light',
              selected: true,
              onPressed: () {},
            ),
          ),
          SizedBox(
            width: 140,
            child: OptionCard(
              title: 'Dark',
              selected: false,
              onPressed: () {},
            ),
          ),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => const SizedBox(
        width: 220,
        child: OptionCard(
          title: 'Enterprise',
          description: 'Not on this plan.',
          selected: false,
          onPressed: null,
        ),
      ),
    ),
    Story(
      'In a grid',
      (context) => const SizedBox(
        width: 640,
        child: TileGrid(
          min: 200,
          children: [
            OptionCard(
              title: 'React',
              description: 'Base UI under the drawing.',
              selected: false,
              onPressed: null,
            ),
            OptionCard(
              title: 'Flutter',
              description: 'One theme value, no cascade.',
              selected: true,
              onPressed: null,
            ),
            OptionCard(
              title: 'SwiftUI',
              description: 'Per-subtree themes.',
              selected: false,
              onPressed: null,
            ),
            OptionCard(
              title: 'GPUI',
              description: 'A seed and a const fn.',
              selected: false,
              onPressed: null,
            ),
          ],
        ),
      ),
    ),
  ]);
}

class _Appearance extends StatefulWidget {
  const _Appearance();

  @override
  State<_Appearance> createState() => _AppearanceState();
}

class _AppearanceState extends State<_Appearance> {
  String _mode = 'auto';

  @override
  Widget build(BuildContext context) {
    return RowWrap(
      alignment: WrapCrossAlignment.start,
      children: [
        SizedBox(
          width: 220,
          child: OptionCard(
            title: 'Automatic',
            description: 'Follow the system appearance.',
            selected: _mode == 'auto',
            onPressed: () => setState(() => _mode = 'auto'),
          ),
        ),
        SizedBox(
          width: 220,
          child: OptionCard(
            title: 'Manual',
            description: 'Pick a theme and keep it.',
            selected: _mode == 'manual',
            onPressed: () => setState(() => _mode = 'manual'),
          ),
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
      'OptionCard',
      description:
          'The choosable card: a radio’s job in a card’s shape, for a decision '
          'that needs a sentence of explanation rather than a word. It is '
          'interactive, so unlike `Card` it is in the shared state machine and '
          'answers the pointer.',
      children: [
        DocSection(
          'Selection',
          description:
              'Selection shifts weight as well as colour — the title goes to '
              'the strong label face, the edge takes the tint, and the fill '
              'takes the tint’s wash. That is what makes a chosen card '
              'readable at a glance in a grid of four.',
          children: [_Appearance()],
        ),
        DocSection(
          'Shapes',
          description:
              'The description is optional: without one the card is a wide '
              'radio row, which is the right shape for two short answers. '
              'Disabled flattens to the groove, like every other control.',
          children: [
            RowWrap(
              alignment: WrapCrossAlignment.start,
              children: [
                SizedBox(
                  width: 140,
                  child: OptionCard(
                    title: 'Light',
                    selected: true,
                    onPressed: null,
                  ),
                ),
                SizedBox(
                  width: 220,
                  child: OptionCard(
                    title: 'Enterprise',
                    description: 'Not on this plan.',
                    selected: false,
                    onPressed: null,
                  ),
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
