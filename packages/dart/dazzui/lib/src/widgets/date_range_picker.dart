import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/widgets.dart';

import '../foundation/widget_size.dart';
import '../foundation/widget_tint.dart';
import '../generated/theme_variables.dart';
import '../painting/widget_property.dart';
import '../theme/theme.dart';
import 'button.dart';
import 'calendar.dart';
import 'field_box.dart';
import 'menu.dart';
import 'popover.dart';
import 'pressable.dart';
import 'select.dart';
import 'text_field.dart';
import 'toggle.dart';

/// What the picker chooses: when something starts, when it ends, and whether
/// the times matter. An all-day span keeps its times — switching all-day off
/// again brings them back — and a consumer reads [allDay] before them.
@immutable
class DateRangeValue {
  const DateRangeValue({
    required this.start,
    required this.end,
    this.allDay = false,
  });

  final DateTime start;
  final DateTime end;
  final bool allDay;

  DateRangeValue copyWith({DateTime? start, DateTime? end, bool? allDay}) =>
      DateRangeValue(
        start: start ?? this.start,
        end: end ?? this.end,
        allDay: allDay ?? this.allDay,
      );

  @override
  bool operator ==(Object other) =>
      other is DateRangeValue &&
      other.start == start &&
      other.end == end &&
      other.allDay == allDay;

  @override
  int get hashCode => Object.hash(start, end, allDay);
}

/// A named span the reader can pick in one press, counted from today.
@immutable
class DateRangePreset {
  const DateRangePreset({required this.label, required this.range});

  final String label;
  final DateRange Function(DateTime today) range;
}

/// A length the end is set to, from the start.
@immutable
class DateRangeDuration {
  const DateRangeDuration({required this.label, required this.minutes});

  final String label;
  final int minutes;
}

/// The ramp the picker resolves against.
enum DateRangePickerTint with WidgetTint {
  primary,
  neutral,
  info,
  success,
  warning,
  danger,
}

/// The strings the sheet is labelled with, for a product in another
/// language, and the way it writes a day.
@immutable
class DateRangePickerStrings {
  const DateRangePickerStrings({
    this.placeholder = 'Choose dates',
    this.presets = 'Quick pick',
    this.start = 'Start',
    this.end = 'End',
    this.duration = 'Duration',
    this.allDay = 'All day',
    this.cancel = 'Cancel',
    this.done = 'Done',
    this.day = _day,
    this.calendar = const CalendarStrings(),
  });

  /// What the trigger says while nothing is chosen.
  final String placeholder;
  final String presets;
  final String start;
  final String end;
  final String duration;
  final String allDay;
  final String cancel;
  final String done;

  /// A day in the trigger and the column, e.g. `Fri, Oct 16`.
  final String Function(DateTime day) day;

  final CalendarStrings calendar;

  static const List<String> _months = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
  ];
  static const List<String> _days = [
    'Mon',
    'Tue',
    'Wed',
    'Thu',
    'Fri',
    'Sat',
    'Sun',
  ];

  static String _day(DateTime day) =>
      '${_days[day.weekday - 1]}, ${_months[day.month - 1]} ${day.day}';
}

DateTime _weekend(DateTime today) {
  final int until = (DateTime.saturday - today.weekday + 7) % 7;
  return addDays(today, until == 0 ? 7 : until);
}

DateTime _nextMonday(DateTime today) {
  final int until = (DateTime.monday - today.weekday + 7) % 7;
  return addDays(today, until == 0 ? 7 : until);
}

/// The spans a calendar reaches for most.
const List<DateRangePreset> dateRangePresets = [
  DateRangePreset(label: 'Today', range: _today),
  DateRangePreset(label: 'Tomorrow', range: _tomorrow),
  DateRangePreset(label: 'This weekend', range: _thisWeekend),
  DateRangePreset(label: 'Next Monday', range: _monday),
  DateRangePreset(label: 'Next week', range: _nextWeek),
  DateRangePreset(label: 'This month', range: _thisMonth),
];

DateRange _today(DateTime t) => DateRange(start: t, end: t);
DateRange _tomorrow(DateTime t) =>
    DateRange(start: addDays(t, 1), end: addDays(t, 1));
DateRange _thisWeekend(DateTime t) =>
    DateRange(start: _weekend(t), end: addDays(_weekend(t), 1));
DateRange _monday(DateTime t) =>
    DateRange(start: _nextMonday(t), end: _nextMonday(t));
DateRange _nextWeek(DateTime t) =>
    DateRange(start: _nextMonday(t), end: addDays(_nextMonday(t), 6));
DateRange _thisMonth(DateTime t) => DateRange(
  start: startOfMonth(t),
  end: DateTime(t.year, t.month + 1, 0),
);

const List<DateRangeDuration> dateRangeDurations = [
  DateRangeDuration(label: '30 min', minutes: 30),
  DateRangeDuration(label: '1 hour', minutes: 60),
  DateRangeDuration(label: '2 hours', minutes: 120),
];

String _pad(int n) => n.toString().padLeft(2, '0');
String _clock(DateTime date) => '${_pad(date.hour)}:${_pad(date.minute)}';

/// A span of days and times, chosen in one sheet.
///
/// The trigger is a field — it wears the select's box — and what it opens is
/// a sheet: a month to pick the span on, two for a product whose spans mostly
/// cross one, and beside it a column that opens with a menu of the spans a
/// calendar reaches for most, then the start and end with their times and a
/// row of durations. Nothing in the sheet reaches the value until Done; an
/// outside press or Escape puts it back. That is the one way this differs
/// from every other popup here, and it is deliberate: a span is several
/// decisions, and a reader who has made two of them should not lose both to
/// a stray click.
///
/// The sheet is composition rather than behaviour: the kit's own [Calendar],
/// [Select], [Menu] and [Toggle] own the parts, and the popover layer owns
/// the outside press and the Escape. The two rules of its own are the ones a
/// calendar app keeps: moving the start carries the end with it, and an end
/// set before its start pulls the start back an hour ahead of it.
class DateRangePicker extends StatefulWidget {
  const DateRangePicker({
    super.key,
    this.value,
    this.initialValue,
    this.onChanged,
    this.presets = dateRangePresets,
    this.durations = dateRangeDurations,
    this.timeStep = 15,
    this.months = 1,
    this.weekStartsOn = DateTime.monday,
    this.strings = const DateRangePickerStrings(),
    this.today,
    this.size = WidgetSize.medium,
    this.state = TextFieldState.normal,
    this.tint = DateRangePickerTint.primary,
    this.enabled = true,
    this.semanticsLabel,
  });

  /// Driven from outside. Left null the widget holds its own.
  final DateRangeValue? value;
  final DateRangeValue? initialValue;
  final ValueChanged<DateRangeValue>? onChanged;

  /// The quick-pick menu. [dateRangePresets] unless the product has its own.
  final List<DateRangePreset> presets;

  /// The duration chips. [dateRangeDurations] unless the product has its own.
  final List<DateRangeDuration> durations;

  /// Minutes between the times the lists offer.
  final int timeStep;

  /// Months side by side in the sheet: one, or two.
  final int months;

  final int weekStartsOn;

  final DateRangePickerStrings strings;

  /// What the grid counts as today.
  final DateTime? today;

  final WidgetSize size;
  final TextFieldState state;
  final DateRangePickerTint tint;
  final bool enabled;
  final String? semanticsLabel;

  @override
  State<DateRangePicker> createState() => _DateRangePickerState();
}

class _DateRangePickerState extends State<DateRangePicker> {
  final OverlayPortalController _controller = OverlayPortalController();
  final LayerLink _link = LayerLink();
  bool _open = false;
  DateRangeValue? _own;
  late DateRangeValue _draft;
  // The days as the calendar holds them, apart from the draft: between the
  // first press and the second the calendar's end is unset, and the draft
  // reads that as a one-day span until the reader finishes.
  DateRange _days = const DateRange.empty();
  late DateTime _month;

  static const Duration _hour = Duration(hours: 1);

  DateTime get _today => startOfDay(widget.today ?? DateTime.now());

  DateRangeValue? get _value => widget.value ?? _own;

  @override
  void initState() {
    super.initState();
    _own = widget.initialValue;
    _draft = _blank();
    _month = startOfMonth(_today);
  }

  /// The next whole hour, for an hour — what a new event defaults to.
  DateRangeValue _blank() {
    final DateTime now = DateTime.now();
    final DateTime today = _today;
    final DateTime start = sameDay(now, today)
        ? DateTime(
            today.year,
            today.month,
            today.day,
            (now.hour + 1).clamp(0, 23),
          )
        : DateTime(today.year, today.month, today.day, 9);
    return DateRangeValue(start: start, end: start.add(_hour));
  }

  void _toggle() => _open ? _close() : _openSheet();

  void _openSheet() {
    if (_open) return;
    final DateRangeValue initial = _value ?? _blank();
    setState(() {
      _draft = initial;
      _days = DateRange(start: initial.start, end: initial.end);
      _month = startOfMonth(initial.start);
      _open = true;
    });
    _controller.show();
  }

  void _close() {
    if (!_open) return;
    setState(() => _open = false);
    _controller.hide();
  }

  void _commit() {
    if (widget.value == null) setState(() => _own = _draft);
    widget.onChanged?.call(_draft);
    _close();
  }

  void _setDays(DateRange range) {
    final DateTime start = range.start ?? _draft.start;
    final DateTime end = range.end ?? start;
    setState(() {
      _days = range;
      final DateTime nextStart = withTime(start, _draft.start);
      DateTime nextEnd = withTime(end, _draft.end);
      if (nextEnd.isBefore(nextStart)) nextEnd = nextStart.add(_hour);
      _draft = _draft.copyWith(start: nextStart, end: nextEnd);
    });
  }

  static (int, int) _parse(String time) {
    final List<String> parts = time.split(':');
    return (int.tryParse(parts[0]) ?? 0, int.tryParse(parts[1]) ?? 0);
  }

  void _setStartTime(String? time) {
    if (time == null) return;
    final (int h, int m) = _parse(time);
    setState(() {
      final Duration length = _draft.end.difference(_draft.start);
      final DateTime start = DateTime(
        _draft.start.year,
        _draft.start.month,
        _draft.start.day,
        h,
        m,
      );
      _draft = _draft.copyWith(start: start, end: start.add(length));
    });
  }

  void _setEndTime(String? time) {
    if (time == null) return;
    final (int h, int m) = _parse(time);
    setState(() {
      final DateTime end = DateTime(
        _draft.end.year,
        _draft.end.month,
        _draft.end.day,
        h,
        m,
      );
      final DateTime start = end.isBefore(_draft.start)
          ? end.subtract(_hour)
          : _draft.start;
      _draft = _draft.copyWith(start: start, end: end);
    });
  }

  void _setDuration(int minutes) {
    setState(() {
      _draft = _draft.copyWith(
        allDay: false,
        end: _draft.start.add(Duration(minutes: minutes)),
      );
    });
  }

  void _applyPreset(DateRangePreset preset) {
    final DateRange range = preset.range(_today);
    setState(() => _month = startOfMonth(range.start ?? _today));
    _setDays(range);
  }

  bool _presetOn(DateRangePreset preset) {
    final DateRange range = preset.range(_today);
    return sameDay(range.start, _draft.start) && sameDay(range.end, _draft.end);
  }

  bool get _oneDay => compareDay(_draft.start, _draft.end) == 0;

  bool _durationOn(DateRangeDuration duration) =>
      !_draft.allDay &&
      _draft.end.difference(_draft.start) ==
          Duration(minutes: duration.minutes);

  /// A time list that starts on the step but still shows a value off it — a
  /// span handed in at 14:07 keeps 14:07 until the reader moves it.
  List<SelectOption<String>> _times(DateTime date) {
    final String current = _clock(date);
    final List<SelectOption<String>> options = [
      if ((date.hour * 60 + date.minute) % widget.timeStep != 0)
        SelectOption<String>(value: current, label: current),
      for (int m = 0; m < 24 * 60; m += widget.timeStep)
        SelectOption<String>(
          value: '${_pad(m ~/ 60)}:${_pad(m % 60)}',
          label: '${_pad(m ~/ 60)}:${_pad(m % 60)}',
        ),
    ];
    return options;
  }

  ColorSwatch<int> _ramp(ThemeVariables vars) =>
      vars.controlColor.tinted<ColorSwatch<int>>(
        widget.state == TextFieldState.error
            ? DateRangePickerTint.danger
            : widget.tint,
      );

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;

    return CompositedTransformTarget(
      link: _link,
      child: OverlayPortal(
        controller: _controller,
        overlayChildBuilder: (context) => PopoverOverlay(
          link: _link,
          side: PopoverSide.bottom,
          align: PopoverAlign.start,
          gap: vars.spacing1,
          onDismiss: _close,
          child: _sheet(context, vars),
        ),
        child: Pressable(
          onPressed: widget.enabled ? _toggle : null,
          enabled: widget.enabled,
          semanticsLabel: widget.semanticsLabel,
          borderRadius: BorderRadius.circular(vars.controlFieldRadius),
          builder: (context, states) => _trigger(context, vars, states),
        ),
      ),
    );
  }

  /// The trigger: the select's box, with the calendar glyph at its leading
  /// edge and the span where the value goes.
  Widget _trigger(
    BuildContext context,
    ThemeVariables vars,
    Set<WidgetState> states,
  ) {
    final FieldMetrics metrics = FieldMetrics.of(vars, widget.size);
    final DateRangeValue? value = _value;
    final bool invalid = widget.state == TextFieldState.error;
    final Color ink = !widget.enabled
        ? vars.controlColorNormalContent.disabledColor!
        : (value == null
              ? vars.colorContentFaint
              : (invalid ? vars.colorDanger[800]! : vars.colorContent));
    final TextStyle face = metrics.face.copyWith(
      fontWeight: vars.bodySmall.fontWeight,
      height: 1,
      color: ink,
      fontFeatures: const [FontFeature.tabularFigures()],
    );

    return AnimatedContainer(
      duration: vars.motionDuration,
      curve: vars.motionEasing,
      height: metrics.height,
      constraints: const BoxConstraints(minWidth: 256),
      decoration: fieldBoxDecoration(
        context,
        ramp: _ramp(vars),
        focused: states.contains(WidgetState.focused) || _open,
        invalid: invalid,
        enabled: widget.enabled,
      ),
      padding: EdgeInsetsDirectional.fromSTEB(
        vars.spacing3,
        metrics.paddingBlock,
        vars.spacing3,
        metrics.paddingBlock,
      ),
      child: Row(
        spacing: vars.spacing2,
        children: [
          Icon(
            FluentIcons.calendar_ltr_16_regular,
            size: vars.iconSmall,
            color: widget.enabled
                ? vars.colorContentSubtle
                : vars.colorContentFaint,
          ),
          Expanded(
            child: value == null
                ? Text(
                    widget.strings.placeholder,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: face,
                  )
                : _Summary(value: value, strings: widget.strings, style: face),
          ),
          Icon(
            FluentIcons.chevron_down_12_regular,
            size: vars.iconSmall,
            color: widget.enabled
                ? vars.colorContentSubtle
                : vars.colorContentFaint,
          ),
        ],
      ),
    );
  }

  /// The sheet: the shared popup holding the months, the column beside them
  /// and a footer, hairlined off each other.
  Widget _sheet(BuildContext context, ThemeVariables vars) {
    final BorderSide hairline = BorderSide(
      color: vars.colorBorder,
      width: context.hairlineWidth,
    );

    return Container(
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: vars.colorSurfaceOverlay,
        border: Border.all(
          color: vars.colorBorderStrong,
          width: context.hairlineWidth,
        ),
        borderRadius: BorderRadius.circular(vars.radiusLarge),
        boxShadow: vars.shadowLg,
      ),
      // The width is the sum of its parts rather than measured: the months
      // and the column are both stated lengths, and a select inside measures
      // through a LayoutBuilder, which an intrinsic-width pass cannot ask.
      width:
          _calendarWidth(vars) +
          vars.dateRangePickerSideWidth +
          2 * context.hairlineWidth,
      child: DefaultTextStyle.merge(
        style: vars.bodySmall.copyWith(color: vars.colorContent),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            IntrinsicHeight(
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Padding(
                    padding: EdgeInsets.all(vars.spacing2),
                    child: Calendar.range(
                      size: WidgetSize.small,
                      months: widget.months,
                      month: _month,
                      onMonthChanged: (m) => setState(() => _month = m),
                      range: _days,
                      onRangeChanged: _setDays,
                      weekStartsOn: widget.weekStartsOn,
                      strings: widget.strings.calendar,
                      today: _today,
                      tint: CalendarTint.values.byName(widget.tint.name),
                    ),
                  ),
                  Container(
                    width: vars.dateRangePickerSideWidth,
                    padding: EdgeInsets.fromLTRB(
                      vars.spacing3,
                      vars.spacing2,
                      vars.spacing3,
                      vars.spacing3,
                    ),
                    decoration: BoxDecoration(
                      border: BorderDirectional(start: hairline),
                    ),
                    child: _side(context, vars),
                  ),
                ],
              ),
            ),
            Container(
              padding: EdgeInsets.fromLTRB(
                vars.spacing3,
                vars.spacing2,
                vars.spacing2,
                vars.spacing2,
              ),
              decoration: BoxDecoration(border: Border(top: hairline)),
              child: Row(
                spacing: vars.spacing3,
                children: [
                  Expanded(
                    child: _Summary(
                      value: _draft,
                      strings: widget.strings,
                      style: vars.labelSmall.copyWith(
                        color: vars.colorContentSecondary,
                        fontFeatures: const [FontFeature.tabularFigures()],
                      ),
                    ),
                  ),
                  Button(
                    variant: ButtonVariant.normal,
                    onPressed: _close,
                    child: Text(widget.strings.cancel),
                  ),
                  Button(
                    variant: ButtonVariant.filled,
                    tint: ButtonTint.values.byName(widget.tint.name),
                    onPressed: _commit,
                    child: Text(widget.strings.done),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  /// The months' column: each month's seven cells and six gaps, the run
  /// between two months, and the inset around them.
  double _calendarWidth(ThemeVariables vars) {
    final double month =
        7 * vars.calendarSmallCellSize + 6 * vars.calendarCellGap;
    return widget.months * month +
        (widget.months - 1) * vars.spacing4 +
        2 * vars.spacing2;
  }

  /// The column: the quick-pick menu's name on the first line, then a key
  /// over a date and its time, twice, then the durations.
  Widget _side(BuildContext context, ThemeVariables vars) {
    final TextStyle key = vars.captionSmall.copyWith(
      fontWeight: vars.labelSmall.fontWeight,
      height: 1,
      letterSpacing: 0.06 * (vars.captionSmall.fontSize ?? 11),
      color: vars.colorContentSubtle,
    );
    final TextStyle date = vars.labelQuiet.copyWith(
      height: 1,
      color: vars.colorContent,
      fontFeatures: const [FontFeature.tabularFigures()],
    );
    final ToggleTint chipTint = ToggleTint.values.byName(widget.tint.name);

    Widget end({
      required String label,
      required DateTime at,
      required ValueChanged<String?> onTime,
    }) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        spacing: vars.spacing1,
        children: [
          Text(label.toUpperCase(), style: key),
          Row(
            spacing: vars.spacing2,
            children: [
              Expanded(child: Text(widget.strings.day(at), style: date)),
              // Tight on both axes, so the row's intrinsic height — which
              // the rule beside the months needs — is read off the box
              // rather than asked of the select's own layout.
              SizedBox(
                width: 80,
                height: vars.controlSmallSize,
                child: Select<String>(
                  size: WidgetSize.small,
                  options: _times(at),
                  value: _clock(at),
                  onChanged: _draft.allDay ? null : onTime,
                ),
              ),
            ],
          ),
        ],
      );
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      spacing: vars.spacing3,
      children: [
        // A plain button pads its label by the profile's inline step; pulling
        // it back by that sets the name on the column's own edge.
        Transform.translate(
          offset: Offset(-vars.controlTinyPaddingInline, 0),
          child: Menu(
            align: MenuAlign.start,
            items: [
              for (final DateRangePreset preset in widget.presets)
                MenuItem(
                  label: preset.label,
                  checked: _presetOn(preset),
                  onSelect: () => _applyPreset(preset),
                ),
            ],
            trigger: (context, state) => Button(
              size: WidgetSize.tiny,
              variant: ButtonVariant.plain,
              tint: ButtonTint.neutral,
              trailing: const Icon(FluentIcons.chevron_down_12_regular),
              onPressed: state.toggle,
              child: Text(widget.strings.presets),
            ),
          ),
        ),
        end(
          label: widget.strings.start,
          at: _draft.start,
          onTime: _setStartTime,
        ),
        end(label: widget.strings.end, at: _draft.end, onTime: _setEndTime),
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          spacing: vars.spacing15,
          children: [
            Text(widget.strings.duration.toUpperCase(), style: key),
            Wrap(
              spacing: vars.spacing1,
              runSpacing: vars.spacing1,
              children: [
                for (final DateRangeDuration duration in widget.durations)
                  Toggle(
                    variant: ToggleVariant.outlined,
                    tint: chipTint,
                    pressed: _durationOn(duration),
                    // A duration is a length within a day; a span across
                    // days has none.
                    enabled: _oneDay,
                    onPressedChanged: (_) => _setDuration(duration.minutes),
                    child: Text(duration.label),
                  ),
                Toggle(
                  variant: ToggleVariant.outlined,
                  tint: chipTint,
                  pressed: _draft.allDay,
                  onPressedChanged: (bool allDay) =>
                      setState(() => _draft = _draft.copyWith(allDay: allDay)),
                  child: Text(widget.strings.allDay),
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }
}

/// A span in one line: the day once when both ends share it, the times
/// unless it is all day, and the joins in the faint ink so the dates and
/// times read as the values and the arrow as punctuation.
class _Summary extends StatelessWidget {
  const _Summary({
    required this.value,
    required this.strings,
    required this.style,
  });

  final DateRangeValue value;
  final DateRangePickerStrings strings;
  final TextStyle style;

  @override
  Widget build(BuildContext context) {
    final ThemeVariables vars = Theme.of(context).vars;
    final bool oneDay = compareDay(value.start, value.end) == 0;
    final TextStyle faint = style.copyWith(color: vars.colorContentFaint);
    final String startDay = strings.day(value.start);
    final String endDay = strings.day(value.end);

    TextSpan sep(String glyph) => TextSpan(text: ' $glyph ', style: faint);

    final List<InlineSpan> spans = value.allDay
        ? (oneDay
              ? [
                  TextSpan(text: startDay),
                  sep('·'),
                  TextSpan(text: strings.allDay),
                ]
              : [TextSpan(text: startDay), sep('→'), TextSpan(text: endDay)])
        : (oneDay
              ? [
                  TextSpan(text: startDay),
                  sep('·'),
                  TextSpan(text: _clock(value.start)),
                  sep('→'),
                  TextSpan(text: _clock(value.end)),
                ]
              : [
                  TextSpan(text: '$startDay ${_clock(value.start)}'),
                  sep('→'),
                  TextSpan(text: '$endDay ${_clock(value.end)}'),
                ]);

    return Text.rich(
      TextSpan(style: style, children: spans),
      maxLines: 1,
      overflow: TextOverflow.ellipsis,
    );
  }
}
