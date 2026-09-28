import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Progress` — the bar, determinate and not.
class ProgressStories {
  ProgressStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static StoryPage get page => StoryPage('Progress', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Default',
      (context) => const SizedBox(width: 320, child: Progress(value: 0.35)),
    ),
    Story(
      'Values',
      (context) => const Col(
        width: 320,
        children: [
          Progress(value: 0),
          Progress(value: 0.25),
          Progress(value: 0.5),
          Progress(value: 0.75),
          Progress(value: 1),
        ],
      ),
    ),
    Story(
      'Indeterminate',
      (context) => const SizedBox(width: 320, child: Progress()),
    ),
    Story(
      'Sizes',
      (context) => Col(
        width: 320,
        children: [
          for (final size in sizes) Progress(value: 0.6, size: size),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => Col(
        width: 320,
        children: [
          for (final tint in ProgressTint.values)
            Progress(value: 0.6, tint: tint),
        ],
      ),
    ),
    Story(
      'Gradient',
      (context) => const Col(
        width: 320,
        children: [
          Progress(value: 0.6),
          Progress(value: 0.6, gradient: true),
        ],
      ),
    ),
    Story('Live', (context) => const _Live()),
    Story('With a count', (context) => const _Counted()),
    Story(
      'Beside stages',
      (context) => const SizedBox(
        width: 360,
        child: Card(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              _Counted(),
              StepList(
                children: [
                  Step(
                    status: StepStatus.done,
                    label: 'Resolve the token graph',
                  ),
                  Step(
                    status: StepStatus.active,
                    label: 'Write the platform sources',
                  ),
                  Step(status: StepStatus.idle, label: 'Publish the package'),
                ],
              ),
            ],
          ),
        ),
      ),
    ),
  ]);
}

class _Live extends StatefulWidget {
  const _Live();

  @override
  State<_Live> createState() => _LiveState();
}

class _LiveState extends State<_Live> {
  double _value = 0.15;

  @override
  Widget build(BuildContext context) {
    return Col(
      width: 320,
      children: [
        Progress(value: _value, size: WidgetSize.medium, gradient: true),
        ActionBar(
          children: [
            Button(
              onPressed: () => setState(() {
                _value = _value >= 1 ? 0 : (_value + 0.2).clamp(0, 1);
              }),
              child: const Text('Advance'),
            ),
          ],
        ),
      ],
    );
  }
}

class _Counted extends StatelessWidget {
  const _Counted();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return SizedBox(
      width: 320,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing2,
        children: [
          Row(
            children: [
              Text(
                'Rebuilding tokens',
                style: vars.labelMedium.copyWith(color: vars.colorContent),
              ),
              const Spacer(),
              Text(
                '3 of 4',
                style: vars.captionSmall.copyWith(
                  color: vars.colorContentMuted,
                ),
              ),
            ],
          ),
          const Progress(value: 0.75),
        ],
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Progress',
      description:
          'A bar. With a value it fills; without one it sweeps, which is the '
          'honest drawing for work whose end is not known. The track is a '
          'token rather than the tint washed down, so a bar reads the same on '
          'every surface.',
      children: [
        DocSection(
          'Values',
          description: 'Zero draws the track alone; one fills it.',
          children: [
            Col(
              width: 420,
              children: [
                Progress(value: 0),
                Progress(value: 0.25),
                Progress(value: 0.5),
                Progress(value: 0.75),
                Progress(value: 1),
              ],
            ),
          ],
        ),
        DocSection(
          'Sizes and tints',
          description:
              'Three heights on the same corner. The tint is the fill’s ramp; '
              'the gradient is the two-stop version of it, for the one bar a '
              'view points at.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'small',
                  children: [
                    SizedBox(width: 160, child: Progress(value: 0.6)),
                    SizedBox(
                      width: 160,
                      child: Progress(value: 0.6, tint: ProgressTint.success),
                    ),
                    SizedBox(
                      width: 160,
                      child: Progress(value: 0.6, gradient: true),
                    ),
                  ],
                ),
                Specimen(
                  'medium',
                  children: [
                    SizedBox(
                      width: 160,
                      child: Progress(value: 0.6, size: WidgetSize.medium),
                    ),
                    SizedBox(
                      width: 160,
                      child: Progress(
                        value: 0.6,
                        size: WidgetSize.medium,
                        gradient: true,
                      ),
                    ),
                  ],
                ),
                Specimen(
                  'large',
                  children: [
                    SizedBox(
                      width: 160,
                      child: Progress(value: 0.6, size: WidgetSize.large),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Indeterminate',
          description:
              'No value at all is the sweep. It is the same bar: nothing about '
              'the box changes, only what is drawn inside it.',
          children: [
            SizedBox(width: 420, child: Progress(size: WidgetSize.medium)),
          ],
        ),
      ],
    );
  }
}
