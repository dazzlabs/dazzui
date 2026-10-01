import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../data/mails.dart';
import '../l10n.dart';

/// The open message: a toolbar, the headers, the body, and a reply box at
/// the foot — the one place an input method gets exercised in line.
class Reader extends StatefulWidget {
  const Reader({
    super.key,
    required this.mail,
    required this.onToggleStar,
    required this.onMarkUnread,
    required this.onMove,
    required this.onReply,
  });

  final Mail mail;
  final VoidCallback onToggleStar;
  final VoidCallback onMarkUnread;
  final ValueChanged<Folder> onMove;
  final ValueChanged<String> onReply;

  @override
  State<Reader> createState() => _ReaderState();
}

class _ReaderState extends State<Reader> {
  final TextEditingController _reply = TextEditingController();
  final FocusNode _replyFocus = FocusNode();

  @override
  void dispose() {
    _reply.dispose();
    _replyFocus.dispose();
    super.dispose();
  }

  void _send() {
    if (_reply.text.trim().isEmpty) return;
    widget.onReply(_reply.text.trim());
    _reply.clear();
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    final Mail mail = widget.mail;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        _Toolbar(
          mail: mail,
          onReply: () => _replyFocus.requestFocus(),
          onToggleStar: widget.onToggleStar,
          onMarkUnread: widget.onMarkUnread,
          onMove: widget.onMove,
        ),
        const Divider(),
        Expanded(
          child: SelectableRegion(
            selectionControls: emptyTextSelectionControls,
            child: ListView(
              padding: EdgeInsets.symmetric(
                horizontal: vars.spacing8,
                vertical: vars.spacing6,
              ),
              children: [
                Align(
                  alignment: AlignmentDirectional.topStart,
                  child: ConstrainedBox(
                    constraints: const BoxConstraints(maxWidth: 720),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        Text(s.subjectOf(mail), style: vars.headlineSmall),
                        SizedBox(height: vars.spacing5),
                        _Headers(mail: mail),
                        if (mail.labels.isNotEmpty ||
                            mail.attachments.isNotEmpty) ...[
                          SizedBox(height: vars.spacing4),
                          Wrap(
                            spacing: vars.spacing2,
                            runSpacing: vars.spacing2,
                            children: [
                              for (final label in mail.labels)
                                Tag(
                                  label: Text(s.label(label)),
                                  icon: FluentIcons.tag_20_regular,
                                  size: WidgetSize.small,
                                ),
                              for (final name in mail.attachments)
                                Tag(
                                  label: Text(name),
                                  icon: FluentIcons.attach_20_regular,
                                  size: WidgetSize.small,
                                  tint: TagTint.neutral,
                                ),
                            ],
                          ),
                        ],
                        SizedBox(height: vars.spacing6),
                        _Body(text: mail.body),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
        const Divider(),
        Padding(
          padding: EdgeInsets.all(vars.spacing4),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.end,
            spacing: vars.spacing3,
            children: [
              Expanded(
                child: TextField(
                  controller: _reply,
                  focusNode: _replyFocus,
                  minLines: 1,
                  maxLines: 5,
                  placeholder: s.replyPlaceholder(s.nameOf(mail.from)),
                ),
              ),
              Button(
                tint: ButtonTint.primary,
                variant: ButtonVariant.filled,
                size: WidgetSize.medium,
                onPressed: _send,
                child: Text(s.send),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class _Toolbar extends StatelessWidget {
  const _Toolbar({
    required this.mail,
    required this.onReply,
    required this.onToggleStar,
    required this.onMarkUnread,
    required this.onMove,
  });

  final Mail mail;
  final VoidCallback onReply;
  final VoidCallback onToggleStar;
  final VoidCallback onMarkUnread;
  final ValueChanged<Folder> onMove;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);

    Widget action(String label, IconData icon, VoidCallback? onPressed) {
      return Tooltip(
        label: label,
        side: PopoverSide.bottom,
        child: IconButton(
          icon: Icon(icon),
          size: WidgetSize.small,
          variant: IconButtonVariant.plain,
          semanticsLabel: label,
          onPressed: onPressed,
        ),
      );
    }

    return Container(
      height: vars.frameTitlebarSize,
      padding: EdgeInsets.symmetric(horizontal: vars.spacing3),
      child: Row(
        spacing: vars.spacing1,
        children: [
          action(s.reply, FluentIcons.arrow_reply_20_regular, onReply),
          action(s.forward, FluentIcons.arrow_forward_20_regular, null),
          action(
            s.archive,
            FluentIcons.archive_20_regular,
            mail.folder == Folder.archive ? null : () => onMove(Folder.archive),
          ),
          action(
            s.delete,
            FluentIcons.delete_20_regular,
            mail.folder == Folder.trash ? null : () => onMove(Folder.trash),
          ),
          const Spacer(),
          action(
            s.markUnread,
            FluentIcons.mail_unread_20_regular,
            onMarkUnread,
          ),
          action(
            mail.starred ? s.unstar : s.star,
            mail.starred
                ? FluentIcons.star_20_filled
                : FluentIcons.star_20_regular,
            onToggleStar,
          ),
          Menu(
            trigger: (context, state) => IconButton(
              icon: const Icon(FluentIcons.more_horizontal_20_regular),
              size: WidgetSize.small,
              variant: IconButtonVariant.plain,
              semanticsLabel: s.more,
              onPressed: state.toggle,
            ),
            items: [
              MenuItem(
                label: s.moveToInbox,
                icon: FluentIcons.mail_inbox_20_regular,
                enabled: mail.folder != Folder.inbox,
                onSelect: () => onMove(Folder.inbox),
              ),
              MenuItem(label: s.showOriginal, shortcut: '⌥⌘U'),
              MenuItem(label: s.print, shortcut: '⌘P'),
            ],
          ),
        ],
      ),
    );
  }
}

class _Headers extends StatelessWidget {
  const _Headers({required this.mail});

  final Mail mail;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    final TextStyle muted = vars.bodySmall.copyWith(
      color: vars.colorContentMuted,
    );

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      spacing: vars.spacing3,
      children: [
        Avatar(name: s.nameOf(mail.from), size: WidgetSize.large),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            spacing: vars.spacing05,
            children: [
              Row(
                crossAxisAlignment: CrossAxisAlignment.baseline,
                textBaseline: TextBaseline.alphabetic,
                spacing: vars.spacing2,
                children: [
                  Text(s.nameOf(mail.from), style: vars.titleSmall),
                  Flexible(
                    child: Text(
                      '<${mail.from.address}>',
                      style: muted,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                ],
              ),
              Text(s.toLine(mail.to.map(s.nameOf).toList()), style: muted),
            ],
          ),
        ),
        Text(s.date(mail.sent), style: muted),
      ],
    );
  }
}

/// The body, paragraph by paragraph: plain, quoted (`> `) or code (`$ `).
class _Body extends StatelessWidget {
  const _Body({required this.text});

  final String text;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final TextStyle body = vars.bodyLarge.copyWith(height: 1.75);
    final List<String> paragraphs = text.split(RegExp(r'\n\s*\n'));

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      spacing: vars.spacing4,
      children: [
        for (final paragraph in paragraphs)
          if (paragraph.startsWith('> '))
            Container(
              padding: EdgeInsetsDirectional.only(start: vars.spacing4),
              decoration: BoxDecoration(
                border: BorderDirectional(
                  start: BorderSide(color: vars.colorBorderStrong, width: 3),
                ),
              ),
              child: Text(
                _strip(paragraph, '> '),
                style: body.copyWith(color: vars.colorContentMuted),
              ),
            )
          else if (paragraph.startsWith(r'$ '))
            Container(
              padding: EdgeInsets.all(vars.spacing3),
              decoration: BoxDecoration(
                color: vars.colorSurfaceSunken,
                borderRadius: BorderRadius.circular(vars.radiusMedium),
              ),
              child: Text(
                _strip(paragraph, r'$ '),
                style: TextStyle(
                  fontFamily: vars.fontCode.family,
                  fontFamilyFallback: vars.fontCode.fallback,
                  fontSize: 13,
                  height: 1.6,
                  color: vars.colorContent,
                ),
              ),
            )
          else
            Text(paragraph, style: body),
      ],
    );
  }

  static String _strip(String paragraph, String prefix) => paragraph
      .split('\n')
      .map(
        (line) => line.startsWith(prefix)
            ? line.substring(prefix.length)
            : line.replaceFirst(RegExp(r'^[>$]$'), ''),
      )
      .join('\n');
}

class EmptyReader extends StatelessWidget {
  const EmptyReader({super.key});

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing2,
        children: [
          Icon(
            FluentIcons.mail_20_regular,
            size: 40,
            color: vars.colorContentFaint,
          ),
          Text(
            s.selectMail,
            style: vars.bodyMedium.copyWith(color: vars.colorContentMuted),
          ),
        ],
      ),
    );
  }
}
