import 'package:dazzui/dazzui.dart';

/// These are original demo recordings, not Spotify catalogue entries.
class Track {
  const Track(this.id, this.title, this.artist, this.album, this.art);

  final String id;
  final String title;
  final String artist;
  final String album;
  final int art;

  Duration get duration => const Duration(seconds: 24);
  String get asset => 'audio/$id.wav';
}

const tracks = [
  Track('first_light', 'First Light', 'Onda', 'Soft Horizons', 0),
  Track('weightless', 'Weightless', 'Low Season', 'Above the Clouds', 1),
  Track('slow_motion', 'Slow Motion', 'Still North', 'Blue Hours', 2),
  Track('warmth', 'Warmth', 'June & the Sea', 'Sundaze', 3),
  Track('night_swim', 'Night Swim', 'Studio Koi', 'After Hours', 4),
  Track('somewhere_else', 'Somewhere Else', 'Paper Planes', 'Open Spaces', 5),
];

class Collection {
  const Collection(this.title, this.subtitle, this.art, this.trackIds);

  final String title;
  final String subtitle;
  final int art;
  final List<int> trackIds;

  List<Track> get songs => [for (final index in trackIds) tracks[index]];
}

const collections = [
  Collection('Soft Focus', 'A little less noise. A little more you.', 0, [
    0,
    1,
    2,
    3,
    4,
    5,
  ]),
  Collection('After Hours', 'For when the world gets quiet.', 4, [4, 2, 1, 5]),
  Collection('Golden Hour', 'Keep the warm feeling a little longer.', 3, [
    3,
    0,
    5,
    1,
  ]),
  Collection('Quiet Mornings', 'An unhurried start to your day.', 1, [
    1,
    5,
    0,
    2,
  ]),
];

// Artwork colours are content, like the pixels of an album cover. All UI
// colours and controls use the existing generated Spotify theme unchanged.
const artworkPalettes = [
  [Color(0xFFB8C6AD), Color(0xFFF6D591), Color(0xFFCA724E), Color(0xFF444D3E)],
  [Color(0xFFACB9C9), Color(0xFFF1E7D2), Color(0xFF7586A0), Color(0xFF343E5B)],
  [Color(0xFF9ABEC3), Color(0xFFEAE1B4), Color(0xFF4D8293), Color(0xFF25475A)],
  [Color(0xFFD9A686), Color(0xFFF2DA93), Color(0xFFB86240), Color(0xFF6B4939)],
  [Color(0xFF807FA3), Color(0xFFE4BC98), Color(0xFF545B85), Color(0xFF2A334D)],
  [Color(0xFFA5B59B), Color(0xFFF0D4AF), Color(0xFF7D956D), Color(0xFF3D5B4D)],
];

String timestamp(Duration duration) =>
    '${duration.inMinutes}:${(duration.inSeconds % 60).toString().padLeft(2, '0')}';
