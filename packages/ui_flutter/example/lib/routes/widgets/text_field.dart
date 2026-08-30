import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class TextFieldPage extends StatelessWidget {
  const TextFieldPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/TextField'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: const TextField(
          placeholder: 'Type something...',
        ),
      ),
    );
  }
}

