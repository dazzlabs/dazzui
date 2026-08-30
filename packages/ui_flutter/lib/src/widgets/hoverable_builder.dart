import 'package:flutter/widgets.dart';

/// A builder that changes its appearance when hovered.
class HoverableBuilder extends StatefulWidget {
  const HoverableBuilder({
    super.key,
    required this.builder,
    this.child,
  });

  /// The builder that builds the widget.
  final ValueWidgetBuilder<bool> builder;

  /// The child widget to pass to the [builder].
  final Widget? child;

  @override
  State<HoverableBuilder> createState() => _HoverableBuilderState();
}

class _HoverableBuilderState extends State<HoverableBuilder> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (event) {
        _isHovered = true;
        setState(() {});
      },
      onExit: (event) {
        _isHovered = false;
        setState(() {});
      },
      onHover: (event) {
        if (!_isHovered) {
          _isHovered = true;
          setState(() {});
        }
      },
      child: widget.builder(context, _isHovered, widget.child),
    );
  }
}
