import 'dart:io';
import 'dart:ui' as ui;

import 'package:dazzui/dazzui.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_spotify_example/app.dart';
import 'package:flutter_spotify_example/catalog.dart';
import 'package:flutter_spotify_example/player_controller.dart';
import 'package:flutter_spotify_example/widgets/common.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('native audio, seeking, queue, volume and both appearances', (
    tester,
  ) async {
    final player = PlayerController(transport: LocalAudioTransport());
    final boundary = GlobalKey();
    await tester.binding.setSurfaceSize(const Size(1440, 918));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    await tester.pumpWidget(
      RepaintBoundary(
        key: boundary,
        child: SpotifyApp(player: player),
      ),
    );
    await tester.pumpAndSettle();
    await tester.runAsync(() async {
      await player.play(tracks.first);
      await Future<void>.delayed(const Duration(milliseconds: 1200));
    });
    await tester.pump();
    expect(player.error, isNull);
    expect(player.playing, isTrue);
    expect(player.position.inMilliseconds, greaterThan(200));

    await tester.runAsync(() => player.toggle());
    await tester.pumpAndSettle();
    expect(player.playing, isFalse);
    final paused = player.position;
    await tester.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 500)),
    );
    expect((player.position - paused).inMilliseconds.abs(), lessThan(400));

    await tester.runAsync(() => player.seek(const Duration(seconds: 10)));
    expect((player.position.inMilliseconds - 10000).abs(), lessThan(600));
    await tester.runAsync(() async {
      await player.setVolume(0.25);
      await player.toggleMute();
      expect(player.volume, 0);
      await player.toggleMute();
      expect(player.volume, 0.25);
      await player.next();
    });
    await tester.pumpAndSettle();
    expect(player.current, tracks[1]);
    expect(player.playing, isTrue);

    Future<void> snapshot(String appearance) async {
      await tester.pumpAndSettle();
      final render =
          boundary.currentContext!.findRenderObject()! as RenderRepaintBoundary;
      final image = await render.toImage(pixelRatio: 1.5);
      final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
      final directory =
          Platform.environment['SPOTIFY_SCREENSHOT_DIR'] ??
          Directory.systemTemp.path;
      final file = File('$directory/dazzui-spotify-$appearance.png');
      await file.parent.create(recursive: true);
      await file.writeAsBytes(bytes!.buffer.asUint8List());
      debugPrint('Screenshot: ${file.path}');
      image.dispose();
    }

    // Hold a real native frame still for review.
    await tester.runAsync(() => player.toggle());
    await tester.runAsync(() => player.seek(const Duration(seconds: 10)));
    await tester.pumpAndSettle();
    await snapshot('dark');
    await tester.tap(find.byKey(const Key('appearance-toggle')));
    await tester.pumpAndSettle();
    await snapshot('light');
    expect(tester.takeException(), isNull);

    // The next track is picked on a real native completion event.
    await tester.runAsync(() async {
      await player.toggle();
      await player.seek(const Duration(milliseconds: 23200));
      await Future<void>.delayed(const Duration(seconds: 2));
    });
    await tester.pumpAndSettle();
    expect(player.current, tracks[2]);
    expect(player.playing, isTrue);
    await tester.tap(
      find.byWidgetPredicate(
        (widget) => widget is PlayerIcon && widget.label == 'Show queue',
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('Next in queue'), findsOneWidget);
    expect(tester.takeException(), isNull);

    await tester.runAsync(() => player.toggle());
    await tester.pumpWidget(const SizedBox());
    player.dispose();
  });
}
