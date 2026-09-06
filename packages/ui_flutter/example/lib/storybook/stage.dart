import 'package:dazzui/dazzui.dart';

/// The arrangements a bare specimen needs — the counterpart of the Storybook's
/// `components/stage.tsx`.
///
/// Only a widget's `Overview` is a documented page; every other story draws the
/// widget and nothing else, so all it ever needs is a row or a column with a
/// gap in it, and a width where the widget has none of its own. Nothing here
/// draws.

/// A row that wraps rather than clipping, which is what a shelf of variants
/// wants at any window width.
class RowWrap extends StatelessWidget {
  const RowWrap({
    super.key,
    this.spacing,
    this.alignment = WrapCrossAlignment.center,
    required this.children,
  });

  final double? spacing;
  final WrapCrossAlignment alignment;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final double gap = spacing ?? Theme.of(context).vars.spacing3;

    return Wrap(
      spacing: gap,
      runSpacing: gap,
      crossAxisAlignment: alignment,
      children: children,
    );
  }
}

/// A column, for the widgets that stack. It is a [Column] that has already
/// been told to take its own height and to hang from the leading edge, which
/// is what every story wants and no story should have to say.
class Col extends StatelessWidget {
  const Col({
    super.key,
    this.spacing,
    this.width,
    this.crossAxisAlignment = CrossAxisAlignment.start,
    required this.children,
  });

  final double? spacing;
  final double? width;
  final CrossAxisAlignment crossAxisAlignment;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final Widget column = Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: crossAxisAlignment,
      spacing: spacing ?? Theme.of(context).vars.spacing3,
      children: children,
    );

    return width == null ? column : SizedBox(width: width, child: column);
  }
}

/// A window, for the columns that only make sense with a pane beside them.
///
/// It is a frame rather than a shell: the corner and the border are the
/// design's own window metrics, but nothing here draws a titlebar, because the
/// real one belongs to the app.
class Frame extends StatelessWidget {
  const Frame({
    super.key,
    this.width = 620,
    this.height = 340,
    required this.children,
  });

  final double width;
  final double height;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final BorderRadius radius = BorderRadius.circular(vars.frameWindowRadius);

    return Container(
      width: width,
      height: height,
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: vars.colorSurface,
        borderRadius: radius,
        border: Border.all(color: vars.colorBorder, width: vars.spacingPx),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: children,
      ),
    );
  }
}

/// The empty pane a navigation column is a navigation column to.
class Pane extends StatelessWidget {
  const Pane({super.key, this.children = const []});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Expanded(
      child: Padding(
        padding: EdgeInsets.all(vars.spacing5),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          spacing: vars.spacing3,
          children: children,
        ),
      ),
    );
  }
}

/// A vertical rule between two runs of controls, given the height of the
/// controls beside it.
class ToolbarSeparator extends StatelessWidget {
  const ToolbarSeparator({super.key, this.size = WidgetSize.tiny});

  final WidgetSize size;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final double height = switch (size.namedSize) {
      NamedSize.tiny => vars.controlTinySize,
      NamedSize.small => vars.controlSmallSize,
      NamedSize.medium => vars.controlMediumSize,
      NamedSize.large => vars.controlLargeSize,
    };

    return SizedBox(
      height: height,
      child: VerticalDivider(width: vars.spacingPx),
    );
  }
}
