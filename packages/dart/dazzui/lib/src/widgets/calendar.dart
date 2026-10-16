import 'dart:ui' show PathMetric;

import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../foundation/widget_variant.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';
import 'icon_button.dart';
import 'pressable.dart';

/// The ramp a [Calendar]'s selection resolves against.
enum CalendarTint with WidgetTint {
  primary,
  neutral,
  info,
  success,
  warning,
  danger,
}

/// The recipes a day wears: `tinted` inside a range, `filled` when picked.
/// At rest a day is drawn as a row rather than through a recipe, which is
/// why `plain` is not here.
enum _DayVariant with WidgetVariant { filled, tinted }

/// A span of days. [end] is null while the first day has been picked and the
/// second has not — a range mid-pick is still a value, because the picker
/// around the calendar wants to show the start while the reader chooses the
/// end.
@immutable
class DateRange {
  const DateRange({this.start, this.end});

  const DateRange.empty() : start = null, end = null;

  final DateTime? start;
  final DateTime? end;

  @override
  bool operator ==(Object other) =>
      other is DateRange &&
      sameDay(other.start, start) &&
      sameDay(other.end, end);

  @override
  int get hashCode =>
      Object.hash(start?.day, start?.month, end?.day, end?.month);
}

/// Whether two dates fall on one day, ignoring the time of day.
bool sameDay(DateTime? a, DateTime? b) =>
    a != null &&
    b != null &&
    a.year == b.year &&
    a.month == b.month &&
    a.day == b.day;

/// The date at local midnight.
DateTime startOfDay(DateTime date) => DateTime(date.year, date.month, date.day);

/// The first of the month.
DateTime startOfMonth(DateTime date) => DateTime(date.year, date.month);

/// [days] later, at the same time of day.
DateTime addDays(DateTime date, int days) => DateTime(
  date.year,
  date.month,
  date.day + days,
  date.hour,
  date.minute,
);

/// The first of the month [months] away.
DateTime addMonths(DateTime date, int months) =>
    DateTime(date.year, date.month + months);

/// Day order, ignoring the time of day: negative when [a] is earlier.
int compareDay(DateTime a, DateTime b) =>
    startOfDay(a).compareTo(startOfDay(b));

/// [day]'s date carrying [time]'s hours and minutes.
DateTime withTime(DateTime day, DateTime time) =>
    DateTime(day.year, day.month, day.day, time.hour, time.minute);

/// The words a calendar is labelled with. English unless the product says
/// otherwise; the kit does not carry a locale of its own.
@immutable
class CalendarStrings {
  const CalendarStrings({
    this.monthTitle = _monthTitle,
    this.weekday = _weekday,
    this.dayLabel = _dayLabel,
    this.previousMonth = 'Previous month',
    this.nextMonth = 'Next month',
  });

  /// The month's name over its grid, e.g. `October 2026`.
  final String Function(DateTime month) monthTitle;

  /// The narrow name over a column, from `DateTime.weekday` (1 is Monday).
  final String Function(int weekday) weekday;

  /// A day's accessible name, e.g. `Friday, October 16, 2026`.
  final String Function(DateTime day) dayLabel;

  final String previousMonth;
  final String nextMonth;

  static const List<String> _months = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ];
  static const List<String> _days = [
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday',
    'Sunday',
  ];

  static String _monthTitle(DateTime month) =>
      '${_months[month.month - 1]} ${month.year}';
  static String _weekday(int weekday) => _days[weekday - 1][0];
  static String _dayLabel(DateTime day) =>
      '${_days[day.weekday - 1]}, ${_months[day.month - 1]} ${day.day}, '
      '${day.year}';
}

/// A month grid, in one of two modes: one day, or a span of them.
///
/// Flutter has no calendar widget this kit can wear — material's is
/// material's — so this is one of the few widgets that writes its behaviour
/// out, and the reason is written here: a grid's keyboard is not a list's.
/// The arrow keys move by a day and by a week, Page Up and Page Down by a
/// month, Home and End to the ends of the week, and the whole grid is one
/// tab stop that lands on the day last reached. Picking is a press, Enter
/// or Space; in range mode the first press sets the start and the second
/// the end, in either order, and while the end is pending the day under the
/// pointer or the keyboard previews the span.
///
/// Every day is a control, so it wears the shared recipes: `tinted` inside
/// a range — the system's one drawing for a selection — and `filled` for the
/// day or the two days actually picked. At rest it is drawn as a row, in
/// ink with the neutral hover wash, the way a menu row is.
class Calendar extends StatefulWidget {
  /// One day.
  const Calendar({
    super.key,
    this.value,
    this.initialValue,
    this.onChanged,
    this.size = WidgetSize.medium,
    this.months = 1,
    this.month,
    this.initialMonth,
    this.onMonthChanged,
    this.min,
    this.max,
    this.weekStartsOn = DateTime.monday,
    this.strings = const CalendarStrings(),
    this.today,
    this.navigation = true,
    this.tint = CalendarTint.primary,
    this.semanticsLabel,
  }) : range = null,
       initialRange = null,
       onRangeChanged = null,
       mode = CalendarMode.single;

  /// A span of days.
  const Calendar.range({
    super.key,
    this.range,
    this.initialRange,
    this.onRangeChanged,
    this.size = WidgetSize.medium,
    this.months = 1,
    this.month,
    this.initialMonth,
    this.onMonthChanged,
    this.min,
    this.max,
    this.weekStartsOn = DateTime.monday,
    this.strings = const CalendarStrings(),
    this.today,
    this.navigation = true,
    this.tint = CalendarTint.primary,
    this.semanticsLabel,
  }) : value = null,
       initialValue = null,
       onChanged = null,
       mode = CalendarMode.range;

  final CalendarMode mode;

  /// The picked day, driven from outside. Left null the widget holds its own.
  final DateTime? value;
  final DateTime? initialValue;
  final ValueChanged<DateTime>? onChanged;

  /// The picked span, driven from outside. Left null the widget holds its own.
  final DateRange? range;
  final DateRange? initialRange;
  final ValueChanged<DateRange>? onRangeChanged;

  /// The regular control height per day, or the small step for a panel that
  /// holds two months. Any other step is read as medium.
  final WidgetSize size;

  /// How many months stand side by side: one or two.
  final int months;

  /// The month shown — the first of two. Any day in it will do.
  final DateTime? month;
  final DateTime? initialMonth;
  final ValueChanged<DateTime>? onMonthChanged;

  /// Days before [min] and after [max] are drawn but cannot be picked.
  final DateTime? min;
  final DateTime? max;

  /// `DateTime.monday` … `DateTime.sunday`. Monday by default, as ISO counts
  /// the week.
  final int weekStartsOn;

  final CalendarStrings strings;

  /// What the grid counts as today. The clock's, unless a test says
  /// otherwise.
  final DateTime? today;

  /// Whether the head carries the chevrons that move the month. Off for a
  /// calendar whose host moves it, which then shows the month's name alone.
  final bool navigation;

  final CalendarTint tint;

  final String? semanticsLabel;

  @override
  State<Calendar> createState() => _CalendarState();
}

enum CalendarMode { single, range }

class _CalendarState extends State<Calendar> {
  DateTime? _ownValue;
  DateRange _ownRange = const DateRange.empty();
  late DateTime _ownMonth;
  late DateTime _focused;
  DateTime? _hovered;
  final Map<int, FocusNode> _nodes = <int, FocusNode>{};
  int? _focusPending;

  DateTime get _today => startOfDay(widget.today ?? DateTime.now());

  DateRange get _range {
    if (widget.mode == CalendarMode.range) {
      return widget.range ?? _ownRange;
    }
    final DateTime? day = widget.value ?? _ownValue;
    return DateRange(start: day, end: day);
  }

  bool get _pending =>
      widget.mode == CalendarMode.range &&
      _range.start != null &&
      _range.end == null;

  DateTime get _firstMonth =>
      widget.month != null ? startOfMonth(widget.month!) : _ownMonth;

  @override
  void initState() {
    super.initState();
    _ownValue = widget.initialValue;
    _ownRange = widget.initialRange ?? const DateRange.empty();
    final DateTime? start = _range.start;
    _ownMonth = startOfMonth(widget.initialMonth ?? start ?? _today);
    _focused = start ?? _today;
  }

  @override
  void dispose() {
    for (final FocusNode node in _nodes.values) {
      node.dispose();
    }
    super.dispose();
  }

  static int _key(DateTime day) =>
      startOfDay(day).millisecondsSinceEpoch ~/ Duration.millisecondsPerDay;

  FocusNode _node(DateTime day) =>
      _nodes.putIfAbsent(_key(day), () => FocusNode(debugLabel: 'day'));

  void _setMonth(DateTime next) {
    final DateTime first = startOfMonth(next);
    if (widget.month == null) setState(() => _ownMonth = first);
    widget.onMonthChanged?.call(first);
  }

  bool _visible(DateTime day) {
    for (int i = 0; i < widget.months; i++) {
      final DateTime month = addMonths(_firstMonth, i);
      if (month.year == day.year && month.month == day.month) return true;
    }
    return false;
  }

  bool _disabled(DateTime day) =>
      (widget.min != null && compareDay(day, widget.min!) < 0) ||
      (widget.max != null && compareDay(day, widget.max!) > 0);

  void _pick(DateTime day) {
    if (_disabled(day)) return;
    if (widget.mode == CalendarMode.range) {
      final DateRange current = _range;
      final DateRange next;
      if (current.start == null || current.end != null) {
        next = DateRange(start: day);
      } else if (compareDay(day, current.start!) < 0) {
        next = DateRange(start: day, end: current.start);
      } else {
        next = DateRange(start: current.start, end: day);
      }
      if (widget.range == null) setState(() => _ownRange = next);
      widget.onRangeChanged?.call(next);
    } else {
      if (widget.value == null) setState(() => _ownValue = day);
      widget.onChanged?.call(day);
    }
  }

  void _moveFocus(DateTime day) {
    setState(() {
      _focused = day;
      _focusPending = _key(day);
    });
    if (!_visible(day)) {
      // Reached past the shown months: slide them so the day is in the first
      // one going back, or the last one going forward.
      final bool forward = compareDay(day, _firstMonth) > 0;
      _setMonth(forward ? addMonths(day, 1 - widget.months) : day);
    }
  }

  KeyEventResult _onKey(FocusNode node, KeyEvent event) {
    if (event is! KeyDownEvent && event is! KeyRepeatEvent) {
      return KeyEventResult.ignored;
    }
    final DateTime day = _focused;
    final int offset = (day.weekday - widget.weekStartsOn + 7) % 7;
    final LogicalKeyboardKey key = event.logicalKey;
    final bool shift = HardwareKeyboard.instance.isShiftPressed;
    final DateTime? next;
    if (key == LogicalKeyboardKey.arrowLeft) {
      next = addDays(day, -1);
    } else if (key == LogicalKeyboardKey.arrowRight) {
      next = addDays(day, 1);
    } else if (key == LogicalKeyboardKey.arrowUp) {
      next = addDays(day, -7);
    } else if (key == LogicalKeyboardKey.arrowDown) {
      next = addDays(day, 7);
    } else if (key == LogicalKeyboardKey.pageUp) {
      next = _sameDayIn(addMonths(day, shift ? -12 : -1), day.day);
    } else if (key == LogicalKeyboardKey.pageDown) {
      next = _sameDayIn(addMonths(day, shift ? 12 : 1), day.day);
    } else if (key == LogicalKeyboardKey.home) {
      next = addDays(day, -offset);
    } else if (key == LogicalKeyboardKey.end) {
      next = addDays(day, 6 - offset);
    } else {
      return KeyEventResult.ignored;
    }
    _moveFocus(next);
    return KeyEventResult.handled;
  }

  static DateTime _sameDayIn(DateTime month, int day) {
    final int last = DateTime(month.year, month.month + 1, 0).day;
    return DateTime(month.year, month.month, day < last ? day : last);
  }

  /// The six weeks a month grid draws, starting on the week day the widget
  /// starts on — always forty-two days, so the grid never changes height.
  List<DateTime> _grid(DateTime month) {
    final DateTime first = startOfMonth(month);
    final int offset = (first.weekday - widget.weekStartsOn + 7) % 7;
    final DateTime start = addDays(first, -offset);
    return List<DateTime>.generate(42, (int i) => addDays(start, i));
  }

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final double cell = switch (widget.size.namedSize) {
      NamedSize.small => vars.calendarSmallCellSize,
      _ => vars.calendarMediumCellSize,
    };
    final ColorSwatch<int> seed = vars.controlColor.tinted<ColorSwatch<int>>(
      widget.tint,
    );
    final DateTime today = _today;
    final DateRange range = _range;
    final bool pending = _pending;

    // What the span to draw is: the two picked ends, or the one picked end
    // and the day being previewed against it.
    final DateTime? preview = pending ? (_hovered ?? _focused) : null;
    DateTime? lo;
    DateTime? hi;
    if (widget.mode == CalendarMode.range) {
      final DateTime? a = range.start;
      final DateTime? b = pending ? preview : range.end;
      if (a != null && b != null) {
        lo = compareDay(a, b) <= 0 ? a : b;
        hi = compareDay(a, b) <= 0 ? b : a;
      }
    }

    // The one tab stop is the focused day, if it is shown; otherwise today,
    // or failing that the first day shown.
    final DateTime stop = _visible(_focused)
        ? _focused
        : (_visible(today) ? today : _grid(_firstMonth).first);

    if (_focusPending != null) {
      final int key = _focusPending!;
      _focusPending = null;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _nodes[key]?.requestFocus();
      });
    }

    final List<Widget> months = <Widget>[
      for (int index = 0; index < widget.months; index++)
        _month(
          context,
          vars,
          index: index,
          cell: cell,
          seed: seed,
          today: today,
          range: range,
          pending: pending,
          preview: preview,
          lo: lo,
          hi: hi,
          stop: stop,
        ),
    ];

    return Semantics(
      container: true,
      label: widget.semanticsLabel,
      child: Focus(
        onKeyEvent: _onKey,
        skipTraversal: true,
        child: MouseRegion(
          onExit: (_) {
            if (_hovered != null) setState(() => _hovered = null);
          },
          child: Row(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            spacing: vars.spacing4,
            children: months,
          ),
        ),
      ),
    );
  }

  Widget _month(
    BuildContext context,
    ThemeVariables vars, {
    required int index,
    required double cell,
    required ColorSwatch<int> seed,
    required DateTime today,
    required DateRange range,
    required bool pending,
    required DateTime? preview,
    required DateTime? lo,
    required DateTime? hi,
    required DateTime stop,
  }) {
    final DateTime current = addMonths(_firstMonth, index);
    final List<DateTime> days = _grid(current);
    final double width = 7 * cell + 6 * vars.calendarCellGap;
    final Widget spacer = SizedBox(
      width: vars.controlTinySize,
      height: vars.controlTinySize,
    );

    // The bar: a chevron at each end of the month's name, in the tiny glyph
    // square. A middle month of a run has neither, and keeps the room.
    final Widget head = Row(
      children: [
        if (widget.navigation && index == 0)
          IconButton(
            icon: const Icon(FluentIcons.chevron_left_12_regular),
            semanticsLabel: widget.strings.previousMonth,
            onPressed: () => _setMonth(addMonths(_firstMonth, -1)),
          )
        else if (widget.navigation)
          spacer,
        Expanded(
          child: Text(
            widget.strings.monthTitle(current),
            textAlign: TextAlign.center,
            style: vars.labelMedium.copyWith(
              color: vars.colorContent,
              fontFeatures: const [FontFeature.tabularFigures()],
            ),
          ),
        ),
        if (widget.navigation && index == widget.months - 1)
          IconButton(
            icon: const Icon(FluentIcons.chevron_right_12_regular),
            semanticsLabel: widget.strings.nextMonth,
            onPressed: () => _setMonth(addMonths(_firstMonth, 1)),
          )
        else if (widget.navigation)
          spacer,
      ],
    );

    final Widget weekdays = Row(
      spacing: vars.calendarCellGap,
      children: [
        for (int i = 0; i < 7; i++)
          SizedBox(
            width: cell,
            height: vars.spacing5,
            child: Center(
              child: Text(
                widget.strings.weekday((widget.weekStartsOn - 1 + i) % 7 + 1),
                style: vars.captionSmall.copyWith(
                  fontWeight: vars.labelSmall.fontWeight,
                  height: 1,
                  color: vars.colorContentSubtle,
                ),
              ),
            ),
          ),
      ],
    );

    final List<Widget> rows = <Widget>[
      for (int r = 0; r < 6; r++)
        Row(
          spacing: vars.calendarCellGap,
          children: [
            for (final DateTime day in days.sublist(r * 7, r * 7 + 7))
              _day(
                context,
                vars,
                day: day,
                current: current,
                cell: cell,
                seed: seed,
                today: today,
                range: range,
                pending: pending,
                preview: preview,
                lo: lo,
                hi: hi,
                stop: sameDay(day, stop),
              ),
          ],
        ),
    ];

    return SizedBox(
      width: width,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        spacing: vars.spacing1,
        children: [
          head,
          weekdays,
          Column(
            mainAxisSize: MainAxisSize.min,
            spacing: vars.calendarCellGap,
            children: rows,
          ),
        ],
      ),
    );
  }

  Widget _day(
    BuildContext context,
    ThemeVariables vars, {
    required DateTime day,
    required DateTime current,
    required double cell,
    required ColorSwatch<int> seed,
    required DateTime today,
    required DateRange range,
    required bool pending,
    required DateTime? preview,
    required DateTime? lo,
    required DateTime? hi,
    required bool stop,
  }) {
    final bool isStart = sameDay(day, range.start);
    final bool isEnd = sameDay(day, range.end);
    final bool picked = widget.mode == CalendarMode.range
        ? isStart || isEnd
        : isStart;
    final bool inSpan =
        widget.mode == CalendarMode.range &&
        lo != null &&
        hi != null &&
        compareDay(day, lo) >= 0 &&
        compareDay(day, hi) <= 0;
    final bool spanStart = inSpan && sameDay(day, lo);
    final bool spanEnd = inSpan && sameDay(day, hi);
    final bool previewed =
        pending && preview != null && sameDay(day, preview) && !isStart;
    final bool outside = day.month != current.month;
    final bool isToday = sameDay(day, today);
    final bool off = _disabled(day);
    final FocusNode node = _node(day);
    node.skipTraversal = !stop || outside;

    final Radius corner = Radius.circular(vars.radiusSmall);
    // The span between the two ends is one band: the days inside square
    // their corners, and the two ends keep the corner only on the outer
    // side, so the band's edge stays concentric with the day's.
    final BorderRadius radius = !inSpan
        ? BorderRadius.all(corner)
        : BorderRadiusDirectional.horizontal(
            start: spanStart ? corner : Radius.zero,
            end: spanEnd ? corner : Radius.zero,
          ).resolve(Directionality.of(context));

    return Pressable(
      onPressed: off
          ? null
          : () {
              // A press lands the grid's tab stop on the day, as a click
              // focuses a button.
              setState(() => _focused = day);
              node.requestFocus();
              _pick(day);
            },
      enabled: !off,
      focusNode: node,
      onFocusChange: (bool value) {
        if (value && !sameDay(_focused, day)) setState(() => _focused = day);
      },
      onHover: (bool value) {
        if (widget.mode != CalendarMode.range) return;
        if (value && !sameDay(_hovered, day)) setState(() => _hovered = day);
      },
      selected: picked || inSpan,
      semanticsLabel: widget.strings.dayLabel(day),
      borderRadius: radius,
      cursor: off ? SystemMouseCursors.forbidden : SystemMouseCursors.click,
      builder: (context, states) {
        final bool hovered = states.contains(WidgetState.hovered);
        final bool pressed = states.contains(WidgetState.pressed);

        Color surface;
        Color content;
        if (picked || inSpan) {
          final _DayVariant drawn = picked
              ? _DayVariant.filled
              : _DayVariant.tinted;
          surface = vars.controlColorSurface.varianted<Color>(
            drawn,
            states,
            seed,
          );
          content = vars.controlColorContent.varianted<Color>(
            drawn,
            states,
            seed,
          );
        } else {
          // At rest: the row's drawing, not the quiet accent control's.
          surface = hovered || pressed
              ? (vars.controlColorNormalSurface.hoveredColor ??
                    vars.colorSurfaceSubtle)
              : const Color(0x00000000);
          content = off || outside
              ? vars.colorContentFaint
              : (isToday ? seed[600]! : vars.colorContent);
        }

        final TextStyle face = vars.labelQuiet.copyWith(
          height: 1,
          color: content,
          fontFeatures: const [FontFeature.tabularFigures()],
          // Today is a weight heavier, and a picked day the selection's
          // weight — the half of a selection that survives on a monochrome
          // theme.
          fontWeight: isToday
              ? vars.labelStrong.fontWeight
              : (picked
                    ? vars.labelMedium.fontWeight
                    : vars.labelQuiet.fontWeight),
        );

        return AnimatedContainer(
          duration: vars.motionDuration,
          curve: vars.motionEasing,
          width: cell,
          height: cell,
          decoration: BoxDecoration(color: surface, borderRadius: radius),
          // The end a pending range is previewing against: dashed, because
          // it is where the span would end, not where it does.
          foregroundDecoration: previewed
              ? _DashedOutline(
                  color: seed[600]!,
                  width: vars.strokeHairline,
                  radius: radius,
                )
              : null,
          child: Stack(
            alignment: Alignment.center,
            children: [
              Text('${day.day}', style: face),
              // Today's dot, in whatever ink the recipe gave the label, so
              // today is never lost under a selection.
              if (isToday)
                Positioned(
                  bottom: vars.spacing05,
                  child: Container(
                    width: vars.spacing05,
                    height: vars.spacing05,
                    decoration: BoxDecoration(
                      color: content,
                      shape: BoxShape.circle,
                    ),
                  ),
                ),
            ],
          ),
        );
      },
    );
  }
}

/// A dashed hairline just inside a rounded box — `outline: dashed` with a
/// negative offset, which Flutter's borders cannot draw.
class _DashedOutline extends Decoration {
  const _DashedOutline({
    required this.color,
    required this.width,
    required this.radius,
  });

  final Color color;
  final double width;
  final BorderRadius radius;

  @override
  BoxPainter createBoxPainter([VoidCallback? onChanged]) =>
      _DashedOutlinePainter(this);
}

class _DashedOutlinePainter extends BoxPainter {
  _DashedOutlinePainter(this.decoration);

  final _DashedOutline decoration;

  @override
  void paint(Canvas canvas, Offset offset, ImageConfiguration configuration) {
    final Size size = configuration.size ?? Size.zero;
    final Rect rect = (offset & size).deflate(decoration.width / 2);
    final Path outline = Path()..addRRect(decoration.radius.toRRect(rect));
    final Paint paint = Paint()
      ..color = decoration.color
      ..style = PaintingStyle.stroke
      ..strokeWidth = decoration.width;
    const double dash = 3;
    const double gap = 2;
    for (final PathMetric metric in outline.computeMetrics()) {
      double at = 0;
      while (at < metric.length) {
        final double to = (at + dash).clamp(0, metric.length);
        canvas.drawPath(metric.extractPath(at, to), paint);
        at = to + gap;
      }
    }
  }
}
