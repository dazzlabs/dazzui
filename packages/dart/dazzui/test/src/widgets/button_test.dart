import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

void main() {
  testWidgets('Button', (WidgetTester tester) async {
    await tester.pumpWidget(
      host(
        Button(
          onPressed: () {},
          child: const Text('Button'),
        ),
      ),
    );
    expect(find.text('Button'), findsOneWidget);
  });

  // The end a glyph sits at pads `control.label-glyph.inset` less, and the
  // glyph is the profile's icon less `control.label-glyph.shrink`.
  for (final size in [
    WidgetSize.tiny,
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ]) {
    testWidgets('a ${size.namedSize.name} glyph trims its own end', (
      tester,
    ) async {
      const vars = themeVariables;
      // The box reserves the stroke outside the profile's padding.
      final double pad =
          vars.strokeHairline +
          switch (size.namedSize) {
            NamedSize.tiny => vars.controlTinyPaddingInline,
            NamedSize.small => vars.controlSmallPaddingInline,
            NamedSize.medium => vars.controlMediumPaddingInline,
            NamedSize.large => vars.controlLargePaddingInline,
          };
      Future<EdgeInsets> padding(Widget button) async {
        await tester.pumpWidget(host(button));
        return tester
            .widget<AnimatedContainer>(find.byType(AnimatedContainer))
            .padding!
            .resolve(TextDirection.ltr);
      }

      const glyph = Icon(FluentIcons.add_20_regular, key: ValueKey('glyph'));
      final leading = await padding(
        Button(
          size: size,
          onPressed: () {},
          leading: glyph,
          child: const Text('New'),
        ),
      );
      expect(
        tester.getSize(find.byKey(const ValueKey('glyph'))),
        Size.square(vars.controlIconSize(size) - vars.controlLabelGlyphShrink),
      );
      expect(leading.left, pad - vars.controlLabelGlyphInset);
      expect(leading.right, pad);

      final trailing = await padding(
        Button(
          size: size,
          onPressed: () {},
          trailing: glyph,
          child: const Text('Next'),
        ),
      );
      expect(trailing.left, pad);
      expect(trailing.right, pad - vars.controlLabelGlyphInset);

      final both = await padding(
        Button(
          size: size,
          onPressed: () {},
          leading: glyph,
          trailing: const Icon(FluentIcons.chevron_down_20_regular),
          child: const Text('Both'),
        ),
      );
      expect(both.left, pad - vars.controlLabelGlyphInset);
      expect(both.right, pad - vars.controlLabelGlyphInset);
    });
  }
}
