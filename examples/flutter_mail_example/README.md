# flutter_mail_example

A mail client drawn with `dazzui`, for checking how Chinese text looks on
each desktop: macOS, Windows, and Linux (GNOME, KDE, Hyprland and others).

It works like a real app: folders, a message list, a reader, a reply box and
a compose dialog. Every message hides a case that some desktop gets wrong:

| Message | What it checks |
| --- | --- |
| 第 42 周产品周会纪要 | Chinese mixed with Latin text and numbers, `——` `……` `《》` `“”`, bold subjects, ellipsis truncation in the list |
| 關於下週發表會… | Traditional (Taiwan): 直 骨 角 in TW forms, punctuation the font sets in the middle of the box |
| 搬辦公室嘅安排 | Hong Kong Cantonese: 𨋢 㗎 嘅 啲, attachment tags |
| 【ご確認】フォント表示… | Japanese text in a Chinese locale: the same code points drawn with Chinese glyphs |
| 欢迎新同事 | Characters outside the BMP (𠮷 𰻝 𪚥), Ext-A, pinyin tone marks, emoji |
| 读书会：《千字文》 | A thousand-character text, to check font coverage and line height |
| 构建失败 / 9 月账单 | Monospace blocks with CJK, full-width and half-width digits, ％ and ￥ |

The status bar at the bottom holds three settings:

- **Language**: switches the window between Simplified Chinese, Traditional
  Chinese (Taiwan and Hong Kong), Japanese and English. The same locale picks
  the Simplified, Traditional or Japanese glyph when a font falls back. The
  messages stay as written, since they are the test data. Under the theme's
  own font (which names PingFang SC, then Noto Sans CJK SC) the glyph forms
  stay Simplified; pick **System default** to see them follow the language.
- **Font**: the theme's own font, the system default, or a named CJK family.
- **Appearance**: light, dark, or follow the system.

The interface strings live in `lib/l10n.dart`.

Next to them are a one-line environment summary and two buttons. The info
button opens the full report: platform, desktop, session, input method,
locales, scale. The camera button saves the window as a PNG to `~/Pictures`.

The window title is Chinese too, so the window manager's own title bar gets
checked as well.

## Run

```bash
flutter run -d macos   # or -d windows, -d linux
```

## Snapshots without clicking

Set `FLUTTER_MAIL_SNAPSHOT` and the app draws, writes the window to that PNG
and quits. This makes it easy to collect shots from several machines over SSH.

```bash
FLUTTER_MAIL_SNAPSHOT=/tmp/taipei.png \
FLUTTER_MAIL_OPEN=taipei \
FLUTTER_MAIL_LOCALE=zhHantTW \
FLUTTER_MAIL_FONT=notoSansSC \
FLUTTER_MAIL_APPEARANCE=dark \
  build/linux/x64/release/bundle/flutter_mail_example
```

| Variable | Values |
| --- | --- |
| `FLUTTER_MAIL_OPEN` | `weekly` `taipei` `hongkong` `tokyo` `welcome` `bookclub` `ci` `bill` |
| `FLUTTER_MAIL_LOCALE` | `system` `zhHansCN` `zhHantTW` `zhHantHK` `jaJP` `enUS` |
| `FLUTTER_MAIL_FONT` | `theme` `system` `notoSansSC` `notoSerifSC` `sourceHanSansSC` `pingFangSC` `microsoftYaHei` `wenQuanYi` |
| `FLUTTER_MAIL_APPEARANCE` | `system` `light` `dark` |

The macOS app runs without the App Sandbox, so that it can write snapshots
wherever it is told to.
