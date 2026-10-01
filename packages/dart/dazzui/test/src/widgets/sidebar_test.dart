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
}
