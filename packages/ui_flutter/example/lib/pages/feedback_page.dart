import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../gallery.dart';

/// What the app says back: the callouts that stay, in every tint, and the one
/// that carries actions.
class FeedbackPage extends StatefulWidget {
  const FeedbackPage({super.key});

  @override
  State<FeedbackPage> createState() => _FeedbackPageState();
}

class _FeedbackPageState extends State<FeedbackPage> {
  Future<void> _confirm(BuildContext context) {
    return showDialog<void>(
      context: context,
      builder: (context) => Dialog(
        tone: DialogTone.danger,
        children: [
          const DialogHeader(
            title: 'Discard these changes?',
            subtitle:
                'The three files you edited will go back to their '
                'last saved state.',
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
    );
  }

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing6,
      children: [
        const PageTitle(
          'Feedback',
          'The tint carries the meaning; the shape carries how long it stays.',
        ),
        Specimen(
          'Callouts',
          child: ConstrainedBox(
            // The SwiftUI playground caps its callouts at 1.5 toast widths.
            constraints: BoxConstraints(maxWidth: vars.toastMaxWidth * 1.5),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing2,
              children: const [
                Callout(
                  tint: CalloutTint.info,
                  icon: Icon(FluentIcons.info_16_regular),
                  title: Text('A new version is available'),
                  message: Text('Restart to finish updating to 2.1.0.'),
                ),
                Callout(
                  tint: CalloutTint.success,
                  icon: Icon(FluentIcons.checkmark_circle_16_regular),
                  title: Text('Tokens rebuilt'),
                  message: Text('Three platform packages were refreshed.'),
                ),
                Callout(
                  tint: CalloutTint.warning,
                  icon: Icon(FluentIcons.warning_16_regular),
                  title: Text('Ramp moves only part-way'),
                  message: Text(
                    'color.warning cannot move whole under bright-dark.',
                  ),
                ),
                Callout(
                  tint: CalloutTint.danger,
                  icon: Icon(FluentIcons.dismiss_circle_16_regular),
                  title: Text('The build failed'),
                  message: Text('dazzui: 4 errors in sidebar.dart.'),
                ),
              ],
            ),
          ),
        ),
        Specimen(
          'With actions',
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: vars.toastMaxWidth * 1.5),
            child: Callout(
              tint: CalloutTint.primary,
              icon: const Icon(FluentIcons.arrow_download_16_regular),
              title: const Text('Update ready'),
              message: const Text('2.1.0 has been downloaded.'),
              actions: [
                Button(
                  size: WidgetSize.small,
                  variant: ButtonVariant.tinted,
                  onPressed: () {},
                  child: const Text('Restart now'),
                ),
                Button(
                  size: WidgetSize.small,
                  variant: ButtonVariant.plain,
                  onPressed: () {},
                  child: const Text('Later'),
                ),
              ],
            ),
          ),
        ),
        Specimen(
          'Empty state',
          child: Card(
            variant: CardVariant.sunken,
            child: EmptyState(
              title: 'No components match this filter',
              actions: [
                Button(
                  variant: ButtonVariant.tinted,
                  onPressed: () {},
                  child: const Text('Clear the filter'),
                ),
              ],
            ),
          ),
        ),
        Specimen(
          'Toasts',
          note: 'one line, no wrapping — a second sentence is a callout',
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing2,
            children: [
              Toast(
                tint: ToastTint.success,
                icon: FluentIcons.checkmark_circle_16_regular,
                message: 'Tokens rebuilt — three packages refreshed.',
                onDismiss: () {},
              ),
              Toast(
                tint: ToastTint.danger,
                icon: FluentIcons.dismiss_circle_16_regular,
                message: 'The build failed.',
                action: Button(
                  size: WidgetSize.tiny,
                  variant: ButtonVariant.plain,
                  onPressed: () {},
                  child: const Text('Retry'),
                ),
                onDismiss: () {},
              ),
            ],
          ),
        ),
        Specimen(
          'Menu',
          note: 'opens on the trigger; Escape or an outside press closes it',
          child: Menu(
            trigger: (context, state) => Button(
              variant: state.open
                  ? ButtonVariant.tinted
                  : ButtonVariant.recessed,
              onPressed: state.toggle,
              shortcut: const Text('▾'),
              child: const Text('Actions'),
            ),
            items: [
              MenuItem(
                label: 'Rename',
                icon: FluentIcons.rename_16_regular,
                shortcut: '⌘R',
                onSelect: () {},
              ),
              MenuItem(
                label: 'Duplicate',
                icon: FluentIcons.square_multiple_16_regular,
                onSelect: () {},
              ),
              MenuItem(
                label: 'Show hidden files',
                checked: true,
                onSelect: () {},
              ),
              const MenuItem(
                label: 'Delete',
                icon: FluentIcons.delete_16_regular,
                enabled: false,
              ),
            ],
          ),
        ),
        Specimen(
          'Dialog',
          note: 'the scrim covers the window, not this column',
          child: Button(
            variant: ButtonVariant.filled,
            onPressed: () => _confirm(context),
            child: const Text('Discard changes…'),
          ),
        ),
        Specimen(
          'Empty state',
          child: Card(
            variant: CardVariant.sunken,
            child: EmptyState(
              title: 'No components match this filter',
              actions: [
                Button(
                  variant: ButtonVariant.tinted,
                  onPressed: () {},
                  child: const Text('Clear the filter'),
                ),
              ],
            ),
          ),
        ),
        Specimen(
          'Toasts',
          note: 'one line, no wrapping — a second sentence is a callout',
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing2,
            children: [
              Toast(
                tint: ToastTint.success,
                icon: FluentIcons.checkmark_circle_16_regular,
                message: 'Tokens rebuilt — three packages refreshed.',
                onDismiss: () {},
              ),
              Toast(
                tint: ToastTint.danger,
                icon: FluentIcons.dismiss_circle_16_regular,
                message: 'The build failed.',
                action: Button(
                  size: WidgetSize.tiny,
                  variant: ButtonVariant.plain,
                  onPressed: () {},
                  child: const Text('Retry'),
                ),
                onDismiss: () {},
              ),
            ],
          ),
        ),
        Specimen(
          'Menu',
          note: 'opens on the trigger; Escape or an outside press closes it',
          child: Menu(
            trigger: (context, state) => Button(
              variant: state.open
                  ? ButtonVariant.tinted
                  : ButtonVariant.recessed,
              onPressed: state.toggle,
              shortcut: const Text('▾'),
              child: const Text('Actions'),
            ),
            items: [
              MenuItem(
                label: 'Rename',
                icon: FluentIcons.rename_16_regular,
                shortcut: '⌘R',
                onSelect: () {},
              ),
              MenuItem(
                label: 'Duplicate',
                icon: FluentIcons.square_multiple_16_regular,
                onSelect: () {},
              ),
              MenuItem(
                label: 'Show hidden files',
                checked: true,
                onSelect: () {},
              ),
              const MenuItem(
                label: 'Delete',
                icon: FluentIcons.delete_16_regular,
                enabled: false,
              ),
            ],
          ),
        ),
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}
