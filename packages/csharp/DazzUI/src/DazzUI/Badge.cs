using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI;

/// <summary>A non-interactive label drawn with a resting tint recipe.</summary>
public sealed class Badge : Surface
{
    public Badge(string text, Theme? theme = null, WidgetTint tint = WidgetTint.Primary)
    {
        if (theme is not null) Theme = theme;
        theme ??= Theme.Default;
        Tint = tint;
        Background = theme.Recipe(ControlVariant.Tinted, tint, "surface");
        Skin.SetCorner(this, theme.Number("radius.tiny"));
        BorderThickness = new Thickness(theme.Number("stroke.hairline"));
        Padding = new Thickness(theme.Number("spacing.2"), theme.Number("spacing.0.5"),
            theme.Number("spacing.2"), theme.Number("spacing.0.5"));
        Content = text;
        IsTabStop = false;
        HorizontalAlignment = HorizontalAlignment.Left;
    }
    public Badge() { HorizontalAlignment = HorizontalAlignment.Left; }
    public static readonly DependencyProperty VariantProperty = DependencyProperty.Register(nameof(Variant), typeof(BadgeVariant), typeof(Badge), new PropertyMetadata(BadgeVariant.Tinted, (sender, _) => Appearance.Refresh(sender)));
    public BadgeVariant Variant { get => (BadgeVariant)GetValue(VariantProperty); set => SetValue(VariantProperty, value); }
    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        var recipe = Variant == BadgeVariant.Filled ? ControlVariant.Filled : Variant == BadgeVariant.Outlined ? ControlVariant.Outlined : ControlVariant.Tinted;
        Background = Variant == BadgeVariant.Raised ? Theme.Brush("color.surface") : Theme.Recipe(recipe, Tint, "surface");
        Foreground = Variant == BadgeVariant.Raised ? Theme.Brush("color.content-muted") : Theme.Recipe(recipe, Tint, "content");
        BorderBrush = Theme.Recipe(recipe, Tint, "border");
        Skin.SetCorner(this, Theme.Number("radius.tiny"));
        BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        var block = Theme.Number(Size == ControlSize.Large ? "spacing.1" : "spacing.0.5");
        var inline = Theme.Number(Size is ControlSize.Small or ControlSize.Tiny ? "spacing.1.5" : "spacing.2");
        Padding = new Thickness(inline, block, inline, block);
        Theme.Typography(this, Size == ControlSize.Large ? "labelMedium" : "labelSmall");
        FontWeight = Theme.Weight("labelMedium");
        Skin.TextLine(this, FontSize * 1.4);
    }
}
public enum BadgeVariant { Tinted, Filled, Outlined, Raised }
