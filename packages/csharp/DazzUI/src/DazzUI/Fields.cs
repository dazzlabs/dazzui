using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Windows.System;

namespace DazzUI;

public class TextField : TextBox
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    private bool invalid;
    public bool Invalid { get => invalid; set { invalid = value; Apply(); } }
    private bool mono;
    public bool Mono { get => mono; set { mono = value; Apply(); } }
    private int rows = 2;
    /// <summary>The initial number of prose lines in a multiline field.</summary>
    public int Rows { get => rows; set { rows = Math.Max(1, value); Apply(); } }
    public TextField()
    {
        Appearance.Bind(this, Apply);
        IsEnabledChanged += (_, _) => Apply();
        RegisterPropertyChangedCallback(AcceptsReturnProperty, (_, _) => Apply());
    }
    protected virtual void Apply()
    {
        Skin.Input(this, Theme, Size, Invalid);
        if (Mono) FontFamily = Theme.CodeFont;
        if (!AcceptsReturn) Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        // A multiline field grows from a floor rather than sitting at a height.
        else
        {
            Height = double.NaN;
            MinHeight = Math.Max(Theme.Number("spacing.20"), Rows * FontSize * 1.6 + 2 * (Theme.Number("spacing.2.5") + Theme.Number("stroke.hairline")));
            TextWrapping = TextWrapping.Wrap;
        }
        // Native editors lay text out from the top. Derive the inset from the
        // platform font metrics rather than assuming a CSS line box or a fixed
        // Windows font height; the same padding also positions the placeholder.
        Skin.InputPadding(this, Theme, Size, AcceptsReturn);
        if (AcceptsReturn) Skin.SetCorner(this, Theme.Number("radius.large"));
    }
}
public sealed class FieldValue : TextField
{
    public FieldValue() { IsReadOnly = true; IsTabStop = false; }
}
public sealed class SearchField : TextField
{
    private Icon? searchIcon;
    private KeyCap? hint;
    public string? Hint { get; set; }
    public event EventHandler? Dismissed;
    public SearchField()
    {
        PlaceholderText = "Search";
        TextChanged += (_, _) => { if (hint is not null) hint.Visibility = Text.Length == 0 ? Visibility.Visible : Visibility.Collapsed; };
        KeyDown += (_, args) => { if (args.Key == VirtualKey.Escape) { Text = ""; Dismissed?.Invoke(this, EventArgs.Empty); args.Handled = true; } };
    }
    protected override void OnApplyTemplate()
    {
        base.OnApplyTemplate();
        if (GetTemplateChild("ContentElement") is FrameworkElement editor && editor.Parent is Grid grid)
        {
            searchIcon = new Icon { Glyph = FluentIcons.Common.Icon.Search, HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Center, IsHitTestVisible = false };
            Grid.SetRow(searchIcon, Grid.GetRow(editor)); grid.Children.Add(searchIcon);
            hint = new KeyCap(Hint ?? "") { HorizontalAlignment = HorizontalAlignment.Right, VerticalAlignment = VerticalAlignment.Center, IsHitTestVisible = false, Visibility = Text.Length == 0 ? Visibility.Visible : Visibility.Collapsed };
            Grid.SetRow(hint, Grid.GetRow(editor)); grid.Children.Add(hint);
        }
        ApplySearchInsets();
    }
    protected override void Apply()
    {
        base.Apply();
        // A search field is a control, not a field box: it keeps the regular
        // control corner, which is a capsule where buttons are (macOS 27).
        Skin.SetCorner(this, Theme.Number("radius.medium"));
        ApplySearchInsets();
    }
    private void ApplySearchInsets()
    {
        var inset = Theme.Number("spacing.3");
        var glyph = Theme.Number($"control.{Theme.Key(Size)}.icon-size");
        var gap = Theme.Number($"control.{Theme.Key(Size)}.gap");
        Padding = new Thickness(inset + glyph + gap, Padding.Top, Hint is null ? inset : inset + Theme.Number("spacing.10"), Padding.Bottom);
        if (searchIcon is not null)
        {
            searchIcon.Margin = new Thickness(inset + BorderThickness.Left, 0, 0, 0);
            searchIcon.GlyphSize = glyph;
            searchIcon.Foreground = Theme.Brush("color.content-faint");
            Appearance.Refresh(searchIcon);
        }
        if (hint is not null) { hint.Content = Hint ?? ""; hint.Margin = new Thickness(0, 0, inset + BorderThickness.Right, 0); }
    }
}
public record SelectOption(string Value, string Label, string? Group = null, bool Disabled = false)
{
    public override string ToString() => Label;
}
public class Select : ComboBox
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public ControlSize Size { get => Appearance.GetSize(this); set => Appearance.SetSize(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    private bool invalid;
    public bool Invalid { get => invalid; set { invalid = value; Apply(); } }
    private bool mono;
    public bool Mono { get => mono; set { mono = value; Apply(); } }
    public Select()
    {
        Appearance.Bind(this, Apply);
        IsEnabledChanged += (_, _) => Apply();
        HorizontalAlignment = HorizontalAlignment.Stretch;
    }
    private void Apply()
    {
        Skin.Input(this, Theme, Size, Invalid);
        if (Mono) FontFamily = Theme.CodeFont;
        Height = Theme.Number($"control.{Theme.Key(Size)}.size");
    }
    public void SetOptions(IEnumerable<SelectOption> options)
    {
        Items.Clear();
        string? lastGroup = null;
        foreach (var option in options)
        {
            if (option.Group is not null && option.Group != lastGroup)
            {
                Items.Add(new ComboBoxItem { Content = option.Group, IsEnabled = false });
            }
            lastGroup = option.Group;
            Items.Add(new ComboBoxItem { Content = option.Label, Tag = option.Value, IsEnabled = !option.Disabled });
        }
    }
    public string? Value
    {
        get => (SelectedItem as ComboBoxItem)?.Tag as string ?? SelectedItem?.ToString();
        set => SelectedItem = Items.OfType<ComboBoxItem>().FirstOrDefault(item => Equals(item.Tag, value));
    }
}
public class Autocomplete : Surface
{
    private Style? nativeTextBoxStyle;
    private bool painted;
    public AutoSuggestBox Input { get; } = new();
    public string Text { get => Input.Text; set => Input.Text = value; }
    public string PlaceholderText { get => Input.PlaceholderText; set => Input.PlaceholderText = value; }
    public bool IsSuggestionListOpen { get => Input.IsSuggestionListOpen; set => Input.IsSuggestionListOpen = value; }
    public event EventHandler<string>? ValueChanged;
    public IList<string> Suggestions { get; } = new List<string>();
    public IList<SelectOption> Options { get; } = new List<SelectOption>();
    public string? SelectedValue { get; private set; }
    public bool Invalid { get; set; }
    public bool Mono { get; set; }
    public Autocomplete()
    {
        Content = Input;
        Input.Loaded += (_, _) => RefreshAppearance();
        Input.GotFocus += (_, _) => PaintPlaceholder();
        Input.LostFocus += (_, _) => PaintPlaceholder();
        Input.LayoutUpdated += (_, _) => { if (!painted && IsLoaded) { painted = true; PaintPlaceholder(); } };
        Input.TextChanged += (_, args) =>
        {
            if (args.Reason == AutoSuggestionBoxTextChangeReason.UserInput)
                RefreshSuggestions(Text);
            ValueChanged?.Invoke(this, Text);
        };
        Input.SuggestionChosen += (_, args) => Choose(args.SelectedItem);
        Input.QuerySubmitted += (_, args) => { if (args.ChosenSuggestion is not null) Choose(args.ChosenSuggestion); };
    }
    /// <summary>
    /// The query box is the native control's own part, templated before a theme
    /// is known: its placeholder presenter takes the faint ink directly.
    /// </summary>
    private void PaintPlaceholder()
    {
        var faint = Theme.Brush("color.content-faint");
        foreach (var box in Parts<TextBox>(Input)) box.PlaceholderForeground = faint;
        foreach (var words in Parts<TextBlock>(Input).Where(part => part.Name == "PlaceholderTextContentPresenter")) words.Foreground = faint;
    }
    private static IEnumerable<T> Parts<T>(DependencyObject root) where T : DependencyObject
    {
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
        {
            var child = VisualTreeHelper.GetChild(root, i);
            if (child is T part) yield return part;
            foreach (var deeper in Parts<T>(child)) yield return deeper;
        }
    }
    private void Choose(object item)
    {
        if (item is ListViewItem { Tag: SelectOption option }) { SelectedValue = option.Value; Text = option.Label; }
        else { SelectedValue = item.ToString(); Text = SelectedValue ?? ""; }
    }
    protected void RefreshSuggestions(string query)
    {
        if (Options.Count == 0) { Input.ItemsSource = Suggestions.Where(item => item.Contains(query, StringComparison.CurrentCultureIgnoreCase)).ToArray(); return; }
        var items = new List<ListViewItem>();
        string? group = null;
        foreach (var option in Options.Where(option => option.Label.Contains(query, StringComparison.CurrentCultureIgnoreCase)))
        {
            if (option.Group is not null && option.Group != group)
            {
                group = option.Group;
                items.Add(new ListViewItem { Content = group, IsEnabled = false, IsTabStop = false });
            }
            items.Add(new ListViewItem { Content = option.Label, Tag = option, IsEnabled = !option.Disabled });
        }
        Input.ItemsSource = items;
    }
    protected override void RefreshAppearance()
    {
        Skin.Input(Input, Theme, Size, Invalid);
        if (Mono) Input.FontFamily = Theme.CodeFont;
        PaintPlaceholder();
        Skin.InputPadding(Input, Theme, Size);
        nativeTextBoxStyle ??= Input.TextBoxStyle;
        if (nativeTextBoxStyle is not null)
        {
            // Keep the native query/clear buttons and suggestion behavior.
            var style = new Style(typeof(TextBox)) { BasedOn = nativeTextBoxStyle };
            style.Setters.Add(new Setter(Control.PaddingProperty, Input.Padding));
            style.Setters.Add(new Setter(TextBox.PlaceholderForegroundProperty, Theme.Brush("color.content-faint")));
            style.Setters.Add(new Setter(FrameworkElement.MinHeightProperty, Input.MinHeight));
            style.Setters.Add(new Setter(FrameworkElement.HeightProperty, Input.MinHeight));
            Input.TextBoxStyle = style;
        }
    }
}
public sealed class Combobox : Autocomplete
{
    public Combobox()
    {
        // The list drops from a chevron at the trailing edge, as a Select's does.
        var chevron = Icon.Element(FluentIcons.Common.Icon.ChevronDown);
        chevron.FontSize = 12;
        Input.QueryIcon = chevron;
        GotFocus += (_, _) => { RefreshSuggestions(""); IsSuggestionListOpen = true; };
    }
}
/// <summary>
/// A number in the text controls' box, with its unit and a stepper inside it
/// at the trailing edge: two half-height steps in one column, hairlined off
/// the value. Up and Down step it too; the value is clamped to its range.
/// </summary>
public sealed class NumberField : Surface
{
    private readonly Border box = new();
    private readonly Grid row = new();
    private readonly TextBox editor = new() { BorderThickness = new Thickness(0), VerticalAlignment = VerticalAlignment.Center };
    private readonly TextBlock unit = new() { VerticalAlignment = VerticalAlignment.Center };
    private readonly Border stepper = new();
    private readonly Microsoft.UI.Xaml.Controls.Button up = Step(FluentIcons.Common.Icon.ChevronUp);
    private readonly Microsoft.UI.Xaml.Controls.Button down = Step(FluentIcons.Common.Icon.ChevronDown);
    private double value = double.NaN;
    private bool focused;
    /// <summary>The native editor that holds the typed value.</summary>
    public TextBox Input => editor;
    public double Value { get => value; set { this.value = double.IsNaN(value) ? value : Math.Clamp(value, Minimum, Maximum); Show(); } }
    public double Minimum { get; set; } = double.MinValue;
    public double Maximum { get; set; } = double.MaxValue;
    public double SmallChange { get; set; } = 1;
    public double LargeChange { get; set; } = 10;
    public string PlaceholderText { get => editor.PlaceholderText; set => editor.PlaceholderText = value; }
    public event EventHandler<double>? ValueChanged;
    private bool invalid;
    public bool Invalid { get => invalid; set { invalid = value; if (IsLoaded) RefreshAppearance(); } }
    private string? unitText;
    public string? Unit { get => unitText; set { unitText = value; if (IsLoaded) RefreshAppearance(); } }
    public NumberField()
    {
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        Grid.SetColumn(unit, 1);
        Grid.SetColumn(stepper, 2);
        stepper.Child = new StackPanel { VerticalAlignment = VerticalAlignment.Center, Children = { up, down } };
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(up, "Increase");
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(down, "Decrease");
        row.Children.Add(editor);
        row.Children.Add(unit);
        row.Children.Add(stepper);
        box.Child = row;
        Content = box;
        up.Click += (_, _) => Commit(Read() + SmallChange);
        down.Click += (_, _) => Commit(Read() - SmallChange);
        IsEnabledChanged += (_, _) => { if (IsLoaded) RefreshAppearance(); };
        editor.GotFocus += (_, _) => { focused = true; RefreshAppearance(); };
        editor.LostFocus += (_, _) => { focused = false; Commit(Read()); RefreshAppearance(); };
        editor.KeyDown += (_, args) =>
        {
            var step = args.Key switch
            {
                Windows.System.VirtualKey.Up => SmallChange,
                Windows.System.VirtualKey.Down => -SmallChange,
                Windows.System.VirtualKey.PageUp => LargeChange,
                Windows.System.VirtualKey.PageDown => -LargeChange,
                _ => 0,
            };
            if (args.Key == Windows.System.VirtualKey.Enter) { Commit(Read()); args.Handled = true; }
            else if (step != 0) { Commit(Read() + step); args.Handled = true; }
        };
    }
    private static Microsoft.UI.Xaml.Controls.Button Step(FluentIcons.Common.Icon glyph) => new()
    {
        Content = new Icon { Glyph = glyph },
        Padding = new Thickness(0),
        MinWidth = 0,
        MinHeight = 0,
        BorderThickness = new Thickness(0),
        IsTabStop = false,
    };
    private double Read() =>
        double.TryParse(editor.Text, System.Globalization.NumberStyles.Float, System.Globalization.CultureInfo.CurrentCulture, out var typed)
            ? typed
            : double.IsNaN(value) ? Math.Max(Minimum, Math.Min(0, Maximum)) : value;
    private void Commit(double next)
    {
        var previous = value;
        Value = next;
        if (!Equals(previous, value)) ValueChanged?.Invoke(this, value);
    }
    private void Show()
    {
        editor.Text = double.IsNaN(value) ? "" : value.ToString(System.Globalization.CultureInfo.CurrentCulture);
        up.IsEnabled = IsEnabled && (double.IsNaN(value) || value < Maximum);
        down.IsEnabled = IsEnabled && (double.IsNaN(value) || value > Minimum);
        foreach (var step in new[] { up, down })
            if (step.Content is Icon glyph)
                glyph.Foreground = step.IsEnabled ? Theme.Brush("color.content-subtle") : Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled");
    }
    protected override void RefreshAppearance()
    {
        var height = Theme.Number($"control.{Theme.Key(Size)}.size");
        Skin.Input(editor, Theme, Size, Invalid);
        // The box draws; the editor disappears into it.
        box.Background = focused && !Invalid ? Theme.Brush("color.surface") : editor.Background;
        box.BorderBrush = focused || Invalid ? Theme.Brush($"color.{(Invalid ? "danger" : Theme.Key(Tint))}.600") : editor.BorderBrush;
        box.BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        box.CornerRadius = new CornerRadius(Math.Min(Theme.Number("control.field.radius"), height / 2));
        box.Height = height;
        var block = Theme.Number($"control.{Theme.Key(Size)}.padding.block");
        box.Padding = new Thickness(Theme.Number("spacing.3"), block, Theme.Number("spacing.1"), block);
        row.ColumnSpacing = Theme.Number("spacing.1");
        var columns = Unit is null ? 2 : 3;
        while (row.ColumnDefinitions.Count > columns) row.ColumnDefinitions.RemoveAt(row.ColumnDefinitions.Count - 1);
        while (row.ColumnDefinitions.Count < columns) row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        Grid.SetColumn(stepper, columns - 1);
        var clear = new SolidColorBrush(Microsoft.UI.Colors.Transparent);
        editor.Background = clear;
        editor.BorderThickness = new Thickness(0);
        foreach (var key in new[] { "TextControlBackground", "TextControlBackgroundPointerOver", "TextControlBackgroundFocused", "TextControlBorderBrush", "TextControlBorderBrushPointerOver", "TextControlBorderBrushFocused" })
            editor.Resources[key] = clear;
        editor.Height = height - 2 * Theme.Number("stroke.hairline");
        // The group owns the shared field insets. The native editor keeps its
        // full font bounds, centered through that line box, without charging
        // the horizontal padding a second time.
        editor.Margin = new Thickness(0, -block, 0, -block);
        editor.Padding = new Thickness(0, editor.Padding.Top, 0, editor.Padding.Bottom);
        Skin.InputPadding(editor, Theme, Size);
        unit.Text = Unit ?? "";
        unit.Visibility = Unit is null ? Visibility.Collapsed : Visibility.Visible;
        unit.Foreground = Theme.Brush("color.content-faint");
        unit.FontSize = Theme.Text("", "labelSmall").FontSize;
        unit.FontFamily = editor.FontFamily;
        unit.LineHeight = unit.FontSize;
        unit.LineStackingStrategy = LineStackingStrategy.BlockLineHeight;
        stepper.BorderBrush = Theme.Brush("color.border");
        stepper.BorderThickness = new Thickness(Theme.Number("stroke.hairline"), 0, 0, 0);
        stepper.Margin = new Thickness(0, Theme.Number("spacing.1"), 0, Theme.Number("spacing.1"));
        stepper.Padding = new Thickness(Theme.Number("spacing.1"), 0, 0, 0);
        foreach (var step in new[] { up, down })
        {
            step.Width = Theme.Number("spacing.4");
            step.Height = Theme.Number("spacing.3");
            step.CornerRadius = new CornerRadius(Theme.Number("radius.tiny"));
            step.Background = clear;
            step.Foreground = Theme.Brush("color.content-subtle");
            step.Resources["ButtonBackgroundPointerOver"] = Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered");
            step.Resources["ButtonBackgroundPressed"] = Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "pressed");
            step.Resources["ButtonBorderBrushPointerOver"] = clear;
            step.Resources["ButtonBorderBrushPressed"] = clear;
            if (step.Content is Icon glyph)
            {
                glyph.GlyphSize = Math.Min(Theme.Number("icon.small"), Theme.Number("spacing.3"));
                glyph.Foreground = step.IsEnabled ? step.Foreground : Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled");
            }
        }
        Show();
    }
}
public sealed class FormField : Surface
{
    public string Label { get; set; } = "";
    public string? Hint { get; set; }
    public string? Error { get; set; }
    public bool Required { get; set; }
    public UIElement? Field { get; set; }
    protected override void RefreshAppearance()
    {
        var subtle = Theme.Brush("color.content-subtle");
        var danger = Theme.Brush("color.danger.700");
        var label = Theme.Text(Label + (Required ? " *" : ""), "labelSmall");
        label.Foreground = Error is null ? subtle : danger;
        var body = new StackPanel { Spacing = Theme.Number("spacing.1.5") };
        body.Children.Add(label);
        if (Field is not null)
        {
            // Reusing the input keeps editing state intact when a validation message changes.
            Skin.Detach(Field);
            body.Children.Add(Field);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetLabeledBy(Field, label);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetIsRequiredForForm(Field, Required);
        }
        if ((Error ?? Hint) is string note)
        {
            var message = Theme.Text(note, "labelQuiet");
            message.LineHeight = message.FontSize * 1.6;
            message.Foreground = Error is null ? subtle : danger;
            body.Children.Add(message);
        }
        Content = body;
    }
    public void SetError(string? error) { Error = error; RefreshAppearance(); }
}
/// <summary>Fields that belong together under a legend in the caption face; no border, only the air around it.</summary>
public sealed class Fieldset : Surface
{
    public string Legend { get; set; } = "";
    public IList<UIElement> Fields { get; } = new List<UIElement>();
    protected override void RefreshAppearance()
    {
        var legend = Theme.Text(Legend, "captionSmall");
        legend.FontWeight = Theme.Text("", "labelSmall").FontWeight;
        legend.Foreground = Theme.Brush("color.content-subtle");
        var body = new StackPanel { Spacing = Theme.Number("spacing.3") };
        body.Children.Add(legend);
        foreach (var field in Fields) { Skin.Detach(field); body.Children.Add(field); }
        Content = body;
    }
}
public sealed class Form : Surface
{
    public IList<FormField> Fields { get; } = new List<FormField>();
    public Func<IReadOnlyDictionary<string, string>>? Validate { get; set; }
    public event EventHandler? Submitted;
    public UIElement? Footer { get; set; }
    public bool Submit()
    {
        var errors = Validate?.Invoke() ?? new Dictionary<string, string>();
        foreach (var field in Fields) field.SetError(errors.GetValueOrDefault(field.Label));
        if (errors.Count > 0) { (Fields.FirstOrDefault(f => f.Error is not null)?.Field as Control)?.Focus(FocusState.Programmatic); return false; }
        Submitted?.Invoke(this, EventArgs.Empty); return true;
    }
    protected override void RefreshAppearance()
    {
        var body = Skin.Column(Theme, Fields.Cast<UIElement>().ToArray());
        if (Footer is not null) { Skin.Detach(Footer); body.Children.Add(Footer); }
        Content = body;
    }
}
