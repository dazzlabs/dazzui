import 'dart:ffi' as ffi;
import 'dart:ui' show Offset;

import 'package:ffi/ffi.dart';

final class _Point extends ffi.Struct {
  @ffi.Int32()
  external int x;

  @ffi.Int32()
  external int y;
}

final _user32 = ffi.DynamicLibrary.open('user32.dll');
final _foregroundWindow = _user32
    .lookupFunction<
      ffi.Pointer<ffi.Void> Function(),
      ffi.Pointer<ffi.Void> Function()
    >('GetForegroundWindow');
final _dpiForWindow = _user32
    .lookupFunction<
      ffi.Uint32 Function(ffi.Pointer<ffi.Void>),
      int Function(ffi.Pointer<ffi.Void>)
    >('GetDpiForWindow');
final _clientToScreen = _user32
    .lookupFunction<
      ffi.Int32 Function(ffi.Pointer<ffi.Void>, ffi.Pointer<_Point>),
      int Function(ffi.Pointer<ffi.Void>, ffi.Pointer<_Point>)
    >('ClientToScreen');
final _trackPopupMenu = _user32
    .lookupFunction<
      ffi.Uint32 Function(
        ffi.Pointer<ffi.Void>,
        ffi.Uint32,
        ffi.Int32,
        ffi.Int32,
        ffi.Pointer<ffi.Void>,
        ffi.Pointer<ffi.Void>,
      ),
      int Function(
        ffi.Pointer<ffi.Void>,
        int,
        int,
        int,
        ffi.Pointer<ffi.Void>,
        ffi.Pointer<ffi.Void>,
      )
    >('TrackPopupMenuEx');

const _rightAlign = 0x0008;
const _noNotify = 0x0080;
const _returnCommand = 0x0100;

/// Tracks nativeapi's HMENU against the Flutter window. nativeapi's own
/// popup presenter gives its hidden dispatcher window the foreground.
/// Returns the command ID, zero on dismissal, or null without a usable owner.
int? trackWindowsMenu({
  required ffi.Pointer<ffi.Void> menu,
  required ffi.Pointer<ffi.Void> owner,
  required Offset position,
  required bool alignEnd,
}) {
  // Icon rendering can finish after the user has switched to another window.
  if (owner == ffi.nullptr ||
      menu == ffi.nullptr ||
      owner != _foregroundWindow()) {
    return null;
  }
  final dpi = _dpiForWindow(owner);
  if (dpi == 0) return null;
  final point = calloc<_Point>();
  try {
    final scale = dpi / 96;
    point.ref
      ..x = (position.dx * scale).round()
      ..y = (position.dy * scale).round();
    if (_clientToScreen(owner, point) == 0) return null;
    return _trackPopupMenu(
      menu,
      _noNotify | _returnCommand | (alignEnd ? _rightAlign : 0),
      point.ref.x,
      point.ref.y,
      owner,
      ffi.nullptr,
    );
  } finally {
    calloc.free(point);
  }
}
