import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart' as app;

/// `Aside` — the trailing column collapses the way the leading one does,
/// mirrored: what it holds holds still against the window's own edge while
/// the edge facing the content slides over it.
void main() {
  const ThemeVariables vars = themeVariables;

  testWidgets('a panel put away is cut by its inner edge', (tester) async {
    final SemanticsHandle semantics = tester.ensureSemantics();

    String tree() => tester
        .binding
        .renderViews
        .first
        .owner!
        .semanticsOwner!
        .rootSemanticsNode!
        .toStringDeep();

    Widget window({required bool collapsed}) => app.host(
      Row(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Expanded(child: SizedBox(key: ValueKey('pane'))),
          Aside(
            width: 320,
            collapsed: collapsed,
            child: const Center(child: Text('Theme')),
          ),
        ],
      ),
    );

    await tester.pumpWidget(window(collapsed: false));
    expect(tester.getSize(find.byType(Aside)).width, 320);
    final double pane = tester
        .getSize(find.byKey(const ValueKey('pane')))
        .width;
    final Rect held = tester.getRect(find.text('Theme'));
    expect(tree(), contains('Theme'));

    await tester.pumpWidget(window(collapsed: true));
    await tester.pump();
    await tester.pump(vars.motionDuration ~/ 2);

    final double halfway = tester.getSize(find.byType(Aside)).width;
    expect(halfway, greaterThan(0));
    expect(halfway, lessThan(320));
    // The panel is cut, not laid out again: what it holds has not moved.
    expect(tester.getRect(find.text('Theme')), held);
    // And the document beside it took exactly the room the panel gave up.
    expect(
      tester.getSize(find.byKey(const ValueKey('pane'))).width,
      closeTo(pane + (320 - halfway), 0.5),
    );

    await tester.pumpAndSettle();
    expect(tester.getSize(find.byType(Aside)).width, 0);
    // Shut, there is nothing there to read.
    expect(tree(), isNot(contains('Theme')));

    // Opening it again brings the panel back over the document.
    await tester.pumpWidget(window(collapsed: false));
    await tester.pumpAndSettle();
    expect(tester.getSize(find.byType(Aside)).width, 320);
    expect(tester.getRect(find.text('Theme')), held);
    expect(tree(), contains('Theme'));

    semantics.dispose();
  });
}
