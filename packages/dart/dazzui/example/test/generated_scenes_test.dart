import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/generated/scenes.dart';
import 'package:dazzui_example/host.dart';
import 'package:dazzui_example/playground/scene.dart';
import 'package:dazzui_example/playground_theme.dart';
import 'package:dazzui_example/storybook/gallery.dart';
import 'package:dazzui_example/storybook/story.dart';
import 'package:flutter_test/flutter_test.dart';

import 'layout_font.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  testWidgets(
    'grid columns include gaps, equal row heights, and retain empty columns',
    (tester) async {
      for (final width in [640.0, 600.0]) {
        await tester.pumpWidget(
          Host(
            title: 'Grid',
            home: Theme(
              data: PlaygroundTheme.values.first.data,
              child: Center(
                child: SizedBox(
                  width: width,
                  child: TileGrid(
                    min: 200,
                    children: [
                      for (var i = 0; i < 4; i++)
                        SizedBox(
                          key: ValueKey('tile-$i'),
                          height: i == 1 ? 40 : 20,
                        ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        );
        final first = tester.getRect(find.byKey(const ValueKey('tile-0')));
        final third = tester.getRect(find.byKey(const ValueKey('tile-2')));
        final fourth = tester.getRect(find.byKey(const ValueKey('tile-3')));
        final gap = PlaygroundTheme.values.first.data.vars.spacing4;
        expect(first.height, 40);
        if (width == 640) {
          expect(first.top, third.top);
          expect(fourth.top, greaterThan(first.top));
          expect(first.width, closeTo((width - gap * 2) / 3, 0.01));
        } else {
          expect(third.top, greaterThan(first.top));
          expect(first.width, closeTo((width - gap) / 2, 0.01));
        }
        expect(first.width, fourth.width);
        expect(tester.takeException(), isNull);
      }
    },
  );
  setUpAll(loadLayoutFont);
  final sceneTheme = layoutTheme(PlaygroundTheme.values.first.data);
  testWidgets(
    'vertical dividers stretch to their controls instead of disappearing',
    (tester) async {
      await tester.pumpWidget(
        Host(
          title: 'Vertical rule',
          home: Theme(
            data: sceneTheme,
            child: Center(
              child: GeneratedScene(
                scene: const {
                  'content': {
                    'component': 'ActionBar',
                    'children': [
                      {
                        'component': 'Button',
                        'children': ['Undo'],
                      },
                      {
                        'component': 'Divider',
                        'props': {'orientation': 'vertical'},
                      },
                      {
                        'component': 'Button',
                        'children': ['Redo'],
                      },
                    ],
                  },
                },
              ),
            ),
          ),
        ),
      );
      await tester.pump();
      final line = tester.getSize(find.byType(VerticalDivider));
      final button = tester.getSize(find.byType(Button).first);
      expect(line.height, button.height);
      expect(line.height, greaterThan(0));
      expect(tester.takeException(), isNull);
    },
  );
  testWidgets('slot actions update bound text and the visible input', (
    tester,
  ) async {
    final key = GlobalKey<GeneratedSceneState>();
    await tester.pumpWidget(
      Host(
        title: 'Slot state',
        home: Theme(
          data: sceneTheme,
          child: Center(
            child: GeneratedScene(
              key: key,
              scene: const {
                'state': {'query': 'Ada'},
                'content': {
                  'component': 'EmptyState',
                  'props': {'title': 'No results'},
                  'slots': {
                    'action': {
                      'component': 'Col',
                      'props': {'width': 240},
                      'children': [
                        {
                          'component': 'TextField',
                          'id': 'query',
                          'bind': {'value': 'query'},
                        },
                        {
                          'component': 'Button',
                          'id': 'clear',
                          'children': ['Clear'],
                          'on': {
                            'press': {'set': 'query', 'value': ''},
                          },
                        },
                      ],
                    },
                  },
                },
              },
            ),
          ),
        ),
      ),
    );
    await tester.pump();
    final input = find.byType(EditableText);
    expect(tester.widget<EditableText>(input).controller.text, 'Ada');
    await tester.enterText(input, 'Grace');
    await tester.pump();
    expect(key.currentState!.snapshot['query'], 'Grace');
    await tester.tap(find.byKey(const ValueKey<String>('clear')));
    await tester.pump();
    expect(key.currentState!.snapshot['query'], '');
    expect(tester.widget<EditableText>(input).controller.text, '');
    expect(tester.takeException(), isNull);
  });
  testWidgets('unmounting a scene cancels its periodic updates', (
    tester,
  ) async {
    final key = GlobalKey<GeneratedSceneState>();
    final scene = generatedScenes.values.singleWhere(
      (s) => s['id'] == 'components-progress--live',
    );
    await tester.pumpWidget(
      Host(
        title: 'Timer lifecycle',
        home: Theme(
          data: sceneTheme,
          child: Center(
            child: GeneratedScene(key: key, scene: scene),
          ),
        ),
      ),
    );
    final state = key.currentState!;
    await tester.pump(const Duration(milliseconds: 400));
    expect(state.snapshot['value'], 12);
    expect(tester.widget<Progress>(find.byType(Progress)).value, 0.12);
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pump(const Duration(milliseconds: 800));
    expect(state.snapshot['value'], 12);
    expect(tester.takeException(), isNull);
  });
  testWidgets('progress percentages clamp and preserve indeterminate mode', (
    tester,
  ) async {
    await tester.pumpWidget(
      Host(
        title: 'Progress values',
        home: Theme(
          data: sceneTheme,
          child: Center(
            child: GeneratedScene(
              scene: {
                'content': {
                  'component': 'Col',
                  'props': {'width': 240},
                  'children': [
                    for (final value in [-12, 43, 150])
                      {
                        'component': 'Progress',
                        'props': {'value': value},
                      },
                    {'component': 'Progress'},
                  ],
                },
              },
            ),
          ),
        ),
      ),
    );
    await tester.pump();
    expect(
      tester
          .widgetList<Progress>(find.byType(Progress))
          .map((bar) => bar.value),
      [0, 0.43, 1, null],
    );
    expect(tester.takeException(), isNull);
  });
  testWidgets('a generated card hugs its content like the native card', (
    tester,
  ) async {
    await tester.pumpWidget(
      Host(
        title: 'Card layout',
        home: Theme(
          data: sceneTheme,
          child: Center(
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                const Card(size: WidgetSize.small, child: Spinner()),
                GeneratedScene(
                  scene: const {
                    'content': {
                      'component': 'Card',
                      'props': {'size': 'small'},
                      'children': [
                        {'component': 'Spinner'},
                      ],
                    },
                  },
                ),
              ],
            ),
          ),
        ),
      ),
    );
    await tester.pump();
    final cards = find.byType(Card);
    expect(tester.getSize(cards.at(1)), tester.getSize(cards.at(0)));
    expect(tester.takeException(), isNull);
  });
  test('generated catalog creates pages without handwritten placeholders', () {
    final groups = generatedGroups([]);
    final stories = groups.expand((g) => g.pages).expand((p) => p.stories);
    expect(
      stories.map((s) => s.sourceId).toSet(),
      generatedScenes.values.map((s) => s['id']).toSet(),
    );
    expect(
      groups.single.pages.map((p) => p.name),
      containsAll(['Button', 'TextField', 'Dialog']),
    );
  });

  test(
    'generated catalog replaces matching stories and retains extensions',
    () {
      final original = [
        StoryGroup('Components', [
          StoryPage('Button', [
            Story('Normal', (_) => const SizedBox.shrink()),
            Story('Local extension', (_) => const SizedBox.shrink()),
          ]),
        ]),
      ];
      final merged = generatedGroups(original);
      final buttons = merged.single.pages
          .singleWhere((page) => page.name == 'Button')
          .stories;
      expect(buttons.where((s) => s.name == 'Normal').length, 1);
      expect(
        buttons.singleWhere((s) => s.name == 'Normal').sourceId,
        'components-button--normal',
      );
      expect(buttons.any((s) => s.name == 'Local extension'), isTrue);
      expect(original.single.pages.single.stories.length, 2);
      expect(
        generatedGroups(merged).single.pages
            .singleWhere((page) => page.name == 'Button')
            .stories
            .length,
        buttons.length,
      );
    },
  );

  for (final entry in generatedScenes.entries) {
    final points =
        entry.value['checkpoints'] ??
        [
          {'id': 'initial', 'steps': []},
        ];
    for (final point in points) {
      testWidgets('${entry.value['id']} / ${point['id']}', (tester) async {
        tester.view.physicalSize = const Size(1920, 1280);
        tester.view.devicePixelRatio = 2;
        addTearDown(tester.view.reset);
        final key = GlobalKey<GeneratedSceneState>();
        await tester.pumpWidget(
          Host(
            title: 'Generated scene',
            home: Theme(
              data: sceneTheme,
              child: entry.value['layout'] == 'page'
                  ? SingleChildScrollView(
                      child: GeneratedScene(key: key, scene: entry.value),
                    )
                  : Center(
                      child: GeneratedScene(key: key, scene: entry.value),
                    ),
            ),
          ),
        );
        await tester.pump();
        if ((entry.value['timers'] as List? ?? []).isEmpty)
          await tester.pump(const Duration(milliseconds: 400));
        for (final step in point['steps']) {
          if (step['action'] == 'advance') {
            await tester.pump(Duration(milliseconds: step['milliseconds']));
            continue;
          }
          final target = find.byKey(ValueKey<String>(step['target']));
          if (step['action'] == 'click') {
            await tester.tap(target);
          } else {
            await tester.enterText(
              find.descendant(of: target, matching: find.byType(EditableText)),
              step['value'],
            );
          }
          await tester.pump();
          if ((entry.value['timers'] as List? ?? []).isEmpty)
            await tester.pump(const Duration(milliseconds: 400));
        }
        for (final expected in (point['expect'] as Map? ?? {}).entries) {
          expect(key.currentState!.snapshot[expected.key], expected.value);
        }
        for (final expected in (point['expectText'] as Map? ?? {}).entries) {
          final target = find.byKey(ValueKey<String>(expected.key));
          final texts = tester.widgetList<Text>(
            find.descendant(of: target, matching: find.byType(Text)),
          );
          expect(
            texts
                .map((text) => text.data ?? text.textSpan!.toPlainText())
                .join(),
            expected.value,
          );
        }
        for (final expected
            in (point['expectDisabled'] as Map? ?? {}).entries) {
          final button = tester.widget<Button>(
            find.byKey(ValueKey<String>(expected.key)),
          );
          expect(!button.enabled, expected.value);
        }
        expect(tester.takeException(), isNull);
        final navigator = tester.state<NavigatorState>(
          find.byType(Navigator).first,
        );
        if (navigator.canPop()) {
          navigator.pop();
          await tester.pumpAndSettle();
        }
        await tester.pumpWidget(const SizedBox.shrink());
        await tester.pump();
      });
    }
  }
}
