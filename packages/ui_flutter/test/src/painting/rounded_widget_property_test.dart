import 'package:dazzui/src/foundation/widget_radius.dart';
import 'package:dazzui/src/painting/widget_property.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('RoundedWidgetProperty', () {
    test(
      'should return the correct BorderRadius for the given WidgetRadius',
      () async {
        const property = RoundedWidgetProperty(
          none: BorderRadius.zero,
          tiny: BorderRadius.all(Radius.circular(4)),
          small: BorderRadius.all(Radius.circular(8)),
          medium: BorderRadius.all(Radius.circular(16)),
          large: BorderRadius.all(Radius.circular(24)),
          big: BorderRadius.all(Radius.circular(32)),
          full: BorderRadius.all(Radius.circular(9999)),
        );
        final noneValue = property.rounded(WidgetRadius.none);
        expect(noneValue, BorderRadius.zero);
        final tinyValue = property.rounded(WidgetRadius.tiny);
        expect(tinyValue, const BorderRadius.all(Radius.circular(4)));
        final smallValue = property.rounded(WidgetRadius.small);
        expect(smallValue, const BorderRadius.all(Radius.circular(8)));
        final mediumValue = property.rounded(WidgetRadius.medium);
        expect(mediumValue, const BorderRadius.all(Radius.circular(16)));
        final largeValue = property.rounded(WidgetRadius.large);
        expect(largeValue, const BorderRadius.all(Radius.circular(24)));
        final bigValue = property.rounded(WidgetRadius.big);
        expect(bigValue, const BorderRadius.all(Radius.circular(32)));
        final fullValue = property.rounded(WidgetRadius.full);
        expect(fullValue, const BorderRadius.all(Radius.circular(9999)));
      },
    );
  });
}
