import 'dart:async';

import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/widgets.dart';

import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../generated/theme_variables.dart';
import '../theme/theme.dart';
import 'button.dart';
import 'icon_button.dart';

/// The tint a [Toast]'s glyph carries.
enum ToastTint with WidgetTint {
  primary,
  neutral,
  info,
  success,
  warning,
  danger,
}

/// A transient receipt for something that happened out of view.
///
/// One line, no wrapping — anything needing a second sentence belongs in an
/// [Callout] in the flow instead.
class Toast extends StatelessWidget {
  const Toast({
    super.key,
    required this.message,
    this.icon,
    this.tint = ToastTint.neutral,
    this.action,
    this.onDismiss,
  });

  final String message;
  final IconData? icon;
  final ToastTint tint;
  final Widget? action;
  final VoidCallback? onDismiss;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final ColorSwatch<int> ramp = switch (tint) {
      ToastTint.primary => vars.colorPrimary,
      ToastTint.neutral => vars.colorNeutral,
      ToastTint.info => vars.colorInfo,
      ToastTint.success => vars.colorSuccess,
      ToastTint.warning => vars.colorWarning,
      ToastTint.danger => vars.colorDanger,
    };

    return ConstrainedBox(
      constraints: BoxConstraints(
        // Sized to its content but never past the viewport, so the stack
        // needs no magic width and a long message truncates instead of
        // pushing wide.
        maxWidth: vars.toastMaxWidth,
        minHeight: vars.spacing9,
      ),
      child: IntrinsicWidth(
        child: Container(
          padding: EdgeInsetsDirectional.only(
            top: vars.spacing15,
            bottom: vars.spacing15,
            // Asymmetric on purpose: the text gets breathing room, the
            // dismiss button sits tight in the corner it lives in.
            start: vars.spacing3,
            end: vars.spacing15,
          ),
          decoration: BoxDecoration(
            color: vars.colorSurfaceOverlay,
            border: Border.all(
              color: vars.colorBorderStrong,
              width: context.hairlineWidth,
            ),
            borderRadius: BorderRadius.circular(vars.radiusBig),
            boxShadow: vars.shadowMd,
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            spacing: vars.spacing25,
            children: [
              if (icon != null)
                Icon(icon, size: vars.iconMedium, color: ramp[600]),
              Flexible(
                child: Padding(
                  padding: EdgeInsets.symmetric(vertical: vars.spacing05),
                  child: Text(
                    message,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    softWrap: false,
                    style: vars.labelMedium.copyWith(
                      height: 1.4,
                      color: vars.colorContent,
                    ),
                  ),
                ),
              ),
              ?action,
              if (onDismiss != null)
                IconButton(
                  icon: const Icon(FluentIcons.dismiss_12_regular),
                  onPressed: onDismiss,
                ),
            ],
          ),
        ),
      ),
    );
  }
}

/// What a toast offers to do about the thing it reports.
class ToastAction {
  const ToastAction({required this.label, required this.onPressed});

  final String label;
  final VoidCallback onPressed;
}

/// A toast to raise: what happened, and how it is shown.
///
/// Unless [timeout] says otherwise a danger toast stays until dismissed, one
/// with an action stays long enough to be acted on, and any other leaves
/// after five seconds.
class ToastOptions {
  const ToastOptions({
    required this.title,
    this.tint = ToastTint.neutral,
    this.icon,
    this.action,
    this.dismissable = true,
    this.timeout,
  });

  final String title;
  final ToastTint tint;
  final IconData? icon;
  final ToastAction? action;
  final bool dismissable;
  final Duration? timeout;

  Duration get _timeout =>
      timeout ??
      (tint == ToastTint.danger
          ? Duration.zero
          : action != null
          ? const Duration(seconds: 8)
          : const Duration(seconds: 5));
}

class _Raised {
  _Raised(this.id, this.options);

  final int id;
  final ToastOptions options;
  Timer? timer;
}

/// The toasts raised under a [ToastProvider], oldest first.
class Toaster extends ChangeNotifier {
  final List<_Raised> _toasts = [];
  int _next = 0;

  /// Raises a toast and returns its id, for [close].
  int add(ToastOptions options) {
    final _Raised raised = _Raised(_next++, options);
    final Duration timeout = options._timeout;
    if (timeout > Duration.zero) {
      raised.timer = Timer(timeout, () => close(raised.id));
    }
    _toasts.add(raised);
    notifyListeners();
    return raised.id;
  }

  /// Dismisses one toast, or every toast when [id] is null.
  void close([int? id]) {
    _toasts.removeWhere((raised) {
      final bool gone = id == null || raised.id == id;
      if (gone) raised.timer?.cancel();
      return gone;
    });
    notifyListeners();
  }

  @override
  void dispose() {
    for (final _Raised raised in _toasts) {
      raised.timer?.cancel();
    }
    super.dispose();
  }

  /// The toaster the nearest [ToastProvider] holds.
  static Toaster of(BuildContext context) {
    final _ToasterScope? scope = context
        .dependOnInheritedWidgetOfExactType<_ToasterScope>();
    assert(scope != null, 'Toaster.of used without a ToastProvider');
    return scope!.notifier!;
  }
}

class _ToasterScope extends InheritedNotifier<Toaster> {
  const _ToasterScope({required Toaster toaster, required super.child})
    : super(notifier: toaster);
}

/// Where toasts are raised from: anything under it can reach
/// [Toaster.of], and a [ToastViewport] under it shows what was raised.
class ToastProvider extends StatefulWidget {
  const ToastProvider({super.key, required this.child});

  final Widget child;

  @override
  State<ToastProvider> createState() => _ToastProviderState();
}

class _ToastProviderState extends State<ToastProvider> {
  final Toaster _toaster = Toaster();

  @override
  void dispose() {
    _toaster.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) =>
      _ToasterScope(toaster: _toaster, child: widget.child);
}

enum ToastPlacement { bottom, top }

/// The stack the provider's toasts are shown in, across the window's bottom
/// edge — or its top, where the newest sits nearest the edge.
class ToastViewport extends StatefulWidget {
  const ToastViewport({super.key, this.placement = ToastPlacement.bottom});

  final ToastPlacement placement;

  @override
  State<ToastViewport> createState() => _ToastViewportState();
}

class _ToastViewportState extends State<ToastViewport> {
  final OverlayPortalController _controller = OverlayPortalController();

  @override
  void initState() {
    super.initState();
    // Always in the overlay: an empty stack draws nothing and takes no
    // pointer, and a portal shown once never has to be toggled mid-build.
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _controller.show();
    });
  }

  @override
  Widget build(BuildContext context) {
    final Toaster toaster = Toaster.of(context);
    final List<_Raised> toasts = toaster._toasts;
    final bool top = widget.placement == ToastPlacement.top;

    return OverlayPortal(
      controller: _controller,
      overlayChildBuilder: (context) {
        final ThemeVariables vars = Theme.of(context).vars;
        final List<Widget> stack = [
          for (final _Raised raised in toasts)
            Toast(
              key: ValueKey(raised.id),
              message: raised.options.title,
              icon: raised.options.icon,
              tint: raised.options.tint,
              action: raised.options.action == null
                  ? null
                  : Button(
                      size: WidgetSize.small,
                      variant: ButtonVariant.plain,
                      tint: ButtonTint.values.byName(raised.options.tint.name),
                      onPressed: () {
                        raised.options.action!.onPressed();
                        toaster.close(raised.id);
                      },
                      child: Text(raised.options.action!.label),
                    ),
              onDismiss: raised.options.dismissable
                  ? () => toaster.close(raised.id)
                  : null,
            ),
        ];
        return Positioned(
          left: 0,
          right: 0,
          top: top ? 0 : null,
          bottom: top ? null : 0,
          child: Padding(
            padding: EdgeInsets.all(vars.spacing4),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing2,
              children: top ? stack.reversed.toList() : stack,
            ),
          ),
        );
      },
      child: const SizedBox.shrink(),
    );
  }
}
