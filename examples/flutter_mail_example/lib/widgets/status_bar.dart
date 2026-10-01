import 'dart:io';

import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/services.dart';

import '../l10n.dart';
import '../settings.dart';
import '../snapshot.dart';

/// The foot of the window: the three switches that change how every piece
/// of text above is set, and a one-line reading of the environment.
class StatusBar extends StatefulWidget {
  const StatusBar({super.key, required this.settings, required this.onChanged});

  final Settings settings;
  final ValueChanged<Settings> onChanged;

  @override
  State<StatusBar> createState() => _StatusBarState();
}

class _StatusBarState extends State<StatusBar> {
  /// Where the last snapshot went, shown in place of the summary for a
  /// moment.
  String? _saved;

  Settings get settings => widget.settings;
  ValueChanged<Settings> get onChanged => widget.onChanged;

  Future<void> _snapshot() async {
    final S s = S.of(context);
    final String path;
    try {
      path = (await saveSnapshot(defaultSnapshotPath())).path;
    } on FileSystemException catch (error) {
      setState(() => _saved = s.snapshotFailed(error.message));
      return;
    }
    setState(() => _saved = s.snapshotSaved(path));
    await Future<void>.delayed(const Duration(seconds: 6));
    if (mounted) setState(() => _saved = null);
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    final TextStyle caption = vars.captionMedium.copyWith(
      color: vars.colorContentMuted,
    );
    final Environment env = Environment.read(context, settings);

    Widget labelled<T>(
      String label,
      double width,
      T value,
      List<T> values,
      String Function(T) name,
      ValueChanged<T> onChosen,
    ) {
      return Row(
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing2,
        children: [
          Text(label, style: caption),
          SizedBox(
            width: width,
            child: Select<T>(
              size: WidgetSize.small,
              value: value,
              options: [
                for (final v in values) SelectOption(value: v, label: name(v)),
              ],
              onChanged: (v) {
                if (v != null) onChosen(v);
              },
            ),
          ),
        ],
      );
    }

    return Container(
      height: 40,
      padding: EdgeInsets.symmetric(horizontal: vars.spacing3),
      decoration: BoxDecoration(
        color: vars.colorSurfaceColumn,
        border: Border(
          top: BorderSide(
            color: vars.colorBorder,
            width: context.hairlineWidth,
          ),
        ),
      ),
      child: Row(
        spacing: vars.spacing5,
        children: [
          labelled(
            s.language,
            190,
            settings.locale,
            LocaleChoice.values,
            s.localeName,
            (c) => onChanged(settings.copyWith(locale: c)),
          ),
          labelled(
            s.font,
            250,
            settings.font,
            FontChoice.values,
            s.fontName,
            (c) => onChanged(settings.copyWith(font: c)),
          ),
          labelled(
            s.appearance,
            150,
            settings.appearance,
            AppearanceChoice.values,
            s.appearanceName,
            (c) => onChanged(settings.copyWith(appearance: c)),
          ),
          const Spacer(),
          Flexible(
            flex: 0,
            child: Text(
              _saved ?? env.summary,
              style: caption,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
          ),
          Tooltip(
            label: s.saveSnapshot,
            child: IconButton(
              icon: const Icon(FluentIcons.camera_20_regular),
              variant: IconButtonVariant.plain,
              semanticsLabel: s.saveSnapshot,
              onPressed: _snapshot,
            ),
          ),
          Tooltip(
            label: s.environment,
            child: IconButton(
              icon: const Icon(FluentIcons.info_20_regular),
              variant: IconButtonVariant.plain,
              semanticsLabel: s.environment,
              onPressed: () => _showEnvironment(context, env),
            ),
          ),
        ],
      ),
    );
  }

  void _showEnvironment(BuildContext context, Environment env) {
    final ThemeData theme = Theme.of(context);
    showGeneralDialog<void>(
      context: context,
      barrierDismissible: true,
      barrierLabel: S.of(context).ok,
      barrierColor: const Color(0x00000000),
      transitionDuration: Duration.zero,
      pageBuilder: (context, _, _) => Theme(
        data: theme,
        child: _EnvironmentDialog(env: env),
      ),
    );
  }
}

class _EnvironmentDialog extends StatelessWidget {
  const _EnvironmentDialog({required this.env});

  final Environment env;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    void close() => Navigator.of(context).pop();

    return CallbackShortcuts(
      bindings: {const SingleActivator(LogicalKeyboardKey.escape): close},
      child: Focus(
        autofocus: true,
        child: DialogScrim(
          onDismiss: close,
          child: Dialog(
            children: [
              DialogHeader(
                title: s.environment,
                subtitle: s.environmentSubtitle,
              ),
              DialogBody(
                children: [
                  Container(
                    padding: EdgeInsets.all(vars.spacing3),
                    decoration: BoxDecoration(
                      color: vars.colorSurfaceSunken,
                      borderRadius: BorderRadius.circular(vars.radiusMedium),
                    ),
                    child: Text(
                      env.report,
                      style: TextStyle(
                        fontFamily: vars.fontCode.family,
                        fontFamilyFallback: vars.fontCode.fallback,
                        fontSize: 12,
                        height: 1.6,
                        color: vars.colorContent,
                      ),
                    ),
                  ),
                ],
              ),
              DialogFooter(
                children: [
                  Button(
                    onPressed: () =>
                        Clipboard.setData(ClipboardData(text: env.report)),
                    child: Text(s.copy),
                  ),
                  Button(
                    tint: ButtonTint.primary,
                    variant: ButtonVariant.filled,
                    onPressed: close,
                    child: Text(s.ok),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// What decides how the text was set: the platform, the desktop session and
/// input method on Linux, the locales asked for and resolved, the font, and
/// the scale.
class Environment {
  const Environment({
    required this.platform,
    required this.desktop,
    required this.session,
    required this.resolved,
    required this.pixelRatio,
    required this.report,
  });

  factory Environment.read(BuildContext context, Settings settings) {
    final S s = S.of(context);
    final Map<String, String> env = Platform.environment;
    String? variable(String name) {
      final String? value = env[name];
      return value == null || value.isEmpty ? null : value;
    }

    final String resolved = Localizations.localeOf(context).toLanguageTag();
    final double pixelRatio = MediaQuery.devicePixelRatioOf(context);
    final List<Locale> system =
        WidgetsBinding.instance.platformDispatcher.locales;
    final List<(String, String)> entries = [
      (s.platform, Platform.operatingSystem),
      (s.osVersion, Platform.operatingSystemVersion),
      if (Platform.isLinux) ...[
        (s.desktop, variable('XDG_CURRENT_DESKTOP') ?? '—'),
        (s.session, variable('XDG_SESSION_TYPE') ?? '—'),
        (
          s.inputMethod,
          [
            if (variable('GTK_IM_MODULE') case final v?) 'GTK_IM_MODULE=$v',
            if (variable('QT_IM_MODULE') case final v?) 'QT_IM_MODULE=$v',
            if (variable('XMODIFIERS') case final v?) 'XMODIFIERS=$v',
          ].join(' ').ifEmpty('—'),
        ),
        ('LANG', variable('LANG') ?? '—'),
        if (variable('LC_ALL') case final v?) ('LC_ALL', v),
      ],
      (s.systemLocale, Platform.localeName),
      (s.systemLocales, system.map((l) => l.toLanguageTag()).join(', ')),
      (s.chosenLocale, s.localeName(settings.locale)),
      (s.resolvedLocale, resolved),
      (s.font, s.fontName(settings.font)),
      (s.pixelRatio, pixelRatio.toString()),
      (s.textScale, MediaQuery.textScalerOf(context).scale(1).toString()),
    ];

    return Environment(
      platform: Platform.operatingSystem,
      desktop: Platform.isLinux ? variable('XDG_CURRENT_DESKTOP') : null,
      session: Platform.isLinux ? variable('XDG_SESSION_TYPE') : null,
      resolved: resolved,
      pixelRatio: pixelRatio,
      report: entries.map((entry) => s.field(entry.$1, entry.$2)).join('\n'),
    );
  }

  final String platform;
  final String? desktop;
  final String? session;
  final String resolved;
  final double pixelRatio;

  /// Every reading, one per line, in the window's language.
  final String report;

  /// One line for the status bar.
  String get summary => [
    platform,
    desktop,
    session,
    resolved,
    '@${pixelRatio}x',
  ].whereType<String>().join(' · ');
}

extension on String {
  String ifEmpty(String fallback) => isEmpty ? fallback : this;
}
