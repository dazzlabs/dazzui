import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../artwork.dart';
import '../catalog.dart';
import 'common.dart';

class Library extends StatelessWidget {
  const Library({
    super.key,
    required this.selected,
    required this.likedOnly,
    required this.likedCount,
    required this.saved,
    required this.onSelect,
    required this.onLiked,
  });

  final Collection? selected;
  final bool likedOnly;
  final int likedCount;
  final Set<String> saved;
  final ValueChanged<Collection> onSelect;
  final VoidCallback onLiked;

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    return Panel(
      child: Padding(
        padding: EdgeInsets.all(v.spacing3),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: EdgeInsets.fromLTRB(
                v.spacing2,
                v.spacing3,
                0,
                v.spacing5,
              ),
              child: Row(
                children: [
                  Icon(
                    FluentIcons.library_24_regular,
                    size: 22,
                    color: v.colorContentSecondary,
                  ),
                  SizedBox(width: v.spacing3),
                  const Expanded(
                    child: Text(
                      'Your Library',
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontWeight: FontWeight.w700,
                        fontSize: 15,
                      ),
                    ),
                  ),
                ],
              ),
            ),
            Padding(
              padding: EdgeInsets.only(left: v.spacing2, bottom: v.spacing5),
              child: Wrap(
                spacing: v.spacing2,
                runSpacing: v.spacing2,
                children: [
                  Button(
                    size: WidgetSize.small,
                    variant: likedOnly
                        ? ButtonVariant.recessed
                        : ButtonVariant.tinted,
                    onPressed: () => onSelect(collections.first),
                    child: const Text('Playlists'),
                  ),
                  Button(
                    size: WidgetSize.small,
                    variant: likedOnly
                        ? ButtonVariant.tinted
                        : ButtonVariant.plain,
                    onPressed: onLiked,
                    child: const Text('Liked'),
                  ),
                ],
              ),
            ),
            Expanded(
              child: ListView(
                children: [
                  _LibraryItem(
                    title: 'Liked Songs',
                    subtitle: '$likedCount songs',
                    selected: likedOnly,
                    onPressed: onLiked,
                    artwork: Container(
                      width: 46,
                      height: 46,
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(v.radiusTiny),
                        gradient: LinearGradient(
                          begin: Alignment.topLeft,
                          end: Alignment.bottomRight,
                          colors: [v.colorPrimary[700]!, v.colorPrimary[300]!],
                        ),
                      ),
                      child: Icon(
                        FluentIcons.heart_24_filled,
                        color: v.colorOnAccent,
                        size: 22,
                      ),
                    ),
                  ),
                  for (final collection in collections.where(
                    (item) => saved.contains(item.title),
                  ))
                    _LibraryItem(
                      title: collection.title,
                      subtitle: 'Playlist · DazzUI',
                      selected: !likedOnly && collection == selected,
                      onPressed: () => onSelect(collection),
                      artwork: Artwork(
                        index: collection.art,
                        size: 46,
                        radius: v.radiusTiny,
                      ),
                    ),
                ],
              ),
            ),
            Container(
              padding: EdgeInsets.all(v.spacing4),
              decoration: BoxDecoration(
                color: v.colorSurfaceMuted,
                borderRadius: BorderRadius.circular(v.radiusLarge),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        FluentIcons.music_note_2_24_regular,
                        size: 16,
                        color: v.colorPrimary[500],
                      ),
                      SizedBox(width: v.spacing2),
                      const Expanded(
                        child: Text(
                          'A space to tune out',
                          maxLines: 2,
                          style: TextStyle(
                            fontWeight: FontWeight.w700,
                            fontSize: 12,
                          ),
                        ),
                      ),
                    ],
                  ),
                  SizedBox(height: v.spacing2),
                  Text(
                    'Original sounds. No account needed.\nFind a little room for yourself.',
                    style: TextStyle(
                      color: v.colorContentSecondary,
                      fontSize: 11,
                      height: 1.65,
                    ),
                  ),
                ],
              ),
            ),
            Padding(
              padding: EdgeInsets.fromLTRB(
                v.spacing2,
                v.spacing5,
                0,
                v.spacing2,
              ),
              child: const MutedText('DAZZUI SESSIONS  /  VOL. 01', size: 9),
            ),
          ],
        ),
      ),
    );
  }
}

class _LibraryItem extends StatelessWidget {
  const _LibraryItem({
    required this.title,
    required this.subtitle,
    required this.selected,
    required this.artwork,
    required this.onPressed,
  });
  final String title;
  final String subtitle;
  final bool selected;
  final Widget artwork;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final v = Theme.of(context).vars;
    return Padding(
      padding: EdgeInsets.only(bottom: v.spacing1),
      child: Pressable(
        selected: selected,
        semanticsLabel: title,
        onPressed: onPressed,
        borderRadius: BorderRadius.circular(v.radiusTiny),
        builder: (context, states) => Container(
          padding: EdgeInsets.all(v.spacing2),
          decoration: BoxDecoration(
            color: selected
                ? v.colorSurfaceMuted
                : states.contains(WidgetState.hovered)
                ? v.colorSurfaceSubtle
                : null,
            borderRadius: BorderRadius.circular(v.radiusTiny),
          ),
          child: Row(
            children: [
              artwork,
              SizedBox(width: v.spacing3),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      title,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontWeight: FontWeight.w600,
                        color: selected ? v.colorPrimary[500] : v.colorContent,
                        fontSize: 13,
                      ),
                    ),
                    SizedBox(height: v.spacing1),
                    MutedText(subtitle, size: 11),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
