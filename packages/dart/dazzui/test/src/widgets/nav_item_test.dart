import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart' as app;

/// `NavItem` — the row's fill and its ink cross to the selected state
/// together, over the motion token.
void main() {
  testWidgets('the ink crosses with the fill', (tester) async {
    const ThemeVariables vars = themeVariables;

    Widget row({required bool current}) => app.host(
      Center(
        child: SizedBox(
          width: 240,
          child: NavItem(label: 'Inbox', current: current, onPressed: () {}),
        ),
      ),
    );

    Color? ink() => tester.widget<Text>(find.text('Inbox')).style?.color;

    await tester.pumpWidget(row(current: false));
    final Color idle = ink()!;
    expect(idle, vars.colorContentNav);

    await tester.pumpWidget(row(current: true));
    await tester.pump(vars.motionDuration ~/ 2);

    // Halfway, the label is neither colour: the fill has been crossing since
    // the state changed and the ink has to keep up with it, or the selection
    // would wash a row in and leave its label a beat behind.
    final Color midway = ink()!;
    expect(midway, isNot(idle));
    expect(midway, isNot(vars.colorOnAccent));

    await tester.pumpAndSettle();
    expect(ink(), vars.colorOnAccent);
  });
}
