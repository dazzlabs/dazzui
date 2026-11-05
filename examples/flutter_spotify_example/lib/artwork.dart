import 'dart:math' as math;

import 'package:dazzui/dazzui.dart';

import 'catalog.dart';

/// Original vector sleeves: no network images, image assets or font downloads.
class Artwork extends StatelessWidget {
  const Artwork({
    super.key,
    required this.index,
    this.size,
    this.title,
    this.radius,
  });

  final int index;
  final double? size;
  final String? title;
  final double? radius;

  @override
  Widget build(BuildContext context) => ClipRRect(
    borderRadius: BorderRadius.circular(
      radius ?? Theme.of(context).vars.radiusLarge,
    ),
    child: SizedBox.square(
      dimension: size,
      child: AspectRatio(
        aspectRatio: 1,
        child: LayoutBuilder(
          builder: (context, constraints) => CustomPaint(
            painter: _SleevePainter(index),
            child: title == null || constraints.maxWidth < 100
                ? null
                : Padding(
                    padding: EdgeInsets.all(constraints.maxWidth * 0.09),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'DAZZUI  /  SESSIONS',
                          style: TextStyle(
                            fontSize: constraints.maxWidth * 0.037,
                            letterSpacing: 1.6,
                            fontWeight: FontWeight.w600,
                            color: const Color(0xFF26372E),
                          ),
                        ),
                        const Spacer(),
                        Text(
                          title!,
                          maxLines: 2,
                          style: TextStyle(
                            fontSize: constraints.maxWidth * 0.16,
                            height: 0.98,
                            letterSpacing: -1.5,
                            fontWeight: FontWeight.w800,
                            color: const Color(0xFFFFF7E8),
                          ),
                        ),
                        SizedBox(height: constraints.maxWidth * 0.035),
                        Text(
                          'SOUNDTRACK TO SLOWER DAYS',
                          style: TextStyle(
                            fontSize: constraints.maxWidth * 0.026,
                            letterSpacing: 1.3,
                            color: const Color(0xFFEAE9D4),
                          ),
                        ),
                      ],
                    ),
                  ),
          ),
        ),
      ),
    ),
  );
}

class _SleevePainter extends CustomPainter {
  const _SleevePainter(this.index);
  final int index;

  @override
  void paint(Canvas canvas, Size size) {
    final palette = artworkPalettes[index % artworkPalettes.length];
    final side = size.width;
    canvas.drawRect(
      Offset.zero & size,
      Paint()
        ..shader = LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: [palette[0], palette[1]],
        ).createShader(Offset.zero & size),
    );
    canvas.drawCircle(
      Offset(side * (0.67 - index % 3 * 0.13), side * 0.35),
      side * (0.17 + index % 2 * 0.035),
      Paint()..color = palette[1],
    );
    // Overlapping paper-cut hills, with a different horizon on each sleeve.
    for (var layer = 0; layer < 4; layer++) {
      final path = Path()..moveTo(0, side);
      for (var x = 0; x <= 100; x++) {
        final fraction = x / 100;
        final y =
            0.49 +
            layer * 0.125 +
            math.sin(fraction * 4.6 + layer * 1.3 + index * 0.7) * 0.105 +
            math.cos(fraction * 8 + index) * 0.025;
        path.lineTo(fraction * side, y * side);
      }
      path
        ..lineTo(side, side)
        ..close();
      canvas.drawPath(
        path,
        Paint()..color = Color.lerp(palette[2], palette[3], layer / 3)!,
      );
    }
    final random = math.Random(index + 84);
    final grain = Paint()..color = const Color(0x14FFF7E8);
    for (var dot = 0; dot < 1800; dot++) {
      canvas.drawCircle(
        Offset(random.nextDouble() * side, random.nextDouble() * side),
        side * 0.0014,
        grain,
      );
    }
  }

  @override
  bool shouldRepaint(_SleevePainter oldDelegate) => index != oldDelegate.index;
}

class SpotifyMark extends StatelessWidget {
  const SpotifyMark({super.key, this.size = 30});
  final double size;

  @override
  Widget build(BuildContext context) => CustomPaint(
    size: Size.square(size),
    painter: _MarkPainter(
      Theme.of(context).vars.colorContent,
      Theme.of(context).vars.colorSurface,
    ),
  );
}

class _MarkPainter extends CustomPainter {
  const _MarkPainter(this.color, this.ink);
  final Color color;
  final Color ink;

  @override
  void paint(Canvas canvas, Size size) {
    final s = size.width;
    canvas.drawCircle(Offset(s / 2, s / 2), s / 2, Paint()..color = color);
    final ink = Paint()
      ..color = this.ink
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round;
    for (var i = 0; i < 3; i++) {
      ink.strokeWidth = s * (0.078 - i * 0.009);
      final y = s * (0.36 + i * 0.16);
      canvas.drawPath(
        Path()
          ..moveTo(s * (0.23 + i * 0.02), y)
          ..quadraticBezierTo(
            s * 0.51,
            y - s * 0.10,
            s * (0.79 - i * 0.03),
            y + s * 0.04,
          ),
        ink,
      );
    }
  }

  @override
  bool shouldRepaint(_MarkPainter oldDelegate) =>
      oldDelegate.color != color || oldDelegate.ink != ink;
}
