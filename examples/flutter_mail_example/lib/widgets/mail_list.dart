import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../data/mails.dart';
import '../l10n.dart';

/// The middle column: the mailbox's name, a search field, and one row per
/// message — sender and date, the subject in one line, the preview in two.
/// The two truncations are where a CJK ellipsis goes wrong.
class MailList extends StatelessWidget {
  const MailList({
    super.key,
    required this.title,
    required this.mails,
    required this.openId,
    required this.query,
    required this.onQuery,
    required this.onOpen,
    required this.onToggleRead,
    required this.onToggleStar,
    required this.onMove,
  });

  final String title;
  final List<Mail> mails;
  final String? openId;
  final String query;
  final ValueChanged<String> onQuery;
  final ValueChanged<Mail> onOpen;
  final ValueChanged<Mail> onToggleRead;
  final ValueChanged<Mail> onToggleStar;
  final void Function(Mail mail, Folder folder) onMove;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    final int unread = mails.where((mail) => mail.unread).length;

    return Container(
      width: 360,
      decoration: BoxDecoration(
        color: vars.colorSurface,
        border: BorderDirectional(
          end: BorderSide(
            color: vars.colorBorder,
            width: context.hairlineWidth,
          ),
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Container(
            height: vars.frameTitlebarSize,
            padding: EdgeInsets.symmetric(horizontal: vars.spacing4),
            alignment: AlignmentDirectional.centerStart,
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.baseline,
              textBaseline: TextBaseline.alphabetic,
              spacing: vars.spacing2,
              children: [
                Text(title, style: vars.titleMedium),
                Text(
                  s.count(mails.length, unread),
                  style: vars.captionMedium.copyWith(
                    color: vars.colorContentMuted,
                  ),
                ),
              ],
            ),
          ),
          Padding(
            padding: EdgeInsets.fromLTRB(
              vars.spacing3,
              0,
              vars.spacing3,
              vars.spacing2,
            ),
            child: SearchField(
              value: query,
              placeholder: s.searchPlaceholder,
              clearLabel: s.clearSearch,
              onChanged: onQuery,
            ),
          ),
          Expanded(
            child: mails.isEmpty
                ? Center(
                    child: Text(
                      query.isEmpty ? s.emptyBox : s.noMatch(query),
                      style: vars.bodyMedium.copyWith(
                        color: vars.colorContentMuted,
                      ),
                    ),
                  )
                : ListView.builder(
                    padding: EdgeInsets.symmetric(horizontal: vars.spacing2),
                    itemCount: mails.length,
                    itemBuilder: (context, index) {
                      final Mail mail = mails[index];
                      return ContextMenu(
                        items: [
                          MenuItem(
                            label: mail.unread ? s.markRead : s.markUnread,
                            icon: mail.unread
                                ? FluentIcons.mail_read_20_regular
                                : FluentIcons.mail_unread_20_regular,
                            onSelect: () => onToggleRead(mail),
                          ),
                          MenuItem(
                            label: mail.starred ? s.unstar : s.star,
                            icon: FluentIcons.star_20_regular,
                            onSelect: () => onToggleStar(mail),
                          ),
                          MenuItem(
                            label: s.archive,
                            icon: FluentIcons.archive_20_regular,
                            enabled: mail.folder != Folder.archive,
                            onSelect: () => onMove(mail, Folder.archive),
                          ),
                          MenuItem(
                            label: s.delete,
                            icon: FluentIcons.delete_20_regular,
                            enabled: mail.folder != Folder.trash,
                            onSelect: () => onMove(mail, Folder.trash),
                          ),
                        ],
                        child: _Row(
                          mail: mail,
                          open: mail.id == openId,
                          onPressed: () => onOpen(mail),
                        ),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }
}

class _Row extends StatelessWidget {
  const _Row({required this.mail, required this.open, required this.onPressed});

  final Mail mail;
  final bool open;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    final BorderRadius radius = BorderRadius.circular(vars.radiusMedium);
    // The sent folder lists who it went to rather than who sent it.
    final Person who = mail.from.isMe && mail.to.isNotEmpty
        ? mail.to.first
        : mail.from;

    return Pressable(
      onPressed: onPressed,
      selected: open,
      borderRadius: radius,
      builder: (context, states) {
        final bool hovered = states.contains(WidgetState.hovered);
        final Color? surface = open
            ? vars.colorPrimary[500]!.withValues(alpha: 0.14)
            : hovered
            ? vars.colorContent.withValues(alpha: 0.05)
            : null;
        final FontWeight weight = mail.unread
            ? FontWeight.w600
            : FontWeight.w400;

        return Container(
          margin: EdgeInsets.only(bottom: vars.spacingPx),
          padding: EdgeInsets.all(vars.spacing3),
          decoration: BoxDecoration(color: surface, borderRadius: radius),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            spacing: vars.spacing3,
            children: [
              Stack(
                clipBehavior: Clip.none,
                children: [
                  Avatar(name: s.nameOf(who)),
                  if (mail.unread)
                    Positioned(
                      left: -6,
                      top: 12,
                      child: Container(
                        width: 6,
                        height: 6,
                        decoration: BoxDecoration(
                          color: vars.colorPrimary[500],
                          shape: BoxShape.circle,
                        ),
                      ),
                    ),
                ],
              ),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  spacing: vars.spacing05,
                  children: [
                    Row(
                      spacing: vars.spacing2,
                      children: [
                        Expanded(
                          child: Text(
                            s.nameOf(who),
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: vars.labelLarge.copyWith(fontWeight: weight),
                          ),
                        ),
                        if (mail.attachments.isNotEmpty)
                          Icon(
                            FluentIcons.attach_20_regular,
                            size: vars.iconSmall,
                            color: vars.colorContentMuted,
                          ),
                        if (mail.starred)
                          Icon(
                            FluentIcons.star_20_filled,
                            size: vars.iconSmall,
                            color: vars.colorWarning[500],
                          ),
                        Text(
                          s.date(mail.sent),
                          style: vars.captionMedium.copyWith(
                            color: vars.colorContentMuted,
                          ),
                        ),
                      ],
                    ),
                    Text(
                      s.subjectOf(mail),
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: vars.bodyMedium.copyWith(fontWeight: weight),
                    ),
                    Text(
                      mail.preview,
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                      style: vars.bodySmall.copyWith(
                        color: vars.colorContentMuted,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
