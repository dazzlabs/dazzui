import 'package:dazzui/dazzui.dart';
import 'package:flutter/services.dart';

import '../l10n.dart';

/// A new message, over a scrim: three fields to type Chinese into, single
/// line and multi-line.
class Compose extends StatefulWidget {
  const Compose({super.key, required this.onCancel, required this.onSend});

  final VoidCallback onCancel;
  final void Function(String to, String subject, String body) onSend;

  @override
  State<Compose> createState() => _ComposeState();
}

class _ComposeState extends State<Compose> {
  final TextEditingController _to = TextEditingController();
  final TextEditingController _subject = TextEditingController();
  final TextEditingController _body = TextEditingController();

  @override
  void dispose() {
    _to.dispose();
    _subject.dispose();
    _body.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final S s = S.of(context);

    return CallbackShortcuts(
      bindings: {
        const SingleActivator(LogicalKeyboardKey.escape): widget.onCancel,
      },
      child: DialogScrim(
        onDismiss: widget.onCancel,
        child: SizedBox(
          width: 640,
          child: Dialog(
            children: [
              DialogHeader(title: s.compose, subtitle: s.composeSubtitle),
              DialogBody(
                children: [
                  FormField(
                    label: s.recipient,
                    child: TextField(
                      controller: _to,
                      autofocus: true,
                      placeholder: s.recipientPlaceholder,
                    ),
                  ),
                  FormField(
                    label: s.subject,
                    child: TextField(
                      controller: _subject,
                      placeholder: s.subjectPlaceholder,
                    ),
                  ),
                  SizedBox(
                    height: 220,
                    child: TextField(
                      controller: _body,
                      expands: true,
                      maxLines: null,
                      textAlignVertical: TextAlignVertical.top,
                      placeholder: s.bodyPlaceholder,
                      padding: EdgeInsets.all(vars.spacing3),
                    ),
                  ),
                ],
              ),
              DialogFooter(
                children: [
                  Button(onPressed: widget.onCancel, child: Text(s.cancel)),
                  Button(
                    tint: ButtonTint.primary,
                    variant: ButtonVariant.filled,
                    onPressed: () => widget.onSend(
                      _to.text.trim(),
                      _subject.text,
                      _body.text,
                    ),
                    child: Text(s.send),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
