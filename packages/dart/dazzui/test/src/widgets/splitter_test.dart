import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart' as app;

/// `Splitter` — a seam a drag moves on the spot, and every other change of
/// width settles over the motion token.
void main() {
  const ThemeVariables vars = themeVariables;

  /// What two panels divide of the 600px splitter below.
  final double run = 600 - vars.spacing2;

  Finder seam() => find.byWidgetPredicate(
    (Widget widget) => widget is Semantics && widget.properties.slider == true,
  );

  Widget window({ValueChanged<List<double>>? onSizesChanged}) => app.host(
    Center(
      child: SizedBox(
        width: 600,
        height: 400,
        child: Splitter(
          onSizesChanged: onSizesChanged,
          panels: const [
            SplitterPanel(
              key: ValueKey('a'),
              defaultSize: 40,
              child: SizedBox.expand(),
            ),
            SplitterPanel(
              key: ValueKey('b'),
              defaultSize: 60,
              child: SizedBox.expand(),
            ),
          ],
        ),
      ),
    ),
  );

  double panel(WidgetTester tester, String key) =>
      tester.getSize(find.byKey(ValueKey(key))).width;

  testWidgets('a double-tap evens the pair over the token', (tester) async {
    await tester.pumpWidget(window());
    expect(panel(tester, 'a'), closeTo(run * 0.4, 0.5));

    // Two taps inside the double-tap window: the seam resets its pair rather
    // than being dragged.
    await tester.tap(seam());
    await tester.pump(const Duration(milliseconds: 40));
    await tester.tap(seam());
    await tester.pump();
    await tester.pump(vars.motionDuration ~/ 2);

    // Partway there — the width is in transit rather than landed.
    final double midway = panel(tester, 'a');
    expect(midway, greaterThan(run * 0.4 + 1));
    expect(midway, lessThan(run * 0.5 - 1));

    await tester.pumpAndSettle();
    expect(panel(tester, 'a'), closeTo(run * 0.5, 0.6));
    expect(panel(tester, 'b'), closeTo(run * 0.5, 0.6));
  });

  testWidgets('a drag lands on the spot, with nothing left to travel', (
    tester,
  ) async {
    await tester.pumpWidget(window());
    final double before = panel(tester, 'a');

    final TestGesture drag = await tester.startGesture(
      tester.getCenter(seam()),
    );
    // The first move is what the gesture uses to accept a drag, and its travel
    // is the slop every drag affordance has; the next one is a width.
    await drag.moveBy(const Offset(30, 0));
    await drag.moveBy(const Offset(20, 0));
    await tester.pump();

    // The seam followed the pointer in the frame the pointer moved: a drag
    // that settled would still be on its way.
    final double dragged = panel(tester, 'a');
    expect(dragged, greaterThan(before + 5));

    // Releasing asks for nothing more, and the panels stay where the hand
    // left them.
    await drag.up();
    await tester.pump();
    expect(panel(tester, 'a'), dragged);
    await tester.pumpAndSettle();
    expect(panel(tester, 'a'), dragged);
  });

  testWidgets('a settle a drag interrupts is caught where it stands', (
    tester,
  ) async {
    await tester.pumpWidget(window());
    await tester.tap(seam());
    await tester.pump(const Duration(milliseconds: 40));
    await tester.tap(seam());
    await tester.pump();
    await tester.pump(vars.motionDuration ~/ 2);

    // The settle is heading for an even pair; the pointer takes the seam
    // partway there.
    final double midway = panel(tester, 'a');
    expect(midway, greaterThan(run * 0.4 + 1));

    final TestGesture drag = await tester.startGesture(
      tester.getCenter(seam()),
    );
    // The first move is what the gesture uses to accept a drag; the second is
    // a width, applied from where the seam stood rather than from the even
    // pair the settle was heading for.
    await drag.moveBy(const Offset(30, 0));
    await drag.moveBy(const Offset(20, 0));
    await tester.pump();
    expect(panel(tester, 'a'), closeTo(midway + 20, 1));

    // And the settle it interrupted is over: the panels stay where the hand
    // left them.
    await drag.up();
    await tester.pumpAndSettle();
    expect(panel(tester, 'a'), closeTo(midway + 20, 1));
  });

  testWidgets('a host is handed the width a change commits', (tester) async {
    final List<List<double>> reported = <List<double>>[];
    await tester.pumpWidget(window(onSizesChanged: reported.add));

    await tester.tap(seam());
    await tester.pump(const Duration(milliseconds: 40));
    await tester.tap(seam());
    await tester.pump();

    // The state a host persists is the destination, once, whatever the
    // panels are drawing on their way there.
    expect(reported, hasLength(1));
    expect(reported.single, [50, 50]);
    await tester.pumpAndSettle();
    expect(reported, hasLength(1));
  });
}
