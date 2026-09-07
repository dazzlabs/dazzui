import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/IconButton` — the square button a toolbar is made of.
class IconButtonStories {
  IconButtonStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.tiny,
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('IconButton', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Plain',
      (context) =>
          IconButton(icon: Icon(FluentIcons.pin_16_regular), onPressed: () {}),
    ),
    Story(
      'Filled',
      (context) => IconButton(
        icon: Icon(FluentIcons.add_16_regular),
        variant: IconButtonVariant.filled,
        onPressed: () {},
      ),
    ),
    Story(
      'Variants',
      (context) => RowWrap(
        children: [
          for (final variant in IconButtonVariant.values)
            IconButton(
              icon: Icon(FluentIcons.star_16_regular),
              variant: variant,
              onPressed: () {},
            ),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => RowWrap(
        children: [
          for (final variant in IconButtonVariant.values)
            IconButton(
              icon: Icon(FluentIcons.star_16_regular),
              variant: variant,
              onPressed: null,
            ),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => RowWrap(
        children: [
          for (final size in sizes)
            IconButton(
              icon: Icon(FluentIcons.star_16_regular),
              size: size,
              onPressed: () {},
            ),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => RowWrap(
        children: [
          for (final tint in IconButtonTint.values)
            IconButton(
              icon: Icon(FluentIcons.star_16_regular),
              variant: IconButtonVariant.tinted,
              tint: tint,
              onPressed: () {},
            ),
        ],
      ),
    ),
    Story('Active', (context) => const _Pin()),
    Story(
      'Active tints',
      (context) => RowWrap(
        children: [
          for (final tint in IconButtonTint.values)
            IconButton(
              icon: Icon(FluentIcons.pin_16_regular),
              tint: tint,
              active: true,
              onPressed: () {},
            ),
        ],
      ),
    ),
    Story('A toolbar', (context) => const _Toolbar()),
    Story(
      'In a list header',
      (context) => SizedBox(
        width: 300,
        child: Card(
          child: Row(
            children: [
              const Text('Builds'),
              const Spacer(),
              IconButton(
                icon: Icon(FluentIcons.filter_16_regular),
                onPressed: () {},
              ),
              IconButton(
                icon: Icon(FluentIcons.add_16_regular),
                onPressed: () {},
              ),
            ],
          ),
        ),
      ),
    ),
  ]);
}

/// The on-state: an icon button that holds a decision rather than firing one.
class _Pin extends StatefulWidget {
  const _Pin();

  @override
  State<_Pin> createState() => _PinState();
}

class _PinState extends State<_Pin> {
  bool _pinned = true;

  @override
  Widget build(BuildContext context) {
    return RowWrap(
      children: [
        IconButton(
          icon: Icon(FluentIcons.pin_16_regular),
          active: _pinned,
          onPressed: () => setState(() => _pinned = !_pinned),
        ),
        IconButton(icon: Icon(FluentIcons.pin_16_regular), onPressed: () {}),
      ],
    );
  }
}

class _Toolbar extends StatelessWidget {
  const _Toolbar();

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 300,
      child: ActionBar(
        children: [
          IconButton(
            icon: Icon(FluentIcons.chevron_left_16_regular),
            onPressed: () {},
          ),
          IconButton(
            icon: Icon(FluentIcons.chevron_right_16_regular),
            onPressed: () {},
          ),
          const ToolbarSeparator(),
          IconButton(
            icon: Icon(FluentIcons.arrow_clockwise_16_regular),
            onPressed: () {},
          ),
          IconButton(
            icon: Icon(FluentIcons.share_16_regular),
            onPressed: () {},
          ),
          const ToolbarSeparator(),
          IconButton(
            icon: Icon(FluentIcons.delete_16_regular),
            tint: IconButtonTint.danger,
            onPressed: () {},
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
    return DocPage(
      'IconButton',
      description:
          'A button whose label is a glyph. It is the same control as '
          '`Button` — the same recipes, the same states — squared to the '
          'control height, with the label moved into the accessible name. It '
          'defaults to `plain` at the `tiny` step, because a toolbar of these '
          'should read as chrome rather than as a row of accented actions.',
      children: [
        DocSection(
          'Variants',
          description:
              'Every control recipe, on any ramp. Pass a tint where one button '
              'genuinely leads — a destructive action in a toolbar of neutral '
              'ones.',
          children: [
            Specimens(
              children: [
                for (final variant in IconButtonVariant.values)
                  Specimen(
                    variant.name,
                    children: [
                      for (final tint in IconButtonTint.values)
                        IconButton(
                          icon: Icon(FluentIcons.star_16_regular),
                          variant: variant,
                          tint: tint,
                          onPressed: () {},
                        ),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes',
          description:
              'The control profile again, and `tiny` is the one this widget '
              'was drawn for: a square on the toolbar step.',
          children: [
            RowWrap(
              children: [
                for (final size in IconButtonStories.sizes)
                  IconButton(
                    icon: Icon(FluentIcons.star_16_regular),
                    size: size,
                    onPressed: () {},
                  ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Active',
          description:
              '`active` is the on-state of a button that holds a decision — a '
              'pin, a filter, a panel toggle. It takes the filled recipe, so '
              'an on button reads the way a selected row does rather than as a '
              'pressed one.',
          children: [_Pin()],
        ),
        const DocSection(
          'In a toolbar',
          description:
              'Runs of buttons separated by a vertical rule, and at most one '
              'tinted button.',
          children: [_Toolbar()],
        ),
      ],
    );
  }
}
