import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class DividerPage extends StatelessWidget {
  const DividerPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Divider'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: const SizedBox(
          width: 300,
          child: Divider(),
        ),
      ),
    );
  }
}

