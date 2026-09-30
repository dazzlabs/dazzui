using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Input;
using Windows.System;

namespace DazzUI;

public class Checkbox : CheckBox
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public Checkbox()
    {
        DefaultStyleKey = typeof(Checkbox);
        Appearance.Bind(this, Apply);
        Checked += (_, _) => Apply(); Unchecked += (_, _) => Apply(); Indeterminate += (_, _) => Apply(); IsEnabledChanged += (_, _) => Apply();
    }
    protected override void OnApplyTemplate() { base.OnApplyTemplate(); Apply(); }
    private void Apply()
    {
        Skin.Focus(this, Theme); MinWidth = MinHeight = 0;
        Theme.Typography(this, IsChecked == true ? "labelMedium" : "labelQuiet");
        Foreground = Theme.Brush(IsEnabled ? IsChecked == true ? "color.content" : "color.content-muted" : "color.content-faint");
        if (GetTemplateChild("Box") is Border box)
        {
            box.Width = box.Height = Theme.Number($"checkbox.{Skin.SizeKey(Size)}.box");
            box.CornerRadius = new CornerRadius(Theme.Number("checkbox.radius"));
            box.BorderThickness = new Thickness(Theme.Number("checkbox.thickness"));
            box.Margin = new Thickness(0, 0, Theme.Number("spacing.2.5"), 0);
            var marked = IsChecked != false;
            box.Background = !IsEnabled ? Theme.Brush("color.surface-sunken") : marked ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : null;
            box.BorderBrush = !IsEnabled ? null : marked ? box.Background : Theme.Brush("color.border-muted");
            var ink = Theme.Recipe(ControlVariant.Filled, Tint, "content", IsEnabled ? "normal" : "disabled");
            box.Child = !marked ? null : IsChecked is null
                ? new Border { Background = ink, Width = box.Width - Theme.Number("spacing.2"), Height = Theme.Number("stroke.control"), HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center }
                : new Icon { Glyph = FluentIcons.Common.Icon.Checkmark, GlyphSize = box.Width - Theme.Number("spacing.1"), Foreground = ink, HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
        }
    }
}
public sealed class CheckboxGroup : Surface
{
    public IList<Checkbox> Items { get; } = new List<Checkbox>();
    public IReadOnlyList<string> Values => Items.Where(item => item.IsChecked == true).Select(item => item.Tag?.ToString() ?? item.Content?.ToString() ?? "").ToArray();
    public event EventHandler? SelectionChanged;
    public CheckboxGroup() { }
    protected override void RefreshAppearance()
    {
        if (Content is null)
            foreach (var item in Items) { item.Checked += (_, _) => SelectionChanged?.Invoke(this, EventArgs.Empty); item.Unchecked += (_, _) => SelectionChanged?.Invoke(this, EventArgs.Empty); }
        Content = Skin.Column(Theme, Items.Cast<UIElement>().ToArray());
    }
}
public class Radio : RadioButton
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public Radio()
    {
        DefaultStyleKey = typeof(Radio); Appearance.Bind(this, Apply);
        Checked += (_, _) => Apply(); Unchecked += (_, _) => Apply(); IsEnabledChanged += (_, _) => Apply();
    }
    protected override void OnApplyTemplate() { base.OnApplyTemplate(); Apply(); }
    protected virtual void Apply()
    {
        Skin.Focus(this, Theme); MinWidth = MinHeight = 0;
        Theme.Typography(this, IsChecked == true ? "labelMedium" : "labelQuiet");
        Foreground = Theme.Brush(IsEnabled ? "color.content" : "color.content-faint");
        if (GetTemplateChild("Box") is Border box)
        {
            box.Width = box.Height = Theme.Number($"radio.{Skin.SizeKey(Size)}.box");
            box.CornerRadius = new CornerRadius(Theme.Number("radius.full"));
            box.BorderThickness = new Thickness(Theme.Number("radio.thickness"));
            box.Margin = new Thickness(0, 0, Theme.Number("spacing.2.5"), 0);
            box.Background = IsEnabled ? null : Theme.Brush("color.surface-sunken");
            box.BorderBrush = !IsEnabled ? null : IsChecked == true ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : Theme.Brush("color.border-muted");
            if (box.Child is Border dot)
            {
                dot.Width = dot.Height = Theme.Number($"radio.{Skin.SizeKey(Size)}.dot");
                dot.CornerRadius = box.CornerRadius;
                dot.Background = Theme.Recipe(ControlVariant.Filled, Tint, "surface", IsEnabled ? "normal" : "disabled");
                dot.Visibility = IsChecked == true ? Visibility.Visible : Visibility.Collapsed;
            }
        }
    }
}
public sealed class RadioGroup : Surface
{
    private readonly string groupName = Guid.NewGuid().ToString("N");
    public IList<Radio> Items { get; } = new List<Radio>();
    public Orientation Orientation { get; set; }
    public string? Value { get => Items.FirstOrDefault(item => item.IsChecked == true)?.Tag?.ToString(); set { foreach (var item in Items) item.IsChecked = Equals(item.Tag?.ToString(), value); } }
    protected override void RefreshAppearance()
    {
        foreach (var item in Items) item.GroupName = groupName;
        Content = Orientation == Orientation.Horizontal ? Skin.Row(Theme, Items.Cast<UIElement>().ToArray()) : Skin.Column(Theme, Items.Cast<UIElement>().ToArray());
    }
}
public class Toggle : ToggleButton
{
    private bool hovered;
    private bool pressed;
    protected string RecipeState => !IsEnabled ? "disabled" : pressed ? "pressed" : hovered ? "hovered" : "normal";
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public ControlVariant Variant { get => Appearance.GetVariant(this); set => Appearance.SetVariant(this, value); }
    public Toggle()
    {
        DefaultStyleKey = typeof(Toggle); Appearance.Bind(this, Apply);
        Size = ControlSize.Tiny;
        Checked += (_, _) => Apply(); Unchecked += (_, _) => Apply(); IsEnabledChanged += (_, _) => Apply();
    }
    protected virtual void Apply()
    {
        Skin.Control(this, Theme, Size); Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        // A glyph alone is a square control: the inline padding is for a label.
        if (Content is Icon && this is not NavItem) { Padding = new Thickness(0); Width = Height; }
        var variant = IsChecked == true ? ControlVariant.Tinted : Variant;
        var state = RecipeState;
        Background = Theme.Recipe(variant, Tint, "surface", state);
        Foreground = Theme.Recipe(variant, Tint, "content", state);
        BorderBrush = Theme.Recipe(variant, Tint, "border", state);
        BorderThickness = new Thickness(Theme.Number("stroke.control"));
    }
    protected override void OnPointerEntered(PointerRoutedEventArgs args) { base.OnPointerEntered(args); hovered = true; Apply(); }
    protected override void OnPointerExited(PointerRoutedEventArgs args) { base.OnPointerExited(args); hovered = false; pressed = false; Apply(); }
    protected override void OnPointerPressed(PointerRoutedEventArgs args) { base.OnPointerPressed(args); pressed = true; Apply(); }
    protected override void OnPointerReleased(PointerRoutedEventArgs args) { base.OnPointerReleased(args); pressed = false; Apply(); }
    protected override void OnPointerCaptureLost(PointerRoutedEventArgs args) { base.OnPointerCaptureLost(args); pressed = false; Apply(); }
    protected override void OnKeyDown(KeyRoutedEventArgs args) { base.OnKeyDown(args); if (args.Key == VirtualKey.Space) { pressed = true; Apply(); } }
    protected override void OnKeyUp(KeyRoutedEventArgs args) { base.OnKeyUp(args); if (args.Key == VirtualKey.Space) { pressed = false; Apply(); } }
}
public sealed class Switch : Toggle
{
    public Switch() { DefaultStyleKey = typeof(Switch); Size = ControlSize.Medium; }
    protected override void OnApplyTemplate() { base.OnApplyTemplate(); Apply(); }
    protected override void Apply()
    {
        Skin.Focus(this, Theme);
        Width = Theme.Number($"switch.{Skin.SizeKey(Size)}.width");
        Height = Theme.Number($"switch.{Skin.SizeKey(Size)}.height");
        MinWidth = MinHeight = 0; Padding = new Thickness(Theme.Number("spacing.0.5"));
        Skin.SetCorner(this, Theme.Number("radius.full"));
        Background = IsChecked == true ? Theme.Recipe(ControlVariant.Filled, Tint, "surface", RecipeState) : Theme.Brush("color.surface-sunken");
        if (GetTemplateChild("Thumb") is Border thumb)
        {
            thumb.Width = thumb.Height = Theme.Number($"switch.{Skin.SizeKey(Size)}.thumb");
            thumb.CornerRadius = CornerRadius;
            thumb.HorizontalAlignment = IsChecked == true ? HorizontalAlignment.Right : HorizontalAlignment.Left;
            thumb.Background = IsChecked == true ? Theme.Recipe(ControlVariant.Filled, Tint, "content") : Theme.Brush("color.content-faint");
        }
    }
}
public class ToggleGroup : Surface
{
    public ToggleGroup() { Size = ControlSize.Small; }
    public IList<Toggle> Items { get; } = new List<Toggle>();
    public bool Multiple { get; set; } = true;
    public IReadOnlyList<string> Values => Items.Where(item => item.IsChecked == true).Select(item => item.Tag?.ToString() ?? item.Content?.ToString() ?? "").ToArray();
    protected override void RefreshAppearance()
    {
        foreach (var item in Items) { item.Size = Size; item.Tint = Tint; }
        if (Content is null)
        {
            foreach (var item in Items)
            {
                item.Checked += (_, _) => { if (!Multiple) foreach (var other in Items.Where(other => other != item)) other.IsChecked = false; };
                item.KeyDown += (_, args) =>
                {
                    var direction = args.Key is VirtualKey.Right or VirtualKey.Down ? 1 : args.Key is VirtualKey.Left or VirtualKey.Up ? -1 : 0;
                    if (direction == 0) return;
                    var enabled = Items.Where(child => child.IsEnabled).ToArray();
                    if (enabled.Length == 0) return;
                    var next = enabled[(Array.IndexOf(enabled, item) + direction + enabled.Length) % enabled.Length];
                    next.Focus(FocusState.Keyboard); if (!Multiple) next.IsChecked = true; args.Handled = true;
                };
            }
        }
        Content = Skin.Row(Theme, Items.Cast<UIElement>().ToArray());
    }
}
public class SegmentedControl : ListBox
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public bool Filled { get; set; }
    public SegmentedControl()
    {
        ItemsPanel = (ItemsPanelTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load("<ItemsPanelTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'><StackPanel Orientation='Horizontal'/></ItemsPanelTemplate>");
        Appearance.Bind(this, Apply); SelectionChanged += (_, _) => Apply();
        RegisterPropertyChangedCallback(CornerRadiusProperty, (_, _) => { if (GetTemplateChild("LayoutRoot") is Border border) border.CornerRadius = CornerRadius; });
    }
    protected override void OnApplyTemplate()
    {
        base.OnApplyTemplate();
        // WinUI's ListBox template omits CornerRadius on its background border.
        if (GetTemplateChild("LayoutRoot") is Border border)
            border.CornerRadius = CornerRadius;
    }
    protected override void PrepareContainerForItemOverride(DependencyObject element, object item)
    {
        base.PrepareContainerForItemOverride(element, item);
        if (element is ListBoxItem container)
        {
            container.Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load("<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='ListBoxItem'><ContentPresenter Content='{TemplateBinding Content}' Padding='{TemplateBinding Padding}' Background='{TemplateBinding Background}' Foreground='{TemplateBinding Foreground}' CornerRadius='{TemplateBinding CornerRadius}' HorizontalContentAlignment='Center' VerticalContentAlignment='Center'/></ControlTemplate>");
            container.RegisterPropertyChangedCallback(ListBoxItem.IsSelectedProperty, (_, _) => ApplyItem(container));
            ApplyItem(container);
        }
    }
    private void Apply()
    {
        Skin.Control(this, Theme, Size); Padding = new Thickness(Theme.Number("segmented-control.inset"));
        Background = this is PillTabs ? null : Theme.Brush("color.surface-sunken"); BorderThickness = new Thickness(0);
        if (this is PillTabs) Padding = new Thickness(0);
        for (var i = 0; i < Items.Count; i++)
            if (ContainerFromIndex(i) is ListBoxItem item) ApplyItem(item);
    }
    private void ApplyItem(ListBoxItem item)
    {
        Skin.Control(item, Theme, Size);
        var selected = item.IsSelected;
        var pill = this is PillTabs;
        if (pill) Skin.SetCorner(item, Theme.Number("radius.full"));
        else Skin.SetCorner(item, Math.Max(0, Theme.Number($"control.{Theme.Key(Size)}.radius") - Theme.Number("segmented-control.inset")));
        Theme.Typography(item, selected && pill ? "labelStrong" : selected ? "labelMedium" : "labelQuiet");
        item.Background = selected ? Filled ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : pill ? Theme.Recipe(ControlVariant.Tinted, Tint, "surface") : Theme.Brush("color.surface") : null;
        item.Foreground = selected ? Filled ? Theme.Recipe(ControlVariant.Filled, Tint, "content") : pill ? Theme.Recipe(ControlVariant.Tinted, Tint, "content") : Theme.Brush("color.content") : Theme.Brush("color.content-muted");
    }
}
public sealed class PillTabs : SegmentedControl { }
public sealed class OptionCard : Radio
{
    public string Title { get; set; } = "";
    public string Description { get; set; } = "";
    public OptionCard()
    {
        DefaultStyleKey = typeof(OptionCard);
    }
    protected override void Apply()
    {
        base.Apply();
        var state = IsEnabled ? "normal" : "disabled";
        Background = Theme.Recipe(IsChecked == true ? ControlVariant.Tinted : ControlVariant.Normal, Tint, "surface", state);
        BorderBrush = IsChecked == true ? Theme.Recipe(ControlVariant.Outlined, Tint, "border", state) : Theme.Brush("color.border");
        BorderThickness = new Thickness(Theme.Number("stroke.control"));
        Skin.SetCorner(this, Theme.Number("radius.large"));
        Padding = new Thickness(Theme.Number("spacing.4"));
        Content = Skin.Column(Theme, Theme.Text(Title, IsChecked == true ? "labelStrong" : "labelMedium"), Theme.Text(Description, "bodySmall"));
    }
}
public sealed class Tag : Surface
{
    public Toggle Selection { get; } = new();
    public bool Removable { get; set; }
    public string Text { get => Selection.Content?.ToString() ?? ""; set => Selection.Content = value; }
    public bool IsSelected { get => Selection.IsChecked == true; set => Selection.IsChecked = value; }
    public event EventHandler? Removed;
    protected override void RefreshAppearance()
    {
        Selection.Size = Size; Selection.Tint = Tint;
        var body = Skin.Row(Theme, Selection);
        if (Removable)
        {
            var remove = new Button { Content = "×", Variant = ControlVariant.Plain, Size = ControlSize.Tiny };
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(remove, $"Remove {Text}");
            remove.Click += (_, _) => Removed?.Invoke(this, EventArgs.Empty);
            body.Children.Add(remove);
        }
        Content = body;
    }
}
