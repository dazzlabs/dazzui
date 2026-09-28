import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Radio` — one of several.
class RadioStories {
  RadioStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Radio', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Checked',
      (context) => Radio<String>(
        value: 'team',
        groupValue: 'team',
        onChanged: (_) {},
        label: const Text('Team'),
      ),
    ),
    Story(
      'Unchecked',
      (context) => Radio<String>(
        value: 'team',
        groupValue: 'solo',
        onChanged: (_) {},
        label: const Text('Team'),
      ),
    ),
    Story('A group', (context) => const _Plans()),
    Story(
      'Disabled',
      (context) => const Col(
        children: [
          Radio<String>(
            value: 'team',
            groupValue: 'solo',
            onChanged: null,
            label: Text('Unavailable'),
          ),
          Radio<String>(
            value: 'team',
            groupValue: 'team',
            onChanged: null,
            label: Text('Fixed for this plan'),
          ),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => Col(
        children: [
          for (final size in sizes)
            Radio<String>(
              value: 'on',
              groupValue: 'on',
              onChanged: (_) {},
              size: size,
              label: Text(label(size.namedSize.name)),
            ),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        children: [
          for (final tint in RadioTint.values)
            Radio<String>(
              value: 'on',
              groupValue: 'on',
              onChanged: (_) {},
              tint: tint,
              label: Text(label(tint.name)),
            ),
        ],
      ),
    ),
    Story(
      'Against a checkbox',
      (context) => Col(
        children: [
          Radio<String>(
            value: 'on',
            groupValue: 'on',
            onChanged: (_) {},
            label: const Text('A radio picks one of several'),
          ),
          Checkbox(
            value: true,
            onChanged: (_) {},
            label: const Text('A checkbox is one decision'),
          ),
        ],
      ),
    ),
    Story(
      'In a form',
      (context) => const Col(
        width: 320,
        children: [FormField(label: 'Plan', child: _Plans())],
      ),
    ),
  ]);
}

class _Plans extends StatefulWidget {
  const _Plans();

  @override
  State<_Plans> createState() => _PlansState();
}

class _PlansState extends State<_Plans> {
  String _plan = 'team';

  @override
  Widget build(BuildContext context) {
    return RadioGroup<String>(
      value: _plan,
      onChanged: (value) => setState(() => _plan = value ?? _plan),
      options: const [
        RadioItem(value: 'solo', label: Text('Solo')),
        RadioItem(value: 'team', label: Text('Team')),
        RadioItem(
          value: 'enterprise',
          label: Text('Enterprise'),
          enabled: false,
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
      'Radio',
      description:
          'One of several. The ring draws its own mark rather than stacking a '
          'dot on a circle, so the two never disagree about where the centre '
          'is. A single radio is rarely right — `RadioGroup` is the usual '
          'form, and it owns the value.',
      children: [
        const DocSection(
          'A group',
          description:
              'The group carries the options, the selection and the tint, so a '
              'row cannot disagree with its neighbours. A disabled option '
              'flattens to the groove, whether or not it is the one selected.',
          children: [_Plans()],
        ),
        DocSection(
          'Sizes and tints',
          description:
              'The box and the dot are both the radio’s own tokens, sized '
              'together so the mark stays centred at every step.',
          children: [
            Specimens(
              children: [
                for (final size in RadioStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      for (final tint in RadioTint.values)
                        Radio<String>(
                          value: 'on',
                          groupValue: 'on',
                          onChanged: (_) {},
                          size: size,
                          tint: tint,
                          label: Text(label(tint.name)),
                        ),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Against a checkbox',
          description:
              'A radio is one of several; a checkbox is one decision on its '
              'own. The shapes say which: a circle belongs to a set, a square '
              'does not.',
          children: [
            Col(
              children: [
                Radio<String>(
                  value: 'on',
                  groupValue: 'on',
                  onChanged: (_) {},
                  label: const Text('A radio picks one of several'),
                ),
                Checkbox(
                  value: true,
                  onChanged: (_) {},
                  label: const Text('A checkbox is one decision'),
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Tokens',
          description: 'The ring and the mark inside it.',
          children: [
            TokenTable([
              TokenRow.length('radio.thickness', vars.radioThickness),
              TokenRow.length('radio.box.small', vars.radioSmallBox),
              TokenRow.length('radio.dot.small', vars.radioSmallDot),
              TokenRow.length('radio.box.medium', vars.radioMediumBox),
              TokenRow.length('radio.dot.medium', vars.radioMediumDot),
              TokenRow.length('radio.box.large', vars.radioLargeBox),
              TokenRow.length('radio.dot.large', vars.radioLargeDot),
            ]),
          ],
        ),
      ],
    );
  }
}
