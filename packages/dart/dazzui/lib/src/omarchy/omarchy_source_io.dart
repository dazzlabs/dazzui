import 'dart:async';
import 'dart:io';

import 'package:flutter/widgets.dart';

import 'omarchy_palette.dart';
import 'shell_style.dart';

/// The palette the machine is wearing now, or null when this is not an
/// Omarchy machine or its current theme has no `colors.toml`.
///
/// Omarchy keeps its current theme in a directory it swaps whole on
/// `omarchy theme set`: `$XDG_STATE_HOME/omarchy/current/` (by default
/// `~/.local/state`), holding the staged `theme/` and a `theme.name`; before
/// that it was `~/.config/omarchy/current/`, whose `theme` is a symlink into
/// the theme itself. The first of those that holds a readable palette is the
/// one read. [directory] reads that one directory instead.
///
/// The type scale is read the way the shell reads it: the theme's
/// `shell.toml`, then the user's own in `~/.config/omarchy/` — or in
/// [config], when [directory] is given — whose keys win.
Future<OmarchyPalette?> readOmarchyPalette({
  String? directory,
  String? config,
}) async {
  final String? user = _config(directory, config);
  for (final current in _candidates(directory)) {
    final palette = await _read(current, user);
    if (palette != null) return palette;
  }
  return null;
}

/// The current palette, and again every time the machine changes theme.
///
/// Omarchy replaces `current/theme` outright and then writes
/// `current/theme.name`, so what is watched is `current/` itself, beside the
/// user's config directory for their own `shell.toml`; a burst of events is
/// read once, after it settles, and a palette the same as the last one is
/// not emitted twice.
Stream<OmarchyPalette?> watchOmarchyPalette({
  String? directory,
  String? config,
}) {
  final subscriptions = <StreamSubscription<FileSystemEvent>>[];
  Timer? settle;
  OmarchyPalette? last;
  var emitted = false;
  late final StreamController<OmarchyPalette?> controller;

  Future<void> refresh() async {
    final palette = await readOmarchyPalette(
      directory: directory,
      config: config,
    );
    if (controller.isClosed || (emitted && palette == last)) return;
    emitted = true;
    last = palette;
    controller.add(palette);
  }

  controller = StreamController<OmarchyPalette?>(
    onListen: () {
      refresh();
      for (final path in [
        ..._candidates(directory),
        ?_config(directory, config),
      ]) {
        final watched = Directory(path);
        if (!watched.existsSync()) continue;
        subscriptions.add(
          watched.watch().listen(
            (_) {
              settle?.cancel();
              settle = Timer(const Duration(milliseconds: 150), refresh);
            },
            // A directory that goes away is no longer an Omarchy to follow;
            // the others still are.
            onError: (Object _) {},
          ),
        );
      }
    },
    onCancel: () async {
      settle?.cancel();
      for (final subscription in subscriptions) {
        await subscription.cancel();
      }
    },
  );
  return controller.stream;
}

Iterable<String> _candidates(String? directory) sync* {
  if (directory != null) {
    yield directory;
    return;
  }
  if (!Platform.isLinux) return;
  final env = Platform.environment;
  final home = env['HOME'];
  final state =
      env['XDG_STATE_HOME'] ?? (home == null ? null : '$home/.local/state');
  if (state != null) yield '$state/omarchy/current';
  if (home != null) yield '$home/.config/omarchy/current';
}

String? _config(String? directory, String? config) {
  if (config != null || directory != null) return config;
  if (!Platform.isLinux) return null;
  final home = Platform.environment['HOME'];
  return home == null ? null : '$home/.config/omarchy';
}

Future<OmarchyPalette?> _read(String current, String? config) async {
  try {
    final colors = File('$current/theme/colors.toml');
    if (!await colors.exists()) return null;
    final lightMode = await File('$current/theme/light.mode').exists();
    return OmarchyPalette.tryParse(
      await colors.readAsString(),
      name: await _name(current),
      brightness: lightMode ? Brightness.light : null,
      shellStyle: OmarchyShellStyle.fromShellToml([
        for (final shell in [
          '$current/theme/shell.toml',
          if (config != null) '$config/shell.toml',
        ])
          if (await File(shell).exists()) await File(shell).readAsString(),
      ]),
    );
  } on FileSystemException {
    // Caught mid-swap: the next event reads it whole.
    return null;
  }
}

Future<String?> _name(String current) async {
  final file = File('$current/theme.name');
  if (await file.exists()) {
    final name = (await file.readAsString()).trim();
    if (name.isNotEmpty) return name;
  }
  final link = Link('$current/theme');
  if (await link.exists()) {
    final target = await link.target();
    return target.split('/').lastWhere((part) => part.isNotEmpty);
  }
  return null;
}
