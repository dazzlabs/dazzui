import 'dart:io';

import 'package:dazzui/dazzui.dart';
import 'package:flutter/services.dart';

/// A real face for layout/interaction tests; never a React pixel baseline.
/// Ahem's square glyphs overflow layouts whose controls fit with normal text.
Future<void> loadLayoutFont() async {
  final loader = FontLoader('SceneTestSans')
    ..addFont(
      File('test/fonts/Roboto-Regular.ttf').readAsBytes().then(
        (bytes) => ByteData.sublistView(bytes),
      ),
    );
  await loader.load();
}

ThemeData layoutTheme(ThemeData base) => base.copyWith(
  vars: base.vars.copyWith(
    fontUi: const FontFace(family: 'SceneTestSans'),
    fontDisplay: const FontFace(family: 'SceneTestSans'),
  ),
);
