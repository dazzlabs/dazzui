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
          'Table',
          note: 'hover is neutral so the tinted selection stays the heavier',
          // A table brings its own rules, so it is framed rather than carded.
          child: Container(
            clipBehavior: Clip.antiAlias,
            decoration: BoxDecoration(
              border: Border.all(
                color: vars.colorBorder,
                width: context.hairlineWidth,
              ),
              borderRadius: BorderRadius.circular(vars.radiusLarge),
            ),
            child: DataTable(
              children: [
                const TableHead(
                  children: [
                    TableCell(head: true, child: Text('Build')),
                    TableCell(head: true, child: Text('Branch')),
                    TableCell(
                      head: true,
                      align: TableCellAlign.end,
                      width: 96,
                      child: Text('Status'),
                    ),
                  ],
                ),
                for (final build in _builds)
                  TableRow(
                    active: _selected == build.id,
                    onPressed: () => setState(() => _selected = build.id),
                    children: [
                      TableCell(child: Text(build.id)),
                      TableCell(child: Text(build.branch)),
                      TableCell(
                        align: TableCellAlign.end,
                        width: 96,
                        child: Badge(
                          size: WidgetSize.small,
                          variant: BadgeVariant.tinted,
                          tint: build.tint,
                          child: Text(build.status),
                        ),
                      ),
                    ],
                  ),
              ],
            ),
          ),
        ),
        Specimen(
          'Progress',
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: vars.dialogWidth),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing3,
              children: const [
                Progress(value: 0.35),
                Progress(value: 0.7, size: WidgetSize.medium, gradient: true),
                // No value at all is the indeterminate sweep.
                Progress(size: WidgetSize.large, tint: ProgressTint.info),
              ],
            ),
          ),
        ),
        const Specimen(
          'Steps',
          child: StepList(
            children: [
              Step(status: StepStatus.done, label: 'Resolve the token graph'),
              Step(
                status: StepStatus.done,
                label: 'Write the platform sources',
              ),
              Step(
                status: StepStatus.active,
                label: 'Compile dazzui',
                meta: 'now',
              ),
              Step(status: StepStatus.idle, label: 'Publish the package'),
            ],
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
