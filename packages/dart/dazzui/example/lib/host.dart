import 'package:dazzui/dazzui.dart';
import 'package:flutter/cupertino.dart' show DefaultCupertinoLocalizations;

/// The plumbing an app is, with material left out of it.
///
/// `MaterialApp` used to stand here for the four things it brought along —
/// a `Directionality`, a `MediaQuery`, a `Navigator` for the overlay a dialog
/// or a menu is pushed into, and the localizations the text field reads its
/// button labels out of. None of those are material's; they are `WidgetsApp`'s,
/// which is what this is. The cupertino delegate is the one addition: the text
/// field this kit draws is a cupertino one underneath, and it asks
/// `CupertinoLocalizations` for the word on its clear button.
///
/// Nothing here paints. The look is the [Theme] the window puts on its own
/// subtree, which is the point the playground exists to make.
class Host extends StatelessWidget {
  const Host({super.key, required this.title, required this.home});

  final String title;
  final Widget home;

  @override
  Widget build(BuildContext context) {
    return WidgetsApp(
      title: title,
      color: themeVariables.colorCanvas,
      debugShowCheckedModeBanner: false,
      localizationsDelegates: const [DefaultCupertinoLocalizations.delegate],
      pageRouteBuilder: <T>(RouteSettings settings, WidgetBuilder builder) =>
          PageRouteBuilder<T>(
            settings: settings,
            pageBuilder: (context, _, _) => builder(context),
          ),
      home: home,
    );
  }
}
