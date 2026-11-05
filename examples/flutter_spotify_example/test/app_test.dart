import 'package:dazzui/dazzui.dart';
import 'package:flutter_spotify_example/app.dart';
import 'package:flutter_spotify_example/player_controller.dart';
import 'package:flutter_spotify_example/widgets/collection_view.dart';
import 'package:flutter_spotify_example/widgets/common.dart';
import 'package:flutter_test/flutter_test.dart';

import 'fake_transport.dart';

Finder action(String label) => find.byWidgetPredicate(
  (widget) => widget is PlayerIcon && widget.label == label,
);

void main() {
  for (final size in [
    const Size(1440, 940),
    const Size(1000, 740),
    const Size(800, 600),
    const Size(390, 760),
  ]) {
    testWidgets('both appearances fit $size and retain playback state', (
      tester,
    ) async {
      tester.view.devicePixelRatio = 1;
      tester.view.physicalSize = size;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final player = PlayerController(transport: FakeTransport());
      await tester.pumpWidget(SpotifyApp(player: player));
      await tester.pumpAndSettle();
      expect(tester.takeException(), isNull);
      expect(find.byType(TrackRow), findsNWidgets(6));
      await tester.tap(find.byKey(const Key('playback-toggle')));
      await tester.pumpAndSettle();
      expect(player.playing, isTrue);
      await tester.tap(find.byKey(const Key('appearance-toggle')));
      await tester.pumpAndSettle();
      expect(tester.takeException(), isNull);
      expect(player.playing, isTrue);
      expect(
        Theme.of(tester.element(find.byType(CollectionView))).brightness,
        Brightness.light,
      );
      await tester.tap(action('Next track'));
      await tester.pumpAndSettle();
      expect(player.current.title, 'Weightless');
      if (size.width < 1100) {
        await tester.tap(action('Toggle now playing'));
        await tester.pumpAndSettle();
        expect(find.text('Next in queue'), findsOneWidget);
        expect(tester.takeException(), isNull);
        await tester.tap(action('Close now playing'));
        await tester.pumpAndSettle();
        expect(find.text('Next in queue'), findsNothing);
      }
      await tester.pumpWidget(const SizedBox());
      player.dispose();
    });
  }

  testWidgets(
    'a progress drag commits once and playlist playback replaces the queue',
    (tester) async {
      tester.view.devicePixelRatio = 1;
      tester.view.physicalSize = const Size(1440, 940);
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final audio = FakeTransport();
      final player = PlayerController(transport: audio);
      await tester.pumpWidget(SpotifyApp(player: player));
      await tester.tap(find.byKey(const Key('playback-toggle')));
      await tester.pumpAndSettle();
      final rect = tester.getRect(find.byKey(const Key('playback-progress')));
      final gesture = await tester.startGesture(
        Offset(rect.left + rect.width * 0.2, rect.center.dy),
      );
      for (var step = 3; step < 9; step++) {
        await gesture.moveTo(
          Offset(rect.left + rect.width * step / 10, rect.center.dy),
        );
        await tester.pump();
      }
      expect(audio.seekCount, 0);
      await gesture.up();
      await tester.pumpAndSettle();
      expect(audio.seekCount, 1);
      expect(player.position.inSeconds, greaterThan(15));

      await tester.tap(
        find.byWidgetPredicate(
          (widget) =>
              widget is Pressable && widget.semanticsLabel == 'After Hours',
        ),
      );
      await tester.pumpAndSettle();
      await tester.tap(action('Play playlist'));
      await tester.pumpAndSettle();
      expect(player.current.title, 'Night Swim');
      expect(player.queue.map((track) => track.title), [
        'Night Swim',
        'Slow Motion',
        'Weightless',
        'Somewhere Else',
      ]);
      expect(tester.takeException(), isNull);
      await tester.pumpWidget(const SizedBox());
      player.dispose();
    },
  );

  testWidgets('search filters real rows and likes update the library', (
    tester,
  ) async {
    tester.view.devicePixelRatio = 1;
    tester.view.physicalSize = const Size(1440, 940);
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final player = PlayerController(transport: FakeTransport());
    await tester.pumpWidget(SpotifyApp(player: player));
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(EditableText), 'koi');
    await tester.pumpAndSettle();
    expect(find.byType(TrackRow), findsOneWidget);
    expect(find.text('Search results'), findsOneWidget);
    await tester.tap(action('Like Night Swim'));
    await tester.pumpAndSettle();
    expect(player.liked, contains('night_swim'));
    await tester.tap(
      find.byWidgetPredicate(
        (widget) =>
            widget is Pressable && widget.semanticsLabel == 'Liked Songs',
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byType(TrackRow), findsNWidgets(3));
    expect(tester.takeException(), isNull);
    await tester.pumpWidget(const SizedBox());
    player.dispose();
  });
}
