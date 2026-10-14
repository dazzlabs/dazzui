import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

/// `Select` — the field-shaped trigger and the list it drops.
///
/// The geometry the React stylesheet decides is held by the reference test; what
/// is here is what only this port can get wrong: the trigger reading the
/// field's own box, the list being at least as wide as the trigger, and the
/// options being rows rather than a second drawing.
void main() {
  const List<SelectOption<String>> languages = [
    SelectOption(value: 'zh', label: 'Chinese (Simplified)'),
    SelectOption(value: 'ja', label: 'Japanese'),
    SelectOption(value: 'de', label: 'German'),
  ];

  testWidgets('mixed options and groups keep order, choice and disabled rows', (
    tester,
  ) async {
    String? selected;
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 240,
            child: Select<String>(
              options: const [
                SelectOption(value: 'none', label: 'Unassigned'),
                SelectGroup(
                  label: 'People',
                  options: [
                    SelectOption(value: 'ada', label: 'Ada'),
                    SelectOption(
                      value: 'grace',
                      label: 'Grace',
                      enabled: false,
                    ),
                  ],
                ),
                SelectOption(value: 'bot', label: 'Agent'),
              ],
              value: 'ada',
              onChanged: (value) => selected = value,
            ),
          ),
        ),
      ),
    );
    expect(find.text('Ada'), findsNWidgets(2));
    await tester.tap(find.byType(Select<String>));
    await tester.pumpAndSettle();
    Finder row(String text) => find.descendant(
      of: find.byType(MenuPanel),
      matching: find.text(text),
    );
    final positions = [
      for (final text in ['Unassigned', 'People', 'Ada', 'Grace', 'Agent'])
        tester.getTopLeft(row(text)).dy,
    ];
    expect(positions, orderedEquals([...positions]..sort()));
    await tester.tap(row('Grace'));
    await tester.pump();
    expect(selected, isNull);
    expect(find.byType(MenuPanel), findsOneWidget);
    await tester.tap(row('Agent'));
    await tester.pump();
    expect(selected, 'bot');
    expect(find.byType(MenuPanel), findsNothing);
  });

  testWidgets('the trigger shows the chosen label, or the placeholder', (
    tester,
  ) async {
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 240,
            child: Select<String>(
              options: languages,
              value: 'ja',
              placeholder: 'Choose a language…',
              onChanged: (_) {},
            ),
          ),
        ),
      ),
    );

    // One `Japanese` is the value; the other is the hidden label the box
    // measures itself against, and there is one of those per option.
    expect(find.text('Japanese'), findsNWidgets(2));
    expect(
      find.descendant(
        of: find.byType(Visibility),
        matching: find.byType(Text),
      ),
      findsNWidgets(languages.length),
    );

    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 240,
            child: Select<String>(
              options: languages,
              placeholder: 'Choose a language…',
              onChanged: (_) {},
            ),
          ),
        ),
      ),
    );

    // No choice, so the value line is the placeholder; the labels are all
    // hidden and only there to hold the width.
    expect(find.text('Choose a language…'), findsOneWidget);
    expect(
      find.descendant(
        of: find.byType(Visibility),
        matching: find.byType(Text),
      ),
      findsNWidgets(languages.length),
    );
  });

  testWidgets('the trigger is the field box, at the size profile height', (
    tester,
  ) async {
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 240,
            child: Select<String>(
              options: languages,
              value: 'zh',
              onChanged: (_) {},
            ),
          ),
        ),
      ),
    );

    final ThemeVariables vars = ThemeData.studioLight().vars;
    expect(
      tester.getSize(find.byType(Select<String>)).height,
      vars.controlMediumSize,
    );
  });

  testWidgets('the list is at least as wide as the trigger it dropped from', (
    tester,
  ) async {
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 320,
            child: Select<String>(
              options: languages,
              value: 'zh',
              onChanged: (_) {},
            ),
          ),
        ),
      ),
    );

    await tester.tap(find.byType(Select<String>));
    await tester.pump();

    // The stylesheet's `min-width: var(--anchor-width)`: a list narrower than
    // the trigger reads as a different control.
    expect(tester.getSize(find.byType(MenuPanel)).width, 320);
    // Once in the open list, once in the trigger's hidden measurements.
    expect(find.text('German'), findsNWidgets(2));
  });

  testWidgets('choosing a row reports the value and closes the list', (
    tester,
  ) async {
    String? chosen = 'zh';

    await tester.pumpWidget(
      host(
        Center(
          child: StatefulBuilder(
            builder: (context, setState) => SizedBox(
              width: 240,
              child: Select<String>(
                options: languages,
                value: chosen,
                onChanged: (value) => setState(() => chosen = value),
              ),
            ),
          ),
        ),
      ),
    );

    await tester.tap(find.byType(Select<String>));
    await tester.pump();
    await tester.tap(
      find.descendant(
        of: find.byType(MenuPanel),
        matching: find.text('German'),
      ),
    );
    await tester.pump();

    expect(chosen, 'de');
    expect(find.byType(MenuPanel), findsNothing);
    // The trigger now reads German, beside the hidden label of the same name.
    expect(find.text('German'), findsNWidgets(2));
  });

  testWidgets('a select with no handler is a disabled one', (tester) async {
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 240,
            child: Select<String>(options: languages, value: 'zh'),
          ),
        ),
      ),
    );

    await tester.tap(find.byType(Select<String>));
    await tester.pump();

    // No list, because there is nothing to choose with: the two are the same
    // fact about the control rather than a separate `enabled` flag.
    expect(find.byType(MenuPanel), findsNothing);
  });

  testWidgets('a select in a shrink-wrapping row is measured from its labels', (
    tester,
  ) async {
    // A `Row` whose width is unbounded refuses a flex child outright, so this
    // is the case that decides whether the value may be laid over the hidden
    // labels at all. A select here is as wide as its longest option.
    await tester.pumpWidget(
      host(
        Center(
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Select<String>(
                options: languages,
                value: 'zh',
                onChanged: (_) {},
              ),
            ],
          ),
        ),
      ),
    );

    expect(tester.takeException(), isNull);
    expect(
      tester.getSize(find.byType(Select<String>)).width,
      greaterThan(0),
    );
  });

  testWidgets('groups drop as a heading over their rows', (tester) async {
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 240,
            child: Select<String>(
              groups: const [
                SelectGroup(
                  label: 'Recently used',
                  options: [SelectOption(value: 'zh', label: 'Chinese')],
                ),
                SelectGroup(
                  label: 'All languages',
                  options: [SelectOption(value: 'de', label: 'German')],
                ),
              ],
              value: 'zh',
              onChanged: (_) {},
            ),
          ),
        ),
      ),
    );

    await tester.tap(find.byType(Select<String>));
    await tester.pump();

    expect(
      find.descendant(
        of: find.byType(MenuPanel),
        matching: find.text('Recently used'),
      ),
      findsOneWidget,
    );
    expect(
      find.descendant(
        of: find.byType(MenuPanel),
        matching: find.text('All languages'),
      ),
      findsOneWidget,
    );
  });

  testWidgets(
    'a list longer than the room below scrolls, opened on its choice',
    (
      tester,
    ) async {
      tester.view.physicalSize = const Size(400, 300);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.reset);
      final List<SelectOption<int>> many = [
        for (int i = 0; i < 40; i++) SelectOption(value: i, label: 'Option $i'),
      ];

      await tester.pumpWidget(
        host(
          Align(
            alignment: Alignment.topCenter,
            child: Padding(
              padding: const EdgeInsets.only(top: 40),
              child: SizedBox(
                width: 200,
                child: Select<int>(options: many, value: 30, onChanged: (_) {}),
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.byType(Select<int>));
      await tester.pumpAndSettle();

      // `max-height: var(--available-height)`: the panel stops short of the
      // window's bottom edge, by Base UI's collision padding, and draws no
      // overflow.
      expect(tester.takeException(), isNull);
      final Rect panel = tester.getRect(find.byType(MenuPanel));
      expect(panel.bottom, lessThanOrEqualTo(300 - 5));
      // The choice is in view; the first rows have scrolled away.
      final Finder chosen = find.descendant(
        of: find.byType(MenuPanel),
        matching: find.text('Option 30'),
      );
      expect(panel.contains(tester.getCenter(chosen)), isTrue);
      expect(
        panel.contains(
          tester.getCenter(
            find.descendant(
              of: find.byType(MenuPanel),
              matching: find.text('Option 0'),
            ),
          ),
        ),
        isFalse,
      );

      // The rows still answer after scrolling.
      await tester.tap(chosen);
      await tester.pump();
      expect(find.byType(MenuPanel), findsNothing);
    },
  );

  testWidgets('a select at the foot of the window opens upward', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(400, 300);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(
      host(
        Align(
          alignment: Alignment.bottomLeft,
          child: SizedBox(
            width: 240,
            child: Select<String>(
              options: languages,
              value: 'ja',
              onChanged: (_) {},
            ),
          ),
        ),
      ),
    );

    await tester.tap(find.byType(Select<String>));
    await tester.pumpAndSettle();

    // Base UI's `flip`: with no room below, the list sits above its trigger
    // at full height rather than being clamped to nothing.
    expect(tester.takeException(), isNull);
    final Rect trigger = tester.getRect(find.byType(Select<String>));
    final Rect panel = tester.getRect(find.byType(MenuPanel));
    expect(panel.bottom, lessThanOrEqualTo(trigger.top));
    for (final SelectOption<String> option in languages) {
      expect(
        find.descendant(
          of: find.byType(MenuPanel),
          matching: find.text(option.label),
        ),
        findsOneWidget,
      );
    }
    expect(panel.height, greaterThan(languages.length * 20));
  });

  testWidgets('a wide select holds its chevron at the end edge', (
    tester,
  ) async {
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 480,
            child: Select<String>(
              options: languages,
              value: 'ja',
              onChanged: (_) {},
            ),
          ),
        ),
      ),
    );

    // The box is far wider than its longest label; the chevron still sits
    // at the end, inside the field's padding, not just past the text.
    final Rect box = tester.getRect(find.byType(Select<String>));
    final Rect chevron = tester.getRect(
      find.descendant(
        of: find.byType(Select<String>),
        matching: find.byType(Icon),
      ),
    );
    expect(box.right - chevron.right, lessThanOrEqualTo(16));
  });
}
