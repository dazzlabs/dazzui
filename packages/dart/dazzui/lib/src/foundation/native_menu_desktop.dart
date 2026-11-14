import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/widgets.dart';
import 'package:nativeapi/nativeapi.dart' as native;

import 'native_menu_model.dart';

NativeMenuBackend createNativeMenuBackend() => _DesktopBackend();

class _DesktopBackend implements NativeMenuBackend {
  @override
  bool get supported =>
      Platform.isMacOS || Platform.isWindows || Platform.isLinux;

  @override
  Offset? get pointerPosition {
    if (!supported) return null;
    final window = native.WindowManager.instance.getCurrent();
    if (window == null) return null;
    try {
      final cursor = native.DisplayManager.instance.getCursorPosition();
      final bounds = window.contentBounds;
      return Offset(cursor.x - bounds.x, cursor.y - bounds.y);
    } finally {
      window.dispose();
    }
  }

  @override
  Future<NativeMenuSession?> open({
    required List<NativeMenuEntry> entries,
    required List<Uint8List?> icons,
    required Offset position,
    required bool alignEnd,
    required VoidCallback onDismiss,
    required ValueChanged<int> onSelect,
  }) async {
    if (!supported ||
        !native.Menu.isBackendSupported(native.MenuBackend.native)) {
      return null;
    }
    final window = native.WindowManager.instance.getCurrent();
    if (window == null) return null;
    final menu = native.Menu.create();
    if (menu == null) {
      window.dispose();
      return null;
    }
    final session = _DesktopSession(menu, onDismiss);
    try {
      for (var index = 0; index < entries.length; index++) {
        final entry = entries[index];
        // A hint is display text, not an accelerator registered with the OS.
        final label = entry.shortcut == null
            ? entry.label
            : '${entry.label}\t${entry.shortcut}';
        final item = native.MenuItem.createWithLabelAndType(
          label,
          entry.checked == null
              ? native.MenuItemType.normal
              : native.MenuItemType.checkbox,
        );
        if (item == null) throw StateError('Could not create native menu item');
        session.items.add(item);
        item.isEnabled = !entry.heading && entry.enabled;
        if (entry.checked != null) {
          item.state = entry.checked!
              ? native.MenuItemState.checked
              : native.MenuItemState.unchecked;
        }
        final bytes = icons[index];
        if (bytes != null) {
          final image = native.Image.fromBase64(base64Encode(bytes));
          if (image != null) {
            session.images.add(image);
            item.icon = image;
          }
        }
        final selectedIndex = index;
        final listener = item.addListener((event) {
          if (event is native.MenuItemClickedEvent &&
              !session.closed &&
              entry.enabled &&
              !entry.heading) {
            session.close();
            onSelect(selectedIndex);
          }
        });
        session.listeners.add(listener);
        menu.addItem(item);
      }
      session.menuListener = menu.addListener((event) {
        if (event is native.MenuOpenedEvent) {
          session.tracking = true;
          if (session.closed) menu.close();
        }
        if (event is native.MenuClosedEvent) {
          session.tracking = false;
          session.opening = false;
          if (session.closed) {
            session.release();
            return;
          }
          // AppKit delivers menuDidClose before the selected item's action.
          // Keep item listeners alive until queued native actions have arrived.
          session.dismissal ??= Timer(const Duration(milliseconds: 50), () {
            if (session.closed) return;
            session.close();
            onDismiss();
          });
        }
      });
      final strategy = native.PositioningStrategy.relativeWithWindowAndOffset(
        window,
        native.Point(x: position.dx, y: position.dy),
      );
      if (strategy == null) {
        session.close();
        return null;
      }
      try {
        session.opening = true;
        if (!menu.open(
          strategy,
          alignEnd ? native.Placement.bottomEnd : native.Placement.bottomStart,
        )) {
          session.opening = false;
          session.close();
          return null;
        }
      } finally {
        strategy.dispose();
      }
      return session;
    } catch (_) {
      session.opening = false;
      session.close();
      rethrow;
    } finally {
      window.dispose();
    }
  }
}

class _DesktopSession implements NativeMenuSession {
  _DesktopSession(this.menu, this.onDismiss);

  final native.Menu menu;
  final VoidCallback onDismiss;
  final items = <native.MenuItem>[];
  final listeners = <int>[];
  final images = <native.Image>[];
  int? menuListener;
  Timer? dismissal;
  bool closed = false;
  bool tracking = false;
  bool opening = false;
  bool released = false;

  @override
  void close() {
    if (closed) return;
    closed = true;
    dismissal?.cancel();
    if (opening || tracking) {
      // A queued AppKit popup still owns the native menu. Cancel it when its
      // opened event arrives, and release only after tracking has finished.
      if (tracking) menu.close();
      return;
    }
    release();
  }

  void release() {
    if (released) return;
    released = true;
    if (menuListener != null) menu.removeListener(menuListener!);
    for (var index = 0; index < listeners.length; index++) {
      items[index].removeListener(listeners[index]);
    }
    menu.dispose();
    for (final item in items) {
      item.dispose();
    }
    for (final image in images) {
      image.dispose();
    }
  }
}
