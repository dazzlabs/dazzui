/// The flat subset of TOML Omarchy's files are written in, as Omarchy's
/// shell reads them (`shell/Commons/Color.qml`): `key = value` lines, a key
/// under a `[section]` named `section.key`, a value's quotes and a trailing
/// `#` comment dropped. Arrays, inline tables and multi-line strings are
/// not read; neither `colors.toml` nor `shell.toml` writes one.
Map<String, String> readOmarchyToml(String source) {
  final values = <String, String>{};
  var section = '';
  for (final raw in source.split('\n')) {
    final line = raw.trim();
    if (line.isEmpty || line.startsWith('#')) continue;
    final header = _section.firstMatch(line);
    if (header != null) {
      section = header[1]!.trim();
      continue;
    }
    final entry = _entry.firstMatch(line);
    if (entry == null) continue;
    final key = section.isEmpty ? entry[1]! : '$section.${entry[1]}';
    values[key] = entry[2] ?? entry[3] ?? entry[4]!.trim();
  }
  return values;
}

final _section = RegExp(r'^\[([^\]]+)\]');

final _entry = RegExp(
  r'''^([A-Za-z0-9_-]+)\s*=\s*(?:"([^"]*)"|'([^']*)'|([^#]*))''',
);
