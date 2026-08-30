import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/material.dart' show Scaffold, AppBar;

class AlertBannerPage extends StatelessWidget {
  const AlertBannerPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/AlertBanner'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: SizedBox(
          width: 640,
          child: Column(
            spacing: 20,
            mainAxisSize: MainAxisSize.min,
            children: const [
              AlertBanner(
                tint: AlertBannerTint.success,
                variant: AlertBannerVariant.filled,
                icon: Icon(FluentIcons.info_20_regular),
                title: Text('Order completed'),
                message: Text(
                  'Lorem ipsum dolor sit amet consectetur adipisicing elit. '
                  'Aliquid pariatur, ipsum similique veniam.',
                ),
              ),
              AlertBanner(
                tint: AlertBannerTint.success,
                variant: AlertBannerVariant.tinted,
                icon: Icon(FluentIcons.info_20_regular),
                title: Text('Title'),
                message: Text(
                  'A new software update is available. '
                  'See what\'s new in version 2.0.4.',
                ),
              ),
              AlertBanner(
                tint: AlertBannerTint.success,
                variant: AlertBannerVariant.outlined,
                icon: Icon(FluentIcons.info_20_regular),
                title: Text('There were 2 errors with your submission'),
                message: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text('Your password must be at least 8 characters'),
                    Text(
                      'Your password must include at least '
                      'one pro wrestling finishing move',
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class AlertBannerWithVariantPage extends StatelessWidget {
  const AlertBannerWithVariantPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Widgets/AlertBanner - With Variant'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16),
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final tint in AlertBannerTint.values)
              Row(
                spacing: 16,
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final variant in AlertBannerVariant.values)
                    SizedBox(
                      width: 260,
                      child: AlertBanner(
                        tint: tint,
                        variant: variant,
                        icon: const Icon(FluentIcons.info_12_regular),
                        title: const Text('New software update'),
                        message: const Text(
                          'A new software update is available. '
                          'See what\'s new in version 2.0.4.',
                        ),
                      ),
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}

