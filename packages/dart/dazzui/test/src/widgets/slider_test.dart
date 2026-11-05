import 'package:dazzui/dazzui.dart';
import 'package:flutter_test/flutter_test.dart';

import '../../host.dart';

void main() {
  for (final vars in [themeVariablesSpotifyLight, themeVariablesSpotifyDark]) {
    for (final values in [
      <double>[25],
      <double>[25, 75],
    ]) {
      testWidgets(
        'the filled span is visible for $values on ${vars.colorSurface}',
        (tester) async {
          await tester.pumpWidget(
            host(
              Center(
                child: SizedBox(
                  width: 400,
                  child: Slider(values: values, onChanged: (_) {}),
                ),
              ),
              theme: ThemeData(
                vars: vars,
                brightness: vars == themeVariablesSpotifyDark
                    ? Brightness.dark
                    : Brightness.light,
              ),
            ),
          );
          final fill = find.byWidgetPredicate(
            (widget) =>
                widget is ColoredBox && widget.color == vars.colorPrimary[600],
          );
          final span = values.length == 1
              ? values.single
              : values.last - values.first;
          // A correctly sized outer track can still hide a zero-height fill.
          // Check the painted run itself, for a scalar and a range.
          expect(
            tester.getSize(fill),
            Size(400 * span / 100, vars.sliderMediumTrack),
          );
          expect(tester.takeException(), isNull);
        },
      );
    }
  }
}
