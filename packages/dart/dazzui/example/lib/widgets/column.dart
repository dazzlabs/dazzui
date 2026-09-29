import 'package:dazzui/dazzui.dart';

enum ShellSide { left, right }

/// A full-height column of the window: a header as tall as the toolbar, then
/// a list that scrolls, on the column's own surface, with the hairline on the
/// edge that faces the content.
class ShellColumn extends StatelessWidget {
  const ShellColumn({
    super.key,
    required this.width,
    required this.side,
    required this.surface,
    required this.header,
    required this.headerPadding,
    this.headerRule = true,
    this.contentPadding,
    required this.children,
  });

  final double width;
  final ShellSide side;
  final Color surface;
  final Widget header;
  final EdgeInsetsGeometry headerPadding;

  /// Whether a hairline separates the header from the list.
  final bool headerRule;

  final EdgeInsetsGeometry? contentPadding;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final BorderSide rule = BorderSide(
      color: vars.colorBorder,
      width: context.hairlineWidth,
    );
    return Container(
      width: width,
      decoration: BoxDecoration(
        color: surface,
        border: side == ShellSide.left
            ? BorderDirectional(end: rule)
            : BorderDirectional(start: rule),
      ),
      child: OverflowBox(
        alignment: AlignmentDirectional.topStart,
        minWidth: 0,
        maxWidth: width == 0 ? 1 : width,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Container(
              constraints: BoxConstraints(minHeight: vars.frameTitlebarSize),
              padding: headerPadding,
              alignment: AlignmentDirectional.centerStart,
              decoration: headerRule
                  ? BoxDecoration(border: Border(bottom: rule))
                  : null,
              child: header,
            ),
            Expanded(
              child: SingleChildScrollView(
                padding:
                    contentPadding ??
                    EdgeInsets.symmetric(vertical: vars.spacing2),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: children,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
