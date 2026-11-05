import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/cupertino.dart' show DefaultCupertinoLocalizations;

import 'artwork.dart';
import 'catalog.dart';
import 'player_controller.dart';
import 'widgets/collection_view.dart';
import 'widgets/common.dart';
import 'widgets/library.dart';
import 'widgets/now_playing.dart';
import 'widgets/player_bar.dart';

class SpotifyApp extends StatefulWidget {
  const SpotifyApp({super.key, this.player});

  /// An injected controller is owned by the caller.
  final PlayerController? player;

  @override
  State<SpotifyApp> createState() => _SpotifyAppState();
}

class _SpotifyAppState extends State<SpotifyApp> {
  late final PlayerController _player =
      widget.player ?? PlayerController(transport: LocalAudioTransport());
  bool _light = const String.fromEnvironment('SPOTIFY_APPEARANCE') == 'light';

  @override
  void dispose() {
    if (widget.player == null) _player.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = ThemeData(
      vars: _light ? themeVariablesSpotifyLight : themeVariablesSpotifyDark,
      brightness: _light ? Brightness.light : Brightness.dark,
    );
    return WidgetsApp(
      title: 'Spotify · DazzUI Sessions',
      color: theme.vars.colorCanvas,
      debugShowCheckedModeBanner: false,
      localizationsDelegates: const [DefaultCupertinoLocalizations.delegate],
      pageRouteBuilder: <T>(settings, builder) => PageRouteBuilder<T>(
        settings: settings,
        pageBuilder: (context, _, _) => builder(context),
      ),
      home: Theme(
        data: theme,
        child: SpotifyWindow(
          player: _player,
          light: _light,
          onAppearance: () => setState(() => _light = !_light),
        ),
      ),
    );
  }
}

class SpotifyWindow extends StatefulWidget {
  const SpotifyWindow({
    super.key,
    required this.player,
    required this.light,
    required this.onAppearance,
  });
  final PlayerController player;
  final bool light;
  final VoidCallback onAppearance;

  @override
  State<SpotifyWindow> createState() => _SpotifyWindowState();
}

class _SpotifyWindowState extends State<SpotifyWindow> {
  Collection? _collection = collections.first;
  bool _likedOnly = false;
  bool _aside = true;
  bool _queue = false;
  String _query = '';
  final Set<String> _saved = {
    for (final collection in collections) collection.title,
  };
  final TextEditingController _search = TextEditingController();

  @override
  void dispose() {
    _search.dispose();
    super.dispose();
  }

  void _select(Collection collection) => setState(() {
    _collection = collection;
    _likedOnly = false;
    _query = '';
    _search.clear();
  });

  void _home() => setState(() {
    _collection = null;
    _likedOnly = false;
    _query = '';
    _search.clear();
  });

  void _liked() => setState(() {
    _likedOnly = true;
    _query = '';
    _search.clear();
  });

  void _showPanel(double height, {required bool queue}) {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => SizedBox(
        width: 340,
        height: height * 0.8,
        child: ListenableBuilder(
          listenable: widget.player,
          builder: (context, _) => NowPlaying(
            player: widget.player,
            queueVisible: queue,
            onClose: () => Navigator.of(dialogContext).pop(),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    return ListenableBuilder(
      listenable: widget.player,
      builder: (context, _) => ColoredBox(
        color: v.colorCanvas,
        child: SafeArea(
          child: LayoutBuilder(
            builder: (context, constraints) {
              final wide = constraints.maxWidth >= 1100;
              final libraryVisible = constraints.maxWidth >= 880;
              final compact = constraints.maxWidth < 600;
              return Column(
                children: [
                  Padding(
                    padding: EdgeInsets.symmetric(
                      horizontal: compact ? v.spacing3 : v.spacing6,
                      vertical: v.spacing3,
                    ),
                    child: Row(
                      children: [
                        const SpotifyMark(),
                        if (!compact) ...[
                          SizedBox(width: v.spacing2),
                          const Text(
                            'Spotify',
                            style: TextStyle(
                              fontSize: 22,
                              fontWeight: FontWeight.w700,
                              letterSpacing: -1,
                            ),
                          ),
                        ],
                        SizedBox(width: compact ? v.spacing3 : v.spacing6),
                        PlayerIcon(
                          label: 'Home',
                          icon: FluentIcons.home_24_filled,
                          onPressed: _home,
                          active:
                              _collection == null &&
                              !_likedOnly &&
                              _query.isEmpty,
                          size: WidgetSize.medium,
                        ),
                        SizedBox(width: v.spacing2),
                        Expanded(
                          child: Align(
                            alignment: Alignment.centerLeft,
                            child: ConstrainedBox(
                              constraints: const BoxConstraints(maxWidth: 420),
                              child: SearchField(
                                controller: _search,
                                placeholder: compact
                                    ? 'Search music'
                                    : 'What do you want to listen to?',
                                size: WidgetSize.medium,
                                onChanged: (value) =>
                                    setState(() => _query = value),
                              ),
                            ),
                          ),
                        ),
                        SizedBox(width: v.spacing3),
                        if (!libraryVisible)
                          PlayerIcon(
                            label: 'Liked Songs',
                            icon: FluentIcons.heart_24_regular,
                            active: _likedOnly,
                            onPressed: _liked,
                          ),
                        if (wide)
                          Padding(
                            padding: EdgeInsets.only(right: v.spacing4),
                            child: const MutedText(
                              'YOUR DAILY DOSE OF GOOD SOUND',
                              size: 9,
                            ),
                          ),
                        PlayerIcon(
                          key: const Key('appearance-toggle'),
                          label: widget.light
                              ? 'Switch to dark appearance'
                              : 'Switch to light appearance',
                          icon: widget.light
                              ? FluentIcons.weather_moon_24_regular
                              : FluentIcons.weather_sunny_24_regular,
                          onPressed: widget.onAppearance,
                        ),
                        if (!compact) SizedBox(width: v.spacing2),
                        PlayerIcon(
                          label: 'Toggle now playing',
                          icon: FluentIcons.panel_right_24_regular,
                          active: wide && _aside,
                          onPressed: () {
                            if (wide) {
                              setState(() => _aside = !_aside);
                            } else {
                              _showPanel(constraints.maxHeight, queue: true);
                            }
                          },
                        ),
                      ],
                    ),
                  ),
                  Expanded(
                    child: Padding(
                      padding: EdgeInsets.symmetric(horizontal: v.spacing2),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          if (libraryVisible) ...[
                            SizedBox(
                              width: wide ? 246 : 218,
                              child: Library(
                                selected: _collection,
                                likedOnly: _likedOnly,
                                likedCount: widget.player.liked.length,
                                saved: _saved,
                                onSelect: _select,
                                onLiked: _liked,
                              ),
                            ),
                            SizedBox(width: v.spacing2),
                          ],
                          Expanded(
                            child: CollectionView(
                              player: widget.player,
                              collection: _collection,
                              likedOnly: _likedOnly,
                              query: _query,
                              saved: _saved.contains(_collection?.title),
                              onSave: () => setState(() {
                                final title = _collection?.title;
                                if (title != null && !_saved.add(title)) {
                                  _saved.remove(title);
                                }
                              }),
                              onSelect: _select,
                            ),
                          ),
                          if (wide && _aside) ...[
                            SizedBox(width: v.spacing2),
                            SizedBox(
                              width: 278,
                              child: NowPlaying(
                                player: widget.player,
                                queueVisible: _queue,
                                onClose: () => setState(() => _aside = false),
                              ),
                            ),
                          ],
                        ],
                      ),
                    ),
                  ),
                  if (widget.player.error != null)
                    Padding(
                      padding: EdgeInsets.all(v.spacing2),
                      child: Text(
                        widget.player.error!,
                        style: TextStyle(
                          color: v.colorDanger[500],
                          fontSize: 12,
                        ),
                      ),
                    ),
                  PlayerBar(
                    player: widget.player,
                    queueVisible: _queue && _aside,
                    onQueue: () {
                      if (wide) {
                        setState(() {
                          _queue = !(_queue && _aside);
                          _aside = true;
                        });
                      } else {
                        _showPanel(constraints.maxHeight, queue: true);
                      }
                    },
                  ),
                ],
              );
            },
          ),
        ),
      ),
    );
  }
}
