import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/rendering.dart';
import 'package:flutter/widgets.dart';

/// The boundary around the whole window, so a snapshot is exactly what the
/// app drew — the same pixels on every desktop, without the window manager's
/// frame or anything else on screen.
final GlobalKey snapshotKey = GlobalKey(debugLabel: 'snapshot');

/// Writes the window to [path] as a PNG at the screen's own pixel ratio.
Future<File> saveSnapshot(String path) async {
  final BuildContext context = snapshotKey.currentContext!;
  final RenderRepaintBoundary boundary =
      context.findRenderObject()! as RenderRepaintBoundary;
  final double ratio = MediaQuery.devicePixelRatioOf(context);
  final ui.Image image = await boundary.toImage(pixelRatio: ratio);
  final data = await image.toByteData(format: ui.ImageByteFormat.png);
  image.dispose();
  final File file = File(path);
  await file.parent.create(recursive: true);
  return file.writeAsBytes(data!.buffer.asUint8List());
}

/// Where the status bar's button saves to: the pictures folder when there
/// is one, the home directory otherwise, named for the platform and the
/// moment so shots from several machines can share a folder.
String defaultSnapshotPath() {
  final Map<String, String> env = Platform.environment;
  final String home = env['HOME'] ?? env['USERPROFILE'] ?? '.';
  final String pictures = '$home${Platform.pathSeparator}Pictures';
  final String dir = Directory(pictures).existsSync() ? pictures : home;
  final String stamp = DateTime.now()
      .toIso8601String()
      .replaceAll(RegExp(r'[:.]'), '-')
      .substring(0, 19);
  final String desktop = (env['XDG_CURRENT_DESKTOP'] ?? '')
      .split(':')
      .first
      .toLowerCase();
  final String name = [
    'flutter_mail_example',
    Platform.operatingSystem,
    if (desktop.isNotEmpty) desktop,
    stamp,
  ].join('-');
  return '$dir${Platform.pathSeparator}$name.png';
}
