import 'dart:typed_data';

import 'package:flutter/widgets.dart';

/// Data shared with the platform presenter; callbacks stay in the widget tree.
class NativeMenuEntry {
  const NativeMenuEntry({
    required this.label,
    this.enabled = true,
    this.checked,
    this.icon,
    this.shortcut,
    this.onSelect,
    this.heading = false,
  });

  final String label;
  final bool enabled;
  final bool? checked;
  final IconData? icon;
  final String? shortcut;
  final VoidCallback? onSelect;
  final bool heading;
}

abstract interface class NativeMenuSession {
  void close();
}

abstract interface class NativeMenuBackend {
  bool get supported;

  /// Cursor location in the current Flutter window's logical coordinates.
  Offset? get pointerPosition;

  Future<NativeMenuSession?> open({
    required List<NativeMenuEntry> entries,
    required List<Uint8List?> icons,
    required Offset position,
    required bool alignEnd,
    required VoidCallback onDismiss,
    required ValueChanged<int> onSelect,
  });
}
