import 'package:dazzui/dazzui.dart';

/// The name, and which of its implementations this is. Every port has an
/// example of its own, and a window of one looks like a window of the next.
class Brand extends StatelessWidget {
  const Brand({super.key, required this.onPressed});

  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    return Pressable(
      onPressed: onPressed,
      semanticsLabel: 'DazzUI',
      builder: (context, states) => Row(
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing2,
        children: [
          Container(
            width: vars.spacing5,
            height: vars.spacing5,
            decoration: BoxDecoration(
              color: vars.colorPrimary[500],
              borderRadius: BorderRadius.circular(vars.radiusSmall),
            ),
          ),
          Text(
            'DazzUI',
            style: vars.titleSmall.copyWith(color: vars.colorContent),
          ),
          const Badge(
            size: WidgetSize.small,
            variant: BadgeVariant.tinted,
            child: Text('Flutter'),
          ),
        ],
      ),
    );
  }
}
