import 'dart:io';

import 'package:dazzui/dazzui.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

import 'l10n.dart';
import 'settings.dart';
import 'snapshot.dart';
import 'widgets/mail_window.dart';

/// Set `FLUTTER_MAIL_SNAPSHOT=<path>` to draw the window, write it to that
/// PNG and quit — one command per desktop instead of a screenshot by hand.
/// `FLUTTER_MAIL_LOCALE`, `FLUTTER_MAIL_FONT`, `FLUTTER_MAIL_APPEARANCE` and
/// `FLUTTER_MAIL_OPEN` (a message id) set the window up first.
void main() {
  runApp(const MailApp());

  final String? snapshot = Platform.environment['FLUTTER_MAIL_SNAPSHOT'];
  if (snapshot != null && snapshot.isNotEmpty) {
    // Long enough for the window to reach its size and the fonts to load.
    Future<void>.delayed(const Duration(seconds: 3), () async {
      try {
        final File file = await saveSnapshot(snapshot);
        stdout.writeln('snapshot: ${file.path}');
        exit(0);
      } on FileSystemException catch (error) {
        stderr.writeln('snapshot failed: $error');
        exit(1);
      }
    });
  }
}

/// The app plumbing — `WidgetsApp`, not `MaterialApp`, the way the DazzUI
/// example is hosted — with the locale and the font held here, above the
/// window, because both change what every piece of text is set in.
class MailApp extends StatefulWidget {
  const MailApp({super.key, this.settings});

  /// The settings to start in. Defaults to what the environment asks for.
  final Settings? settings;

  @override
  State<MailApp> createState() => _MailAppState();
}

class _MailAppState extends State<MailApp> with WidgetsBindingObserver {
  late Settings _settings =
      widget.settings ?? Settings.fromEnvironment(Platform.environment);
  Brightness _platformBrightness =
      SchedulerBinding.instance.platformDispatcher.platformBrightness;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangePlatformBrightness() {
    setState(() {
      _platformBrightness =
          SchedulerBinding.instance.platformDispatcher.platformBrightness;
    });
  }

  @override
  Widget build(BuildContext context) {
    final ThemeData base = _settings.appearance.resolve(_platformBrightness);
    final ThemeData theme = base.copyWith(
      vars: _settings.font.apply(base.vars),
    );

    return WidgetsApp(
      onGenerateTitle: (context) => S.of(context).appTitle,
      color: theme.vars.colorCanvas,
      debugShowCheckedModeBanner: false,
      locale: _settings.locale.locale,
      supportedLocales: LocaleChoice.supported,
      localizationsDelegates: const [
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      pageRouteBuilder: <T>(RouteSettings settings, WidgetBuilder builder) =>
          PageRouteBuilder<T>(
            settings: settings,
            pageBuilder: (context, _, _) => builder(context),
          ),
      home: RepaintBoundary(
        key: snapshotKey,
        child: Theme(
          data: theme,
          child: MailWindow(
            openId: Platform.environment['FLUTTER_MAIL_OPEN'],
            settings: _settings,
            onSettingsChanged: (settings) =>
                setState(() => _settings = settings),
          ),
        ),
      ),
    );
  }
}
