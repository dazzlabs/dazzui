import 'dart:convert';
import 'dart:io';

import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' as material;
import 'package:flutter_test/flutter_test.dart';

import 'specimens.dart';

/// The React reference, measured against.
///
/// `reference/*.json` is what `apps/storybook` rendered: every component root
/// on every story, in every theme, with its `data-*` axes and the box,
/// border, corner, colours and type the browser computed. This test reads
/// those probes back as *specimens* — a class, its axes, its text — builds
/// each one here with the matching widget, lays it out under the same theme,
/// and measures the same things. Where the two disagree is where the port
/// has drifted from the standard, and it is listed rather than argued about.
///
/// What is compared is only what can be read on both sides without a screen:
/// the box, the outermost decoration, and the first run of text. A width
/// that depends on text is skipped, because the test font here is not the
/// browser's; a width that varies between the stories a specimen appears in
/// is skipped, because it belonged to the story rather than the component.
///
/// `drift.txt` is the ratchet: every check that fails today is named there,
/// held both ways, so the list can only shrink. Bringing a component into
/// line means deleting its lines in the same change.
///
/// None of this is committed: the reference is what `pnpm parity` writes,
/// and the list is recorded by the first run that finds it missing. A
/// checkout without the reference skips rather than fails.
void main() {
  const dir = 'test/parity';
  final bool present = Directory('$dir/reference').existsSync();
  final List<Specimen> specimens = present
      ? loadSpecimens('$dir/reference')
      : const [];
  final File driftFile = File('$dir/drift.txt');
  final Set<String>? drift = driftFile.existsSync()
      ? driftFile
            .readAsLinesSync()
            .map((line) => line.trim())
            .where((line) => line.isNotEmpty && !line.startsWith('#'))
            .toSet()
      : null;

  testWidgets('every specimen the port can build measures as React drew it', (
    tester,
  ) async {
    if (!present) {
      markTestSkipped('no $dir/reference — run `pnpm parity` at the root');
      return;
    }
    tester.view.physicalSize = const Size(1920, 1280);
    tester.view.devicePixelRatio = 2.0;
    addTearDown(tester.view.reset);

    final Set<String> failing = {};
    final Map<String, String> detail = {};
    int built = 0;
    final Set<String> unsupported = {};
    // What this port measured, written beside the drift list for the
    // viewer (`pnpm parity:view`): the value on this side of every check.
    final Map<String, Map<String, Map<String, Object?>>> measuredAll = {};

    for (final Specimen specimen in specimens) {
      final Widget? widget = build(specimen.probe);
      if (widget == null) {
        unsupported.add(specimen.probe.className);
        continue;
      }
      built += 1;

      for (final MapEntry<String, Expected> entry in specimen.themes.entries) {
        final Measured? measured = await measure(
          tester,
          widget,
          key: '${specimen.key}/${entry.key}',
          theme: entry.key,
          // A field has no width of its own; give it the one the story did.
          width: specimen.probe.isField ? (entry.value.width ?? 240) : null,
        );
        if (measured == null) {
          failing.add('${specimen.key} · builds');
          detail['${specimen.key} · builds'] = 'threw under ${entry.key}';
          continue;
        }
        (measuredAll[specimen.key] ??= {})[entry.key] = measured.toJson(
          decoration: !specimen.probe.decorationIsAPart,
        );
        for (final Mismatch mismatch in compare(
          specimen.probe,
          entry.value,
          measured,
        )) {
          final String check = '${specimen.key} · ${mismatch.field}';
          failing.add(check);
          detail[check] ??=
              '${entry.key}: React ${mismatch.expected}, '
              'here ${mismatch.actual}';
        }
      }
    }

    File('$dir/measured.json').writeAsStringSync(
      const JsonEncoder.withIndent(' ').convert({
        'platform': 'flutter',
        'specimens': measuredAll,
      }),
    );

    if (drift == null) {
      driftFile.writeAsStringSync(
        [
          '# Checks that fail against the React reference today, one per line:',
          '# a specimen (class and axes) and the field that differs.',
          '# Recorded by the first test run; held both ways from now on, so it',
          '# can only shrink. Run the test to see the values on each side.',
          ...failing.toList()..sort(),
          '',
        ].join('\n'),
      );
      // ignore: avoid_print
      print('recorded ${failing.length} checks into $dir/drift.txt');
    }
    final Set<String> held = drift ?? failing;

    final List<String> problems = [
      for (final check in failing.difference(held))
        'DRIFT  $check — ${detail[check]}',
      for (final check in held.difference(failing))
        'STALE  $check — passes now, remove it from drift.txt',
    ]..sort();

    // Printed one per line rather than carried in the expectation, which
    // the reporter wraps: these lines are what drift.txt is written from.
    // ignore: avoid_print
    print(
      'parity: ${specimens.length} specimens, $built built, '
      '${failing.length} checks drifting, '
      'no adapter for: ${(unsupported.toList()..sort()).join(' ')}',
    );
    for (final String problem in problems) {
      // ignore: avoid_print
      print(problem);
    }

    expect(
      problems,
      isEmpty,
      reason:
          '${problems.length} checks are out of step with drift.txt; '
          'see the lines above',
    );
  });
}

/// Lay one specimen out under a theme and read it back.
Future<Measured?> measure(
  WidgetTester tester,
  Widget widget, {
  required String key,
  required String theme,
  required double? width,
}) async {
  // A key of its own per specimen and theme, which is load bearing: the
  // tester reuses the element tree between pumps, so a shared key would
  // hand the next specimen the last one's `State` — and an implicitly
  // animated decoration would still be showing the colour, corner and box
  // of the specimen before it. Every measurement is of a subtree built
  // from scratch.
  final Key subtreeKey = ValueKey(key);
  final Widget subject = KeyedSubtree(key: subtreeKey, child: widget);

  await tester.pumpWidget(
    material.MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Theme(
        data: themeData(theme),
        child: material.Material(
          type: material.MaterialType.transparency,
          child: Center(
            child: width == null
                ? subject
                : SizedBox(width: width, child: subject),
          ),
        ),
      ),
    ),
  );
  await tester.pump();
  if (tester.takeException() != null) return null;

  final Finder root = find.byKey(subtreeKey);
  final Size size = tester.getSize(root);

  // The box React's probe is of: the one that fills the component's own
  // bounds. A progress bar is the reason it has to be measured rather than
  // taken in order — its first drawn box is the fill, which is a fraction
  // of the track the root actually is. Among the boxes that do fill it, a
  // fill outranks a bare border, since a focus ring is a transparent
  // border sitting outside the control.
  final List<BoxDecoration> filling = [];
  final List<BoxDecoration> rest = [];
  for (final Element element
      in find
          .descendant(
            of: root,
            matching: find.byWidgetPredicate(
              (widget) => widget is DecoratedBox || widget is ColoredBox,
            ),
          )
          .evaluate()) {
    final Widget widget = element.widget;
    // A flat fill is a `ColoredBox` rather than a `DecoratedBox` — a
    // progress bar's track is one — and to the stylesheet it is the same
    // `background-color` either way.
    final Decoration? candidate = switch (widget) {
      DecoratedBox(decoration: final Decoration value) => value,
      ColoredBox(color: final Color value) => BoxDecoration(color: value),
      _ => null,
    };
    if (candidate is! BoxDecoration) continue;
    final RenderObject? render = element.renderObject;
    final bool fills =
        render is RenderBox &&
        (render.size.width - size.width).abs() <= 0.5 &&
        (render.size.height - size.height).abs() <= 0.5;
    (fills ? filling : rest).add(candidate);
  }
  final List<BoxDecoration> boxes = [...filling, ...rest];
  final BoxDecoration? decoration =
      boxes.where((box) => box.color != null).firstOrNull ??
      boxes.where((box) => box.border != null).firstOrNull;

  // The type the component sets its content in. A field's value is drawn by
  // an `EditableText` rather than a `RichText`, so it has to be asked
  // directly; everywhere else the first run that says something is the
  // component's own label, and an empty one is the placeholder a field
  // draws behind its value.
  TextStyle? text;
  final Finder editables = find.descendant(
    of: root,
    matching: find.byType(EditableText),
  );
  if (editables.evaluate().isNotEmpty) {
    text = tester.widget<EditableText>(editables.first).style;
  } else {
    for (final RichText run in tester.widgetList<RichText>(
      find.descendant(of: root, matching: find.byType(RichText)),
    )) {
      if (run.text.toPlainText().trim().isEmpty) continue;
      text = run.text.style;
      break;
    }
  }

  final BoxBorder? border = decoration?.border;
  final BorderSide? side = border is Border ? border.top : null;
  final BorderRadiusGeometry? corners = decoration?.borderRadius;

  return Measured(
    width: size.width,
    height: size.height,
    backgroundColor: decoration == null
        ? null
        : hex(decoration.color ?? const Color(0x00000000)),
    borderColor: side == null
        ? null
        : hex(side.width == 0 ? const Color(0x00000000) : side.color),
    borderWidth: side?.width,
    radius: corners is BorderRadius ? corners.topLeft.x : null,
    color: text?.color == null ? null : hex(text!.color!),
    fontSize: text?.fontSize,
    fontWeight: text?.fontWeight == null ? null : text!.fontWeight!.value,
  );
}

ThemeData themeData(String theme) => switch (theme) {
  'studio-dark' => ThemeData.studioDark(),
  'bright-light' => ThemeData.brightLight(),
  'bright-dark' => ThemeData.brightDark(),
  _ => ThemeData.studioLight(),
};

/// `#rrggbb`, or `#rrggbbaa` when translucent — the reference's spelling.
String hex(Color color) {
  String channel(double value) =>
      (value * 255).round().toRadixString(16).padLeft(2, '0');
  final String rgb =
      '#${channel(color.r)}${channel(color.g)}${channel(color.b)}';
  return color.a >= 1 ? rgb : '$rgb${channel(color.a)}';
}

/// What React measured for one specimen in one theme, and what this port
/// measured, compared field by field where both sides have a value.
List<Mismatch> compare(Probe probe, Expected expected, Measured measured) {
  final List<Mismatch> out = [];

  void length(String field, double? want, double? got) {
    if (want == null || got == null) return;
    if ((want - got).abs() > 0.5) out.add(Mismatch(field, '$want', '$got'));
  }

  /// A style field, compared only where the React stylesheet declares it.
  void styled(String field, void Function() compare) {
    if (probe.declares.contains(field)) compare();
  }

  void colour(String field, String? want, String? got) {
    if (want == null || got == null) return;
    if (!sameColour(want, got)) out.add(Mismatch(field, want, got));
  }

  // A width that depends on text is the test font's, not the design's.
  if (probe.text.isEmpty) length('width', expected.width, measured.width);
  if (!probe.heightIsContentDriven) {
    length('height', expected.height, measured.height);
  }
  // A checkbox's root is the row its box sits in; the React probe carries
  // the row's (transparent) decoration where this side reads the box's.
  if (!probe.decorationIsAPart) {
    styled('background', () {
      colour('background', expected.backgroundColor, measured.backgroundColor);
    });
    // A spinner's ring is two colours — the head on one side, the track on
    // the other three — and the probe carries only the top. Comparing it
    // against a port's single border colour is comparing the head with the
    // track.
    if (probe.className != 'dz-spinner') {
      styled('border-color', () {
        colour('border-color', expected.borderColor, measured.borderColor);
      });
    }
    styled('border-width', () {
      length('border-width', expected.borderWidth, measured.borderWidth);
    });
    styled('radius', () => length('radius', expected.radius, measured.radius));
  }
  styled('color', () => colour('color', expected.color, measured.color));
  styled('font-size', () {
    length('font-size', expected.fontSize, measured.fontSize);
  });
  styled('font-weight', () {
    if (expected.fontWeight != null &&
        measured.fontWeight != null &&
        expected.fontWeight != measured.fontWeight) {
      out.add(
        Mismatch(
          'font-weight',
          '${expected.fontWeight}',
          '${measured.fontWeight}',
        ),
      );
    }
  });
  return out;
}

/// Equal to the eye: the same channels, with one step of slack on alpha,
/// which is where `0.12 × 255` rounds differently on two platforms.
bool sameColour(String a, String b) {
  int? alpha(String value) =>
      value.length == 9 ? int.parse(value.substring(7), radix: 16) : 255;
  final String rgbA = a.substring(0, 7);
  final String rgbB = b.substring(0, 7);
  if (alpha(a) == 0 && alpha(b) == 0) return true;
  return rgbA == rgbB && (alpha(a)! - alpha(b)!).abs() <= 1;
}

class Mismatch {
  const Mismatch(this.field, this.expected, this.actual);
  final String field;
  final String expected;
  final String actual;
}

class Measured {
  const Measured({
    required this.width,
    required this.height,
    this.backgroundColor,
    this.borderColor,
    this.borderWidth,
    this.radius,
    this.color,
    this.fontSize,
    this.fontWeight,
  });
  final double width;
  final double height;
  final String? backgroundColor;
  final String? borderColor;
  final double? borderWidth;
  final double? radius;
  final String? color;
  final double? fontSize;
  final int? fontWeight;

  /// The fields by the names the drift list uses. Decoration is left out
  /// where the React root is a row and the drawn box is a part inside it.
  Map<String, Object?> toJson({required bool decoration}) => {
    'width': width,
    'height': height,
    if (decoration) 'background': backgroundColor,
    if (decoration) 'border-color': borderColor,
    if (decoration) 'border-width': borderWidth,
    if (decoration) 'radius': radius,
    'color': color,
    'font-size': fontSize,
    'font-weight': fontWeight,
  };
}

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
