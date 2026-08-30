import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class CardPage extends StatelessWidget {
  const CardPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Card'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Card(
          child: Container(
            width: 320,
            height: 240,
            padding: const EdgeInsets.all(16),
            child: const Center(
              child: Text('Card'),
            ),
          ),
        ),
      ),
    );
  }
}

