import 'package:dazzui/dazzui.dart';
import 'package:dazzui/src/foundation/inline_outset.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart' as app;

void main() {
  Widget host(Widget child, {ThemeData? theme}) => app.host(
    Center(child: SizedBox(width: 400, child: child)),
    theme: theme,
  );
  Finder button() => find.descendant(
    of: find.byType(PreferenceRow),
    matching: find.byType(Pressable),
  );

  testWidgets('opening rows expand the real button and retain text alignment', (
    tester,
  ) async {
    int opened = 0;
    await tester.pumpWidget(
      host(
        Preferences(
          children: [
            PreferenceSection(
              children: [
                const PreferenceRow(title: 'Inert'),
                PreferenceRow(title: 'Open', onOpen: () => opened++),
              ],
            ),
          ],
        ),
      ),
    );
    final Rect allocated = tester.getRect(find.byType(InlineOutset));
    final Rect box = tester.getRect(button());
    expect(box.width, allocated.width + 2 * themeVariables.spacing2);
    expect(box.left, allocated.left - themeVariables.spacing2);
    expect(
      tester.getTopLeft(find.text('Open')).dx,
      tester.getTopLeft(find.text('Inert')).dx,
    );
    expect(find.text('›'), findsOneWidget);
    await tester.tapAt(Offset(box.left + 1, box.center.dy));
    await tester.tapAt(Offset(box.right - 1, box.center.dy));
    expect(opened, 2, reason: 'Both negative margins receive real taps');
    await tester.tapAt(box.topLeft + const Offset(.2, .2));
    expect(
      opened,
      2,
      reason: 'The rounded corner cutout is outside the button',
    );
    Focus.of(tester.element(find.text('Open'))).requestFocus();
    await tester.pump();
    await tester.sendKeyEvent(LogicalKeyboardKey.enter);
    expect(opened, 3);
  });

  testWidgets('the painted hover radius follows every registered theme', (
    tester,
  ) async {
    final mouse = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await mouse.addPointer(location: const Offset(1, 1));
    for (final entry in themeVariablesByName.entries) {
      final vars = entry.value;
      final theme = ThemeData(
        vars: vars,
        brightness: entry.key.endsWith('-dark')
            ? Brightness.dark
            : Brightness.light,
      );
      await tester.pumpWidget(
        host(
          PreferenceRow(
            title: 'Open',
            tint: PreferenceRowTint.danger,
            onOpen: () {},
          ),
          theme: theme,
        ),
      );
      final box = tester.getRect(button());
      await mouse.moveTo(Offset(box.left + 1, box.center.dy));
      await tester.pumpAndSettle();
      final surface = find.descendant(
        of: button(),
        matching: find.byType(DecoratedBox),
      );
      final decoration =
          tester.widget<DecoratedBox>(surface).decoration as BoxDecoration;
      expect(tester.getRect(surface), box);
      expect(
        decoration.borderRadius,
        BorderRadius.circular(vars.controlContainerRadius),
        reason: entry.key,
      );
      expect(
        decoration.color,
        vars.colorDanger[vars.controlColorPlainSurface.hoveredShade!]!
            .withValues(alpha: vars.controlColorPlainSurface.hoveredOpacity),
        reason: entry.key,
      );
      await mouse.moveTo(const Offset(1, 1));
      await tester.pumpAndSettle();
    }
    await mouse.removePointer();
  });

  testWidgets('clipping and pointer blockers exclude the negative margin', (
    tester,
  ) async {
    int opened = 0;
    for (final wrap in <Widget Function(Widget)>[
      (child) => ClipRect(child: child),
      (child) => IgnorePointer(child: child),
      (child) => AbsorbPointer(child: child),
    ]) {
      await tester.pumpWidget(
        host(wrap(PreferenceRow(title: 'Open', onOpen: () => opened++))),
      );
      final box = tester.getRect(button());
      await tester.tapAt(Offset(box.left + 1, box.center.dy));
      expect(opened, 0);
    }
  });

  testWidgets(
    'removing interaction removes the arrow, focus and old hit region',
    (tester) async {
      await tester.pumpWidget(
        host(PreferenceRow(title: 'Open', onOpen: () {})),
      );
      await tester.pumpWidget(
        host(const PreferenceRow(title: 'Open', subtitle: '')),
      );
      expect(button(), findsNothing);
      expect(find.text('›'), findsNothing);
      expect(tester.takeException(), isNull);
      expect(
        tester.getSize(find.byType(PreferenceRow)).height,
        themeVariables.controlMediumSize,
      );
    },
  );

  testWidgets(
    'rich headings, descriptions and row content retain their widgets',
    (tester) async {
      const title = SizedBox(key: ValueKey('title'), width: 60, height: 20);
      const subtitle = Text('Rich subtitle');
      await tester.pumpWidget(
        host(
          Preferences(
            children: [
              PreferenceGroup(
                title: const Text('Group'),
                description: const Text('Description'),
                action: Button(
                  onPressed: () {},
                  child: const Text('Group action'),
                ),
                children: [
                  PreferenceSection(
                    label: const Text('Section'),
                    footer: const Text('Footer'),
                    action: Button(
                      onPressed: () {},
                      child: const Text('Section action'),
                    ),
                    children: const [
                      PreferenceRow(
                        title: title,
                        subtitle: subtitle,
                        trailing: Badge(child: Text('Ready')),
                      ),
                    ],
                  ),
                ],
              ),
            ],
          ),
        ),
      );
      expect(find.byWidget(title), findsOneWidget);
      expect(find.byWidget(subtitle), findsOneWidget);
      expect(find.text('Group action'), findsOneWidget);
      expect(find.text('Section action'), findsOneWidget);
      expect(tester.takeException(), isNull);
    },
  );
  testWidgets(
    'foreground controls receive the outset before the row behind them',
    (tester) async {
      int opened = 0, covered = 0;
      await tester.pumpWidget(
        app.host(
          Center(
            child: SizedBox(
              width: 416,
              height: 32,
              child: Stack(
                clipBehavior: Clip.none,
                children: [
                  Positioned(
                    left: 8,
                    top: 0,
                    width: 400,
                    child: PreferenceRow(title: 'Open', onOpen: () => opened++),
                  ),
                  Positioned(
                    left: 0,
                    top: 0,
                    bottom: 0,
                    width: 16,
                    child: GestureDetector(
                      behavior: HitTestBehavior.opaque,
                      onTap: () => covered++,
                      child: const SizedBox.expand(),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      );
      final box = tester.getRect(button());
      await tester.tapAt(Offset(box.left + 1, box.center.dy));
      expect(covered, 1);
      expect(opened, 0);
    },
  );

  testWidgets('scroll gestures bubble from the outset to their scroll view', (
    tester,
  ) async {
    int opened = 0;
    final scroll = ScrollController();
    await tester.pumpWidget(
      app.host(
        Center(
          child: SizedBox(
            width: 400,
            height: 200,
            child: SingleChildScrollView(
              controller: scroll,
              clipBehavior: Clip.none,
              child: Column(
                children: [
                  PreferenceRow(title: 'Open', onOpen: () => opened++),
                  const SizedBox(height: 600),
                ],
              ),
            ),
          ),
        ),
      ),
    );
    final box = tester.getRect(button());
    await tester.dragFrom(
      Offset(box.left + 1, box.center.dy),
      const Offset(0, -80),
    );
    await tester.pumpAndSettle();
    expect(scroll.offset, greaterThan(0));
    expect(opened, 0);
    await tester.pumpWidget(const SizedBox.shrink());
    scroll.dispose();
  });
  testWidgets(
    'theme corners change immediately rather than tweening between roles',
    (tester) async {
      Widget row() => PreferenceRow(title: 'Open', onOpen: () {});
      await tester.pumpWidget(host(row()));
      await tester.pumpWidget(host(row(), theme: ThemeData.brightLight()));
      final decoration =
          tester
                  .widget<DecoratedBox>(
                    find.descendant(
                      of: button(),
                      matching: find.byType(DecoratedBox),
                    ),
                  )
                  .decoration
              as BoxDecoration;
      expect(
        decoration.borderRadius,
        BorderRadius.circular(
          ThemeData.brightLight().vars.controlContainerRadius,
        ),
      );
      expect(
        tester.widget<Pressable>(button()).borderRadius,
        decoration.borderRadius,
      );
      await tester.pumpAndSettle();
    },
  );
}
