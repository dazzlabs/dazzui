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
        // React's Icon defaults to the theme's `icon.medium`; a caller that
        // names a size gets exactly that, as a rendered `<svg>` would.
        Width = Height = GlyphSize ?? Theme.Number("icon.medium");
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
        Variant = ControlVariant.Plain;
        Tint = WidgetTint.Neutral;
        Size = ControlSize.Tiny;
        Loaded += (_, _) =>
        {
            // A caller may hand the button a glyph element of its own — the
            // React story's `<Icon>` child — which stays as drawn.
            if (Content is not Icon) Content = new Icon { Glyph = Glyph, GlyphVariant = GlyphVariant };
            Square();
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, Label);
            Tooltip.Set(this, Label);
        };
        // A theme change re-applies the button's label padding and height;
        // keep the glyph's button square and unpadded through it.
        RegisterPropertyChangedCallback(PaddingProperty, (_, _) => { if (IsLoaded && Padding != new Thickness(0)) Padding = new Thickness(0); });
        RegisterPropertyChangedCallback(HeightProperty, (_, _) => { if (IsLoaded) Square(); });
    }
    /// <summary>
    /// The default posture — plain and neutral — is quiet chrome: a subtle-ink
    /// glyph that washes grey and darkens to full ink under the pointer.
    /// </summary>
    protected override void Tailor()
    {
        if (Variant != ControlVariant.Plain || Tint != WidgetTint.Neutral) return;
        Repaint("ButtonForeground", Theme.Color("color.content-subtle"));
        Repaint("ButtonForegroundPointerOver", Theme.Color("color.content"));
        Repaint("ButtonForegroundPressed", Theme.Color("color.content"));
        Repaint("ButtonBackgroundPointerOver", Theme.Color("color.surface-subtle"));
    }
    /// <summary>The glyph alone fills the square: no label padding, no label-glyph shrink.</summary>
    protected override void LayContent() { if (IsLoaded) Square(); }
    /// <summary>A square glyph button keeps the control's weight and lies flat in every variant.</summary>
    private protected override bool Raised => false;
    private void Square()
    {
        Width = Height; Padding = new Thickness(0);
        if (Content is not Icon icon) return;
        // The profile's icon, except alone in the tiny square, where the
        // profile's small glyph would be half the box: that takes its own.
        icon.GlyphSize = Theme.Number(Size == ControlSize.Tiny ? "icon-button.tiny.icon-size" : $"control.{Theme.Key(Size)}.icon-size");
        Appearance.Refresh(icon);
    }
}
public enum LinkUnderline { Hover, Always, None }
public sealed class Link : HyperlinkButton
{
    private bool hovered;
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    /// <summary>When the underline shows: under the pointer by default, always, or never.</summary>
    public LinkUnderline Underline { get; set; }
    public Link()
    {
        Appearance.Bind(this, Apply);
        PointerEntered += (_, _) => { hovered = true; Apply(); };
        PointerExited += (_, _) => { hovered = false; Apply(); };
    }
    private void Apply()
    {
        Theme.Typography(this, "bodyMedium"); Foreground = Theme.Recipe(ControlVariant.Plain, Tint, "content"); Padding = new Thickness(0); Skin.Focus(this, Theme);
        BorderThickness = new Thickness(0);
        Skin.SetCorner(this, Theme.Number("radius.tiny"));
        if (Content is string text) Content = Theme.Text(text, "bodyMedium");
        if (Content is TextBlock words)
        {
            words.Foreground = Foreground;
            words.TextDecorations = Underline == LinkUnderline.Always || Underline == LinkUnderline.Hover && hovered ? Windows.UI.Text.TextDecorations.Underline : Windows.UI.Text.TextDecorations.None;
        }
    }
}
