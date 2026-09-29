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

  @override
  Widget build(BuildContext context) {
    return Host(
      title: 'DazzUI — Flutter example',
      home: Theme(
        data: _theme.data,
        child: Shell(
          page: _page,
          open: (page) => setState(() => _page = page),
          theme: _theme,
          setTheme: (theme) => setState(() => _theme = theme),
        ),
      ),
    );
  }
}
