import 'dart:ui';

import 'package:dazzui_example/main.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('every page draws in every theme', (tester) async {
    tester.view.physicalSize = const Size(2160, 1440);
    tester.view.devicePixelRatio = 2.0;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(const PlaygroundApp());
    expect(find.text('DazzUI'), findsOneWidget);

    // pump with a duration rather than pumpAndSettle: the Data page's
    // loaders animate forever, so the tree never settles.
    for (final theme in ['Studio', 'Studio Dark', 'Bright', 'Bright Dark']) {
      await tester.tap(find.text(theme));
      await tester.pump(const Duration(milliseconds: 300));

      for (final page in [
        'Forms',
        'Data',
        'Feedback',
        'Settings',
        'Themes',
        'Controls',
      ]) {
        await tester.tap(find.text(page).first);
        await tester.pump(const Duration(milliseconds: 300));
        expect(tester.takeException(), isNull, reason: '$theme / $page');
      }
    }
  });
}
