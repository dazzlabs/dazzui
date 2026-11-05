using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Animation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Windows.Foundation;
using Windows.UI.ViewManagement;

namespace DazzUI;

/// <summary>The native animation preference, observed only by mounted feedback.</summary>
internal static class FeedbackMotion
{
    private static readonly UISettings Settings = new();
    private static int testEnabled = -1;
    private static event Action? Changed;
    internal static bool Enabled => Volatile.Read(ref testEnabled) is var value && value >= 0 ? value != 0 : Settings.AnimationsEnabled;
    static FeedbackMotion() => Settings.AnimationsEnabledChanged += (_, _) => Changed?.Invoke();
    internal static void Follow(FrameworkElement element, Action update, Action stop)
    {
        void ChangedOnDispatcher() => element.DispatcherQueue.TryEnqueue(() => { if (element.IsLoaded) update(); });
        element.Loaded += (_, _) => { Changed += ChangedOnDispatcher; update(); };
        element.Unloaded += (_, _) => { Changed -= ChangedOnDispatcher; stop(); };
    }
    internal static void SetEnabledForTest(bool? enabled)
    {
        Volatile.Write(ref testEnabled, enabled is null ? -1 : enabled.Value ? 1 : 0);
        Changed?.Invoke();
    }
    internal static KeySpline EaseInOut() => new() { ControlPoint1 = new Point(.42, 0), ControlPoint2 = new Point(.58, 1) };
    internal static KeySpline Easing(Theme theme)
    {
        var curve = theme.Token("motion.easing");
        return new KeySpline { ControlPoint1 = new Point(curve[0].GetDouble(), curve[1].GetDouble()), ControlPoint2 = new Point(curve[2].GetDouble(), curve[3].GetDouble()) };
    }
    internal static Storyboard Between(DependencyObject target, string property, double from, double to, TimeSpan duration, KeySpline curve, bool repeat = false)
    {
        var animation = new DoubleAnimationUsingKeyFrames { EnableDependentAnimation = true, Duration = duration };
        animation.KeyFrames.Add(new DiscreteDoubleKeyFrame { KeyTime = TimeSpan.Zero, Value = from });
        animation.KeyFrames.Add(new SplineDoubleKeyFrame { KeyTime = duration, Value = to, KeySpline = curve });
        if (repeat) animation.RepeatBehavior = RepeatBehavior.Forever;
        Storyboard.SetTarget(animation, target);
        Storyboard.SetTargetProperty(animation, property);
        return new Storyboard { Children = { animation } };
    }
}

/// <summary>
/// A rule cut into the surface: a sunken groove with a soft square corner,
/// and a fill hard-clipped against it. React's <c>Progress</c>.
/// </summary>
public class Progress : Surface
{
    private readonly Grid track = new();
    private readonly Border fill = new() { HorizontalAlignment = HorizontalAlignment.Left };
    private readonly TranslateTransform sweep = new();
    private Storyboard? sweeping;
    private Storyboard? changing;
    private readonly System.Diagnostics.Stopwatch sweepClock = new();
    private double sweepWidth;
    private double targetWidth = double.NaN;
    private int changeVersion;
    private double value, minimum, maximum = 100;
    private bool indeterminate = true;
    public double Value { get => value; set { this.value = value; indeterminate = false; ValueChanged(); } }
    public double Minimum { get => minimum; set { minimum = value; ValueChanged(); } }
    public double Maximum { get => maximum; set { maximum = value; ValueChanged(); } }
    public bool IsIndeterminate { get => indeterminate; set { indeterminate = value; Arrange(); } }
    public bool Gradient { get; set; }
    public Progress()
    {
        Size = ControlSize.Small;
        track.Children.Add(fill);
        fill.RenderTransform = sweep;
        Content = track;
        track.SizeChanged += (_, _) => Arrange();
        FeedbackMotion.Follow(this, () => Arrange(), StopMotion);
    }
    /// <summary>The groove and fill, for a subclass that sets a head above them.</summary>
    protected FrameworkElement Track => track;
    protected virtual bool AnimateValue => true;
    protected virtual void UpdateReading() { }
    private void ValueChanged() { Arrange(animate: true); if (IsLoaded) UpdateReading(); }
    protected override void RefreshAppearance()
    {
        track.Background = Theme.Brush("color.surface-sunken");
        var height = Theme.Number(Size is ControlSize.Large ? "spacing.2" : Size is ControlSize.Medium ? "spacing.1.5" : "spacing.1");
        track.Height = height;
        track.CornerRadius = new CornerRadius(height / 2);
        fill.CornerRadius = new CornerRadius(0);
        fill.Background = Gradient
            ? new LinearGradientBrush
            {
                StartPoint = new Windows.Foundation.Point(0, .5),
                EndPoint = new Windows.Foundation.Point(1, .5),
                GradientStops = { new GradientStop { Color = Theme.Color("progress.gradient.from"), Offset = 0 }, new GradientStop { Color = Theme.Color("progress.gradient.to"), Offset = 1 } },
            }
            : Theme.Brush($"color.{Theme.Key(Tint)}.600");
        Arrange();
    }
    private void Arrange(bool animate = false)
    {
        var width = track.ActualWidth;
        if (indeterminate)
        {
            // An indeterminate bar has no width to show, so it sweeps instead.
            StopChange();
            targetWidth = double.NaN;
            fill.Width = width * 0.4;
            if (!IsLoaded || !FeedbackMotion.Enabled || width <= 0)
            {
                StopSweep();
                return;
            }
            if (sweeping is null || sweepWidth != width)
            {
                sweeping?.Stop();
                sweepWidth = width;
                // The web's 1.2s ease-in-out sweep is relative to the current
                // width. Resize preserves its phase instead of its old end.
                var duration = TimeSpan.FromSeconds(1.2);
                if (!sweepClock.IsRunning) sweepClock.Restart();
                sweeping = FeedbackMotion.Between(sweep, "X", -width * .4, width, duration, FeedbackMotion.EaseInOut(), repeat: true);
                sweeping.Begin();
                sweeping.SeekAlignedToLastTick(TimeSpan.FromMilliseconds(sweepClock.Elapsed.TotalMilliseconds % duration.TotalMilliseconds));
            }
            return;
        }
        StopSweep();
        var span = maximum - minimum;
        var next = span <= 0 ? 0 : width * Math.Clamp((value - minimum) / span, 0, 1);
        if (next == targetWidth && FeedbackMotion.Enabled) return;
        var from = fill.ActualWidth;
        var hadValue = !double.IsNaN(targetWidth);
        StopChange();
        targetWidth = next;
        if (animate && hadValue && AnimateValue && IsLoaded && FeedbackMotion.Enabled && from != next)
        {
            var duration = TimeSpan.FromMilliseconds(Theme.Milliseconds("motion.duration") * 2);
            fill.Width = next;
            changing = FeedbackMotion.Between(fill, "Width", from, next, duration, FeedbackMotion.Easing(Theme));
            var version = changeVersion;
            changing.Completed += (_, _) => { if (version == changeVersion) { StopChange(); fill.Width = next; } };
            changing.Begin();
        }
        else fill.Width = next;
    }
    private void StopSweep()
    {
        sweeping?.Stop();
        sweeping = null;
        sweepClock.Reset();
        sweep.X = 0;
    }
    private void StopChange()
    {
        changeVersion++;
        changing?.Stop();
        changing = null;
    }
    private void StopMotion()
    {
        StopSweep();
        StopChange();
        targetWidth = double.NaN;
    }
    protected override AutomationPeer OnCreateAutomationPeer() => new ProgressPeer(this);
    private sealed class ProgressPeer(Progress owner) : FrameworkElementAutomationPeer(owner), IRangeValueProvider
    {
        public bool IsReadOnly => true;
        public double Minimum => owner.Minimum;
        public double Maximum => owner.Maximum;
        public double Value => owner.Value;
        public double SmallChange => 0;
        public double LargeChange => 0;
        public void SetValue(double value) => throw new InvalidOperationException("A progress bar is read, not set.");
        protected override object GetPatternCore(PatternInterface pattern) => pattern == PatternInterface.RangeValue ? this : base.GetPatternCore(pattern);
        protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.ProgressBar;
    }
}
/// <summary>
/// The same groove a progress bar is, read rather than watched: its label
/// and its number on a line above it, and a fill that never moves.
/// </summary>
public sealed class Meter : Progress
{
    private readonly Grid head = new() { ColumnDefinitions = { new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) }, new ColumnDefinition { Width = GridLength.Auto } } };
    private readonly TextBlock label = new() { TextTrimming = TextTrimming.CharacterEllipsis };
    private readonly TextBlock reading = new();
    private readonly StackPanel body = new();
    public double Low { get; set; } = 25;
    public double High { get; set; } = 75;
    /// <summary>What it measures, over the groove's start.</summary>
    public string? Label { get; set; }
    /// <summary>The reading, over the groove's end: the value as a percentage when this is on.</summary>
    public bool ShowValue { get; set; }
    protected override bool AnimateValue => false;
    protected override void UpdateReading()
    {
        var span = Maximum - Minimum;
        reading.Text = ShowValue ? $"{Math.Round(span <= 0 ? 0 : Math.Clamp((Value - Minimum) / span, 0, 1) * 100)}%" : "";
    }
    public Meter()
    {
        IsIndeterminate = false;
        Size = ControlSize.Medium;
        Grid.SetColumn(reading, 1);
        head.Children.Add(label);
        head.Children.Add(reading);
        var track = Track;
        Content = null;
        body.Children.Add(head);
        body.Children.Add(track);
        Content = body;
    }
    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        head.Visibility = Label is null && !ShowValue ? Visibility.Collapsed : Visibility.Visible;
        body.Spacing = Theme.Number("spacing.2");
        head.ColumnSpacing = Theme.Number("spacing.3");
        label.Text = Label ?? "";
        label.Foreground = Theme.Brush("color.content");
        label.IsTextScaleFactorEnabled = reading.IsTextScaleFactorEnabled = Theme.Sizing != ThemeSizing.System;
        label.FontSize = Theme.Text("", "labelSmall").FontSize;
        label.FontFamily = PlatformFonts.Ui;
        TextStyle.SetWeight(label, Theme.Weight("labelMedium"));
        label.LineHeight = label.FontSize;
        label.LineStackingStrategy = LineStackingStrategy.BlockLineHeight;
        UpdateReading();
        reading.Foreground = Theme.Brush("color.content-muted");
        reading.FontSize = label.FontSize;
        reading.FontFamily = Theme.CodeFont;
        reading.LineHeight = reading.FontSize;
        reading.LineStackingStrategy = LineStackingStrategy.BlockLineHeight;
    }
}
/// <summary>
/// A ring with one segment picked out: the track the tint at quarter strength,
/// the head the tint itself, turning once a second.
/// </summary>
public sealed class Spinner : Surface
{
    private readonly Microsoft.UI.Xaml.Shapes.Ellipse track = new();
    private readonly Microsoft.UI.Xaml.Shapes.Path head = new();
    private readonly Grid ring = new();
    private readonly RotateTransform turn = new();
    private Storyboard? turning;
    private double turnSeconds;
    /// <summary>Drawn on an accent fill, in the on-accent ink.</summary>
    public bool OnAccent { get; set; }
    public Spinner()
    {
        Size = ControlSize.Medium;
        ring.Children.Add(track);
        ring.Children.Add(head);
        ring.RenderTransform = turn;
        ring.RenderTransformOrigin = new Windows.Foundation.Point(.5, .5);
        Content = ring;
        FeedbackMotion.Follow(this, Turn, StopTurning);
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, "Loading");
    }
    private void Turn()
    {
        // React keeps the spinner moving under reduced motion: 2.4s instead
        // of 1s. A frozen ring otherwise looks like stalled work.
        var seconds = FeedbackMotion.Enabled ? 1 : 2.4;
        if (turning is not null && seconds == turnSeconds) return;
        StopTurning();
        turnSeconds = seconds;
        var run = new DoubleAnimation { From = 0, To = 360, Duration = TimeSpan.FromSeconds(seconds), RepeatBehavior = RepeatBehavior.Forever };
        Storyboard.SetTarget(run, turn);
        Storyboard.SetTargetProperty(run, "Angle");
        turning = new Storyboard { Children = { run } };
        turning.Begin();
    }
    private void StopTurning() { turning?.Stop(); turning = null; }
    protected override void RefreshAppearance()
    {
        var size = Theme.Number(Size is ControlSize.Small or ControlSize.Tiny ? "spacing.3.5" : Size == ControlSize.Large ? "spacing.5" : "spacing.4");
        var stroke = Theme.Number("spacing.0.5");
        Width = Height = ring.Width = ring.Height = size;
        var ink = OnAccent ? Theme.Color("color.on-accent") : Theme.Color($"color.{Theme.Key(Tint)}.600");
        var wash = ink; wash.A = (byte)Math.Round(ink.A * (OnAccent ? 0.35 : 0.25));
        track.Stroke = new SolidColorBrush(wash);
        track.StrokeThickness = stroke;
        // The head is the border's top edge: a quarter from 45° left of top to 45° right.
        var r = (size - stroke) / 2;
        var c = size / 2;
        var d = r * Math.Sqrt(0.5);
        var geometry = new PathGeometry();
        var figure = new PathFigure { StartPoint = new Windows.Foundation.Point(c - d, c - d) };
        figure.Segments.Add(new ArcSegment { Point = new Windows.Foundation.Point(c + d, c - d), Size = new Windows.Foundation.Size(r, r), SweepDirection = SweepDirection.Clockwise });
        geometry.Figures.Add(figure);
        head.Data = geometry;
        head.Stroke = new SolidColorBrush(ink);
        head.StrokeThickness = stroke;
        head.Width = head.Height = size;
    }
}
public enum SkeletonShape { Text, Circle, Rect, Rectangle = Rect }
public class Skeleton : Surface
{
    private readonly Border highlight = new() { IsHitTestVisible = false };
    private readonly TranslateTransform sweep = new();
    private readonly GradientStop glint = new() { Offset = .5 };
    private Storyboard? sweeping;
    private double sweepMilliseconds;
    public SkeletonShape Shape { get; set; } = SkeletonShape.Text;
    public Skeleton()
    {
        // CSS paints a gradient image twice the box's width. Relative brush
        // coordinates keep that same image when the placeholder resizes.
        highlight.Background = new LinearGradientBrush
        {
            StartPoint = new Point(0, .5), EndPoint = new Point(2, .5),
            RelativeTransform = sweep,
            GradientStops =
            {
                new GradientStop { Color = Microsoft.UI.Colors.Transparent, Offset = 0 },
                glint,
                new GradientStop { Color = Microsoft.UI.Colors.Transparent, Offset = 1 },
            },
        };
        Content = highlight;
        RegisterPropertyChangedCallback(CornerRadiusProperty, (_, _) => highlight.CornerRadius = CornerRadius);
        FeedbackMotion.Follow(this, Sweep, StopSweep);
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetAccessibilityView(this, Microsoft.UI.Xaml.Automation.Peers.AccessibilityView.Raw);
    }
    protected override void RefreshAppearance()
    {
        Background = Theme.Brush("color.surface-sunken");
        glint.Color = Theme.Color("color.surface-subtle");
        Skin.SetCorner(this, Theme.Number(Shape == SkeletonShape.Circle ? "radius.full" : Shape == SkeletonShape.Text ? "radius.tiny" : "radius.small"));
        // A line of text is the height of a line of body copy, not a length of its own.
        if (Shape == SkeletonShape.Text) Height = Theme.Text("", "bodySmall").LineHeight;
        if (Shape == SkeletonShape.Circle)
        {
            if (!double.IsNaN(Width)) Height = Width;
            else if (!double.IsNaN(Height)) Width = Height;
        }
        Sweep();
    }
    private void Sweep()
    {
        if (!IsLoaded || !FeedbackMotion.Enabled) { StopSweep(); return; }
        var duration = Theme.Milliseconds("skeleton.sweep-duration");
        if (sweeping is not null && sweepMilliseconds == duration) return;
        StopSweep();
        sweepMilliseconds = duration;
        // background-position runs 200% -> -200%. For an image twice the
        // box's width these percentages place its start at -2w -> 2w.
        sweeping = FeedbackMotion.Between(sweep, "X", -2, 2, TimeSpan.FromMilliseconds(duration), FeedbackMotion.EaseInOut(), repeat: true);
        sweeping.Begin();
    }
    private void StopSweep()
    {
        sweeping?.Stop();
        sweeping = null;
        sweep.X = 0;
    }
}
public sealed class SkeletonText : Surface
{
    public int Lines { get; set; } = 3;
    protected override void RefreshAppearance()
    {
        // The gap is the leading between two lines; the last line stops short, as a paragraph's does.
        var lines = new Grid { RowSpacing = Theme.Number("spacing.1.5") };
        var last = Theme.Number("skeleton.last-line-width");
        lines.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(last, GridUnitType.Star) });
        lines.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1 - last, GridUnitType.Star) });
        for (var i = 0; i < Lines; i++)
        {
            lines.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
            var line = new Skeleton { Shape = SkeletonShape.Text };
            Grid.SetRow(line, i);
            if (i < Lines - 1) Grid.SetColumnSpan(line, 2);
            lines.Children.Add(line);
        }
        Content = lines;
    }
}
public class Slider : Microsoft.UI.Xaml.Controls.Slider
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public Slider()
    {
        Appearance.Bind(this, () =>
        {
            Foreground = Theme.Recipe(ControlVariant.Filled, Tint, "surface");
            Background = Theme.Brush("color.surface-sunken");
            Resources["SliderTrackFill"] = Background;
            Resources["SliderTrackValueFill"] = Foreground;
            Resources["SliderTrackValueFillPointerOver"] = Foreground;
            Resources["SliderThumbBackground"] = Theme.Brush("color.surface");
            Resources["SliderThumbBorderBrush"] = Theme.Brush("color.border");
            Resources["SliderTrackThemeHeight"] = Theme.Number($"slider.{Skin.SizeKey(Size)}.track");
            Resources["SliderThumbWidth"] = Theme.Number($"slider.{Skin.SizeKey(Size)}.thumb");
            Resources["SliderThumbHeight"] = Theme.Number($"slider.{Skin.SizeKey(Size)}.thumb");
            // The row is the control step itself, as the web's control box
            // is; the native template would otherwise floor it at 34.
            Height = Theme.Number($"control.{Theme.Key(Size)}.size");
            MinHeight = 0; Skin.Focus(this, Theme);
        });
    }
}
public enum StepStatus { Pending, Active, Complete, Error }
/// <summary>
/// One stage of a pipeline: a marker — a check for done, a spinner for the
/// live one, an empty ring for what is to come — its label, and a trailing
/// detail. Only the active row takes the label weight.
/// </summary>
public sealed class Step : Surface
{
    public string Title { get; set; } = "";
    public string? Detail { get; set; }
    public StepStatus Status { get; set; }
    protected override void RefreshAppearance()
    {
        var marker = new Border { Width = Theme.Number("icon.medium"), Height = Theme.Number("icon.medium"), CornerRadius = new CornerRadius(Theme.Number("icon.medium") / 2), VerticalAlignment = VerticalAlignment.Center };
        switch (Status)
        {
            case StepStatus.Complete:
                marker.Background = Theme.Alpha("color.success.600", 0.14);
                marker.Child = new Icon { Glyph = FluentIcons.Common.Icon.Checkmark, GlyphVariant = FluentIcons.Common.IconVariant.Filled, GlyphSize = Theme.Number("icon.small"), Foreground = Theme.Brush("color.success.600"), HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
                break;
            case StepStatus.Error:
                marker.Background = Theme.Alpha("color.danger.600", 0.14);
                marker.Child = new Icon { Glyph = FluentIcons.Common.Icon.Dismiss, GlyphSize = Theme.Number("icon.small"), Foreground = Theme.Brush("color.danger.600"), HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
                break;
            case StepStatus.Active:
                marker.Width = marker.Height = Theme.Number("spacing.4");
                marker.Child = new Spinner { Size = ControlSize.Medium, Tint = Tint };
                break;
            default:
                marker.BorderBrush = Theme.Brush("color.border-muted");
                marker.BorderThickness = new Thickness(Theme.Number("stroke.control"));
                break;
        }
        var label = Theme.Text(Title, "labelQuiet");
        if (Status == StepStatus.Active) TextStyle.SetWeight(label, Theme.Weight("labelMedium"));
        label.Foreground = Theme.Brush(Status switch { StepStatus.Complete => "color.content-muted", StepStatus.Active => "color.content", StepStatus.Error => "color.danger.700", _ => "color.content-subtle" });
        label.VerticalAlignment = VerticalAlignment.Center;
        var row = new Grid { ColumnSpacing = Theme.Number("spacing.2.5") };
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.Children.Add(marker);
        Grid.SetColumn(label, 1); row.Children.Add(label);
        if (Detail is not null)
        {
            var meta = Theme.Text(Detail, "labelSmall");
            TextStyle.SetWeight(meta, Theme.Weight("captionSmall"));
            meta.Foreground = Status == StepStatus.Active ? Theme.Recipe(ControlVariant.Plain, Tint, "content") : Theme.Brush("color.content-faint");
            meta.VerticalAlignment = VerticalAlignment.Center;
            Grid.SetColumn(meta, 2); row.Children.Add(meta);
        }
        Opacity = Status == StepStatus.Pending ? 0.8 : 1;
        Content = row;
    }
}public sealed class StepList : Surface
{
    public IList<Step> Steps { get; } = new List<Step>();
    protected override void RefreshAppearance()
    {
        var column = Skin.Stack(Theme, Orientation.Vertical, Steps.Cast<UIElement>().ToArray(), Content as StackPanel);
        column.Spacing = Theme.Number("spacing.2");
        Content = column;
    }
}
