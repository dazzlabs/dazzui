import 'package:flutter/rendering.dart';
import 'package:flutter/widgets.dart';

import '../foundation/inline_outset.dart';
import '../foundation/widget_tint.dart';
import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'divider.dart';
import 'pressable.dart';
import 'section_label.dart';

bool _hasWords(Object? value) => value != null && value != '';
Widget _words(Object value, TextStyle style, {bool truncate = false}) =>
    value is Widget
    ? DefaultTextStyle.merge(
        style: style,
        maxLines: truncate ? 1 : null,
        overflow: truncate ? TextOverflow.ellipsis : null,
        child: value,
      )
    : Text(
        value as String,
        style: style,
        maxLines: truncate ? 1 : null,
        overflow: truncate ? TextOverflow.ellipsis : null,
      );

/// The settings column.
///
/// The spacing is the whole design here. Rows sit close together, a heading
/// sits further from the section above it than from its own rows, a group
/// title stands further still, and one group stands furthest from the next —
/// so a heading always reads as belonging to what follows it rather than
/// floating between two blocks. The ladder is decided in ds — one
/// `preferences*Gap` per level, and the pad a row's text wears — and nothing
/// here restates a step.

/// A run of children at one step, with a rule drawn in the run rather than
/// added to it.
///
/// A `Column`'s own `spacing` would put the whole step on each side of a
/// [Divider], and two sections with a rule between them would then stand
/// further apart than two groups. So the run is laid by hand: half the step
/// on either side of a rule, the whole step everywhere else.
List<Widget> _run(List<Widget> children, double step) {
  final List<Widget> out = [];
  for (int i = 0; i < children.length; i++) {
    if (i > 0) {
      final bool rule = children[i - 1] is Divider || children[i] is Divider;
      out.add(SizedBox(height: rule ? step / 2 : step));
    }
    out.add(children[i]);
  }
  return out;
}

/// The settings column, and the root the rest of this file sits in.
///
/// It owns the two things no group or section can: the run between groups,
/// which is the widest step of the spacing ladder this column climbs, and the
/// width. The column is capped rather than filling its pane — a row puts its
/// label on one edge and its control on the other, and past
/// [ThemeVariables.preferencesWidth] the two stop reading as one line.
class Preferences extends StatelessWidget {
  const Preferences({super.key, required this.children});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    // Full width up to the cap, so it fills a narrow pane and stops in a wide
    // one: `width: 100%; max-width` in the stylesheet. The inner box is what
    // makes it the former — under a loose constraint alone a centred column
    // would shrink to its rows.
    return Align(
      alignment: AlignmentDirectional.topStart,
      widthFactor: 1,
      heightFactor: 1,
      child: ConstrainedBox(
        constraints: BoxConstraints(maxWidth: vars.preferencesWidth),
        child: SizedBox(
          width: double.infinity,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            mainAxisSize: MainAxisSize.min,
            children: _run(children, vars.preferencesGroupGap),
          ),
        ),
      ),
    );
  }
}

/// A group of sections, with a title that outranks them.
class PreferenceGroup extends StatelessWidget {
  const PreferenceGroup({
    super.key,
    required this.title,
    this.description,
    this.action,
    required this.children,
  }) : assert(title is String || title is Widget),
       assert(
         description == null || description is String || description is Widget,
       );

  final Object title;
  final Object? description;
  final Widget? action;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisSize: MainAxisSize.min,
      children: [
        Padding(
          padding: EdgeInsets.only(bottom: vars.preferencesTitleGap),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            spacing: vars.spacing4,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.min,
                  // Half a step: the title's leading and the description's
                  // already open the pair.
                  spacing: vars.spacing05,
                  children: [
                    // The group outranks its sections typographically, and
                    // it has to outrank the rows by more than a weight: a
                    // row's title is the same 12px, so a title one grade
                    // heavier read as one more row. `titleMedium` — two
                    // sizes up, at the label weight — is the first face that
                    // stands over a row. A section heading stays a
                    // SectionLabel.
                    _words(
                      title,
                      vars.titleMedium.copyWith(
                        color: vars.colorContent,
                      ),
                    ),
                    if (_hasWords(description))
                      _words(
                        description!,
                        vars.captionSmall.copyWith(
                          height: 1.7,
                          color: vars.colorContentSubtle,
                        ),
                      ),
                  ],
                ),
              ),
              if (action != null) _HeadingAction(child: action!),
            ],
          ),
        ),
        Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisSize: MainAxisSize.min,
          children: _run(children, vars.preferencesSectionGap),
        ),
      ],
    );
  }
}

/// A labelled run of rows.
class PreferenceSection extends StatelessWidget {
  const PreferenceSection({
    super.key,
    this.label,
    this.footer,
    this.action,
    required this.children,
  }) : assert(label == null || label is String || label is Widget),
       assert(footer == null || footer is String || footer is Widget);

  final Object? label;
  final Object? footer;
  final Widget? action;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisSize: MainAxisSize.min,
      children: [
        if (_hasWords(label) || action != null)
          Padding(
            padding: EdgeInsets.only(bottom: vars.preferencesHeadingGap),
            child: Row(
              spacing: vars.spacing4,
              children: [
                Expanded(
                  child: !_hasWords(label)
                      ? const SizedBox.shrink()
                      : label is String
                      ? SectionLabel(label! as String)
                      : _words(
                          label!,
                          vars.labelSmall.copyWith(
                            color: vars.colorContentSubtle,
                          ),
                        ),
                ),
                if (action != null) _HeadingAction(child: action!),
              ],
            ),
          ),
        Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisSize: MainAxisSize.min,
          spacing: vars.preferencesRowGap,
          children: children,
        ),
        if (_hasWords(footer))
          Padding(
            padding: EdgeInsets.only(top: vars.preferencesFooterGap),
            child: _words(
              footer!,
              vars.captionSmall.copyWith(
                height: 1.7,
                color: vars.colorContentSubtle,
              ),
            ),
          ),
      ],
    );
  }
}

/// A heading's control sits in a zero-height slot it overhangs on both sides.
///
/// Letting a full-height control size the line would make a heading with a
/// button taller than one without, and two sections on the same page would
/// then start at different heights. Zero rather than a negative margin, so it
/// holds for any control height and any heading size.
///
/// This is `height: 0` on a flex item, and it takes a render object rather
/// than a composition because nothing off the shelf does both halves of it.
/// An `OverflowBox` sizes itself to the constraints it is handed, and a
/// `Row` hands a non-flex child an unbounded main axis — so the slot took an
/// infinite width and every heading with an action crashed. And a zero-height
/// box drops every press on what overhangs it, because `RenderBox.hitTest`
/// checks its own bounds before it reaches the child.
class _HeadingAction extends SingleChildRenderObjectWidget {
  const _HeadingAction({required Widget child}) : super(child: child);

  @override
  RenderObject createRenderObject(BuildContext context) {
    return _RenderHeadingAction();
  }
}

class _RenderHeadingAction extends RenderShiftedBox {
  _RenderHeadingAction() : super(null);

  /// The child takes the slot's width constraints and none of its height: it
  /// is the one thing here allowed to be as tall as it likes.
  BoxConstraints _innerConstraints(BoxConstraints constraints) {
    return BoxConstraints(
      minWidth: constraints.minWidth,
      maxWidth: constraints.maxWidth,
    );
  }

  @override
  double computeMinIntrinsicWidth(double height) =>
      child?.getMinIntrinsicWidth(double.infinity) ?? 0;

  @override
  double computeMaxIntrinsicWidth(double height) =>
      child?.getMaxIntrinsicWidth(double.infinity) ?? 0;

  @override
  double computeMinIntrinsicHeight(double width) => 0;

  @override
  double computeMaxIntrinsicHeight(double width) => 0;

  @override
  Size computeDryLayout(BoxConstraints constraints) {
    final RenderBox? child = this.child;
    if (child == null) return constraints.smallest;
    final Size childSize = child.getDryLayout(_innerConstraints(constraints));
    return constraints.constrain(Size(childSize.width, 0));
  }

  @override
  void performLayout() {
    final RenderBox? child = this.child;
    if (child == null) {
      size = constraints.smallest;
      return;
    }
    child.layout(_innerConstraints(constraints), parentUsesSize: true);
    size = constraints.constrain(Size(child.size.width, 0));
    (child.parentData! as BoxParentData).offset = Offset(
      0,
      -child.size.height / 2,
    );
  }

  @override
  bool hitTest(BoxHitTestResult result, {required Offset position}) {
    // Skipping the usual bounds check on purpose: the whole control lies
    // outside a box with no height, so testing this box first would mean a
    // heading's action could never be pressed.
    if (hitTestChildren(result, position: position)) {
      result.add(BoxHitTestEntry(this, position));
      return true;
    }
    return false;
  }
}

/// The ramp an opening row previews on hover.
enum PreferenceRowTint with WidgetTint {
  primary,
  neutral,
  info,
  success,
  warning,
  danger,
}

/// One decision. Text sets the minimum height; an opening row owns its
/// expanded border box, hover wash, focus ring and button semantics.
class PreferenceRow extends StatelessWidget {
  const PreferenceRow({
    super.key,
    required this.title,
    this.subtitle,
    this.icon,
    this.trailing,
    this.onOpen,
    this.tint = PreferenceRowTint.neutral,
    @Deprecated('Use onOpen, as in the React API.') this.onPressed,
  }) : assert(onOpen == null || onPressed == null),
       assert(title is String || title is Widget),
       assert(subtitle == null || subtitle is String || subtitle is Widget);

  final Object title;
  final Object? subtitle;
  final Widget? icon;
  final Widget? trailing;
  final VoidCallback? onOpen;
  final PreferenceRowTint tint;
  @Deprecated('Use onOpen, as in the React API.')
  final VoidCallback? onPressed;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final VoidCallback? open = onOpen ?? onPressed;
    final bool interactive = open != null;

    Widget content(Set<WidgetState> states) {
      final bool hovered = states.contains(WidgetState.hovered);

      final Widget row = Row(
        spacing: vars.spacing25,
        children: [
          // Uncoloured on purpose: a Badge or a Switch in these slots keeps
          // its own ink rather than inheriting a wash.
          if (icon != null)
            IconTheme.merge(
              data: IconThemeData(size: vars.iconMedium),
              child: icon!,
            ),
          Expanded(
            child: Padding(
              // The pad rides on the text, so a row that grows a subtitle
              // keeps air above the title and below it. A single line padded
              // this way is shorter than the control height, and the row's
              // minimum still centres it.
              padding: EdgeInsets.symmetric(
                vertical: vars.preferencesRowPadding,
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                spacing: vars.spacing1,
                children: [
                  // A row rests at `labelQuiet`, the way a list row does:
                  // every title at the label weight was a column of bold
                  // with nothing left to outrank it.
                  _words(
                    title,
                    vars.labelQuiet.copyWith(
                      color: vars.colorContent,
                    ),
                    truncate: true,
                  ),
                  if (_hasWords(subtitle))
                    _words(
                      subtitle!,
                      vars.captionSmall.copyWith(
                        color: vars.colorContentSubtle,
                      ),
                      truncate: true,
                    ),
                ],
              ),
            ),
          ),
          if (trailing != null || interactive)
            Row(
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing25,
              children: [
                ?trailing,
                if (interactive)
                  ExcludeSemantics(
                    child: Text(
                      '›',
                      style: vars.bodyMedium.copyWith(color: vars.colorContent),
                    ),
                  ),
              ],
            ),
        ],
      );

      if (!interactive) {
        return ConstrainedBox(
          constraints: BoxConstraints(minHeight: vars.controlMediumSize),
          child: row,
        );
      }

      return ConstrainedBox(
        constraints: BoxConstraints(minHeight: vars.controlMediumSize),
        child: TweenAnimationBuilder<Color?>(
          duration: vars.motionDuration,
          curve: vars.motionEasing,
          tween: ColorTween(
            end: hovered
                ? vars.controlColor
                      .resolveWith(
                        {},
                        tint: tint,
                      )[vars.controlColorPlainSurface.hoveredShade!]!
                      .withValues(
                        alpha: vars.controlColorPlainSurface.hoveredOpacity,
                      )
                : const Color(0x00000000),
          ),
          builder: (context, color, child) => DecoratedBox(
            decoration: BoxDecoration(
              color: color,
              borderRadius: BorderRadius.circular(vars.controlContainerRadius),
            ),
            child: child,
          ),
          child: Padding(
            padding: EdgeInsets.symmetric(horizontal: vars.spacing2),
            child: row,
          ),
        ),
      );
    }

    if (!interactive) return content(const {});

    return InlineOutset(
      amount: vars.spacing2,
      radius: BorderRadius.circular(vars.controlContainerRadius),
      child: Pressable(
        onPressed: open,
        borderRadius: BorderRadius.circular(vars.controlContainerRadius),
        builder: (context, states) => content(states),
      ),
    );
  }
}
