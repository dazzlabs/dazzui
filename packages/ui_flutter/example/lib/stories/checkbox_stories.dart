import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Checkbox` — a box, its label, and the note under it.
class CheckboxStories {
  CheckboxStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Checkbox', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Checked',
      (context) => Checkbox(
        value: true,
        onChanged: (_) {},
        label: const Text('Send me the newsletter'),
      ),
    ),
    Story(
      'Unchecked',
      (context) => Checkbox(
        value: false,
        onChanged: (_) {},
        label: const Text('Send me the newsletter'),
      ),
    ),
    Story('Interactive', (context) => const _Live()),
    Story('A group', (context) => const _Group()),
    Story(
      'Disabled',
      (context) => const Col(
        children: [
          Checkbox(
            value: false,
            onChanged: null,
            label: Text('Unavailable on this plan'),
          ),
          Checkbox(
            value: true,
            onChanged: null,
            label: Text('Always on for this plan'),
          ),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => Col(
        children: [
          for (final size in sizes)
            Checkbox(
              value: true,
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
          for (final tint in CheckboxTint.values)
            Checkbox(
              value: true,
              onChanged: (_) {},
              tint: tint,
              label: Text(label(tint.name)),
            ),
        ],
      ),
    ),
    Story(
      'With a note',
      (context) => SizedBox(
        width: 380,
        child: Checkbox(
          value: true,
          onChanged: (_) {},
          label: const Text('Share anonymous usage data'),
          note: const Text('Crash reports only.'),
        ),
      ),
    ),
    Story('A checklist', (context) => const _Checklist()),
    Story(
      'In a form',
      (context) => const Col(
        width: 380,
        children: [
          FormField(
            label: 'Platforms',
            hint: 'The packages this change reaches.',
            child: _Checklist(),
          ),
        ],
      ),
    ),
  ]);
}

/// The box is a control over its own state rather than a pseudo-class, so the
/// value comes off the host — here, off a `State`.
class _Live extends StatefulWidget {
  const _Live();

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  bool _checked = true;

  @override
  Widget build(BuildContext context) {
    return Checkbox(
      value: _checked,
      onChanged: (value) => setState(() => _checked = value),
      label: const Text('Send me the monthly newsletter'),
    );
  }
}

class _Checklist extends StatefulWidget {
  const _Checklist();

  @override
  State<_Checklist> createState() => _ChecklistState();
}

class _ChecklistState extends State<_Checklist> {
  final Set<String> _selected = {'react', 'flutter'};

  static const List<(String, String)> _platforms = [
    ('react', 'React'),
    ('flutter', 'Flutter'),
    ('swiftui', 'SwiftUI'),
    ('gpui', 'GPUI'),
  ];

  @override
  Widget build(BuildContext context) {
    return Col(
      spacing: Theme.of(context).vars.spacing2,
      children: [
        for (final (id, name) in _platforms)
          Checkbox(
            value: _selected.contains(id),
            onChanged: (on) => setState(() {
              if (on) {
                _selected.add(id);
              } else {
                _selected.remove(id);
              }
            }),
            label: Text(name),
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
      'Checkbox',
      description:
          'A box, a label beside it and an optional note under that. The whole '
          'row is the target, and the box carries the tint — so a checklist '
          'reads as one column of decisions rather than as a column of squares.',
      children: [
        DocSection(
          'States',
          description:
              'Checked, unchecked and disabled. Disabled flattens to the '
              'groove — one dead look whatever the state under it — rather '
              'than a translucent ghost of the live control.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'checked',
                  children: [
                    Checkbox(
                      value: true,
                      onChanged: (_) {},
                      label: const Text('Newsletter'),
                    ),
                  ],
                ),
                Specimen(
                  'unchecked',
                  children: [
                    Checkbox(
                      value: false,
                      onChanged: (_) {},
                      label: const Text('Newsletter'),
                    ),
                  ],
                ),
                const Specimen(
                  'disabled',
                  children: [
                    Checkbox(
                      value: false,
                      onChanged: null,
                      label: Text('Newsletter'),
                    ),
                    Checkbox(
                      value: true,
                      onChanged: null,
                      label: Text('Newsletter'),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes and tints',
          description:
              'The box is a token of its own — a checkbox is squarer than the '
              'control radius — and its three sizes are the checkbox’s, not '
              'the control profile’s.',
          children: [
            Specimens(
              children: [
                for (final size in CheckboxStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      for (final tint in CheckboxTint.values)
                        Checkbox(
                          value: true,
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
          'The note',
          description:
              'The note is the aside beside the label, in faint ink — the '
              'qualification the decision needs, not a second decision. It '
              'shares the row, so it has to stay short: a sentence belongs in '
              'a `FormField` hint instead.',
          children: [
            SizedBox(
              width: 420,
              child: Checkbox(
                value: true,
                onChanged: (_) {},
                label: const Text('Share anonymous usage data'),
                note: const Text('Crash reports only.'),
              ),
            ),
          ],
        ),
        DocSection(
          'Tokens',
          description: 'The ones the checkbox owns; everything else is shared.',
          children: [
            TokenTable([
              TokenRow.length('checkbox.radius', vars.checkboxRadius),
              TokenRow.length('checkbox.thickness', vars.checkboxThickness),
              TokenRow.length('checkbox.box.small', vars.checkboxSmallBox),
              TokenRow.length('checkbox.box.medium', vars.checkboxMediumBox),
              TokenRow.length('checkbox.box.large', vars.checkboxLargeBox),
            ]),
          ],
        ),
      ],
    );
  }
}

/// The parent is what a lone checkbox can never be: mixed. It reads the group
/// rather than a value of its own, and pressing it sets or clears the lot —
/// twice restores what was checked before.
class _Group extends StatefulWidget {
  const _Group();

  @override
  State<_Group> createState() => _GroupState();
}

class _GroupState extends State<_Group> {
  static const List<String> platforms = ['react', 'flutter', 'swiftui', 'gpui'];

  Set<String> _value = {'react', 'flutter'};

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 288,
      child: CheckboxGroup(
        allValues: platforms,
        value: _value,
        onChanged: (value) => setState(() => _value = value),
        children: [
          Checkbox(
            parent: true,
            label: const Text('Every platform'),
            note: Text('${_value.length} of ${platforms.length}'),
          ),
          for (final String platform in platforms)
            Checkbox(name: platform, label: Text(platform)),
        ],
      ),
    );
  }
}
