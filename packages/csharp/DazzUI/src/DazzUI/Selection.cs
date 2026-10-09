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
    /// <summary>A quieter aside set after the label, on its line.</summary>
    public string? Note { get; set; }
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
        Theme.Typography(this, "labelQuiet");
        if (IsChecked == true) FontWeight = Theme.Text("", "labelMedium").FontWeight;
        Foreground = Theme.Brush(IsChecked == true ? "color.content" : "color.content-muted");
        // A disabled row keeps its colours and steps back as a whole.
        Opacity = IsEnabled ? 1 : 0.6;
        if (Note is not null && Content is string label)
            Content = new TextBlock { Inlines = { new Microsoft.UI.Xaml.Documents.Run { Text = label + " " }, new Microsoft.UI.Xaml.Documents.Run { Text = Note, Foreground = Theme.Brush("color.content-faint") } } };
        if (GetTemplateChild("Box") is Border box)
        {
            box.Width = box.Height = Theme.Number($"checkbox.{Skin.SizeKey(Size)}.box");
            box.CornerRadius = new CornerRadius(Theme.Number("checkbox.radius"));
            box.BorderThickness = new Thickness(Theme.Number("checkbox.thickness"));
            box.Margin = new Thickness(0, 0, Theme.Number("spacing.2.5"), 0);
            var marked = IsChecked != false;
            box.Background = marked ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : null;
            box.BorderBrush = marked ? box.Background : Theme.Brush("color.border-muted");
            var ink = Theme.Recipe(ControlVariant.Filled, Tint, "content");
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
        Theme.Typography(this, "labelQuiet");
        if (IsChecked == true) FontWeight = Theme.Text("", "labelMedium").FontWeight;
        Foreground = Theme.Brush(IsChecked == true ? "color.content" : "color.content-muted");
        Opacity = IsEnabled || this is OptionCard ? 1 : 0.6;
        if (GetTemplateChild("Box") is Border box)
        {
            box.Width = box.Height = Theme.Number($"radio.{Skin.SizeKey(Size)}.box");
            box.CornerRadius = new CornerRadius(Theme.Number("radius.full"));
            box.BorderThickness = new Thickness(Theme.Number("radio.thickness"));
            box.Margin = new Thickness(0, 0, Theme.Number("spacing.2.5"), 0);
            box.Background = null;
            box.BorderBrush = IsChecked == true ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : Theme.Brush("color.border-muted");
            if (box.Child is Border dot)
            {
                dot.Width = dot.Height = Theme.Number($"radio.{Skin.SizeKey(Size)}.dot");
                dot.CornerRadius = box.CornerRadius;
                dot.Background = Theme.Recipe(ControlVariant.Filled, Tint, "surface");
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
        foreach (var item in Items) { item.GroupName = groupName; item.Size = Size; item.Tint = Tint; }
        var panel = Orientation == Orientation.Horizontal ? Skin.Row(Theme, Items.Cast<UIElement>().ToArray()) : Skin.Column(Theme, Items.Cast<UIElement>().ToArray());
        panel.Spacing = Theme.Number("spacing.2.5");
        Content = panel;
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
    /// <summary>Whether the toggle shares an edge with the one before or after it, as a group's cells do.</summary>
    internal (bool Before, bool After) Joined { get; set; }
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
        BorderThickness = new Thickness(Theme.Number("spacing.px"));
        // Down is the tinted wash and the label a weight heavier.
        if (IsChecked == true) FontWeight = Theme.Text("", "labelStrong").FontWeight;
        Skin.Join(this, !Joined.Before, !Joined.After);
        Margin = new Thickness(Joined.Before ? -Theme.Number("spacing.px") : 0, 0, 0, 0);
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
        // A disabled switch keeps its colours and steps back as a whole.
        Opacity = IsEnabled ? 1 : 0.6;
        Background = IsChecked == true ? Theme.Recipe(ControlVariant.Filled, Tint, "surface", IsEnabled ? RecipeState : "normal") : Theme.Brush("color.surface-sunken");
        if (GetTemplateChild("Thumb") is Border thumb)
        {
            thumb.Width = thumb.Height = Theme.Number($"switch.{Skin.SizeKey(Size)}.thumb");
            thumb.CornerRadius = CornerRadius;
            thumb.HorizontalAlignment = IsChecked == true ? HorizontalAlignment.Right : HorizontalAlignment.Left;
            thumb.Background = IsChecked == true ? Theme.Recipe(ControlVariant.Filled, Tint, "content") : Theme.Brush("color.content-faint");
        }
    }
}
/// <summary>Toggles joined into one control: the cells share their edges, and several may be on at once.</summary>
public class ToggleGroup : Surface
{
    public ToggleGroup() { Size = ControlSize.Small; Variant = ControlVariant.Recessed; }
    public ControlVariant Variant { get; set; }
    public IList<Toggle> Items { get; } = new List<Toggle>();
    public bool Multiple { get; set; } = true;
    public IReadOnlyList<string> Values => Items.Where(item => item.IsChecked == true).Select(item => item.Tag?.ToString() ?? item.Content?.ToString() ?? "").ToArray();
    protected override void RefreshAppearance()
    {
        for (var i = 0; i < Items.Count; i++)
        {
            var item = Items[i];
            item.Size = Size; item.Tint = Tint; item.Variant = Variant;
            item.Joined = (i > 0, i < Items.Count - 1);
        }
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
        var row = Skin.Row(Theme, Items.Cast<UIElement>().ToArray());
        row.Spacing = 0;
        Content = row;
    }
}
/// <summary>
/// A recessed track with one segment picked out, raised off the track in the
/// surface's lightest step — or, <see cref="Filled"/>, painted with the tint.
/// The segment sits the inset inside the track, its corner the track's less
/// the inset, so the two curves stay concentric. The arrow keys move it.
/// </summary>
public class SegmentedControl : Surface
{
    private const string SegmentTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='RadioButton'><Border Background='{TemplateBinding Background}' CornerRadius='{TemplateBinding CornerRadius}' Padding='{TemplateBinding Padding}'><ContentPresenter Content='{TemplateBinding Content}' HorizontalAlignment='Center' VerticalAlignment='Center'/></Border></ControlTemplate>";
    private readonly StackPanel row = new() { Orientation = Orientation.Horizontal };
    private readonly List<RadioButton> segments = [];
    private readonly string group = Guid.NewGuid().ToString("N");
    private int selectedIndex = -1;
    public IList<string> Items { get; } = new List<string>();
    /// <summary>
    /// A glyph per segment, by index beside <see cref="Items"/>; a segment
    /// past its end, or with null, has none. A glyph is drawn before its
    /// label, a half-step under the profile's icon as Button draws one.
    /// </summary>
    public IList<Glyph?> Icons { get; } = new List<Glyph?>();
    /// <summary>Draw each segment's glyph alone, at the profile's full icon size; its label stays its accessible name.</summary>
    public bool IconOnly { get; set; }
    /// <summary>The segments, in order.</summary>
    public IReadOnlyList<RadioButton> Segments => segments;
    public int SelectedIndex { get => selectedIndex; set { selectedIndex = value; foreach (var (segment, i) in segments.Select((segment, i) => (segment, i))) segment.IsChecked = i == value; if (IsLoaded) Paint(); } }
    public bool Filled { get; set; }
    public event EventHandler? SelectionChanged;
    public SegmentedControl() { Content = row; HorizontalAlignment = HorizontalAlignment.Left; }
    protected override void RefreshAppearance()
    {
        if (segments.Count != Items.Count)
        {
            row.Children.Clear();
            segments.Clear();
            for (var i = 0; i < Items.Count; i++)
            {
                var index = i;
                var segment = new RadioButton { Content = Face(i), GroupName = group, Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(SegmentTemplate), IsChecked = i == selectedIndex };
                if (IconOnly && GlyphAt(i) is not null) Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(segment, Items[i]);
                segment.Checked += (_, _) => { if (selectedIndex == index) return; selectedIndex = index; Paint(); SelectionChanged?.Invoke(this, EventArgs.Empty); };
                segment.KeyDown += (_, args) =>
                {
                    var step = args.Key is VirtualKey.Right or VirtualKey.Down ? 1 : args.Key is VirtualKey.Left or VirtualKey.Up ? -1 : 0;
                    if (step == 0 || segments.Count == 0) return;
                    var next = segments[(index + step + segments.Count) % segments.Count];
                    next.IsChecked = true;
                    next.Focus(FocusState.Keyboard);
                    args.Handled = true;
                };
                segments.Add(segment);
                row.Children.Add(segment);
            }
        }
        Paint();
    }
    private Glyph? GlyphAt(int index) => index < Icons.Count ? Icons[index] : null;

    /// <summary>A segment's face: its label, its glyph alone, or the glyph before the label.</summary>
    private object Face(int index)
    {
        if (GlyphAt(index) is not Glyph glyph) return Items[index];
        var icon = new Icon { Glyph = glyph };
        if (IconOnly) return icon;
        var line = new StackPanel { Orientation = Orientation.Horizontal };
        line.Children.Add(icon);
        line.Children.Add(new TextBlock { Text = Items[index], VerticalAlignment = VerticalAlignment.Center });
        icon.VerticalAlignment = VerticalAlignment.Center;
        return line;
    }

    private void Paint()
    {
        var inset = Theme.Number("segmented-control.inset");
        var height = Theme.Number($"control.{Theme.Key(Size)}.size");
        // At medium the track is a container's corner, one step up from a button's.
        var radius = Math.Min(Theme.Number(Size == ControlSize.Medium ? "radius.medium" : $"control.{Theme.Key(Size)}.radius"), height / 2);
        Height = height;
        Padding = new Thickness(inset);
        Background = Theme.Brush("color.surface-inset");
        CornerRadius = new CornerRadius(radius);
        Theme.Typography(this, $"control.{Theme.Key(Size)}.content");
        for (var i = 0; i < segments.Count; i++)
        {
            var segment = segments[i];
            var chosen = i == selectedIndex;
            segment.Height = height - 2 * inset;
            segment.MinWidth = segment.MinHeight = 0;
            // A glyph beside a label: drawn `control.label-glyph.shrink` under
            // the profile's icon, spaced the profile's gap, and the end it
            // sits at `control.label-glyph.inset` closer. Alone it is full size.
            var pad = Theme.Number("spacing.3");
            var iconSize = Theme.Number($"control.{Theme.Key(Size)}.icon-size");
            var lead = pad;
            if (segment.Content is Icon only) Lay(only, iconSize);
            else if (segment.Content is StackPanel { Children: [Icon leading, ..] } line)
            {
                Lay(leading, iconSize - Theme.Number("control.label-glyph.shrink"));
                line.Spacing = Theme.Number($"control.{Theme.Key(Size)}.gap");
                lead = pad - Theme.Number("control.label-glyph.inset");
            }
            segment.Padding = new Thickness(lead, 0, pad, 0);
            segment.CornerRadius = new CornerRadius(Math.Max(0, radius - inset));
            segment.FontSize = FontSize;
            segment.FontWeight = Theme.Text("", chosen ? "labelMedium" : "labelQuiet").FontWeight;
            segment.Background = chosen ? Filled ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : Theme.Brush("color.surface-raised") : null;
            segment.Foreground = chosen ? Filled ? Theme.Recipe(ControlVariant.Filled, Tint, "content") : Theme.Brush("color.content") : Theme.Brush("color.content-secondary");
            Skin.Focus(segment, Theme);
        }
    }

    private static void Lay(Icon icon, double size)
    {
        if (icon.GlyphSize == size) return;
        icon.GlyphSize = size;
        Appearance.Refresh(icon);
    }
}
/// <summary>
/// Free-standing filter pills. There is no track: each pill stands on the page
/// with its own fill, the active one filled with the accent and a weight
/// heavier. One size, drawn tight. The arrow keys move the choice.
/// </summary>
public sealed class PillTabs : Surface
{
    private const string PillTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='RadioButton'><Border Background='{TemplateBinding Background}' CornerRadius='{TemplateBinding CornerRadius}' Padding='{TemplateBinding Padding}'><ContentPresenter Content='{TemplateBinding Content}' HorizontalAlignment='Center' VerticalAlignment='Center'/></Border></ControlTemplate>";
    private readonly StackPanel row = new() { Orientation = Orientation.Horizontal };
    private readonly string group = Guid.NewGuid().ToString("N");
    private int selectedIndex = -1;
    public IList<string> Items { get; } = new List<string>();
    public int SelectedIndex { get => selectedIndex; set { selectedIndex = value; Sync(); } }
    public event EventHandler? SelectionChanged;
    public PillTabs() { Content = row; HorizontalAlignment = HorizontalAlignment.Left; }
    private void Sync()
    {
        for (var i = 0; i < row.Children.Count; i++)
            if (row.Children[i] is RadioButton pill) pill.IsChecked = i == selectedIndex;
        if (IsLoaded) Paint();
    }
    protected override void RefreshAppearance()
    {
        if (row.Children.Count != Items.Count)
        {
            row.Children.Clear();
            for (var i = 0; i < Items.Count; i++)
            {
                var index = i;
                var pill = new RadioButton { Content = Items[i], GroupName = group, Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(PillTemplate), IsChecked = i == selectedIndex };
                pill.Checked += (_, _) => { if (selectedIndex == index) return; selectedIndex = index; Paint(); SelectionChanged?.Invoke(this, EventArgs.Empty); };
                pill.KeyDown += (_, args) =>
                {
                    var step = args.Key is VirtualKey.Right or VirtualKey.Down ? 1 : args.Key is VirtualKey.Left or VirtualKey.Up ? -1 : 0;
                    if (step == 0 || Items.Count == 0) return;
                    var next = (index + step + Items.Count) % Items.Count;
                    ((RadioButton)row.Children[next]).IsChecked = true;
                    ((RadioButton)row.Children[next]).Focus(FocusState.Keyboard);
                    args.Handled = true;
                };
                row.Children.Add(pill);
            }
        }
        row.Spacing = Theme.Number("spacing.2");
        Paint();
    }
    private void Paint()
    {
        var height = Theme.Number("control.tiny.size");
        var size = Theme.Text("", "labelSmall").FontSize;
        for (var i = 0; i < row.Children.Count; i++)
        {
            if (row.Children[i] is not RadioButton pill) continue;
            var chosen = i == selectedIndex;
            pill.Height = height;
            pill.MinWidth = pill.MinHeight = 0;
            pill.Padding = new Thickness(Theme.Number("spacing.3"), 0, Theme.Number("spacing.3"), 0);
            pill.CornerRadius = new CornerRadius(height / 2);
            pill.FontSize = size;
            pill.FontWeight = Theme.Text("", chosen ? "labelStrong" : "labelSmall").FontWeight;
            pill.Background = chosen ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : Theme.Brush("color.surface-inset");
            pill.Foreground = chosen ? Theme.Recipe(ControlVariant.Filled, Tint, "content") : Theme.Brush("color.content-nav");
            Skin.Focus(pill, Theme);
        }
    }
}
/// <summary>
/// A selectable card for one of a few choices that each need a line of
/// explanation: the title over its description, on the muted surface; chosen,
/// the tint's wash with the accent drawn around it at the control stroke.
/// </summary>
public sealed class OptionCard : Radio
{
    private bool hovered;
    public string Title { get; set; } = "";
    public string Description { get; set; } = "";
    public OptionCard()
    {
        DefaultStyleKey = typeof(OptionCard);
        PointerEntered += (_, _) => { hovered = true; Apply(); };
        PointerExited += (_, _) => { hovered = false; Apply(); };
    }
    protected override void Apply()
    {
        base.Apply();
        if (GetTemplateChild("Box") is FrameworkElement box) box.Visibility = Visibility.Collapsed;
        var chosen = IsChecked == true;
        Background = !IsEnabled ? Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "disabled")
            : chosen ? Theme.Alpha($"color.{Theme.Key(Tint)}.600", Theme.Number("wash.surface"))
            : Theme.Brush(hovered ? "color.surface-subtle" : "color.surface-muted");
        BorderBrush = chosen ? Theme.Recipe(ControlVariant.Outlined, Tint, "border") : Theme.Brush("color.border");
        BorderThickness = new Thickness(Theme.Number(chosen ? "stroke.control" : "stroke.hairline"));
        Skin.SetCorner(this, Theme.Number("radius.large"));
        // The border is inside the box, so the thicker chosen one gives its
        // difference back out of the padding: the content stays where it was.
        var give = chosen ? Theme.Number("stroke.control") - Theme.Number("stroke.hairline") : 0;
        Padding = new Thickness(Theme.Number("spacing.3") - give, Theme.Number("spacing.3.5") - give, Theme.Number("spacing.3") - give, Theme.Number("spacing.3.5") - give);
        var title = Theme.Text(Title, "labelStrong");
        title.Foreground = Theme.Brush("color.content");
        var description = Theme.Text(Description, "labelSmall");
        description.FontWeight = Theme.Text("", "captionSmall").FontWeight;
        description.LineHeight = description.FontSize * 1.5;
        description.Foreground = Theme.Brush(chosen ? "color.content-muted" : "color.content-subtle");
        var body = new StackPanel { Spacing = Theme.Number("spacing.1.5") };
        body.Children.Add(title);
        if (Description.Length > 0) body.Children.Add(description);
        Content = body;
        HorizontalContentAlignment = HorizontalAlignment.Stretch;
    }
}
/// <summary>
/// A chip you can press or dismiss: the badge's capsule and type, the tinted
/// recipe's colours through their states, an accent edge when selected, and a
/// faint dismiss inside the trailing padding.
/// </summary>
public sealed class Tag : Surface
{
    private const string LabelTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='ToggleButton'><ContentPresenter Content='{TemplateBinding Content}' Background='Transparent' VerticalAlignment='Center'/></ControlTemplate>";
    private readonly Microsoft.UI.Xaml.Controls.Button remove = new() { Padding = new Thickness(0), MinWidth = 0, MinHeight = 0, BorderThickness = new Thickness(0), VerticalAlignment = VerticalAlignment.Center, Opacity = 0.6 };
    private bool hovered;
    /// <summary>The label's own hit target, which latches when the tag is selectable.</summary>
    public ToggleButton Selection { get; } = new();
    public bool Removable { get; set; }
    public ControlVariant Variant { get; set; } = ControlVariant.Tinted;
    public string Text { get => Selection.Content?.ToString() ?? ""; set => Selection.Content = value; }
    public bool IsSelected { get => Selection.IsChecked == true; set => Selection.IsChecked = value; }
    public event EventHandler? Removed;
    public Tag()
    {
        Selection.Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(LabelTemplate);
        remove.Content = new Icon { Glyph = FluentIcons.Common.Icon.Dismiss };
        remove.Click += (_, _) => Removed?.Invoke(this, EventArgs.Empty);
        Content = new StackPanel { Orientation = Orientation.Horizontal, Children = { Selection, remove } };
        HorizontalAlignment = HorizontalAlignment.Left;
        Selection.Checked += (_, _) => RefreshAppearance();
        Selection.Unchecked += (_, _) => RefreshAppearance();
        PointerEntered += (_, _) => { hovered = true; RefreshAppearance(); };
        PointerExited += (_, _) => { hovered = false; RefreshAppearance(); };
    }
    protected override void RefreshAppearance()
    {
        if (!IsLoaded) return;
        var state = !IsEnabled ? "disabled" : hovered && Selection.IsHitTestVisible ? "hovered" : "normal";
        Background = Theme.Recipe(Variant, Tint, "surface", state);
        Foreground = Theme.Recipe(Variant, Tint, "content", state);
        BorderBrush = IsSelected ? Theme.Recipe(ControlVariant.Outlined, Tint, "border") : new SolidColorBrush(Microsoft.UI.Colors.Transparent);
        BorderThickness = new Thickness(Theme.Number("spacing.px"));
        var large = Size == ControlSize.Large;
        var block = Theme.Number(large ? "spacing.1" : "spacing.0.5");
        var inline = Theme.Number(Size is ControlSize.Small or ControlSize.Tiny ? "spacing.1.5" : "spacing.2");
        Padding = new Thickness(inline, block, inline, block);
        Skin.SetCorner(this, Theme.Number("radius.tiny"));
        Theme.Typography(Selection, large ? "labelMedium" : "labelSmall");
        Selection.FontWeight = Theme.Text("", "labelMedium").FontWeight;
        Selection.Foreground = Foreground;
        Skin.Focus(Selection, Theme);
        remove.Visibility = Removable ? Visibility.Visible : Visibility.Collapsed;
        remove.Margin = new Thickness(Theme.Number("spacing.1"), 0, 0, 0);
        remove.Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent);
        remove.Foreground = Foreground;
        remove.CornerRadius = new CornerRadius(Theme.Number("radius.full"));
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(remove, $"Remove {Text}");
        if (remove.Content is Icon glyph) glyph.GlyphSize = Theme.Number("icon.small");
    }
}
