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
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}

String _capitalize(String word) => word[0].toUpperCase() + word.substring(1);
