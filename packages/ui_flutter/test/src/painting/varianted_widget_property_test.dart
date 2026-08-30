import 'package:dazzui/src/foundation/widget_variant.dart';
import 'package:dazzui/src/generated/colors.dart';
import 'package:dazzui/src/painting/widget_property.dart';
import 'package:flutter_test/flutter_test.dart';

enum MockWidgetVariant with WidgetVariant {
  filled,
  tinted,
  outlined,
  muted,
  cleared,
}

void main() {
  group('VariantedWidgetProperty', () {
    test(
      'should return the correct color value for the given WidgetVariant',
      () async {
        const property = VariantedWidgetProperty(
          normal: Colors.black,
          filled: Colors.brand,
          tinted: Colors.neutral,
          outlined: Colors.green,
          plain: Colors.amber,
        );
        final filledValue = property.varianted(
          MockWidgetVariant.filled,
          {},
          null,
        );
        expect(filledValue, Colors.brand);
        final tintedValue = property.varianted(
          MockWidgetVariant.tinted,
          {},
          null,
        );
        expect(tintedValue, Colors.neutral);
        final outlinedValue = property.varianted(
          MockWidgetVariant.outlined,
          {},
          null,
        );
        expect(outlinedValue, Colors.green);
        final mutedValue = property.varianted(
          MockWidgetVariant.muted,
          {},
          null,
        );
        expect(mutedValue, Colors.red);
        final clearedValue = property.varianted(
          MockWidgetVariant.cleared,
          {},
          null,
        );
        expect(clearedValue, Colors.amber);
      },
    );
  });
}
