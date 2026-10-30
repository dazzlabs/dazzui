import 'dart:typed_data';

import 'package:flutter/widgets.dart';

import 'native_menu_model.dart';

NativeMenuBackend createNativeMenuBackend() => _Unsupported();

class _Unsupported implements NativeMenuBackend {
  @override
  bool get supported => false;

  @override
  Offset? get pointerPosition => null;

  @override
  Future<NativeMenuSession?> open({
    required List<NativeMenuEntry> entries,
    required List<Uint8List?> icons,
    required Offset position,
    required bool alignEnd,
    required VoidCallback onDismiss,
    required ValueChanged<int> onSelect,
  }) async => null;
}
