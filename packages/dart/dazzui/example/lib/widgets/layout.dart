import 'dart:math' as math;

import 'package:dazzui/dazzui.dart';
import 'package:flutter/rendering.dart';

/*
 * The two web layouts Flutter has no widget for that does not measure its
 * children twice through intrinsics — which a `LayoutBuilder` anywhere
 * below cannot answer. Each lays its children out once to learn their size,
 * then once more at the size they share.
 */

/// Children side by side, every one as tall as the tallest: a flex row's
/// `align-items: stretch`. With [columns] the width is shared equally, as a
/// grid track is; without, each child keeps its own width.
class StretchRow extends MultiChildRenderObjectWidget {
  const StretchRow({
    super.key,
    this.gap = 0,
    this.columns,
    required super.children,
  });

  final double gap;
  final int? columns;

  @override
  RenderObject createRenderObject(BuildContext context) =>
      _RenderStretch(axis: Axis.horizontal, gap: gap, columns: columns);

  @override
  void updateRenderObject(BuildContext context, RenderObject renderObject) {
    final _RenderStretch render = renderObject as _RenderStretch;
    render
      ..gap = gap
      ..columns = columns;
  }
}

/// Children stacked, every one as wide as the widest: a shrink-to-fit
/// column's `align-items: stretch`.
class StretchColumn extends MultiChildRenderObjectWidget {
  const StretchColumn({super.key, this.gap = 0, required super.children});

  final double gap;

  @override
  RenderObject createRenderObject(BuildContext context) =>
      _RenderStretch(axis: Axis.vertical, gap: gap);

  @override
  void updateRenderObject(BuildContext context, RenderObject renderObject) {
    final _RenderStretch render = renderObject as _RenderStretch;
    render.gap = gap;
  }
}

class _StretchData extends ContainerBoxParentData<RenderBox> {}

class _RenderStretch extends RenderBox
    with
        ContainerRenderObjectMixin<RenderBox, _StretchData>,
        RenderBoxContainerDefaultsMixin<RenderBox, _StretchData> {
  _RenderStretch({required this.axis, required double gap, int? columns})
    : _gap = gap,
      _columns = columns;

  final Axis axis;

  double _gap;
  set gap(double value) {
    if (value == _gap) return;
    _gap = value;
    markNeedsLayout();
  }

  int? _columns;
  set columns(int? value) {
    if (value == _columns) return;
    _columns = value;
    markNeedsLayout();
  }

  @override
  void setupParentData(RenderBox child) {
    if (child.parentData is! _StretchData) child.parentData = _StretchData();
  }

  @override
  void performLayout() {
    final List<RenderBox> children = getChildrenAsList();
    if (children.isEmpty) {
      size = constraints.smallest;
      return;
    }
    final bool row = axis == Axis.horizontal;
    final int? columns = _columns;
    final double? track = row && columns != null && constraints.hasBoundedWidth
        ? (constraints.maxWidth - _gap * (columns - 1)) / columns
        : null;

    // First pass: each child at the size it wants.
    for (final RenderBox child in children) {
      child.layout(
        row
            ? BoxConstraints(
                minWidth: track ?? 0,
                maxWidth: track ?? constraints.maxWidth,
                maxHeight: constraints.maxHeight,
              )
            : BoxConstraints(maxWidth: constraints.maxWidth),
        parentUsesSize: true,
      );
    }
    final double shared = children.fold(
      0,
      (most, child) =>
          math.max(most, row ? child.size.height : child.size.width),
    );

    // Second pass: each at the shared cross size.
    double offset = 0;
    for (final RenderBox child in children) {
      child.layout(
        row
            ? BoxConstraints.tightFor(width: child.size.width, height: shared)
            : BoxConstraints.tightFor(width: shared),
        parentUsesSize: true,
      );
      (child.parentData! as _StretchData).offset = row
          ? Offset(offset, 0)
          : Offset(0, offset);
      offset += (row ? child.size.width : child.size.height) + _gap;
    }
    final double main = offset - _gap;
    size = constraints.constrain(
      row
          ? Size(track != null ? constraints.maxWidth : main, shared)
          : Size(shared, main),
    );
  }

  @override
  bool hitTestChildren(BoxHitTestResult result, {required Offset position}) =>
      defaultHitTestChildren(result, position: position);

  @override
  void paint(PaintingContext context, Offset offset) =>
      defaultPaint(context, offset);
}
