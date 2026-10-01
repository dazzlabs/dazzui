import 'package:dazzui/dazzui.dart';
import 'package:flutter_mail_example/main.dart';
import 'package:flutter_mail_example/settings.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('the status bar switches open above the bar and apply', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1360, 860);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(const MailApp());
    await tester.pumpAndSettle();

    // The first switch is the locale; it sits at the foot of the window, so
    // its list has to open upward to be seen at all.
    final Finder locale = find.byWidgetPredicate((w) => w is Select).first;
    await tester.tap(locale);
    await tester.pumpAndSettle();

    final Rect trigger = tester.getRect(locale);
    final Rect panel = tester.getRect(find.byType(MenuPanel));
    expect(panel.bottom, lessThanOrEqualTo(trigger.top));

    await tester.tap(
      find.descendant(
        of: find.byType(MenuPanel),
        matching: find.text('繁體中文（台灣）'),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.byType(MenuPanel), findsNothing);
    expect(find.textContaining('zh-Hant-TW'), findsOneWidget);
    // The window speaks the language it was switched to.
    expect(find.text('撰寫郵件'), findsOneWidget);
    expect(find.text('收件匣'), findsWidgets);
    expect(find.text('写邮件'), findsNothing);
  });

  for (final (locale, compose, inbox) in [
    ('zhHansCN', '写邮件', '收件箱'),
    ('zhHantHK', '撰寫郵件', '收件匣'),
    ('jaJP', '新規メール', '受信トレイ'),
    ('enUS', 'New Message', 'Inbox'),
  ]) {
    testWidgets('the window speaks $locale', (tester) async {
      tester.view.physicalSize = const Size(1360, 860);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.reset);

      await tester.pumpWidget(
        MailApp(
          settings: Settings.fromEnvironment({'FLUTTER_MAIL_LOCALE': locale}),
        ),
      );
      await tester.pumpAndSettle();

      expect(find.text(compose), findsOneWidget);
      expect(find.text(inbox), findsWidgets);
    });
  }
}
