import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/story.dart';

/// `Components/Preferences` — the settings column, and the group, the
/// section and the row that run down it.
class PreferencesStories {
  PreferencesStories._();

  static StoryPage get page => StoryPage('Preferences', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('A section', (context) => const _General()),
    Story('Row shapes', (context) => const _Shapes()),
    Story('Controls', (context) => const _Controls()),
    Story(
      'Section headings',
      (context) => const Preferences(
        children: [
          PreferenceSection(
            label: 'Appearance',
            footer: 'Automatic follows the system between light and dark.',
            children: [
              PreferenceRow(title: 'Theme', trailing: KeyCap('⌘,')),
            ],
          ),
        ],
      ),
    ),
    Story(
      'Unlabelled',
      (context) => Preferences(
        children: [
          PreferenceSection(
            children: [
              PreferenceRow(
                title: 'Launch at login',
                trailing: Switch(value: true, onChanged: (_) {}),
              ),
              PreferenceRow(
                title: 'Play sounds',
                trailing: Switch(value: false, onChanged: (_) {}),
              ),
            ],
          ),
        ],
      ),
    ),
    Story('A group', (context) => const _Grouped()),
    Story('A settings page', (context) => const _SettingsPage()),
  ]);
}

class _General extends StatefulWidget {
  const _General();

  @override
  State<_General> createState() => _GeneralState();
}

class _GeneralState extends State<_General> {
  bool _launch = true;
  bool _updates = true;

  @override
  Widget build(BuildContext context) {
    return Preferences(
      children: [
        PreferenceSection(
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
              subtitle:
                  'Downloads in the background and asks before installing.',
              trailing: Switch(
                value: _updates,
                onChanged: (value) => setState(() => _updates = value),
              ),
            ),
          ],
        ),
      ],
    );
  }
}

class _Shapes extends StatelessWidget {
  const _Shapes();

  @override
  Widget build(BuildContext context) {
    return Preferences(
      children: [
        PreferenceSection(
          label: 'Rows',
          children: [
            const PreferenceRow(title: 'Title only'),
            const PreferenceRow(
              title: 'With a subtitle',
              subtitle: 'The line that explains it.',
            ),
            const PreferenceRow(
              title: 'With an icon',
              icon: Icon(FluentIcons.folder_16_regular),
            ),
            PreferenceRow(
              title: 'With a control',
              trailing: Switch(value: true, onChanged: (_) {}),
            ),
            PreferenceRow(
              title: 'That opens something',
              subtitle: 'A row with an action is a row you can click.',
              onPressed: () {},
            ),
          ],
        ),
      ],
    );
  }
}

class _Controls extends StatefulWidget {
  const _Controls();

  @override
  State<_Controls> createState() => _ControlsState();
}

class _ControlsState extends State<_Controls> {
  bool _sounds = false;

  @override
  Widget build(BuildContext context) {
    return Preferences(
      children: [
        PreferenceSection(
          label: 'Controls',
          children: [
            PreferenceRow(
              title: 'Play sounds',
              trailing: Switch(
                value: _sounds,
                onChanged: (value) => setState(() => _sounds = value),
              ),
            ),
            const PreferenceRow(
              title: 'Rebuild tokens',
              trailing: KeyCap('⇧⌘B'),
            ),
            PreferenceRow(
              title: 'Cache',
              trailing: Button(onPressed: () {}, child: const Text('Clear')),
            ),
          ],
        ),
      ],
    );
  }
}

class _Grouped extends StatelessWidget {
  const _Grouped();

  @override
  Widget build(BuildContext context) {
    return const Preferences(
      children: [
        PreferenceGroup(
          title: 'Application',
          description: 'Settings that apply to every window.',
          children: [
            _General(),
            PreferenceSection(
              label: 'Shortcuts',
              children: [
                PreferenceRow(
                  title: 'Toggle the palette',
                  trailing: KeyCap('⌘K'),
                ),
                PreferenceRow(title: 'Rebuild tokens', trailing: KeyCap('⇧⌘B')),
              ],
            ),
          ],
        ),
      ],
    );
  }
}

class _SettingsPage extends StatelessWidget {
  const _SettingsPage();

  @override
  Widget build(BuildContext context) {
    return Preferences(
      children: [
        const _Grouped(),
        PreferenceSection(
          label: 'Danger zone',
          children: [
            PreferenceRow(
              title: 'Reset every preference',
              subtitle:
                  'Windows, shortcuts and appearance go back to their '
                  'defaults.',
              trailing: Button(
                variant: ButtonVariant.tinted,
                tint: ButtonTint.danger,
                onPressed: () {},
                child: const Text('Reset'),
              ),
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
      'Preferences',
      description:
          'A settings screen is one decision per row, grouped under a label, '
          'with the control pinned to the trailing edge — where the eye lands '
          'last. A column is the root: it caps the width and spaces the '
          'groups. Under it, a group is the level above a section, a section '
          'is a run of rows under one heading, and a row is the decision.',
      children: [
        DocSection(
          'Rows',
          description:
              'A row is a title, optionally a subtitle under it, optionally a '
              'leading glyph, and optionally a trailing control. A row with an '
              '`onPressed` is one you can click — that is what makes it a way '
              'in rather than a fact.',
          children: [_Shapes()],
        ),
        DocSection(
          'What sits at the trailing edge',
          description:
              'A switch for a decision that takes effect at once, a key cap '
              'for something bound elsewhere, a button for an action. Only one '
              'of them per row.',
          children: [_Controls()],
        ),
        DocSection(
          'Sections and groups',
          description:
              'A section carries a label above its rows and an optional footer '
              'under them. A group is the level above: sections stay one size '
              'everywhere, and the group’s title outranks them rather than '
              'being a section shrunk to fit underneath one.',
          children: [_Grouped()],
        ),
      ],
    );
  }
}
