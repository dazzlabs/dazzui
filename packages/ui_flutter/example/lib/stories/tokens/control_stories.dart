import 'package:dazzui/dazzui.dart';

import '../../storybook/gallery.dart';
import '../../storybook/stage.dart';
import '../../storybook/story.dart';

/// `Tokens/Control` — the shared vocabulary most widgets read instead of
/// owning tokens of their own.
class ControlStories {
  ControlStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.tiny,
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Control', [
    Story('Sizing', (context) => const _Sizing(), layout: StoryLayout.page),
    Story('Colors', (context) => const _Colors(), layout: StoryLayout.page),
    Story('Focus', (context) => const _Focus(), layout: StoryLayout.page),
  ]);
}

class _Sizing extends StatelessWidget {
  const _Sizing();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    final List<(String, double, double, double, double, double)> profiles = [
      (
        'tiny',
        vars.controlTinySize,
        vars.controlTinyPaddingBlock,
        vars.controlTinyPaddingInline,
        vars.controlTinyGap,
        vars.controlTinyRadius,
      ),
      (
        'small',
        vars.controlSmallSize,
        vars.controlSmallPaddingBlock,
        vars.controlSmallPaddingInline,
        vars.controlSmallGap,
        vars.controlSmallRadius,
      ),
      (
        'medium',
        vars.controlMediumSize,
        vars.controlMediumPaddingBlock,
        vars.controlMediumPaddingInline,
        vars.controlMediumGap,
        vars.controlMediumRadius,
      ),
      (
        'large',
        vars.controlLargeSize,
        vars.controlLargePaddingBlock,
        vars.controlLargePaddingInline,
        vars.controlLargeGap,
        vars.controlLargeRadius,
      ),
    ];

    return DocPage(
      'Control sizing',
      description:
          'Four size profiles, each a height, a gap, two pads, a corner and a '
          'type face. A sized control reads the profile once rather than '
          'restating geometry per size — Button and IconButton keep no '
          'per-size block at all, because the profile carries the design’s own '
          'control type.',
      children: [
        DocSection(
          'Profiles',
          description: 'What every control on that step is built from.',
          children: [
            ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: proseMeasure),
              child: DataTable(
                children: [
                  const TableHead(
                    children: [
                      TableCell(head: true, child: Text('Size')),
                      TableCell(
                        head: true,
                        align: TableCellAlign.end,
                        child: Text('Height'),
                      ),
                      TableCell(
                        head: true,
                        align: TableCellAlign.end,
                        child: Text('Pad'),
                      ),
                      TableCell(
                        head: true,
                        align: TableCellAlign.end,
                        child: Text('Gap'),
                      ),
                      TableCell(
                        head: true,
                        align: TableCellAlign.end,
                        child: Text('Radius'),
                      ),
                    ],
                  ),
                  for (final (name, height, block, inline, gap, radius)
                      in profiles)
                    TableRow(
                      children: [
                        TableCell(child: Text(name)),
                        TableCell(
                          align: TableCellAlign.end,
                          child: Text(px(height)),
                        ),
                        TableCell(
                          align: TableCellAlign.end,
                          child: Text('${px(block)} · ${px(inline)}'),
                        ),
                        TableCell(
                          align: TableCellAlign.end,
                          child: Text(px(gap)),
                        ),
                        TableCell(
                          align: TableCellAlign.end,
                          child: Text(px(radius)),
                        ),
                      ],
                    ),
                ],
              ),
            ),
          ],
        ),
        DocSection(
          'On the controls themselves',
          description:
              'A field, a button and an icon button on the same step are the '
              'same height, which is the whole point of the profile.',
          children: [
            Specimens(
              children: [
                for (final size in ControlStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      Button(
                        size: size,
                        variant: ButtonVariant.filled,
                        onPressed: () {},
                        child: const Text('Export'),
                      ),
                      Button(
                        size: size,
                        onPressed: () {},
                        child: const Text('Cancel'),
                      ),
                    ],
                  ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}

/// The recipe table, resolved live.
class _Colors extends StatefulWidget {
  const _Colors();

  @override
  State<_Colors> createState() => _ColorsState();
}

class _ColorsState extends State<_Colors> {
  ButtonTint _tint = ButtonTint.primary;

  static const List<(String, Set<WidgetState>)> _states = [
    ('normal', <WidgetState>{}),
    ('hovered', {WidgetState.hovered}),
    ('pressed', {WidgetState.pressed}),
    ('disabled', {WidgetState.disabled}),
  ];

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final ColorSwatch<int> ramp = vars.controlColor.resolveWith(
      const <WidgetState>{},
      tint: _tint,
    );

    return DocPage(
      'Control colors',
      description:
          'Interactive colours are stored as *recipes* rather than finished '
          'colours: a shade to pick off a ramp plus an alpha to apply, '
          'resolved against whichever ramp the control is tinted with. '
          'Everything below is resolved live — nothing here is a written-down '
          'colour.',
      children: [
        DocSection(
          'The tint',
          description:
              'Pick a ramp and every recipe under it re-resolves. `recessed` '
              'and `normal` do not move: they are the neutral controls, '
              'reading the theme’s surface and ink roles rather than a ramp.',
          children: [
            SegmentedControl<ButtonTint>(
              value: _tint,
              onChanged: (value) => setState(() => _tint = value),
              items: [
                for (final tint in ButtonTint.values)
                  SegmentedItem(value: tint, label: label(tint.name)),
              ],
            ),
          ],
        ),
        DocSection(
          'Resolved',
          description:
              'Surface, content and border, in each of the four states. A cell '
              'most recipes never write is filled in by the expansion — '
              '`hovered` unwritten is `normal`, `pressed` unwritten is '
              '`hovered` dimmed — so a written cell is always a decision.',
          children: [
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing5,
              children: [
                for (final variant in ButtonVariant.values)
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    spacing: vars.spacing2,
                    children: [
                      Text(
                        variant.name,
                        style: vars.labelMedium.copyWith(
                          color: vars.colorContent,
                        ),
                      ),
                      RowWrap(
                        children: [
                          for (final (name, states) in _states)
                            _Cell(
                              name: name,
                              surface: vars.controlColorSurface.resolveWith(
                                states,
                                variant: variant,
                                extra: {'seedColor': ramp},
                              ),
                              content: vars.controlColorContent.resolveWith(
                                states,
                                variant: variant,
                                extra: {'seedColor': ramp},
                              ),
                              border: vars.controlColorBorder.resolveWith(
                                states,
                                variant: variant,
                                extra: {'seedColor': ramp},
                              ),
                            ),
                        ],
                      ),
                    ],
                  ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}

class _Cell extends StatelessWidget {
  const _Cell({
    required this.name,
    required this.surface,
    required this.content,
    required this.border,
  });

  final String name;
  final Color surface;
  final Color content;
  final Color border;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing1,
      children: [
        Container(
          width: vars.spacing20,
          height: vars.spacing8,
          alignment: Alignment.center,
          decoration: BoxDecoration(
            color: surface,
            borderRadius: BorderRadius.circular(vars.radiusSmall),
            border: Border.all(color: border, width: vars.strokeControl),
          ),
          child: Text('Aa', style: vars.labelSmall.copyWith(color: content)),
        ),
        Text(
          name,
          style: mono(
            vars.captionSmall.copyWith(color: vars.colorContentMuted),
          ),
        ),
      ],
    );
  }
}

class _Focus extends StatelessWidget {
  const _Focus();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Focus',
      description:
          'The focus ring is the system accent, always. It never follows the '
          'control’s tint — a ring that changed colour per control would stop '
          'meaning focus — and it hugs the control with no offset, the way '
          'macOS draws it.',
      children: [
        DocSection(
          'Try it',
          description:
              'Tab through these. Each is a different tint, and every ring is '
              'the same colour.',
          children: [
            RowWrap(
              children: [
                for (final tint in ButtonTint.values)
                  Button(
                    variant: ButtonVariant.tinted,
                    tint: tint,
                    onPressed: () {},
                    child: Text(label(tint.name)),
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Tokens',
          description: 'What the ring is made of.',
          children: [
            TokenTable([
              TokenRow.length('focus.width', vars.focusWidth),
              TokenRow.length('focus.offset', vars.focusOffset),
              TokenRow('focus.ring.shade', '${vars.focusRingShade}'),
              TokenRow(
                'focus.ring.alpha',
                vars.focusRingAlpha.toStringAsFixed(2),
              ),
              TokenRow.length('stroke.control', vars.strokeControl),
              TokenRow.length('stroke.hairline', vars.strokeHairline),
            ]),
          ],
        ),
      ],
    );
  }
}
