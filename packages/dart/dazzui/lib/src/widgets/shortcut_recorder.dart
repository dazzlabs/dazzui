import 'dart:async';

import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

import '../foundation/widget_tint.dart';
import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'pressable.dart';
import 'text_field.dart';

/// Keys that name themselves as a glyph, by what they are.
final Map<LogicalKeyboardKey, String> _glyphs = {
  LogicalKeyboardKey.space: 'Space',
  LogicalKeyboardKey.enter: '↩',
  LogicalKeyboardKey.numpadEnter: '⌤',
  LogicalKeyboardKey.tab: '⇥',
  LogicalKeyboardKey.backspace: '⌫',
  LogicalKeyboardKey.delete: '⌦',
  LogicalKeyboardKey.arrowUp: '↑',
  LogicalKeyboardKey.arrowDown: '↓',
  LogicalKeyboardKey.arrowLeft: '←',
  LogicalKeyboardKey.arrowRight: '→',
  LogicalKeyboardKey.home: '↖',
  LogicalKeyboardKey.end: '↘',
  LogicalKeyboardKey.pageUp: '⇞',
  LogicalKeyboardKey.pageDown: '⇟',
  LogicalKeyboardKey.minus: '-',
  LogicalKeyboardKey.equal: '=',
  LogicalKeyboardKey.bracketLeft: '[',
  LogicalKeyboardKey.bracketRight: ']',
  LogicalKeyboardKey.backslash: r'\',
  LogicalKeyboardKey.semicolon: ';',
  LogicalKeyboardKey.quoteSingle: "'",
  LogicalKeyboardKey.comma: ',',
  LogicalKeyboardKey.period: '.',
  LogicalKeyboardKey.slash: '/',
  LogicalKeyboardKey.backquote: '`',
};

final Set<LogicalKeyboardKey> _modifiers = {
  ...LogicalKeyboardKey.expandSynonyms({
    LogicalKeyboardKey.control,
    LogicalKeyboardKey.alt,
    LogicalKeyboardKey.shift,
    LogicalKeyboardKey.meta,
  }),
  LogicalKeyboardKey.control,
  LogicalKeyboardKey.alt,
  LogicalKeyboardKey.shift,
  LogicalKeyboardKey.meta,
};

final RegExp _functionKey = RegExp(r'^F([1-9]|1\d|20)$');

/// The key an event names, as the recorder writes it — read off the
/// physical key for letters and digits, so ⌥A is `A` and not the `å` the
/// layout would type.
String? _keyGlyph(KeyEvent event) {
  final String? named = _glyphs[event.logicalKey];
  if (named != null) return named;
  final int usage = event.physicalKey.usbHidUsage;
  if (usage >= 0x00070004 && usage <= 0x0007001d) {
    return String.fromCharCode(0x41 + usage - 0x00070004);
  }
  if (usage >= 0x0007001e && usage <= 0x00070027) {
    return usage == 0x00070027 ? '0' : '${usage - 0x0007001e + 1}';
  }
  if (usage >= 0x00070059 && usage <= 0x00070062) {
    return usage == 0x00070062 ? '0' : '${usage - 0x00070059 + 1}';
  }
  final String label = event.logicalKey.keyLabel;
  if (_functionKey.hasMatch(label)) return label;
  return null;
}

String _modifierGlyphs(HardwareKeyboard keyboard) {
  return [
    if (keyboard.isControlPressed) '⌃',
    if (keyboard.isAltPressed) '⌥',
    if (keyboard.isShiftPressed) '⇧',
    if (keyboard.isMetaPressed) '⌘',
  ].join();
}

/// The shortcut an event records, or null when it is not one: a bare letter
/// is typing, so a key needs ⌃, ⌥ or ⌘ unless it is a function key.
String? formatShortcut(KeyEvent event, HardwareKeyboard keyboard) {
  final String? key = _keyGlyph(event);
  if (key == null) return null;
  final bool function = RegExp(r'^F\d+$').hasMatch(key);
  if (!function &&
      !keyboard.isControlPressed &&
      !keyboard.isAltPressed &&
      !keyboard.isMetaPressed) {
    return null;
  }
  final String mods = _modifierGlyphs(keyboard);
  return key.length > 1 ? '$mods${mods.isEmpty ? '' : ' '}$key' : '$mods$key';
}

/// A field that takes the keys pressed in it as its value.
///
/// Pressing it starts recording: the modifiers held are shown as they are
/// held, a chord ends it and becomes [value], Escape abandons it, and a bare
/// Backspace or Delete clears. A press that is not a shortcut — a bare
/// letter — flashes the edge rather than being taken.
class ShortcutRecorder extends StatefulWidget {
  const ShortcutRecorder({
    super.key,
    required this.value,
    required this.onChanged,
    this.onClear,
    this.placeholder = 'Record shortcut',
    this.recordingLabel = 'Press keys…',
    this.clearLabel = 'Clear shortcut',
    this.state = TextFieldState.normal,
    this.enabled = true,
    this.tint = TextFieldTint.primary,
    this.semanticsLabel,
  });

  final String value;
  final ValueChanged<String>? onChanged;

  /// Called for a clear; without it a clear is [onChanged] with `''`.
  final VoidCallback? onClear;
  final String placeholder;
  final String recordingLabel;
  final String clearLabel;

  /// An error paints the stored value as a conflict.
  final TextFieldState state;
  final bool enabled;
  final TextFieldTint tint;
  final String? semanticsLabel;

  @override
  State<ShortcutRecorder> createState() => _ShortcutRecorderState();
}

class _ShortcutRecorderState extends State<ShortcutRecorder> {
  late final FocusNode _focus = FocusNode(onKeyEvent: _onKey);
  bool _recording = false;
  String _held = '';
  bool _rejected = false;
  Timer? _flash;

  @override
  void dispose() {
    _flash?.cancel();
    _focus.dispose();
    super.dispose();
  }

  void _stop() {
    if (!_recording && _held.isEmpty) return;
    setState(() {
      _recording = false;
      _held = '';
    });
  }

  void _clear() {
    _stop();
    if (widget.onClear != null) {
      widget.onClear!();
    } else {
      widget.onChanged?.call('');
    }
  }

  void _reject() {
    _flash?.cancel();
    setState(() => _rejected = true);
    _flash = Timer(const Duration(milliseconds: 360), () {
      if (mounted) setState(() => _rejected = false);
    });
  }

  KeyEventResult _onKey(FocusNode node, KeyEvent event) {
    if (!_recording) return KeyEventResult.ignored;
    final HardwareKeyboard keyboard = HardwareKeyboard.instance;
    if (_modifiers.contains(event.logicalKey)) {
      setState(() => _held = _modifierGlyphs(keyboard));
      return KeyEventResult.handled;
    }
    if (event is KeyUpEvent) return KeyEventResult.handled;
    if (event.logicalKey == LogicalKeyboardKey.escape) {
      _stop();
      return KeyEventResult.handled;
    }
    final bool bare =
        !keyboard.isControlPressed &&
        !keyboard.isAltPressed &&
        !keyboard.isMetaPressed &&
        !keyboard.isShiftPressed;
    if (bare &&
        (event.logicalKey == LogicalKeyboardKey.backspace ||
            event.logicalKey == LogicalKeyboardKey.delete)) {
      _clear();
      return KeyEventResult.handled;
    }
    final String? next = formatShortcut(event, keyboard);
    if (next == null) {
      _reject();
      return KeyEventResult.handled;
    }
    _stop();
    widget.onChanged?.call(next);
    return KeyEventResult.handled;
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final ColorSwatch<int> ramp = switch (widget.tint.namedTint) {
      NamedTint.primary => vars.colorPrimary,
      NamedTint.neutral => vars.colorNeutral,
      NamedTint.info => vars.colorInfo,
      NamedTint.success => vars.colorSuccess,
      NamedTint.warning => vars.colorWarning,
      NamedTint.danger => vars.colorDanger,
    };
    final bool bound = widget.value.isNotEmpty;
    final String label = _recording
        ? (_held.isNotEmpty ? _held : widget.recordingLabel)
        : bound
        ? widget.value
        : widget.placeholder;
    final bool glyphs = _recording ? _held.isNotEmpty : bound;
    final bool clearable = bound && !_recording && widget.enabled;
    final bool error = widget.state == TextFieldState.error;

    final Color border = _rejected || error
        ? vars.colorDanger[600]!
        : _recording
        ? ramp[600]!
        : vars.colorBorderStrong;
    final Color surface = error
        ? vars.colorDanger[50]!
        : _recording
        ? vars.colorSurface
        : vars.colorSurfaceMuted;
    final Color? glow = _rejected
        ? vars.colorDanger[50]
        : _recording
        ? ramp[vars.focusGlowShade]!.withValues(alpha: vars.focusGlowAlpha)
        : null;
    final TextStyle face = glyphs ? vars.labelStrong : vars.labelQuiet;
    final Color ink = !widget.enabled || !glyphs
        ? vars.colorContentFaint
        : error
        ? vars.colorDanger[800]!
        : vars.colorContent;
    final BorderRadius radius = BorderRadius.circular(vars.controlFieldRadius);

    final Widget field = Pressable(
      focusNode: _focus,
      enabled: widget.enabled,
      borderRadius: radius,
      semanticsLabel: widget.semanticsLabel,
      checked: _recording,
      onFocusChange: (focused) {
        if (!focused) _stop();
      },
      onPressed: () {
        if (_recording) return;
        setState(() => _recording = true);
        _focus.requestFocus();
      },
      builder: (context, states) => Opacity(
        opacity: widget.enabled ? 1 : 0.6,
        child: AnimatedContainer(
          duration: vars.motionDuration,
          curve: vars.motionEasing,
          height: vars.controlMediumSize,
          constraints: BoxConstraints(minWidth: vars.shortcutRecorderWidth),
          padding: EdgeInsets.symmetric(
            horizontal: clearable ? vars.spacing8 : vars.spacing3,
          ),
          alignment: Alignment.center,
          decoration: BoxDecoration(
            color: surface,
            border: Border.all(color: border, width: context.hairlineWidth),
            borderRadius: radius,
            boxShadow: glow == null
                ? null
                : [BoxShadow(color: glow, spreadRadius: vars.focusWidth)],
          ),
          child: Text(
            label,
            maxLines: 1,
            softWrap: false,
            style: face.copyWith(color: ink),
          ),
        ),
      ),
    );

    if (!clearable) return field;
    return Stack(
      alignment: AlignmentDirectional.centerEnd,
      children: [
        field,
        PositionedDirectional(
          end: vars.spacing2,
          child: Pressable(
            semanticsLabel: widget.clearLabel,
            onPressed: _clear,
            borderRadius: BorderRadius.circular(vars.radiusFull),
            builder: (context, states) => Icon(
              FluentIcons.dismiss_circle_12_filled,
              size: vars.labelMedium.fontSize,
              color: states.contains(WidgetState.hovered)
                  ? vars.colorContentSubtle
                  : vars.colorContentFaint,
            ),
          ),
        ),
      ],
    );
  }
}
