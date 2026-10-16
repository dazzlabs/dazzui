import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

/// `DateRangePicker` — the field-shaped trigger and the sheet it opens.
///
/// What is here is the sheet's own rules: nothing reaches the value until
/// Done, an outside press puts the draft back, and a preset moves the span.
void main() {
  final DateTime today = DateTime(2026, 10, 16);
  final DateRangeValue meeting = DateRangeValue(
    start: DateTime(2026, 10, 16, 14, 30),
    end: DateTime(2026, 10, 16, 15, 30),
  );

  Widget picker({ValueChanged<DateRangeValue>? onChanged}) => host(
    Align(
      alignment: Alignment.topCenter,
      child: Padding(
        padding: const EdgeInsets.only(top: 40),
        child: DateRangePicker(
          initialValue: meeting,
          today: today,
          onChanged: onChanged,
        ),
      ),
    ),
  );

  testWidgets('the trigger wears the field box and reads the span back', (
    tester,
  ) async {
    await tester.pumpWidget(picker());

    final ThemeVariables vars = ThemeData.light().vars;
    final Size box = tester.getSize(find.byType(DateRangePicker));
    expect(box.height, vars.controlMediumSize);
    expect(find.textContaining('Fri, Oct 16'), findsOneWidget);
    expect(find.textContaining('14:30'), findsOneWidget);
  });

  testWidgets('a preset moves the span, and nothing reaches the value until '
      'Done', (tester) async {
    final List<DateRangeValue> changes = <DateRangeValue>[];
    await tester.pumpWidget(picker(onChanged: changes.add));

    await tester.tap(find.byType(DateRangePicker));
    await tester.pump();
    expect(find.text('Done'), findsOneWidget);

    await tester.tap(find.text('Quick pick'));
    await tester.pump();
    await tester.tap(find.text('Tomorrow'));
    await tester.pump();

    // The column and the footer follow the draft; the value does not.
    expect(find.textContaining('Sat, Oct 17'), findsWidgets);
    expect(changes, isEmpty);

    await tester.tap(find.text('Done'));
    await tester.pump();
    expect(changes.single.start, DateTime(2026, 10, 17, 14, 30));
    expect(changes.single.end, DateTime(2026, 10, 17, 15, 30));
    expect(find.text('Done'), findsNothing);
  });

  testWidgets('an outside press puts the sheet back', (tester) async {
    final List<DateRangeValue> changes = <DateRangeValue>[];
    await tester.pumpWidget(picker(onChanged: changes.add));

    await tester.tap(find.byType(DateRangePicker));
    await tester.pump();
    await tester.tap(find.text('All day'));
    await tester.pump();
    expect(find.textContaining('All day'), findsWidgets);

    await tester.tapAt(const Offset(5, 5));
    await tester.pump();
    expect(find.text('Done'), findsNothing);
    expect(changes, isEmpty);

    await tester.tap(find.byType(DateRangePicker));
    await tester.pump();
    // The draft opens on the value again, not on what was abandoned.
    expect(find.textContaining('14:30 → 15:30'), findsWidgets);
  });

  testWidgets('a duration sets the end from the start, and is offered only '
      'for one day', (tester) async {
    final List<DateRangeValue> changes = <DateRangeValue>[];
    await tester.pumpWidget(picker(onChanged: changes.add));

    await tester.tap(find.byType(DateRangePicker));
    await tester.pump();
    await tester.tap(find.text('2 hours'));
    await tester.pump();
    await tester.tap(find.text('Done'));
    await tester.pump();
    expect(changes.single.end, DateTime(2026, 10, 16, 16, 30));

    await tester.tap(find.byType(DateRangePicker));
    await tester.pump();
    await tester.tap(find.text('Quick pick'));
    await tester.pump();
    await tester.tap(find.text('Next week'));
    await tester.pump();
    final Toggle chip = tester.widget(
      find.ancestor(of: find.text('2 hours'), matching: find.byType(Toggle)),
    );
    expect(chip.enabled, isFalse);
  });
}
