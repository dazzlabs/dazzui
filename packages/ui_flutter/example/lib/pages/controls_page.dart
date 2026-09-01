import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../gallery.dart';

/// The control vocabulary: every recipe, every size, and the annotations that
/// sit beside them.
class ControlsPage extends StatelessWidget {
  const ControlsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing6,
      children: [
        const PageTitle(
          'Controls',
          'A variant picks the recipe, a tint picks the ramp it resolves '
              'against.',
        ),
        Specimen(
          'Variants',
          note: 'filled · tinted · outlined · plain',
          child: RowWrap(
            children: [
              for (final variant in ButtonVariant.values)
                Button(
                  variant: variant,
                  onPressed: () {},
                  child: Text(_capitalize(variant.name)),
                ),
            ],
          ),
        ),
        Specimen(
          'Sizes',
          note: 'geometry and type both come off the size profile',
          child: RowWrap(
            children: [
              for (final size in const [
                WidgetSize.small,
                WidgetSize.medium,
                WidgetSize.large,
              ])
                Button(
                  size: size,
                  variant: ButtonVariant.filled,
                  onPressed: () {},
                  child: Text(_capitalize(size.namedSize.name)),
                ),
            ],
          ),
        ),
        Specimen(
          'Tints',
          note: 'one recipe table, six ramps',
          child: RowWrap(
            children: [
              for (final tint in ButtonTint.values)
                Button(
                  variant: ButtonVariant.tinted,
                  tint: tint,
                  onPressed: () {},
                  child: Text(_capitalize(tint.name)),
                ),
            ],
          ),
        ),
        Specimen(
          'Disabled',
          note: 'every variant flattens to the same groove',
          child: RowWrap(
            children: [
              for (final variant in ButtonVariant.values)
                Button(
                  variant: variant,
                  onPressed: null,
                  child: Text(_capitalize(variant.name)),
                ),
            ],
          ),
        ),
        Specimen(
          'Icon buttons',
          note: 'quiet chrome until touched',
          child: RowWrap(
            children: [
              IconButton(
                icon: FluentIcons.chevron_left_16_regular,
                onPressed: () {},
              ),
              IconButton(
                icon: FluentIcons.chevron_right_16_regular,
                onPressed: () {},
              ),
              IconButton(
                icon: FluentIcons.delete_16_regular,
                variant: IconButtonVariant.tinted,
                tint: IconButtonTint.danger,
                onPressed: () {},
              ),
              const IconButton(
                icon: FluentIcons.star_16_regular,
                onPressed: null,
              ),
            ],
          ),
        ),
        Specimen(
          'Badges',
          note: 'annotations, so they never react to the pointer',
          child: RowWrap(
            children: [
              const Badge(variant: BadgeVariant.filled, child: Text('Filled')),
              const Badge(variant: BadgeVariant.tinted, child: Text('Tinted')),
              const Badge(
                variant: BadgeVariant.outlined,
                child: Text('Outlined'),
              ),
              const Badge(variant: BadgeVariant.plain, child: Text('Plain')),
              const Badge(
                variant: BadgeVariant.tinted,
                tint: BadgeTint.warning,
                child: Text('Beta'),
              ),
              const Badge(
                variant: BadgeVariant.tinted,
                tint: BadgeTint.danger,
                child: Text('Failed'),
              ),
            ],
          ),
        ),
        Specimen(
          'Segmented control',
          note: 'the capsule sits inside the track, corners concentric',
          child: _SegmentedDemo(),
        ),
        Specimen(
          'Pill tabs',
          note: 'no track — each pill stands on the page',
          child: _PillDemo(),
        ),
        const Specimen(
          'Keys',
          child: RowWrap(
            children: [
              Kbd('⌘K'),
              Kbd('⇧⌘P'),
              Kbd('Esc'),
            ],
          ),
        ),
        Specimen(
          'Section labels',
          child: RowWrap(
            spacing: vars.spacing4,
            children: const [
              SectionLabel('Components'),
              SectionLabel('Navigation', tone: SectionLabelTone.faint),
              SectionLabel('Failing', tint: SectionLabelTint.danger),
            ],
          ),
        ),
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}

class _SegmentedDemo extends StatefulWidget {
  @override
  State<_SegmentedDemo> createState() => _SegmentedDemoState();
}

class _SegmentedDemoState extends State<_SegmentedDemo> {
  String _mode = 'design';

  @override
  Widget build(BuildContext context) {
    return SegmentedControl<String>(
      value: _mode,
      onChanged: (value) => setState(() => _mode = value),
      items: const [
        SegmentedItem(value: 'design', label: 'Design'),
        SegmentedItem(value: 'code', label: 'Code'),
        SegmentedItem(value: 'preview', label: 'Preview'),
      ],
    );
  }
}

class _PillDemo extends StatefulWidget {
  @override
  State<_PillDemo> createState() => _PillDemoState();
}

class _PillDemoState extends State<_PillDemo> {
  String _tab = 'all';

  @override
  Widget build(BuildContext context) {
    return PillTabs<String>(
      value: _tab,
      onChanged: (value) => setState(() => _tab = value),
      items: const [
        PillTabItem(value: 'all', label: 'All', count: 24),
        PillTabItem(value: 'open', label: 'Open', count: 3),
        PillTabItem(value: 'done', label: 'Done', count: 21),
        PillTabItem(value: 'archived', label: 'Archived', enabled: false),
      ],
    );
  }
}

String _capitalize(String word) => word[0].toUpperCase() + word.substring(1);
