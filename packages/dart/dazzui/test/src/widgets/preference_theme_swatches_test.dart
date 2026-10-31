import 'dart:ui' as ui;

import 'package:dazzui/dazzui.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

const options = [
  PreferenceThemeOption(value: 'paper', label: 'Studio', theme: ''),
  PreferenceThemeOption(
    value: 'night',
    label: 'Nocturne',
    theme: 'nocturne-dark',
  ),
];

void main() {
  testWidgets('independent palettes survive page changes and selection', (
    tester,
  ) async {
    final boundaryKey = GlobalKey();
    String? changed;
    Widget control(ThemeData theme) => host(
      Center(
        child: RepaintBoundary(
          key: boundaryKey,
          child: PreferenceThemeSwatches(
            options: options,
            defaultValue: 'paper',
            onValueChange: (value) => changed = value,
          ),
        ),
      ),
      theme: theme,
    );
    Future<List<int>> pixel(int x, int y) async =>
        (await tester.runAsync(() async {
          final boundary =
              boundaryKey.currentContext!.findRenderObject()!
                  as RenderRepaintBoundary;
          final image = await boundary.toImage();
          final bytes = (await image.toByteData(
            format: ui.ImageByteFormat.rawRgba,
          ))!;
          final offset = (y * image.width + x) * 4;
          final result = bytes.buffer.asUint8List().sublist(offset, offset + 3);
          image.dispose();
          return result;
        }))!;

    List<int> rgb(Color color) => [
      (color.r * 255).round(),
      (color.g * 255).round(),
      (color.b * 255).round(),
    ];
    await tester.pumpWidget(control(ThemeData.studioDark()));
    await tester.pump();
    // Inside each window's column, clear of the border and rounded corner.
    expect(await pixel(4, 12), rgb(themeVariables.colorSurfaceColumn));
    expect(
      await pixel(46, 12),
      rgb(themeVariablesNocturneDark.colorSurfaceColumn),
    );
    final before = tester.getSize(find.byType(PreferenceThemeSwatches<String>));
    await tester.tap(find.byKey(const ValueKey('night')));
    await tester.pump();
    expect(changed, 'night');
    expect(
      tester.getSize(find.byType(PreferenceThemeSwatches<String>)),
      before,
    );
    expect(
      tester
          .widget<Pressable>(
            find.descendant(
              of: find.byKey(const ValueKey('night')),
              matching: find.byType(Pressable),
            ),
          )
          .checked,
      isTrue,
    );
    await tester.pumpWidget(control(ThemeData.brightLight()));
    await tester.pump();
    expect(await pixel(4, 12), rgb(themeVariables.colorSurfaceColumn));
    expect(
      await pixel(46, 12),
      rgb(themeVariablesNocturneDark.colorSurfaceColumn),
    );
  });

  testWidgets('controlled refusal, external updates and keyboard activation', (
    tester,
  ) async {
    String? changed;
    Widget control(String value) => host(
      Center(
        child: PreferenceThemeSwatches(
          options: options,
          value: value,
          onValueChange: (next) => changed = next,
        ),
      ),
    );
    await tester.pumpWidget(control('paper'));
    await tester.tap(find.byKey(const ValueKey('night')));
    await tester.pump();
    expect(changed, 'night');
    Pressable swatch(String value) => tester.widget<Pressable>(
      find.descendant(
        of: find.byKey(ValueKey(value)),
        matching: find.byType(Pressable),
      ),
    );
    expect(swatch('paper').checked, isTrue);
    expect(swatch('night').checked, isFalse);
    await tester.pumpWidget(control('night'));
    expect(swatch('night').checked, isTrue);
    final semantics = tester.ensureSemantics();
    expect(
      tester.getSemantics(find.byType(Pressable).last),
      matchesSemantics(
        label: 'Nocturne',
        hasCheckedState: true,
        isChecked: true,
        hasEnabledState: true,
        isEnabled: true,
        isFocusable: true,
        isInMutuallyExclusiveGroup: true,
        hasTapAction: true,
        hasFocusAction: true,
      ),
    );
    semantics.dispose();
    await tester.sendKeyEvent(LogicalKeyboardKey.tab);
    await tester.sendKeyEvent(LogicalKeyboardKey.space);
    await tester.pump();
    expect(changed, 'paper');
  });

  testWidgets('a narrow preference row wraps swatches and retains its label', (
    tester,
  ) async {
    await tester.pumpWidget(
      host(
        Center(
          child: SizedBox(
            width: 150,
            child: PreferenceRow(
              title: 'Theme',
              trailing: PreferenceThemeSwatches(
                options: [
                  for (int i = 0; i < 6; i++)
                    PreferenceThemeOption(
                      value: '$i',
                      label: 'Theme $i',
                      theme: 'studio-light',
                    ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
    expect(tester.takeException(), isNull);
    final swatches = find.byType(PreferenceThemeSwatches<String>);
    expect(
      tester.getSize(swatches).height,
      greaterThan(themeVariables.controlSmallSize),
    );
    expect(
      tester.getRect(find.text('Theme')).right,
      lessThan(tester.getRect(swatches).left),
    );
  });
}
