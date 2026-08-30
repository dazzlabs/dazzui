import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('ListSection', () {
    testWidgets('creates a base list section with children', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListSection(
              children: [
                Text('Item 1'),
                Text('Item 2'),
                Text('Item 3'),
              ],
            ),
          ),
        ),
      );

      expect(find.text('Item 1'), findsOneWidget);
      expect(find.text('Item 2'), findsOneWidget);
      expect(find.text('Item 3'), findsOneWidget);
    });

    testWidgets('creates a base list section with header and footer', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListSection(
              header: Text('Header'),
              footer: Text('Footer'),
              children: [
                Text('Item 1'),
              ],
            ),
          ),
        ),
      );

      expect(find.text('Header'), findsOneWidget);
      expect(find.text('Footer'), findsOneWidget);
      expect(find.text('Item 1'), findsOneWidget);
    });

    testWidgets('creates an inset grouped list section', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListSection.insetGrouped(
              header: Text('Header'),
              children: [
                Text('Item 1'),
                Text('Item 2'),
              ],
            ),
          ),
        ),
      );

      expect(find.text('Header'), findsOneWidget);
      expect(find.text('Item 1'), findsOneWidget);
      expect(find.text('Item 2'), findsOneWidget);
    });

    testWidgets('respects custom divider margin', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListSection(
              dividerMargin: 50.0,
              children: [
                Text('Item 1'),
                Text('Item 2'),
              ],
            ),
          ),
        ),
      );

      expect(find.text('Item 1'), findsOneWidget);
      expect(find.text('Item 2'), findsOneWidget);
    });

    testWidgets('can be created with only header', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListSection(
              header: Text('Header Only'),
            ),
          ),
        ),
      );

      expect(find.text('Header Only'), findsOneWidget);
    });
  });
}

