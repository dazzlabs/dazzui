import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' as material;

import 'router_config.dart';
import 'widgets/theme_customizer.dart';

void main() {
  runApp(const PlaygroundApp());
}

class PlaygroundApp extends StatefulWidget {
  const PlaygroundApp({super.key});

  @override
  State<PlaygroundApp> createState() => _PlaygroundAppState();
}

class _PlaygroundAppState extends State<PlaygroundApp> {
  String _themeId = 'light';

  Widget _buildWithThemeCustomizer(BuildContext context, Widget child) {
    return Stack(
      children: [
        child,
        Positioned(
          bottom: 16,
          right: 16,
          child: ThemeCustomizer(
            themeId: _themeId,
            onThemeIdChanged: (value) {
              if (value == 'light') {
                _themeId = 'light';
              } else {
                _themeId = 'dark';
              }
              setState(() {});
            },
          ),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return material.MaterialApp.router(
      title: 'DazzUI Playground',
      theme: material.ThemeData.light().copyWith(
        extensions: [ThemeData.light()],
      ),
      darkTheme: material.ThemeData.dark().copyWith(
        extensions: [ThemeData.dark()],
      ),
      themeMode: _themeId == 'light'
          ? material.ThemeMode.light
          : material.ThemeMode.dark,
      builder: (context, child) => _buildWithThemeCustomizer(context, child!),
      routerConfig: routerConfig,
    );
  }
}
