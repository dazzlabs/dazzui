import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class BadgePage extends StatefulWidget {
  const BadgePage({super.key});

  @override
  State<BadgePage> createState() => _BadgePageState();
}

class _BadgePageState extends State<BadgePage> {
  String radioValue = '1';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Badge'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Row(
          spacing: 8,
          mainAxisSize: MainAxisSize.min,
          children: [
            const Badge(
              child: Text('Badge'),
            ),
            for (final variant in BadgeVariant.values)
              Badge(
                tint: BadgeTint.primary,
                variant: variant,
                child: const Text('Badge'),
              ),
          ],
        ),
      ),
    );
  }
}

class BadgeWithSizePage extends StatelessWidget {
  const BadgeWithSizePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/Badge - With Size'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final namedSize in NamedSize.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Badge(
                    size: WidgetSize(namedSize),
                    child: const Text('Badge'),
                  ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

