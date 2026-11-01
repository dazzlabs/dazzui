import 'package:dazzui/dazzui.dart';
import 'package:dazzui_example/lib/themes.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import 'theme_preview.dart';

/// The theme, and nothing else: which half of it, then which family.
class ThemePanel extends StatelessWidget {
  const ThemePanel({
    super.key,
    required this.theme,
    required this.onChanged,
    required this.width,
    required this.collapsed,
    this.omarchy,
    this.followingOmarchy = false,
    this.onFollowOmarchy,
    this.menuPresentation = MenuPresentation.custom,
    this.onMenuPresentationChanged,
  });

  final ThemeChoice theme;
  final ValueChanged<ThemeChoice> onChanged;

  /// The width it has when open; put away, the panel keeps this one.
  final double width;

  /// Whether the panel is put away. The shell owns the state — its toolbar
  /// button — and the panel travels there.
  final bool collapsed;

  /// The machine's own Omarchy theme, offered first when there is one.
  final OmarchyPalette? omarchy;
  final bool followingOmarchy;
  final ValueChanged<bool>? onFollowOmarchy;
  final MenuPresentation menuPresentation;
  final ValueChanged<MenuPresentation>? onMenuPresentationChanged;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final BorderSide rule = BorderSide(
      color: vars.colorBorder,
      width: context.hairlineWidth,
    );
    return Aside(
      width: width,
      collapsed: collapsed,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Container(
            constraints: BoxConstraints(minHeight: vars.frameTitlebarSize),
            padding: EdgeInsets.symmetric(horizontal: vars.spacing4),
            alignment: AlignmentDirectional.centerStart,
            decoration: BoxDecoration(border: Border(bottom: rule)),
            child: Row(
              children: [
                Expanded(
                  child: Text(
                    'Theme',
                    style: TextStyle(
                      fontWeight: vars.labelStrong.fontWeight,
                    ),
                  ),
                ),
                Tooltip(
                  label: theme.dark ? 'Switch to light' : 'Switch to dark',
                  child: Toggle(
                    pressed: theme.dark,
                    onPressedChanged: (dark) =>
                        onChanged(theme.copyWith(dark: dark)),
                    semanticsLabel: theme.dark
                        ? 'Switch to light'
                        : 'Switch to dark',
                    child: Icon(
                      theme.dark
                          ? FluentIcons.weather_moon_20_regular
                          : FluentIcons.weather_sunny_20_regular,
                    ),
                  ),
                ),
              ],
            ),
          ),
          Expanded(
            child: SingleChildScrollView(
              padding: EdgeInsets.symmetric(vertical: vars.spacing2),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Padding(
                    padding: EdgeInsets.all(vars.spacing4),
                    child: Row(
                      children: [
                        const Expanded(child: Text('Native menus')),
                        Switch(
                          value: menuPresentation == MenuPresentation.native,
                          onChanged: onMenuPresentationChanged == null
                              ? null
                              : (value) => onMenuPresentationChanged!(
                                  value
                                      ? MenuPresentation.native
                                      : MenuPresentation.custom,
                                ),
                        ),
                      ],
                    ),
                  ),
                  if (omarchy case final omarchy?)
                    _Group(
                      title: 'This machine',
                      children: [
                        _Tile(
                          vars: _half(omarchy, theme.dark).themeVariables,
                          title: omarchy.title ?? 'Omarchy',
                          selected: followingOmarchy,
                          onPressed: () => onFollowOmarchy?.call(true),
                        ),
                      ],
                    ),
                  for (final group in themeFamilyGroups)
                    _Group(
                      title: group.title,
                      children: [
                        for (final family in group.families)
                          _Tile(
                            vars: themeVariablesOf(
                              theme.copyWith(family: family.value).id,
                            ),
                            title: family.title,
                            selected:
                                !followingOmarchy &&
                                family.value == theme.family,
                            onPressed: () {
                              onFollowOmarchy?.call(false);
                              onChanged(theme.copyWith(family: family.value));
                            },
                          ),
                      ],
                    ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

/// The Omarchy theme in the half asked for: as it ships, or its opposite.
OmarchyPalette _half(OmarchyPalette palette, bool dark) =>
    (palette.brightness == Brightness.dark) == dark
    ? palette
    : palette.opposite;

class _Group extends StatelessWidget {
  const _Group({required this.title, required this.children});

  final String title;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    return Padding(
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
                child: SectionLabel(title),
              ),
            ),
          ),
          _Grid(columns: 3, gap: vars.spacing2, children: children),
        ],
      ),
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
    required this.vars,
    required this.title,
    required this.selected,
    required this.onPressed,
  });

  final ThemeVariables vars;
  final String title;
  final bool selected;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables page = Theme.of(context).vars;
    return OptionCard(
      title: title,
      selected: selected,
      onPressed: onPressed,
      semanticsLabel: title,
      padding: EdgeInsets.zero,
      cornerRadius: 8,
      titleContent: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          ThemePreview(vars: vars, height: page.spacing10, framed: false),
          SizedBox(
            height: page.spacing6,
            child: Padding(
              padding: EdgeInsets.symmetric(horizontal: page.spacing15),
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
                    color: page.colorContent,
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
