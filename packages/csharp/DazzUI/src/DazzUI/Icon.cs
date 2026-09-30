using FluentIcons.Common;
using FluentIcons.WinUI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI;

/// <summary>
/// A Fluent System Icon, from the same set React draws — by name
/// (<see cref="Glyph"/>, <c>FluentIcons.Common.Icon</c>) and <see cref="GlyphVariant"/> — on the
/// resizable 20px grid, at <see cref="GlyphSize"/> or the size the control's
/// <c>Size</c> gives it. It takes its ink from the control it stands in.
/// </summary>
public sealed class Icon : Surface
{
    public Glyph Glyph { get; set; }
    public IconVariant GlyphVariant { get; set; } = IconVariant.Regular;
    public double? GlyphSize { get; set; }
    protected override void RefreshAppearance()
    {
        Width = Height = GlyphSize ?? Theme.Number(Size == ControlSize.Small ? "spacing.3" : Size == ControlSize.Large ? "spacing.5" : "spacing.4");
        var glyph = Element(Glyph, GlyphVariant);
        glyph.FontSize = Width;
        glyph.SetBinding(IconElement.ForegroundProperty, new Microsoft.UI.Xaml.Data.Binding { Source = this, Path = new PropertyPath(nameof(Foreground)) });
        Content = glyph;
    }
    /// <summary>A glyph as an icon element, for a menu row or anywhere WinUI asks for one.</summary>
    public static FluentIcon Element(Glyph glyph, IconVariant variant = IconVariant.Regular) =>
        new() { Icon = glyph, IconVariant = variant, IconSize = IconSize.Resizable };
}
public sealed class IconButton : Button
{
    public Glyph Glyph { get; set; }
    public IconVariant GlyphVariant { get; set; } = IconVariant.Regular;
    public string Label { get; set; } = "Action";
    public IconButton()
    {
        Size = ControlSize.Tiny;
        Loaded += (_, _) =>
        {
            Content = new Icon { Glyph = Glyph, GlyphVariant = GlyphVariant };
            Square();
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, Label);
            Tooltip.Set(this, Label);
        };
        // A theme change re-applies the button's label padding and height;
        // keep the glyph's button square and unpadded through it.
        RegisterPropertyChangedCallback(PaddingProperty, (_, _) => { if (IsLoaded && Padding != new Thickness(0)) Padding = new Thickness(0); });
        RegisterPropertyChangedCallback(HeightProperty, (_, _) => { if (IsLoaded) Square(); });
    }
    private void Square()
    {
        Width = Height; Padding = new Thickness(0);
        if (Content is not Icon icon) return;
        icon.GlyphSize = Theme.Number($"control.{Theme.Key(Size)}.size") - Theme.Number("spacing.3");
        Appearance.Refresh(icon);
    }
}
public sealed class Link : HyperlinkButton
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public Link()
    {
        Appearance.Bind(this, () => { Theme.Typography(this, "bodyMedium"); Foreground = Theme.Recipe(ControlVariant.Plain, Tint, "content"); Padding = new Thickness(0); Skin.Focus(this, Theme); });
    }
}
