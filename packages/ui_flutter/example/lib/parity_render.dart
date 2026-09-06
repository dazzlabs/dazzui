import 'dart:async';
import 'dart:io';
import 'dart:typed_data';
import 'dart:ui' as ui;

import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/parity/reference.dart';
import 'package:dazzui_example/parity/specimens.dart';
import 'package:flutter/material.dart' as material;
import 'package:flutter/rendering.dart';
import 'package:flutter/scheduler.dart';

/// Draw every specimen this port can build, in a real window.
///
/// The widget test beside this could draw them too, and its pictures would be
/// useless: a test resolves a `TextStyle` with no family to a font whose every
/// glyph is a filled rectangle, and the generated type styles name only a
/// *fallback* family — deliberately, since `-apple-system` is a request for
/// the platform's own face rather than a family to look up. Here that request
/// is answered, because here there is a platform.
///
/// It reads and writes inside its own sandbox container, which is the only
/// place a macOS app may: `HOME` points there rather than at the person's
/// home directory, so `$HOME/parity` is the app's own corner of the disk.
/// The reference is staged into it and the drawings are carried back out,
/// which `pnpm parity:flutter` does in three lines around this:
///
/// ```bash
/// pnpm parity:flutter
/// ```
///
/// It draws, writes `$HOME/parity/renders/<theme>/<id>.png`, and quits.
void main() {
  final binding = material.WidgetsFlutterBinding.ensureInitialized();
  binding.addPostFrameCallback((_) async {
    final int drawn = await draw(binding);
    stdout.writeln('parity: drew $drawn renderings into test/parity/renders');
    exit(drawn > 0 ? 0 : 1);
  });
  material.runApp(const _Host());
}

/// The window the specimens are drawn in.
///
/// It shows nothing: every drawing is made off to the side of the screen and
/// captured through its own repaint boundary, so the window is only here to
/// give the engine something to render into.
class _Host extends material.StatelessWidget {
  const _Host();

  @override
  Widget build(BuildContext context) {
    return material.MaterialApp(
      debugShowCheckedModeBanner: false,
      // The transparent `Material` is what carries a default text style, the
      // same way the widget test's host does. Without it a `Text` falls back
      // to the red-on-white it is drawn in when nothing has said how.
      home: material.Material(
        type: material.MaterialType.transparency,
        child: _slot,
      ),
    );
  }
}

final GlobalKey _boundary = GlobalKey();
final ValueNotifier<Widget?> _subject = ValueNotifier<Widget?>(null);

Widget get _slot => ValueListenableBuilder<Widget?>(
  valueListenable: _subject,
  builder: (context, subject, _) {
    if (subject == null) return const SizedBox.shrink();
    return material.Align(
      alignment: Alignment.topLeft,
      child: RepaintBoundary(key: _boundary, child: subject),
    );
  },
);

Future<int> draw(SchedulerBinding binding) async {
  const String define = String.fromEnvironment('PARITY_DIR');
  final String dir = define.isNotEmpty
      ? define
      : '${Platform.environment['HOME']}/parity';
  stdout.writeln('parity: reading $dir');
  if (!Directory('$dir/reference').existsSync()) {
    stdout.writeln('no $dir/reference — run `pnpm parity` at the root');
    return 0;
  }
  final Directory renders = Directory('$dir/renders');
  if (renders.existsSync()) renders.deleteSync(recursive: true);

  final List<Specimen> specimens = loadSpecimens('$dir/reference');
  int drawn = 0;
  for (final Specimen specimen in specimens) {
    final Widget? widget = build(specimen.probe);
    if (widget == null) continue;
    for (final String theme in specimen.themes.keys.toList()..sort()) {
      // The box React drew it in, whatever the specimen is. A checkbox row
      // and a nav row are as wide as the column they were laid out in, and
      // a picture of one at its intrinsic width is a picture of a different
      // arrangement — the point of the drawing is that the two are of the
      // same thing.
      final double? width = specimen.themes[theme]?.width;
      final Widget subject = width == null
          ? widget
          : SizedBox(width: width, child: widget);
      _subject.value = Theme(data: themeData(theme), child: subject);
      // Two frames: one to lay the specimen out, one to be sure an
      // implicitly animated decoration has arrived at its resting value.
      await _frame(binding);
      await _frame(binding);

      final RenderRepaintBoundary? render =
          _boundary.currentContext?.findRenderObject()
              as RenderRepaintBoundary?;
      if (render == null) continue;
      final ui.Image image = await render.toImage(pixelRatio: 2);
      final ByteData? data = await image.toByteData(
        format: ui.ImageByteFormat.png,
      );
      image.dispose();
      if (data == null) continue;

      final File file = File(
        '$dir/renders/$theme/${renderName(specimen.key)}.png',
      );
      file.parent.createSync(recursive: true);
      file.writeAsBytesSync(data.buffer.asUint8List());
      drawn += 1;
    }
  }
  return drawn;
}

/// One frame, awaited.
Future<void> _frame(SchedulerBinding binding) {
  final Completer<void> done = Completer<void>();
  binding.addPostFrameCallback((_) => done.complete());
  binding.scheduleFrame();
  return done.future;
}

/// The theme a name stands for.
ThemeData themeData(String theme) => switch (theme) {
  'studio-dark' => ThemeData.studioDark(),
  'bright-light' => ThemeData.brightLight(),
  'bright-dark' => ThemeData.brightDark(),
  _ => ThemeData.studioLight(),
};
