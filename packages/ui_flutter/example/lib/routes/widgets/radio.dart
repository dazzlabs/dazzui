import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class RadioPage extends StatefulWidget {
  const RadioPage({super.key});

  @override
  State<RadioPage> createState() => _RadioPageState();
}

class _RadioPageState extends State<RadioPage> {
  String radioValue = '1';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Radio'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Radio<String>(
              value: '1',
              groupValue: radioValue,
              onChanged: (newValue) => setState(() {
                radioValue = newValue!;
              }),
            ),
            Radio<String>(
              value: '2',
              groupValue: radioValue,
              onChanged: (newValue) => setState(() {
                radioValue = newValue!;
              }),
            ),
            Radio<String>(
              value: '1',
              groupValue: radioValue,
              onChanged: (newValue) => setState(() {
                radioValue = newValue!;
              }),
              useCheckmarkStyle: true,
            ),
            Radio<String>(
              value: '2',
              groupValue: radioValue,
              onChanged: (newValue) => setState(() {
                radioValue = newValue!;
              }),
              useCheckmarkStyle: true,
            ),
          ],
        ),
      ),
    );
  }
}

