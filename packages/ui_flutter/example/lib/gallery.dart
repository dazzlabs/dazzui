import 'package:dazzui/dazzui.dart';

/// The playground's own presentation — a heading and a frame around a row of
/// specimens. It draws with the design's tokens like everything else, so a
/// theme switch carries the page furniture with it.
class Specimen extends StatelessWidget {
  const Specimen(
    this.title, {
    super.key,
    this.note,
    required this.child,
  });

  final String title;
  final String? note;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing2,
      children: [
        Row(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.baseline,
          textBaseline: TextBaseline.alphabetic,
          spacing: vars.spacing2,
          children: [
            Text(
              title,
              style: vars.labelLarge.copyWith(color: vars.colorContent),
            ),
            if (note != null)
              Flexible(
                child: Text(
                  note!,
                  style: vars.captionSmall.copyWith(
                    color: vars.colorContentSubtle,
                  ),
                ),
              ),
          ],
        ),
        child,
      ],
    );
  }
}

/// A page heading: what this page is, and the one sentence saying why the
/// components on it belong together.
class PageTitle extends StatelessWidget {
  const PageTitle(this.title, this.subtitle, {super.key});

  final String title;
  final String subtitle;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      spacing: vars.spacing1,
      children: [
        Text(
          title,
          style: vars.headlineSmall.copyWith(color: vars.colorContent),
        ),
        Text(
          subtitle,
          style: vars.bodyMedium.copyWith(color: vars.colorContentMuted),
        ),
      ],
    );
  }
}

/// A row of specimens that wraps rather than clipping, which is what a
/// gallery of variants wants at any window width.
class RowWrap extends StatelessWidget {
  const RowWrap({super.key, this.spacing, required this.children});

  final double? spacing;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    final vars = Theme.of(context).vars;
    final gap = spacing ?? vars.spacing2;

    return Wrap(
      spacing: gap,
      runSpacing: gap,
      crossAxisAlignment: WrapCrossAlignment.center,
      children: children,
    );
  }
}
