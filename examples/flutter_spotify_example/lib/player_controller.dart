import 'dart:async';
import 'dart:math';

import 'package:audioplayers/audioplayers.dart';
import 'package:flutter/foundation.dart';

import 'catalog.dart';

/// A small boundary so player state can be exercised without a native plugin.
abstract class AudioTransport {
  Stream<Duration> get positions;
  Stream<void> get completions;
  Future<void> play(Track track);
  Future<void> pause();
  Future<void> resume();
  Future<void> seek(Duration position);
  Future<void> setVolume(double volume);
  Future<void> dispose();
}

class LocalAudioTransport implements AudioTransport {
  LocalAudioTransport() {
    // Audio position keeps advancing when the window has no animation frames.
    // Five updates per second also avoid rebuilding the player at display Hz.
    _player.positionUpdater = TimerPositionUpdater(
      getPosition: _player.getCurrentPosition,
      interval: const Duration(milliseconds: 200),
    );
  }

  final AudioPlayer _player = AudioPlayer();

  @override
  Stream<Duration> get positions => _player.onPositionChanged;
  @override
  Stream<void> get completions => _player.onPlayerComplete;
  @override
  Future<void> play(Track track) => _player.play(AssetSource(track.asset));
  @override
  Future<void> pause() => _player.pause();
  @override
  Future<void> resume() => _player.resume();
  @override
  Future<void> seek(Duration position) => _player.seek(position);
  @override
  Future<void> setVolume(double volume) => _player.setVolume(volume);
  @override
  Future<void> dispose() => _player.dispose();
}

enum RepeatMode { off, all, one }

class PlayerController extends ChangeNotifier {
  PlayerController({required AudioTransport transport, Random? random})
    // Keep the public injection point named, rather than exposing a private
    // positional field in the constructor.
    // ignore: prefer_initializing_formals
    : _transport = transport,
      _random = random ?? Random() {
    _subscriptions.add(
      _transport.positions.listen((value) {
        if (_loading) return;
        position = value > current.duration ? current.duration : value;
        notifyListeners();
      }),
    );
    _subscriptions.add(
      _transport.completions.listen((_) => unawaited(_complete())),
    );
  }

  final AudioTransport _transport;
  final Random _random;
  final List<StreamSubscription<dynamic>> _subscriptions = [];
  final Set<String> liked = {'weightless', 'warmth'};
  final List<Track> _history = [];
  List<Track> _queue = List.of(tracks);
  List<Track> _shuffled = [];
  Track current = tracks.first;
  Duration position = Duration.zero;
  bool playing = false;
  bool shuffle = false;
  RepeatMode repeat = RepeatMode.off;
  double volume = 0.65;
  double _previousVolume = 0.65;
  String? error;
  bool _loaded = false;
  bool _loading = false;
  bool _disposed = false;
  Future<void> _pending = Future.value();

  List<Track> get queue => List.unmodifiable(_queue);
  bool hasQueue(List<Track> songs) => listEquals(_queue, songs);
  bool get busy => _loading;
  List<Track> get _order => shuffle ? _shuffled : _queue;
  List<Track> get upcoming {
    final index = _order.indexOf(current);
    return index < 0 ? List.of(_order) : _order.skip(index + 1).toList();
  }

  // Native source changes are serialized: rapid clicks cannot leave the UI
  // showing a different track from the one the audio backend finally loaded.
  Future<void> _run(Future<void> Function() operation) {
    _pending = _pending.then((_) async {
      if (_disposed) return;
      error = null;
      try {
        await operation();
      } catch (_) {
        playing = false;
        _loaded = false;
        error = 'Could not play this local track. Try playing it again.';
      } finally {
        _loading = false;
        if (!_disposed) notifyListeners();
      }
    });
    return _pending;
  }

  Future<void> _start(Track track) async {
    _loading = true;
    playing = false;
    position = Duration.zero;
    if (current != track) _history.add(current);
    current = track;
    _loaded = false;
    notifyListeners();
    await _transport.setVolume(volume);
    await _transport.play(track);
    _loaded = true;
    playing = true;
  }

  Future<void> play(Track track, {List<Track>? from}) => _run(() async {
    if (from != null && from.isNotEmpty) _queue = List.of(from);
    if (!_queue.contains(track)) _queue.add(track);
    if (shuffle && (from != null || !_shuffled.contains(track))) {
      _shuffleFrom(track);
    }
    await _start(track);
  });

  Future<void> toggle() => _run(() async {
    if (!_loaded) {
      await _start(current);
    } else if (playing) {
      await _transport.pause();
      playing = false;
    } else {
      await _transport.resume();
      playing = true;
    }
  });

  Track? _next({bool automatic = false}) {
    final index = _order.indexOf(current) + 1;
    if (index < _order.length) return _order[index];
    if (!automatic || repeat == RepeatMode.all) return _order.first;
    return null;
  }

  Future<void> next() => _run(() => _start(_next()!));

  Future<void> previous() => _run(() async {
    if (position.inSeconds > 3 && _loaded) {
      await _transport.seek(Duration.zero);
      position = Duration.zero;
    } else {
      final track = _history.isNotEmpty
          ? _history.removeLast()
          : _queue[(_queue.indexOf(current) - 1) % _queue.length];
      // Going back must not add the current song to forward history.
      current = track;
      await _start(track);
    }
  });

  Future<void> _complete() => _run(() async {
    final track = repeat == RepeatMode.one ? current : _next(automatic: true);
    if (track == null) {
      playing = false;
      _loaded = false;
      position = Duration.zero;
    } else {
      await _start(track);
    }
  });

  Future<void> seek(Duration value) => _run(() async {
    final clamped = Duration(
      milliseconds: value.inMilliseconds.clamp(
        0,
        current.duration.inMilliseconds,
      ),
    );
    if (!_loaded) {
      await _start(current);
      await _transport.pause();
      playing = false;
    }
    await _transport.seek(clamped);
    position = clamped;
  });

  Future<void> setVolume(double value) => _run(() async {
    volume = value.clamp(0, 1);
    if (volume > 0) _previousVolume = volume;
    await _transport.setVolume(volume);
  });

  Future<void> toggleMute() => setVolume(volume == 0 ? _previousVolume : 0);

  void toggleLike(Track track) {
    if (!liked.add(track.id)) liked.remove(track.id);
    notifyListeners();
  }

  void toggleShuffle() {
    shuffle = !shuffle;
    if (shuffle) _shuffleFrom(current);
    notifyListeners();
  }

  void _shuffleFrom(Track track) {
    final rest = _queue.where((item) => item != track).toList()
      ..shuffle(_random);
    _shuffled = [track, ...rest];
  }

  void cycleRepeat() {
    repeat = RepeatMode.values[(repeat.index + 1) % RepeatMode.values.length];
    notifyListeners();
  }

  @override
  void dispose() {
    _disposed = true;
    for (final subscription in _subscriptions) {
      unawaited(subscription.cancel());
    }
    // Finish a native operation before disposing its backend.
    unawaited(_pending.whenComplete(_transport.dispose));
    super.dispose();
  }
}
