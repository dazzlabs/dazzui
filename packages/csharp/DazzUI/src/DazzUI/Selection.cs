using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Input;
using Windows.System;
using Windows.UI.Text;

namespace DazzUI;

public class Checkbox : CheckBox
{
    private TextBlock? noteLabel;
    private Microsoft.UI.Xaml.Documents.Run? noteRun;
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
        if (IsChecked == true) FontWeight = Theme.Weight("labelMedium");
        Foreground = Theme.Brush(IsChecked == true ? "color.content" : "color.content-muted");
        // A disabled row keeps its colours and steps back as a whole.
        Opacity = IsEnabled ? 1 : 0.6;
        if (GetTemplateChild("Row") is StackPanel row) row.Spacing = Theme.Number("spacing.2.5");
        if (Note is not null && Content is string label)
        {
            noteRun = new Microsoft.UI.Xaml.Documents.Run();
            noteLabel = new TextBlock { Inlines = { new Microsoft.UI.Xaml.Documents.Run { Text = label + " " }, noteRun } };
            Content = noteLabel;
        }
        if (noteLabel is not null && ReferenceEquals(Content, noteLabel))
        {
            Theme.Text("", "labelQuiet").CopyTo(noteLabel);
            Skin.TextWeight(noteLabel, this);
            noteRun!.Text = Note ?? "";
            noteRun.Foreground = Theme.Brush("color.content-faint");
        }
        else Skin.TextLine(this, Theme.Text("", "labelQuiet").LineHeight);
        if (GetTemplateChild("Box") is Border box)
        {
            box.Width = box.Height = Theme.Number($"checkbox.{Skin.SizeKey(Size)}.box");
            box.CornerRadius = new CornerRadius(Theme.Number("checkbox.radius"));
            box.BorderThickness = new Thickness(Theme.Number("checkbox.thickness"));
            box.Margin = new Thickness(0);
            var marked = IsChecked != false;
            box.Background = marked ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : null;
            box.BorderBrush = marked ? box.Background : Theme.Brush("color.border-muted");
            var ink = Theme.Recipe(ControlVariant.Filled, Tint, "content");
            var markSize = Math.Min(Theme.Number("icon.small"), Math.Max(0, box.Width - 2 * box.BorderThickness.Left));
            // React scales the filled twelve-pixel tick/dash inside the box's
            // content area, rather than changing glyph grids with the box size.
            var mark = Icon.Element(IsChecked is null ? FluentIcons.Common.Icon.Subtract : FluentIcons.Common.Icon.Checkmark, FluentIcons.Common.IconVariant.Filled);
            mark.IconSize = FluentIcons.Common.IconSize.Size12;
            mark.FontSize = mark.Width = mark.Height = markSize;
            mark.Foreground = ink;
            mark.HorizontalAlignment = HorizontalAlignment.Center;
            mark.VerticalAlignment = VerticalAlignment.Center;
            box.Child = marked ? mark : null;
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
        var column = Skin.Stack(Theme, Orientation.Vertical, Items.Cast<UIElement>().ToArray(), Content as StackPanel);
        column.Spacing = Theme.Number("spacing.2.5");
        Content = column;
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
        Skin.TextLine(this, Theme.Text("", "labelQuiet").LineHeight);
        if (IsChecked == true) FontWeight = Theme.Weight("labelMedium");
        Foreground = Theme.Brush(IsChecked == true ? "color.content" : "color.content-muted");
        Opacity = IsEnabled || this is OptionCard ? 1 : 0.6;
        if (GetTemplateChild("Row") is StackPanel row) row.Spacing = Theme.Number("spacing.2.5");
        if (GetTemplateChild("Box") is Border box)
        {
            box.Width = box.Height = Theme.Number($"radio.{Skin.SizeKey(Size)}.box");
            box.CornerRadius = new CornerRadius(Theme.Number("radius.full"));
            box.BorderThickness = new Thickness(Theme.Number("radio.thickness"));
            box.Margin = new Thickness(0);
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
        var panel = Skin.Stack(Theme, Orientation, Items.Cast<UIElement>().ToArray(), Content as StackPanel);
        panel.Spacing = Theme.Number("spacing.2.5");
        Content = panel;
    }
}
public class Toggle : ToggleButton
{
    private bool hovered;
    private bool pressed;
    private bool glyphWidth;
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
    protected override void OnApplyTemplate()
    {
        base.OnApplyTemplate();
        if (GetTemplateChild("Presenter") is ContentPresenter presenter) Skin.TextWeight(presenter, this);
    }
    protected virtual void Apply()
    {
        Skin.Control(this, Theme, Size); Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        // A glyph alone is a square control: the padding is for a label.
        if (Content is Icon icon && this is not NavItem)
        {
            Padding = new Thickness(0); Width = Height;
            glyphWidth = true;
            icon.GlyphSize = Theme.Number($"control.{Theme.Key(Size)}.icon-size");
            Appearance.Refresh(icon);
        }
        else
        {
            if (glyphWidth) { Width = double.NaN; glyphWidth = false; }
            var profile = $"control.{Theme.Key(Size)}";
            Padding = new Thickness(Theme.Number(profile + ".padding.inline"), Theme.Number(profile + ".padding.block"),
                Theme.Number(profile + ".padding.inline"), Theme.Number(profile + ".padding.block"));
        }
        var variant = IsChecked == true ? ControlVariant.Tinted : Variant;
        var state = RecipeState;
        Background = Theme.Recipe(variant, Tint, "surface", state);
        Foreground = Theme.Recipe(variant, Tint, "content", state);
        BorderBrush = Theme.Recipe(variant, Tint, "border", state);
        BorderThickness = new Thickness(Theme.Number("spacing.px"));
        // Down is the tinted wash and the label a weight heavier.
        if (IsChecked == true && IsEnabled) FontWeight = Theme.Weight("labelStrong");
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
        Background = IsChecked == true ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : Theme.Brush("color.surface-sunken");
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
        var row = Skin.Stack(Theme, Orientation.Horizontal, Items.Cast<UIElement>().ToArray(), Content as StackPanel);
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
    private const string SegmentTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='RadioButton'><Grid><Border x:Name='Lift' xmlns:x='http://schemas.microsoft.com/winfx/2006/xaml' IsHitTestVisible='False'/><Border Background='{TemplateBinding Background}' CornerRadius='{TemplateBinding CornerRadius}' Padding='{TemplateBinding Padding}'><ContentPresenter Content='{TemplateBinding Content}' HorizontalAlignment='Center' VerticalAlignment='Center'/></Border></Grid></ControlTemplate>";
    private readonly Grid row = new();
    private readonly List<RadioButton> segments = [];
    private readonly string group = Guid.NewGuid().ToString("N");
    private int selectedIndex = -1;
    public IList<string> Items { get; } = new List<string>();
    /// <summary>Indices unavailable for pointer, keyboard and automation activation.</summary>
    public ISet<int> DisabledItems { get; } = new HashSet<int>();
    /// <summary>Fill the available width and divide it equally among the segments.</summary>
    public bool Stretch { get; set; }
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
            row.ColumnDefinitions.Clear();
            segments.Clear();
            for (var i = 0; i < Items.Count; i++)
            {
                var index = i;
                var segment = new ChoiceButton { Content = Face(i), GroupName = group, Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(SegmentTemplate), IsChecked = i == selectedIndex, AppearanceChanged = Paint };
                if (IconOnly && GlyphAt(i) is not null) Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(segment, Items[i]);
                segment.Checked += (_, _) => { if (selectedIndex == index) return; selectedIndex = index; Paint(); SelectionChanged?.Invoke(this, EventArgs.Empty); };
                segment.KeyDown += (_, args) =>
                {
                    var step = args.Key is VirtualKey.Right or VirtualKey.Down ? 1 : args.Key is VirtualKey.Left or VirtualKey.Up ? -1 : 0;
                    if (step == 0 || segments.Count == 0) return;
                    var enabled = segments.Where(item => item.IsEnabled).ToArray();
                    if (enabled.Length == 0) return;
                    var next = enabled[(Array.IndexOf(enabled, segment) + step + enabled.Length) % enabled.Length];
                    next.IsChecked = true;
                    next.Focus(FocusState.Keyboard);
                    args.Handled = true;
                };
                segments.Add(segment);
                row.ColumnDefinitions.Add(new ColumnDefinition());
                Grid.SetColumn(segment, i);
                row.Children.Add(segment);
            }
        }
        HorizontalAlignment = Stretch ? HorizontalAlignment.Stretch : HorizontalAlignment.Left;
        for (var i = 0; i < segments.Count; i++)
        {
            segments[i].IsEnabled = !DisabledItems.Contains(i);
            // RadioButton's native style aligns left; the segment must fill
            // its grid cell when the track distributes space with star columns.
            segments[i].HorizontalAlignment = HorizontalAlignment.Stretch;
            row.ColumnDefinitions[i].Width = Stretch ? new GridLength(1, GridUnitType.Star) : GridLength.Auto;
            var content = segments[i].Content;
            var glyph = GlyphAt(i);
            if (glyph is null)
            {
                if (content is TextBlock words) words.Text = Items[i];
                else segments[i].Content = Items[i];
            }
            else if (IconOnly)
            {
                if (content is Icon icon) icon.Glyph = glyph.Value;
                else segments[i].Content = Face(i);
            }
            else if (content is StackPanel { Children: [Icon icon, TextBlock words] })
            {
                icon.Glyph = glyph.Value;
                words.Text = Items[i];
            }
            else segments[i].Content = Face(i);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(segments[i], IconOnly && glyph is not null ? Items[i] : "");
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
            var over = segment is ChoiceButton { Hovered: true } && segment.IsEnabled;
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
            segment.IsTextScaleFactorEnabled = Theme.Sizing != ThemeSizing.System;
            segment.FontSize = FontSize;
            segment.FontFamily = FontFamily;
            segment.FontWeight = Theme.Weight(chosen ? "labelMedium" : "labelQuiet");
            Skin.TextLine(segment, segment.FontSize);
            if (segment.Content is StackPanel words)
                foreach (var label in words.Children.OfType<TextBlock>())
                {
                    Theme.Text("", $"control.{Theme.Key(Size)}.content").CopyTo(label);
                    label.LineHeight = segment.FontSize;
                    Skin.TextWeight(label, segment);
                }
            segment.Background = chosen ? Filled ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : Theme.Brush("color.surface-raised") : null;
            segment.Foreground = chosen ? Filled ? Theme.Recipe(ControlVariant.Filled, Tint, "content") : Theme.Brush("color.content") : Theme.Brush(over ? "color.content" : "color.content-secondary");
            segment.Opacity = segment.IsEnabled ? 1 : 0.5;
            if (segment is ChoiceButton face) face.DrawShadow(Theme, chosen ? "2xs" : null);
            Skin.Focus(segment, Theme);
        }
    }

    private static void Lay(Icon icon, double size)
    {
        icon.GlyphSize = size;
        Appearance.Refresh(icon);
    }
}
/// <summary>Native radio mechanics with the state changes the shared drawing observes.</summary>
internal sealed class ChoiceButton : RadioButton
{
    internal bool Hovered { get; private set; }
    internal Action? AppearanceChanged { get; init; }
    internal ChoiceButton() => IsEnabledChanged += (_, _) => AppearanceChanged?.Invoke();
    protected override void OnApplyTemplate() { base.OnApplyTemplate(); AppearanceChanged?.Invoke(); }
    protected override void OnPointerEntered(PointerRoutedEventArgs args) { base.OnPointerEntered(args); Hovered = true; AppearanceChanged?.Invoke(); }
    protected override void OnPointerExited(PointerRoutedEventArgs args) { base.OnPointerExited(args); Hovered = false; AppearanceChanged?.Invoke(); }
    internal void DrawShadow(Theme theme, string? token)
    {
        if (GetTemplateChild("Lift") is FrameworkElement host) Elevation.Draw(this, host, theme, token);
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
    /// <summary>Indices unavailable for pointer, keyboard and automation activation.</summary>
    public ISet<int> DisabledItems { get; } = new HashSet<int>();
    /// <summary>The native tabs, in order.</summary>
    public IReadOnlyList<RadioButton> Tabs => row.Children.OfType<RadioButton>().ToArray();
    /// <summary>Optional trailing counts, keyed by item index.</summary>
    public IDictionary<int, int> Counts { get; } = new Dictionary<int, int>();
    public int SelectedIndex { get => selectedIndex; set { selectedIndex = value; Sync(); } }
    public event EventHandler? SelectionChanged;
    // The root lays out one child — the row — so what a reader sees as the
    // row's gap is the list's, exactly as React's flex root reads.
    public PillTabs() { Content = new StackPanel { Children = { row } }; HorizontalAlignment = HorizontalAlignment.Left; }
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
                var pill = new ChoiceButton { Content = Items[i], GroupName = group, Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(PillTemplate), IsChecked = i == selectedIndex, AppearanceChanged = Paint };
                pill.Checked += (_, _) => { if (selectedIndex == index) return; selectedIndex = index; Paint(); SelectionChanged?.Invoke(this, EventArgs.Empty); };
                pill.KeyDown += (_, args) =>
                {
                    var step = args.Key is VirtualKey.Right or VirtualKey.Down ? 1 : args.Key is VirtualKey.Left or VirtualKey.Up ? -1 : 0;
                    if (step == 0 || Items.Count == 0) return;
                    var enabled = row.Children.OfType<RadioButton>().Where(item => item.IsEnabled).ToArray();
                    if (enabled.Length == 0) return;
                    var next = enabled[(Array.IndexOf(enabled, pill) + step + enabled.Length) % enabled.Length];
                    next.IsChecked = true;
                    next.Focus(FocusState.Keyboard);
                    args.Handled = true;
                };
                row.Children.Add(pill);
            }
        }
        for (var i = 0; i < row.Children.Count; i++) ((RadioButton)row.Children[i]).IsEnabled = !DisabledItems.Contains(i);
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
            var over = pill is ChoiceButton { Hovered: true } && pill.IsEnabled;
            pill.Height = height;
            pill.MinWidth = pill.MinHeight = 0;
            pill.Padding = new Thickness(Theme.Number("spacing.3"), 0, Theme.Number("spacing.3"), 0);
            pill.CornerRadius = new CornerRadius(height / 2);
            pill.IsTextScaleFactorEnabled = Theme.Sizing != ThemeSizing.System;
            pill.FontSize = size;
            pill.FontFamily = PlatformFonts.Ui;
            pill.FontWeight = Theme.Weight(chosen ? "labelStrong" : "labelSmall");
            if (Counts.TryGetValue(i, out var count))
            {
                var words = pill.Content as StackPanel;
                if (words is null)
                {
                    words = new StackPanel { Orientation = Orientation.Horizontal };
                    words.Children.Add(new TextBlock { Text = Items[i] });
                    words.Children.Add(new TextBlock());
                    pill.Content = words;
                }
                words.Spacing = Theme.Number("control.tiny.gap");
                foreach (var text in words.Children.OfType<TextBlock>())
                {
                    Theme.Text("", "labelSmall").CopyTo(text);
                    Skin.TextWeight(text, pill);
                    text.LineHeight = size;
                    text.ClearValue(TextBlock.ForegroundProperty);
                }
                var countLabel = (TextBlock)words.Children[1];
                ((TextBlock)words.Children[0]).Text = Items[i];
                countLabel.Text = count.ToString();
                Microsoft.UI.Xaml.Documents.Typography.SetNumeralAlignment(countLabel, FontNumeralAlignment.Tabular);
            }
            else
            {
                if (pill.Content is StackPanel || pill.Content is string text && text != Items[i]) pill.Content = Items[i];
                else if (pill.Content is TextBlock label && label.Text != Items[i]) label.Text = Items[i];
                Skin.TextLine(pill, size);
            }
            pill.Background = chosen ? Theme.Recipe(ControlVariant.Filled, Tint, "surface") : Theme.Brush(over ? "color.surface-sunken" : "color.surface-inset");
            pill.Foreground = chosen ? Theme.Recipe(ControlVariant.Filled, Tint, "content") : Theme.Brush(over ? "color.content" : "color.content-nav");
            pill.Opacity = pill.IsEnabled ? 1 : 0.5;
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
        BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        Skin.SetCorner(this, Theme.Number("radius.large"));
        // Selection adds a painted edge without changing the layout border.
        if (GetTemplateChild("SelectionOutline") is Border outline)
        {
            outline.BorderThickness = new Thickness(Theme.Number("stroke.control"));
            outline.Visibility = chosen ? Visibility.Visible : Visibility.Collapsed;
        }
        Padding = new Thickness(Theme.Number("spacing.3"), Theme.Number("spacing.3.5"), Theme.Number("spacing.3"), Theme.Number("spacing.3.5"));
        var title = Theme.Text(Title, "labelStrong");
        title.Foreground = Theme.Brush("color.content");
        var description = Theme.Text(Description, "labelSmall");
        TextStyle.SetWeight(description, Theme.Weight("captionSmall"));
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
    public string Text { get => Selection.Content is TextBlock text ? text.Text : Selection.Content?.ToString() ?? ""; set => Selection.Content = value; }
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
        Selection.FontWeight = Theme.Weight("labelMedium");
        Selection.Foreground = Foreground;
        Skin.TextLine(Selection, Selection.FontSize * 1.4);
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
