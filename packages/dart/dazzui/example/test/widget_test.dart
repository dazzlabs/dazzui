import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/app.dart';
import 'package:dazzui_example/lib/themes.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('the playground opens its component overview', (tester) async {
    tester.view.physicalSize = const Size(1360, 860);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(const App());
    await tester.pump(const Duration(seconds: 1));

    expect(find.text('Overview'), findsWidgets);
    expect(find.text('Button'), findsWidgets);
    expect(tester.takeException(), isNull);
  });

  for (final width in [800.0, 1280.0, 1360.0]) {
    for (final dark in [false, true]) {
      testWidgets('Spotify overview fits at $width, dark=$dark', (
        tester,
      ) async {
        tester.view.physicalSize = Size(width, 860);
        tester.view.devicePixelRatio = 1;
        addTearDown(tester.view.reset);

        await tester.pumpWidget(App(theme: ThemeChoice('spotify', dark: dark)));
        await tester.pump(const Duration(seconds: 1));
        expect(find.text('Overview'), findsWidgets);
        expect(tester.takeException(), isNull);
      });
    }
  }
}
