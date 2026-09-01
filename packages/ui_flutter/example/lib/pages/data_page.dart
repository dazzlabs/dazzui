import 'package:dazzui/dazzui.dart';

import '../gallery.dart';

/// Surfaces and the things that sit on them: cards, the divided list that
/// stands in for a table, and the loaders.
class DataPage extends StatefulWidget {
  const DataPage({super.key});

  @override
  State<DataPage> createState() => _DataPageState();
}

class _DataPageState extends State<DataPage> {
  String _selected = 'aurora';

  static const _builds = [
    (id: 'aurora', branch: 'main', status: 'Passed', tint: BadgeTint.success),
    (
      id: 'beacon',
      branch: 'release/2.1',
      status: 'Running',
      tint: BadgeTint.info,
    ),
    (
      id: 'cinder',
      branch: 'fix/tokens',
      status: 'Failed',
      tint: BadgeTint.danger,
    ),
  ];

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing6,
      children: [
        const PageTitle(
          'Data',
          'A card is a surface; a row is a surface that answers the pointer.',
        ),
        Specimen(
          'Card',
          child: Card(
            child: Padding(
              padding: EdgeInsets.all(vars.spacing4),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                spacing: vars.spacing1,
                children: [
                  Text(
                    'Raised',
                    style: vars.labelMedium.copyWith(color: vars.colorContent),
                  ),
                  Text(
                    'A surface.',
                    style: vars.captionSmall.copyWith(
                      color: vars.colorContentMuted,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
        Specimen(
          'Rows',
          note: 'the list tile answers the pointer; the badge only annotates',
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: vars.dialogWidth),
            child: ListSection.insetGrouped(
              header: const Text('Builds'),
              footer: Text('Selected: $_selected'),
              children: [
                for (final build in _builds)
                  ListTile(
                    title: Text(build.id),
                    subtitle: Text(build.branch),
                    trailing: Badge(
                      size: WidgetSize.small,
                      variant: BadgeVariant.tinted,
                      tint: build.tint,
                      child: Text(build.status),
                    ),
                    onTap: () => setState(() => _selected = build.id),
                  ),
              ],
            ),
          ),
        ),
        Specimen(
          'Divided rows',
          note: 'one hairline between neighbours, none at the edges',
          child: Card(
            child: SizedBox(
              width: vars.dialogWidth / 2,
              child: DividedColumn(
                divider: const Divider(height: 1),
                children: [
                  for (final build in _builds)
                    Padding(
                      padding: EdgeInsets.all(vars.spacing3),
                      child: Row(
                        children: [
                          Text(
                            build.id,
                            style: vars.labelMedium.copyWith(
                              color: vars.colorContent,
                            ),
                          ),
                          const Spacer(),
                          Flexible(
                            child: Text(
                              build.branch,
                              overflow: TextOverflow.ellipsis,
                              style: vars.captionSmall.copyWith(
                                color: vars.colorContentMuted,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                ],
              ),
            ),
          ),
        ),
        Specimen(
          'Loaders',
          child: RowWrap(
            spacing: vars.spacing4,
            children: [
              const Loader(),
              Loader(color: vars.colorContentMuted),
              Loader(color: vars.colorDanger),
            ],
          ),
        ),
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}
