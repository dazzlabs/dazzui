import 'dart:async';
import 'dart:ui' as ui;

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';

import 'menu_configuration.dart';
import 'native_menu_model.dart';
import 'native_menu_stub.dart' if (dart.library.io) 'native_menu_desktop.dart';

export 'native_menu_model.dart';

/// Internal test seam; production always uses the platform backend.
@visibleForTesting
NativeMenuBackend? debugNativeMenuBackend;

/// Owns one opening, including async icon rendering and cancellation.
class NativeMenuController {
  NativeMenuSession? _session;
  int _generation = 0;
  NativeMenuBackend? _backend;

  Offset? get pointerPosition =>
      _session == null ? null : _backend?.pointerPosition;

  Future<bool> open({
    required BuildContext context,
    required List<NativeMenuEntry> entries,
    Offset? position,
    bool alignEnd = false,
    required VoidCallback onDismiss,
  }) async {
    close();
    final generation = _generation;
    if (MenuConfiguration.of(context) != MenuPresentation.native) return false;
    final backend = debugNativeMenuBackend ?? createNativeMenuBackend();
    _backend = backend;
    if (!backend.supported) return false;
    final render = context.findRenderObject();
    if (position == null && render is! RenderBox) return false;
    final at =
        position ??
        (render as RenderBox).localToGlobal(
          alignEnd
              ? render.size.bottomRight(Offset.zero)
              : render.size.bottomLeft(Offset.zero),
        );
    final color =
        DefaultTextStyle.of(context).style.color ?? const Color(0xff000000);
    final icons = <Uint8List?>[];
    try {
      for (final entry in entries) {
        icons.add(await _icon(entry.icon, color));
        if (_generation != generation) return true;
      }
      var completed = false;
      final session = await backend.open(
        entries: entries,
        icons: icons,
        position: at,
        alignEnd: alignEnd,
        onDismiss: () {
          if (_generation != generation) return;
          completed = true;
          close();
          onDismiss();
        },
        onSelect: (index) {
          if (_generation != generation ||
              index < 0 ||
              index >= entries.length) {
            return;
          }
          final entry = entries[index];
          if (!entry.enabled || entry.heading) return;
          completed = true;
          close();
          onDismiss();
          entry.onSelect?.call();
        },
      );
      if (_generation != generation || completed) {
        session?.close();
        return true;
      }
      _session = session;
      return session != null;
    } catch (error, stack) {
      if (_generation != generation) return true;
      FlutterError.reportError(
        FlutterErrorDetails(
          exception: error,
          stack: stack,
          library: 'dazzui',
          context: ErrorDescription('while opening a native menu'),
        ),
      );
      return false;
    }
  }

  void close() {
    _generation++;
    _session?.close();
    _session = null;
  }
}

Future<Uint8List?> _icon(IconData? icon, Color color) async {
  if (icon == null) return null;
  final recorder = ui.PictureRecorder();
  final canvas = Canvas(recorder);
  final painter = TextPainter(
    textDirection: TextDirection.ltr,
    text: TextSpan(
      text: String.fromCharCode(icon.codePoint),
      style: TextStyle(
        fontFamily: icon.fontFamily,
        package: icon.fontPackage,
        fontSize: 32,
        color: color,
      ),
    ),
  )..layout();
  painter.paint(
    canvas,
    Offset((32 - painter.width) / 2, (32 - painter.height) / 2),
  );
  painter.dispose();
  final picture = recorder.endRecording();
  final image = await picture.toImage(32, 32);
  picture.dispose();
  final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
  image.dispose();
  return bytes?.buffer.asUint8List();
}

/// Shared cleanup for every dropdown, including inherited changes and unmount.
mixin NativeMenuOwner<T extends StatefulWidget> on State<T> {
  final nativeMenu = NativeMenuController();
  MenuPresentation? _presentation;

  void dismissMenu();

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final presentation = MenuConfiguration.of(context);
    if (_presentation != null && _presentation != presentation) {
      nativeMenu.close();
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) dismissMenu();
      });
    }
    _presentation = presentation;
  }

  @override
  void dispose() {
    nativeMenu.close();
    super.dispose();
  }
}
