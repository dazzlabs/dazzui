import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/main.dart';
import 'package:dazzui_example/playground_theme.dart';
import 'package:dazzui_example/storybook/story.dart';
import 'package:dazzui_example/storybook/story_index.dart';
import 'package:flutter/material.dart' as material;
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('the window opens on the first story', (tester) async {
    tester.view.physicalSize = const Size(2160, 1440);
    tester.view.devicePixelRatio = 2.0;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(const PlaygroundApp());
    await tester.pump(const Duration(milliseconds: 300));

    expect(find.text('DazzUI'), findsOneWidget);
    expect(find.text(StoryIndex.first.page), findsWidgets);
  });

  testWidgets('a page opens on its overview and a story can be selected', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(2160, 1440);
    tester.view.devicePixelRatio = 2.0;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(const PlaygroundApp());
    await tester.pump(const Duration(milliseconds: 300));

    // The sidebar row, not the breadcrumb: `first` matches both.
    await tester.tap(find.text('Badge').first);
    await tester.pump(const Duration(milliseconds: 300));
    expect(tester.takeException(), isNull);

    await tester.tap(find.text('Sizes').first);
    await tester.pump(const Duration(milliseconds: 300));
    expect(tester.takeException(), isNull);
  });

  // Every story, in every theme. The stage is pumped directly rather than
  // reached through the sidebar: what is being tested is that a story draws,
  // and walking the tree to it would only be testing the tree.
  testWidgets('every story draws in every theme', (tester) async {
    tester.view.physicalSize = const Size(2160, 2880);
    tester.view.devicePixelRatio = 2.0;
    addTearDown(tester.view.reset);

    for (final PlaygroundTheme theme in PlaygroundTheme.values) {
      for (final StoryGroup group in StoryIndex.groups) {
        for (final StoryPage page in group.pages) {
          for (final Story story in page.stories) {
            await tester.pumpWidget(
              _Harness(theme: theme, ref: StoryRef(page.name, story.name)),
            );
            // pump with a duration rather than pumpAndSettle: an
            // indeterminate progress bar and a loader animate forever, so the
            // tree never settles.
            await tester.pump(const Duration(milliseconds: 300));

            expect(
              tester.takeException(),
              isNull,
              reason: '${theme.name} / ${page.name} / ${story.name}',
            );
          }
        }
      }
    }
  });
}

/// One story under one theme, in the plumbing the app gives it.
class _Harness extends StatelessWidget {
  const _Harness({required this.theme, required this.ref});

  final PlaygroundTheme theme;
  final StoryRef ref;

  @override
  Widget build(BuildContext context) {
    return material.MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Theme(
        data: theme.data,
        child: material.Material(
          type: material.MaterialType.transparency,
          child: ColoredBox(
            color: theme.data.vars.colorSurface,
            child: StoryStage(selection: ref),
          ),
        ),
      ),
    );
  }
}
