import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../gallery.dart';

/// What the app says back: the banners that stay, in every tint, and the one
/// that carries actions.
class FeedbackPage extends StatelessWidget {
  const FeedbackPage({super.key});

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing6,
      children: [
        const PageTitle(
          'Feedback',
          'The tint carries the meaning; the shape carries how long it stays.',
        ),
        Specimen(
          'Alert banners',
          child: ConstrainedBox(
            // The SwiftUI playground caps its banners at 1.5 toast widths.
            constraints: BoxConstraints(maxWidth: vars.toastMaxWidth * 1.5),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              spacing: vars.spacing2,
              children: const [
                AlertBanner(
                  tint: AlertBannerTint.info,
                  icon: Icon(FluentIcons.info_16_regular),
                  title: Text('A new version is available'),
                  message: Text('Restart to finish updating to 2.1.0.'),
                ),
                AlertBanner(
                  tint: AlertBannerTint.success,
                  icon: Icon(FluentIcons.checkmark_circle_16_regular),
                  title: Text('Tokens rebuilt'),
                  message: Text('Three platform packages were refreshed.'),
                ),
                AlertBanner(
                  tint: AlertBannerTint.warning,
                  icon: Icon(FluentIcons.warning_16_regular),
                  title: Text('Ramp moves only part-way'),
                  message: Text(
                    'color.warning cannot move whole under bright-dark.',
                  ),
                ),
                AlertBanner(
                  tint: AlertBannerTint.danger,
                  icon: Icon(FluentIcons.dismiss_circle_16_regular),
                  title: Text('The build failed'),
                  message: Text('dazzui: 4 errors in sidebar.dart.'),
                ),
              ],
            ),
          ),
        ),
        Specimen(
          'With actions',
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: vars.toastMaxWidth * 1.5),
            child: AlertBanner(
              tint: AlertBannerTint.primary,
              icon: const Icon(FluentIcons.arrow_download_16_regular),
              title: const Text('Update ready'),
              message: const Text('2.1.0 has been downloaded.'),
              actions: [
                Button(
                  size: WidgetSize.small,
                  variant: ButtonVariant.tinted,
                  onPressed: () {},
                  child: const Text('Restart now'),
                ),
                Button(
                  size: WidgetSize.small,
                  variant: ButtonVariant.plain,
                  onPressed: () {},
                  child: const Text('Later'),
                ),
              ],
            ),
          ),
        ),
        SizedBox(height: vars.spacing8),
      ],
    );
  }
}
