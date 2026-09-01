import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/material.dart' as material;

import 'pages/controls_page.dart';
import 'pages/data_page.dart';
import 'pages/feedback_page.dart';
import 'pages/forms_page.dart';
import 'pages/settings_page.dart';
import 'pages/themes_page.dart';
import 'playground_theme.dart';

void main() {
  runApp(const PlaygroundApp());
}

class PlaygroundApp extends StatelessWidget {
  const PlaygroundApp({super.key});

  @override
  Widget build(BuildContext context) {
    // MaterialApp supplies the plumbing (Directionality, Overlay, text
    // editing); everything visible is drawn by the design's own Theme below.
    return material.MaterialApp(
      title: 'DazzUI Playground',
      debugShowCheckedModeBanner: false,
      home: const PlaygroundWindow(),
    );
  }
}

/// The playground itself: the component gallery on the left, the selected
/// page on the right, and the theme picker over both.
///
/// The theme is applied with the package's own [Theme] widget on one subtree
/// rather than to the process, which is the point worth demonstrating —
/// themes are per-subtree here, so the four can sit on screen at once (see
/// [ThemesPage]).
class PlaygroundWindow extends StatefulWidget {
  const PlaygroundWindow({super.key});

  @override
  State<PlaygroundWindow> createState() => _PlaygroundWindowState();
}

class _PlaygroundWindowState extends State<PlaygroundWindow> {
  PlaygroundPage _page = PlaygroundPage.controls;
  PlaygroundTheme _theme = PlaygroundTheme.studioLight;

  @override
  Widget build(BuildContext context) {
    final data = _theme.data;

    return Theme(
      data: data,
      child: material.Material(
        type: material.MaterialType.transparency,
        child: ColoredBox(
          color: data.vars.colorCanvas,
          child: Column(
            children: [
              _Toolbar(
                theme: _theme,
                onThemeChanged: (theme) => setState(() => _theme = theme),
              ),
              const Divider(height: 1),
              Expanded(
                child: Row(
                  children: [
                    _Navigator(
                      page: _page,
                      onPageChanged: (page) => setState(() => _page = page),
                    ),
                    const VerticalDivider(width: 1),
                    Expanded(child: _PageHost(page: _page)),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _Toolbar extends StatelessWidget {
  const _Toolbar({required this.theme, required this.onThemeChanged});

  final PlaygroundTheme theme;
  final ValueChanged<PlaygroundTheme> onThemeChanged;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Container(
      height: vars.frameTitlebarSize,
      color: vars.colorSurfaceMuted,
      padding: EdgeInsets.symmetric(horizontal: vars.spacing4),
      child: Row(
        spacing: vars.spacing3,
        children: [
          Text(
            'DazzUI',
            style: vars.titleSmall.copyWith(color: vars.colorContent),
          ),
          const Badge(
            size: WidgetSize.small,
            variant: BadgeVariant.tinted,
            child: Text('Flutter'),
          ),
          const Spacer(),
          SegmentedControl<PlaygroundTheme>(
            value: theme,
            onChanged: onThemeChanged,
            items: [
              for (final candidate in PlaygroundTheme.values)
                SegmentedItem(value: candidate, label: candidate.title),
            ],
          ),
        ],
      ),
    );
  }
}

class _Navigator extends StatelessWidget {
  const _Navigator({required this.page, required this.onPageChanged});

  final PlaygroundPage page;
  final ValueChanged<PlaygroundPage> onPageChanged;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Sidebar(
      header: Text(
        'Components',
        style: vars.titleSmall.copyWith(color: vars.colorContent),
      ),
      children: [
        SidebarGroup(
          label: 'Library',
          children: [
            for (final item in PlaygroundPage.values)
              NavItem(
                label: item.title,
                icon: item.icon,
                current: item == page,
                onPressed: () => onPageChanged(item),
              ),
          ],
        ),
      ],
    );
  }
}

class _PageHost extends StatelessWidget {
  const _PageHost({required this.page});

  final PlaygroundPage page;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return ColoredBox(
      color: vars.colorSurface,
      child: Align(
        alignment: Alignment.topLeft,
        child: SingleChildScrollView(
          padding: EdgeInsets.all(vars.spacing6),
          child: switch (page) {
            PlaygroundPage.controls => const ControlsPage(),
            PlaygroundPage.forms => const FormsPage(),
            PlaygroundPage.data => const DataPage(),
            PlaygroundPage.feedback => const FeedbackPage(),
            PlaygroundPage.settings => const SettingsPage(),
            PlaygroundPage.themes => const ThemesPage(),
          },
        ),
      ),
    );
  }
}

enum PlaygroundPage {
  controls,
  forms,
  data,
  feedback,
  settings,
  themes;

  String get title {
    return switch (this) {
      PlaygroundPage.controls => 'Controls',
      PlaygroundPage.forms => 'Forms',
      PlaygroundPage.data => 'Data',
      PlaygroundPage.feedback => 'Feedback',
      PlaygroundPage.settings => 'Settings',
      PlaygroundPage.themes => 'Themes',
    };
  }

  IconData get icon {
    return switch (this) {
      PlaygroundPage.controls => FluentIcons.square_multiple_16_regular,
      PlaygroundPage.forms => FluentIcons.checkbox_checked_16_regular,
      PlaygroundPage.data => FluentIcons.table_16_regular,
      PlaygroundPage.feedback => FluentIcons.alert_16_regular,
      PlaygroundPage.settings => FluentIcons.settings_16_regular,
      PlaygroundPage.themes => FluentIcons.paint_brush_16_regular,
    };
  }
}
