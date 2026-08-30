import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class KbdPage extends StatelessWidget {
  const KbdPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Kbd'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: const Kbd('Ctrl'),
      ),
    );
  }
}

