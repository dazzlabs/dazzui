using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Documents;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;

namespace DazzUI;

/// <summary>
/// What the picker chooses: when something starts, when it ends, and whether
/// the times matter. An all-day span keeps its times — switching all-day off
/// again brings them back — and a consumer reads <see cref="AllDay"/> before
/// them.
/// </summary>
public readonly record struct DateRangeValue(DateTime Start, DateTime End, bool AllDay = false);

/// <summary>A named span the reader can pick in one press, counted from today.</summary>
public sealed record DateRangePreset(string Label, Func<DateOnly, (DateOnly Start, DateOnly End)> Range)
{
    /// <summary>A span as whole days from today — the form a preset takes as data: <c>Days("Today", 0, 0)</c>.</summary>
    public static DateRangePreset Days(string label, int from, int to) =>
        new(label, today => (today.AddDays(from), today.AddDays(to)));

    /// <summary>The week's Saturday, or next week's if today is one.</summary>
    private static DateOnly Weekend(DateOnly today)
    {
        var until = (6 - (int)today.DayOfWeek + 7) % 7;
        return today.AddDays(until == 0 ? 7 : until);
    }

    /// <summary>Next Monday, never today.</summary>
    private static DateOnly NextMonday(DateOnly today)
    {
        var until = (8 - (int)today.DayOfWeek) % 7;
        return today.AddDays(until == 0 ? 7 : until);
    }

    /// <summary>The spans a calendar reaches for most.</summary>
    public static IReadOnlyList<DateRangePreset> Default { get; } =
    [
        new("Today", t => (t, t)),
        new("Tomorrow", t => (t.AddDays(1), t.AddDays(1))),
        new("This weekend", t => (Weekend(t), Weekend(t).AddDays(1))),
        new("Next Monday", t => (NextMonday(t), NextMonday(t))),
        new("Next week", t => (NextMonday(t), NextMonday(t).AddDays(6))),
        new("This month", t => (Calendar.StartOfMonth(t), Calendar.AddMonths(t, 1).AddDays(-1))),
    ];
}

/// <summary>A length the end is set to, from the start.</summary>
public sealed record DateRangeDuration(string Label, int Minutes)
{
    public static IReadOnlyList<DateRangeDuration> Default { get; } =
    [
        new("30 min", 30),
        new("1 hour", 60),
        new("2 hours", 120),
    ];
}

/// <summary>The strings the sheet is labelled with, for a product in another language, and the way it writes a day.</summary>
public sealed record DateRangePickerStrings(
    string Placeholder = "Choose dates",
    string Presets = "Quick pick",
    string Start = "Start",
    string End = "End",
    string Duration = "Duration",
    string AllDay = "All day",
    string Cancel = "Cancel",
    string Done = "Done",
    Func<DateOnly, string>? Day = null,
    CalendarStrings? Calendar = null)
{
    /// <summary>A day in the trigger and the column, e.g. <c>Fri, Oct 16</c>.</summary>
    public string DayText(DateOnly day) =>
        (Day ?? (d => $"{CalendarStrings.Weekdays[(int)d.DayOfWeek][..3]}, {CalendarStrings.Months[d.Month - 1][..3]} {d.Day}"))(day);
}

/// <summary>
/// The sheet's own rules, apart from the view so they can be read and tested
/// as data: moving the start carries the end with it, an end set before its
/// start pulls the start back an hour ahead of it, and the days the calendar
/// holds stay apart from the draft while a range is mid-pick.
/// </summary>
internal sealed class DateRangeDraft(DateRangeValue initial)
{
    private static readonly TimeSpan Hour = TimeSpan.FromHours(1);

    public DateRangeValue Value { get; set; } = initial;
    /// <summary>The days as the calendar holds them: between the first press and the second the calendar's end is unset.</summary>
    public DateRange Days { get; private set; } = new(DateOnly.FromDateTime(initial.Start), DateOnly.FromDateTime(initial.End));
    /// <summary>The month the sheet shows — the first of two.</summary>
    public DateOnly Month { get; set; } = Calendar.StartOfMonth(DateOnly.FromDateTime(initial.Start));

    private static DateTime WithTime(DateOnly day, DateTime time) => day.ToDateTime(new TimeOnly(time.Hour, time.Minute));

    public void SetDays(DateRange range)
    {
        Days = range;
        var start = range.Start ?? DateOnly.FromDateTime(Value.Start);
        var end = range.End ?? start;
        var nextStart = WithTime(start, Value.Start);
        var nextEnd = WithTime(end, Value.End);
        if (nextEnd < nextStart) nextEnd = nextStart + Hour;
        Value = Value with { Start = nextStart, End = nextEnd };
    }

    public void SetStartTime(int hour, int minute)
    {
        var length = Value.End - Value.Start;
        var start = Value.Start.Date + new TimeSpan(hour, minute, 0);
        Value = Value with { Start = start, End = start + length };
    }

    public void SetEndTime(int hour, int minute)
    {
        var end = Value.End.Date + new TimeSpan(hour, minute, 0);
        var start = end < Value.Start ? end - Hour : Value.Start;
        Value = Value with { Start = start, End = end };
    }

    public void SetDuration(int minutes) =>
        Value = Value with { AllDay = false, End = Value.Start + TimeSpan.FromMinutes(minutes) };

    /// <summary>A duration is a length within a day; a span across days has none.</summary>
    public bool OneDay => Value.Start.Date == Value.End.Date;

    public bool Matches(DateRangePreset preset, DateOnly today)
    {
        var (start, end) = preset.Range(today);
        return start == DateOnly.FromDateTime(Value.Start) && end == DateOnly.FromDateTime(Value.End);
    }

    public bool Matches(DateRangeDuration duration) =>
        !Value.AllDay && Value.End - Value.Start == TimeSpan.FromMinutes(duration.Minutes);
}

/// <summary>
/// A span of days and times, chosen in one sheet.
///
/// The trigger is a field — it wears the select's box — and what it opens
/// is a sheet: a month to pick the span on, two for a product whose spans
/// mostly cross one, and beside it a column that opens with a menu of the
/// spans a calendar reaches for most, then the start and end with their
/// times and a row of durations. Nothing in the sheet reaches the value
/// until Done; an outside press or Escape puts it back — a span is several
/// decisions, and a reader who has made two of them should not lose both to
/// a stray click.
///
/// The sheet is composition rather than behaviour: the kit's own
/// <see cref="Calendar"/>, <see cref="Select"/>, <see cref="Menu"/> and
/// <see cref="Toggle"/> own the parts, and WinUI's flyout owns the outside
/// press and the Escape.
/// </summary>
public sealed class DateRangePicker : Surface
{
    private const string TriggerTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='Button'><Border Background='{TemplateBinding Background}' BorderBrush='{TemplateBinding BorderBrush}' BorderThickness='{TemplateBinding BorderThickness}' CornerRadius='{TemplateBinding CornerRadius}' Padding='{TemplateBinding Padding}'><ContentPresenter Content='{TemplateBinding Content}' Foreground='{TemplateBinding Foreground}' HorizontalContentAlignment='Stretch' VerticalContentAlignment='Center'/></Border></ControlTemplate>";
    private readonly Microsoft.UI.Xaml.Controls.Button trigger = new() { HorizontalAlignment = HorizontalAlignment.Stretch, HorizontalContentAlignment = HorizontalAlignment.Stretch };
    private readonly Flyout sheet = new() { Placement = FlyoutPlacementMode.BottomEdgeAlignedLeft };
    private DateRangeValue? value;
    private bool invalid;
    private bool focused;
    private DateRangeDraft draft = new(Blank(DateOnly.FromDateTime(DateTime.Now)));
    private bool syncing;

    // The sheet's parts, kept to update in place while it is open.
    private Calendar? calendar;
    private Menu? quickMenu;
    private Microsoft.UI.Xaml.Controls.Button? quick;
    private TextBlock? startDate;
    private TextBlock? endDate;
    private Select? startTime;
    private Select? endTime;
    private readonly List<(Toggle Chip, DateRangeDuration Duration)> chips = [];
    private Toggle? allDay;
    private TextBlock? summary;

    public DateRangePicker()
    {
        trigger.Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(TriggerTemplate);
        trigger.Flyout = sheet;
        trigger.GotFocus += (_, _) => { focused = true; PaintTrigger(); };
        trigger.LostFocus += (_, _) => { focused = false; PaintTrigger(); };
        sheet.Opening += (_, _) => Open();
        sheet.Closed += (_, _) => PaintTrigger();
        IsEnabledChanged += (_, _) => { trigger.IsEnabled = IsEnabled; PaintTrigger(); };
        HorizontalAlignment = HorizontalAlignment.Stretch;
        Content = trigger;
    }

    /// <summary>The span chosen, or null while nothing is.</summary>
    public DateRangeValue? Value
    {
        get => value;
        set { this.value = value; if (IsLoaded) PaintTrigger(); }
    }
    public event EventHandler<DateRangeValue>? ValueChanged;

    /// <summary>The quick-pick menu. <see cref="DateRangePreset.Default"/> unless the product has its own.</summary>
    public IList<DateRangePreset> Presets { get; } = [.. DateRangePreset.Default];
    /// <summary>The duration chips. <see cref="DateRangeDuration.Default"/> unless the product has its own.</summary>
    public IList<DateRangeDuration> Durations { get; } = [.. DateRangeDuration.Default];
    /// <summary>Minutes between the times the lists offer.</summary>
    public int TimeStep { get; set; } = 15;
    /// <summary>Months side by side in the sheet: one, or two for a sheet that mostly picks spans across them.</summary>
    public int Months { get; set; } = 1;
    public DayOfWeek WeekStartsOn { get; set; } = DayOfWeek.Monday;
    public DateRangePickerStrings Strings { get; set; } = new();
    /// <summary>What the grid counts as today.</summary>
    public DateOnly? Today { get; set; }
    /// <summary>React's <c>state="error"</c>: the box re-tinted to danger.</summary>
    public bool Invalid { get => invalid; set { invalid = value; if (IsLoaded) PaintTrigger(); } }
    /// <summary>The native button that opens the sheet.</summary>
    public Microsoft.UI.Xaml.Controls.Button Trigger => trigger;
    /// <summary>The flyout the sheet is drawn in.</summary>
    public Flyout Sheet => sheet;

    private DateOnly TodayDay => Today ?? DateOnly.FromDateTime(DateTime.Now);

    /// <summary>The next whole hour, for an hour — what a new event defaults to.</summary>
    private static DateRangeValue Blank(DateOnly today)
    {
        var now = DateTime.Now;
        var hour = DateOnly.FromDateTime(now) == today ? Math.Min(23, now.Hour + 1) : 9;
        var start = today.ToDateTime(new TimeOnly(hour, 0));
        return new DateRangeValue(start, start.AddHours(1));
    }

    private static string Clock(DateTime at) => $"{at.Hour:00}:{at.Minute:00}";

    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        PaintTrigger();
    }

    /// <summary>The trigger: the select's box, with the calendar glyph at its leading edge and the span where the value goes.</summary>
    private void PaintTrigger()
    {
        trigger.IsEnabled = IsEnabled;
        Skin.Input(trigger, Theme, Size, Invalid);
        trigger.Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        trigger.MinWidth = 16 * 16;
        var block = Theme.Number($"control.{Theme.Key(Size)}.padding.block");
        trigger.Padding = new Thickness(Theme.Number("spacing.3"), block, Theme.Number("spacing.3"), block);
        if ((focused || sheet.IsOpen) && IsEnabled)
        {
            trigger.BorderBrush = Theme.Recipe(ControlVariant.Outlined, Invalid ? WidgetTint.Danger : Tint, "border");
            if (!Invalid) trigger.Background = Theme.Brush("color.surface");
        }
        var glyphInk = Theme.Brush(IsEnabled ? "color.content-subtle" : "color.content-faint");
        var grid = new Grid { ColumnSpacing = Theme.Number("spacing.2") };
        grid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        grid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        var icon = new Icon { Glyph = FluentIcons.Common.Icon.Calendar, GlyphSize = Theme.Number("icon.small"), Foreground = glyphInk, VerticalAlignment = VerticalAlignment.Center };
        grid.Children.Add(icon);
        var words = new TextBlock { FontSize = trigger.FontSize, FontFamily = trigger.FontFamily, FontWeight = trigger.FontWeight, VerticalAlignment = VerticalAlignment.Center, TextTrimming = TextTrimming.CharacterEllipsis, TextWrapping = TextWrapping.NoWrap, UseLayoutRounding = false };
        Typography.SetNumeralAlignment(words, FontNumeralAlignment.Tabular);
        if (value is DateRangeValue chosen) Summarize(words, chosen, trigger.Foreground);
        else { words.Text = Strings.Placeholder; words.Foreground = Theme.Brush("color.content-faint"); }
        Grid.SetColumn(words, 1);
        grid.Children.Add(words);
        var chevron = new Icon { Glyph = FluentIcons.Common.Icon.ChevronDown, GlyphSize = Theme.Number("icon.small"), Foreground = glyphInk, VerticalAlignment = VerticalAlignment.Center };
        Grid.SetColumn(chevron, 2);
        grid.Children.Add(chevron);
        trigger.Content = grid;
    }

    /// <summary>
    /// A span in one line: the day once when both ends share it, the times
    /// unless it is all day, and the joins in the faint ink so the dates and
    /// times read as the values and the arrow as punctuation.
    /// </summary>
    private void Summarize(TextBlock words, DateRangeValue range, Brush ink)
    {
        var faint = Theme.Brush("color.content-faint");
        var oneDay = range.Start.Date == range.End.Date;
        string Day(DateTime at) => Strings.DayText(DateOnly.FromDateTime(at));
        words.Inlines.Clear();
        words.Foreground = ink;
        void Word(string text) => words.Inlines.Add(new Run { Text = text });
        void Sep(string glyph) => words.Inlines.Add(new Run { Text = $" {glyph} ", Foreground = faint });
        if (range.AllDay)
        {
            Word(Day(range.Start));
            if (oneDay) { Sep("·"); Word(Strings.AllDay); }
            else { Sep("→"); Word(Day(range.End)); }
        }
        else if (oneDay)
        {
            Word(Day(range.Start)); Sep("·"); Word(Clock(range.Start)); Sep("→"); Word(Clock(range.End));
        }
        else
        {
            Word($"{Day(range.Start)} {Clock(range.Start)}"); Sep("→"); Word($"{Day(range.End)} {Clock(range.End)}");
        }
    }

    /* The sheet ----------------------------------------------------------- */

    /// <summary>Opening seeds the draft from the value; closing drops it, and only Done writes it back.</summary>
    private void Open()
    {
        draft = new DateRangeDraft(value ?? Blank(TodayDay));
        var theme = Theme;
        var style = new Style(typeof(FlyoutPresenter));
        style.Setters.Add(new Setter(Control.BackgroundProperty, theme.Brush("color.surface-overlay")));
        style.Setters.Add(new Setter(Control.ForegroundProperty, theme.Brush("color.content")));
        style.Setters.Add(new Setter(Control.BorderBrushProperty, theme.Brush("color.border-strong")));
        style.Setters.Add(new Setter(Control.BorderThicknessProperty, new Thickness(theme.Number("stroke.hairline"))));
        style.Setters.Add(new Setter(Control.CornerRadiusProperty, new CornerRadius(theme.Number("radius.large"))));
        style.Setters.Add(new Setter(Control.PaddingProperty, new Thickness(0)));
        style.Setters.Add(new Setter(FrameworkElement.MinWidthProperty, 0d));
        style.Setters.Add(new Setter(FrameworkElement.MaxWidthProperty, 4000d));
        style.Setters.Add(new Setter(FrameworkElement.MaxHeightProperty, 4000d));
        sheet.FlyoutPresenterStyle = style;
        var root = BuildSheet();
        Appearance.SetTheme(root, theme);
        root.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
        sheet.Content = root;
        PaintTrigger();
        Sync();
    }

    private void Commit()
    {
        value = draft.Value;
        sheet.Hide();
        PaintTrigger();
        ValueChanged?.Invoke(this, draft.Value);
    }

    /// <summary>The sheet: the shared popup holding the months, the column beside them and a footer, hairlined off each other.</summary>
    private FrameworkElement BuildSheet()
    {
        var theme = Theme;
        var hairline = theme.Number("stroke.hairline");
        double Step(string token) => theme.Number(token);
        var tint = Tint;

        calendar = new Calendar
        {
            Mode = CalendarMode.Range,
            Size = ControlSize.Small,
            Months = Months,
            WeekStartsOn = WeekStartsOn,
            Strings = Strings.Calendar ?? new CalendarStrings(),
            Today = TodayDay,
            Tint = tint,
            Range = draft.Days,
            Month = draft.Month,
        };
        calendar.RangeChanged += (_, range) => { draft.SetDays(range); Sync(); };
        calendar.MonthChanged += (_, month) => draft.Month = month;
        var months = new Border { Padding = new Thickness(Step("spacing.2"), Step("spacing.1"), Step("spacing.2"), Step("spacing.2")), Child = calendar };

        // The column: the quick-pick menu's name on the first line, set
        // flush with the keys under it, then a key over a date and its time,
        // twice, then the durations.
        var side = new StackPanel { Spacing = Step("spacing.3"), Width = Step("date-range-picker.side-width") };
        var sideBox = new Border
        {
            BorderBrush = theme.Brush("color.border"),
            BorderThickness = new Thickness(hairline, 0, 0, 0),
            Padding = new Thickness(Step("spacing.3"), Step("spacing.2"), Step("spacing.3"), Step("spacing.3")),
            Child = side,
        };
        quick = new Button
        {
            Size = ControlSize.Tiny,
            Variant = ControlVariant.Plain,
            Tint = WidgetTint.Neutral,
            HorizontalAlignment = HorizontalAlignment.Left,
            // A plain button pads its label by the profile's inline step;
            // pulling it back by that sets the name on the column's own edge.
            Margin = new Thickness(-Step("control.tiny.padding.inline"), 0, 0, 0),
            Content = new StackPanel
            {
                Orientation = Orientation.Horizontal,
                Children = { new TextBlock { Text = Strings.Presets, VerticalAlignment = VerticalAlignment.Center }, new Icon { Glyph = FluentIcons.Common.Icon.ChevronDown, VerticalAlignment = VerticalAlignment.Center } },
            },
        };
        quickMenu = new Menu();
        side.Children.Add(quick);

        (TextBlock Date, Select Time) End(string label, Action<int, int> onTime)
        {
            var column = new StackPanel { Spacing = Step("spacing.1") };
            column.Children.Add(Key(label));
            var row = new Grid { ColumnSpacing = Step("spacing.2") };
            row.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
            row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
            var date = theme.Text("", "labelQuiet");
            date.TextWrapping = TextWrapping.NoWrap;
            date.TextTrimming = TextTrimming.CharacterEllipsis;
            date.VerticalAlignment = VerticalAlignment.Center;
            date.LineHeight = 0;
            Typography.SetNumeralAlignment(date, FontNumeralAlignment.Tabular);
            row.Children.Add(date);
            var time = new Select { Size = ControlSize.Small, Tint = tint, MinWidth = 5 * 16, HorizontalAlignment = HorizontalAlignment.Right };
            time.SelectionChanged += (_, _) =>
            {
                if (syncing || time.Value is not string chosen) return;
                var parts = chosen.Split(':');
                if (parts.Length == 2 && int.TryParse(parts[0], out var hour) && int.TryParse(parts[1], out var minute))
                {
                    onTime(hour, minute);
                    Sync();
                }
            };
            Grid.SetColumn(time, 1);
            row.Children.Add(time);
            column.Children.Add(row);
            side.Children.Add(column);
            return (date, time);
        }
        (startDate, startTime) = End(Strings.Start, draft.SetStartTime);
        (endDate, endTime) = End(Strings.End, draft.SetEndTime);

        var durations = new StackPanel { Spacing = Step("spacing.1.5") };
        durations.Children.Add(Key(Strings.Duration));
        var chipRow = new WrapRow { Spacing = Step("spacing.1") };
        chips.Clear();
        foreach (var duration in Durations)
        {
            var chip = new Toggle { Content = duration.Label, Variant = ControlVariant.Outlined, Tint = tint };
            var minutes = duration.Minutes;
            chip.Click += (_, _) => { draft.SetDuration(minutes); Sync(); };
            chips.Add((chip, duration));
            chipRow.Children.Add(chip);
        }
        allDay = new Toggle { Content = Strings.AllDay, Variant = ControlVariant.Outlined, Tint = tint };
        allDay.Click += (_, _) => { draft.Value = draft.Value with { AllDay = allDay.IsChecked == true }; Sync(); };
        chipRow.Children.Add(allDay);
        durations.Children.Add(chipRow);
        side.Children.Add(durations);

        var body = new Grid();
        body.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        body.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        body.Children.Add(months);
        Grid.SetColumn(sideBox, 1);
        body.Children.Add(sideBox);

        // A read-only line of what the sheet holds, and the two actions at
        // the trailing edge, as a dialog sets its own.
        var foot = new Grid { ColumnSpacing = Step("spacing.2") };
        foot.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        foot.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        foot.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        summary = theme.Text("", "labelSmall");
        summary.Foreground = theme.Brush("color.content-secondary");
        summary.TextWrapping = TextWrapping.NoWrap;
        summary.TextTrimming = TextTrimming.CharacterEllipsis;
        summary.VerticalAlignment = VerticalAlignment.Center;
        summary.Margin = new Thickness(0, 0, Step("spacing.1"), 0);
        Typography.SetNumeralAlignment(summary, FontNumeralAlignment.Tabular);
        foot.Children.Add(summary);
        var cancel = new Button { Content = Strings.Cancel, Variant = ControlVariant.Normal };
        cancel.Click += (_, _) => sheet.Hide();
        Grid.SetColumn(cancel, 1);
        foot.Children.Add(cancel);
        var done = new Button { Content = Strings.Done, Variant = ControlVariant.Filled, Tint = tint };
        done.Click += (_, _) => Commit();
        Grid.SetColumn(done, 2);
        foot.Children.Add(done);
        var footBox = new Border
        {
            BorderBrush = theme.Brush("color.border"),
            BorderThickness = new Thickness(0, hairline, 0, 0),
            Padding = new Thickness(Step("spacing.3"), Step("spacing.2"), Step("spacing.2"), Step("spacing.2")),
            Child = foot,
        };

        var root = new StackPanel { UseLayoutRounding = false };
        root.Children.Add(body);
        root.Children.Add(footBox);
        return root;
    }

    /// <summary>The caption-voiced key over a date, a time or the durations.</summary>
    private TextBlock Key(string text)
    {
        var key = Theme.Text(text.ToUpperInvariant(), "captionSmall");
        key.FontWeight = Theme.Weight("labelSmall");
        key.Foreground = Theme.Brush("color.content-subtle");
        key.CharacterSpacing = 60;
        key.LineHeight = 0;
        key.TextLineBounds = TextLineBounds.Tight;
        return key;
    }

    /// <summary>The time list: every step of the day, and the value shown when it is off the step, so a span handed in at 14:07 keeps 14:07 until the reader moves it.</summary>
    private IEnumerable<SelectOption> Times(DateTime at)
    {
        var current = Clock(at);
        if ((at.Hour * 60 + at.Minute) % TimeStep != 0) yield return new SelectOption(current, current);
        for (var minutes = 0; minutes < 24 * 60; minutes += TimeStep)
        {
            var label = $"{minutes / 60:00}:{minutes % 60:00}";
            yield return new SelectOption(label, label);
        }
    }

    /// <summary>Brings every part of the open sheet to the draft, in place.</summary>
    private void Sync()
    {
        if (calendar is null || quick is null || quickMenu is null || startDate is null || endDate is null || startTime is null || endTime is null || allDay is null || summary is null) return;
        syncing = true;
        try
        {
            var today = TodayDay;
            calendar.Range = draft.Days;
            calendar.Month = draft.Month;
            quickMenu.Items.Clear();
            foreach (var preset in Presets)
            {
                var chosen = preset;
                quickMenu.Items.Add(new MenuItem(preset.Label, () =>
                {
                    var (start, end) = chosen.Range(today);
                    draft.Month = Calendar.StartOfMonth(start);
                    draft.SetDays(new DateRange(start, end));
                    Sync();
                }, Checked: draft.Matches(preset, today)));
            }
            quickMenu.Attach(quick);
            startDate.Text = Strings.DayText(DateOnly.FromDateTime(draft.Value.Start));
            endDate.Text = Strings.DayText(DateOnly.FromDateTime(draft.Value.End));
            startTime.SetOptions(Times(draft.Value.Start));
            startTime.Value = Clock(draft.Value.Start);
            startTime.IsEnabled = !draft.Value.AllDay;
            endTime.SetOptions(Times(draft.Value.End));
            endTime.Value = Clock(draft.Value.End);
            endTime.IsEnabled = !draft.Value.AllDay;
            foreach (var (chip, duration) in chips)
            {
                chip.IsChecked = draft.Matches(duration);
                chip.IsEnabled = draft.OneDay;
            }
            allDay.IsChecked = draft.Value.AllDay;
            Summarize(summary, draft.Value, Theme.Brush("color.content-secondary"));
        }
        finally { syncing = false; }
    }
}

/// <summary>A run of chips that wraps — CSS's <c>flex-wrap</c>, which the stack panels lack.</summary>
public sealed class WrapRow : Panel
{
    public double Spacing { get; set; }

    protected override Size MeasureOverride(Size available)
    {
        double x = 0, y = 0, row = 0, widest = 0;
        foreach (var child in Children)
        {
            child.Measure(new Size(double.PositiveInfinity, double.PositiveInfinity));
            var size = child.DesiredSize;
            if (x > 0 && x + size.Width > available.Width) { x = 0; y += row + Spacing; row = 0; }
            x += size.Width + Spacing;
            row = Math.Max(row, size.Height);
            widest = Math.Max(widest, x - Spacing);
        }
        return new Size(double.IsInfinity(available.Width) ? widest : Math.Min(widest, available.Width), y + row);
    }

    protected override Size ArrangeOverride(Size final)
    {
        double x = 0, y = 0, row = 0;
        foreach (var child in Children)
        {
            var size = child.DesiredSize;
            if (x > 0 && x + size.Width > final.Width) { x = 0; y += row + Spacing; row = 0; }
            child.Arrange(new Rect(x, y, size.Width, size.Height));
            x += size.Width + Spacing;
            row = Math.Max(row, size.Height);
        }
        return final;
    }
}
