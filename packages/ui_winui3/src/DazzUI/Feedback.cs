using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Animation;

namespace DazzUI;

public class Progress : Surface
{
    public ProgressBar Indicator { get; } = new();
    public double Value { get => Indicator.Value; set => Indicator.Value = value; }
    public double Minimum { get => Indicator.Minimum; set => Indicator.Minimum = value; }
    public double Maximum { get => Indicator.Maximum; set => Indicator.Maximum = value; }
    public bool IsIndeterminate { get => Indicator.IsIndeterminate; set => Indicator.IsIndeterminate = value; }
    public bool Gradient { get; set; }
    public Progress()
    {
        Content = Indicator;
    }
    protected override void RefreshAppearance()
    {
        Indicator.Foreground = Theme.Recipe(ControlVariant.Filled, Tint, "surface");
        if (Gradient)
            Indicator.Foreground = new LinearGradientBrush
            {
                StartPoint = new Windows.Foundation.Point(0, .5),
                EndPoint = new Windows.Foundation.Point(1, .5),
                GradientStops = { new GradientStop { Color = Theme.Color("progress.gradient.from"), Offset = 0 }, new GradientStop { Color = Theme.Color("progress.gradient.to"), Offset = 1 } },
            };
        Indicator.Background = Theme.Brush("color.surface-sunken");
        Height = Indicator.Height = Theme.Number(Size == ControlSize.Small ? "spacing.1" : Size == ControlSize.Large ? "spacing.2" : "spacing.1.5");
        Indicator.MinHeight = 0; Indicator.CornerRadius = new CornerRadius(Theme.Number("radius.full"));
    }
}
public sealed class Meter : Progress
{
    public double Low { get; set; } = 25;
    public double High { get; set; } = 75;
    public Meter() { IsIndeterminate = false; }
}
public sealed class Spinner : Surface
{
    public ProgressRing Indicator { get; } = new() { IsActive = true };
    public Spinner() { Content = Indicator; }
    protected override void RefreshAppearance()
    {
        Width = Height = Theme.Number(Size == ControlSize.Small ? "spacing.3" : Size == ControlSize.Large ? "spacing.5" : "spacing.4");
        Indicator.Width = Width; Indicator.Height = Height; Indicator.MinWidth = Indicator.MinHeight = 0;
        Indicator.Foreground = Theme.Recipe(ControlVariant.Plain, Tint, "content");
        Indicator.Background = Theme.Recipe(ControlVariant.Tinted, Tint, "surface");
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, "Loading");
    }
}
public enum SkeletonShape { Rectangle, Circle, Text }
public class Skeleton : Surface
{
    public SkeletonShape Shape { get; set; }
    protected override void RefreshAppearance()
    {
        Background = Theme.Brush("color.surface-sunken");
        CornerRadius = new CornerRadius(Theme.Number(Shape == SkeletonShape.Circle ? "radius.full" : "radius.small"));
        if (double.IsNaN(Height)) Height = Theme.Number(Shape == SkeletonShape.Text ? "spacing.3" : "spacing.8");
        if (Shape == SkeletonShape.Circle && double.IsNaN(Width)) Width = Height;
    }
}
public sealed class SkeletonText : Surface
{
    public int Lines { get; set; } = 3;
    protected override void RefreshAppearance()
    {
        Content = Skin.Column(Theme, Enumerable.Range(0, Lines).Select(i => (UIElement)new Skeleton { Shape = SkeletonShape.Text, HorizontalAlignment = HorizontalAlignment.Stretch }).ToArray());
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
public sealed class Step : Surface
{
    public string Title { get; set; } = "";
    public string? Detail { get; set; }
    public StepStatus Status { get; set; }
    protected override void RefreshAppearance()
    {
        var mark = Status switch { StepStatus.Complete => "✓", StepStatus.Error => "!", StepStatus.Active => "•", _ => "○" };
        var label = Theme.Text(mark, "titleSmall");
        label.Foreground = Theme.Recipe(ControlVariant.Plain, Status == StepStatus.Error ? WidgetTint.Danger : Tint, "content");
        var text = Skin.Column(Theme, Theme.Text(Title, "labelMedium"));
        if (Detail is not null) text.Children.Add(Theme.Text(Detail, "bodySmall"));
        Content = Skin.Row(Theme, label, text);
    }
}
public sealed class StepList : Surface
{
    public IList<Step> Steps { get; } = new List<Step>();
    protected override void RefreshAppearance() { Content = Skin.Column(Theme, Steps.Cast<UIElement>().ToArray()); }
}
