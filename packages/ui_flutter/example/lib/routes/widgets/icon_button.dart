import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' as material show Icons;
import 'package:flutter/material.dart' show Scaffold, AppBar;

class IconButtonPage extends StatelessWidget {
  const IconButtonPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/IconButton'),
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
            IconButton(
              onPressed: () {},
              icon: material.Icons.star,
            ),
          ],
        ),
      ),
    );
  }
}

class IconButtonWithTintPage extends StatelessWidget {
  const IconButtonWithTintPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/IconButton - With Tint'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final tint in IconButtonTint.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final variant in IconButtonVariant.values)
                    IconButton(
                      icon: material.Icons.star,
                      tint: tint,
                      variant: variant,
                      onPressed: () {},
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

class IconButtonWithVariantPage extends StatelessWidget {
  const IconButtonWithVariantPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/IconButton - With Variant'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final variant in IconButtonVariant.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final tint in IconButtonTint.values)
                    IconButton(
                      icon: material.Icons.star,
                      tint: tint,
                      variant: variant,
                      onPressed: () {},
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

class IconButtonWithSizePage extends StatelessWidget {
  const IconButtonWithSizePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/IconButton - With Size'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final variant in IconButtonVariant.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final namedSize in NamedSize.values)
                    IconButton(
                      icon: material.Icons.star,
                      size: WidgetSize(namedSize),
                      tint: IconButtonTint.primary,
                      variant: variant,
                      onPressed: () {},
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

