import 'dart:ffi' as ffi;
import 'dart:io';

import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/host.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

typedef _ObserveNative = ffi.Void Function();
typedef _CountNative = ffi.Int32 Function();
typedef _KeyNative = ffi.Void Function(ffi.Uint16);
typedef _MoveNative = ffi.Void Function(ffi.Double, ffi.Double);

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets(
    'nativeapi opens, selects, cancels and reopens real macOS menus',
    (tester) async {
      if (!Platform.isMacOS) fail('This integration test requires macOS');
      final driver = ffi.DynamicLibrary.process();
      final observe = driver.lookupFunction<_ObserveNative, void Function()>(
        'dazzui_observe_menus',
      );
      final count = driver.lookupFunction<_CountNative, int Function()>(
        'dazzui_menu_count',
      );
      final key = driver.lookupFunction<_KeyNative, void Function(int)>(
        'dazzui_menu_key',
      );
      final move = driver
          .lookupFunction<_MoveNative, void Function(double, double)>(
            'dazzui_move_pointer',
          );
      final stop = driver.lookupFunction<_ObserveNative, void Function()>(
        'dazzui_stop_menu_driver',
      );
      addTearDown(stop);
      observe();
      String? chosen;
      var calls = 0;
      await tester.pumpWidget(
        Host(
          title: 'DazzUI native menu integration',
          home: MenuConfiguration(
            presentation: MenuPresentation.native,
            child: Theme(
              data: ThemeData.studioLight(),
              child: Center(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Menubar(
                      menus: [
                        MenubarMenu(
                          label: 'File',
                          items: [
                            MenuItem(label: 'New', onSelect: () => calls += 10),
                          ],
                        ),
                        MenubarMenu(
                          label: 'Edit',
                          items: [
                            MenuItem(
                              label: 'Copy',
                              onSelect: () => calls += 100,
                            ),
                          ],
                        ),
                      ],
                    ),
                    Menu(
                      items: [
                        MenuItem(label: 'Run', onSelect: () => calls++),
                        const MenuItem(label: 'Disabled', enabled: false),
                      ],
                      trigger: (context, state) => Button(
                        onPressed: state.toggle,
                        child: const Text('Actions'),
                      ),
                    ),
                    SizedBox(
                      width: 240,
                      child: Select<String>(
                        options: const [
                          SelectOption(value: 'a', label: 'Alpha'),
                          SelectOption(value: 'b', label: 'Beta'),
                        ],
                        onChanged: (value) => chosen = value,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      Future<void> waitFor(bool Function() condition) =>
          tester.runAsync(() async {
            final deadline = DateTime.now().add(const Duration(seconds: 5));
            while (!condition() && DateTime.now().isBefore(deadline)) {
              await Future<void>.delayed(const Duration(milliseconds: 50));
            }
            expect(condition(), isTrue);
          });
      await tester.tap(find.text('Actions'));
      await tester.pump();
      await waitFor(() => count() == 2);
      expect(find.byType(MenuPanel), findsNothing);
      key(1000);
      await waitFor(() => calls == 1);
      await tester.pumpAndSettle();
      await tester.tap(find.text('Actions'));
      await tester.pump();
      await waitFor(() => count() == 2);
      key(53);
      await waitFor(() => count() == 0);
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 150)),
      );
      await tester.pumpAndSettle();
      expect(calls, 1);
      await tester.tap(find.byType(Select<String>));
      await tester.pump();
      await waitFor(() => count() == 2);
      key(1000);
      await waitFor(() => chosen != null);
      await tester.pumpAndSettle();
      expect(chosen, 'a');
      await tester.tap(find.text('File'));
      await tester.pump();
      await waitFor(() => count() == 1);
      final edit = tester.getCenter(find.text('Edit'));
      move(edit.dx, edit.dy);
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 350)),
      );
      key(125);
      await tester.runAsync(
        () => Future<void>.delayed(const Duration(milliseconds: 150)),
      );
      key(36);
      await waitFor(() => calls == 101);
      await tester.pumpAndSettle();
      await tester.pumpWidget(const SizedBox());
      await tester.pumpAndSettle();
    },
  );
}
