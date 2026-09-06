import 'dart:convert';
import 'dart:io';

import 'package:dazzui_example/storybook/story_index.dart';
import 'package:flutter_test/flutter_test.dart';

/// The React story list is the contract's index.
///
/// `stories.json` is what `apps/storybook` renders, written by its parity
/// extractor and distributed here; every id in it is a story this playground
/// is expected to draw under the same name. A story missing here is a gap in
/// the port, and a story here that React does not have is a name the port
/// invented — both are the kind of drift that used to be invisible.
///
/// The two lists beside it are the ratchet: `pending.txt` names the React
/// stories not drawn yet, `extra.txt` the ones drawn here that React has no
/// counterpart for. Each is held both ways, so an entry cannot go stale — a
/// pending story that lands has to leave the list in the same change.
///
/// None of this is committed: the data is what `pnpm parity` writes, and the
/// lists are recorded by the first run that finds them missing. A checkout
/// without the data skips rather than fails.
void main() {
  const dir = 'test/parity';

  Set<String> listed(String file) => File('$dir/$file')
      .readAsLinesSync()
      .map((line) => line.trim())
      .where((line) => line.isNotEmpty && !line.startsWith('#'))
      .toSet();

  /// The list as it stands, or — on a first run — the list as it is now,
  /// written down so the next run is held to it.
  Set<String> ratchet(String file, String note, Set<String> current) {
    final File target = File('$dir/$file');
    if (target.existsSync()) return listed(file);
    target.writeAsStringSync(
      [
        '# $note',
        '# Recorded by the first test run; held both ways from now on, so it',
        '# can only shrink. Shrink it; never grow it to pass.',
        ...current.toList()..sort(),
        '',
      ].join('\n'),
    );
    // ignore: avoid_print
    print('recorded ${current.length} entries into $dir/$file');
    return current;
  }

  test('every story id is unique', () {
    final List<String> ids = StoryIndex.ids;
    expect(ids.toSet().length, ids.length, reason: 'two stories share an id');
  });

  test('every React story is drawn here, or is listed as pending', () {
    final File stories = File('$dir/stories.json');
    if (!stories.existsSync()) {
      markTestSkipped('no $dir/stories.json — run `pnpm parity` at the root');
      return;
    }
    final manifest =
        jsonDecode(stories.readAsStringSync()) as Map<String, Object?>;
    final Set<String> react = (manifest['stories'] as List<Object?>)
        .map((story) => (story as Map<String, Object?>)['id'] as String)
        .toSet();
    final Set<String> drawn = StoryIndex.ids.toSet();
    final Set<String> missing = react.difference(drawn);
    final Set<String> surplus = drawn.difference(react);
    final Set<String> pending = ratchet(
      'pending.txt',
      'React stories this playground does not draw yet, one id per line.',
      missing,
    );
    final Set<String> extra = ratchet(
      'extra.txt',
      'Stories this playground draws that React has no counterpart for.',
      surplus,
    );

    final List<String> problems = [
      for (final id in missing.difference(pending))
        'React draws $id and this playground does not — draw it, or add it to pending.txt',
      for (final id in pending.difference(missing))
        '$id is in pending.txt but is drawn now, or gone from React — remove it',
      for (final id in surplus.difference(extra))
        '$id is drawn here but is not a React story — rename it, or add it to extra.txt',
      for (final id in extra.difference(surplus))
        '$id is in extra.txt but is no longer drawn — remove it',
    ]..sort();

    expect(problems, isEmpty, reason: problems.join('\n'));
  });
}
