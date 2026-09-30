import 'dart:async';

import 'package:flutter/widgets.dart';

import '../theme/theme.dart';
import 'omarchy_palette.dart';
import 'omarchy_source.dart';

/// Draws [child] in the theme the Omarchy machine it runs on is wearing, and
/// follows it when `omarchy theme set` changes it.
///
/// Off Omarchy — another desktop, macOS, the web — or before the first read
/// lands, [child] is drawn in [fallback].
class OmarchyTheme extends StatefulWidget {
  const OmarchyTheme({
    super.key,
    required this.fallback,
    required this.child,
    this.brightness,
    this.directory,
  });

  /// The theme when there is no Omarchy palette to read.
  final ThemeData fallback;

  /// The appearance to draw in. Null draws the one the Omarchy theme ships
  /// in; the other one is [OmarchyPalette.opposite].
  final Brightness? brightness;

  /// Omarchy's `current` directory, when it is not where Omarchy keeps it.
  final String? directory;

  final Widget child;

  /// The palette the nearest [OmarchyTheme] is drawing, as Omarchy ships it,
  /// or null when it is drawing its fallback.
  static OmarchyPalette? paletteOf(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<_OmarchyScope>()?.palette;

  @override
  State<OmarchyTheme> createState() => _OmarchyThemeState();
}

class _OmarchyThemeState extends State<OmarchyTheme> {
  StreamSubscription<OmarchyPalette?>? _subscription;
  OmarchyPalette? _palette;
  ThemeData? _data;

  @override
  void initState() {
    super.initState();
    _listen();
  }

  @override
  void didUpdateWidget(OmarchyTheme oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.directory != widget.directory) {
      _subscription?.cancel();
      _listen();
    }
    if (oldWidget.brightness != widget.brightness) _data = _derive(_palette);
  }

  void _listen() {
    _subscription = watchOmarchyPalette(directory: widget.directory).listen(
      (palette) => setState(() {
        _palette = palette;
        _data = _derive(palette);
      }),
    );
  }

  ThemeData? _derive(OmarchyPalette? palette) {
    if (palette == null) return null;
    final wanted = widget.brightness ?? palette.brightness;
    return (wanted == palette.brightness ? palette : palette.opposite)
        .themeData;
  }

  @override
  void dispose() {
    _subscription?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return _OmarchyScope(
      palette: _palette,
      child: Theme(data: _data ?? widget.fallback, child: widget.child),
    );
  }
}

class _OmarchyScope extends InheritedWidget {
  const _OmarchyScope({required this.palette, required super.child});

  final OmarchyPalette? palette;

  @override
  bool updateShouldNotify(_OmarchyScope oldWidget) =>
      palette != oldWidget.palette;
}
