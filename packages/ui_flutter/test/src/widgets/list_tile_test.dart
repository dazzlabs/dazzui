import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('ListTile', () {
    testWidgets('creates a base list tile with title', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: Text('Title'),
            ),
          ),
        ),
      );

      expect(find.text('Title'), findsOneWidget);
    });

    testWidgets('creates a list tile with subtitle', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: Text('Title'),
              subtitle: Text('Subtitle'),
            ),
          ),
        ),
      );

      expect(find.text('Title'), findsOneWidget);
      expect(find.text('Subtitle'), findsOneWidget);
    });

    testWidgets('creates a list tile with leading', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              leading: Icon(IconData(0xe047)),
              title: Text('Title'),
            ),
          ),
        ),
      );

      expect(find.text('Title'), findsOneWidget);
      expect(find.byType(Icon), findsOneWidget);
    });

    testWidgets('creates a list tile with trailing', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: Text('Title'),
              trailing: Icon(IconData(0xe047)),
            ),
          ),
        ),
      );

      expect(find.text('Title'), findsOneWidget);
      expect(find.byType(Icon), findsOneWidget);
    });

    testWidgets('creates a list tile with additional info', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: Text('Title'),
              additionalInfo: Text('Info'),
            ),
          ),
        ),
      );

      expect(find.text('Title'), findsOneWidget);
      expect(find.text('Info'), findsOneWidget);
    });

    testWidgets('creates a notched list tile', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile.notched(
              title: Text('Title'),
              subtitle: Text('Subtitle'),
            ),
          ),
        ),
      );

      expect(find.text('Title'), findsOneWidget);
      expect(find.text('Subtitle'), findsOneWidget);
    });

    testWidgets('handles tap callback', (WidgetTester tester) async {
      bool tapped = false;

      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: const Text('Title'),
              onTap: () {
                tapped = true;
              },
            ),
          ),
        ),
      );

      await tester.tap(find.text('Title'));
      await tester.pump();

      expect(tapped, isTrue);
    });

    testWidgets('handles async tap callback', (WidgetTester tester) async {
      bool tapped = false;

      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: const Text('Title'),
              onTap: () async {
                await Future.delayed(const Duration(milliseconds: 100));
                tapped = true;
              },
            ),
          ),
        ),
      );

      await tester.tap(find.text('Title'));
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 100));

      expect(tapped, isTrue);
    });

    testWidgets('shows tap feedback', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: const Text('Title'),
              onTap: () {},
            ),
          ),
        ),
      );

      // Tap down should show feedback
      final gesture = await tester.startGesture(tester.getCenter(find.text('Title')));
      await tester.pump();

      // Release should remove feedback
      await gesture.up();
      await tester.pump();
    });

    testWidgets('respects custom background color', (WidgetTester tester) async {
      const customColor = Color(0xFF123456);

      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: Text('Title'),
              backgroundColor: customColor,
            ),
          ),
        ),
      );

      final coloredBox = tester.widget<ColoredBox>(
        find.descendant(
          of: find.byType(ListTile),
          matching: find.byType(ColoredBox),
        ),
      );

      expect(coloredBox.color, customColor);
    });

    testWidgets('respects custom padding', (WidgetTester tester) async {
      const customPadding = EdgeInsets.all(20.0);

      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: Text('Title'),
              padding: customPadding,
            ),
          ),
        ),
      );

      final padding = tester.widget<Padding>(
        find.descendant(
          of: find.byType(ColoredBox),
          matching: find.byType(Padding),
        ),
      );

      expect(padding.padding, customPadding);
    });

    testWidgets('respects custom leading size', (WidgetTester tester) async {
      const customSize = 40.0;

      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              leading: Icon(IconData(0xe047)),
              title: Text('Title'),
              leadingSize: customSize,
            ),
          ),
        ),
      );

      final sizedBox = tester.widget<SizedBox>(
        find.descendant(
          of: find.byType(ListTile),
          matching: find.byWidgetPredicate(
            (widget) => widget is SizedBox && widget.width == customSize && widget.height == customSize,
          ),
        ),
      );

      expect(sizedBox.width, customSize);
      expect(sizedBox.height, customSize);
    });

    testWidgets('does not respond to tap when onTap is null', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: const Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              title: Text('Title'),
              onTap: null,
            ),
          ),
        ),
      );

      // Should not find GestureDetector when onTap is null
      expect(find.byType(GestureDetector), findsNothing);
    });

    testWidgets('creates a complete list tile with all properties', (WidgetTester tester) async {
      await tester.pumpWidget(
        Theme(
          data: ThemeData.light(iconLibrary: IconLibrary.material()),
          child: Directionality(
            textDirection: TextDirection.ltr,
            child: ListTile(
              leading: const Icon(IconData(0xe047)),
              title: const Text('Title'),
              subtitle: const Text('Subtitle'),
              additionalInfo: const Text('Info'),
              trailing: const Icon(IconData(0xe5cc)),
              onTap: () {},
            ),
          ),
        ),
      );

      expect(find.text('Title'), findsOneWidget);
      expect(find.text('Subtitle'), findsOneWidget);
      expect(find.text('Info'), findsOneWidget);
      expect(find.byType(Icon), findsNWidgets(2));
    });
  });
}

