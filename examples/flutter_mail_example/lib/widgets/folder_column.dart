import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../data/mails.dart';
import '../l10n.dart';
import 'mail_window.dart';

const Map<Folder, IconData> _folderIcons = {
  Folder.inbox: FluentIcons.mail_inbox_20_regular,
  Folder.sent: FluentIcons.send_20_regular,
  Folder.drafts: FluentIcons.drafts_20_regular,
  Folder.archive: FluentIcons.archive_20_regular,
  Folder.trash: FluentIcons.delete_20_regular,
};

class FolderColumn extends StatelessWidget {
  const FolderColumn({
    super.key,
    required this.box,
    required this.unread,
    required this.onBox,
    required this.onCompose,
  });

  final Mailbox box;
  final Map<Folder, int> unread;
  final ValueChanged<Mailbox> onBox;
  final VoidCallback onCompose;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);

    return Sidebar(
      header: Button(
        tint: ButtonTint.primary,
        variant: ButtonVariant.filled,
        expand: true,
        onPressed: onCompose,
        // Flexible inside the button's own row, so a label too long for the
        // column — a long translation, a large text scale — is cut short
        // with an ellipsis rather than overflowing.
        child: Flexible(
          child: Row(
            mainAxisSize: MainAxisSize.min,
            spacing: 6,
            children: [
              const Icon(FluentIcons.compose_20_regular),
              Flexible(
                child: Text(
                  s.compose,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
              ),
            ],
          ),
        ),
      ),
      children: [
        SidebarGroup(
          label: s.mailboxes,
          children: [
            for (final folder in Folder.values)
              NavItem(
                label: s.folder(folder),
                icon: _folderIcons[folder],
                current: box == FolderBox(folder),
                // A count rather than a badge: a tinted badge vanishes into
                // the filled row of the current folder.
                trailing: (unread[folder] ?? 0) > 0
                    ? Text(
                        '${unread[folder]}',
                        style: vars.labelSmall.copyWith(
                          color: box == FolderBox(folder)
                              ? vars.colorOnAccent
                              : vars.colorContentMuted,
                        ),
                      )
                    : null,
                onPressed: () => onBox(FolderBox(folder)),
              ),
          ],
        ),
        SidebarGroup(
          label: s.labels,
          children: [
            for (final label in Label.values)
              NavItem(
                label: s.label(label),
                icon: FluentIcons.tag_20_regular,
                current: box == LabelBox(label),
                onPressed: () => onBox(LabelBox(label)),
              ),
          ],
        ),
      ],
    );
  }
}
