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
class Sidebar extends StatelessWidget {
  const Sidebar({
    super.key,
    this.header,
    this.width,
    required this.children,
  });

  /// Content for the strip above the row list, held at exactly the titlebar's
  /// height so it lines up with the toolbar in the pane beside it.
  final Widget? header;

  /// An explicit width. Leave it out and the column takes the token's.
  final double? width;

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Container(
      width: width ?? vars.frameSidebarWidth,
      decoration: BoxDecoration(
        color: vars.colorSurfaceColumn,
        border: BorderDirectional(
          end: BorderSide(
            color: vars.colorBorder,
            width: context.hairlineWidth,
          ),
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          if (header != null)
            Container(
              height: vars.frameTitlebarSize,
              alignment: AlignmentDirectional.centerStart,
              padding: EdgeInsets.symmetric(horizontal: vars.spacing4),
              child: header!,
            ),
          // Only the row list scrolls, so a pinned card stays put. The
          // groups carry their own block padding, as `.dz-sidebar__group`
          // does, so two of them stand a full step apart while the column
          // itself adds only `.dz-sidebar__content`'s.
          Expanded(
            child: SingleChildScrollView(
              padding: EdgeInsets.symmetric(
                vertical: vars.spacing2,
                horizontal: vars.spacing25,
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                mainAxisSize: MainAxisSize.min,
                children: children,
              ),
            ),
          ),
        ],
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
