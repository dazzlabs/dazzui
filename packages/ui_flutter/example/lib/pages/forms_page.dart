import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../gallery.dart';

/// The form vocabulary, shown as a form rather than as a shelf of specimens —
/// the fields, the choices under them, and the disabled state are only really
/// legible next to each other.
class FormsPage extends StatefulWidget {
  const FormsPage({super.key});

  @override
  State<FormsPage> createState() => _FormsPageState();
}

class _FormsPageState extends State<FormsPage> {
  final _name = TextEditingController(text: 'DazzUI');
  final _search = TextEditingController();

  bool _newsletter = true;
  bool _telemetry = false;
  String _plan = 'team';
  bool _notify = true;
  String _appearance = 'auto';

  @override
  void dispose() {
    _name.dispose();
    _search.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing6,
      children: [
        const PageTitle(
          'Forms',
          'Every field is pinned to the same control height, so a row of '
              'them lines up.',
        ),
        Specimen(
          'Fields',
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: vars.dialogWidth),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing4,
              children: [
                TextField(controller: _name, placeholder: 'Workspace name'),
                const TextField(placeholder: 'you@example.com'),
                const TextField(
                  placeholder: 'Unavailable on this plan',
                  enabled: false,
                ),
              ],
            ),
          ),
        ),
        Specimen(
          'Search',
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: vars.dialogWidth),
            child: TextField(
              controller: _search,
              placeholder: 'Search components',
              prefix: Padding(
                padding: EdgeInsets.only(left: vars.spacing2),
                child: Icon(
                  FluentIcons.search_16_regular,
                  size: vars.spacing4,
                  color: vars.colorContentSubtle,
                ),
              ),
            ),
          ),
        ),
        Specimen(
          'Checkboxes',
          note: 'a note rides on the label, not on the row',
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing25,
            children: [
              Checkbox(
                value: _newsletter,
                label: const Text('Send me the monthly newsletter'),
                onChanged: (value) => setState(() => _newsletter = value),
              ),
              Checkbox(
                value: _telemetry,
                label: const Text('Share anonymous usage data'),
                note: const Text('Crash reports only'),
                onChanged: (value) => setState(() => _telemetry = value),
              ),
              const Checkbox(
                value: false,
                label: Text('Unavailable on this plan'),
                onChanged: null,
              ),
            ],
          ),
        ),
        Specimen(
          'Radio group',
          note: 'selection shifts weight as well as colour',
          child: RadioGroup<String>(
            value: _plan,
            onChanged: (value) => setState(() => _plan = value!),
            options: const [
              RadioItem(value: 'solo', label: Text('Solo')),
              RadioItem(value: 'team', label: Text('Team')),
              RadioItem(
                value: 'enterprise',
                label: Text('Enterprise'),
                enabled: false,
              ),
            ],
          ),
        ),
        Specimen(
          'Switches',
          child: RowWrap(
            spacing: vars.spacing4,
            children: [
              Switch(
                value: _notify,
                onChanged: (value) => setState(() => _notify = value),
              ),
              Switch(
                value: !_notify,
                onChanged: (value) => setState(() => _notify = !value),
              ),
              const Switch(value: false, onChanged: null),
            ],
          ),
        ),
        Specimen(
          'Option cards',
          note: 'the border thickens as well as colours when chosen',
          // IntrinsicHeight is what gives the pair equal height where the
          // stylesheet uses `flex: 1` on a flex row.
          child: IntrinsicHeight(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              spacing: vars.spacing3,
              children: [
                Expanded(
                  child: OptionCard(
                    title: 'Automatic',
                    description: 'Follow the system appearance.',
                    selected: _appearance == 'auto',
                    onPressed: () => setState(() => _appearance = 'auto'),
                  ),
                ),
                Expanded(
                  child: OptionCard(
                    title: 'Manual',
                    description: 'Pick a theme and keep it.',
                    selected: _appearance == 'manual',
                    onPressed: () => setState(() => _appearance = 'manual'),
                  ),
                ),
              ],
            ),
          ),
        ),
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}
