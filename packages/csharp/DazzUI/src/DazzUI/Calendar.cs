using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Shapes;
using Windows.System;

namespace DazzUI;

/// <summary>
/// A span of days. <see cref="End"/> is null while the first day has been
/// picked and the second has not — a range mid-pick is still a value,
/// because the picker around the calendar wants to show the start while the
/// reader chooses the end.
/// </summary>
public readonly record struct DateRange(DateOnly? Start = null, DateOnly? End = null)
{
    /// <summary>The first day picked, the second still to come.</summary>
    public bool IsPending => Start is not null && End is null;
}

public enum CalendarMode { Single, Range }

/// <summary>
/// The words a calendar is labelled with. English unless the product says
/// otherwise; the kit carries no locale of its own, and a product hands it
/// the formatters for the one it speaks.
/// </summary>
public sealed record CalendarStrings(
    Func<DateOnly, string>? MonthTitle = null,
    Func<DayOfWeek, string>? Weekday = null,
    Func<DateOnly, string>? DayLabel = null,
    string PreviousMonth = "Previous month",
    string NextMonth = "Next month")
{
    internal static readonly string[] Months =
    [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];
    internal static readonly string[] Weekdays =
    [
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
    ];

    public string Title(DateOnly month) => (MonthTitle ?? (m => $"{Months[m.Month - 1]} {m.Year}"))(month);
    public string Narrow(DayOfWeek day) => (Weekday ?? (d => Weekdays[(int)d][..1]))(day);
    public string Label(DateOnly day) =>
        (DayLabel ?? (d => $"{Weekdays[(int)d.DayOfWeek]}, {Months[d.Month - 1]} {d.Day}, {d.Year}"))(day);
}

/// <summary>
/// A month grid, in one of two modes: one day, or a span of them.
///
/// WinUI's own <c>CalendarView</c> is Windows' drawing, not this design's,
/// so this is one of the few controls that writes its behaviour out, and the
/// reason is written here: a grid's keyboard is not a list's. The arrow keys
/// move by a day and by a week, Page Up and Page Down by a month, Home and
/// End to the ends of the week, and the whole grid is one tab stop that
/// lands on the day last reached. Picking is a press, Enter or Space; in
/// range mode the first press sets the start and the second the end, in
/// either order, and while the end is pending the day under the pointer or
/// the keyboard previews the span.
///
/// Every day is a control, so it wears the shared recipes: <c>tinted</c>
/// inside a range — the system's one drawing for a selection — and
/// <c>filled</c> for the day or the two days actually picked. At rest it is
/// drawn as a row, in ink with the neutral hover wash, the way a menu row
/// is.
/// </summary>
public sealed class Calendar : Surface
{
    private readonly StackPanel months = new() { Orientation = Orientation.Horizontal };
    // A day stands in two grids when it opens or closes a month beside its
    // own, so the cells are a list, each knowing its day.
    private readonly List<DayCell> cells = [];
    private DateOnly? value;
    private DateRange range;
    private DateOnly? month;
    private DateOnly? focused;
    private DateOnly? hovered;
    private bool building;

    public Calendar()
    {
        Content = months;
        HorizontalAlignment = HorizontalAlignment.Left;
        VerticalAlignment = VerticalAlignment.Top;
        IsTabStop = false;
        KeyDown += OnKeyDown;
        PointerExited += (_, _) => { if (hovered is not null) { hovered = null; Paint(); } };
    }

    public CalendarMode Mode { get; set; }

    /// <summary>The day picked, in single mode.</summary>
    public DateOnly? Value
    {
        get => value;
        set { this.value = value; if (IsLoaded) Paint(); }
    }
    public event EventHandler<DateOnly>? ValueChanged;

    /// <summary>The span picked, in range mode.</summary>
    public DateRange Range
    {
        get => range;
        set { range = value; if (IsLoaded) Paint(); }
    }
    public event EventHandler<DateRange>? RangeChanged;

    /// <summary>How many months stand side by side: one or two.</summary>
    public int Months { get; set; } = 1;

    /// <summary>The month shown — the first of two. Any day in it will do; null lets the grid keep its own.</summary>
    public DateOnly? Month
    {
        get => month;
        set
        {
            var next = value is DateOnly m ? new DateOnly(m.Year, m.Month, 1) : (DateOnly?)null;
            if (next == month) return;
            month = next;
            if (IsLoaded) Build();
        }
    }
    public event EventHandler<DateOnly>? MonthChanged;

    /// <summary>Days before <see cref="Min"/> and after <see cref="Max"/> are drawn but cannot be picked.</summary>
    public DateOnly? Min { get; set; }
    public DateOnly? Max { get; set; }

    /// <summary>Monday by default, as ISO counts the week.</summary>
    public DayOfWeek WeekStartsOn { get; set; } = DayOfWeek.Monday;

    public CalendarStrings Strings { get; set; } = new();

    /// <summary>What the grid counts as today. The clock's, unless a test says otherwise.</summary>
    public DateOnly? Today { get; set; }

    /// <summary>Whether the head carries the chevrons that move the month. Off for a calendar whose host moves it.</summary>
    public bool Navigation { get; set; } = true;

    private DateOnly TodayDay => Today ?? DateOnly.FromDateTime(DateTime.Now);

    /// <summary>The value, read as a range either way: a single day is a range whose two ends are the same day.</summary>
    private DateRange Span => Mode == CalendarMode.Range ? range : new DateRange(value, value);

    private bool Pending => Mode == CalendarMode.Range && range.IsPending;

    private DateOnly FirstMonth
    {
        get
        {
            var first = month ?? Span.Start ?? TodayDay;
            return new DateOnly(first.Year, first.Month, 1);
        }
    }

    private DateOnly Focused => focused ?? Span.Start ?? TodayDay;

    /* Day arithmetic ------------------------------------------------------ */

    internal static DateOnly StartOfMonth(DateOnly day) => new(day.Year, day.Month, 1);

    /// <summary>The first of the month <paramref name="months"/> away, so the day never overflows.</summary>
    internal static DateOnly AddMonths(DateOnly day, int months) => StartOfMonth(day).AddMonths(months);

    /// <summary>The same day of another month, or that month's last day.</summary>
    internal static DateOnly SameDayIn(DateOnly month, int day) =>
        new(month.Year, month.Month, Math.Min(day, DateTime.DaysInMonth(month.Year, month.Month)));

    private int OffsetInWeek(DateOnly day) => ((int)day.DayOfWeek - (int)WeekStartsOn + 7) % 7;

    /// <summary>The six weeks a month grid draws — always forty-two days, so the grid never changes height.</summary>
    private IEnumerable<DateOnly> MonthGrid(DateOnly month)
    {
        var first = StartOfMonth(month);
        var start = first.AddDays(-OffsetInWeek(first));
        return Enumerable.Range(0, 42).Select(start.AddDays);
    }

    private bool Visible(DateOnly day) =>
        Enumerable.Range(0, Months).Any(i => { var m = AddMonths(FirstMonth, i); return m.Year == day.Year && m.Month == day.Month; });

    private bool Disabled(DateOnly day) => (Min is DateOnly min && day < min) || (Max is DateOnly max && day > max);

    /* Behaviour ------------------------------------------------------------- */

    private void SetMonth(DateOnly next)
    {
        var first = StartOfMonth(next);
        month = first;
        Build();
        MonthChanged?.Invoke(this, first);
    }

    private void Pick(DateOnly day)
    {
        if (Disabled(day)) return;
        if (Mode == CalendarMode.Range)
        {
            var current = range;
            var next = current.Start is null || current.End is not null ? new DateRange(day)
                : day < current.Start ? new DateRange(day, current.Start)
                : new DateRange(current.Start, day);
            range = next;
            Paint();
            RangeChanged?.Invoke(this, next);
        }
        else
        {
            value = day;
            Paint();
            ValueChanged?.Invoke(this, day);
        }
    }

    private void MoveFocus(DateOnly day)
    {
        focused = day;
        if (!Visible(day))
        {
            // Reached past the shown months: slide them so the day is in the
            // first one going back, or the last one going forward.
            var forward = StartOfMonth(day) > FirstMonth;
            SetMonth(forward ? AddMonths(day, 1 - Months) : day);
        }
        else Paint();
        // The day in its own month, where it stands in two grids.
        var cell = cells.FirstOrDefault(c => c.Day == day && !c.Outside) ?? cells.FirstOrDefault(c => c.Day == day);
        cell?.Focus(FocusState.Keyboard);
    }

    private void OnKeyDown(object sender, KeyRoutedEventArgs args)
    {
        var day = Focused;
        var shift = Microsoft.UI.Input.InputKeyboardSource.GetKeyStateForCurrentThread(VirtualKey.Shift).HasFlag(Windows.UI.Core.CoreVirtualKeyStates.Down);
        var offset = OffsetInWeek(day);
        DateOnly? next = args.Key switch
        {
            VirtualKey.Left => day.AddDays(-1),
            VirtualKey.Right => day.AddDays(1),
            VirtualKey.Up => day.AddDays(-7),
            VirtualKey.Down => day.AddDays(7),
            VirtualKey.PageUp => SameDayIn(AddMonths(day, shift ? -12 : -1), day.Day),
            VirtualKey.PageDown => SameDayIn(AddMonths(day, shift ? 12 : 1), day.Day),
            VirtualKey.Home => day.AddDays(-offset),
            VirtualKey.End => day.AddDays(6 - offset),
            _ => null,
        };
        if (next is not DateOnly target) return;
        MoveFocus(target);
        args.Handled = true;
    }

    /* Drawing ---------------------------------------------------------------- */

    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        Build();
    }

    private double Cell => Theme.Number(Size == ControlSize.Small ? "calendar.small.cell-size" : "calendar.medium.cell-size");

    /// <summary>Lays the months out afresh — on a theme change and when the month shown moves.</summary>
    private void Build()
    {
        if (building) return;
        building = true;
        try
        {
            months.Spacing = Theme.Number("spacing.4");
            months.Children.Clear();
            cells.Clear();
            var cell = Cell;
            var gap = Theme.Number("calendar.cell-gap");
            for (var index = 0; index < Months; index++)
            {
                var current = AddMonths(FirstMonth, index);
                var column = new StackPanel { Spacing = Theme.Number("spacing.1"), Width = 7 * cell + 6 * gap };
                column.Children.Add(Head(current, index));
                column.Children.Add(WeekdayRow(cell, gap));
                var grid = new Grid { ColumnSpacing = gap, RowSpacing = gap };
                for (var c = 0; c < 7; c++) grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(cell) });
                for (var r = 0; r < 6; r++) grid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(cell) });
                var i = 0;
                foreach (var day in MonthGrid(current))
                {
                    var dayCell = new DayCell(this, day, current, cell);
                    Grid.SetRow(dayCell, i / 7);
                    Grid.SetColumn(dayCell, i % 7);
                    grid.Children.Add(dayCell);
                    cells.Add(dayCell);
                    i++;
                }
                column.Children.Add(grid);
                months.Children.Add(column);
            }
            Paint();
        }
        finally { building = false; }
    }

    /// <summary>The bar: a chevron at each end of the month's name, in the tiny glyph square. A middle month of a run has neither, and keeps the room.</summary>
    private UIElement Head(DateOnly current, int index)
    {
        var head = new Grid { ColumnSpacing = Theme.Number("spacing.1") };
        head.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        head.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        head.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        var tiny = Theme.Number("control.tiny.size");
        FrameworkElement Spacer() => new Border { Width = tiny, Height = tiny };
        FrameworkElement Chevron(Glyph glyph, string label, int step)
        {
            var button = new IconButton { Glyph = glyph, Label = label };
            button.Click += (_, _) => SetMonth(AddMonths(FirstMonth, step));
            return button;
        }
        if (Navigation)
        {
            var previous = index == 0 ? Chevron(FluentIcons.Common.Icon.ChevronLeft, Strings.PreviousMonth, -1) : Spacer();
            Grid.SetColumn(previous, 0);
            head.Children.Add(previous);
            var next = index == Months - 1 ? Chevron(FluentIcons.Common.Icon.ChevronRight, Strings.NextMonth, 1) : Spacer();
            Grid.SetColumn(next, 2);
            head.Children.Add(next);
        }
        var title = Theme.Text(Strings.Title(current), "labelMedium");
        title.TextAlignment = TextAlignment.Center;
        title.TextWrapping = TextWrapping.NoWrap;
        title.VerticalAlignment = VerticalAlignment.Center;
        Microsoft.UI.Xaml.Documents.Typography.SetNumeralAlignment(title, FontNumeralAlignment.Tabular);
        Grid.SetColumn(title, 1);
        head.Children.Add(title);
        return head;
    }

    private UIElement WeekdayRow(double cell, double gap)
    {
        var row = new Grid { ColumnSpacing = gap };
        for (var i = 0; i < 7; i++)
        {
            row.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(cell) });
            var day = (DayOfWeek)(((int)WeekStartsOn + i) % 7);
            var name = Theme.Text(Strings.Narrow(day), "captionSmall");
            name.FontWeight = Theme.Weight("labelSmall");
            name.Foreground = Theme.Brush("color.content-subtle");
            name.LineHeight = 0;
            name.TextLineBounds = TextLineBounds.Tight;
            name.HorizontalAlignment = HorizontalAlignment.Center;
            name.VerticalAlignment = VerticalAlignment.Center;
            var box = new Border { Height = Theme.Number("spacing.5"), Child = name };
            Grid.SetColumn(box, i);
            row.Children.Add(box);
        }
        return row;
    }

    /// <summary>Repaints every day from the value, the preview and the focus, without laying the grid out again.</summary>
    private void Paint()
    {
        if (cells.Count == 0) return;
        var span = Span;
        var pending = Pending;
        var today = TodayDay;
        var focusedDay = Focused;
        // What the span to draw is: the two picked ends, or the one picked
        // end and the day being previewed against it.
        var preview = pending ? hovered ?? focusedDay : (DateOnly?)null;
        DateOnly? lo = null, hi = null;
        if (Mode == CalendarMode.Range && span.Start is DateOnly a && (pending ? preview : span.End) is DateOnly b)
        {
            lo = a <= b ? a : b;
            hi = a <= b ? b : a;
        }
        // The one tab stop is the focused day, if it is shown; otherwise
        // today, or failing that the first day shown.
        var stop = Visible(focusedDay) ? focusedDay : Visible(today) ? today : cells.Min(c => c.Day);
        foreach (var cell in cells)
        {
            var day = cell.Day;
            var isStart = day == span.Start;
            var isEnd = day == span.End;
            var picked = Mode == CalendarMode.Range ? isStart || isEnd : isStart;
            var inSpan = lo is DateOnly l && hi is DateOnly h && day >= l && day <= h;
            cell.Apply(new DayState(
                Picked: picked,
                InSpan: inSpan,
                SpanStart: inSpan && day == lo,
                SpanEnd: inSpan && day == hi,
                Previewed: pending && preview == day && !isStart,
                Today: day == today,
                Disabled: Disabled(day),
                Stop: day == stop && !cell.Outside));
        }
    }

    internal void Hover(DateOnly day)
    {
        if (Mode != CalendarMode.Range || hovered == day) return;
        hovered = day;
        Paint();
    }

    internal void Press(DateOnly day)
    {
        focused = day;
        Pick(day);
    }

    internal void FocusDay(DateOnly day)
    {
        if (focused == day) return;
        focused = day;
        Paint();
    }

    private readonly record struct DayState(bool Picked, bool InSpan, bool SpanStart, bool SpanEnd, bool Previewed, bool Today, bool Disabled, bool Stop);

    /// <summary>
    /// One day: a square at the cell size, drawn by its state. A native
    /// button, so Enter and Space press it and automation sees a button;
    /// the states are painted here, through the shared recipes.
    /// </summary>
    private sealed class DayCell : Microsoft.UI.Xaml.Controls.Button
    {
        private const string CellTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='Button'><Border Background='{TemplateBinding Background}' CornerRadius='{TemplateBinding CornerRadius}'><ContentPresenter Content='{TemplateBinding Content}' Foreground='{TemplateBinding Foreground}' HorizontalContentAlignment='Stretch' VerticalContentAlignment='Stretch'/></Border></ControlTemplate>";
        private readonly Calendar calendar;
        private readonly DateOnly day;
        private readonly TextBlock label;
        private readonly Ellipse dot;
        private readonly Rectangle preview;
        private DayState state;
        private bool hovered;
        private bool pressed;

        public DateOnly Day => day;
        public bool Outside { get; }

        public DayCell(Calendar calendar, DateOnly day, DateOnly month, double size)
        {
            this.calendar = calendar;
            this.day = day;
            Outside = day.Month != month.Month || day.Year != month.Year;
            var theme = calendar.Theme;
            Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(CellTemplate);
            Width = Height = size;
            MinWidth = MinHeight = 0;
            Padding = new Thickness(0);
            BorderThickness = new Thickness(0);
            UseLayoutRounding = false;
            label = theme.Text(day.Day.ToString(), "labelQuiet");
            label.LineHeight = 0;
            label.TextLineBounds = TextLineBounds.Tight;
            label.HorizontalAlignment = HorizontalAlignment.Center;
            label.VerticalAlignment = VerticalAlignment.Center;
            Microsoft.UI.Xaml.Documents.Typography.SetNumeralAlignment(label, FontNumeralAlignment.Tabular);
            var dotSize = theme.Number("spacing.0.5");
            dot = new Ellipse { Width = dotSize, Height = dotSize, VerticalAlignment = VerticalAlignment.Bottom, HorizontalAlignment = HorizontalAlignment.Center, Margin = new Thickness(0, 0, 0, dotSize), Visibility = Visibility.Collapsed };
            preview = new Rectangle { StrokeThickness = theme.Number("stroke.hairline"), StrokeDashArray = new DoubleCollection { 3, 2 }, Visibility = Visibility.Collapsed, IsHitTestVisible = false };
            Content = new Grid { Children = { label, dot, preview } };
            Skin.Focus(this, theme);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, calendar.Strings.Label(day));
            Click += (_, _) => calendar.Press(day);
            GotFocus += (_, _) => calendar.FocusDay(day);
            PointerEntered += (_, _) => { hovered = true; calendar.Hover(day); Paint(); };
            PointerExited += (_, _) => { hovered = false; pressed = false; Paint(); };
            PointerPressed += (_, _) => { pressed = true; Paint(); };
            PointerReleased += (_, _) => { pressed = false; Paint(); };
            PointerCaptureLost += (_, _) => { pressed = false; Paint(); };
        }

        public void Apply(DayState next)
        {
            state = next;
            IsTabStop = next.Stop;
            // A disabled day stays a button the keyboard can reach — the
            // grid moves over it — but a press does nothing and the cursor
            // says so.
            IsHitTestVisible = true;
            Paint();
        }

        private void Paint()
        {
            var theme = calendar.Theme;
            var tint = calendar.Tint;
            var radius = theme.Number("radius.small");
            // The span between the two ends is one band: the days inside
            // square their corners, and the two ends keep the corner only on
            // the outer side.
            var start = !state.InSpan || state.SpanStart ? radius : 0;
            var end = !state.InSpan || state.SpanEnd ? radius : 0;
            CornerRadius = new CornerRadius(start, end, end, start);
            var off = state.Disabled;
            var recipeState = off ? "disabled" : pressed ? "pressed" : hovered ? "hovered" : "normal";
            Brush surface;
            Brush content;
            if (state.Picked || state.InSpan)
            {
                var variant = state.Picked ? ControlVariant.Filled : ControlVariant.Tinted;
                surface = theme.Recipe(variant, tint, "surface", recipeState);
                content = theme.Recipe(variant, tint, "content", recipeState);
            }
            else
            {
                // At rest: the row's drawing, not the quiet accent control's.
                surface = !off && (hovered || pressed)
                    ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", pressed ? "pressed" : "hovered")
                    : new SolidColorBrush(Microsoft.UI.Colors.Transparent);
                content = off || Outside ? theme.Brush("color.content-faint")
                    : state.Today ? theme.Brush($"color.{Theme.Key(tint)}.600")
                    : theme.Brush("color.content");
            }
            Background = surface;
            Foreground = content;
            label.Foreground = content;
            // Today is a weight heavier, and a picked day the selection's
            // weight — the half of a selection that survives on a monochrome
            // theme.
            label.FontWeight = theme.Weight(state.Today ? "labelStrong" : state.Picked ? "labelMedium" : "labelQuiet");
            dot.Fill = content;
            dot.Visibility = state.Today ? Visibility.Visible : Visibility.Collapsed;
            // The end a pending range is previewing against: dashed, because
            // it is where the span would end, not where it does.
            preview.Visibility = state.Previewed ? Visibility.Visible : Visibility.Collapsed;
            preview.Stroke = theme.Brush($"color.{Theme.Key(tint)}.600");
            preview.RadiusX = preview.RadiusY = radius;
            ProtectedCursor = Microsoft.UI.Input.InputSystemCursor.Create(off ? Microsoft.UI.Input.InputSystemCursorShape.UniversalNo : Microsoft.UI.Input.InputSystemCursorShape.Hand);
        }
    }
}
