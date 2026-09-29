/// A story, as codegen compiled it out of the React example: what the list
/// shows, the one scene that stands for the component on the overview, and
/// the examples its page shows.
class Story {
  const Story({
    required this.id,
    required this.name,
    required this.summary,
    required this.preview,
    required this.examples,
  });

  /// The kit's file name for the component, and its place in the address.
  final String id;
  final String name;
  final String summary;
  final Scene preview;
  final List<Example> examples;
}

class Example {
  const Example({required this.title, this.note, required this.demo});

  final String title;
  final String? note;
  final Scene demo;
}

/// A finished tree of components, and the slots it keeps live.
///
/// [content] is a node: a string is text; a map with `$el` is a component
/// with its `props` and `children`. A prop may hold a node, a glyph
/// (`$icon`), a slot's value (`$state`), a test on one (`$eq`, `$not`,
/// `$if`), or — for an event — the writes it makes (`$do`).
class Scene {
  const Scene({this.state = const {}, required this.content});

  final Map<String, Object?> state;
  final Map<String, Object?> content;
}
