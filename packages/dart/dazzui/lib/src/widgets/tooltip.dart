import 'dart:async';

import 'package:flutter/widgets.dart';

import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'popover.dart';

/// The name of a thing that has no room to say it: a phrase that appears
/// beside [child] while the pointer rests on it or focus is in it.
///
/// It is never reached into — the panel takes no pointer — so anything a
/// reader might want to press belongs in a [PreviewCard] instead.
class Tooltip extends StatefulWidget {
  const Tooltip({
    super.key,
    required this.label,
    required this.child,
    this.side = PopoverSide.top,
    this.align = PopoverAlign.center,
    this.delay = const Duration(milliseconds: 600),
    this.closeDelay = Duration.zero,
    this.enabled = true,
  });

  final String label;
  final Widget child;
  final PopoverSide side;
  final PopoverAlign align;

  /// How long the pointer rests before the label appears. Focus shows it at
  /// once: a keyboard reader has already said where they are.
  final Duration delay;
  final Duration closeDelay;
  final bool enabled;

  @override
  State<Tooltip> createState() => _TooltipState();
}

class _TooltipState extends State<Tooltip> {
  final OverlayPortalController _controller = OverlayPortalController();
  final LayerLink _link = LayerLink();
  Timer? _timer;

  void _schedule(Duration after, VoidCallback run) {
    _timer?.cancel();
    if (after == Duration.zero) {
      run();
    } else {
      _timer = Timer(after, run);
    }
  }

  void _show([Duration? after]) {
    if (!widget.enabled) return;
    _schedule(after ?? Duration.zero, () {
      if (mounted && !_controller.isShowing) _controller.show();
    });
  }

  void _hide() {
    _schedule(widget.closeDelay, () {
      if (mounted && _controller.isShowing) _controller.hide();
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return OverlayPortal(
      controller: _controller,
      overlayChildBuilder: (context) {
        final ThemeVariables vars = Theme.of(context).vars;
        return PopoverOverlay(
          link: _link,
          side: widget.side,
          align: widget.align,
          gap: vars.spacing1,
          onDismiss: _hide,
          dismissOnOutsidePress: false,
          child: IgnorePointer(child: _TooltipPanel(label: widget.label)),
        );
      },
      child: CompositedTransformTarget(
        link: _link,
        child: Semantics(
          tooltip: widget.label,
          child: Focus(
            canRequestFocus: false,
            skipTraversal: true,
            onFocusChange: (focused) => focused ? _show() : _hide(),
            child: MouseRegion(
              onEnter: (_) => _show(widget.delay),
              onExit: (_) => _hide(),
              child: Listener(
                behavior: HitTestBehavior.translucent,
                onPointerDown: (_) => _hide(),
                child: widget.child,
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _TooltipPanel extends StatelessWidget {
  const _TooltipPanel({required this.label});

  final String label;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    return ConstrainedBox(
      constraints: BoxConstraints(maxWidth: vars.toastMaxWidth),
      child: DecoratedBox(
        decoration: BoxDecoration(
          color: vars.colorSurfaceRaised,
          border: Border.all(
            color: vars.colorBorder,
            width: context.hairlineWidth,
          ),
          borderRadius: BorderRadius.circular(vars.radiusSmall),
          boxShadow: vars.shadowSm,
        ),
        child: Padding(
          padding: EdgeInsets.symmetric(
            vertical: vars.spacing1 + vars.spacing05,
            horizontal: vars.spacing2,
          ),
          child: Text(
            label,
            style: vars.labelSmall.copyWith(color: vars.colorContent),
          ),
        ),
      ),
    );
  }
}
