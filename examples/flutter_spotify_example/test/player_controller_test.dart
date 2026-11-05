import 'dart:math';

import 'package:flutter_spotify_example/catalog.dart';
import 'package:flutter_spotify_example/player_controller.dart';
import 'package:flutter_test/flutter_test.dart';

import 'fake_transport.dart';

void main() {
  late FakeTransport audio;
  late PlayerController player;

  setUp(() {
    audio = FakeTransport();
    player = PlayerController(transport: audio, random: Random(42));
  });
  tearDown(() => player.dispose());

  test('plays, pauses and resumes the same local recording', () async {
    await player.toggle();
    expect(audio.audible, tracks.first);
    expect(player.playing, isTrue);
    await player.toggle();
    expect(audio.paused, isTrue);
    expect(player.playing, isFalse);
    await player.toggle();
    expect(audio.paused, isFalse);
    expect(player.playing, isTrue);
  });

  test('native position, seek bounds and previous restart agree', () async {
    await player.play(tracks[1]);
    audio.positionEvents.add(const Duration(seconds: 8));
    expect(player.position.inSeconds, 8);
    await player.previous();
    expect(player.current, tracks[1]);
    expect(audio.position, Duration.zero);
    await player.seek(const Duration(seconds: 90));
    expect(audio.position, tracks[1].duration);
    await player.seek(const Duration(seconds: -4));
    expect(audio.position, Duration.zero);
    await player.previous();
    expect(player.current, tracks[0]);
  });

  test('seeking before first play loads audio and stays paused', () async {
    await player.seek(const Duration(seconds: 8));
    expect(audio.audible, tracks[0]);
    expect(audio.paused, isTrue);
    expect(audio.position.inSeconds, 8);
    expect(player.playing, isFalse);
    await player.toggle();
    expect(player.playing, isTrue);
    expect(audio.position.inSeconds, 8);
  });

  test('completion follows playlist order and stops at its end', () async {
    final songs = collections[1].songs;
    await player.play(songs.first, from: songs);
    audio.completeEvents.add(null);
    await Future<void>.delayed(Duration.zero);
    expect(player.current, songs[1]);
    await player.play(songs.last);
    audio.completeEvents.add(null);
    await Future<void>.delayed(Duration.zero);
    expect(player.playing, isFalse);
    expect(player.current, songs.last);
    expect(player.upcoming, isEmpty);
  });

  test('repeat all wraps and repeat one replays the same song', () async {
    player.cycleRepeat();
    await player.play(tracks.last);
    audio.completeEvents.add(null);
    await Future<void>.delayed(Duration.zero);
    expect(player.current, tracks.first);
    player.cycleRepeat();
    audio.completeEvents.add(null);
    await Future<void>.delayed(Duration.zero);
    expect(player.current, tracks.first);
    expect(player.playing, isTrue);
  });

  test('shuffle skips current and previous follows actual history', () async {
    await player.play(tracks.first);
    player.toggleShuffle();
    final planned = player.upcoming.first;
    await player.next();
    final second = player.current;
    expect(second, planned);
    expect(second, isNot(tracks.first));
    await player.next();
    expect(player.current, isNot(second));
    await player.previous();
    expect(player.current, second);
    await player.previous();
    expect(player.current, tracks.first);
  });

  test('mute restores volume and failed playback can be retried', () async {
    await player.setVolume(0.35);
    await player.toggleMute();
    expect(audio.volume, 0);
    await player.toggleMute();
    expect(audio.volume, 0.35);
    audio.fail = true;
    await player.toggle();
    expect(player.error, isNotNull);
    expect(player.playing, isFalse);
    audio.fail = false;
    await player.toggle();
    expect(player.error, isNull);
    expect(player.playing, isTrue);
  });

  test(
    'rapid source changes leave native audio and UI on the same song',
    () async {
      await Future.wait([
        player.play(tracks[1]),
        player.play(tracks[4]),
        player.next(),
      ]);
      expect(player.current, tracks[5]);
      expect(audio.audible, player.current);
    },
  );
}
