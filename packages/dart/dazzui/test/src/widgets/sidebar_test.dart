import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart' as app;

/// `Sidebar` and `SidebarGroup` — the column's vertical rhythm, which React
/// draws with `.dz-sidebar__content` and `.dz-sidebar__group`.
void main() {
  testWidgets('groups stand a step apart; a label sits on its rows', (
    tester,
  ) async {
    const ThemeVariables vars = themeVariables;
    await tester.pumpWidget(
      app.host(
        Sidebar(
          children: [
            SidebarGroup(
              label: 'Mailboxes',
              children: [
                NavItem(label: 'Inbox', onPressed: () {}),
                NavItem(label: 'Sent', onPressed: () {}),
              ],
            ),
            SidebarGroup(
              label: 'Labels',
              children: [NavItem(label: 'Work', onPressed: () {})],
            ),
          ],
        ),
      ),
    );

    Rect rect(String text) => tester.getRect(
      find.ancestor(of: find.text(text), matching: find.byType(NavItem)),
    );
    Rect label(String text) => tester.getRect(find.text(text));

    // Between two groups: the first one's bottom padding and the second's
    // top padding, then the label's own — not the rows' 3px nav gap.
    final double between = label('Labels').top - rect('Sent').bottom;
    expect(between, greaterThanOrEqualTo(vars.spacing15 * 2 + vars.spacing1));

    // Rows in a group keep the nav gap.
    expect(rect('Sent').top - rect('Inbox').bottom, vars.frameNavGap);

    // A label sits closer to its own rows than to the group above it.
    final double toRows = rect('Inbox').top - label('Mailboxes').bottom;
    expect(toRows, lessThan(between));
  });

  testWidgets('a column put away sweeps shut behind its own clip', (
    tester,
  ) async {
    const ThemeVariables vars = themeVariables;
    final SemanticsHandle semantics = tester.ensureSemantics();

    // What an assistive technology reads, rather than what the widget tree
    // still holds: a dropped semantics node can be stale on its render object.
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
          Sidebar(
            collapsed: collapsed,
            children: [
              SidebarGroup(
                label: 'Mailboxes',
                children: [NavItem(label: 'Inbox', onPressed: () {})],
              ),
            ],
          ),
          const Expanded(child: SizedBox()),
        ],
      ),
    );

    await tester.pumpWidget(window(collapsed: false));
    final double open = tester.getSize(find.byType(Sidebar)).width;
    expect(open, vars.frameSidebarWidth);
    // The row is laid out on the column's open content box.
    final double row = tester.getSize(find.byType(NavItem)).width;
    expect(tree(), contains('Inbox'));

    await tester.pumpWidget(window(collapsed: true));
    await tester.pump();
    await tester.pump(vars.motionDuration ~/ 2);

    // The column is part of the way shut, and the pane beside it grew with
    // the edge rather than jumping a column's width in a frame.
    final double halfway = tester.getSize(find.byType(Sidebar)).width;
    expect(halfway, greaterThan(0));
    expect(halfway, lessThan(open));
    // And it is cut by that edge rather than laying itself out again: a row
    // still measures what it did when the column was open.
    expect(tester.getSize(find.byType(NavItem)).width, row);

    await tester.pumpAndSettle();
    expect(tester.getSize(find.byType(Sidebar)).width, 0);
    // Shut, there is nothing there to read.
    expect(tree(), isNot(contains('Inbox')));

    // And opening it again brings the row back — the column was never
    // unmounted, so it is the same row rather than a rebuilt one.
    await tester.pumpWidget(window(collapsed: false));
    await tester.pumpAndSettle();
    expect(tester.getSize(find.byType(Sidebar)).width, open);
    expect(tree(), contains('Inbox'));

    semantics.dispose();
  });
}
