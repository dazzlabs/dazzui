import 'dart:async';
import 'dart:typed_data';

import 'package:dazzui/dazzui.dart';
import 'package:dazzui/src/foundation/native_menu.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

class _Session implements NativeMenuSession {
  bool closed = false;
  @override
  void close() => closed = true;
}

class _Backend implements NativeMenuBackend {
  @override
  bool supported = true;
  @override
  Offset? pointerPosition;
  bool fail = false;
  final sessions = <_Session>[];
  List<NativeMenuEntry> entries = [];
  late VoidCallback dismiss;
  late ValueChanged<int> select;
  Offset? position;
  Completer<NativeMenuSession?>? pending;

  @override
  Future<NativeMenuSession?> open({
    required List<NativeMenuEntry> entries,
    required List<Uint8List?> icons,
    required Offset position,
    required bool alignEnd,
    required VoidCallback onDismiss,
    required ValueChanged<int> onSelect,
  }) async {
    this.entries = entries;
    this.position = position;
    dismiss = onDismiss;
    select = onSelect;
    if (fail) return null;
    final session = _Session();
    sessions.add(session);
    return pending == null ? session : pending!.future;
  }
}

Widget _configured(
  Widget child, {
  MenuPresentation mode = MenuPresentation.native,
}) => host(
  MenuConfiguration(
    presentation: mode,
    child: Center(child: child),
  ),
);

Widget _menu({VoidCallback? action}) => Menu(
  items: [MenuItem(label: 'Run action', onSelect: action)],
  trigger: (context, state) => Button(
    onPressed: state.toggle,
    child: Text(state.open ? 'Close actions' : 'Actions'),
  ),
);

void main() {
  late _Backend backend;
  setUp(() {
    backend = _Backend();
    debugNativeMenuBackend = backend;
  });
  tearDown(() => debugNativeMenuBackend = null);

  testWidgets('default and nearest custom scope use the drawn menu', (
    tester,
  ) async {
    for (final child in [
      _menu(),
      MenuConfiguration(
        presentation: MenuPresentation.native,
        child: MenuConfiguration(
          presentation: MenuPresentation.custom,
          child: _menu(),
        ),
      ),
    ]) {
      await tester.pumpWidget(host(Center(child: child)));
      await tester.tap(find.text('Actions'));
      await tester.pumpAndSettle();
      expect(find.byType(MenuPanel), findsOneWidget);
      expect(backend.sessions, isEmpty);
      await tester.pumpWidget(const SizedBox());
    }
  });

  testWidgets('native action runs once and closes its trigger', (tester) async {
    var calls = 0;
    await tester.pumpWidget(_configured(_menu(action: () => calls++)));
    await tester.tap(find.text('Actions'));
    await tester.pumpAndSettle();
    expect(find.byType(MenuPanel), findsNothing);
    expect(backend.entries.single.label, 'Run action');
    expect(find.text('Close actions'), findsOneWidget);
    backend.select(0);
    backend.select(0);
    await tester.pump();
    expect(calls, 1);
    expect(backend.sessions.single.closed, isTrue);
    expect(find.text('Actions'), findsOneWidget);
  });

  testWidgets('native cancellation and unavailable backend allow reopening', (
    tester,
  ) async {
    await tester.pumpWidget(_configured(_menu()));
    await tester.tap(find.text('Actions'));
    await tester.pumpAndSettle();
    backend.dismiss();
    await tester.pump();
    expect(find.text('Actions'), findsOneWidget);
    backend.fail = true;
    await tester.tap(find.text('Actions'));
    await tester.pumpAndSettle();
    expect(find.byType(MenuPanel), findsOneWidget);
  });

  testWidgets(
    'grouped Select preserves order checks disabled state and value',
    (tester) async {
      String? chosen;
      await tester.pumpWidget(
        _configured(
          SizedBox(
            width: 240,
            child: Select<String>(
              value: 'ada',
              options: const [
                SelectOption(value: 'none', label: 'Unassigned'),
                SelectGroup(
                  label: 'People',
                  options: [
                    SelectOption(value: 'ada', label: 'Ada'),
                    SelectOption(
                      value: 'grace',
                      label: 'Grace',
                      enabled: false,
                    ),
                  ],
                ),
              ],
              onChanged: (value) => chosen = value,
            ),
          ),
        ),
      );
      await tester.tap(find.byType(Select<String>));
      await tester.pumpAndSettle();
      expect(backend.entries.map((item) => item.label), [
        'Unassigned',
        'People',
        'Ada',
        'Grace',
      ]);
      expect(backend.entries[1].heading, isTrue);
      expect(backend.entries[2].checked, isTrue);
      backend.select(3);
      expect(chosen, isNull);
      backend.select(0);
      expect(chosen, 'none');
      await tester.pump();
    },
  );

  testWidgets('context menus use the pointer position', (tester) async {
    await tester.pumpWidget(
      _configured(
        ContextMenu(
          items: const [MenuItem(label: 'Inspect')],
          child: const SizedBox(width: 100, height: 100, child: Text('Target')),
        ),
      ),
    );
    final at = tester.getCenter(find.text('Target'));
    final gesture = await tester.startGesture(
      at,
      kind: PointerDeviceKind.mouse,
      buttons: kSecondaryButton,
    );
    await gesture.up();
    await tester.pumpAndSettle();
    expect(backend.position, at);
    expect(backend.entries.single.label, 'Inspect');
  });

  testWidgets('menubar switches the active native session', (tester) async {
    await tester.pumpWidget(
      _configured(
        const Menubar(
          menus: [
            MenubarMenu(
              label: 'File',
              items: [MenuItem(label: 'New')],
            ),
            MenubarMenu(
              label: 'Edit',
              items: [MenuItem(label: 'Copy')],
            ),
          ],
        ),
      ),
    );
    await tester.tap(find.text('File'));
    await tester.pumpAndSettle();
    expect(backend.entries.single.label, 'New');
    backend.pointerPosition = tester.getCenter(find.text('Edit'));
    await tester.pump(const Duration(milliseconds: 60));
    await tester.pumpAndSettle();
    expect(backend.sessions.first.closed, isTrue);
    expect(backend.entries.single.label, 'Copy');
  });

  testWidgets('changing scope closes an open menu, with no late callback', (
    tester,
  ) async {
    var calls = 0;
    final menu = _menu(action: () => calls++);
    await tester.pumpWidget(_configured(menu));
    await tester.tap(find.text('Actions'));
    await tester.pumpAndSettle();
    final lateSelect = backend.select;
    await tester.pumpWidget(_configured(menu, mode: MenuPresentation.custom));
    await tester.pumpAndSettle();
    expect(backend.sessions.single.closed, isTrue);
    lateSelect(0);
    expect(calls, 0);
    await tester.tap(find.text('Actions'));
    await tester.pumpAndSettle();
    expect(find.byType(MenuPanel), findsOneWidget);
  });

  testWidgets('an opening completed after unmount closes immediately', (
    tester,
  ) async {
    backend.pending = Completer<NativeMenuSession?>();
    await tester.pumpWidget(_configured(_menu()));
    await tester.tap(find.text('Actions'));
    await tester.pump();
    await tester.pumpWidget(const SizedBox());
    backend.pending!.complete(backend.sessions.single);
    await tester.pump();
    expect(backend.sessions.single.closed, isTrue);
    expect(tester.takeException(), isNull);
  });

  testWidgets('searchable combobox stays custom under native configuration', (
    tester,
  ) async {
    await tester.pumpWidget(
      _configured(
        SizedBox(
          width: 240,
          child: Combobox<String>(
            options: const [
              ComboboxOption(value: 'a', label: 'Apple'),
              ComboboxOption(value: 'b', label: 'Banana'),
            ],
            onChanged: (_) {},
          ),
        ),
      ),
    );
    await tester.enterText(find.byType(EditableText), 'Ban');
    await tester.pumpAndSettle();
    expect(find.byType(MenuPanel), findsOneWidget);
    expect(find.text('Banana'), findsOneWidget);
    expect(find.text('Apple'), findsNothing);
    expect(backend.sessions, isEmpty);
  });

  testWidgets('unsupported native menus fall back without loading a backend', (
    tester,
  ) async {
    backend.supported = false;
    await tester.pumpWidget(_configured(_menu()));
    await tester.tap(find.text('Actions'));
    await tester.pumpAndSettle();
    expect(find.byType(MenuPanel), findsOneWidget);
    expect(backend.sessions, isEmpty);
  });
}
