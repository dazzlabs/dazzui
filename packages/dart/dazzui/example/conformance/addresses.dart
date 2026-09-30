import 'package:dazzui_example/lib/story.dart';

/*
 * The walk that addresses a scene's specimens, as tools/codegen/scenes.mjs
 * defines it: pre-order — an element, then the elements among its props in
 * the order the story wrote them, then its children — at the scene's
 * initial state. The maps it yields are the story's own, the ones the
 * scene's `SceneComponent`s were built from, so a map leads to the widget
 * that drew it.
 */

const _stage = {'Row', 'Col', 'Frame', 'Text'};

bool _truthy(Object? value) =>
    !(value == null || value == false || value == 0 || value == '');

Object? _resolve(Object? value, Map<String, Object?> state) {
  if (value is List) return [for (final item in value) _resolve(item, state)];
  if (value is! Map) return value;
  if (value.containsKey(r'$state')) {
    return _resolve(state[value[r'$state']], state);
  }
  if (value.containsKey(r'$arg')) return null;
  if (value.containsKey(r'$not')) {
    return !_truthy(_resolve(value[r'$not'], state));
  }
  if (value.containsKey(r'$eq')) {
    final List<Object?> pair = value[r'$eq'] as List<Object?>;
    return _resolve(pair[0], state) == _resolve(pair[1], state);
  }
  if (value.containsKey(r'$if')) {
    final List<Object?> branches = value[r'$if'] as List<Object?>;
    return _resolve(
      _truthy(_resolve(branches[0], state)) ? branches[1] : branches[2],
      state,
    );
  }
  if (value.containsKey(r'$concat')) {
    return (value[r'$concat'] as List<Object?>)
        .map((part) => '${_resolve(part, state)}')
        .join();
  }
  return value;
}

List<Object?> _nodes(Object? value, Map<String, Object?> state) {
  final Object? resolved = _resolve(value, state);
  if (resolved is List) {
    return [for (final item in resolved) ..._nodes(item, state)];
  }
  if (resolved == null || resolved == false || resolved == true) return [];
  return [resolved];
}

/// Every specimen's node in [scene], in address order, with its component.
List<Map<Object?, Object?>> specimensOf(Scene scene) {
  final List<Map<Object?, Object?>> found = [];

  void visit(Object? node) {
    if (node is! Map || !node.containsKey(r'$el')) return;
    if (!_stage.contains(node[r'$el'])) found.add(node);
    void inProps(Object? value) {
      final Object? resolved = _resolve(value, scene.state);
      if (resolved is List) return resolved.forEach(inProps);
      if (resolved is! Map) return;
      if (resolved.containsKey(r'$el')) return visit(resolved);
      if (resolved.containsKey(r'$do') || resolved.containsKey(r'$icon')) {
        return;
      }
      resolved.values.forEach(inProps);
    }

    final Map<Object?, Object?> props =
        (node['props'] as Map<Object?, Object?>?) ?? const {};
    props.values.forEach(inProps);
    _nodes(node['children'] ?? const [], scene.state).forEach(visit);
  }

  _nodes(scene.content, scene.state).forEach(visit);
  return found;
}

/// `SegmentedControl` → `segmented-control`, as scenes.json names it.
String kebab(String name) => name
    .replaceAllMapped(
      RegExp('([a-z0-9])([A-Z])'),
      (m) => '${m[1]}-${m[2]}',
    )
    .toLowerCase();
