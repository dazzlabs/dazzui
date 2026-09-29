import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/lib/themes.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import 'column.dart';
import 'theme_preview.dart';

/// The theme, and nothing else: which half of it, then which family.
class ThemePanel extends StatelessWidget {
  const ThemePanel({
    super.key,
    required this.theme,
    required this.onChanged,
    required this.width,
  });

  final ThemeChoice theme;
  final ValueChanged<ThemeChoice> onChanged;
  final double width;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    return ShellColumn(
      width: width,
      side: ShellSide.right,
      surface: vars.colorSurfaceColumn,
      header: Row(
        children: [
          Expanded(
            child: Text(
              'Theme',
              style: TextStyle(fontWeight: vars.labelStrong.fontWeight),
            ),
          ),
          Tooltip(
            label: theme.dark ? 'Switch to light' : 'Switch to dark',
            child: Toggle(
              pressed: theme.dark,
              onPressedChanged: (dark) => onChanged(theme.copyWith(dark: dark)),
              semanticsLabel: theme.dark ? 'Switch to light' : 'Switch to dark',
              child: Icon(
                theme.dark
                    ? FluentIcons.weather_moon_20_regular
                    : FluentIcons.weather_sunny_20_regular,
                size: 16,
              ),
            ),
          ),
        ],
      ),
      headerPadding: EdgeInsets.symmetric(horizontal: vars.spacing4),
      children: [
        for (final group in themeFamilyGroups)
          Padding(
            padding: EdgeInsets.symmetric(
              vertical: vars.spacing2,
              horizontal: vars.spacing4,
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              spacing: vars.spacing2,
              children: [
                SizedBox(
                  height: vars.spacing5,
                  child: Align(
                    alignment: AlignmentDirectional.centerStart,
                    child: DefaultTextStyle.merge(
                      style: const TextStyle(fontSize: 11),
                      child: SectionLabel(group.title),
                    ),
                  ),
                ),
                _Grid(
                  columns: 3,
                  gap: vars.spacing2,
                  children: [
                    for (final family in group.families)
                      _Tile(
                        choice: theme.copyWith(family: family.value),
                        title: family.title,
                        selected: family.value == theme.family,
                        onPressed: () =>
                            onChanged(theme.copyWith(family: family.value)),
                      ),
                  ],
                ),
              ],
            ),
          ),
      ],
    );
  }
}

class _Grid extends StatelessWidget {
  const _Grid({
    required this.columns,
    required this.gap,
    required this.children,
  });

  final int columns;
  final double gap;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      spacing: gap,
      children: [
        for (int start = 0; start < children.length; start += columns)
          Row(
            spacing: gap,
            children: [
              for (int i = start; i < start + columns; i++)
                Expanded(
                  child: i < children.length
                      ? children[i]
                      : const SizedBox.shrink(),
                ),
            ],
          ),
      ],
    );
  }
}

class _Tile extends StatelessWidget {
  const _Tile({
    required this.choice,
    required this.title,
    required this.selected,
    required this.onPressed,
  });

  final ThemeChoice choice;
  final String title;
  final bool selected;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    return OptionCard(
      title: title,
      selected: selected,
      onPressed: onPressed,
      semanticsLabel: title,
      padding: EdgeInsets.zero,
      titleContent: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          ThemePreview(theme: choice, height: vars.spacing10, framed: false),
          SizedBox(
            height: vars.spacing6,
            child: Padding(
              padding: EdgeInsets.symmetric(horizontal: vars.spacing15),
              child: Center(
                child: Text(
                  title,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  softWrap: false,
                  style: TextStyle(
                    fontSize: 11,
                    height: 1,
                    fontWeight: FontWeight.w500,
                    color: vars.colorContent,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
