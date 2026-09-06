import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/StepList` — a job, reported.
class StepListStories {
  StepListStories._();

  static StoryPage get page => StoryPage('StepList', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'In flight',
      (context) => const SizedBox(
        width: 320,
        child: StepList(
          children: [
            Step(status: StepStatus.done, label: 'Resolve the token graph'),
            Step(status: StepStatus.done, label: 'Write the platform sources'),
            Step(status: StepStatus.active, label: 'Compile dazzui'),
            Step(status: StepStatus.idle, label: 'Publish the package'),
          ],
        ),
      ),
    ),
    Story(
      'Statuses',
      (context) => SizedBox(
        width: 320,
        child: StepList(
          children: [
            for (final status in StepStatus.values)
              Step(status: status, label: label(status.name)),
          ],
        ),
      ),
    ),
    Story(
      'Finished',
      (context) => const SizedBox(
        width: 320,
        child: StepList(
          children: [
            Step(status: StepStatus.done, label: 'Resolve the token graph'),
            Step(status: StepStatus.done, label: 'Write the platform sources'),
            Step(status: StepStatus.done, label: 'Compile dazzui'),
          ],
        ),
      ),
    ),
    Story('Timed detail', (context) => const _Timed()),
    Story('Advancing', (context) => const _Advancing()),
    Story(
      'In a card',
      (context) => const SizedBox(
        width: 360,
        child: Card(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [Text('Rebuilding tokens'), _Timed()],
          ),
        ),
      ),
    ),
  ]);
}

class _Timed extends StatelessWidget {
  const _Timed();

  @override
  Widget build(BuildContext context) {
    return const StepList(
      children: [
        Step(
          status: StepStatus.done,
          label: 'Resolve the token graph',
          meta: '0.4s',
        ),
        Step(
          status: StepStatus.done,
          label: 'Write the platform sources',
          meta: '1.2s',
        ),
        Step(
          status: StepStatus.active,
          label: 'Compile dazzui',
          meta: 'running',
        ),
        Step(status: StepStatus.idle, label: 'Publish the package', meta: '—'),
      ],
    );
  }
}

class _Advancing extends StatefulWidget {
  const _Advancing();

  @override
  State<_Advancing> createState() => _AdvancingState();
}

class _AdvancingState extends State<_Advancing> {
  int _reached = 1;

  static const List<String> _steps = [
    'Resolve the token graph',
    'Write the platform sources',
    'Compile dazzui',
    'Publish the package',
  ];

  StepStatus _status(int index) {
    if (index < _reached) return StepStatus.done;
    return index == _reached ? StepStatus.active : StepStatus.idle;
  }

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 360,
      children: [
        StepList(
          children: [
            for (final (index, step) in _steps.indexed)
              Step(status: _status(index), label: step),
          ],
        ),
        ActionBar(
          children: [
            Button(
              onPressed: () => setState(() {
                _reached = _reached >= _steps.length ? 0 : _reached + 1;
              }),
              child: const Text('Advance'),
            ),
          ],
        ),
      ],
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return DocPage(
      'StepList',
      description:
          'What a long job says while it runs: a line per stage, a mark saying '
          'where it got to, and an optional trailing detail. It reports rather '
          'than navigates — nothing in it is clickable.',
      children: [
        DocSection(
          'Statuses',
          description:
              '`done` is the tick; `active` is the stage in flight and the '
              'only one that carries the tint; `idle` is the stage not reached '
              'yet, in faint ink. Exactly one step should be active.',
          children: [
            SizedBox(
              width: 360,
              child: StepList(
                children: [
                  for (final status in StepStatus.values)
                    Step(status: status, label: label(status.name)),
                ],
              ),
            ),
          ],
        ),
        const DocSection(
          'Detail',
          description:
              'The trailing slot is for the number the stage produced — a '
              'duration, a count, a hyphen where there is nothing yet. It is '
              'not a second label: a step whose detail explains what it does '
              'is a step whose label is wrong.',
          children: [
            SizedBox(width: 400, child: _Timed()),
          ],
        ),
        const DocSection(
          'Advancing',
          description: 'The list as a job actually moves through it.',
          children: [_Advancing()],
        ),
      ],
    );
  }
}
