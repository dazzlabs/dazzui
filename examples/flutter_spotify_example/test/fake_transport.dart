import 'dart:async';

import 'package:flutter_spotify_example/catalog.dart';
import 'package:flutter_spotify_example/player_controller.dart';

class FakeTransport implements AudioTransport {
  final positionEvents = StreamController<Duration>.broadcast(sync: true);
  final completeEvents = StreamController<void>.broadcast(sync: true);
  Track? audible;
  bool paused = true;
  bool fail = false;
  double volume = 1;
  Duration position = Duration.zero;
  int seekCount = 0;

  @override
  Stream<Duration> get positions => positionEvents.stream;
  @override
  Stream<void> get completions => completeEvents.stream;

  @override
  Future<void> play(Track track) async {
    if (fail) throw StateError('Audio device unavailable');
    audible = track;
    paused = false;
    position = Duration.zero;
  }

  @override
  Future<void> pause() async => paused = true;
  @override
  Future<void> resume() async => paused = false;
  @override
  Future<void> seek(Duration value) async {
    seekCount += 1;
    position = value;
  }

  @override
  Future<void> setVolume(double value) async => volume = value;

  @override
  Future<void> dispose() async {
    await positionEvents.close();
    await completeEvents.close();
  }
}
