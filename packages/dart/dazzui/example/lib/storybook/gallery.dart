import 'package:dazzui/dazzui.dart';
import 'package:flutter/rendering.dart' show OverflowBoxFit;
import 'package:flutter/widgets.dart' as widgets;

import 'stage.dart';

/// Documentation presentation — the counterpart of the Storybook's
/// `components/gallery.tsx`, and used by the same one story per widget:
/// `Overview`. Every other story is the widget alone and reaches for
/// `stage.dart` instead.
///
/// It is drawn with the design system's own tokens, so a theme switch carries
/// the page furniture with it too.

/// The measure the Storybook sets its prose to — 48rem. A line length is a
/// typographic decision rather than a step off the spacing scale, so it is
/// stated once here instead of being composed out of one.
const double proseMeasure = 768;

/// A length, printed the way the token graph states it.
String px(double value) {
  final String number = value == value.roundToDouble()
      ? value.round().toString()
      : value.toStringAsFixed(2).replaceFirst(RegExp(r'0+$'), '');
  return '${number}px';
}

/// The face a token name is printed in.
///
/// `base.font.code` has no semantic token in front of it, and Flutter has no
/// "the system monospaced face" request the way SwiftUI's `Font.Design` does,
/// so the stack is named once here. It is a playground decision rather than a
/// design one — nothing a widget draws reads it.
TextStyle mono(TextStyle style) => style.copyWith(
  fontFamily: 'SF Mono',
  fontFamilyFallback: const ['Menlo', 'Consolas', 'monospace'],
);

/// An axis case, printed the way a story labels it.
String label(String name) => name[0].toUpperCase() + name.substring(1);

/// The name column a specimen and a bar both hang their label in: the
/// Storybook's 6rem, composed out of the two steps that make it rather than
/// written down.
double nameColumn(ThemeVariables vars) => vars.spacing20 + vars.spacing4;

/// A documented page: what the widget is, then the sections that argue for
/// each of its axes.
class DocPage extends StatelessWidget {
  const DocPage(
    this.title, {
    super.key,
    this.description,
    required this.children,
  });

  final String title;
  final String? description;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Padding(
      padding: EdgeInsets.all(vars.spacing8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing10,
        children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing2,
            children: [
              Text(
                title,
                style: vars.headlineMedium.copyWith(color: vars.colorContent),
              ),
              if (description != null) Prose(description!),
            ],
          ),
          ...children,
        ],
      ),
    );
  }
}

/// One axis of a widget, named and argued for.
class DocSection extends StatelessWidget {
  const DocSection(
    this.title, {
    super.key,
    this.description,
    required this.children,
  });

  final String title;
  final String? description;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing4,
      children: [
        Container(
          width: double.infinity,
          padding: EdgeInsets.only(bottom: vars.spacing3),
          decoration: BoxDecoration(
            border: Border(
              bottom: BorderSide(
                color: vars.colorBorder,
                width: context.hairlineWidth,
              ),
            ),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing2,
            children: [
              Text(
                title,
                style: vars.titleMedium.copyWith(color: vars.colorContent),
              ),
              if (description != null) Prose(description!),
            ],
          ),
        ),
        ...children,
      ],
    );
  }
}

/// A paragraph, held to the measure.
class Prose extends StatelessWidget {
  const Prose(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: proseMeasure),
      child: Text(
        text,
        style: vars.bodySmall.copyWith(color: vars.colorContentMuted),
      ),
    );
  }
}

/// A run of specimens.
class Specimens extends StatelessWidget {
  const Specimens({super.key, required this.children});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing3,
      children: children,
    );
  }
}

/// One demo row: what the variation is called, and the live widget.
class Specimen extends StatelessWidget {
  const Specimen(this.name, {super.key, required this.children});

  final String name;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      spacing: vars.spacing4,
      children: [
        SizedBox(
          // The Storybook's specimen grid: a 6rem name column, then the stage.
          width: nameColumn(vars),
          child: Text(
            name,
            style: mono(
              vars.captionMedium.copyWith(color: vars.colorContentMuted),
            ),
          ),
        ),
        Expanded(child: RowWrap(children: children)),
      ],
    );
  }
}

/// A grid of tiles, for the token pages where a shelf would run off the edge.
class TileGrid extends StatelessWidget {
  const TileGrid({super.key, this.min = 240, required this.children});

  final double min;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return LayoutBuilder(
      builder: (context, constraints) {
        final double available = constraints.maxWidth.isFinite
            ? constraints.maxWidth
            : proseMeasure;
        final gap = vars.spacing4;
        final extent = available < min ? min : available;
        final int columns = ((extent + gap) / (min + gap)).floor().clamp(
          1,
          65535,
        );
        final double width = (extent - gap * (columns - 1)) / columns;

        return OverflowBox(
          alignment: Alignment.topLeft,
          fit: OverflowBoxFit.deferToChild,
          minWidth: extent,
          maxWidth: extent,
          child: widgets.Table(
            defaultVerticalAlignment:
                widgets.TableCellVerticalAlignment.intrinsicHeight,
            columnWidths: {
              for (var i = 0; i < columns * 2 - 1; i++)
                i: widgets.FixedColumnWidth(i.isEven ? width : gap),
            },
            children: [
              for (
                var start = 0;
                start < children.length;
                start += columns
              ) ...[
                if (start > 0)
                  widgets.TableRow(
                    children: [
                      for (var i = 0; i < columns * 2 - 1; i++)
                        SizedBox(height: gap),
                    ],
                  ),
                widgets.TableRow(
                  children: [
                    for (var i = 0; i < columns * 2 - 1; i++)
                      if (i.isEven && start + i ~/ 2 < children.length)
                        children[start + i ~/ 2]
                      else
                        const SizedBox.shrink(),
                  ],
                ),
              ],
            ],
          ),
        );
      },
    );
  }
}

/// One row of a widget's token table.
class TokenRow {
  const TokenRow(this.name, this.value);

  TokenRow.length(this.name, double value) : value = px(value);

  final String name;
  final String value;
}

/// The tokens a widget owns, and what they resolve to in this theme.
///
/// It appears only where the widget genuinely owns tokens — the files in
/// `ds/tokens/component/`. Most widgets own none: they read the
/// shared vocabulary directly, and a table that would come back empty is not
/// worth a section.
class TokenTable extends StatelessWidget {
  const TokenTable(this.rows, {super.key});

  final List<TokenRow> rows;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: proseMeasure),
      child: Table(
        children: [
          const TableHead(
            children: [
              TableCell(head: true, child: Text('Token')),
              TableCell(
                head: true,
                align: TableCellAlign.end,
                child: Text('Resolved'),
              ),
            ],
          ),
          for (final row in rows)
            TableRow(
              children: [
                TableCell(
                  child: Text(
                    row.name,
                    style: mono(vars.captionMedium),
                  ),
                ),
                TableCell(
                  align: TableCellAlign.end,
                  child: Text(
                    row.value,
                    style: mono(
                      vars.captionSmall.copyWith(
                        color: vars.colorContentMuted,
                      ),
                    ),
                  ),
                ),
              ],
            ),
        ],
      ),
    );
  }
}

/// A colour role: the swatch, its name, and what it is for.
class Swatch extends StatelessWidget {
  const Swatch(this.name, this.color, {super.key, this.note});

  final String name;
  final Color color;
  final String? note;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing2,
      children: [
        Container(
          height: vars.spacing14,
          decoration: BoxDecoration(
            color: color,
            borderRadius: BorderRadius.circular(vars.radiusMedium),
            border: Border.all(color: vars.colorBorder, width: vars.spacingPx),
          ),
        ),
        Text(
          name,
          style: mono(vars.captionMedium.copyWith(color: vars.colorContent)),
        ),
        if (note != null)
          Text(
            note!,
            style: vars.captionSmall.copyWith(color: vars.colorContentMuted),
          ),
      ],
    );
  }
}

/// The eleven steps of one ramp, in the theme it is read against.
class Ramp extends StatelessWidget {
  const Ramp(this.name, this.swatch, {super.key});

  final String name;
  final ColorSwatch<int> swatch;

  static const List<int> shades = [
    50,
    100,
    200,
    300,
    400,
    500,
    600,
    700,
    800,
    900,
    950,
  ];

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing2,
      children: [
        Text(name, style: vars.labelMedium.copyWith(color: vars.colorContent)),
        RowWrap(
          spacing: vars.spacing2,
          children: [
            for (final shade in shades)
              Column(
                mainAxisSize: MainAxisSize.min,
                spacing: vars.spacing1,
                children: [
                  Container(
                    width: vars.spacing12,
                    height: vars.spacing10,
                    decoration: BoxDecoration(
                      color: swatch[shade],
                      borderRadius: BorderRadius.circular(vars.radiusSmall),
                      border: Border.all(
                        color: vars.colorBorder,
                        width: vars.spacingPx,
                      ),
                    ),
                  ),
                  Text(
                    '$shade',
                    style: mono(
                      vars.captionSmall.copyWith(
                        color: vars.colorContentMuted,
                      ),
                    ),
                  ),
                ],
              ),
          ],
        ),
      ],
    );
  }
}

/// A framed preview with its name under it — a radius, a shadow, a face.
class Tile extends StatelessWidget {
  const Tile(this.name, {super.key, this.note, required this.child});

  final String name;
  final String? note;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      padding: EdgeInsets.all(vars.spacing4),
      decoration: BoxDecoration(
        color: vars.colorSurface,
        borderRadius: BorderRadius.circular(vars.radiusMedium),
        border: Border.all(color: vars.colorBorder, width: vars.spacingPx),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing3,
        children: [
          SizedBox(
            height: vars.spacing20,
            width: double.infinity,
            child: Center(child: child),
          ),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing05,
            children: [
              Text(
                name,
                style: mono(
                  vars.captionMedium.copyWith(color: vars.colorContent),
                ),
              ),
              if (note != null)
                Text(
                  note!,
                  style: vars.captionSmall.copyWith(
                    color: vars.colorContentMuted,
                  ),
                ),
            ],
          ),
        ],
      ),
    );
  }
}

/// A length, drawn to scale beside its name — the spacing scale's page.
class Bar extends StatelessWidget {
  const Bar(this.name, this.length, {super.key});

  final String name;
  final double length;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Row(
      spacing: vars.spacing4,
      children: [
        SizedBox(
          width: nameColumn(vars),
          child: Text(
            name,
            style: mono(
              vars.captionMedium.copyWith(color: vars.colorContent),
            ),
          ),
        ),
        Container(
          width: length < vars.spacingPx ? vars.spacingPx : length,
          height: vars.spacing3,
          decoration: BoxDecoration(
            color: vars.colorPrimary[500],
            borderRadius: BorderRadius.circular(vars.radiusTiny),
          ),
        ),
        Text(
          px(length),
          style: mono(
            vars.captionSmall.copyWith(color: vars.colorContentMuted),
          ),
        ),
      ],
    );
  }
}
