import 'package:dazzui/dazzui.dart';

class PlayerIcon extends StatelessWidget {
  const PlayerIcon({
    super.key,
    required this.label,
    required this.icon,
    required this.onPressed,
    this.active = false,
    this.filled = false,
    this.size = WidgetSize.small,
  });

  final String label;
  final IconData icon;
  final VoidCallback? onPressed;
  final bool active;
  final bool filled;
  final WidgetSize size;

  @override
  Widget build(BuildContext context) => Tooltip(
    label: label,
    child: IconButton(
      semanticsLabel: label,
      icon: Icon(icon),
      onPressed: onPressed,
      size: size,
      iconSize: filled ? 22 : 19,
      tint: active || filled ? IconButtonTint.primary : IconButtonTint.neutral,
      variant: filled
          ? IconButtonVariant.filled
          : active
          ? IconButtonVariant.tinted
          : IconButtonVariant.plain,
    ),
  );
}

class MutedText extends StatelessWidget {
  const MutedText(this.text, {super.key, this.size = 12});
  final String text;
  final double size;

  @override
  Widget build(BuildContext context) => Text(
    text,
    maxLines: 1,
    overflow: TextOverflow.ellipsis,
    style: TextStyle(
      color: Theme.of(context).vars.colorContentSecondary,
      fontSize: size,
    ),
  );
}

class Panel extends StatelessWidget {
  const Panel({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) => ClipRRect(
    borderRadius: BorderRadius.circular(Theme.of(context).vars.radiusLarge),
    child: ColoredBox(color: Theme.of(context).vars.colorSurface, child: child),
  );
}
