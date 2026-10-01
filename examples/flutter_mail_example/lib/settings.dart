import 'package:dazzui/dazzui.dart';

/// The locale the app asks the engine to set text in. It is what picks
/// between the Simplified, Traditional and Japanese forms of a unified Han
/// character when the font falls back.
enum LocaleChoice {
  system(null, null),
  zhHansCN(
    '简体中文（中国大陆）',
    Locale.fromSubtags(
      languageCode: 'zh',
      scriptCode: 'Hans',
      countryCode: 'CN',
    ),
  ),
  zhHantTW(
    '繁體中文（台灣）',
    Locale.fromSubtags(
      languageCode: 'zh',
      scriptCode: 'Hant',
      countryCode: 'TW',
    ),
  ),
  zhHantHK(
    '繁體中文（香港）',
    Locale.fromSubtags(
      languageCode: 'zh',
      scriptCode: 'Hant',
      countryCode: 'HK',
    ),
  ),
  jaJP('日本語', Locale('ja', 'JP')),
  enUS('English', Locale('en', 'US'));

  const LocaleChoice(this.label, this.locale);

  /// The language's own name for itself, as a language picker lists it; null
  /// for [system], which is named in the window's language.
  final String? label;
  final Locale? locale;

  static List<Locale> get supported => [
    for (final choice in values)
      if (choice.locale != null) choice.locale!,
  ];
}

/// The face every text style is set in.
///
/// [theme] keeps the theme's own (SF Pro Text, then PingFang SC — which
/// only a Mac has), and [system] names nothing at all, so the platform's
/// fallback decides. The rest name one family outright; a family that is not
/// installed falls back the same way [system] does.
enum FontChoice {
  theme(null, null),
  system(null, null),
  notoSansSC('Noto Sans CJK SC', 'Noto Sans CJK SC'),
  notoSerifSC('Noto Serif CJK SC', 'Noto Serif CJK SC'),
  sourceHanSansSC('思源黑体 Source Han Sans SC', 'Source Han Sans SC'),
  pingFangSC('苹方 PingFang SC', 'PingFang SC'),
  microsoftYaHei('微软雅黑 Microsoft YaHei', 'Microsoft YaHei'),
  wenQuanYi('文泉驿微米黑 WenQuanYi Micro Hei', 'WenQuanYi Micro Hei');

  const FontChoice(this.label, this.family);

  /// The family's name; null for [theme] and [system], which are named in
  /// the window's language.
  final String? label;
  final String? family;

  ThemeVariables apply(ThemeVariables vars) {
    if (this == theme) return vars;
    final FontFace face = FontFace(family: family);
    return vars.copyWith(fontUi: face, fontDisplay: face);
  }
}

enum AppearanceChoice {
  system,
  light,
  dark;

  ThemeData resolve(Brightness platform) {
    final bool dark = switch (this) {
      system => platform == Brightness.dark,
      light => false,
      AppearanceChoice.dark => true,
    };
    return dark ? ThemeData.studioDark() : ThemeData.studioLight();
  }
}

class Settings {
  const Settings({
    this.locale = LocaleChoice.system,
    this.font = FontChoice.theme,
    this.appearance = AppearanceChoice.system,
  });

  /// Reads `FLUTTER_MAIL_LOCALE`, `FLUTTER_MAIL_FONT` and
  /// `FLUTTER_MAIL_APPEARANCE`, each the name of a value — `zhHantTW`,
  /// `notoSansSC`, `dark`.
  factory Settings.fromEnvironment(Map<String, String> env) {
    T? named<T extends Enum>(List<T> values, String key) {
      final String? name = env[key];
      for (final value in values) {
        if (value.name == name) return value;
      }
      return null;
    }

    return const Settings().copyWith(
      locale: named(LocaleChoice.values, 'FLUTTER_MAIL_LOCALE'),
      font: named(FontChoice.values, 'FLUTTER_MAIL_FONT'),
      appearance: named(AppearanceChoice.values, 'FLUTTER_MAIL_APPEARANCE'),
    );
  }

  final LocaleChoice locale;
  final FontChoice font;
  final AppearanceChoice appearance;

  Settings copyWith({
    LocaleChoice? locale,
    FontChoice? font,
    AppearanceChoice? appearance,
  }) {
    return Settings(
      locale: locale ?? this.locale,
      font: font ?? this.font,
      appearance: appearance ?? this.appearance,
    );
  }
}
