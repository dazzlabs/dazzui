import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

void main() {
  final themes = <String, ThemeVariables>{
    'bright': themeVariables,
    'studio': themeVariablesStudioLight,
    'windows': themeVariablesWindows11Light,
    'windows dark': themeVariablesWindows11Dark,
    'ubuntu': themeVariablesUbuntuLight,
    'macos': themeVariablesMacos15Dark,
  };
  for (final entry in themes.entries) {
    testWidgets('icons follow ${entry.key} and explicit sizes win', (
      tester,
    ) async {
      final vars = entry.value;
      final wide = entry.key.startsWith('windows') || entry.key == 'ubuntu';
      expect([
        vars.iconSmall,
        vars.iconMedium,
        vars.iconLarge,
      ], wide ? [14, 20, 24] : [12, 16, 20]);
      const glyph = FluentIcons.add_20_regular;
      await tester.pumpWidget(
        host(
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Icon(glyph, key: ValueKey('standalone')),
              for (final size in [
                WidgetSize.tiny,
                WidgetSize.small,
                WidgetSize.medium,
                WidgetSize.large,
              ])
                IconButton(
                  size: size,
                  onPressed: () {},
                  icon: Icon(glyph, key: ValueKey(size.namedSize)),
                ),
              IconButton(
                onPressed: () {},
                iconSize: 9,
                icon: const Icon(glyph, key: ValueKey('slot override')),
              ),
              Button(
                onPressed: () {},
                size: WidgetSize.large,
                child: const Icon(glyph, key: ValueKey('button')),
              ),
              Button(
                onPressed: () {},
                size: WidgetSize.large,
                child: const Icon(glyph, size: 10, key: ValueKey('explicit')),
              ),
            ],
          ),
          theme: ThemeData(vars: vars, brightness: Brightness.light),
        ),
      );
      expect(
        tester.getSize(find.byKey(const ValueKey('standalone'))),
        Size.square(vars.iconMedium),
      );
      for (final size in [
        WidgetSize.tiny,
        WidgetSize.small,
        WidgetSize.medium,
        WidgetSize.large,
      ]) {
        // The tiny square takes its own glyph, `icon-button.tiny.icon-size`;
        // every other step keeps the profile's.
        expect(
          tester.getSize(find.byKey(ValueKey(size.namedSize))),
          Size.square(
            size == WidgetSize.tiny
                ? vars.iconButtonTinyIconSize
                : vars.controlIconSize(size),
          ),
        );
      }
      // A glyph in a button is the profile's icon less the label-glyph
      // shrink, as React's `.dz-button` sizes it.
      expect(
        tester.getSize(find.byKey(const ValueKey('button'))),
        Size.square(
          vars.controlIconSize(WidgetSize.large) - vars.controlLabelGlyphShrink,
        ),
      );
      expect(
        tester.getSize(find.byKey(const ValueKey('slot override'))),
        const Size.square(9),
      );
      expect(
        tester.getSize(find.byKey(const ValueKey('explicit'))),
        const Size.square(10),
      );
      expect(tester.takeException(), isNull);
    });
  }
}
