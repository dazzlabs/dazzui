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
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing4,
            children: [
              ListSection.insetGrouped(
                header: const Text('General'),
                children: [
                  ListTile(
                    title: const Text('Launch at login'),
                    trailing: Switch(
                      value: _launch,
                      onChanged: (value) => setState(() => _launch = value),
                    ),
                  ),
                  ListTile(
                    title: const Text('Check for updates'),
                    subtitle: const Text(
                      'Downloads in the background and asks before '
                      'installing.',
                    ),
                    trailing: Switch(
                      value: _updates,
                      onChanged: (value) => setState(() => _updates = value),
                    ),
                  ),
                  ListTile(
                    title: const Text('Play sounds'),
                    trailing: Switch(
                      value: _sounds,
                      onChanged: (value) => setState(() => _sounds = value),
                    ),
                  ),
                ],
              ),
              ListSection.insetGrouped(
                header: const Text('Shortcuts'),
                footer: const Text(
                  'Shortcuts apply while the window has focus.',
                ),
                children: const [
                  ListTile(
                    title: Text('Toggle the palette'),
                    trailing: Kbd('⌘K'),
                  ),
                  ListTile(
                    title: Text('Rebuild tokens'),
                    trailing: Kbd('⇧⌘B'),
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
