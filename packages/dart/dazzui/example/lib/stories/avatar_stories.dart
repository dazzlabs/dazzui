import 'dart:typed_data';

import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Avatar` — the picture that stands for a person or a project.
class AvatarStories {
  AvatarStories._();

  static const List<WidgetSize> sizes = [
    WidgetSize.small,
    WidgetSize.medium,
    WidgetSize.large,
  ];

  static const List<(String, AvatarTint)> people = [
    ('Ada Lovelace', AvatarTint.primary),
    ('Grace Hopper', AvatarTint.info),
    ('Alan Turing', AvatarTint.success),
    ('Katherine Johnson', AvatarTint.warning),
    ('Barbara Liskov', AvatarTint.danger),
  ];

  static List<Widget> get run => [
    for (final (String name, AvatarTint tint) in people)
      Avatar(name: name, tint: tint),
  ];

  static StoryPage get page => StoryPage('Avatar', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'Initials',
      (context) => RowWrap(
        children: [
          for (final (String name, AvatarTint tint) in people.take(4))
            Avatar(name: name, tint: tint),
        ],
      ),
    ),
    Story(
      'Image',
      (context) => RowWrap(
        children: [
          for (final size in sizes)
            Avatar(name: 'Ada Lovelace', size: size, image: _portrait),
        ],
      ),
    ),
    Story(
      'Square',
      (context) => const RowWrap(
        children: [
          Avatar(name: 'DazzUI', shape: AvatarShape.square),
          Avatar(
            name: 'Acme Web',
            shape: AvatarShape.square,
            tint: AvatarTint.info,
          ),
          Avatar(
            name: 'Design Tokens',
            shape: AvatarShape.square,
            tint: AvatarTint.neutral,
          ),
        ],
      ),
    ),
    Story(
      'Sizes',
      (context) => Col(
        children: [
          for (final size in sizes)
            RowWrap(
              children: [
                Avatar(name: 'Ada Lovelace', size: size),
                Avatar(
                  name: 'DazzUI',
                  size: size,
                  shape: AvatarShape.square,
                ),
              ],
            ),
        ],
      ),
    ),
    Story(
      'Tints',
      (context) => RowWrap(
        children: [
          for (final tint in AvatarTint.values)
            Avatar(name: tint.name, tint: tint),
        ],
      ),
    ),
    Story(
      'Groups',
      (context) => Col(
        children: [
          for (final size in sizes)
            AvatarGroup(
              size: size,
              max: 3,
              children: [
                for (final (String name, AvatarTint tint) in people)
                  Avatar(name: name, tint: tint, size: size),
              ],
            ),
        ],
      ),
    ),
  ]);
}

/// An inline picture rather than a URL: the playground has to draw the same
/// thing on a machine with no network, and what the story is showing is the
/// image path rather than the image.
final MemoryImage _portrait = MemoryImage(
  Uint8List.fromList(<int>[
    // A 1×1 violet PNG. The avatar clips and covers it, so one pixel is
    // enough to prove the image path is taken.
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, //
    0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0, 144, 119, 83, 222, //
    0, 0, 0, 15, 73, 68, 65, 84, 8, 153, 99, 96, 96, 248, 207, 192, //
    0, 0, 3, 1, 1, 0, 24, 221, 141, 219, 0, 0, 0, 0, 73, 69, //
    78, 68, 174, 66, 96, 130,
  ]),
);

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Avatar',
      description:
          'The picture that stands for a person or a project. What it is '
          'really for is the case where there is no picture: the initials are '
          'drawn until the image has actually decoded, so a slow or broken URL '
          'never flashes an empty box. It is not interactive and stays off the '
          'shared state machine — an avatar that lit up under the pointer '
          'would read as something you can press.',
      children: [
        DocSection(
          'Sizes',
          description:
              'Three steps off the 4px grid rather than off the control '
              'profile — an avatar is not a control, and these are the sizes a '
              'row, a header and a card actually ask for. The type moves one '
              'step of the scale per size rather than tracking the box, which '
              'grows nearly twice as fast.',
          children: [
            Specimens(
              children: [
                for (final size in AvatarStories.sizes)
                  Specimen(
                    size.namedSize.name,
                    children: [
                      Avatar(name: 'Ada Lovelace', size: size),
                      Avatar(
                        name: 'DazzUI',
                        size: size,
                        shape: AvatarShape.square,
                      ),
                    ],
                  ),
              ],
            ),
          ],
        ),
        DocSection(
          'Circle and square',
          description:
              'A person is a circle and a thing is a square — a project, a '
              'repository. The square takes the container corner rather than '
              'the control corner: under Bright, where controls are pills, a '
              'control-cornered square would go round and stop being the shape '
              'that says “project”.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'circle',
                  children: AvatarStories.run.take(3).toList(),
                ),
                const Specimen(
                  'square',
                  children: [
                    Avatar(name: 'DazzUI', shape: AvatarShape.square),
                    Avatar(
                      name: 'Acme Web',
                      shape: AvatarShape.square,
                      tint: AvatarTint.info,
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        DocSection(
          'Groups',
          description:
              'Overlapped by a third of the box and cut out of each other by a '
              'ring in the ground colour. The ring is drawn as a spread rather '
              'than a border, so it takes no room — a border would grow every '
              'avatar in the run by three pixels.',
          children: [
            Specimens(
              children: [
                Specimen(
                  'on paper',
                  children: [
                    AvatarGroup(max: 3, children: AvatarStories.run),
                  ],
                ),
                Specimen(
                  'on a card',
                  children: [
                    Card(
                      child: Padding(
                        padding: EdgeInsets.all(vars.spacing4),
                        child: AvatarGroup(
                          max: 3,
                          ring: vars.colorSurfaceMuted,
                          children: AvatarStories.run,
                        ),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
