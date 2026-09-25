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
    public bool Mono { get; set; }
    public TextField()
    {
        Appearance.Bind(this, Apply);
        IsEnabledChanged += (_, _) => Apply();
        RegisterPropertyChangedCallback(AcceptsReturnProperty, (_, _) => Apply());
    }
    protected virtual void Apply()
    {
        Skin.Input(this, Theme, Size, Invalid);
        if (Mono) FontFamily = new FontFamily("Cascadia Mono, Consolas");
        if (!AcceptsReturn) Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        else { Height = double.NaN; TextWrapping = TextWrapping.Wrap; }
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
            searchIcon = new Icon { Glyph = IconGlyph.Search, HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Center, IsHitTestVisible = false };
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
        var inset = Theme.Number($"control.{Theme.Key(Size)}.padding.inline");
        Padding = new Thickness(inset + Theme.Number("spacing.6"), Padding.Top, Hint is null ? inset : inset + Theme.Number("spacing.10"), Padding.Bottom);
        if (searchIcon is not null) searchIcon.Margin = new Thickness(inset, 0, 0, 0);
        if (hint is not null) { hint.Content = Hint ?? ""; hint.Margin = new Thickness(0, 0, inset, 0); }
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
    public bool Invalid { get; set; }
    public Select()
    {
        Appearance.Bind(this, () => { Skin.Input(this, Theme, Size, Invalid); Height = Theme.Number($"control.{Theme.Key(Size)}.size"); });
        HorizontalAlignment = HorizontalAlignment.Stretch;
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
                lastGroup = option.Group;
            }
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
        Input.TextChanged += (_, args) =>
        {
            if (args.Reason == AutoSuggestionBoxTextChangeReason.UserInput)
                RefreshSuggestions(Text);
            ValueChanged?.Invoke(this, Text);
        };
        Input.SuggestionChosen += (_, args) => Choose(args.SelectedItem);
        Input.QuerySubmitted += (_, args) => { if (args.ChosenSuggestion is not null) Choose(args.ChosenSuggestion); };
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
        if (Mono) Input.FontFamily = new FontFamily("Cascadia Mono, Consolas");
        Skin.InputPadding(Input, Theme, Size);
        nativeTextBoxStyle ??= Input.TextBoxStyle;
        if (nativeTextBoxStyle is not null)
        {
            // Keep the native query/clear buttons and suggestion behavior.
            var style = new Style(typeof(TextBox)) { BasedOn = nativeTextBoxStyle };
            style.Setters.Add(new Setter(Control.PaddingProperty, Input.Padding));
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
        Input.QueryIcon = new SymbolIcon(Symbol.Find);
        GotFocus += (_, _) => { RefreshSuggestions(""); IsSuggestionListOpen = true; };
    }
}
public sealed class NumberField : Surface
{
    public NumberBox Input { get; } = new();
    public double Value { get => Input.Value; set => Input.Value = value; }
    public double Minimum { get => Input.Minimum; set => Input.Minimum = value; }
    public double Maximum { get => Input.Maximum; set => Input.Maximum = value; }
    public double SmallChange { get => Input.SmallChange; set => Input.SmallChange = value; }
    public double LargeChange { get => Input.LargeChange; set => Input.LargeChange = value; }
    public string PlaceholderText { get => Input.PlaceholderText; set => Input.PlaceholderText = value; }
    public event Windows.Foundation.TypedEventHandler<NumberBox, NumberBoxValueChangedEventArgs> ValueChanged { add => Input.ValueChanged += value; remove => Input.ValueChanged -= value; }
    public bool Invalid { get; set; }
    public string? Unit { get; set; }
    public NumberField()
    {
        Content = Input;
        Input.SpinButtonPlacementMode = NumberBoxSpinButtonPlacementMode.Inline;
        Input.ValidationMode = NumberBoxValidationMode.InvalidInputOverwritten;
        Input.Loaded += (_, _) => SizeEditor(Input);
    }
    private void SizeEditor(DependencyObject root)
    {
        if (root is TextBox editor)
        {
            editor.MinHeight = editor.Height = Theme.Number($"control.{Theme.Key(Size)}.size");
            return;
        }
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++) SizeEditor(VisualTreeHelper.GetChild(root, i));
    }
    protected override void RefreshAppearance() { Skin.Input(Input, Theme, Size, Invalid); Input.Description = Unit; SizeEditor(Input); }
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
        var label = Theme.Text(Label + (Required ? " *" : ""), "labelMedium");
        var body = Skin.Column(Theme, label);
        body.Spacing = Theme.Number("spacing.1.5");
        if (Field is not null)
        {
            // Reusing the input keeps editing state intact when a validation message changes.
            if (Field is FrameworkElement element && element.Parent is Panel previous) previous.Children.Remove(Field);
            body.Children.Add(Field);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetLabeledBy(Field, label);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetIsRequiredForForm(Field, Required);
        }
        if ((Error ?? Hint) is string note)
        {
            var message = Theme.Text(note, "bodySmall");
            if (Error is not null) message.Foreground = Theme.Recipe(ControlVariant.Plain, WidgetTint.Danger, "content");
            body.Children.Add(message);
        }
        Content = body;
    }
    public void SetError(string? error) { Error = error; RefreshAppearance(); }
}
public sealed class Fieldset : Surface
{
    public string Legend { get; set; } = "";
    public IList<UIElement> Fields { get; } = new List<UIElement>();
    protected override void RefreshAppearance() { Content = Skin.Column(Theme, new UIElement[] { Theme.Text(Legend, "titleSmall") }.Concat(Fields).ToArray()); }
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
