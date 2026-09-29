import 'package:dazzui/dazzui.dart';
import 'package:flutter/foundation.dart';

import 'glyphs.dart';
import 'mapping.dart';
import 'story.dart';

/// Draws one [Scene]: its slots, held here, and its tree, built from
/// [components].
class SceneView extends StatefulWidget {
  const SceneView({super.key, required this.scene});

  final Scene scene;

  @override
  State<SceneView> createState() => _SceneViewState();
}

class _SceneViewState extends State<SceneView> {
  late Map<String, Object?> _state = Map.of(widget.scene.state);
  int _revision = 0;

  @override
  void didUpdateWidget(SceneView old) {
    super.didUpdateWidget(old);
    if (!identical(old.scene, widget.scene)) {
      _state = Map.of(widget.scene.state);
    }
  }

  void _write(String slot, Object? value) {
    setState(() {
      _state[slot] = value;
      _revision++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return SceneScope._(
      state: _state,
      revision: _revision,
      write: _write,
      child: Builder(
        builder: (context) => buildNode(context, widget.scene.content),
      ),
    );
  }
}

class SceneScope extends InheritedWidget {
  const SceneScope._({
    required this.state,
    required this.revision,
    required this.write,
    required super.child,
  });

  final Map<String, Object?> state;
  final int revision;
  final void Function(String slot, Object? value) write;

  static SceneScope of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<SceneScope>()!;

  /// [child] under the scene [context] is in — for what a scene opens in a
  /// route of its own, a dialog or a drawer, which is outside it.
  static Widget carry(BuildContext context, Widget child) {
    final SceneScope scope = of(context);
    return SceneScope._(
      state: scope.state,
      revision: scope.revision,
      write: scope.write,
      child: child,
    );
  }

  @override
  bool updateShouldNotify(SceneScope old) => old.revision != revision;
}

/// JavaScript's truthiness, which is what a story's tests were written in.
bool truthy(Object? value) =>
    !(value == null || value == false || value == 0 || value == '');

bool _live(Object? value) =>
    value is Map &&
    (value.containsKey(r'$state') ||
        value.containsKey(r'$eq') ||
        value.containsKey(r'$not') ||
        value.containsKey(r'$if') ||
        value.containsKey(r'$concat') ||
        value.containsKey(r'$arg'));

/// Whether [value] is only known while the scene runs — read from a slot.
bool isLive(Object? value) => _live(value);

/// A prop's value now: a slot read, a test worked out, an argument taken.
Object? resolve(
  BuildContext context,
  Object? value, [
  List<Object?> args = const [],
]) {
  if (value is! Map || !_live(value)) return value;
  if (value.containsKey(r'$state')) {
    return SceneScope.of(context).state[value[r'$state']];
  }
  if (value.containsKey(r'$arg')) {
    final int index = value[r'$arg'] as int;
    return index < args.length ? args[index] : null;
  }
  if (value.containsKey(r'$eq')) {
    final List<Object?> pair = value[r'$eq'] as List<Object?>;
    final Object? a = resolve(context, pair[0], args);
    final Object? b = resolve(context, pair[1], args);
    return a is List && b is List ? listEquals(a, b) : a == b;
  }
  if (value.containsKey(r'$not')) {
    return !truthy(resolve(context, value[r'$not'], args));
  }
  if (value.containsKey(r'$if')) {
    final List<Object?> branches = value[r'$if'] as List<Object?>;
    return truthy(resolve(context, branches[0], args))
        ? resolve(context, branches[1], args)
        : resolve(context, branches[2], args);
  }
  final List<Object?> parts = value[r'$concat'] as List<Object?>;
  return parts.map((part) => '${resolve(context, part, args) ?? ''}').join();
}

/// Makes the writes an event declares: slots set, toasts raised.
void run(
  BuildContext context,
  Object? action, [
  List<Object?> args = const [],
]) {
  if (action is! Map || action[r'$do'] is! List) return;
  for (final Object? step in action[r'$do'] as List<Object?>) {
    if (step is! Map) continue;
    if (step.containsKey('set')) {
      SceneScope.of(
        context,
      ).write(step['set'] as String, resolve(context, step['to'], args));
    } else if (step.containsKey('toast')) {
      final Map<Object?, Object?> toast =
          step['toast'] as Map<Object?, Object?>;
      final Map<Object?, Object?>? act =
          toast['action'] as Map<Object?, Object?>?;
      Toaster.of(context).add(
        ToastOptions(
          title: '${resolve(context, toast['title'], args)}',
          tint: ToastTint.values.byName(
            (resolve(context, toast['tint'], args) as String?) ?? 'neutral',
          ),
          action: act == null
              ? null
              : ToastAction(
                  label: '${act['label']}',
                  onPressed: () => run(context, act['onClick']),
                ),
        ),
      );
    }
  }
}

/// Supplies the press to the trigger drawn under it — the button a dialog,
/// a drawer or a menu opens from never declares its own.
class TriggerPress extends InheritedWidget {
  const TriggerPress({
    super.key,
    required this.onPressed,
    required super.child,
  });

  final VoidCallback onPressed;

  static VoidCallback? of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<TriggerPress>()?.onPressed;

  @override
  bool updateShouldNotify(TriggerPress old) => old.onPressed != onPressed;
}

/// A component in the tree, read the way its builder needs it.
class Node {
  Node(this.context, Map<Object?, Object?> json)
    : component = json[r'$el'] as String,
      props = (json['props'] as Map<Object?, Object?>?) ?? const {},
      children = (json['children'] as List<Object?>?) ?? const [];

  final BuildContext context;
  final String component;
  final Map<Object?, Object?> props;
  final List<Object?> children;

  bool has(String name) => props.containsKey(name);

  /// A prop's current value.
  Object? operator [](String name) => resolve(context, props[name]);

  String? string(String name) {
    final Object? value = this[name];
    return value == null ? null : '$value';
  }

  bool flag(String name) => truthy(this[name]);

  double? number(String name) => (this[name] as num?)?.toDouble();

  T option<T extends Enum>(List<T> values, String name, T fallback) {
    final String? value = string(name);
    return value == null ? fallback : values.byName(value);
  }

  /// A prop that holds a component, drawn.
  Widget? element(String name) {
    final Object? value = props[name];
    return value == null ? null : buildNode(context, value);
  }

  List<Widget> get kids => [
    for (final child in children) buildNode(context, child),
  ];

  /// The children as one string — the label of a button, the body of a
  /// callout.
  String get text => children
      .map(
        (child) => child is String ? child : '${resolve(context, child) ?? ''}',
      )
      .join();

  /// An event with no argument, or null when there is none.
  VoidCallback? action(String name) {
    final Object? value = props[name];
    return value == null ? null : () => run(context, value);
  }

  /// An event that passes what changed.
  ValueChanged<T>? handler<T>(String name) {
    final Object? value = props[name];
    return value == null ? null : (T changed) => run(context, value, [changed]);
  }
}

/// Draws a node: text, a glyph, a live value, or a component.
///
/// A component gets an element of its own, so it is built with the context
/// where it actually stands — under whatever its parent put above it: the
/// press a trigger is given, the state a sidebar holds.
Widget buildNode(BuildContext context, Object? node) {
  if (node is String) return Text(node);
  if (node is Map && node.containsKey(r'$icon')) return Icon(glyphOf(node));
  if (node is Map && node.containsKey(r'$el')) return _Component(node);
  if (_live(node)) {
    return Builder(
      builder: (context) {
        final Object? value = resolve(context, node);
        return value is Map
            ? buildNode(context, value)
            : Text('${value ?? ''}');
      },
    );
  }
  return const SizedBox.shrink();
}

class _Component extends StatelessWidget {
  const _Component(this.json);

  final Map<Object?, Object?> json;

  @override
  Widget build(BuildContext context) {
    final Node node = Node(context, json);
    final ComponentBuilder? build = components[node.component];
    if (build == null) {
      throw FlutterError('No Flutter mapping for <${node.component}>');
    }
    return build(node);
  }
}

/// The glyph a `$icon` names.
IconData glyphOf(Object? value) {
  final String name = (value as Map<Object?, Object?>)[r'$icon'] as String;
  final IconData? glyph = glyphs[name];
  if (glyph == null) throw FlutterError('No glyph for $name');
  return glyph;
}

/// Local state for a prop a story leaves to the component — a
/// `defaultValue`, a `defaultChecked` — since the kit's widgets are
/// controlled.
class Held<T> extends StatefulWidget {
  const Held({super.key, required this.initial, required this.builder});

  final T initial;
  final Widget Function(BuildContext context, T value, ValueChanged<T> set)
  builder;

  @override
  State<Held<T>> createState() => _HeldState<T>();
}

class _HeldState<T> extends State<Held<T>> {
  late T _value = widget.initial;

  @override
  Widget build(BuildContext context) => widget.builder(
    context,
    _value,
    (value) => setState(() => _value = value),
  );
}

/// A value a story either binds (`value` and its change event) or leaves to
/// the component (`default…`). Bound, it reads and writes the scene's slot;
/// left, it is held here.
Widget bound<T>(
  Node node, {
  String value = 'value',
  String? onChange,
  String? fallback,
  required T initial,
  required T Function(Object? raw) read,
  required Widget Function(T value, ValueChanged<T> set) builder,
}) {
  if (node.has(value)) {
    final ValueChanged<T>? write = onChange == null
        ? null
        : node.handler<T>(onChange);
    return builder(read(node[value]), write ?? (_) {});
  }
  final Object? start = fallback == null ? null : node[fallback];
  return Held<T>(
    initial: start == null ? initial : read(start),
    builder: (context, current, set) => builder(current, (next) {
      set(next);
      if (onChange != null) node.handler<T>(onChange)?.call(next);
    }),
  );
}
