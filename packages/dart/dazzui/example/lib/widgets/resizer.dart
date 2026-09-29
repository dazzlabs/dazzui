import 'package:dazzui/dazzui.dart';
import 'package:flutter/services.dart';

/// How wide a column may be, and how wide it starts.
class ColumnWidth {
  const ColumnWidth({
    required this.initial,
    required this.min,
    required this.max,
  });

  final double initial;
  final double min;
  final double max;

  double clamp(double width) => width.clamp(min, max).roundToDouble();
}

/// The edge of a column, to be dragged. It draws nothing until the pointer
/// is on it — the rule is already there, drawn by the column — and then the
/// accent line; the arrow keys step it, and a double-click resets it.
class Resizer extends StatefulWidget {
  const Resizer({
    super.key,
    required this.outward,
    required this.column,
    required this.width,
    required this.onChanged,
    required this.label,
    this.lineAtStart = false,
  });

  /// `1` when dragging right widens the column, `-1` when dragging left does.
  final int outward;
  final ColumnWidth column;
  final double width;
  final ValueChanged<double> onChanged;
  final String label;

  /// Draws the line on the handle's leading edge rather than its middle —
  /// for the handle that stands inside the window, over the column's rule.
  final bool lineAtStart;

  @override
  State<Resizer> createState() => _ResizerState();
}

class _ResizerState extends State<Resizer> {
  bool _hovered = false;
  bool _dragging = false;
  bool _focused = false;
  double _start = 0;
  double _from = 0;

  void _set(double width) => widget.onChanged(widget.column.clamp(width));

  KeyEventResult _onKey(FocusNode node, KeyEvent event) {
    if (event is KeyUpEvent) return KeyEventResult.ignored;
    final double step = HardwareKeyboard.instance.isShiftPressed ? 32 : 8;
    final LogicalKeyboardKey key = event.logicalKey;
    if (key == LogicalKeyboardKey.arrowRight) {
      _set(widget.width + step * widget.outward);
    } else if (key == LogicalKeyboardKey.arrowLeft) {
      _set(widget.width - step * widget.outward);
    } else if (key == LogicalKeyboardKey.home) {
      _set(widget.column.min);
    } else if (key == LogicalKeyboardKey.end) {
      _set(widget.column.max);
    } else {
      return KeyEventResult.ignored;
    }
    return KeyEventResult.handled;
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final bool lit = _hovered || _dragging || _focused;
    return Semantics(
      label: widget.label,
      value: '${widget.width.round()}',
      slider: true,
      child: Focus(
        onKeyEvent: _onKey,
        onFocusChange: (focused) => setState(() => _focused = focused),
        child: MouseRegion(
          cursor: SystemMouseCursors.resizeColumn,
          onEnter: (_) => setState(() => _hovered = true),
          onExit: (_) => setState(() => _hovered = false),
          child: GestureDetector(
            behavior: HitTestBehavior.opaque,
            onDoubleTap: () => _set(widget.column.initial),
            onHorizontalDragStart: (details) => setState(() {
              _dragging = true;
              _start = details.globalPosition.dx;
              _from = widget.width;
            }),
            onHorizontalDragUpdate: (details) => _set(
              _from + (details.globalPosition.dx - _start) * widget.outward,
            ),
            onHorizontalDragEnd: (_) => setState(() => _dragging = false),
            child: SizedBox(
              width: vars.spacing2,
              child: AnimatedOpacity(
                opacity: lit ? 1 : 0,
                duration: vars.motionDuration,
                curve: vars.motionEasing,
                child: Align(
                  alignment: widget.lineAtStart
                      ? AlignmentDirectional.centerStart
                      : Alignment.center,
                  child: Container(
                    width: vars.spacing05,
                    color: vars.colorPrimary[500],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
