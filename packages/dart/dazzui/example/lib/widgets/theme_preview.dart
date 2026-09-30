import 'package:dazzui/dazzui.dart';

/// A theme in miniature: a window of its own, drawn in that theme's colours
/// — the column, the paper, two lines of its ink, a button in its accent.
class ThemePreview extends StatelessWidget {
  const ThemePreview({
    super.key,
    required this.vars,
    this.height,
    this.framed = true,
  });

  /// The theme to draw in miniature.
  final ThemeVariables vars;
  final double? height;

  /// A framed preview has its own border and corner; an unframed one runs
  /// to its container's edge and draws only the rule under it.
  final bool framed;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables page = Theme.of(context).vars;
    final BorderSide rule = BorderSide(
      color: framed ? vars.colorBorderStrong : page.colorBorder,
      width: context.hairlineWidth,
    );
    return Container(
      height: height ?? page.spacing14,
      clipBehavior: framed ? Clip.antiAlias : Clip.none,
      decoration: BoxDecoration(
        color: vars.colorSurface,
        border: framed ? Border.fromBorderSide(rule) : Border(bottom: rule),
        borderRadius: framed ? BorderRadius.circular(page.spacing15) : null,
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Expanded(
            flex: 28,
            child: DecoratedBox(
              decoration: BoxDecoration(
                color: vars.colorSurfaceColumn,
                border: Border(
                  right: BorderSide(
                    color: vars.colorBorderStrong,
                    width: context.hairlineWidth,
                  ),
                ),
              ),
            ),
          ),
          Expanded(
            flex: 72,
            child: Padding(
              padding: EdgeInsets.all(framed ? page.spacing2 : page.spacing15),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                spacing: page.spacing1,
                children: [
                  _Ink(color: vars.colorContent, radius: page.spacingPx),
                  FractionallySizedBox(
                    widthFactor: 0.6,
                    child: Opacity(
                      opacity: 0.4,
                      child: _Ink(
                        color: vars.colorContent,
                        radius: page.spacingPx,
                      ),
                    ),
                  ),
                  const Spacer(),
                  Container(
                    width: page.spacing6,
                    height: page.spacing25,
                    decoration: BoxDecoration(
                      color: vars.colorPrimary[500],
                      borderRadius: BorderRadius.circular(page.spacing1),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _Ink extends StatelessWidget {
  const _Ink({required this.color, required this.radius});

  final Color color;
  final double radius;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: Theme.of(context).vars.spacing05,
      decoration: BoxDecoration(
        color: color,
        borderRadius: BorderRadius.circular(radius),
      ),
    );
  }
}
