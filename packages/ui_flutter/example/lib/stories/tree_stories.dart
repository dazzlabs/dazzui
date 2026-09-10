import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Tree` — a hierarchy the reader walks.
class TreeStories {
  TreeStories._();

  static TreeNode<String> file(String id, String label) => TreeNode<String>(
    id: id,
    label: Text(label),
    icon: FluentIcons.document_16_regular,
  );

  static TreeNode<String> folder(
    String id,
    String label,
    List<TreeNode<String>> children,
  ) => TreeNode<String>(
    id: id,
    label: Text(label),
    icon: FluentIcons.folder_16_regular,
    children: children,
  );

  static List<TreeNode<String>> get files => <TreeNode<String>>[
    folder('src', 'src', <TreeNode<String>>[
      folder('components', 'components', <TreeNode<String>>[
        file('badge.tsx', 'badge.tsx'),
        file('button.tsx', 'button.tsx'),
        file('tree.tsx', 'tree.tsx'),
      ]),
      folder('styles', 'styles', <TreeNode<String>>[
        file('base.css', 'base.css'),
        file('components.css', 'components.css'),
      ]),
      file('index.ts', 'index.ts'),
    ]),
    folder('empty', 'generated', const <TreeNode<String>>[]),
    file('package.json', 'package.json'),
    file('README.md', 'README.md'),
  ];

  static List<TreeNode<String>> get changed => <TreeNode<String>>[
    folder('src2', 'src', <TreeNode<String>>[
      TreeNode<String>(
        id: 'a',
        label: const Text('tree.tsx'),
        icon: FluentIcons.document_16_regular,
        meta: const Badge(size: WidgetSize.small, child: Text('M')),
      ),
      TreeNode<String>(
        id: 'b',
        label: const Text('tree.css'),
        icon: FluentIcons.document_16_regular,
        meta: const Badge(
          size: WidgetSize.small,
          tint: BadgeTint.success,
          child: Text('A'),
        ),
      ),
      TreeNode<String>(
        id: 'c',
        label: const Text('menu.tsx'),
        icon: FluentIcons.document_16_regular,
        meta: const Text('+12 −3'),
      ),
    ]),
    TreeNode<String>(
      id: 'd',
      label: const Text('AGENTS.md'),
      icon: FluentIcons.document_16_regular,
      meta: const Text('locked'),
      enabled: false,
    ),
  ];

  static StoryPage get page => StoryPage('Tree', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'File tree',
      (context) => const Col(width: 288, children: [_FileList()]),
    ),
    Story(
      'Sizes',
      (context) => Col(
        width: 288,
        spacing: 16,
        children: <Widget>[
          const _FileList(size: WidgetSize.small),
          const _FileList(),
        ],
      ),
    ),
    Story(
      'With meta',
      (context) => Col(
        width: 288,
        children: <Widget>[
          Tree<String>(
            semanticsLabel: 'Changes',
            initiallyExpanded: const <String>['src2'],
            items: changed,
            selected: 'a',
          ),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        width: 288,
        spacing: 16,
        children: <Widget>[
          for (final TreeTint tint in TreeTint.values)
            Tree<String>(
              semanticsLabel: 'Files (${tint.name})',
              initiallyExpanded: const <String>['src'],
              items: files,
              selected: 'index.ts',
              tint: tint,
            ),
        ],
      ),
    ),
    Story(
      'Collapsed',
      (context) => Col(
        width: 288,
        children: <Widget>[
          Tree<String>(semanticsLabel: 'Files', items: files),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => Col(
        width: 288,
        children: <Widget>[
          Tree<String>(
            semanticsLabel: 'Changes',
            initiallyExpanded: const <String>['src2'],
            items: changed,
          ),
        ],
      ),
    ),
  ]);
}

class _FileList extends StatefulWidget {
  const _FileList({this.size = WidgetSize.medium});

  final WidgetSize size;

  @override
  State<_FileList> createState() => _FileListState();
}

class _FileListState extends State<_FileList> {
  List<String> _expanded = <String>['src', 'components'];
  String? _selected = 'tree.tsx';

  @override
  Widget build(BuildContext context) {
    return Tree<String>(
      semanticsLabel: 'Files',
      items: TreeStories.files,
      expanded: _expanded,
      selected: _selected,
      size: widget.size,
      onExpandedChanged: (List<String> next) =>
          setState(() => _expanded = next),
      onSelectedChanged: (String id) => setState(() => _selected = id),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'Tree',
      description:
          'A hierarchy the reader walks. It is the one widget here with no '
          'Base UI part behind its React twin, because a tree’s keyboard is '
          'not a list’s: ArrowRight opens a shut branch and descends into an '
          'open one, ArrowLeft shuts one and climbs out of a leaf, and the '
          'whole tree is a single tab stop so focus roves inside it rather '
          'than tabbing through a hundred files.',
      children: <Widget>[
        const DocSection(
          'Branches and leaves',
          description:
              'children present at all — an empty list included — is what '
              'makes a node a branch. That is how an empty folder still draws '
              'a twisty and still opens, which is the difference between '
              '“nothing in here” and “not a folder”. The twisty keeps its '
              'column on a leaf too, empty: files and folders at one level '
              'start at one x, and a leaf that reclaimed the space would put '
              'every filename at a different indent from its siblings.',
          children: <Widget>[
            Col(width: 288, children: [_FileList()]),
          ],
        ),
        const DocSection(
          'Two targets, two meanings',
          description:
              'Tapping a folder opens it and selects it, because in a file '
              'tree those are one gesture. Tapping the twisty alone opens it '
              'and leaves the selection where it was — which is how a reader '
              'looks inside a folder without losing the file they were '
              'reading.',
          children: <Widget>[
            Col(width: 288, children: [_FileList()]),
          ],
        ),
        DocSection(
          'Selection',
          description:
              'Drawn the way every selected thing in this system is: the '
              'tinted wash, and the label at the label weight rather than the '
              'quiet one. On a column of forty rows the weight is doing most '
              'of the work — one grade of colour is hardest to find in exactly '
              'this shape.',
          children: <Widget>[
            Col(
              width: 288,
              children: <Widget>[
                Tree<String>(
                  semanticsLabel: 'Changes',
                  initiallyExpanded: const <String>['src2'],
                  items: TreeStories.changed,
                  selected: 'a',
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
