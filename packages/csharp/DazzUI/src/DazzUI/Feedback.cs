using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Animation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;

namespace DazzUI;

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
    private double value, minimum, maximum = 100;
    private bool indeterminate;
    public double Value { get => value; set { this.value = value; Arrange(); } }
    public double Minimum { get => minimum; set { minimum = value; Arrange(); } }
    public double Maximum { get => maximum; set { maximum = value; Arrange(); } }
    public bool IsIndeterminate { get => indeterminate; set { indeterminate = value; Arrange(); } }
    public bool Gradient { get; set; }
    public Progress()
    {
        Size = ControlSize.Small;
        track.Children.Add(fill);
        fill.RenderTransform = sweep;
        Content = track;
        track.SizeChanged += (_, _) => Arrange();
        Unloaded += (_, _) => sweeping?.Stop();
    }
    /// <summary>The groove and fill, for a subclass that sets a head above them.</summary>
    protected FrameworkElement Track => track;
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
    private void Arrange()
    {
        var width = track.ActualWidth;
        if (indeterminate)
        {
            // An indeterminate bar has no width to show, so it sweeps instead.
            fill.Width = width * 0.4;
            if (sweeping is null && width > 0)
            {
                var run = new DoubleAnimation { From = -width * 0.4, To = width, Duration = TimeSpan.FromSeconds(1.2), RepeatBehavior = RepeatBehavior.Forever, EasingFunction = new SineEase { EasingMode = EasingMode.EaseInOut } };
                Storyboard.SetTarget(run, sweep);
                Storyboard.SetTargetProperty(run, "X");
                sweeping = new Storyboard { Children = { run } };
                sweeping.Begin();
            }
            return;
        }
        sweeping?.Stop();
        sweeping = null;
        sweep.X = 0;
        var span = maximum - minimum;
        fill.Width = span <= 0 ? 0 : width * Math.Clamp((value - minimum) / span, 0, 1);
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
    public Meter()
    {
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
        label.FontSize = Theme.Text("", "labelSmall").FontSize;
        label.FontWeight = Theme.Text("", "labelMedium").FontWeight;
        var span = Maximum - Minimum;
        reading.Text = ShowValue ? $"{Math.Round(span <= 0 ? 0 : (Value - Minimum) / span * 100)}%" : "";
        reading.Foreground = Theme.Brush("color.content-muted");
        reading.FontSize = label.FontSize;
        reading.FontFamily = new FontFamily("Cascadia Mono, Consolas");
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
        Loaded += (_, _) =>
        {
            var run = new DoubleAnimation { From = 0, To = 360, Duration = TimeSpan.FromSeconds(1), RepeatBehavior = RepeatBehavior.Forever };
            Storyboard.SetTarget(run, turn);
            Storyboard.SetTargetProperty(run, "Angle");
            turning = new Storyboard { Children = { run } };
            turning.Begin();
        };
        Unloaded += (_, _) => turning?.Stop();
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, "Loading");
    }
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
}public enum SkeletonShape { Rectangle, Circle, Text }
public class Skeleton : Surface
{
    public SkeletonShape Shape { get; set; }
    protected override void RefreshAppearance()
    {
        Background = Theme.Brush("color.surface-sunken");
        Skin.SetCorner(this, Theme.Number(Shape == SkeletonShape.Circle ? "radius.full" : Shape == SkeletonShape.Text ? "radius.tiny" : "radius.small"));
        // A line of text is the height of a line of body copy, not a length of its own.
        if (double.IsNaN(Height)) Height = Shape == SkeletonShape.Text ? Theme.Text("", "bodySmall").LineHeight : Theme.Number("spacing.8");
        if (Shape == SkeletonShape.Circle && double.IsNaN(Width)) Width = Height;
    }
}
public sealed class SkeletonText : Surface
{
    public int Lines { get; set; } = 3;
    protected override void RefreshAppearance()
    {
        // The gap is the leading between two lines; the last line stops short, as a paragraph's does.
        var lines = new Grid { RowSpacing = Theme.Number("spacing.1.5") };
        lines.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(6, GridUnitType.Star) });
        lines.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(4, GridUnitType.Star) });
        for (var i = 0; i < Lines; i++)
        {
            lines.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
            var line = new Skeleton { Shape = SkeletonShape.Text };
            Grid.SetRow(line, i);
            if (i < Lines - 1 || Lines == 1) Grid.SetColumnSpan(line, 2);
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
            MinHeight = Theme.Number($"control.{Theme.Key(Size)}.size"); Skin.Focus(this, Theme);
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
                marker.Child = new Spinner { Size = ControlSize.Medium, Tint = Tint };
                break;
            default:
                marker.BorderBrush = Theme.Brush("color.border-muted");
                marker.BorderThickness = new Thickness(Theme.Number("stroke.control"));
                break;
        }
        var label = Theme.Text(Title, "labelQuiet");
        if (Status == StepStatus.Active) label.FontWeight = Theme.Text("", "labelMedium").FontWeight;
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
            meta.FontWeight = Theme.Text("", "captionSmall").FontWeight;
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
        var column = Skin.Column(Theme, Steps.Cast<UIElement>().ToArray());
        column.Spacing = Theme.Number("spacing.2");
        Content = column;
    }
}
