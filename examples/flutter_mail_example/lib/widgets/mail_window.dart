import 'package:dazzui/dazzui.dart';

import '../data/mails.dart';
import '../l10n.dart';
import '../settings.dart';
import 'compose.dart';
import 'folder_column.dart';
import 'mail_list.dart';
import 'reader.dart';
import 'status_bar.dart';

/// What the list column shows: a folder, or every message under a label.
sealed class Mailbox {
  const Mailbox();
}

class FolderBox extends Mailbox {
  const FolderBox(this.folder);
  final Folder folder;

  @override
  bool operator ==(Object other) =>
      other is FolderBox && other.folder == folder;

  @override
  int get hashCode => folder.hashCode;
}

class LabelBox extends Mailbox {
  const LabelBox(this.label);
  final Label label;

  @override
  bool operator ==(Object other) => other is LabelBox && other.label == label;

  @override
  int get hashCode => label.hashCode;
}

/// The whole window: folders, the message list, the open message, and the
/// status bar that carries the locale and font switches.
class MailWindow extends StatefulWidget {
  const MailWindow({
    super.key,
    this.openId,
    required this.settings,
    required this.onSettingsChanged,
  });

  /// The message open at start. Defaults to the first one in the inbox.
  final String? openId;
  final Settings settings;
  final ValueChanged<Settings> onSettingsChanged;

  @override
  State<MailWindow> createState() => _MailWindowState();
}

class _MailWindowState extends State<MailWindow> {
  final List<Mail> _mails = seedMails();
  Mailbox _box = const FolderBox(Folder.inbox);
  String _query = '';
  late String? _openId = widget.openId ?? 'weekly';
  bool _composing = false;
  int _sentCount = 0;

  List<Mail> get _visible {
    final String query = _query.trim().toLowerCase();
    return _mails.where((mail) {
      final bool inBox = switch (_box) {
        FolderBox(:final folder) => mail.folder == folder,
        LabelBox(:final label) =>
          mail.labels.contains(label) && mail.folder != Folder.trash,
      };
      if (!inBox) return false;
      if (query.isEmpty) return true;
      return mail.subject.toLowerCase().contains(query) ||
          mail.from.name.toLowerCase().contains(query) ||
          mail.body.toLowerCase().contains(query);
    }).toList();
  }

  Mail? get _open {
    for (final mail in _mails) {
      if (mail.id == _openId) return mail;
    }
    return null;
  }

  int _unreadIn(Folder folder) =>
      _mails.where((mail) => mail.folder == folder && mail.unread).length;

  void _openMail(Mail mail) {
    setState(() {
      _openId = mail.id;
      mail.unread = false;
    });
  }

  void _move(Mail mail, Folder folder) {
    setState(() {
      mail.folder = folder;
      if (_openId == mail.id) _openId = null;
    });
  }

  void _send({
    required List<Person> to,
    required String subject,
    required String body,
  }) {
    setState(() {
      _sentCount++;
      _mails.insert(
        0,
        Mail(
          id: 'sent-$_sentCount',
          folder: Folder.sent,
          from: me,
          to: to,
          subject: subject.trim(),
          sent: DateTime.now(),
          body: body,
        ),
      );
    });
  }

  void _reply(Mail mail, String text) {
    final S s = S.of(context);
    final String quoted = mail.body
        .split('\n')
        .map((line) => '> $line')
        .join('\n');
    _send(
      to: [mail.from],
      subject: mail.subject.startsWith('Re:')
          ? mail.subject
          : 'Re: ${mail.subject}',
      body: '$text\n\n> ${s.wrote(s.nameOf(mail.from))}\n$quoted',
    );
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);
    final Mail? open = _open;

    return ColoredBox(
      color: vars.colorSurface,
      child: Stack(
        children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Expanded(
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    FolderColumn(
                      box: _box,
                      unread: {
                        for (final folder in Folder.values)
                          folder: _unreadIn(folder),
                      },
                      onBox: (box) => setState(() => _box = box),
                      onCompose: () => setState(() => _composing = true),
                    ),
                    MailList(
                      title: switch (_box) {
                        FolderBox(:final folder) => s.folder(folder),
                        LabelBox(:final label) => s.labelTitle(s.label(label)),
                      },
                      mails: _visible,
                      openId: _openId,
                      query: _query,
                      onQuery: (query) => setState(() => _query = query),
                      onOpen: _openMail,
                      onToggleRead: (mail) =>
                          setState(() => mail.unread = !mail.unread),
                      onToggleStar: (mail) =>
                          setState(() => mail.starred = !mail.starred),
                      onMove: _move,
                    ),
                    Expanded(
                      child: open == null
                          ? const EmptyReader()
                          : Reader(
                              key: ValueKey(open.id),
                              mail: open,
                              onToggleStar: () =>
                                  setState(() => open.starred = !open.starred),
                              onMarkUnread: () => setState(() {
                                open.unread = true;
                                _openId = null;
                              }),
                              onMove: (folder) => _move(open, folder),
                              onReply: (text) => _reply(open, text),
                            ),
                    ),
                  ],
                ),
              ),
              StatusBar(
                settings: widget.settings,
                onChanged: widget.onSettingsChanged,
              ),
            ],
          ),
          if (_composing)
            Positioned.fill(
              child: Compose(
                onCancel: () => setState(() => _composing = false),
                onSend: (to, subject, body) {
                  _send(to: [Person(to, to)], subject: subject, body: body);
                  setState(() => _composing = false);
                },
              ),
            ),
        ],
      ),
    );
  }
}
