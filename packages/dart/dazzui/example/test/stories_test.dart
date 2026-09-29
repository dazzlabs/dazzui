import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/app.dart';
import 'package:dazzui_example/host.dart';
import 'package:dazzui_example/lib/scene.dart';
import 'package:dazzui_example/lib/story.dart';
import 'package:dazzui_example/lib/themes.dart';
import 'package:dazzui_example/stories/index.dart';
import 'package:flutter_test/flutter_test.dart';

import 'layout_font.dart';

/// Draws one scene on a stage the size the React page gives it.
Future<void> pumpScene(WidgetTester tester, Scene scene, ThemeChoice theme) {
  final ThemeData data = layoutTheme(theme.data);
  return tester.pumpWidget(
    Host(
      title: 'scene',
      home: Theme(
        data: data,
        child: ColoredBox(
          color: data.vars.colorSurface,
          child: DefaultTextStyle(
            style: data.vars.bodyMedium.copyWith(color: data.vars.colorContent),
            child: Center(
              child: SizedBox(
                width: 720,
                child: Center(child: SceneView(scene: scene)),
              ),
            ),
          ),
        ),
      ),
    ),
  );
}

void main() {
  setUpAll(loadLayoutFont);

  testWidgets('the window opens on the overview and on a story', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1360, 860);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(const App());
    // Spinners never settle, so the window is given a fixed second.
    await tester.pump(const Duration(seconds: 1));
    expect(find.text('Overview'), findsWidgets);
    expect(find.text('${stories.length} components'), findsOneWidget);

    await tester.tap(find.text('Button').first);
    await tester.pump(const Duration(seconds: 1));
    expect(find.text('Variants'), findsOneWidget);
  });

  for (final dark in [false, true]) {
    final ThemeChoice theme = ThemeChoice('studio', dark: dark);
    for (final story in stories) {
      final scenes = <String, Scene>{
        'preview': story.preview,
        for (final example in story.examples) example.title: example.demo,
      };
      for (final MapEntry(key: title, value: scene) in scenes.entries) {
        testWidgets('${story.name} / $title (${theme.id})', (tester) async {
          tester.view.physicalSize = const Size(1024, 768);
          tester.view.devicePixelRatio = 1;
          addTearDown(tester.view.reset);
          await pumpScene(tester, scene, theme);
          await tester.pump(const Duration(seconds: 1));
          expect(tester.takeException(), isNull);
        });
      }
    }
  }

  group('interactions', () {
    Scene demo(String id, String title) => stories
        .firstWhere((story) => story.id == id)
        .examples
        .firstWhere((example) => example.title == title)
        .demo;

    testWidgets('a dialog opens from its trigger and closes from its footer', (
      tester,
    ) async {
      await pumpScene(tester, demo('dialog', 'A form'), defaultTheme);
      await tester.tap(find.text('Rename…'));
      await tester.pumpAndSettle();
      expect(find.text('Rename project'), findsOneWidget);
      await tester.tap(find.text('Cancel'));
      await tester.pumpAndSettle();
      expect(find.text('Rename project'), findsNothing);
    });

    testWidgets('a button raises a toast through its provider', (tester) async {
      await pumpScene(tester, demo('toast', 'With an action'), defaultTheme);
      await tester.tap(find.text('Archive'));
      await tester.pump();
      expect(find.text('Project archived'), findsOneWidget);
      expect(find.text('Undo'), findsOneWidget);
    });

    testWidgets('a slot written by one event is read by another component', (
      tester,
    ) async {
      final Scene scene = stories
          .firstWhere((story) => story.id == 'option-card')
          .preview;
      await pumpScene(tester, scene, defaultTheme);
      OptionCard card(String title) =>
          tester.widget<OptionCard>(find.widgetWithText(OptionCard, title));
      expect(card('Team').selected, isTrue);
      await tester.tap(find.text('Free'));
      await tester.pump();
      expect(card('Free').selected, isTrue);
      expect(card('Team').selected, isFalse);
    });

    testWidgets('a popover opens from its trigger', (tester) async {
      await pumpScene(
        tester,
        demo('popover', 'With a title and content'),
        defaultTheme,
      );
      await tester.tap(find.text('Share'));
      await tester.pumpAndSettle();
      expect(find.text('Share this project'), findsOneWidget);
    });
  });

  test('every theme the panel offers has variables', () {
    for (final group in themeFamilyGroups) {
      for (final family in group.families) {
        for (final dark in [false, true]) {
          expect(ThemeChoice(family.value, dark: dark).data.vars, isNotNull);
        }
      }
    }
  });
}
