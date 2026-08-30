import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class CheckboxPage extends StatefulWidget {
  const CheckboxPage({super.key});

  @override
  State<CheckboxPage> createState() => _CheckboxPageState();
}

class _CheckboxPageState extends State<CheckboxPage> {
  bool value = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Checkbox'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Checkbox(
          value: value,
          onChanged: (newValue) => setState(() {
            value = newValue!;
          }),
        ),
      ),
    );
  }
}

