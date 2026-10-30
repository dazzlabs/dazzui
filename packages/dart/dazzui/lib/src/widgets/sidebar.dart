import 'package:flutter/widgets.dart';

import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'section_label.dart';

/// The left workspace column.
///
/// Its width is a token rather than a layout choice because the columns of a
/// desktop shell have to agree with each other: the header strip is exactly
/// the titlebar's height, or the separator under the two steps where they
/// meet.
///
/// The rows inside are [NavItem]s, run in [SidebarGroup]s — a sidebar owns
/// the column, a group the spacing of its rows.
///
/// A column put away with [collapsed] is not unmounted: it keeps the width it
/// has when open behind a clip anchored to its leading edge and sweeps shut
/// over the motion token, so the pane beside it holds still while the edge
/// slides over it — the way a native split view collapses — and the column's
/// scroll position survives the round trip. Once it is shut it leaves the
/// semantics tree, the focus traversal and the pointer's reach, so a pane
/// that filled the window is the only thing left to read or press. A width the
/// host re-points — a divider being dragged to a new one — is applied as it
/// comes; only the collapse travels.
class Sidebar extends StatefulWidget {
  const Sidebar({
    super.key,
    this.header,
    this.width,
    this.collapsed = false,
    required this.children,
  });

  /// Content for the strip above the row list, held at exactly the titlebar's
  /// height so it lines up with the toolbar in the pane beside it.
  final Widget? header;

  /// An explicit width. Leave it out and the column takes the token's.
  final double? width;

  /// Whether the column is put away. The host owns the state — a trigger, a
  /// shortcut, a drag past the floor — and toggles it; how it travels is the
  /// column's.
  final bool collapsed;

  final List<Widget> children;

  @override
  State<Sidebar> createState() => _SidebarState();
}

class _SidebarState extends State<Sidebar> with SingleTickerProviderStateMixin {
  /// How far open the column is: one is open, zero shut, and every frame
  /// between is the edge the pane beside it sees.
  ///
  /// Only [Sidebar.collapsed] sweeps. A width the host changes — a divider
  /// being dragged, a column re-pointed — is applied as it comes, so a handle
  /// still follows the pointer rather than trailing it over the token.
  late final AnimationController _sweep = AnimationController(
    vsync: this,
    value: widget.collapsed ? 0 : 1,
  );

  @override
  void didUpdateWidget(Sidebar old) {
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
    final double open = widget.width ?? vars.frameSidebarWidth;
    // The content keeps the width it has when open and is cut by the column's
    // own edge as it closes: a column that laid itself out again on the way
    // out would wrap and ellipsize every label in front of the reader.
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
                  // The edge goes with the column rather than standing in the
                  // window after it: a hairline left behind is a column one
                  // pixel wide that nothing can reach.
                  border: BorderDirectional(
                    end: BorderSide(
                      color: vars.colorBorder.withValues(alpha: edge),
                      width: context.hairlineWidth,
                    ),
                  ),
                ),
                child: child,
              );
            },
            child: ClipRect(
              child: OverflowBox(
                alignment: AlignmentDirectional.centerStart,
                minWidth: content,
                maxWidth: content,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    if (widget.header != null)
                      Container(
                        height: vars.frameTitlebarSize,
                        alignment: AlignmentDirectional.centerStart,
                        padding: EdgeInsets.symmetric(
                          horizontal: vars.spacing4,
                        ),
                        child: widget.header!,
                      ),
                    // Only the row list scrolls, so a pinned card stays put.
                    // The groups carry their own block padding, as
                    // `.dz-sidebar__group` does, so two of them stand a full
                    // step apart while the column itself adds only
                    // `.dz-sidebar__content`'s.
                    Expanded(
                      child: SingleChildScrollView(
                        padding: EdgeInsets.symmetric(
                          vertical: vars.spacing2,
                          horizontal: vars.spacing25,
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          mainAxisSize: MainAxisSize.min,
                          children: widget.children,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

/// A labelled run of rows.
///
/// A sidebar breaks its rows into groups rather than stacking them; the gap
/// between groups does most of the work and the label only names it.
class SidebarGroup extends StatelessWidget {
  const SidebarGroup({super.key, this.label, required this.children});

  final String? label;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    // `.dz-sidebar__group`'s block padding, which is the gap between two
    // groups; the label sits on it with only its own padding, and the nav
    // gap is between rows, never between the label and the first of them.
    return Padding(
      padding: EdgeInsets.symmetric(vertical: vars.spacing15),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        mainAxisSize: MainAxisSize.min,
        children: [
          if (label != null)
            Padding(
              padding: EdgeInsets.symmetric(
                horizontal: vars.spacing25,
                vertical: vars.spacing1,
              ),
              child: SectionLabel(label!, tone: SectionLabelTone.faint),
            ),
          Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.frameNavGap,
            children: children,
          ),
        ],
      ),
    );
  }
}

/// The card pinned to the foot of a sidebar — a count, a version, a queue.
class SidebarCard extends StatelessWidget {
  const SidebarCard({super.key, this.label, required this.children});

  final String? label;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      padding: EdgeInsets.all(vars.spacing3),
      decoration: BoxDecoration(
        color: vars.colorSurface,
        border: Border.all(
          color: vars.colorBorder,
          width: context.hairlineWidth,
        ),
        borderRadius: BorderRadius.circular(vars.radiusLarge),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing2,
        children: [
          if (label != null) SectionLabel(label!),
          ...children,
        ],
      ),
    );
  }
}

/// A count at the trailing edge of a sidebar menu row.
///
/// It inherits the row's ink, including its selected and hovered states.
class SidebarMenuBadge extends StatelessWidget {
  const SidebarMenuBadge({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    return Container(
      constraints: BoxConstraints(minWidth: vars.spacing5),
      height: vars.spacing5,
      padding: EdgeInsets.symmetric(horizontal: vars.spacing1),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(vars.radiusSmall),
      ),
      child: DefaultTextStyle.merge(
        style: vars.captionSmall.copyWith(
          color: DefaultTextStyle.of(context).style.color,
          fontWeight: vars.labelMedium.fontWeight,
          height: vars.spacing5 / vars.captionSmall.fontSize!,
        ),
        child: Center(widthFactor: 1, child: child),
      ),
    );
  }
}
