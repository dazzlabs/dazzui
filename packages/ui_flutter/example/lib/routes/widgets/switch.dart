import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class SwitchPage extends StatefulWidget {
  const SwitchPage({super.key});

  @override
  State<SwitchPage> createState() => _SwitchPageState();
}

class _SwitchPageState extends State<SwitchPage> {
  bool value = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Switch'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Switch(
          value: value,
          onChanged: (newValue) => setState(() {
            value = newValue;
          }),
        ),
      ),
    );
  }
}

