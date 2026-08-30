import 'package:dazzui/dazzui.dart';

class ThemeCustomizer extends StatelessWidget {
  const ThemeCustomizer({
    super.key,
    required this.themeId,
    required this.onThemeIdChanged,
  });

  final String themeId;
  final ValueChanged<String> onThemeIdChanged;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Container(
      width: 200,
      height: 140,
      padding: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        color: theme.vars.colorSurfaceMuted,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: theme.vars.colorBorder,
          width: 1,
        ),
      ),
      child: Column(
        children: [
          Text(
            'Theme Customizer',
            style: theme.vars.bodySmall.copyWith(
              color: theme.vars.colorContent,
              decoration: TextDecoration.none,
            ),
          ),
          const SizedBox(height: 8),
          for (var themeId in ['light', 'dark']) ...[
            Row(
              children: [
                Text(
                  themeId,
                  style: theme.vars.bodySmall.copyWith(
                    color: theme.vars.colorContent,
                    decoration: TextDecoration.none,
                  ),
                ),
                Radio(
                  value: themeId,
                  groupValue: this.themeId,
                  onChanged: (value) => onThemeIdChanged(value ?? ''),
                ),
              ],
            ),
          ],
        ],
      ),
    );
  }
}
