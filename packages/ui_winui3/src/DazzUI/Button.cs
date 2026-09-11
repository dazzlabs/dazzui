using Microsoft.UI.Xaml;

namespace DazzUI;

/// <summary>Native button behavior with DazzUI sizing and state recipes.</summary>
public class Button : Microsoft.UI.Xaml.Controls.Button
{
    private readonly Dictionary<string, Microsoft.UI.Xaml.Media.SolidColorBrush> stateBrushes = [];
    public static readonly DependencyProperty ThemeProperty = DependencyProperty.Register(
        nameof(Theme), typeof(Theme), typeof(Button), new PropertyMetadata(null, Changed));
    public static readonly DependencyProperty TintProperty = DependencyProperty.Register(
        nameof(Tint), typeof(WidgetTint), typeof(Button), new PropertyMetadata(WidgetTint.Primary, Changed));
    public static readonly DependencyProperty VariantProperty = DependencyProperty.Register(
        nameof(Variant), typeof(ControlVariant), typeof(Button), new PropertyMetadata(ControlVariant.Filled, Changed));
    public static readonly DependencyProperty SizeProperty = DependencyProperty.Register(
        nameof(Size), typeof(ControlSize), typeof(Button), new PropertyMetadata(ControlSize.Small, Changed));

    public Theme Theme { get => (Theme?)GetValue(ThemeProperty) ?? Appearance.GetTheme(this); set => SetValue(ThemeProperty, value); }
    public WidgetTint Tint { get => (WidgetTint)GetValue(TintProperty); set => SetValue(TintProperty, value); }
    public ControlVariant Variant { get => (ControlVariant)GetValue(VariantProperty); set => SetValue(VariantProperty, value); }
    public ControlSize Size { get => (ControlSize)GetValue(SizeProperty); set => SetValue(SizeProperty, value); }

    public Button()
    {
        DefaultStyleKey = typeof(Button);
        Appearance.Bind(this, ApplyTheme);
        ApplyTheme();
    }

    private static void Changed(DependencyObject sender, DependencyPropertyChangedEventArgs args) => ((Button)sender).ApplyTheme();

    protected override void OnApplyTemplate()
    {
        // Nested content can be materialized after its parent's Loaded event.
        // Resolve its inherited theme before the template reads state resources.
        ApplyTheme();
        base.OnApplyTemplate();
    }

    private void ApplyTheme()
    {
        var profile = $"control.{Theme.Key(Size)}";
        Height = Theme.Number(profile + ".size");
        MinHeight = 0;
        MinWidth = 0;
        Padding = new Thickness(Theme.Number(profile + ".padding.inline"), 0,
            Theme.Number(profile + ".padding.inline"), 0);
        CornerRadius = new CornerRadius(Theme.Number(profile + ".radius"));
        BorderThickness = new Thickness(Theme.Number("stroke.control"));
        Theme.Typography(this, profile + ".content");
        foreach (var (state, suffix) in new[] { ("normal", ""), ("hovered", "PointerOver"), ("pressed", "Pressed"), ("disabled", "Disabled") })
        {
            SetBrush("ButtonBackground" + suffix, Theme.Recipe(Variant, Tint, "surface", state));
            SetBrush("ButtonForeground" + suffix, Theme.Recipe(Variant, Tint, "content", state));
            SetBrush("ButtonBorderBrush" + suffix, Theme.Recipe(Variant, Tint, "border", state));
        }
        Background = stateBrushes["ButtonBackground"];
        Foreground = stateBrushes["ButtonForeground"];
        BorderBrush = stateBrushes["ButtonBorderBrush"];
        var focus = Theme.Color($"color.primary.{Theme.Number("focus.ring.shade")}");
        focus.A = (byte)Math.Round(focus.A * Theme.Number("focus.ring.alpha"));
        FocusVisualPrimaryBrush = new Microsoft.UI.Xaml.Media.SolidColorBrush(focus);
        FocusVisualPrimaryThickness = new Thickness(Theme.Number("focus.width"));
        FocusVisualSecondaryThickness = new Thickness(0);
        FocusVisualMargin = new Thickness(0);
    }

    private void SetBrush(string key, Microsoft.UI.Xaml.Media.SolidColorBrush brush)
    {
        // Keep the resource object alive so an active visual state follows changes.
        if (stateBrushes.TryGetValue(key, out var existing))
            existing.Color = brush.Color;
        else
        {
            stateBrushes[key] = brush;
            Resources[key] = brush;
        }
    }
}
