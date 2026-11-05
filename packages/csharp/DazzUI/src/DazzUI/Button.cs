using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI;

/// <summary>
/// Native button behavior with DazzUI sizing and state recipes. A glyph
/// beside a label is the content's job to place and the button's to size: give
/// it an <see cref="Icon"/> alone, or a horizontal <see cref="StackPanel"/> of
/// icons and labels, and the button draws every icon in it
/// <c>control.label-glyph.shrink</c> under the profile's icon size, spaces the
/// line at the profile's gap, and pads an end a glyph sits at
/// <c>control.label-glyph.inset</c> less, as React's Button does.
/// </summary>
public class Button : Microsoft.UI.Xaml.Controls.Button
{
    private readonly Dictionary<string, Microsoft.UI.Xaml.Media.SolidColorBrush> stateBrushes = [];
    public static readonly DependencyProperty ThemeProperty = DependencyProperty.Register(
        nameof(Theme), typeof(Theme), typeof(Button), new PropertyMetadata(null, Changed));
    public static readonly DependencyProperty TintProperty = DependencyProperty.Register(
        nameof(Tint), typeof(WidgetTint), typeof(Button), new PropertyMetadata(WidgetTint.Primary, Changed));
    public static readonly DependencyProperty VariantProperty = DependencyProperty.Register(
        nameof(Variant), typeof(ControlVariant), typeof(Button), new PropertyMetadata(ControlVariant.Recessed, Changed));
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
        RegisterPropertyChangedCallback(ContentProperty, (_, _) => { AlignLabel(); LayContent(); });
        IsEnabledChanged += (_, _) => ApplyTheme();
        ApplyTheme();
    }

    private static void Changed(DependencyObject sender, DependencyPropertyChangedEventArgs args) => Appearance.Refresh(sender);

    protected override void OnApplyTemplate()
    {
        // Nested content can be materialized after its parent's Loaded event.
        // Resolve its inherited theme before the template reads state resources.
        ApplyTheme();
        base.OnApplyTemplate();
        AlignLabel();
        Appearance.UpdateTextScaling(this);
    }

    private void AlignLabel()
    {
        // Center a single-line label on the font's cap height and baseline,
        // excluding asymmetric font leading. Keep full line spacing for prose
        // and leave consumer-provided visual content in charge of its layout.
        if (GetTemplateChild("Presenter") is Microsoft.UI.Xaml.Controls.ContentPresenter presenter)
        {
            Skin.TextWeight(presenter, this);
            Skin.TextInk(this, presenter);
            var text = Content is string plain ? plain : (Content as TextBlock)?.Text;
            presenter.TextLineBounds = text is not null && !text.Contains('\n') && !text.Contains('\r')
                ? TextLineBounds.Tight : TextLineBounds.Full;
        }
    }

    private void ApplyTheme()
    {
        var profile = $"control.{Theme.Key(Size)}";
        Height = Theme.Number(profile + ".size");
        MinHeight = 0;
        MinWidth = 0;
        LayContent();
        Skin.SetCorner(this, Theme.Number(profile + ".radius"));
        BorderThickness = new Thickness(Theme.Number("spacing.px"));
        Theme.Typography(this, profile + ".content");
        // The one action a view points at carries the extra weight and the
        // hairline lift; both go when it is disabled and flattens.
        var raised = Raised && IsEnabled;
        if (raised) FontWeight = Theme.Weight("labelStrong");
        if (GetTemplateChild("Lift") is FrameworkElement lift) Elevation.Draw(this, lift, Theme, raised ? "2xs" : null);
        foreach (var (state, suffix) in new[] { ("normal", ""), ("hovered", "PointerOver"), ("pressed", "Pressed"), ("disabled", "Disabled") })
        {
            SetBrush("ButtonBackground" + suffix, Theme.Recipe(Variant, Tint, "surface", state));
            SetBrush("ButtonForeground" + suffix, Theme.Recipe(Variant, Tint, "content", state));
            SetBrush("ButtonBorderBrush" + suffix, Theme.Recipe(Variant, Tint, "border", state));
        }
        Background = stateBrushes["ButtonBackground"];
        Foreground = stateBrushes[IsEnabled ? "ButtonForeground" : "ButtonForegroundDisabled"];
        BorderBrush = stateBrushes["ButtonBorderBrush"];
        var focus = Theme.Color($"color.primary.{Theme.Number("focus.ring.shade")}");
        focus.A = (byte)Math.Round(focus.A * Theme.Number("focus.ring.alpha"));
        FocusVisualPrimaryBrush = new Microsoft.UI.Xaml.Media.SolidColorBrush(focus);
        FocusVisualPrimaryThickness = new Thickness(Theme.Number("focus.width"));
        FocusVisualSecondaryThickness = new Thickness(0);
        FocusVisualMargin = new Thickness(0);
        Tailor();
        Skin.TextLine(this, FontSize);
        AlignLabel();
    }

    /// <summary>
    /// The label's padding, and the glyphs beside it: each drawn
    /// <c>control.label-glyph.shrink</c> under the profile's icon size, which
    /// fills a square button alone and overtops the type beside a label. An
    /// end a glyph sits at pads <c>control.label-glyph.inset</c> less, since the
    /// glyph's frame carries whitespace of its own; a glyph alone is an
    /// <see cref="IconButton"/>'s business and trims nothing.
    /// </summary>
    protected virtual void LayContent()
    {
        var profile = $"control.{Theme.Key(Size)}";
        var pad = Theme.Number(profile + ".padding.inline");
        var block = Theme.Number(profile + ".padding.block");
        var inset = Theme.Number("control.label-glyph.inset");
        var glyph = Theme.Number(profile + ".icon-size") - Theme.Number("control.label-glyph.shrink");
        IList<UIElement> parts = Content switch
        {
            Icon icon => new UIElement[] { icon },
            StackPanel { Orientation: Orientation.Horizontal } line => line.Children,
            _ => Array.Empty<UIElement>(),
        };
        if (Content is StackPanel row) row.Spacing = Theme.Number(profile + ".gap");
        // Inside the profile's block padding a label sits on its cap height
        // and baseline, as a lone label does, not on the face's full leading.
        foreach (var words in parts.OfType<TextBlock>())
        {
            TextStyle.Features(words);
            words.TextLineBounds = TextLineBounds.Tight;
            words.IsTextScaleFactorEnabled = Theme.Sizing != ThemeSizing.System;
        }
        foreach (var icon in parts.OfType<Icon>())
            if (icon.GlyphSize != glyph)
            {
                icon.GlyphSize = glyph;
                Appearance.Refresh(icon);
            }
        var leading = parts.Count >= 2 && parts[0] is Icon;
        var trailing = parts.Count >= 2 && parts[^1] is Icon;
        Padding = new Thickness(pad - (leading ? inset : 0), block, pad - (trailing ? inset : 0), block);
    }

    /// <summary>Whether this kind of button lifts and strengthens its label when filled.</summary>
    private protected virtual bool Raised => Variant == ControlVariant.Filled;

    /// <summary>Where a button that departs from the shared recipes repaints its state brushes, after every theme pass.</summary>
    protected virtual void Tailor() { }
    /// <summary>Repaints one state brush in place, so an active visual state follows.</summary>
    protected void Repaint(string key, Windows.UI.Color color)
    {
        if (stateBrushes.TryGetValue(key, out var brush)) brush.Color = color;
        if (key == (IsEnabled ? "ButtonForeground" : "ButtonForegroundDisabled")) Foreground = stateBrushes[key];
        if (key == "ButtonBackground") Background = stateBrushes[key];
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
