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
        Loaded += (_, _) => HideClearSoon();
        GotFocus += (_, _) => HideClearSoon();
        TextChanged += (_, _) => HideClearSoon();
        // Native templates can resolve resources before OnApplyTemplate.
        // Install stable brush objects before the first template is expanded.
        Skin.Input(this, Theme, Size);
    }
    protected override void OnApplyTemplate()
    {
        base.OnApplyTemplate();
        Apply();
        HideClearSoon();
    }
    private void HideClearSoon()
    {
        HideClear();
        DispatcherQueue.TryEnqueue(HideClear);
    }
    /// <summary>Whether the native clear button belongs: a search field keeps it, a plain field does not.</summary>
    protected virtual bool NativeClear => false;
    /// <summary>A web text field has no native clear button; the search field owns its own dismiss.</summary>
    private void HideClear()
    {
        if (NativeClear) return;
        if (GetTemplateChild("DeleteButton") is Microsoft.UI.Xaml.Controls.Button clear)
        {
            // The template's ButtonVisible state sets this button visible
            // again; opacity and width are not part of that state, so the
            // field keeps the web's plain value line.
            clear.Visibility = Visibility.Collapsed;
            clear.Opacity = 0;
            clear.Width = 0;
            clear.MinWidth = 0;
            clear.IsHitTestVisible = false;
        }
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
/// <summary>A masked native password editor using the same field tokens as TextField.</summary>
public sealed class SecureTextField : Surface
{
    public PasswordBox Input { get; } = new() { PasswordRevealMode = PasswordRevealMode.Hidden };
    private bool invalid;
    public bool Invalid { get => invalid; set { invalid = value; RefreshAppearance(); } }
    public string Password { get => Input.Password; set => Input.Password = value; }
    public string PlaceholderText { get => Input.PlaceholderText; set => Input.PlaceholderText = value; }
    public event RoutedEventHandler PasswordChanged { add => Input.PasswordChanged += value; remove => Input.PasswordChanged -= value; }

    public SecureTextField()
    {
        Content = Input;
        IsEnabledChanged += (_, _) => RefreshAppearance();
        RefreshAppearance();
    }

    protected override void OnApplyTemplate()
    {
        RefreshAppearance();
        base.OnApplyTemplate();
    }

    protected override void RefreshAppearance()
    {
        Input.IsEnabled = IsEnabled;
        Appearance.SetSize(Input, Size);
        Skin.Input(Input, Theme, Size, Invalid);
        Input.Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        Skin.InputPadding(Input, Theme, Size);
    }
}
public sealed class SearchField : TextField
{
    private Icon? searchIcon;
    private KeyCap? hint;
    private Microsoft.UI.Xaml.Controls.Button? clear;
    private Icon? clearIcon;
    private bool clearHovered;
    private string clearLabel = "Clear search";
    private string? hintText;
    public string? Hint { get => hintText; set { hintText = value; ApplySearchInsets(); } }
    /// <summary>The accessible name of the trailing clear action.</summary>
    public string ClearLabel { get => clearLabel; set { clearLabel = value; ApplySearchInsets(); } }
    public event EventHandler? Dismissed;
    public SearchField()
    {
        PlaceholderText = "Search";
        TextChanged += (_, _) => ApplySearchInsets();
        RegisterPropertyChangedCallback(IsReadOnlyProperty, (_, _) => ApplySearchInsets());
        PreviewKeyDown += (_, args) =>
        {
            if (args.Key != VirtualKey.Escape) return;
            // Handle before the native editor consumes Escape. A first press
            // clears the query; only an already empty search dismisses.
            if (Text.Length > 0) Text = "";
            else Dismissed?.Invoke(this, EventArgs.Empty);
            args.Handled = true;
        };
    }
    protected override void OnApplyTemplate()
    {
        base.OnApplyTemplate();
        if (GetTemplateChild("ContentElement") is FrameworkElement editor && editor.Parent is Grid grid)
        {
            searchIcon = new Icon { Glyph = FluentIcons.Common.Icon.Search, HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Center, IsHitTestVisible = false };
            Grid.SetRow(searchIcon, Grid.GetRow(editor)); grid.Children.Add(searchIcon);
            hint = new KeyCap(Hint ?? "") { HorizontalAlignment = HorizontalAlignment.Right, VerticalAlignment = VerticalAlignment.Center, IsHitTestVisible = false, Visibility = Visibility.Collapsed };
            hint.Loaded += (_, _) => ApplySearchInsets();
            Grid.SetRow(hint, Grid.GetRow(editor)); grid.Children.Add(hint);
            // The native text-box delete affordance is focus-dependent and
            // carries Windows geometry. This remains a native button, drawn
            // as React's always-present trailing glyph while a query exists.
            clearHovered = false;
            clearIcon = new Icon { Glyph = FluentIcons.Common.Icon.Dismiss, IsHitTestVisible = false };
            clear = new Microsoft.UI.Xaml.Controls.Button
            {
                Content = clearIcon,
                MinWidth = 0,
                MinHeight = 0,
                Padding = new Thickness(0),
                BorderThickness = new Thickness(0),
                Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent),
                HorizontalAlignment = HorizontalAlignment.Right,
                VerticalAlignment = VerticalAlignment.Center,
                Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load("""
                    <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" TargetType="Button">
                        <Grid Background="{TemplateBinding Background}" Control.IsTemplateFocusTarget="True">
                            <ContentPresenter Content="{TemplateBinding Content}" Foreground="{TemplateBinding Foreground}" HorizontalAlignment="Center" VerticalAlignment="Center" />
                        </Grid>
                    </ControlTemplate>
                    """),
            };
            clear.PointerEntered += (_, _) => { clearHovered = true; ApplyClearInk(); };
            clear.PointerExited += (_, _) => { clearHovered = false; ApplyClearInk(); };
            clear.Click += (_, _) =>
            {
                if (!IsEnabled || IsReadOnly) return;
                Text = "";
                Focus(FocusState.Programmatic);
            };
            Grid.SetRow(clear, Grid.GetRow(editor)); grid.Children.Add(clear);
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
        var trailing = inset;
        if (clear is not null && clearIcon is not null)
        {
            var clearSize = Theme.Number("icon.small");
            var showClear = Text.Length > 0;
            clear.Visibility = showClear ? Visibility.Visible : Visibility.Collapsed;
            clear.IsEnabled = IsEnabled && !IsReadOnly;
            clear.Width = clear.Height = clearSize;
            clear.Margin = new Thickness(0, 0, inset + BorderThickness.Right, 0);
            clearIcon.GlyphSize = clearSize;
            if (!showClear || !clear.IsEnabled) clearHovered = false;
            ApplyClearInk();
            Appearance.Refresh(clearIcon);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(clear, ClearLabel);
            Skin.Focus(clear, Theme);
            if (showClear) trailing += clearSize + gap;
        }
        if (hint is not null)
        {
            hint.Content = Hint ?? "";
            hint.Margin = new Thickness(0, 0, inset + BorderThickness.Right, 0);
            var showHint = Text.Length == 0 && !string.IsNullOrEmpty(Hint);
            hint.Visibility = showHint ? Visibility.Visible : Visibility.Collapsed;
            if (showHint)
            {
                Appearance.Refresh(hint);
                hint.Measure(new Windows.Foundation.Size(double.PositiveInfinity, double.PositiveInfinity));
                trailing += hint.DesiredSize.Width - hint.Margin.Right + gap;
            }
        }
        Padding = new Thickness(inset + glyph + gap, Padding.Top, trailing, Padding.Bottom);
        if (searchIcon is not null)
        {
            searchIcon.Margin = new Thickness(inset + BorderThickness.Left, 0, 0, 0);
            searchIcon.GlyphSize = glyph;
            searchIcon.Foreground = Theme.Brush("color.content-faint");
            Appearance.Refresh(searchIcon);
        }
    }
    private void ApplyClearInk()
    {
        if (clear is null || clearIcon is null) return;
        clear.Foreground = clearIcon.Foreground = Theme.Brush(clearHovered && clear.IsEnabled ? "color.content" : "color.content-faint");
    }
}
public record SelectOption(string Value, string Label, string? Group = null, bool Disabled = false)
{
    public override string ToString() => Label;
}
public class Select : ComboBox
{
    private readonly Dictionary<ComboBoxItem, bool> groupHeaders = new();
    private bool paintingPopup;
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
        // The native popup still clamps itself to the available window. The
        // shared sheet adds no independent 504px ceiling to that space.
        MaxDropDownHeight = double.PositiveInfinity;
        DropDownOpened += (_, _) =>
        {
            ApplyPopup();
            DispatcherQueue.TryEnqueue(ApplyPopup);
        };
        SizeChanged += (_, _) => ApplyPopup();
        Apply();
    }
    protected override void OnApplyTemplate()
    {
        base.OnApplyTemplate();
        Apply();
    }
    protected override void PrepareContainerForItemOverride(DependencyObject element, object item)
    {
        base.PrepareContainerForItemOverride(element, item);
        if (element is ComboBoxItem row) PaintRow(row);
    }
    private void Apply()
    {
        Skin.Input(this, Theme, Size, Invalid);
        if (Mono) FontFamily = Theme.CodeFont;
        Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        // Keep the native popup, focus and selection states; replace only
        // its fixed 38px caption strip with the shared glyph and gap.
        if (GetTemplateChild("DropDownGlyph") is FrameworkElement glyph)
        {
            var icon = Theme.Number("icon.small");
            glyph.Width = glyph.Height = icon;
            glyph.MinHeight = 0;
            glyph.Margin = new Thickness(0, 0, Padding.Right, 0);
            if (VisualTreeHelper.GetParent(glyph) is Grid grid && grid.ColumnDefinitions.Count > 1)
                grid.ColumnDefinitions[1].Width = new GridLength(icon + Theme.Number("spacing.2"));
        }
        ApplyPopup();
    }
    private void PaintRow(ComboBoxItem row)
    {
        var heading = groupHeaders.TryGetValue(row, out var groupGap);
        SelectionPopupSkin.Row(row, Theme, heading: heading, groupGap: groupGap);
        row.PreviewKeyDown -= NavigateDisabledOption;
        row.PreviewKeyDown += NavigateDisabledOption;
    }
    private void NavigateDisabledOption(object sender, KeyRoutedEventArgs args)
    {
        if (args.Handled || !IsDropDownOpen || sender is not ComboBoxItem current
            || args.Key is not (VirtualKey.Up or VirtualKey.Down or VirtualKey.Home or VirtualKey.End)
            || Microsoft.UI.Input.InputKeyboardSource.GetKeyStateForCurrentThread(VirtualKey.Menu)
                .HasFlag(Windows.UI.Core.CoreVirtualKeyStates.Down)) return;
        var index = IndexFromContainer(current);
        if (index < 0) index = Items.IndexOf(current);
        if (index < 0) return;
        var step = args.Key is VirtualKey.Up or VirtualKey.End ? -1 : 1;
        var next = args.Key == VirtualKey.Home ? 0 : args.Key == VirtualKey.End ? Items.Count - 1 : index + step;
        while (next >= 0 && next < Items.Count)
        {
            var candidate = Items[next] as ComboBoxItem ?? ContainerFromIndex(next) as ComboBoxItem;
            if (candidate is null) return;
            if (groupHeaders.ContainsKey(candidate) || candidate.Visibility == Visibility.Collapsed)
            {
                next += step;
                continue;
            }
            // Native ComboBox navigation skips disabled containers even when
            // AllowFocusWhenDisabled is set. Bridge only that missing step;
            // ordinary movement, scrolling and selection remain native.
            if (!candidate.IsEnabled && candidate.AllowFocusWhenDisabled && candidate.Focus(FocusState.Keyboard))
            {
                candidate.StartBringIntoView(new BringIntoViewOptions { AnimationDesired = false });
                args.Handled = true;
            }
            return;
        }
    }
    private void ApplyPopup()
    {
        if (paintingPopup) return;
        paintingPopup = true;
        try
        {
            foreach (var row in Items.OfType<ComboBoxItem>()) PaintRow(row);
            for (var index = 0; index < Items.Count; index++)
                if (Items[index] is not ComboBoxItem && ContainerFromIndex(index) is ComboBoxItem generated) PaintRow(generated);
            if (GetTemplateChild("Popup") is not Microsoft.UI.Xaml.Controls.Primitives.Popup popup
                || GetTemplateChild("PopupBorder") is not Border border) return;
            var width = ActualWidth > 0 ? ActualWidth : double.IsFinite(Width) ? Width : 0;
            SelectionPopupSkin.Sheet(popup, border, Theme, width);
            if (GetTemplateChild("ScrollViewer") is ScrollViewer scroll)
            {
                scroll.MinWidth = Math.Max(0, width - 2 * (Theme.Number("spacing.1") + Theme.Number("stroke.hairline")));
                scroll.Margin = scroll.Padding = new Thickness(0);
                scroll.Foreground = Theme.Brush("color.content");
                foreach (var presenter in OverlaySkin.Walk(scroll).OfType<ItemsPresenter>())
                    presenter.Margin = presenter.Padding = new Thickness(0);
            }
        }
        finally { paintingPopup = false; }
    }
    public void SetOptions(IEnumerable<SelectOption> options)
    {
        Items.Clear();
        groupHeaders.Clear();
        string? lastGroup = null;
        foreach (var option in options)
        {
            if (option.Group is not null && option.Group != lastGroup)
            {
                var heading = new ComboBoxItem { Content = option.Group, IsEnabled = false, IsTabStop = false, IsHitTestVisible = false };
                groupHeaders[heading] = lastGroup is not null;
                Items.Add(heading);
            }
            lastGroup = option.Group;
            Items.Add(new ComboBoxItem { Content = option.Label, Tag = option.Value, IsEnabled = !option.Disabled });
        }
        ApplyPopup();
    }
    public string? Value
    {
        get => (SelectedItem as ComboBoxItem)?.Tag as string ?? SelectedItem?.ToString();
        set => SelectedItem = value is null ? null : Items.OfType<ComboBoxItem>().FirstOrDefault(item => !groupHeaders.ContainsKey(item) && Equals(item.Tag, value));
    }
}
public class Autocomplete : Surface
{
    public AutoSuggestBox Input { get; } = new() { UpdateTextOnSelect = false };
    public string Text { get => Input.Text; set => Input.Text = value; }
    public string PlaceholderText { get => Input.PlaceholderText; set => Input.PlaceholderText = value; }
    public bool IsSuggestionListOpen { get => Input.IsSuggestionListOpen; set => Input.IsSuggestionListOpen = value; }
    public event EventHandler<string>? ValueChanged;
    public IList<string> Suggestions { get; } = new List<string>();
    public IList<SelectOption> Options { get; } = new List<SelectOption>();
    private string? selectedValue;
    public string? SelectedValue { get => selectedValue; set => ShowSelection(value); }
    private bool invalid;
    public bool Invalid { get => invalid; set { invalid = value; RefreshAppearance(); } }
    private bool mono;
    public bool Mono { get => mono; set { mono = value; RefreshAppearance(); } }
    private bool readOnly;
    public bool ReadOnly { get => readOnly; set { readOnly = value; RefreshAppearance(); } }
    public string Empty { get; set; } = "No matches";
    protected virtual bool IsChoice => false;
    private bool mounted;
    private int mountVersion;
    internal bool IsMounted => mounted;
    public Autocomplete()
    {
        // The shared popup has a gap and its own round corners; WinUI's
        // attached-panel treatment must not square the adjoining corners.
        Microsoft.UI.Xaml.Controls.Primitives.AutoSuggestBoxHelper.SetKeepInteriorCornersSquare(Input, false);
        Content = Input;
        Loaded += (_, _) =>
        {
            if (mounted) return;
            mounted = true;
            mountVersion++;
            Input.TextChanged += OnTextChanged;
            RefreshAppearance();
            RefreshSuggestions("");
            Input.IsSuggestionListOpen = false;
        };
        Unloaded += (_, _) =>
        {
            if (!mounted) return;
            mounted = false;
            mountVersion++;
            Input.TextChanged -= OnTextChanged;
            QueryPopupSkin.Detach(this);
        };
        Input.Loaded += (_, _) => RefreshInput();
        Input.GotFocus += (_, _) => RefreshInput();
        Input.LostFocus += (_, _) =>
        {
            if (!mounted) return;
            var version = mountVersion;
            DispatcherQueue.TryEnqueue(() =>
            {
                if (mounted && version == mountVersion && IsChoice && !QueryPopupSkin.ContainsFocus(this)) RestoreSelection();
            });
        };
        IsEnabledChanged += (_, _) => RefreshAppearance();
        // Native suggestion highlighting must not commit the selection or
        // rewrite the query. A click or Enter submits the chosen row once.
        Input.QuerySubmitted += (_, args) =>
        {
            if (!mounted || !IsEnabled || ReadOnly || args.ChosenSuggestion is not ListViewItem { Tag: SelectOption option } || option.Disabled) return;
            Choose(option);
            Input.IsSuggestionListOpen = false;
        };
        Input.PreviewKeyDown += (_, args) =>
        {
            if (!mounted || !IsEnabled || ReadOnly) return;
            if (args.Key is VirtualKey.Down or VirtualKey.Up && !IsSuggestionListOpen)
            {
                RefreshSuggestions(IsChoice ? "" : Text);
                Input.IsSuggestionListOpen = true;
            }
            if (!IsSuggestionListOpen || QueryPopupSkin.List(Input) is not ListView list) return;
            if (args.Key == VirtualKey.Enter && list.SelectedItem is ListViewItem row
                && row.Tag is not SelectOption { Disabled: false })
            {
                // Disabled options stay discoverable by keyboard, as in Base
                // UI, but native ASB would otherwise close on every Enter.
                args.Handled = true;
            }
            else if (args.Key is VirtualKey.Down or VirtualKey.Up)
            {
                var step = args.Key == VirtualKey.Down ? 1 : -1;
                var next = list.SelectedIndex < 0 ? step > 0 ? 0 : list.Items.Count - 1 : list.SelectedIndex + step;
                if (next >= 0 && next < list.Items.Count && list.Items[next] is not ListViewItem { Tag: SelectOption })
                {
                    // Native ASB navigates raw indices. Group labels and the
                    // empty message are decoration, not navigable options.
                    while (next >= 0 && next < list.Items.Count && list.Items[next] is not ListViewItem { Tag: SelectOption }) next += step;
                    list.SelectedIndex = next >= 0 && next < list.Items.Count ? next : -1;
                    args.Handled = true;
                }
            }
        };
        Input.RegisterPropertyChangedCallback(AutoSuggestBox.IsSuggestionListOpenProperty, (_, _) =>
        {
            if (!mounted) return;
            if (Input.IsSuggestionListOpen && (!IsEnabled || ReadOnly)) Input.IsSuggestionListOpen = false;
            QueryPopupSkin.Sheet(this);
            RefreshActions();
            if (IsChoice && !Input.IsSuggestionListOpen) RestoreSelection();
        });
        Skin.Input(Input, Theme, Size, Invalid);
    }

    private void OnTextChanged(AutoSuggestBox sender, AutoSuggestBoxTextChangedEventArgs args)
    {
        // AutoSuggestBox's debounce timer can outlive native template disposal.
        // Check managed lifetime state before reading any dependency property.
        if (!mounted || !args.CheckCurrent()) return;
        RefreshInput();
        if (args.Reason != AutoSuggestionBoxTextChangeReason.UserInput || !IsEnabled || ReadOnly) return;
        var wasOpen = Input.IsSuggestionListOpen;
        UserTextChanged(Text);
        RefreshSuggestions(Text);
        if (Text.Trim().Length > 0) Input.IsSuggestionListOpen = true;
        else Input.IsSuggestionListOpen = IsChoice && wasOpen;
    }

    protected virtual void UserTextChanged(string text) => ValueChanged?.Invoke(this, text);
    protected virtual void Choose(SelectOption option)
    {
        selectedValue = option.Value;
        Text = option.Label;
        ValueChanged?.Invoke(this, Text);
    }

    protected void ShowSelection(string? value)
    {
        selectedValue = Options.FirstOrDefault(option => option.Value == value)?.Value;
        RestoreSelection();
        RefreshRows();
        RefreshActions();
    }

    protected void RestoreSelection()
    {
        var label = Options.FirstOrDefault(option => option.Value == selectedValue)?.Label ?? "";
        if (Input.Text != label) Input.Text = label;
    }

    protected void RefreshSuggestions(string query)
    {
        var items = new List<ListViewItem>();
        string? group = null;
        var options = IsChoice || Options.Count > 0 ? Options : Suggestions.Select(text => new SelectOption(text, text));
        foreach (var option in options.Where(option => System.Globalization.CultureInfo.CurrentCulture.CompareInfo.IndexOf(option.Label, query.Trim(),
            System.Globalization.CompareOptions.IgnoreCase | System.Globalization.CompareOptions.IgnoreNonSpace) >= 0))
        {
            if (option.Group is not null && option.Group != group)
            {
                var heading = new ListViewItem { Content = option.Group, Tag = QueryPopupSkin.RowKind.Heading, IsEnabled = false, IsTabStop = false };
                SelectionPopupSkin.Row(heading, Theme, check: false, heading: true, groupGap: items.Count > 0);
                group = option.Group;
                items.Add(heading);
            }
            var row = new ListViewItem { Content = option.Label, Tag = option, IsEnabled = !option.Disabled };
            SelectionPopupSkin.Row(row, Theme, selected: IsChoice && option.Value == selectedValue, check: IsChoice);
            items.Add(row);
        }
        if (items.Count == 0)
        {
            var empty = new ListViewItem { Content = Empty, Tag = QueryPopupSkin.RowKind.Empty, IsEnabled = false, IsTabStop = false };
            SelectionPopupSkin.Row(empty, Theme, check: false, empty: true);
            items.Add(empty);
        }
        Input.ItemsSource = items;
        QueryPopupSkin.Sheet(this);
    }

    private void RefreshRows()
    {
        if (!mounted) return;
        if (Input.ItemsSource is not System.Collections.IEnumerable source) return;
        var rows = source.Cast<object>().OfType<ListViewItem>().ToArray();
        for (var index = 0; index < rows.Length; index++)
        {
            var row = rows[index];
            var heading = Equals(row.Tag, QueryPopupSkin.RowKind.Heading);
            var empty = Equals(row.Tag, QueryPopupSkin.RowKind.Empty);
            SelectionPopupSkin.Row(row, Theme, selected: IsChoice && row.Tag is SelectOption option && option.Value == selectedValue,
                check: IsChoice, heading: heading, empty: empty, groupGap: heading && index > 0);
        }
    }

    protected void OpenSuggestions()
    {
        if (!mounted || !IsEnabled || ReadOnly) return;
        RefreshSuggestions("");
        QueryPopupSkin.Editor(Input)?.Focus(FocusState.Programmatic);
        Input.IsSuggestionListOpen = true;
    }

    protected virtual void RefreshActions() { }
    protected virtual double TrailingInset => Theme.Number("spacing.3");

    protected void RefreshInput()
    {
        if (!mounted) return;
        if (QueryPopupSkin.Editor(Input) is not TextBox editor) return;
        Appearance.SetTint(editor, Tint);
        Skin.Input(editor, Theme, Size, Invalid);
        editor.Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        editor.IsReadOnly = ReadOnly;
        editor.IsEnabled = IsEnabled;
        if (Mono) editor.FontFamily = Theme.CodeFont;
        editor.Padding = new Thickness(Theme.Number("spacing.3"), editor.Padding.Top, TrailingInset, editor.Padding.Bottom);
        QueryPopupSkin.HideNativeActions(editor);
        RefreshActions();
    }

    protected override void RefreshAppearance()
    {
        if (!mounted) return;
        Input.IsEnabled = IsEnabled;
        Appearance.SetTint(Input, Tint);
        Skin.Input(Input, Theme, Size, Invalid);
        if (Mono) Input.FontFamily = Theme.CodeFont;
        Input.Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        RefreshInput();
        RefreshActions();
        RefreshRows();
        QueryPopupSkin.Sheet(this);
    }
}
public sealed class Combobox : Autocomplete
{
    private readonly StackPanel actions = new() { Orientation = Orientation.Horizontal, HorizontalAlignment = HorizontalAlignment.Right, VerticalAlignment = VerticalAlignment.Center };
    private readonly QueryPopupSkin.ActionButton clear = new(FluentIcons.Common.Icon.Dismiss, "Clear");
    private readonly QueryPopupSkin.ActionButton trigger = new(FluentIcons.Common.Icon.ChevronDown, "Open");
    private bool clearable = true;
    public bool Clearable { get => clearable; set { clearable = value; RefreshInput(); } }
    public string? Value { get => SelectedValue; set => SelectedValue = value; }
    public new event EventHandler<string?>? ValueChanged;
    protected override bool IsChoice => true;
    public Combobox()
    {
        PlaceholderText = "Search…";
        Content = null;
        Content = new Grid { Children = { Input, actions } };
        actions.Children.Add(clear);
        actions.Children.Add(trigger);
        clear.Click += (_, _) =>
        {
            if (!IsMounted || !IsEnabled || ReadOnly) return;
            var open = IsSuggestionListOpen;
            Commit(null);
            // Replacing ItemsSource makes native ASB asynchronously reopen its
            // popup. A closed clear only changes the value; the next explicit
            // open will populate the complete list.
            if (open) RefreshSuggestions("");
            QueryPopupSkin.Editor(Input)?.Focus(FocusState.Programmatic);
            IsSuggestionListOpen = open;
        };
        trigger.Click += (_, _) => { if (!IsMounted) return; if (IsSuggestionListOpen) IsSuggestionListOpen = false; else OpenSuggestions(); };
        Input.Tapped += (_, args) =>
        {
            if (!IsMounted) return;
            if (!IsSuggestionListOpen && QueryPopupSkin.Editor(Input) is TextBox editor
                && QueryPopupSkin.Walk(editor).Any(element => ReferenceEquals(element, args.OriginalSource))) OpenSuggestions();
        };
    }

    protected override void UserTextChanged(string text) { if (text.Length == 0) Commit(null); }
    protected override void Choose(SelectOption option) => Commit(option.Value);
    private void Commit(string? value)
    {
        if (Value == value) { RestoreSelection(); return; }
        ShowSelection(value);
        ValueChanged?.Invoke(this, Value);
        RefreshInput();
    }
    protected override double TrailingInset => Theme.Number("spacing.1") * 2 + Theme.Number("spacing.5") * (Clearable && SelectedValue is not null ? 2 : 1);
    protected override void RefreshActions()
    {
        // Base construction can resolve appearance before the action fields
        // have been initialized.
        if (actions is null || !IsMounted) return;
        actions.Margin = new Thickness(0, 0, Theme.Number("spacing.1") + Theme.Number("stroke.hairline"), 0);
        clear.Visibility = Clearable && SelectedValue is not null ? Visibility.Visible : Visibility.Collapsed;
        foreach (var action in new[] { clear, trigger }) action.Apply(Theme, IsEnabled && !ReadOnly);
        trigger.RenderTransformOrigin = new Windows.Foundation.Point(.5, .5);
        trigger.RenderTransform = new RotateTransform { Angle = IsSuggestionListOpen ? 180 : 0 };
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
        RefreshAppearance();
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
        // The editor's state brushes become transparent below. The group
        // keeps independent copies of the field's resolved surface and edge.
        box.Background = focused && !Invalid ? Theme.Brush("color.surface") : new SolidColorBrush(((SolidColorBrush)editor.Background).Color);
        box.BorderBrush = focused || Invalid ? Theme.Brush($"color.{(Invalid ? "danger" : Theme.Key(Tint))}.600") : new SolidColorBrush(((SolidColorBrush)editor.BorderBrush).Color);
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
            Skin.StateBrush(editor, key, clear);
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
        unit.IsTextScaleFactorEnabled = Theme.Sizing != ThemeSizing.System;
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
            Skin.StateBrush(step, "ButtonBackgroundPointerOver", Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered"));
            Skin.StateBrush(step, "ButtonBackgroundPressed", Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "pressed"));
            Skin.StateBrush(step, "ButtonBorderBrushPointerOver", clear);
            Skin.StateBrush(step, "ButtonBorderBrushPressed", clear);
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
    private readonly TextBlock label = new() { TextWrapping = TextWrapping.Wrap };
    private readonly TextBlock message = new() { TextWrapping = TextWrapping.Wrap };
    private readonly ContentPresenter fieldHost = new() { HorizontalContentAlignment = HorizontalAlignment.Stretch };
    private readonly StackPanel body = new();
    public string Label { get; set; } = "";
    public string? Hint { get; set; }
    public string? Error { get; set; }
    public bool Required { get; set; }
    public UIElement? Field { get; set; }
    public FormField()
    {
        body.Children.Add(label);
        body.Children.Add(fieldHost);
        body.Children.Add(message);
        Content = body;
    }
    protected override void RefreshAppearance()
    {
        var subtle = Theme.Brush("color.content-subtle");
        var danger = Theme.Brush("color.danger.700");
        Theme.Text("", "labelSmall").CopyTo(label);
        label.Text = Label + (Required ? " *" : "");
        label.Foreground = Error is null ? subtle : danger;
        body.Spacing = Theme.Number("spacing.1.5");
        // Keep the native editor attached through validation and theme changes.
        Skin.Content(fieldHost, Field);
        if (Field is not null)
        {
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetLabeledBy(Field, label);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetIsRequiredForForm(Field, Required);
            if (Field is SecureTextField secure)
            {
                Microsoft.UI.Xaml.Automation.AutomationProperties.SetLabeledBy(secure.Input, label);
                Microsoft.UI.Xaml.Automation.AutomationProperties.SetIsRequiredForForm(secure.Input, Required);
            }
        }
        Theme.Text("", "labelQuiet").CopyTo(message);
        message.Text = Error ?? Hint ?? "";
        message.LineHeight = message.FontSize * 1.6;
        message.Foreground = Error is null ? subtle : danger;
        message.Visibility = (Error ?? Hint) is null ? Visibility.Collapsed : Visibility.Visible;
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
        TextStyle.SetWeight(legend, Theme.Weight("labelSmall"));
        // The legend's own leading: one, as the web sets it, not the caption
        // step's.
        legend.LineHeight = legend.FontSize;
        legend.LineStackingStrategy = LineStackingStrategy.BlockLineHeight;
        legend.Foreground = Theme.Brush("color.content-subtle");
        var body = Content as StackPanel ?? new StackPanel();
        body.Spacing = Theme.Number("spacing.3");
        Skin.Children(body, new UIElement[] { legend }.Concat(Fields));
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
        var body = Content as StackPanel ?? new StackPanel();
        body.Spacing = Theme.Number("spacing.3");
        Skin.Children(body, Fields.Cast<UIElement>().Concat(Footer is null ? [] : new[] { Footer }));
        Content = body;
    }
}
