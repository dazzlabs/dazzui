import 'package:flutter/rendering.dart';
import 'package:flutter/widgets.dart';

/// Routes presses in an inline outset through otherwise bounded ancestors.
/// The theme supplies this at its root, where the host's own clips still apply.
class OutsetHitTestRegion extends SingleChildRenderObjectWidget {
  const OutsetHitTestRegion({super.key, required super.child});

  @override
  RenderOutsetHitTestRegion createRenderObject(BuildContext context) =>
      RenderOutsetHitTestRegion();
}

class RenderOutsetHitTestRegion extends RenderProxyBox {
  final Set<RenderInlineOutset> _outsets = {};

  bool _inFront(RenderObject hit, RenderInlineOutset target) {
    List<RenderObject> path(RenderObject object) {
      final List<RenderObject> result = [];
      for (
        RenderObject? node = object;
        node != this && node != null;
        node = node.parent
      ) {
        result.add(node);
      }
      return result.reversed.toList();
    }

    final a = path(hit), b = path(target);
    int i = 0;
    while (i < a.length && i < b.length && identical(a[i], b[i])) {
      i++;
    }
    // A background hit on an ancestor is behind its own overflowing child.
    if (i == a.length) return false;
    if (i == b.length) return true;
    final RenderObject common = i == 0 ? this : a[i - 1];
    if (common is! RenderStack &&
        common is! RenderFlex &&
        common is! RenderCustomMultiChildLayoutBox) {
      return true; // Preserve the normal result for custom paint orders.
    }
    final List<RenderObject> children = [];
    common.visitChildren(children.add);
    return children.indexOf(a[i]) > children.indexOf(b[i]);
  }

  bool _reachable(RenderInlineOutset target, Offset position) {
    RenderObject child = target;
    for (
      RenderObject? parent = child.parent;
      parent != this;
      parent = child.parent
    ) {
      if (parent == null) {
        return false;
      }
      if (parent is RenderOffstage && parent.offstage ||
          parent is RenderIgnorePointer && parent.ignoring ||
          parent is RenderAbsorbPointer && parent.absorbing) {
        return false;
      }
      if (parent is RenderBox) {
        final Matrix4 inverse = Matrix4.inverted(parent.getTransformTo(this));
        final Offset local = MatrixUtils.transformPoint(inverse, position);
        final Rect? clip = parent.describeApproximatePaintClip(child);
        if (clip != null && !clip.contains(local)) {
          return false;
        }
        if (parent is RenderClipRRect && parent.clipBehavior != Clip.none) {
          final RRect shape =
              parent.clipper?.getClip(parent.size) ??
              parent.borderRadius
                  .resolve(parent.textDirection ?? TextDirection.ltr)
                  .toRRect(Offset.zero & parent.size);
          if (!shape.contains(local)) {
            return false;
          }
        }
        if (parent is RenderClipPath &&
            parent.clipBehavior != Clip.none &&
            !(parent.clipper?.getClip(parent.size) ??
                    (Path()..addRect(Offset.zero & parent.size)))
                .contains(local)) {
          return false;
        }
      }
      child = parent;
    }
    return true;
  }

  @override
  bool hitTestChildren(BoxHitTestResult result, {required Offset position}) {
    BoxHitTestResult? normal;
    for (final RenderInlineOutset target in _outsets.toList().reversed) {
      if (!target.attached || !target.hasSize) {
        continue;
      }
      final Matrix4 transform = target.getTransformTo(this);
      final Offset local = MatrixUtils.transformPoint(
        Matrix4.inverted(transform),
        position,
      );
      // The normal path handles the row's allocated space. Only the negative
      // margin needs to bypass the intermediate boxes' rectangular bounds.
      if ((Offset.zero & target.size).contains(local) ||
          !target.paintBounds.contains(local) ||
          !_reachable(target, position)) {
        continue;
      }
      if (normal == null) {
        normal = BoxHitTestResult();
        super.hitTestChildren(normal, position: position);
      }
      if (normal.path.isNotEmpty &&
          normal.path.first.target is RenderObject &&
          _inFront(normal.path.first.target as RenderObject, target)) {
        continue;
      }
      if (!result.addWithPaintTransform(
        transform: transform,
        position: position,
        hitTest: (result, transformed) =>
            target.hitTest(result, position: transformed),
      )) {
        continue;
      }
      // Keep bubbling through scroll and gesture ancestors, with each entry's
      // real transform rather than dispatching a synthetic tap to the row.
      for (
        RenderObject? parent = target.parent;
        parent != this;
        parent = parent?.parent
      ) {
        if (parent is RenderBox) {
          final RenderBox box = parent;
          result.addWithPaintTransform(
            transform: parent.getTransformTo(this),
            position: position,
            hitTest: (result, transformed) {
              result.add(BoxHitTestEntry(box, transformed));
              return true;
            },
          );
        }
      }
      return true;
    }
    return super.hitTestChildren(result, position: position);
  }
}

/// CSS's negative inline margin: allocate the original width, but size,
/// paint, focus and hit-test the child at its wider border box.
class InlineOutset extends SingleChildRenderObjectWidget {
  const InlineOutset({
    super.key,
    required this.amount,
    required this.radius,
    required super.child,
  });
  final double amount;
  final BorderRadius radius;

  @override
  RenderInlineOutset createRenderObject(BuildContext context) =>
      RenderInlineOutset(amount, radius);
  @override
  void updateRenderObject(
    BuildContext context,
    RenderInlineOutset renderObject,
  ) {
    renderObject
      ..amount = amount
      ..radius = radius;
  }
}

class RenderInlineOutset extends RenderShiftedBox {
  RenderInlineOutset(this._amount, this._radius) : super(null);
  double _amount;
  BorderRadius _radius;
  final List<RenderOutsetHitTestRegion> _regions = [];
  set amount(double value) {
    if (_amount == value) return;
    _amount = value;
    markNeedsLayout();
  }

  set radius(BorderRadius value) {
    if (_radius == value) return;
    _radius = value;
    markNeedsPaint();
  }

  BoxConstraints _inner(BoxConstraints c) => c.copyWith(
    minWidth: c.minWidth + 2 * _amount,
    maxWidth: c.maxWidth + 2 * _amount,
  );
  @override
  Size computeDryLayout(BoxConstraints constraints) {
    final Size inner = child?.getDryLayout(_inner(constraints)) ?? Size.zero;
    return constraints.constrain(
      Size((inner.width - 2 * _amount).clamp(0, double.infinity), inner.height),
    );
  }

  @override
  void performLayout() {
    if (child == null) {
      size = constraints.smallest;
      return;
    }
    child!.layout(_inner(constraints), parentUsesSize: true);
    size = constraints.constrain(
      Size(
        (child!.size.width - 2 * _amount).clamp(0, double.infinity),
        child!.size.height,
      ),
    );
    (child!.parentData! as BoxParentData).offset = Offset(-_amount, 0);
  }

  @override
  Rect get paintBounds => child == null
      ? super.paintBounds
      : child!.paintBounds
            .shift(Offset(-_amount, 0))
            .expandToInclude(Offset.zero & size);
  @override
  Rect get semanticBounds => child == null
      ? super.semanticBounds
      : child!.semanticBounds.shift(Offset(-_amount, 0));
  @override
  bool hitTest(BoxHitTestResult result, {required Offset position}) {
    if (child == null ||
        !_radius
            .toRRect(
              Rect.fromLTWH(-_amount, 0, child!.size.width, child!.size.height),
            )
            .scaleRadii()
            .contains(position)) {
      return false;
    }
    if (!hitTestChildren(result, position: position)) return false;
    result.add(BoxHitTestEntry(this, position));
    return true;
  }

  @override
  void attach(PipelineOwner owner) {
    super.attach(owner);
    for (
      RenderObject? parent = this.parent;
      parent != null;
      parent = parent.parent
    ) {
      if (parent is RenderOutsetHitTestRegion) {
        parent._outsets.add(this);
        _regions.add(parent);
      }
    }
  }

  @override
  void detach() {
    for (final region in _regions) {
      region._outsets.remove(this);
    }
    _regions.clear();
    super.detach();
  }
}
