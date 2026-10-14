import 'package:dazzui_example/lib/themes.dart';

/// The page and theme an address names, the way the React example writes
/// them: `?theme=ember-dark` and `#/button`. On the web that is the
/// window's own address; native runs can choose their initial theme with
/// `--dart-define=PLAYGROUND_THEME=spotify-dark`.
({String? page, ThemeChoice theme}) readLocation(Uri address) {
  final String fragment = address.fragment;
  final String page = fragment.startsWith('/') ? fragment.substring(1) : '';
  final String id =
      address.queryParameters['theme'] ??
      const String.fromEnvironment('PLAYGROUND_THEME');
  final int cut = id.lastIndexOf('-');
  final String family = cut < 0 ? '' : id.substring(0, cut);
  final String mode = cut < 0 ? '' : id.substring(cut + 1);
  final bool known = themeFamilyGroups.any(
    (group) => group.families.any((f) => f.value == family),
  );
  return (
    page: page.isEmpty ? null : page,
    theme: known && (mode == 'light' || mode == 'dark')
        ? ThemeChoice(family, dark: mode == 'dark')
        : defaultTheme,
  );
}
