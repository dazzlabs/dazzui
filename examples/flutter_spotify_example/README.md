# Spotify · DazzUI Sessions

A Spotify-inspired Flutter desktop player using DazzUI's generated
`spotify-dark` and `spotify-light` themes. Controls are DazzUI `Button`,
`IconButton`, `SearchField`, `Slider`, `Tooltip` and `Pressable`, hosted by
`WidgetsApp`. There is no Material dependency and no copied theme values.

## Run

From this directory, with Flutter stable and Dart 3.13 or later:

```bash
flutter pub get
flutter run -d macos
```

The project also includes Windows, Linux and web runners:

```bash
flutter run -d windows
flutter run -d linux
flutter run -d chrome
```

Linux's audioplayers backend needs GStreamer development packages; see the
[audioplayers Linux setup](https://github.com/bluefireteam/audioplayers/blob/main/packages/audioplayers_linux/README.md).

Dark is the default. The sun/moon button switches appearances without
resetting playback. To start in DazzUI's adapted light appearance:

```bash
flutter run -d macos --dart-define=SPOTIFY_APPEARANCE=light
```

## Explore

- Browse four playlists, open Home for recommendations, or search by title,
  artist and album. Hearts collect tracks in Liked Songs.
- Play and pause actual bundled audio, skip forward and back, seek, adjust
  volume and mute. Shuffle uses playback history for the previous button;
  repeat cycles through off, playlist and one song.
- Open the queue from the playback bar. The library and now-playing panels
  fold as the window narrows; the music list and player remain available.
- Buttons, library items and tracks support keyboard focus and activation.
  Sliders also support arrows, Home/End and Page Up/Page Down.

All six 24-second ambient sketches and vector sleeves are original example
content under the repository's MIT licence. The artist and album names are
fictional. Audio is synthesized by `tool/generate_audio.py`, committed in
`assets/audio/`, and played by
[audioplayers](https://pub.dev/packages/audioplayers). The running player
needs no network connection or account. This demo does not connect to
Spotify's catalogue or APIs. Likes, followed playlists and appearance are
held in memory for the current session.

To regenerate the audio, using only Python's standard library:

```bash
python3 tool/generate_audio.py
```

## Check

```bash
dart format --output=none --set-exit-if-changed lib test integration_test
flutter analyze
flutter test
flutter test integration_test/player_test.dart -d macos
flutter build web
```

Controller tests cover playback order, shuffle history, repeat, seek bounds,
mute, failed audio recovery and rapid source changes. Widget tests exercise
both appearances at 390, 800, 1000 and 1440 logical pixels, search and likes.
The native integration test plays the real assets, checks advancing and paused
positions, seeks, changes volume, skips and waits for automatic advancement.
It writes `dazzui-spotify-dark.png` and `dazzui-spotify-light.png` to the app's
temporary directory and prints their paths (or uses `SPOTIFY_SCREENSHOT_DIR`
when the app has permission to write to that directory).
