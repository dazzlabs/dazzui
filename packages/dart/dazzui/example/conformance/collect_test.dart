import 'dart:collection';
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;

import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/host.dart';
import 'package:dazzui_example/lib/scene.dart';
import 'package:dazzui_example/lib/story.dart';
import 'package:dazzui_example/lib/themes.dart';
import 'package:dazzui_example/stories/index.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';

import 'addresses.dart';
import 'measure.dart';

/*
 * The Flutter collector: draws every scene in scenes.json at 2x, in every
 * theme asked for, and writes readings.json, the scene images and a crop
 * per specimen. Run through tool/conformance.sh, which hands it these:
 */
const String out = String.fromEnvironment('CONFORMANCE_OUT');
const String themesAsked = String.fromEnvironment(
  'CONFORMANCE_THEMES',
  defaultValue: 'drawn',
);
const String scenesAsked = String.fromEnvironment('CONFORMANCE_SCENES');
const bool images = bool.fromEnvironment(
  'CONFORMANCE_IMAGES',
  defaultValue: true,
);
const String commit = String.fromEnvironment('CONFORMANCE_COMMIT');

const double scale = 2;
const String contractPath = '../../../../tools/conformance/scenes.json';

/*
 * React sets type in the system face, which on a Mac is SF. The test
 * binding has no system fonts, so the same files are loaded by name; where
 * they are missing the scenes are drawn in the layout face and the widths
 * of text will say so.
 */
const String uiFace = 'ConformanceUI';
const String codeFace = 'ConformanceCode';

Future<void> loadFaces() async {
  Future<void> load(String family, List<String> files) async {
    final FontLoader loader = FontLoader(family);
    for (final String path in files) {
      final File file = File(path);
      if (file.existsSync()) {
        loader.addFont(
          file.readAsBytes().then((b) => ByteData.sublistView(b)),
        );
        await loader.load();
        return;
      }
    }
  }

  await load(uiFace, [
    '/System/Library/Fonts/SFNS.ttf',
    '../test/fonts/Roboto-Regular.ttf',
    'test/fonts/Roboto-Regular.ttf',
  ]);
  await load(codeFace, ['/System/Library/Fonts/SFNSMono.ttf']);
}

ThemeData faced(ThemeData base) => base.copyWith(
  vars: base.vars.copyWith(
    fontUi: const FontFace(family: uiFace),
    fontDisplay: const FontFace(family: uiFace),
    fontCode: const FontFace(family: codeFace),
  ),
);

List<String> themeIds(Iterable<ThemeFamily> families) => [
  for (final family in families)
    for (final dark in [false, true]) ThemeChoice(family.value, dark: dark).id,
];

ThemeChoice themeOf(String id) {
  final int cut = id.lastIndexOf('-');
  return ThemeChoice(
    id.substring(0, cut),
    dark: id.substring(cut + 1) == 'dark',
  );
}

void main() {
  if (out.isEmpty) {
    test('conformance collection', () {}, skip: 'Run tool/conformance.sh');
    return;
  }

  final Map<String, Object?> contract =
      jsonDecode(File(contractPath).readAsStringSync()) as Map<String, Object?>;
  final List<String> onlyStories = scenesAsked.isEmpty
      ? const []
      : scenesAsked.split(',');
  final List<Map<String, Object?>> scenes = [
    for (final story in contract['stories'] as List<Object?>)
      if (onlyStories.isEmpty ||
          onlyStories.contains((story as Map)['id'] as String))
        for (final scene in (story as Map)['scenes'] as List<Object?>)
          scene as Map<String, Object?>,
  ];

  final Map<String, Scene> drawable = {
    for (final story in stories) ...{
      '${story.id}/preview': story.preview,
      for (final (index, example) in story.examples.indexed)
        '${story.id}/$index': example.demo,
    },
  };

  final List<String> themes = switch (themesAsked) {
    'drawn' => themeIds(themeFamilyGroups.first.families),
    'all' => themeIds(themeFamilyGroups.expand((g) => g.families)),
    _ => themesAsked.split(','),
  };

  final Map<String, Object?> readings = {
    'platform': 'flutter',
    'title': 'Flutter',
    'role': 'implementation',
    'collected': DateTime.now().toUtc().toIso8601String(),
    'commit': commit,
    'scale': scale,
    // The same system face as React's, and no embedded test face.
    'font': null,
    'themes': <String, Object?>{},
    'scenes': <String, Object?>{},
  };
  final List<String> failures = [];

  setUpAll(() async {
    await loadFaces();
    final Directory dir = Directory(out);
    if (dir.existsSync()) dir.deleteSync(recursive: true);
    dir.createSync(recursive: true);
  });

  tearDownAll(() {
    File('$out/readings.json').writeAsStringSync(jsonEncode(readings));
    if (failures.isNotEmpty) {
      File('$out/failures.txt').writeAsStringSync('${failures.join('\n')}\n');
    }
  });

  for (final String theme in themes) {
    testWidgets('flutter: $theme', (tester) async {
      tester.view.physicalSize = const Size(1024 * scale, 2400 * scale);
      tester.view.devicePixelRatio = scale;
      addTearDown(tester.view.reset);

      final ThemeData data = faced(themeOf(theme).data);
      final Map<String, Object?> readingsOf =
          (readings['themes'] as Map<String, Object?>)[theme] =
              <String, Object?>{};
      final Map<String, Object?> scenesOf =
          (readings['scenes'] as Map<String, Object?>)[theme] =
              <String, Object?>{};
      if (images) Directory('$out/images/$theme').createSync(recursive: true);

      for (final Map<String, Object?> contracted in scenes) {
        final String id = contracted['id'] as String;
        final Scene? scene = drawable[id];
        if (scene == null) {
          failures.add('$id: the example has no such scene');
          continue;
        }
        final GlobalKey frame = GlobalKey();
        await tester.pumpWidget(
          Host(
            title: 'conformance',
            home: Theme(
              data: data,
              child: DefaultTextStyle(
                style: data.vars.bodyMedium.copyWith(
                  color: data.vars.colorContent,
                ),
                child: Align(
                  alignment: Alignment.topLeft,
                  child: RepaintBoundary(
                    key: frame,
                    // The frame every collector draws a scene in: 40rem
                    // wide, spacing-6 around it, on the surface, centred.
                    child: Container(
                      width: 640,
                      padding: EdgeInsets.all(data.vars.spacing6),
                      color: data.vars.colorSurface,
                      child: Align(
                        heightFactor: 1,
                        child: SceneView(
                          key: ValueKey('$theme:$id'),
                          scene: scene,
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
        // Spinners never settle; a second lets everything else land.
        await tester.pump(const Duration(seconds: 1));

        final List<Map<Object?, Object?>> nodes = specimensOf(scene);
        final List<Object?> specimens =
            contracted['specimens'] as List<Object?>;
        final String want = [
          for (final s in specimens) (s as Map)['component'],
        ].join(' ');
        final String got = [
          for (final n in nodes) kebab(n[r'$el'] as String),
        ].join(' ');
        if (want != got) {
          failures.add('$id: codegen addressed $want, the walk found $got');
          continue;
        }

        // Each node leads to the components built from it, in tree order.
        final Map<Map<Object?, Object?>, Queue<Element>> built =
            LinkedHashMap.identity();
        // A node the kit is handed as data is found by its key instead.
        for (final Element element
            in find
                .byWidgetPredicate(
                  (w) => w is SceneComponent || w.key is NodeKey,
                  skipOffstage: false,
                )
                .evaluate()) {
          final Widget widget = element.widget;
          built
              .putIfAbsent(
                widget is SceneComponent
                    ? widget.json
                    : (widget.key! as NodeKey).json,
                Queue.new,
              )
              .add(element);
        }
        final List<RenderBox?> roots = [
          for (final node in nodes)
            built[node]?.isNotEmpty == true
                ? built[node]!.removeFirst().findRenderObject() as RenderBox?
                : null,
        ];
        // A provider, or a wrapper lending its trigger a behaviour, draws
        // nothing of its own: its root is a specimen inside it.
        for (int i = 0; i < roots.length; i++) {
          for (int j = i + 1; j < roots.length; j++) {
            if (roots[i] != null && identical(roots[i], roots[j])) {
              roots[i] = null;
            }
          }
        }

        final RenderRepaintBoundary boundary = tester
            .renderObject<RenderRepaintBoundary>(find.byKey(frame));
        final String file = id.replaceAll('/', '-');
        List<String?> crops = List.filled(roots.length, null);
        if (images) {
          final List<Rect?> rects = [
            for (final root in roots)
              root == null || !root.hasSize || !root.attached
                  ? null
                  : MatrixUtils.transformRect(
                      root.getTransformTo(boundary),
                      Offset.zero & root.size,
                    ),
          ];
          crops = (await tester.runAsync(
            () => _draw(boundary, rects, '$out/images/$theme', file),
          ))!;
          scenesOf[id] = 'images/$theme/$file.png';
        }

        final Set<RenderObject> nested = {...roots.nonNulls};
        for (int i = 0; i < roots.length; i++) {
          final RenderBox? root = roots[i];
          if (root == null || !root.hasSize || !root.attached) continue;
          final Map<String, Object?> reading = measure(root, nested, data.vars);
          if (crops[i] != null) reading['image'] = 'images/$theme/${crops[i]}';
          readingsOf[(specimens[i] as Map)['address'] as String] = reading;
        }
      }
      // ignore: avoid_print
      print('flutter: $theme');
    });
  }
}

/// Writes the scene as a PNG and a crop of it per rectangle; answers the
/// crops' file names.
Future<List<String?>> _draw(
  RenderRepaintBoundary boundary,
  List<Rect?> rects,
  String dir,
  String file,
) async {
  final ui.Image image = await boundary.toImage(pixelRatio: scale);
  Future<void> write(ui.Image image, String name) async {
    final ByteData? png = await image.toByteData(
      format: ui.ImageByteFormat.png,
    );
    File('$dir/$name').writeAsBytesSync(png!.buffer.asUint8List());
  }

  await write(image, '$file.png');
  final List<String?> names = [];
  for (final (index, rect) in rects.indexed) {
    if (rect == null || rect.width < 1 || rect.height < 1) {
      names.add(null);
      continue;
    }
    final Rect source = Rect.fromLTRB(
      rect.left * scale,
      rect.top * scale,
      rect.right * scale,
      rect.bottom * scale,
    ).intersect(Offset.zero & Size(image.width * 1.0, image.height * 1.0));
    if (source.isEmpty) {
      names.add(null);
      continue;
    }
    final ui.PictureRecorder recorder = ui.PictureRecorder();
    Canvas(recorder).drawImageRect(
      image,
      source,
      Offset.zero & source.size,
      Paint(),
    );
    final ui.Image crop = await recorder.endRecording().toImage(
      source.width.round(),
      source.height.round(),
    );
    final String name = '$file-$index.png';
    await write(crop, name);
    names.add(name);
  }
  return names;
}
