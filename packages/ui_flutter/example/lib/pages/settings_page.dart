import 'package:dazzui/dazzui.dart';

import '../gallery.dart';

/// A preferences screen, which is the composition most of these components
/// were drawn for: rows of one decision each, grouped under a label, with the
/// control pinned to the trailing edge.
class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  bool _launch = true;
  bool _updates = true;
  bool _sounds = false;
  String _appearance = 'auto';

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing6,
      children: [
        const PageTitle(
          'Settings',
          'One decision per row, and the control sits where the eye lands '
              'last.',
        ),
        ConstrainedBox(
          constraints: BoxConstraints(maxWidth: vars.dialogWidth * 1.5),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing6,
            children: [
              // A group is the level above a section: sections stay one size
              // everywhere, and the title outranks them rather than being a
              // section shrunk to fit underneath one.
              PreferenceGroup(
                title: 'Application',
                description: 'Settings that apply to every window.',
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
                        subtitle: 'Downloads in the background.',
                        trailing: Switch(
                          value: _updates,
                          onChanged: (value) =>
                              setState(() => _updates = value),
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
                  PreferenceSection(
                    label: 'Appearance',
                    footer:
                        'Automatic follows the system between light and '
                        'dark.',
                    children: [
                      PreferenceRow(
                        title: 'Theme',
                        trailing: SegmentedControl<String>(
                          value: _appearance,
                          onChanged: (value) =>
                              setState(() => _appearance = value),
                          items: const [
                            SegmentedItem(value: 'auto', label: 'Auto'),
                            SegmentedItem(value: 'light', label: 'Light'),
                            SegmentedItem(value: 'dark', label: 'Dark'),
                          ],
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              // A section on its own, with no group over it.
              PreferenceSection(
                label: 'Shortcuts',
                children: const [
                  PreferenceRow(
                    title: 'Toggle the palette',
                    trailing: KeyCap('⌘K', variant: KeyCapVariant.key),
                  ),
                  PreferenceRow(
                    title: 'Rebuild tokens',
                    trailing: KeyCap('⇧⌘B', variant: KeyCapVariant.key),
                  ),
                ],
              ),
              PreferenceSection(
                label: 'Navigation',
                children: [
                  PreferenceRow(
                    title: 'Open the changelog',
                    subtitle: 'Opens in your browser.',
                    onPressed: () {},
                  ),
                ],
              ),
            ],
          ),
        ),
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}
