import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../artwork.dart';
import '../catalog.dart';
import '../player_controller.dart';
import 'common.dart';

class CollectionView extends StatelessWidget {
  const CollectionView({
    super.key,
    required this.player,
    required this.collection,
    required this.likedOnly,
    required this.query,
    required this.saved,
    required this.onSave,
    required this.onSelect,
  });

  final PlayerController player;
  final Collection? collection;
  final bool likedOnly;
  final String query;
  final bool saved;
  final VoidCallback onSave;
  final ValueChanged<Collection> onSelect;

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    final searching = query.trim().isNotEmpty;
    final home = collection == null && !likedOnly && !searching;
    final songs =
        (searching || likedOnly ? tracks : collection?.songs ?? tracks)
            .where((track) => !likedOnly || player.liked.contains(track.id))
            .where(
              (track) => '${track.title} ${track.artist} ${track.album}'
                  .toLowerCase()
                  .contains(query.trim().toLowerCase()),
            )
            .toList();
    final playlistPlaying = player.playing && player.hasQueue(songs);
    return Panel(
      child: LayoutBuilder(
        builder: (context, constraints) => SingleChildScrollView(
          key: ValueKey('${collection?.title}-$likedOnly-$searching'),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              if (searching || likedOnly)
                Padding(
                  padding: EdgeInsets.all(v.spacing6),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const MutedText('YOUR PERSONAL SOUNDTRACK', size: 10),
                      SizedBox(height: v.spacing4),
                      Text(
                        searching ? 'Search results' : 'Liked Songs',
                        style: const TextStyle(
                          fontSize: 36,
                          fontWeight: FontWeight.w800,
                          letterSpacing: -1.2,
                        ),
                      ),
                      SizedBox(height: v.spacing2),
                      MutedText(
                        searching
                            ? '${songs.length} results for “$query”'
                            : '${songs.length} songs you want to keep close.',
                      ),
                    ],
                  ),
                )
              else if (home)
                Padding(
                  padding: EdgeInsets.all(v.spacing6),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const MutedText('A MOMENT, JUST FOR YOU', size: 10),
                      SizedBox(height: v.spacing3),
                      const Text(
                        'Find your own rhythm.',
                        style: TextStyle(
                          fontSize: 38,
                          fontWeight: FontWeight.w800,
                          letterSpacing: -1.5,
                        ),
                      ),
                      SizedBox(height: v.spacing2),
                      const MutedText(
                        'Good sounds for whatever kind of day you’re having.',
                      ),
                      SizedBox(height: v.spacing6),
                      CollectionShelf(onSelect: onSelect),
                    ],
                  ),
                )
              else
                _CollectionHero(
                  collection: collection!,
                  compact: constraints.maxWidth < 620,
                ),
              Padding(
                padding: EdgeInsets.symmetric(
                  horizontal: v.spacing6,
                  vertical: v.spacing5,
                ),
                child: Row(
                  children: [
                    PlayerIcon(
                      label: playlistPlaying
                          ? 'Pause playlist'
                          : 'Play playlist',
                      icon: playlistPlaying
                          ? FluentIcons.pause_24_filled
                          : FluentIcons.play_24_filled,
                      filled: true,
                      size: WidgetSize.large,
                      onPressed: songs.isEmpty
                          ? null
                          : () {
                              if (player.hasQueue(songs)) {
                                player.toggle();
                              } else {
                                player.play(
                                  songs.contains(player.current)
                                      ? player.current
                                      : songs.first,
                                  from: songs,
                                );
                              }
                            },
                    ),
                    SizedBox(width: v.spacing4),
                    PlayerIcon(
                      label: 'Shuffle',
                      icon: FluentIcons.arrow_shuffle_24_regular,
                      active: player.shuffle,
                      onPressed: player.toggleShuffle,
                    ),
                    if (!searching && !likedOnly && !home) ...[
                      SizedBox(width: v.spacing3),
                      PlayerIcon(
                        label: saved
                            ? 'Remove playlist from library'
                            : 'Save playlist to library',
                        icon: saved
                            ? FluentIcons.checkmark_circle_24_regular
                            : FluentIcons.add_circle_24_regular,
                        active: saved,
                        onPressed: onSave,
                      ),
                    ],
                    const Spacer(),
                    if (constraints.maxWidth > 420)
                      const MutedText('LOCAL ORIGINALS', size: 10),
                  ],
                ),
              ),
              Padding(
                padding: EdgeInsets.symmetric(horizontal: v.spacing6),
                child: Column(
                  children: [
                    Padding(
                      padding: EdgeInsets.fromLTRB(
                        v.spacing3,
                        0,
                        v.spacing3,
                        v.spacing3,
                      ),
                      child: Row(
                        children: [
                          const SizedBox(width: 30, child: MutedText('#')),
                          const Expanded(flex: 5, child: MutedText('Title')),
                          if (constraints.maxWidth > 590)
                            const Expanded(flex: 3, child: MutedText('Album')),
                          const SizedBox(width: 38),
                          SizedBox(
                            width: 40,
                            child: Icon(
                              FluentIcons.clock_24_regular,
                              size: 16,
                              color: v.colorContentSecondary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    Container(height: v.strokeHairline, color: v.colorBorder),
                    SizedBox(height: v.spacing2),
                    if (songs.isEmpty)
                      Padding(
                        padding: EdgeInsets.symmetric(vertical: v.spacing16),
                        child: Text(
                          searching
                              ? 'No songs found. Try an artist or album name.'
                              : 'Your next favourite is out there. Tap a heart to save it.',
                          textAlign: TextAlign.center,
                          style: TextStyle(color: v.colorContentSecondary),
                        ),
                      ),
                    for (var index = 0; index < songs.length; index++)
                      TrackRow(
                        track: songs[index],
                        index: index + 1,
                        current: songs[index] == player.current,
                        playing: player.playing,
                        liked: player.liked.contains(songs[index].id),
                        showAlbum: constraints.maxWidth > 590,
                        onPlay: () => player.play(songs[index], from: songs),
                        onLike: () => player.toggleLike(songs[index]),
                      ),
                  ],
                ),
              ),
              if (!searching && !likedOnly && !home)
                Padding(
                  padding: EdgeInsets.all(v.spacing6),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      SizedBox(height: v.spacing3),
                      const Text(
                        'Stay in the flow',
                        style: TextStyle(
                          fontWeight: FontWeight.w700,
                          fontSize: 21,
                          letterSpacing: -0.5,
                        ),
                      ),
                      SizedBox(height: v.spacing1),
                      const MutedText('A few more places to get lost in.'),
                      SizedBox(height: v.spacing5),
                      CollectionShelf(onSelect: onSelect, exclude: collection),
                    ],
                  ),
                ),
              Padding(
                padding: EdgeInsets.all(v.spacing6),
                child: const MutedText(
                  'Made with DazzUI. Played at your pace.',
                  size: 10,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _CollectionHero extends StatelessWidget {
  const _CollectionHero({required this.collection, required this.compact});
  final Collection collection;
  final bool compact;

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    return Container(
      padding: EdgeInsets.all(v.spacing6),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            Color.alphaBlend(
              artworkPalettes[collection.art][0].withValues(alpha: 0.30),
              v.colorSurface,
            ),
            v.colorSurface,
          ],
        ),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Container(
            decoration: BoxDecoration(boxShadow: v.shadowLg),
            child: Artwork(
              index: collection.art,
              size: compact ? 116 : 176,
              title: collection.title,
              radius: v.radiusTiny,
            ),
          ),
          SizedBox(width: compact ? v.spacing4 : v.spacing6),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  'MADE FOR YOUR EVERYDAY',
                  style: TextStyle(
                    fontSize: 9,
                    letterSpacing: 1.7,
                    fontWeight: FontWeight.w600,
                    color: v.colorContentSecondary,
                  ),
                ),
                SizedBox(height: v.spacing3),
                FittedBox(
                  fit: BoxFit.scaleDown,
                  alignment: Alignment.centerLeft,
                  child: Text(
                    collection.title,
                    style: TextStyle(
                      fontSize: compact ? 36 : 66,
                      height: 1.05,
                      letterSpacing: -2.6,
                      fontWeight: FontWeight.w800,
                    ),
                  ),
                ),
                SizedBox(height: v.spacing3),
                Text(
                  collection.subtitle,
                  style: TextStyle(
                    fontSize: 12,
                    color: v.colorContentSecondary,
                    height: 1.5,
                  ),
                ),
                SizedBox(height: v.spacing4),
                Wrap(
                  spacing: v.spacing2,
                  runSpacing: v.spacing1,
                  crossAxisAlignment: WrapCrossAlignment.center,
                  children: [
                    const SpotifyMark(size: 18),
                    const Text(
                      'DazzUI',
                      style: TextStyle(
                        fontWeight: FontWeight.w700,
                        fontSize: 12,
                      ),
                    ),
                    MutedText(
                      '· ${collection.songs.length} songs, ${timestamp(Duration(seconds: collection.songs.length * 24))}',
                      size: 11,
                    ),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class TrackRow extends StatelessWidget {
  const TrackRow({
    super.key,
    required this.track,
    required this.index,
    required this.current,
    required this.playing,
    required this.liked,
    required this.showAlbum,
    required this.onPlay,
    required this.onLike,
  });
  final Track track;
  final int index;
  final bool current;
  final bool playing;
  final bool liked;
  final bool showAlbum;
  final VoidCallback onPlay;
  final VoidCallback onLike;

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    return Pressable(
      semanticsLabel: 'Play ${track.title}',
      selected: current,
      onPressed: onPlay,
      borderRadius: BorderRadius.circular(v.radiusTiny),
      builder: (context, states) {
        final hovered = states.contains(WidgetState.hovered);
        return Container(
          height: 58,
          padding: EdgeInsets.symmetric(horizontal: v.spacing3),
          decoration: BoxDecoration(
            color: hovered || current ? v.colorSurfaceSubtle : null,
            borderRadius: BorderRadius.circular(v.radiusTiny),
          ),
          child: Row(
            children: [
              SizedBox(
                width: 30,
                child: current && playing
                    ? Icon(
                        FluentIcons.data_bar_vertical_24_filled,
                        color: v.colorPrimary[500],
                        size: 16,
                      )
                    : hovered
                    ? Icon(
                        FluentIcons.play_24_filled,
                        size: 14,
                        color: v.colorContent,
                      )
                    : MutedText('$index'),
              ),
              Expanded(
                flex: 5,
                child: Row(
                  children: [
                    Artwork(index: track.art, size: 38, radius: v.radiusTiny),
                    SizedBox(width: v.spacing3),
                    Expanded(
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            track.title,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(
                              fontSize: 13,
                              fontWeight: FontWeight.w500,
                              color: current
                                  ? v.colorPrimary[500]
                                  : v.colorContent,
                            ),
                          ),
                          SizedBox(height: v.spacing1),
                          MutedText(track.artist, size: 11),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
              if (showAlbum)
                Expanded(flex: 3, child: MutedText(track.album, size: 12)),
              SizedBox(
                width: 38,
                child: PlayerIcon(
                  label: '${liked ? 'Unlike' : 'Like'} ${track.title}',
                  icon: liked
                      ? FluentIcons.heart_24_filled
                      : FluentIcons.heart_24_regular,
                  active: liked,
                  size: WidgetSize.tiny,
                  onPressed: onLike,
                ),
              ),
              SizedBox(
                width: 40,
                child: MutedText(timestamp(track.duration), size: 11),
              ),
            ],
          ),
        );
      },
    );
  }
}

class CollectionShelf extends StatelessWidget {
  const CollectionShelf({super.key, required this.onSelect, this.exclude});
  final ValueChanged<Collection> onSelect;
  final Collection? exclude;

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    return LayoutBuilder(
      builder: (context, constraints) {
        final items = collections.where((item) => item != exclude).toList();
        final columns = constraints.maxWidth < 480 ? 2 : items.length;
        final width =
            (constraints.maxWidth - v.spacing4 * (columns - 1)) / columns;
        return Wrap(
          spacing: v.spacing4,
          runSpacing: v.spacing4,
          children: [
            for (final item in items)
              SizedBox(
                width: width,
                child: Pressable(
                  semanticsLabel: 'Open ${item.title}',
                  onPressed: () => onSelect(item),
                  borderRadius: BorderRadius.circular(v.radiusLarge),
                  builder: (context, states) => Container(
                    padding: EdgeInsets.all(v.spacing2),
                    decoration: BoxDecoration(
                      color: states.contains(WidgetState.hovered)
                          ? v.colorSurfaceMuted
                          : null,
                      borderRadius: BorderRadius.circular(v.radiusLarge),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Artwork(index: item.art, title: item.title),
                        SizedBox(height: v.spacing3),
                        Text(
                          item.title,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            fontWeight: FontWeight.w700,
                            fontSize: 13,
                          ),
                        ),
                        SizedBox(height: v.spacing1),
                        const MutedText('Playlist · DazzUI', size: 11),
                      ],
                    ),
                  ),
                ),
              ),
          ],
        );
      },
    );
  }
}
