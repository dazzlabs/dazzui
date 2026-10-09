using System.Runtime.CompilerServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

/// <summary>Per-subtree theme inheritance. Registrations are weak and require no disposal.</summary>
public static class Appearance
{
    private sealed record Registration(Action Refresh);
    private static readonly ConditionalWeakTable<FrameworkElement, Registration> Registrations = new();
    public static readonly DependencyProperty ThemeProperty = DependencyProperty.RegisterAttached(
        "Theme", typeof(Theme), typeof(Appearance), new PropertyMetadata(null, Changed));
    public static readonly DependencyProperty TintProperty = DependencyProperty.RegisterAttached(
        "Tint", typeof(WidgetTint), typeof(Appearance), new PropertyMetadata(WidgetTint.Primary, Changed));
    public static readonly DependencyProperty SizeProperty = DependencyProperty.RegisterAttached(
        "Size", typeof(ControlSize), typeof(Appearance), new PropertyMetadata(ControlSize.Medium, Changed));
    public static readonly DependencyProperty VariantProperty = DependencyProperty.RegisterAttached(
        "Variant", typeof(ControlVariant), typeof(Appearance), new PropertyMetadata(ControlVariant.Recessed, Changed));
    public static Theme GetTheme(DependencyObject element)
    {
        for (DependencyObject? current = element; current is not null; current = VisualTreeHelper.GetParent(current))
            if (current.GetValue(ThemeProperty) is Theme theme) return theme;
        return Theme.Default;
    }
    public static void SetTheme(DependencyObject element, Theme? value) => element.SetValue(ThemeProperty, value);
    public static WidgetTint GetTint(DependencyObject element) => (WidgetTint)element.GetValue(TintProperty);
    public static void SetTint(DependencyObject element, WidgetTint value) => element.SetValue(TintProperty, value);
    public static ControlSize GetSize(DependencyObject element) => (ControlSize)element.GetValue(SizeProperty);
    public static void SetSize(DependencyObject element, ControlSize value) => element.SetValue(SizeProperty, value);
    public static ControlVariant GetVariant(DependencyObject element) => (ControlVariant)element.GetValue(VariantProperty);
    public static void SetVariant(DependencyObject element, ControlVariant value) => element.SetValue(VariantProperty, value);
    internal static void Bind(FrameworkElement element, Action refresh)
    {
        Registrations.Add(element, new Registration(refresh));
        element.Loaded += (_, _) => refresh();
    }
    private static void Changed(DependencyObject sender, DependencyPropertyChangedEventArgs args) => Refresh(sender);
    public static void Refresh(DependencyObject root)
    {
        if (root is FrameworkElement element && (element is not Surface || element.IsLoaded) && Registrations.TryGetValue(element, out var registration))
            registration.Refresh();
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++) Refresh(VisualTreeHelper.GetChild(root, i));
    }
}

public class Surface : ContentControl
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public Surface()
    {
        DefaultStyleKey = typeof(Surface);
        IsTabStop = false;
        HorizontalContentAlignment = HorizontalAlignment.Stretch;
        Appearance.Bind(this, RefreshAppearance);
    }
    protected virtual void RefreshAppearance() { Foreground = Theme.Brush("color.content"); }
}

/// <summary>The host can switch a whole subtree without reconstructing its state.</summary>
public sealed class ThemeScope : Surface
{
    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        RequestedTheme = Theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
    }
}

internal static class Skin
{
    private sealed class CornerState(Control control)
    {
        internal double Requested;
        /// <summary>Which ends keep their corners: a control joined to a neighbour squares the edge it shares.</summary>
        internal bool Start = true, End = true;
        internal void Update()
        {
            var width = control.ActualWidth;
            var height = control.ActualHeight;
            if (width <= 0 || height <= 0) return;
            var r = Math.Min(Requested, Math.Min(width, height) / 2);
            var radius = new CornerRadius(Start ? r : 0, End ? r : 0, End ? r : 0, Start ? r : 0);
            if (control.CornerRadius != radius) control.CornerRadius = radius;
        }
    }
    private static readonly ConditionalWeakTable<Control, CornerState> Corners = new();
    // WinUI stretches oversized corner radii into ellipses. Resolve the shared
    // full-radius token against the actual box, including later resizes.
    internal static void SetCorner(Control control, double radius)
    {
        if (!Corners.TryGetValue(control, out var state))
        {
            state = new CornerState(control);
            Corners.Add(control, state);
            control.SizeChanged += (_, _) => state.Update();
        }
        state.Requested = radius;
        if (control.ActualWidth > 0 && control.ActualHeight > 0) state.Update();
        else control.CornerRadius = new CornerRadius(radius);
    }
    /// <summary>Squares the corners on the ends a control shares with its neighbours.</summary>
    internal static void Join(Control control, bool start, bool end)
    {
        if (!Corners.TryGetValue(control, out var state)) return;
        state.Start = start;
        state.End = end;
        state.Update();
        if (control.ActualWidth <= 0) control.CornerRadius = new CornerRadius(start ? state.Requested : 0, end ? state.Requested : 0, end ? state.Requested : 0, start ? state.Requested : 0);
    }
    internal static void RoundCorners(Control control) => SetCorner(control, control.CornerRadius.TopLeft);
    public static string SizeKey(ControlSize size) => Theme.Key(size == ControlSize.Tiny ? ControlSize.Small : size);
    public static void Control(Control control, Theme theme, ControlSize size)
    {
        var prefix = $"control.{Theme.Key(size)}";
        control.MinHeight = theme.Number(prefix + ".size");
        control.MinWidth = 0;
        control.Padding = new Thickness(theme.Number(prefix + ".padding.inline"), 0,
            theme.Number(prefix + ".padding.inline"), 0);
        SetCorner(control, theme.Number(prefix + ".radius"));
        theme.Typography(control, prefix + ".content");
        control.Foreground = theme.Brush("color.content");
        Focus(control, theme);
    }
    public static void Focus(Control control, Theme theme)
    {
        control.UseSystemFocusVisuals = true;
        control.FocusVisualPrimaryBrush = theme.Alpha($"color.primary.{theme.Number("focus.ring.shade")}", theme.Number("focus.ring.alpha"));
        control.FocusVisualPrimaryThickness = new Thickness(theme.Number("focus.width"));
        control.FocusVisualSecondaryThickness = new Thickness(0);
        control.FocusVisualMargin = new Thickness(theme.Number("focus.offset"));
    }
    public static void Input(Control control, Theme theme, ControlSize size, bool invalid = false)
    {
        Control(control, theme, size);
        SetCorner(control, theme.Number("control.field.radius"));
        control.BorderThickness = new Thickness(theme.Number("stroke.hairline"));
        // In error the whole field is re-tinted to danger: edge, wash and ink together.
        control.BorderBrush = invalid ? theme.Brush("color.danger.600") : theme.Brush("color.border-strong");
        control.Background = !control.IsEnabled ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "disabled")
            : invalid ? theme.Brush("color.danger.50") : theme.Brush("color.surface-muted");
        control.Foreground = !control.IsEnabled ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled")
            : invalid ? theme.Brush("color.danger.800") : theme.Brush("color.content");
        control.FontWeight = new Windows.UI.Text.FontWeight { Weight = (ushort)theme.Token("bodySmall").GetProperty("fontWeight").GetInt32() };
        foreach (var prefix in new[] { "TextControl", "ComboBox" })
        {
            control.Resources[prefix + "Background"] = control.Background;
            control.Resources[prefix + "BackgroundPointerOver"] = control.Background;
            control.Resources[prefix + "BackgroundFocused"] = theme.Brush("color.surface");
            control.Resources[prefix + "BackgroundDisabled"] = theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "disabled");
            control.Resources[prefix + "BorderBrush"] = control.BorderBrush;
            control.Resources[prefix + "BorderBrushPointerOver"] = control.BorderBrush;
            control.Resources[prefix + "BorderBrushFocused"] = theme.Recipe(ControlVariant.Outlined, invalid ? WidgetTint.Danger : Appearance.GetTint(control), "border");
            control.Resources[prefix + "Foreground"] = control.Foreground;
            control.Resources[prefix + "ForegroundDisabled"] = theme.Brush("color.content-faint");
        }
        var faint = theme.Brush("color.content-faint");
        foreach (var state in new[] { "", "PointerOver", "Focused", "Disabled" })
            control.Resources["TextControlPlaceholderForeground" + state] = faint;
        if (control is TextBox text) text.PlaceholderForeground = faint;
        if (control is ComboBox choice) choice.PlaceholderForeground = faint;
        control.Resources["TextControlBorderThemeThickness"] = control.BorderThickness;
        control.Resources["TextControlBorderThemeThicknessFocused"] = control.BorderThickness;
        control.Resources["TextControlThemeMinHeight"] = theme.Number($"control.{Theme.Key(size)}.size");
        InputPadding(control, theme, size);
    }
    internal static void InputPadding(Control control, Theme theme, ControlSize size, bool multiline = false)
    {
        var line = new TextBlock { Text = "Ag", FontFamily = control.FontFamily, FontSize = control.FontSize, FontWeight = control.FontWeight, FontStyle = control.FontStyle, FontStretch = control.FontStretch };
        line.Measure(new Windows.Foundation.Size(double.PositiveInfinity, double.PositiveInfinity));
        var block = multiline ? theme.Number("spacing.2.5")
            : Math.Max(0, (theme.Number($"control.{Theme.Key(size)}.size") - control.BorderThickness.Top - control.BorderThickness.Bottom - line.DesiredSize.Height) / 2);
        control.Padding = new Thickness(control.Padding.Left, block, control.Padding.Right, block);
    }
    public static StackPanel Column(Theme theme, params UIElement[] children) => Stack(theme, Orientation.Vertical, children);
    public static StackPanel Row(Theme theme, params UIElement[] children) => Stack(theme, Orientation.Horizontal, children);
    private static StackPanel Stack(Theme theme, Orientation direction, UIElement[] children)
    {
        var result = new StackPanel { Orientation = direction, Spacing = theme.Number("spacing.3") };
        foreach (var child in children) { Detach(child); result.Children.Add(child); }
        return result;
    }
    internal static void Detach(UIElement child)
    {
        var parent = (child as FrameworkElement)?.Parent ?? VisualTreeHelper.GetParent(child);
        if (parent is Panel panel) panel.Children.Remove(child);
        else if (parent is Border border) border.Child = null;
        else if (parent is ContentPresenter presenter) presenter.Content = null;
        else if (parent is ContentControl control) control.Content = null;
    }
}

/// <summary>
/// A ds shadow token drawn under a surface. WinUI's own <c>ThemeShadow</c>
/// takes no offset, blur or colour, so the token's layers are drawn as
/// composition drop shadows, masked to the surface's rounded rectangle, on a
/// host element that sits behind the surface. The token's name stays on the
/// control as <see cref="ShadowProperty"/>, which is what a reader asks.
/// </summary>
public static class Elevation
{
    public static readonly DependencyProperty ShadowProperty = DependencyProperty.RegisterAttached(
        "Shadow", typeof(string), typeof(Elevation), new PropertyMetadata(null));
    /// <summary>The shadow token (<c>2xs</c> … <c>2xl</c>) the kit draws under this control, if any.</summary>
    public static string? GetShadow(DependencyObject element) => (string?)element.GetValue(ShadowProperty);

    private sealed class Lift
    {
        public Theme Theme = Theme.Default;
        public string? Token;
        public double Corner;
    }
    private static readonly ConditionalWeakTable<FrameworkElement, Lift> Lifts = new();

    /// <summary>
    /// Draws <paramref name="token"/> (a <c>shadow.*</c> name less its
    /// prefix) under <paramref name="host"/>, or clears it when null.
    /// </summary>
    internal static void Draw(Control owner, FrameworkElement host, Theme theme, string? token)
    {
        owner.SetValue(ShadowProperty, token);
        if (!Lifts.TryGetValue(host, out var lift))
        {
            lift = new Lift();
            Lifts.Add(host, lift);
            host.SizeChanged += (_, _) => Render(host, lift);
        }
        lift.Theme = theme;
        lift.Token = token;
        lift.Corner = owner.CornerRadius.TopLeft;
        Render(host, lift);
    }

    private static void Render(FrameworkElement host, Lift lift)
    {
        var size = new System.Numerics.Vector2((float)host.ActualWidth, (float)host.ActualHeight);
        if (lift.Token is null || size.X <= 0 || size.Y <= 0)
        {
            Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.SetElementChildVisual(host, null);
            return;
        }
        var compositor = Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.GetElementVisual(host).Compositor;
        var layers = compositor.CreateContainerVisual();
        layers.Size = size;
        foreach (var layer in lift.Theme.Shadow("shadow." + lift.Token).Reverse())
        {
            if (layer.Inset) continue;
            var spread = (float)layer.Spread;
            var shapeSize = new System.Numerics.Vector2(size.X + 2 * spread, size.Y + 2 * spread);
            var geometry = compositor.CreateRoundedRectangleGeometry();
            geometry.Size = shapeSize;
            geometry.CornerRadius = new System.Numerics.Vector2((float)(lift.Corner + layer.Spread));
            var shape = compositor.CreateShapeVisual();
            shape.Size = shapeSize;
            shape.Shapes.Add(compositor.CreateSpriteShape(geometry));
            ((Microsoft.UI.Composition.CompositionSpriteShape)shape.Shapes[0]).FillBrush = compositor.CreateColorBrush(Microsoft.UI.Colors.Black);
            var surface = compositor.CreateVisualSurface();
            surface.SourceVisual = shape;
            surface.SourceSize = shapeSize;
            var shadow = compositor.CreateDropShadow();
            shadow.Mask = compositor.CreateSurfaceBrush(surface);
            shadow.Color = layer.Color;
            shadow.BlurRadius = (float)layer.Blur;
            shadow.Offset = new System.Numerics.Vector3((float)layer.X, (float)layer.Y, 0);
            var sprite = compositor.CreateSpriteVisual();
            sprite.Size = shapeSize;
            sprite.Offset = new System.Numerics.Vector3(-spread, -spread, 0);
            sprite.Shadow = shadow;
            layers.Children.InsertAtTop(sprite);
        }
        Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.SetElementChildVisual(host, layers);
    }
}
