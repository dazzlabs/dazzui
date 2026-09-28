import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Switch` — a decision that takes effect at once.
class SwitchStories {
  SwitchStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Switch', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('On', (context) => Switch(value: true, onChanged: (_) {})),
    Story('Off', (context) => Switch(value: false, onChanged: (_) {})),
    Story('Interactive', (context) => const _Live()),
    Story(
      'Disabled',
      (context) => const RowWrap(
        children: [
          Switch(value: false, onChanged: null),
          Switch(value: true, onChanged: null),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => RowWrap(
        children: [
          for (final size in sizes)
            Switch(value: true, size: size, onChanged: (_) {}),
        ],
      ),
    ),
    Story('Against a checkbox', (context) => const _Against()),
    Story('In settings', (context) => const _InSettings()),
  ]);
}

class _Live extends StatefulWidget {
  const _Live();

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  bool _on = true;

  @override
  Widget build(BuildContext context) {
    return Switch(
      value: _on,
      onChanged: (value) => setState(() => _on = value),
    );
  }
}

class _Against extends StatelessWidget {
  const _Against();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing3,
      children: [
        Row(
          mainAxisSize: MainAxisSize.min,
          spacing: vars.spacing3,
          children: [
            Switch(value: true, onChanged: (_) {}),
            Text(
              'Takes effect the moment it moves',
              style: vars.labelQuiet.copyWith(color: vars.colorContent),
            ),
          ],
        ),
        Checkbox(
          value: true,
          onChanged: (_) {},
          label: const Text('Takes effect when the form is submitted'),
        ),
      ],
    );
  }
}

class _InSettings extends StatefulWidget {
  const _InSettings();

  @override
  State<_InSettings> createState() => _InSettingsState();
}

class _InSettingsState extends State<_InSettings> {
  bool _launch = true;
  bool _updates = true;
  bool _sounds = false;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 460,
      child: PreferenceSection(
        label: 'General',
        children: [
          PreferenceRow(
            title: 'Launch at login',
            trailing: Switch(
              value: _launch,
              onChanged: (value) => setState(() => _launch = value),
            ),
          ),
          PreferenceRow(
            title: 'Check for updates',
            subtitle: 'Downloads in the background and asks before installing.',
            trailing: Switch(
              value: _updates,
              onChanged: (value) => setState(() => _updates = value),
            ),
          ),
          PreferenceRow(
            title: 'Play sounds',
            trailing: Switch(
              value: _sounds,
              onChanged: (value) => setState(() => _sounds = value),
            ),
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
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Switch',
      description:
          'A track and a thumb. It is the control for a setting that applies '
          'the moment it moves — which is what separates it from a checkbox, '
          'and why it belongs at the trailing edge of a preference row rather '
          'than in a form you submit.',
      children: [
        DocSection(
          'Sizes',
          description:
              'The track, the height and the thumb are three tokens moved '
              'together, so the thumb keeps its inset at every size.',
          children: [
            Specimens(
              children: [
                for (final size in SwitchStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      Switch(value: true, size: size, onChanged: (_) {}),
                      Switch(value: false, size: size, onChanged: (_) {}),
                      Switch(value: true, size: size, onChanged: null),
                    ],
                  ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Against a checkbox',
          description:
              'A switch says the change is already made; a checkbox says it '
              'will be made when the form is. Choosing by which one looks '
              'better is how a settings screen ends up with both.',
          children: [_Against()],
        ),
        const DocSection(
          'In settings',
          description: 'Where nearly every switch in an app actually lives.',
          children: [_InSettings()],
        ),
        DocSection(
          'Tokens',
          description: 'The three the switch owns, per size.',
          children: [
            TokenTable([
              TokenRow.length('switch.small.width', vars.switchSmallWidth),
              TokenRow.length('switch.small.height', vars.switchSmallHeight),
              TokenRow.length('switch.small.thumb', vars.switchSmallThumb),
              TokenRow.length('switch.medium.width', vars.switchMediumWidth),
              TokenRow.length('switch.medium.height', vars.switchMediumHeight),
              TokenRow.length('switch.medium.thumb', vars.switchMediumThumb),
              TokenRow.length('switch.large.width', vars.switchLargeWidth),
              TokenRow.length('switch.large.height', vars.switchLargeHeight),
              TokenRow.length('switch.large.thumb', vars.switchLargeThumb),
            ]),
          ],
        ),
      ],
    );
  }
}
