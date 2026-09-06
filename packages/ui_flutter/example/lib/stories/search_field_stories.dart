import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';
import '../storybook/story_index.dart';

/// `Components/SearchField` — a text field that knows what it is for.
class SearchFieldStories {
  SearchFieldStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('SearchField', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Empty', (context) => const _Field()),
    Story('With a query', (context) => const _Field(query: 'button')),
    Story(
      'With a hint',
      (context) => const SizedBox(
        width: 320,
        child: SearchField(
          placeholder: 'Search components',
          hint: KeyCap('⌘K', size: WidgetSize.small),
        ),
      ),
    ),
    Story(
      'Sizes',
      (context) => Col(
        width: 320,
        children: [
          for (final size in sizes)
            SearchField(
              placeholder: 'Search components',
              size: size,
            ),
        ],
      ),
    ),
    Story(
      'Disabled',
      (context) => const SizedBox(
        width: 320,
        child: SearchField(placeholder: 'Search components', enabled: false),
      ),
    ),
    Story('Filtering a list', (context) => const _Filtering()),
    Story(
      'In a pane header',
      (context) => const Frame(
        width: 560,
        height: 300,
        children: [
          Pane(children: [_Filtering()]),
        ],
      ),
    ),
  ]);
}

class _Field extends StatefulWidget {
  const _Field({this.query = ''});

  final String query;

  @override
  State<_Field> createState() => _FieldState();
}

class _FieldState extends State<_Field> {
  late final TextEditingController _controller = TextEditingController(
    text: widget.query,
  );

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 320,
      child: SearchField(
        controller: _controller,
        placeholder: 'Search components',
      ),
    );
  }
}

/// The field doing its job: what it filters is the playground's own index.
class _Filtering extends StatefulWidget {
  const _Filtering();

  @override
  State<_Filtering> createState() => _FilteringState();
}

class _FilteringState extends State<_Filtering> {
  String _query = '';

  List<String> get _matches {
    final String needle = _query.toLowerCase();
    if (needle.isEmpty) return StoryIndex.componentNames;
    return StoryIndex.componentNames
        .where((name) => name.toLowerCase().contains(needle))
        .toList();
  }

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 320,
      children: [
        SearchField(
          placeholder: 'Search components',
          onChanged: (value) => setState(() => _query = value),
        ),
        if (_matches.isEmpty)
          EmptyState(title: 'Nothing matched “$_query”')
        else
          Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            mainAxisSize: MainAxisSize.min,
            children: [
              for (final name in _matches.take(5))
                NavItem(label: name, onPressed: () {}),
            ],
          ),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'SearchField',
      description:
          'The text field with a glyph in front of it, a clear button behind, '
          'and a slot for the key that focuses it. It is the same box as '
          '`TextField` — same heights, same states — so a search field in a '
          'toolbar lines up with everything else on that row.',
      children: [
        DocSection(
          'States',
          description:
              'Empty, with a query, and disabled. The clear button appears '
              'only once there is something to clear, so an empty field is not '
              'offering an action that does nothing.',
          children: [
            Col(
              width: 420,
              children: [
                _Field(),
                _Field(query: 'button'),
                SizedBox(
                  width: 320,
                  child: SearchField(
                    placeholder: 'Search components',
                    enabled: false,
                  ),
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'The hint',
          description:
              'The trailing slot takes the key that focuses the field. It is a '
              '`KeyCap` at the small step — a hint rather than a control, so '
              'it never answers the pointer.',
          children: [
            SizedBox(
              width: 320,
              child: SearchField(
                placeholder: 'Search components',
                hint: KeyCap('⌘K', size: WidgetSize.small),
              ),
            ),
          ],
        ),
        DocSection(
          'Filtering',
          description:
              'What a search field is actually for. This one filters the '
              'playground’s own component index, and falls back to an '
              '`EmptyState` when nothing matches.',
          children: [_Filtering()],
        ),
      ],
    );
  }
}
