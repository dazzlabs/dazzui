import 'dart:convert';
import 'dart:io';

import 'specimens.dart';

/// The React reference, read back.
///
/// Both halves of this port's parity work read it: the test that measures a
/// specimen against it, and the app that draws one. The app is why this sits
/// in `lib/` rather than beside the test — a widget test draws every glyph
/// as a filled box, so the drawings have to be made by a real window.

/// What React measured for a specimen in one theme. A value is null where
/// the specimen's appearances disagreed, which means it was the story's.
class Expected {
  Expected({
    required this.width,
    required this.height,
    required this.backgroundColor,
    required this.borderColor,
    required this.borderWidth,
    required this.radius,
    required this.color,
    required this.fontSize,
    required this.fontWeight,
  });
  final double? width;
  final double? height;
  final String? backgroundColor;
  final String? borderColor;
  final double? borderWidth;
  final double? radius;
  final String? color;
  final double? fontSize;
  final int? fontWeight;
}

class Specimen {
  Specimen(this.key, this.probe, this.themes);
  final String key;
  final Probe probe;
  final Map<String, Expected> themes;
}

/// Read every reference page and fold its probes into specimens.
List<Specimen> loadSpecimens(String dir) {
  // key → theme → the raw probes seen
  final Map<String, Probe> probes = {};
  final Map<String, Map<String, List<Map<String, Object?>>>> seen = {};

  final List<File> files =
      Directory(dir)
          .listSync()
          .whereType<File>()
          .where((file) => file.path.endsWith('.json'))
          .toList()
        ..sort((a, b) => a.path.compareTo(b.path));

  for (final File file in files) {
    final Map<String, Object?> page =
        jsonDecode(file.readAsStringSync()) as Map<String, Object?>;
    final Map<String, Object?> stories =
        page['stories'] as Map<String, Object?>;
    for (final Object? story in stories.values) {
      final Map<String, Object?> themes =
          (story as Map<String, Object?>)['themes'] as Map<String, Object?>;
      for (final MapEntry<String, Object?> theme in themes.entries) {
        final List<Object?> list =
            (theme.value as Map<String, Object?>)['probes'] as List<Object?>;
        for (final Object? raw in list) {
          final Map<String, Object?> probe = raw as Map<String, Object?>;
          final Map<String, String> data = {
            for (final MapEntry<String, Object?> e
                in (probe['data'] as Map<String, Object?>).entries)
              if (!e.key.startsWith('base-ui') && e.key != 'control')
                e.key: e.value as String,
          };
          final String text = probe['text'] as String;
          final String className = probe['class'] as String;
          final String axes =
              (data.entries.map((e) => '${e.key}=${e.value}').toList()..sort())
                  .join(' ');
          // Text is not part of the key: a badge reading Beta and one
          // reading Failed are the same specimen, and a width that depends
          // on the text is skipped anyway. The first text seen is what the
          // specimen is built with.
          final String key = '$className${axes.isEmpty ? '' : ' $axes'}';
          probes[key] ??= Probe(className, data, text, {
            for (final Object? field
                in probe['declares'] as List<Object?>? ?? const [])
              field as String,
          });
          ((seen[key] ??= {})[theme.key] ??= []).add(probe);
        }
      }
    }
  }

  return [
    for (final MapEntry<String, Map<String, List<Map<String, Object?>>>> entry
        in seen.entries)
      Specimen(entry.key, probes[entry.key]!, {
        for (final MapEntry<String, List<Map<String, Object?>>> theme
            in entry.value.entries)
          theme.key: fold(theme.value),
      }),
  ];
}

/// The value every appearance agrees on, or null.
Expected fold(List<Map<String, Object?>> probes) {
  T? agreed<T>(T? Function(Map<String, Object?>) read) {
    final T? first = read(probes.first);
    for (final Map<String, Object?> probe in probes.skip(1)) {
      final T? other = read(probe);
      if (first is double && other is double) {
        if ((first - other).abs() > 0.5) return null;
      } else if (other != first) {
        return null;
      }
    }
    return first;
  }

  double? number(Object? value) => value is num ? value.toDouble() : null;
  Map<String, Object?> rect(Map<String, Object?> p) =>
      p['rect'] as Map<String, Object?>;
  Map<String, Object?> style(Map<String, Object?> p) =>
      p['style'] as Map<String, Object?>;

  return Expected(
    width: agreed((p) => number(rect(p)['width'])),
    height: agreed((p) => number(rect(p)['height'])),
    backgroundColor: agreed((p) => style(p)['backgroundColor'] as String?),
    borderColor: agreed((p) => style(p)['borderTopColor'] as String?),
    borderWidth: agreed((p) => number(style(p)['borderTopWidth'])),
    radius: agreed((p) => number(style(p)['borderTopLeftRadius'])),
    color: agreed((p) => style(p)['color'] as String?),
    fontSize: agreed((p) => number(style(p)['fontSize'])),
    fontWeight: agreed(
      (p) => int.tryParse(style(p)['fontWeight'] as String? ?? ''),
    ),
  );
}
