import 'dart:async';

import 'package:dazzui/dazzui.dart';

import 'package:dazzui_example/host.dart';
import 'package:dazzui_example/lib/themes.dart';
import 'widgets/shell.dart';

/// The page being read and the theme it is read in — the two things the
/// React example keeps in its address — and the window that shows them.
class App extends StatefulWidget {
  const App({super.key, this.page, this.theme = defaultTheme});

  /// The story to open on, or null for the overview.
  final String? page;
  final ThemeChoice theme;

  @override
  State<App> createState() => _AppState();
}

class _AppState extends State<App> {
  late String? _page = widget.page;
  late ThemeChoice _theme = widget.theme;

  /// On an Omarchy machine the window wears the machine's theme, and follows
  /// it, until another is picked.
  StreamSubscription<OmarchyPalette?>? _omarchySubscription;
  OmarchyPalette? _omarchy;
  bool _followingOmarchy = true;

  @override
  void initState() {
    super.initState();
    _omarchySubscription = watchOmarchyPalette().listen(
      (palette) => setState(() {
        _omarchy = palette;
        if (palette != null && _followingOmarchy) {
          _theme = _theme.copyWith(dark: palette.brightness == Brightness.dark);
        }
      }),
    );
  }

  @override
  void dispose() {
    _omarchySubscription?.cancel();
    super.dispose();
  }

  ThemeData get _data {
    final OmarchyPalette? omarchy = _omarchy;
    if (omarchy == null || !_followingOmarchy) return _theme.data;
    final bool shipsDark = omarchy.brightness == Brightness.dark;
    return (shipsDark == _theme.dark ? omarchy : omarchy.opposite).themeData;
  }

  @override
  Widget build(BuildContext context) {
    return Host(
      title: 'DazzUI — Flutter example',
      home: Theme(
        data: _data,
        child: Shell(
          page: _page,
          open: (page) => setState(() => _page = page),
          theme: _theme,
          setTheme: (theme) => setState(() => _theme = theme),
          omarchy: _omarchy,
          followingOmarchy: _omarchy != null && _followingOmarchy,
          setFollowingOmarchy: (following) => setState(() {
            _followingOmarchy = following;
            final OmarchyPalette? omarchy = _omarchy;
            if (following && omarchy != null) {
              _theme = _theme.copyWith(
                dark: omarchy.brightness == Brightness.dark,
              );
            }
          }),
        ),
      ),
    );
  }
}
