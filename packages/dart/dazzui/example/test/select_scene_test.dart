import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/host.dart';
import 'package:dazzui_example/lib/scene.dart';
import 'package:dazzui_example/lib/story.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets(
    'select scenes bind grouped values and preserve external updates',
    (
      tester,
    ) async {
      const scene = Scene(
        state: {'person': 'grace'},
        content: {
          r'$el': 'Col',
          'props': {'width': '20rem'},
          'children': [
            {
              r'$el': 'Select',
              'props': {
                'value': {r'$state': 'person'},
                'onValueChange': {
                  r'$do': [
                    {
                      'set': 'person',
                      'to': {r'$arg': 0},
                    },
                  ],
                },
                'options': [
                  {'value': 'none', 'label': 'Unassigned'},
                  {
                    'label': 'People',
                    'options': [
                      {'value': 'ada', 'label': 'Ada'},
                      {'value': 'grace', 'label': 'Grace'},
                    ],
                  },
                ],
              },
            },
            {
              r'$el': 'Text',
              'children': [
                {
                  r'$concat': [
                    'Selected: ',
                    {r'$state': 'person'},
                  ],
                },
              ],
            },
            {
              r'$el': 'Button',
              'props': {
                'onClick': {
                  r'$do': [
                    {'set': 'person', 'to': 'none'},
                  ],
                },
              },
              'children': ['Reset'],
            },
          ],
        },
      );
      await tester.pumpWidget(
        Host(
          title: 'Select',
          home: Theme(
            data: ThemeData.studioLight(),
            child: const Center(child: SceneView(scene: scene)),
          ),
        ),
      );
      expect(find.text('Grace'), findsNWidgets(2));
      expect(find.text('Selected: grace'), findsOneWidget);
      await tester.tap(find.byType(Select<String>));
      await tester.pumpAndSettle();
      await tester.tap(
        find.descendant(of: find.byType(MenuPanel), matching: find.text('Ada')),
      );
      await tester.pumpAndSettle();
      expect(find.text('Selected: ada'), findsOneWidget);
      expect(find.text('Ada'), findsNWidgets(2));
      await tester.tap(find.text('Reset'));
      await tester.pumpAndSettle();
      expect(find.text('Selected: none'), findsOneWidget);
      expect(find.text('Unassigned'), findsNWidgets(2));
      expect(tester.takeException(), isNull);
    },
  );
}
