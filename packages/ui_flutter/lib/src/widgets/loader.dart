import 'package:flutter/widgets.dart';

import '../theme/theme.dart';
import 'oval_loader.dart';

enum LoaderVariant {
  oval,
}

class Loader extends StatelessWidget {
  const Loader({
    super.key,
    this.variant = LoaderVariant.oval,
    this.color,
  });

  final LoaderVariant variant;
  final Color? color;

  @override
  Widget build(BuildContext context) {
    final ThemeData themeData = Theme.of(context);
    Color resolvedColor = color ?? themeData.vars.colorPrimary;
    Size resolvedSize = const Size.square(16);
    return SizedBox(
      width: resolvedSize.width,
      height: resolvedSize.height,
      child: OvalLoader(
        size: resolvedSize.width,
        color: resolvedColor,
      ),
    );
  }
}
