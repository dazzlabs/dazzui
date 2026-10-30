import 'package:flutter/widgets.dart';

import '../generated/theme_variables.dart';
import '../theme/theme.dart';

/// The second column: the groups, books or pages inside whatever the sidebar
/// selected. Lighter than the sidebar, because it sits closer to the content.
class Rail extends StatelessWidget {
  const Rail({super.key, required this.children});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      width: vars.frameRailWidth,
      decoration: BoxDecoration(
        color: vars.colorSurface,
        border: BorderDirectional(
          end: BorderSide(
            color: vars.colorBorder,
            width: context.hairlineWidth,
          ),
        ),
      ),
      child: SingleChildScrollView(
        padding: EdgeInsets.symmetric(
          vertical: vars.spacing35,
          horizontal: vars.spacing25,
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisSize: MainAxisSize.min,
          spacing: vars.frameNavGap,
          children: children,
        ),
      ),
    );
  }
}

/// The trailing information column.
///
/// It shares the sidebar's surface rather than the rail's, so a window reads
/// as content between two quieter edges rather than as three different greys.
/// What it holds is the app's: where the sidebar is a column of rows and owns
/// their rhythm, this is a panel — a title, a list of its own, whatever the
/// selection is about.
///
/// It collapses the way the leading column does, mirrored: the column keeps
/// the width it has when open and is cut by the edge that faces the content
/// as it shuts, so what it holds holds still against the window's own edge
/// while the divider slides over it. Gone, it leaves the semantics tree, the
/// focus traversal and the pointer's reach with it.
class Aside extends StatefulWidget {
  const Aside({
    super.key,
    this.width,
    this.collapsed = false,
    required this.child,
  });

  /// An explicit width. Leave it out and the column takes the token's.
  final double? width;

  /// Whether the column is put away. The host owns the state — a toolbar
  /// button, a shortcut, a drag past the floor — and toggles it; how it
  /// travels is the column's.
  final bool collapsed;

  final Widget child;

  @override
  State<Aside> createState() => _AsideState();
}

class _AsideState extends State<Aside> with SingleTickerProviderStateMixin {
  /// How far open the column is: one is open, zero shut. Only
  /// [Aside.collapsed] sweeps; a width the host re-points is applied as it
  /// comes, so a handle still follows the pointer.
  late final AnimationController _sweep = AnimationController(
    vsync: this,
    value: widget.collapsed ? 0 : 1,
  );

  @override
  void didUpdateWidget(Aside old) {
    super.didUpdateWidget(old);
    if (widget.collapsed == old.collapsed) return;
    _sweep.duration = Theme.of(context).vars.motionDuration;
    if (widget.collapsed) {
      _sweep.reverse();
    } else {
      _sweep.forward();
    }
  }

  @override
  void dispose() {
    _sweep.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final double open = widget.width ?? vars.frameAsideWidth;
    // The content keeps the width it has when open and is cut by the edge
    // that faces the content as the column closes: a panel that laid itself
    // out again on the way out would wrap and ellipsize in front of the
    // reader. It stays against the window's own edge, so the cut comes off
    // the leading side — the sidebar's mirror.
    final double content = (open - context.hairlineWidth)
        .clamp(0.0, open)
        .toDouble();

    return ExcludeFocus(
      excluding: widget.collapsed,
      child: ExcludeSemantics(
        excluding: widget.collapsed,
        child: IgnorePointer(
          ignoring: widget.collapsed,
          child: AnimatedBuilder(
            animation: _sweep,
            builder: (BuildContext context, Widget? child) {
              final double edge = vars.motionEasing.transform(_sweep.value);
              return Container(
                width: open * edge,
                decoration: BoxDecoration(
                  color: vars.colorSurfaceColumn,
                  // The separator is the token's own ink at its own alpha,
                  // scaled by how open the column is — never restated at full
                  // strength, which paints a black rule down the window.
                  border: BorderDirectional(
                    start: BorderSide(
                      color: vars.colorBorder.withValues(
                        alpha: vars.colorBorder.a * edge,
                      ),
                      width: context.hairlineWidth,
                    ),
                  ),
                ),
                child: child,
              );
            },
            child: ClipRect(
              child: OverflowBox(
                alignment: AlignmentDirectional.centerEnd,
                minWidth: content,
                maxWidth: content,
                child: widget.child,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
