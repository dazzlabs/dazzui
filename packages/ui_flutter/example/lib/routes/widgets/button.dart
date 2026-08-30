import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' as material show Icons;
import 'package:flutter/material.dart' show Scaffold, AppBar;

class ButtonPage extends StatefulWidget {
  const ButtonPage({super.key});

  @override
  State<ButtonPage> createState() => _ButtonPageState();
}

class _ButtonPageState extends State<ButtonPage> {
  bool _switchValue1 = true;
  bool _switchValue2 = false;

  bool _checkboxValue1 = true;
  bool _checkboxValue2 = false;
  bool? _checkboxValue3;

  String _radioValue1 = '1';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Button'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        width: double.infinity,
        height: 400,
        child: Wrap(
          spacing: 8,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.center,
          children: [
            Button(
              onPressed: () {},
              child: const Text('Default'),
            ),
            Button(
              tint: ButtonTint.primary,
              variant: ButtonVariant.filled,
              onPressed: () {},
              child: const Text('Text Button'),
            ),
            Button(
              tint: ButtonTint.primary,
              variant: ButtonVariant.filled,
              onPressed: () {},
              child: Row(
                spacing: 4,
                mainAxisSize: MainAxisSize.min,
                children: const [
                  Icon(material.Icons.upload_file),
                  Text('Icon Button'),
                ],
              ),
            ),
            const Badge(
              variant: BadgeVariant.filled,
              child: Text('1'),
            ),
            const Badge(
              variant: BadgeVariant.outlined,
              child: Text('99+'),
            ),
            Switch(
              value: _switchValue1,
              onChanged: (newValue) => setState(() {
                _switchValue1 = newValue;
              }),
            ),
            Switch(
              value: _switchValue2,
              onChanged: (newValue) => setState(() {
                _switchValue2 = newValue;
              }),
            ),
            Checkbox(
              value: _checkboxValue1,
              onChanged: (newValue) => setState(() {
                _checkboxValue1 = newValue!;
              }),
            ),
            Checkbox(
              value: _checkboxValue2,
              onChanged: (newValue) => setState(() {
                _checkboxValue2 = newValue!;
              }),
            ),
            Checkbox(
              value: _checkboxValue3,
              tristate: true,
              onChanged: (newValue) => setState(() {
                _checkboxValue3 = newValue!;
              }),
            ),
            Radio<String>(
              value: '1',
              groupValue: _radioValue1,
              onChanged: (newValue) => setState(() {
                _radioValue1 = newValue!;
              }),
            ),
            Radio<String>(
              value: '2',
              groupValue: _radioValue1,
              onChanged: (newValue) => setState(() {
                _radioValue1 = newValue!;
              }),
            ),
          ],
        ),
      ),
    );
  }
}

class ButtonWithTintPage extends StatelessWidget {
  const ButtonWithTintPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Button - With Tint'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final tint in ButtonTint.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final variant in ButtonVariant.values)
                    Button(
                      tint: tint,
                      variant: variant,
                      onPressed: () {},
                      child: const Text('Text Button'),
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

class ButtonWithVariantPage extends StatelessWidget {
  const ButtonWithVariantPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Button - With Variant'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final variant in ButtonVariant.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final tint in ButtonTint.values)
                    Button(
                      tint: tint,
                      variant: variant,
                      onPressed: () {},
                      child: const Text('Text Button'),
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

class ButtonWithSizePage extends StatelessWidget {
  const ButtonWithSizePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Button - With Size'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final variant in ButtonVariant.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final namedSize in NamedSize.values)
                    Button(
                      size: WidgetSize(namedSize),
                      tint: ButtonTint.primary,
                      variant: variant,
                      onPressed: () {},
                      child: const Text('Text Button'),
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

