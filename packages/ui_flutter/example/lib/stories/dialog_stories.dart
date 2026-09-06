import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/stage.dart';
import '../storybook/story.dart';

/// `Components/Dialog` — the panel, and the scrim it is centred in.
class DialogStories {
  DialogStories._();

  static StoryPage get page => StoryPage('Dialog', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story(
      'A sheet',
      (context) => DialogScrim(
        child: Dialog(
          children: [
            const DialogHeader(
              title: 'Rebuild the token graph?',
              subtitle:
                  'Four platform packages will be rewritten from packages/ds.',
            ),
            const DialogBody(
              children: [
                Text(
                  'The generated files are committed, so the diff is the '
                  'review.',
                ),
              ],
            ),
            DialogFooter(
              children: [
                const Spacer(),
                Button(onPressed: () {}, child: const Text('Cancel')),
                Button(
                  variant: ButtonVariant.filled,
                  onPressed: () {},
                  child: const Text('Rebuild'),
                ),
              ],
            ),
          ],
        ),
      ),
      layout: StoryLayout.fullscreen,
    ),
    Story('A confirmation', (context) => const _Confirmation()),
    Story('Danger', (context) => const _Danger()),
    Story('A form', (context) => const _FormDialog()),
    Story(
      'Body only',
      (context) => const Dialog(
        children: [
          DialogBody(
            children: [
              Text('A sheet with nothing to confirm — only something to read.'),
            ],
          ),
        ],
      ),
    ),
    Story(
      'Without a subtitle',
      (context) => Dialog(
        children: [
          const DialogHeader(title: 'Rename this project'),
          DialogFooter(
            children: [
              const Spacer(),
              Button(onPressed: () {}, child: const Text('Cancel')),
              Button(
                variant: ButtonVariant.filled,
                onPressed: () {},
                child: const Text('Rename'),
              ),
            ],
          ),
        ],
      ),
    ),
    Story(
      'Behind a trigger',
      (context) => const _Triggered(),
      layout: StoryLayout.fullscreen,
    ),
  ]);
}

class _Confirmation extends StatelessWidget {
  const _Confirmation();

  @override
  Widget build(BuildContext context) {
    return Dialog(
      children: [
        const DialogHeader(title: 'Discard these changes?'),
        const DialogBody(
          children: [
            Text(
              'The three files you edited go back to their last saved state.',
            ),
          ],
        ),
        DialogFooter(
          children: [
            const Spacer(),
            Button(onPressed: () {}, child: const Text('Keep editing')),
            Button(
              variant: ButtonVariant.filled,
              onPressed: () {},
              child: const Text('Discard'),
            ),
          ],
        ),
      ],
    );
  }
}

class _Danger extends StatelessWidget {
  const _Danger();

  @override
  Widget build(BuildContext context) {
    return Dialog(
      tone: DialogTone.danger,
      children: [
        const DialogHeader(
          title: 'Delete the glossary?',
          subtitle: '1,204 entries. This cannot be undone.',
        ),
        DialogFooter(
          children: [
            const Spacer(),
            Button(onPressed: () {}, child: const Text('Keep')),
            Button(
              variant: ButtonVariant.filled,
              tint: ButtonTint.danger,
              onPressed: () {},
              child: const Text('Delete'),
            ),
          ],
        ),
      ],
    );
  }
}

class _FormDialog extends StatefulWidget {
  const _FormDialog();

  @override
  State<_FormDialog> createState() => _FormDialogState();
}

class _FormDialogState extends State<_FormDialog> {
  final TextEditingController _name = TextEditingController(text: 'DazzUI');

  @override
  void dispose() {
    _name.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      children: [
        const DialogHeader(
          title: 'New workspace',
          subtitle: 'It can be renamed later.',
        ),
        DialogBody(
          children: [
            FormField(
              label: 'Name',
              child: TextField(controller: _name, placeholder: 'Acme'),
            ),
          ],
        ),
        DialogFooter(
          children: [
            const Spacer(),
            Button(onPressed: () {}, child: const Text('Cancel')),
            Button(
              variant: ButtonVariant.filled,
              onPressed: () {},
              child: const Text('Create'),
            ),
          ],
        ),
      ],
    );
  }
}

/// What a dialog looks like from the app's side: a control opens it as a
/// route, and the scrim covers the surface it was opened from.
class _Triggered extends StatelessWidget {
  const _Triggered();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Button(
        variant: ButtonVariant.filled,
        onPressed: () => showDialog<void>(
          context: context,
          builder: (context) => Dialog(
            children: [
              const DialogHeader(
                title: 'Discard these changes?',
                subtitle:
                    'The three files you edited will go back to their last '
                    'saved state.',
              ),
              const DialogBody(children: [Text('This cannot be undone.')]),
              DialogFooter(
                children: [
                  const Spacer(),
                  Button(
                    onPressed: () => Navigator.of(context).pop(),
                    child: const Text('Keep editing'),
                  ),
                  Button(
                    variant: ButtonVariant.filled,
                    tint: ButtonTint.danger,
                    onPressed: () => Navigator.of(context).pop(),
                    child: const Text('Discard'),
                  ),
                ],
              ),
            ],
          ),
        ),
        child: const Text('Show a dialog'),
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return DocPage(
      'Dialog',
      description:
          'A panel that blocks: the header says what is being asked, the body '
          'says what it costs, and the footer is the answer. The scrim is the '
          'content colour at its own alpha rather than black at an opacity, so '
          'it dims the same amount in every theme.',
      children: [
        const DocSection(
          'The parts',
          description:
              '`DialogHeader`, `DialogBody` and `DialogFooter` are separate '
              'widgets rather than slots, so a sheet that only reads can leave '
              'the footer out and a confirmation can leave the body out. The '
              'footer is an action row: the confirming action last, and the '
              'only filled one.',
          children: [
            Col(
              children: [
                _Confirmation(),
                Dialog(
                  children: [
                    DialogBody(
                      children: [
                        Text('A sheet with only something to read.'),
                      ],
                    ),
                  ],
                ),
              ],
            ),
          ],
        ),
        const DocSection(
          'Tone',
          description:
              '`danger` is the one tone: the sheet’s own edge takes the danger '
              'wash so the panel reads as dangerous before the reader gets to '
              'the button. The button still has to be tinted itself — the tone '
              'colours the frame, not the actions in it.',
          children: [_Danger()],
        ),
        DocSection(
          'Tokens',
          description:
              'The dialog’s own; the scrim is a colour and an alpha resolved '
              'together.',
          children: [
            TokenTable([
              TokenRow.length('dialog.width', vars.dialogWidth),
              TokenRow(
                'dialog.scrim.alpha',
                vars.dialogScrimAlpha.toStringAsFixed(2),
              ),
            ]),
          ],
        ),
      ],
    );
  }
}
