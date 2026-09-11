using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI;

/// <summary>A surface containing caller-owned content.</summary>
public class Card : Surface
{
    public Card(UIElement content, Theme? theme = null)
    {
        if (theme is not null) Theme = theme;
        theme ??= Theme.Default;
        Background = theme.Brush("color.surface-muted");
        BorderBrush = theme.Brush("color.border");
        BorderThickness = new Thickness(theme.Number("stroke.hairline"));
        CornerRadius = new CornerRadius(theme.Number("radius.large"));
        Padding = new Thickness(theme.Number("spacing.4"));
        Content = content;
        HorizontalContentAlignment = HorizontalAlignment.Stretch;
        IsTabStop = false;
    }
    public Card() { }
    public static readonly DependencyProperty VariantProperty = DependencyProperty.Register(nameof(Variant), typeof(CardVariant), typeof(Card), new PropertyMetadata(CardVariant.Sunken, (sender, _) => Appearance.Refresh(sender)));
    public CardVariant Variant { get => (CardVariant)GetValue(VariantProperty); set => SetValue(VariantProperty, value); }
    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        Background = Variant switch
        {
            CardVariant.Raised => Theme.Brush("color.surface"),
            CardVariant.Outlined => null,
            CardVariant.Tinted => Theme.Alpha($"color.{Theme.Key(Tint)}.{Theme.Number("focus.ring.shade")}", Theme.Number("wash.surface")),
            _ => Theme.Brush("color.surface-muted")
        };
        BorderBrush = Variant == CardVariant.Tinted ? Theme.Alpha($"color.{Theme.Key(Tint)}.{Theme.Number("focus.ring.shade")}", Theme.Number("wash.edge")) : Theme.Brush("color.border");
        BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        CornerRadius = new CornerRadius(Theme.Number("radius.large"));
        var block = Theme.Number(Size == ControlSize.Large ? "spacing.4" : "spacing.3");
        var inline = Theme.Number(Size == ControlSize.Large ? "spacing.4" : Size == ControlSize.Medium ? "spacing.3.5" : "spacing.3");
        Padding = new Thickness(inline, block, inline, block);
    }
}
public enum CardVariant { Raised, Sunken, Outlined, Tinted }
