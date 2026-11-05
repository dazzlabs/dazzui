import 'package:dazzui/dazzui.dart' hide RepeatMode;
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../artwork.dart';
import '../catalog.dart';
import '../player_controller.dart';
import 'common.dart';

class PlayerBar extends StatelessWidget {
  const PlayerBar({
    super.key,
    required this.player,
    required this.queueVisible,
    required this.onQueue,
  });
  final PlayerController player;
  final bool queueVisible;
  final VoidCallback onQueue;

  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, constraints) {
      final v = Theme.of(context).vars;
      final compact = constraints.maxWidth < 760;
      final track = player.current;
      final identity = Row(
        children: [
          Artwork(
            index: track.art,
            size: compact ? 42 : 52,
            radius: v.radiusTiny,
          ),
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
                  style: const TextStyle(
                    fontWeight: FontWeight.w600,
                    fontSize: 12,
                  ),
                ),
                SizedBox(height: v.spacing1),
                MutedText(track.artist, size: 10),
              ],
            ),
          ),
          PlayerIcon(
            label: 'Like current song',
            icon: player.liked.contains(track.id)
                ? FluentIcons.heart_24_filled
                : FluentIcons.heart_24_regular,
            active: player.liked.contains(track.id),
            onPressed: () => player.toggleLike(track),
            size: WidgetSize.tiny,
          ),
        ],
      );
      final controls = Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          if (!compact) ...[
            PlayerIcon(
              label: 'Shuffle playback',
              icon: FluentIcons.arrow_shuffle_24_regular,
              active: player.shuffle,
              onPressed: player.toggleShuffle,
            ),
            SizedBox(width: v.spacing3),
          ],
          PlayerIcon(
            label: 'Previous track',
            icon: FluentIcons.previous_24_filled,
            onPressed: player.previous,
          ),
          SizedBox(width: v.spacing3),
          PlayerIcon(
            key: const Key('playback-toggle'),
            label: player.playing ? 'Pause' : 'Play',
            icon: player.playing
                ? FluentIcons.pause_24_filled
                : FluentIcons.play_24_filled,
            filled: true,
            size: WidgetSize.medium,
            onPressed: player.busy ? null : player.toggle,
          ),
          SizedBox(width: v.spacing3),
          PlayerIcon(
            label: 'Next track',
            icon: FluentIcons.next_24_filled,
            onPressed: player.next,
          ),
          if (!compact) ...[
            SizedBox(width: v.spacing3),
            PlayerIcon(
              label: 'Repeat: ${player.repeat.name}',
              icon: player.repeat == RepeatMode.one
                  ? FluentIcons.arrow_repeat_1_24_regular
                  : FluentIcons.arrow_repeat_all_24_regular,
              active: player.repeat != RepeatMode.off,
              onPressed: player.cycleRepeat,
            ),
          ],
        ],
      );
      final progress = _PlaybackTimeline(player: player);
      return Padding(
        padding: EdgeInsets.fromLTRB(
          v.spacing4,
          v.spacing3,
          v.spacing4,
          v.spacing2,
        ),
        child: compact
            ? Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Row(
                    children: [
                      Expanded(child: identity),
                      controls,
                    ],
                  ),
                  SizedBox(height: v.spacing1),
                  progress,
                ],
              )
            : SizedBox(
                height: 82,
                child: Row(
                  children: [
                    SizedBox(
                      width: constraints.maxWidth * 0.23,
                      child: identity,
                    ),
                    Expanded(
                      child: Padding(
                        padding: EdgeInsets.symmetric(horizontal: v.spacing6),
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            controls,
                            SizedBox(height: v.spacing1),
                            progress,
                          ],
                        ),
                      ),
                    ),
                    SizedBox(
                      width: constraints.maxWidth * 0.23,
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: [
                          PlayerIcon(
                            label: 'Show queue',
                            icon: FluentIcons.text_bullet_list_24_regular,
                            active: queueVisible,
                            onPressed: onQueue,
                          ),
                          SizedBox(width: v.spacing3),
                          PlayerIcon(
                            label: player.volume == 0 ? 'Unmute' : 'Mute',
                            icon: player.volume == 0
                                ? FluentIcons.speaker_mute_24_regular
                                : FluentIcons.speaker_2_24_regular,
                            onPressed: player.toggleMute,
                          ),
                          SizedBox(
                            width: constraints.maxWidth > 1100 ? 100 : 64,
                            child: Slider(
                              key: const Key('volume-slider'),
                              values: [player.volume * 100],
                              size: WidgetSize.small,
                              semanticsLabel: 'Volume',
                              onChanged: (values) =>
                                  player.setVolume(values.single / 100),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
      );
    },
  );
}

/// Preview a drag immediately and commit one native seek on release. Sending
/// every pointer sample to the audio backend would queue up stale seeks.
class _PlaybackTimeline extends StatefulWidget {
  const _PlaybackTimeline({required this.player});
  final PlayerController player;

  @override
  State<_PlaybackTimeline> createState() => _PlaybackTimelineState();
}

class _PlaybackTimelineState extends State<_PlaybackTimeline> {
  Duration? _draft;
  late String _trackId = widget.player.current.id;

  @override
  void didUpdateWidget(covariant _PlaybackTimeline oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (_trackId != widget.player.current.id) {
      _draft = null;
      _trackId = widget.player.current.id;
    }
  }

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    final player = widget.player;
    final position = _draft ?? player.position;
    return Row(
      children: [
        SizedBox(width: 34, child: MutedText(timestamp(position), size: 10)),
        Expanded(
          child: Slider(
            key: const Key('playback-progress'),
            values: [position.inMilliseconds.toDouble()],
            max: player.current.duration.inMilliseconds.toDouble(),
            step: 100,
            largeStep: 3000,
            size: WidgetSize.small,
            semanticsLabel: 'Playback position',
            onChanged: (values) => setState(() {
              _draft = Duration(milliseconds: values.single.round());
            }),
            onChangeEnd: (values) async {
              await player.seek(Duration(milliseconds: values.single.round()));
              if (mounted) setState(() => _draft = null);
            },
          ),
        ),
        SizedBox(width: v.spacing2),
        SizedBox(
          width: 30,
          child: MutedText(timestamp(player.current.duration), size: 10),
        ),
      ],
    );
  }
}
