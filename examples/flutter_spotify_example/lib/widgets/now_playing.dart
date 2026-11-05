import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../artwork.dart';
import '../player_controller.dart';
import 'common.dart';

class NowPlaying extends StatelessWidget {
  const NowPlaying({
    super.key,
    required this.player,
    required this.queueVisible,
    required this.onClose,
  });
  final PlayerController player;
  final bool queueVisible;
  final VoidCallback onClose;

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    final track = player.current;
    return Panel(
      child: ListView(
        padding: EdgeInsets.all(v.spacing4),
        children: [
          Row(
            children: [
              Expanded(
                child: Text(
                  queueVisible ? 'Queue' : 'Now playing',
                  style: const TextStyle(
                    fontSize: 14,
                    fontWeight: FontWeight.w700,
                  ),
                ),
              ),
              PlayerIcon(
                label: 'Close now playing',
                icon: FluentIcons.dismiss_24_regular,
                onPressed: onClose,
                size: WidgetSize.tiny,
              ),
            ],
          ),
          SizedBox(height: v.spacing4),
          if (!queueVisible) ...[
            Artwork(index: track.art, title: track.album),
            SizedBox(height: v.spacing5),
          ],
          Row(
            children: [
              if (queueVisible) ...[
                Artwork(index: track.art, size: 48),
                SizedBox(width: v.spacing3),
              ],
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      track.title,
                      maxLines: 2,
                      style: TextStyle(
                        fontSize: queueVisible ? 16 : 23,
                        fontWeight: FontWeight.w800,
                        letterSpacing: -0.7,
                      ),
                    ),
                    SizedBox(height: v.spacing1),
                    MutedText(track.artist, size: 13),
                  ],
                ),
              ),
              PlayerIcon(
                label: 'Save current song',
                icon: player.liked.contains(track.id)
                    ? FluentIcons.heart_24_filled
                    : FluentIcons.heart_24_regular,
                active: player.liked.contains(track.id),
                onPressed: () => player.toggleLike(track),
              ),
            ],
          ),
          SizedBox(height: v.spacing5),
          if (!queueVisible)
            Container(
              padding: EdgeInsets.all(v.spacing4),
              decoration: BoxDecoration(
                color: v.colorSurfaceMuted,
                borderRadius: BorderRadius.circular(v.radiusLarge),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Behind the sound',
                    style: TextStyle(fontWeight: FontWeight.w700, fontSize: 12),
                  ),
                  SizedBox(height: v.spacing3),
                  Text(
                    'A small escape in sound. Warm keys, soft textures, and a little space to breathe.',
                    style: TextStyle(
                      color: v.colorContentSecondary,
                      fontSize: 12,
                      height: 1.7,
                    ),
                  ),
                  SizedBox(height: v.spacing4),
                  const MutedText('Original 24-second studio sketch', size: 10),
                ],
              ),
            ),
          SizedBox(height: v.spacing6),
          Row(
            children: [
              Expanded(
                child: Text(
                  queueVisible ? 'Next in queue' : 'Up next',
                  style: const TextStyle(
                    fontSize: 13,
                    fontWeight: FontWeight.w700,
                  ),
                ),
              ),
              MutedText('${player.upcoming.length} tracks', size: 10),
            ],
          ),
          SizedBox(height: v.spacing3),
          if (player.upcoming.isEmpty)
            const MutedText(
              'You’ve reached the end of this playlist.',
              size: 11,
            ),
          for (final next in player.upcoming.take(queueVisible ? 6 : 2))
            Padding(
              padding: EdgeInsets.only(bottom: v.spacing2),
              child: Pressable(
                semanticsLabel: 'Play next ${next.title}',
                onPressed: () => player.play(next),
                borderRadius: BorderRadius.circular(v.radiusTiny),
                builder: (context, states) => Container(
                  padding: EdgeInsets.all(v.spacing2),
                  decoration: BoxDecoration(
                    color: states.contains(WidgetState.hovered)
                        ? v.colorSurfaceSubtle
                        : null,
                    borderRadius: BorderRadius.circular(v.radiusTiny),
                  ),
                  child: Row(
                    children: [
                      Artwork(index: next.art, size: 38, radius: v.radiusTiny),
                      SizedBox(width: v.spacing3),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              next.title,
                              maxLines: 1,
                              overflow: TextOverflow.ellipsis,
                              style: const TextStyle(
                                fontSize: 12,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                            SizedBox(height: v.spacing1),
                            MutedText(next.artist, size: 10),
                          ],
                        ),
                      ),
                      Icon(
                        FluentIcons.play_24_filled,
                        size: 12,
                        color: v.colorContentSecondary,
                      ),
                    ],
                  ),
                ),
              ),
            ),
        ],
      ),
    );
  }
}
