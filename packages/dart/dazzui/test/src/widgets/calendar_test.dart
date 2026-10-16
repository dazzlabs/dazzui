import 'package:dazzui/dazzui.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

/// `Calendar` — the month grid, in one day and in range mode.
///
/// What is here is what only this port can get wrong: a day being a square
/// at the control height, the two presses that make a span, and the keyboard
/// the grid writes out for itself.
void main() {
  final DateTime today = DateTime(2026, 10, 16);

  // The day's node merges its label with the number it shows, so the match
  // is on the label's start.
  Finder day(String label) => find.bySemanticsLabel(RegExp('^$label'));

  testWidgets('a day is a square at the control height, six rows a month', (
    tester,
  ) async {
    final SemanticsHandle semantics = tester.ensureSemantics();
    await tester.pumpWidget(
      host(
        Center(
          child: Calendar(initialValue: today, today: today),
        ),
      ),
    );

    final ThemeVariables vars = ThemeData.light().vars;
    final Size cell = tester.getSize(day('Friday, October 16, 2026'));
    expect(cell, Size.square(vars.calendarMediumCellSize));

    // Forty-two days, so the grid never changes height between months: the
    // last row is November's first week.
    expect(day('Sunday, November 8, 2026'), findsOneWidget);
    expect(find.text('October 2026'), findsOneWidget);
    semantics.dispose();
  });

  testWidgets('the small step draws two months side by side', (tester) async {
    final SemanticsHandle semantics = tester.ensureSemantics();
    await tester.pumpWidget(
      host(
        Center(
          child: Calendar.range(
            size: WidgetSize.small,
            months: 2,
            initialRange: DateRange(start: today, end: today),
            today: today,
          ),
        ),
      ),
    );

    final ThemeVariables vars = ThemeData.light().vars;
    expect(
      tester.getSize(day('Friday, October 16, 2026')),
      Size.square(vars.calendarSmallCellSize),
    );
    expect(find.text('October 2026'), findsOneWidget);
    expect(find.text('November 2026'), findsOneWidget);
    semantics.dispose();
  });

  testWidgets('two presses make a span, in either order', (tester) async {
    final SemanticsHandle semantics = tester.ensureSemantics();
    final List<DateRange> changes = <DateRange>[];
    await tester.pumpWidget(
      host(
        Center(
          child: Calendar.range(
            today: today,
            onRangeChanged: changes.add,
          ),
        ),
      ),
    );

    await tester.tap(day('Saturday, October 24, 2026'));
    await tester.pump();
    expect(changes.last.start, DateTime(2026, 10, 24));
    expect(changes.last.end, isNull);

    await tester.tap(day('Tuesday, October 20, 2026'));
    await tester.pump();
    expect(changes.last.start, DateTime(2026, 10, 20));
    expect(changes.last.end, DateTime(2026, 10, 24));

    // A third press starts over.
    await tester.tap(day('Thursday, October 1, 2026'));
    await tester.pump();
    expect(changes.last.start, DateTime(2026, 10, 1));
    expect(changes.last.end, isNull);
    semantics.dispose();
  });

  testWidgets('the arrow keys move by a day and a week, Page Down by a month, '
      'and Enter picks', (tester) async {
    final SemanticsHandle semantics = tester.ensureSemantics();
    final List<DateTime> picked = <DateTime>[];
    await tester.pumpWidget(
      host(
        Center(
          child: Calendar(
            initialValue: today,
            today: today,
            onChanged: picked.add,
          ),
        ),
      ),
    );

    await tester.tap(day('Friday, October 16, 2026'));
    await tester.pump();
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowRight);
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowDown);
    await tester.pump();
    await tester.sendKeyEvent(LogicalKeyboardKey.enter);
    await tester.pump();
    expect(picked.last, DateTime(2026, 10, 24));

    await tester.sendKeyEvent(LogicalKeyboardKey.pageDown);
    await tester.pump();
    expect(find.text('November 2026'), findsOneWidget);
    expect(find.text('October 2026'), findsNothing);
    semantics.dispose();
  });

  testWidgets('days outside min and max are drawn but cannot be picked', (
    tester,
  ) async {
    final SemanticsHandle semantics = tester.ensureSemantics();
    final List<DateTime> picked = <DateTime>[];
    await tester.pumpWidget(
      host(
        Center(
          child: Calendar(
            today: today,
            min: DateTime(2026, 10, 12),
            max: DateTime(2026, 10, 23),
            onChanged: picked.add,
          ),
        ),
      ),
    );

    expect(day('Saturday, October 24, 2026'), findsOneWidget);
    await tester.tap(day('Saturday, October 24, 2026'));
    await tester.pump();
    expect(picked, isEmpty);

    await tester.tap(day('Tuesday, October 20, 2026'));
    await tester.pump();
    expect(picked.single, DateTime(2026, 10, 20));
    semantics.dispose();
  });

  testWidgets('every theme draws it', (tester) async {
    final SemanticsHandle semantics = tester.ensureSemantics();
    for (final ThemeData theme in [
      ThemeData.studioLight(),
      ThemeData.studioDark(),
      ThemeData.graphiteDark(),
    ]) {
      await tester.pumpWidget(
        host(
          Center(
            child: Calendar.range(
              initialRange: DateRange(
                start: DateTime(2026, 10, 20),
                end: DateTime(2026, 10, 24),
              ),
              today: today,
            ),
          ),
          theme: theme,
        ),
      );
      expect(day('Friday, October 16, 2026'), findsOneWidget);
    }
    semantics.dispose();
  });
}
