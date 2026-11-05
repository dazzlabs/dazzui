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
    private sealed class TextScaling { }
    private static readonly ConditionalWeakTable<DependencyObject, TextScaling> ResolvedText = new();
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
        {
            if (current is Button && current.GetValue(Button.ThemeProperty) is Theme buttonTheme) return buttonTheme;
            if (current.GetValue(ThemeProperty) is Theme theme) return theme;
        }
        return Theme.Default;
    }
    public static void SetTheme(DependencyObject element, Theme? value) => element.SetValue(ThemeProperty, value);
    public static WidgetTint GetTint(DependencyObject element) => (WidgetTint)element.GetValue(TintProperty);
    public static void SetTint(DependencyObject element, WidgetTint value) => element.SetValue(TintProperty, value);
    public static ControlSize GetSize(DependencyObject element) => (ControlSize)element.GetValue(SizeProperty);
    public static void SetSize(DependencyObject element, ControlSize value) => element.SetValue(SizeProperty, value);
    public static ControlVariant GetVariant(DependencyObject element) => (ControlVariant)element.GetValue(VariantProperty);
    public static void SetVariant(DependencyObject element, ControlVariant value) => element.SetValue(VariantProperty, value);
    /// <summary>Keep host layout in its inherited theme, including live system sizing changes.</summary>
    public static void Follow(FrameworkElement element, Action<Theme> apply) =>
        Bind(element, () => apply(GetTheme(element)));

    internal static void Bind(FrameworkElement element, Action refresh)
    {
        void Apply()
        {
            refresh();
            // Text and glyphs created by custom templates must not apply the
            // accessibility scale a second time after their tokens were resolved.
            UpdateTextScaling(element);
        }
        var pending = 0;
        void SystemChanged()
        {
            if (Interlocked.Exchange(ref pending, 1) != 0) return;
            if (!element.DispatcherQueue.TryEnqueue(() =>
            {
                Interlocked.Exchange(ref pending, 0);
                SystemStyle.RefreshFonts();
                var theme = GetTheme(element);
                if (element.IsLoaded && theme.Sizing == ThemeSizing.System) Apply();
            })) Interlocked.Exchange(ref pending, 0);
        }
        Registrations.Add(element, new Registration(Apply));
        element.Loaded += (_, _) => { SystemStyle.Changed -= SystemChanged; SystemStyle.Changed += SystemChanged; Apply(); };
        element.Unloaded += (_, _) => SystemStyle.Changed -= SystemChanged;
    }
    internal static void UpdateTextScaling(DependencyObject root)
    {
        var theme = GetTheme(root);
        var automatic = theme.Sizing != ThemeSizing.System;
        // Native symbol fonts deliberately opt out of accessibility scaling.
        // Restore only text that this traversal disabled, preserving that opt-out.
        var enabled = root switch { Control control => control.IsTextScaleFactorEnabled, TextBlock text => text.IsTextScaleFactorEnabled, _ => false };
        if (!automatic && enabled)
        {
            ResolvedText.GetOrCreateValue(root);
            if (root is Control control) control.IsTextScaleFactorEnabled = false;
            if (root is TextBlock text) text.IsTextScaleFactorEnabled = false;
        }
        else if (automatic && ResolvedText.Remove(root))
        {
            if (root is Control control) control.IsTextScaleFactorEnabled = true;
            if (root is TextBlock text) text.IsTextScaleFactorEnabled = true;
        }
        // Components own their subtrees; the parent does not overwrite a locally
        // themed component (including its native template descendants).
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
        {
            var child = VisualTreeHelper.GetChild(root, i);
            if (child is FrameworkElement element && Registrations.TryGetValue(element, out _)) continue;
            UpdateTextScaling(child);
        }
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
        UseLayoutRounding = false;
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
        Theme.Typography(this, "bodyMedium");
        RequestedTheme = Theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
    }
}

internal static class Skin
{
    private static readonly ConditionalWeakTable<Control, Dictionary<string, SolidColorBrush>> StateBrushes = new();
    // Visual states retain the resource object they resolved. Updating its
    // colour keeps an already focused, hovered or disabled template current.
    internal static void StateBrush(Control control, string key, SolidColorBrush brush)
    {
        var brushes = StateBrushes.GetOrCreateValue(control);
        if (brushes.TryGetValue(key, out var current)) current.Color = brush.Color;
        else
        {
            brushes[key] = brush;
            control.Resources[key] = brush;
        }
    }
    private sealed class WeightConverter(Control control) : Microsoft.UI.Xaml.Data.IValueConverter
    {
        public object Convert(object value, Type targetType, object parameter, string language) =>
            PlatformFonts.MatchWeight(control.FontFamily, (Windows.UI.Text.FontWeight)value);
        public object ConvertBack(object value, Type targetType, object parameter, string language) =>
            throw new NotSupportedException();
    }
    internal static void TextWeight(FrameworkElement label, Control owner)
    {
        Microsoft.UI.Xaml.Documents.Typography.SetKerning(label, true);
        if (label is TextBlock text) TextStyle.Features(text);
        var property = label is TextBlock ? TextBlock.FontWeightProperty : ContentPresenter.FontWeightProperty;
        label.SetBinding(property, new Microsoft.UI.Xaml.Data.Binding
        {
            Source = owner, Path = new PropertyPath("FontWeight"), Converter = new WeightConverter(owner),
        });
    }
    private sealed class LineBox(ContentControl control)
    {
        internal double Height;
        internal bool Wrap;
        private TextBlock? words;
        internal void Ink(ContentPresenter presenter)
        {
            if (words is not null && ReferenceEquals(control.Content, words))
                words.SetBinding(TextBlock.ForegroundProperty, new Microsoft.UI.Xaml.Data.Binding
                {
                    Source = presenter, Path = new PropertyPath("Foreground"),
                });
        }
        private void AlignBaseline()
        {
            if (words is null || words.ActualHeight <= 0 || !ReferenceEquals(control.Content, words)) return;
            var baseline = PlatformFonts.LineBaseline(words.FontFamily, words.FontSize, words.FontWeight, Height);
            var offset = baseline - words.BaselineOffset;
            if (words.RenderTransform is not TranslateTransform translation || Math.Abs(translation.Y - offset) > 0.001)
                words.RenderTransform = new TranslateTransform { Y = offset };
        }
        internal void Update()
        {
            if (control.Content is string text)
            {
                words = new TextBlock { Text = text, TextWrapping = Wrap ? TextWrapping.Wrap : TextWrapping.NoWrap, LineStackingStrategy = LineStackingStrategy.BlockLineHeight };
                Microsoft.UI.Xaml.Documents.Typography.SetKerning(words, true);
                TextStyle.Features(words);
                if (control is KeyCap) TextStyle.Keyboard(words);
                words.SizeChanged += (_, _) => AlignBaseline();
                words.LayoutUpdated += (_, _) => AlignBaseline();
                foreach (var property in new[] { TextBlock.FontFamilyProperty, TextBlock.FontSizeProperty, TextBlock.FontWeightProperty })
                    words.RegisterPropertyChangedCallback(property, (_, _) => AlignBaseline());
                foreach (var property in new[] { "FontFamily", "FontSize" })
                {
                    var target = property switch
                    {
                        "FontFamily" => TextBlock.FontFamilyProperty,
                        "FontSize" => TextBlock.FontSizeProperty,
                        _ => TextBlock.FontSizeProperty,
                    };
                    words.SetBinding(target, new Microsoft.UI.Xaml.Data.Binding { Source = control, Path = new PropertyPath(property) });
                }
                words.SetBinding(TextBlock.IsTextScaleFactorEnabledProperty, new Microsoft.UI.Xaml.Data.Binding
                {
                    Source = control, Path = new PropertyPath("IsTextScaleFactorEnabled"),
                });
                words.LineHeight = Height;
                control.Content = words;
            }
            if (words is not null && ReferenceEquals(control.Content, words))
            {
                if (control is KeyCap) TextStyle.Keyboard(words);
                words.SetBinding(TextBlock.FontWeightProperty, new Microsoft.UI.Xaml.Data.Binding
                {
                    Source = control, Path = new PropertyPath("FontWeight"), Converter = new WeightConverter(control),
                });
                words.LineHeight = Height;
                words.TextWrapping = Wrap ? TextWrapping.Wrap : TextWrapping.NoWrap;
                AlignBaseline();
            }
        }
    }
    private static readonly ConditionalWeakTable<ContentControl, LineBox> Lines = new();
    internal static void TextInk(ContentControl control, ContentPresenter presenter)
    {
        if (Lines.TryGetValue(control, out var line)) line.Ink(presenter);
    }
    /// <summary>A string label with declared leading, retaining caller-owned rich content.</summary>
    internal static void TextLine(ContentControl control, double height, bool wrap = false)
    {
        if (!Lines.TryGetValue(control, out var line))
        {
            line = new LineBox(control);
            Lines.Add(control, line);
            var owned = line;
            control.RegisterPropertyChangedCallback(ContentControl.ContentProperty, (_, _) => owned.Update());
            control.RegisterPropertyChangedCallback(Microsoft.UI.Xaml.Controls.Control.FontFamilyProperty, (_, _) => owned.Update());
        }
        line.Height = height;
        line.Wrap = wrap;
        line.Update();
    }
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
        // Native templates resolve their disabled/focus state resources
        // against the element's own theme; without this a field keeps the
        // light template's values under a dark DazzUI theme.
        control.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
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
        // Fields use spacing.3 at every size, rather than a button's profile
        // inset. ComboBox positions its value from the outside of its border;
        // TextBox's native viewport already accounts for that border itself.
        var inline = theme.Number("spacing.3") + (control is ComboBox ? control.BorderThickness.Left : 0);
        control.Padding = new Thickness(inline, control.Padding.Top, inline, control.Padding.Bottom);
        // In error the whole field is re-tinted to danger: edge, wash and ink together.
        control.BorderBrush = invalid ? theme.Brush("color.danger.600") : theme.Brush("color.border-strong");
        control.Background = !control.IsEnabled ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "disabled")
            : invalid ? theme.Brush("color.danger.50") : theme.Brush("color.surface-muted");
        control.Foreground = !control.IsEnabled ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled")
            : invalid ? theme.Brush("color.danger.800") : theme.Brush("color.content");
        control.FontWeight = new Windows.UI.Text.FontWeight { Weight = (ushort)theme.Token("bodySmall").GetProperty("fontWeight").GetInt32() };
        foreach (var prefix in new[] { "TextControl", "ComboBox" })
        {
            StateBrush(control, prefix + "Background", (SolidColorBrush)control.Background);
            StateBrush(control, prefix + "BackgroundPointerOver", (SolidColorBrush)control.Background);
            StateBrush(control, prefix + "BackgroundPressed", (SolidColorBrush)control.Background);
            StateBrush(control, prefix + "BackgroundFocused", invalid ? theme.Brush("color.danger.50") : theme.Brush("color.surface"));
            StateBrush(control, prefix + "BackgroundDisabled", theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "disabled"));
            StateBrush(control, prefix + "BorderBrush", (SolidColorBrush)control.BorderBrush);
            StateBrush(control, prefix + "BorderBrushPointerOver", (SolidColorBrush)control.BorderBrush);
            StateBrush(control, prefix + "BorderBrushPressed", (SolidColorBrush)control.BorderBrush);
            StateBrush(control, prefix + "BorderBrushDisabled", (SolidColorBrush)control.BorderBrush);
            StateBrush(control, prefix + "BorderBrushFocused", theme.Recipe(ControlVariant.Outlined, invalid ? WidgetTint.Danger : Appearance.GetTint(control), "border"));
            StateBrush(control, prefix + "Foreground", (SolidColorBrush)control.Foreground);
            foreach (var state in new[] { "PointerOver", "Pressed", "Focused", "FocusedPressed" })
                StateBrush(control, prefix + "Foreground" + state, (SolidColorBrush)control.Foreground);
            StateBrush(control, prefix + "ForegroundDisabled", theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled"));
        }
        var faint = theme.Brush("color.content-faint");
        foreach (var state in new[] { "", "PointerOver", "Focused", "Disabled" })
            StateBrush(control, "TextControlPlaceholderForeground" + state, faint);
        if (control is ComboBox)
        {
            // The native focus border and selection pill otherwise keep the
            // Windows accent even when the field wears another ds family.
            StateBrush(control, "ComboBoxBackgroundBorderBrushFocused", theme.Alpha($"color.primary.{theme.Number("focus.ring.shade")}", theme.Number("focus.ring.alpha")));
            StateBrush(control, "ComboBoxItemPillFillBrush", theme.Brush($"color.{Theme.Key(Appearance.GetTint(control))}.600"));
            foreach (var state in new[] { "", "Focused", "FocusedPressed", "Disabled" })
                StateBrush(control, "ComboBoxDropDownGlyphForeground" + state,
                    state == "Disabled" ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled") : theme.Brush("color.content-subtle"));
        }
        if (control is TextBox text) text.PlaceholderForeground = faint;
        if (control is ComboBox choice) choice.PlaceholderForeground = faint;
        control.Resources["TextControlBorderThemeThickness"] = control.BorderThickness;
        control.Resources["TextControlBorderThemeThicknessFocused"] = control.BorderThickness;
        control.Resources["TextControlThemeMinHeight"] = theme.Number($"control.{Theme.Key(size)}.size");
        InputPadding(control, theme, size);
    }
    internal static void InputPadding(Control control, Theme theme, ControlSize size, bool multiline = false)
    {
        // The design's own block inset, not one derived from the native line
        // box: React gives a field `control.<size>.padding.block` (and a
        // multiline one `spacing.2.5`), and the field's height already holds
        // that line.
        var block = multiline ? theme.Number("spacing.2.5") : theme.Number($"control.{Theme.Key(size)}.padding.block");
        // ComboBox's template puts the value's box inside the field's border,
        // one hairline below the padding Top's origin.
        if (control is ComboBox) block += control.BorderThickness.Top;
        control.Padding = new Thickness(control.Padding.Left, block, control.Padding.Right, block);
    }
    public static StackPanel Column(Theme theme, params UIElement[] children) => Stack(theme, Orientation.Vertical, children);
    public static StackPanel Row(Theme theme, params UIElement[] children) => Stack(theme, Orientation.Horizontal, children);
    internal static StackPanel Stack(Theme theme, Orientation direction, UIElement[] children, StackPanel? existing = null)
    {
        var result = existing ?? new StackPanel();
        result.Orientation = direction;
        result.Spacing = theme.Number("spacing.3");
        Children(result, children);
        return result;
    }
    /// <summary>Update composition without detaching surviving native controls.</summary>
    internal static void Children(Panel panel, IEnumerable<UIElement> children)
    {
        var desired = children.ToArray();
        foreach (var child in panel.Children.Where(child => !desired.Contains(child)).ToArray()) panel.Children.Remove(child);
        for (var i = 0; i < desired.Length; i++)
        {
            if (i < panel.Children.Count && ReferenceEquals(panel.Children[i], desired[i])) continue;
            Detach(desired[i]);
            panel.Children.Insert(i, desired[i]);
        }
    }
    internal static void Content(ContentPresenter host, UIElement? content)
    {
        if (!ReferenceEquals(host.Content, content))
        {
            if (content is not null) Detach(content);
            host.Content = content;
        }
        host.Visibility = content is null ? Visibility.Collapsed : Visibility.Visible;
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
            XamlRoot? root = null;
            var scale = 0d;
            void Changed(XamlRoot sender, XamlRootChangedEventArgs args)
            {
                if (Math.Abs(scale - sender.RasterizationScale) < .001) return;
                scale = sender.RasterizationScale;
                Render(host, lift);
            }
            void Connect()
            {
                if (root == host.XamlRoot) return;
                if (root is not null) root.Changed -= Changed;
                root = host.XamlRoot;
                if (root is not null) { scale = root.RasterizationScale; root.Changed += Changed; }
            }
            host.Loaded += (_, _) => Connect();
            host.Unloaded += (_, _) => { if (root is not null) root.Changed -= Changed; root = null; };
            if (host.IsLoaded) Connect();
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
            // Composition's blur radius spans three standard deviations;
            // CSS's blur length spans two.
            shadow.BlurRadius = (float)(layer.Blur * 1.5);
            var scale = host.XamlRoot?.RasterizationScale ?? 1;
            float Offset(double value) => (float)(Math.Round(value * scale, MidpointRounding.AwayFromZero) / scale);
            shadow.Offset = new System.Numerics.Vector3(Offset(layer.X), Offset(layer.Y), 0);
            var sprite = compositor.CreateSpriteVisual();
            sprite.Size = shapeSize;
            sprite.Offset = new System.Numerics.Vector3(-spread, -spread, 0);
            sprite.Shadow = shadow;
            layers.Children.InsertAtTop(sprite);
        }
        Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.SetElementChildVisual(host, layers);
    }
}
