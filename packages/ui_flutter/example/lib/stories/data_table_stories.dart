import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/DataTable` — rows of one thing.
///
/// Named `DataTable` rather than `Table` because Flutter's own `Table` is a
/// layout widget; the design's grid keeps the plain names for its parts.
class DataTableStories {
  DataTableStories._();

  static StoryPage get page => StoryPage('DataTable', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Selectable', (context) => const _Builds(interactive: true)),
    Story('Inert', (context) => const _Builds(interactive: false)),
    Story(
      'Alignment',
      (context) => const SizedBox(
        width: 380,
        child: DataTable(
          children: [
            TableHead(
              children: [
                TableCell(head: true, child: Text('Package')),
                TableCell(
                  head: true,
                  align: TableCellAlign.end,
                  child: Text('Tokens'),
                ),
              ],
            ),
            TableRow(
              children: [
                TableCell(child: Text('ui_react')),
                TableCell(align: TableCellAlign.end, child: Text('312')),
              ],
            ),
            TableRow(
              children: [
                TableCell(child: Text('ui_flutter')),
                TableCell(align: TableCellAlign.end, child: Text('312')),
              ],
            ),
          ],
        ),
      ),
    ),
    Story('With controls', (context) => const _WithControls()),
    Story(
      'Empty',
      (context) => const SizedBox(
        width: 420,
        child: Card(
          variant: CardVariant.sunken,
          child: EmptyState(title: 'No builds have run on this branch'),
        ),
      ),
    ),
  ]);
}

class _Build {
  const _Build(this.id, this.branch, this.status, this.tint);

  final String id;
  final String branch;
  final String status;
  final BadgeTint tint;

  static const List<_Build> all = [
    _Build('aurora', 'main', 'Passed', BadgeTint.success),
    _Build('beacon', 'release/2.1', 'Running', BadgeTint.info),
    _Build('cinder', 'fix/tokens', 'Failed', BadgeTint.danger),
  ];
}

class _Builds extends StatefulWidget {
  const _Builds({required this.interactive});

  final bool interactive;

  @override
  State<_Builds> createState() => _BuildsState();
}

class _BuildsState extends State<_Builds> {
  String _selected = 'aurora';

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 420,
      child: DataTable(
        children: [
          const TableHead(
            children: [
              TableCell(head: true, child: Text('Build')),
              TableCell(head: true, child: Text('Branch')),
              TableCell(
                head: true,
                align: TableCellAlign.end,
                child: Text('Status'),
              ),
            ],
          ),
          for (final build in _Build.all)
            TableRow(
              active: widget.interactive && _selected == build.id,
              onPressed: widget.interactive
                  ? () => setState(() => _selected = build.id)
                  : null,
              children: [
                TableCell(child: Text(build.id)),
                TableCell(child: Text(build.branch)),
                TableCell(
                  align: TableCellAlign.end,
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
    );
  }
}

class _WithControls extends StatefulWidget {
  const _WithControls();

  @override
  State<_WithControls> createState() => _WithControlsState();
}

class _WithControlsState extends State<_WithControls> {
  final Set<String> _enabled = {'aurora'};

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 380,
      child: DataTable(
        children: [
          const TableHead(
            children: [
              TableCell(head: true, child: Text('Build')),
              TableCell(
                head: true,
                align: TableCellAlign.end,
                child: Text('Notify'),
              ),
            ],
          ),
          for (final build in _Build.all)
            TableRow(
              children: [
                TableCell(child: Text(build.id)),
                TableCell(
                  align: TableCellAlign.end,
                  child: Switch(
                    value: _enabled.contains(build.id),
                    size: WidgetSize.small,
                    onChanged: (on) => setState(() {
                      if (on) {
                        _enabled.add(build.id);
                      } else {
                        _enabled.remove(build.id);
                      }
                    }),
                  ),
                ),
              ],
            ),
        ],
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'DataTable',
      description:
          'A header row and a run of rows under it. The parts are separate '
          'widgets rather than a data source, so a cell holds whatever a cell '
          'should hold — a badge, a switch, a number set to the trailing edge.',
      children: [
        DocSection(
          'Rows',
          description:
              'A row is inert unless it is given an `onPressed`: that is what '
              'gives it the hover wash and a click, and `active` marks the '
              'selected one. Hover outranks selection, so moving over a '
              'selected row still says which row the pointer is on.',
          children: [
            RowWrap(
              alignment: WrapCrossAlignment.start,
              children: [
                _Builds(interactive: true),
                _Builds(interactive: false),
              ],
            ),
          ],
        ),
        DocSection(
          'Cells',
          description:
              'A cell aligns to the start by default and to the end where it '
              'holds a number or a control — which is what keeps a column of '
              'figures readable and a column of switches on one edge.',
          children: [_WithControls()],
        ),
        DocSection(
          'Empty',
          description:
              'A table with no rows is not a table with a message in it: the '
              'empty state replaces the table, in the well the table would '
              'have sat in.',
          children: [
            SizedBox(
              width: 420,
              child: Card(
                variant: CardVariant.sunken,
                child: EmptyState(title: 'No builds have run on this branch'),
              ),
            ),
          ],
        ),
      ],
    );
  }
}
