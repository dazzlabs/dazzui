using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Input;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class QueryFieldTests
{
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }

    private static async Task Settle(Shell shell)
    {
        await Task.Delay(120);
        shell.UpdateLayout();
    }

    private static void Invoke(FrameworkElement control)
    {
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(control);
        ((IInvokeProvider)peer!.GetPattern(PatternInterface.Invoke)!).Invoke();
    }

    private static Microsoft.UI.Xaml.Controls.Button Action(UI.Autocomplete field, string name) =>
        UI.QueryPopupSkin.Walk(field).OfType<Microsoft.UI.Xaml.Controls.Button>()
            .Single(button => AutomationProperties.GetName(button) == name);

    private static string Describe(object? item) => item is ListViewItem row
        ? $"{row.Content}/{row.Tag}/enabled={row.IsEnabled}/selected={row.IsSelected}"
        : item?.GetType().Name ?? "null";

    private static async Task CapturePopup(Shell shell, UI.Combobox combo, string output, string name, bool empty = false)
    {
        var theme = combo.Theme;
        var popup = UI.QueryPopupSkin.Walk(combo.Input).OfType<Popup>().Single(part => part.Name == "SuggestionsPopup");
        var border = UI.QueryPopupSkin.Walk(popup.Child).OfType<Border>().Single(part => part.Name == "SuggestionsContainer");
        var list = UI.QueryPopupSkin.List(combo.Input)!;
        var rows = ((IEnumerable<ListViewItem>)combo.Input.ItemsSource).ToArray();
        var origin = border.TransformToVisual(shell).TransformPoint(default);
        var fieldOrigin = combo.Input.TransformToVisual(shell).TransformPoint(default);
        var gap = origin.Y >= fieldOrigin.Y ? origin.Y - fieldOrigin.Y - combo.Input.ActualHeight
            : fieldOrigin.Y - origin.Y - border.ActualHeight;
        var shadowHost = UI.QueryPopupSkin.Walk(popup.Child).OfType<Control>().FirstOrDefault(control => UI.Elevation.GetShadow(control) == "lg");
        var lift = shadowHost is null ? null : UI.QueryPopupSkin.Walk(shadowHost).OfType<FrameworkElement>().FirstOrDefault(part => part.Name == "DazzSelectionLift");
        var shadowVisual = lift is null ? null : Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.GetElementChildVisual(lift);
        await NativeSnapshot.SaveAsync(border, output, name + ".png", theme.Number("spacing.8"));
        await File.WriteAllTextAsync(Path.Combine(output, name + "-readings.json"), System.Text.Json.JsonSerializer.Serialize(new
        {
            theme = theme.Name, fieldWidth = combo.Input.ActualWidth, width = border.ActualWidth, height = border.ActualHeight,
            gap, padding = border.Padding.Left, radius = border.CornerRadius.TopLeft,
            fill = (border.Background as SolidColorBrush)?.Color.ToString(),
            edge = (border.BorderBrush as SolidColorBrush)?.Color.ToString(),
            shadow = popup.Child.Shadow?.GetType().Name, shadowVisual = shadowVisual?.GetType().Name,
            rows = rows.Select(row => new
            {
                content = row.Content?.ToString(), width = row.ActualWidth, height = row.ActualHeight,
                padX = row.Padding.Left, padTop = row.Padding.Top, padBottom = row.Padding.Bottom,
                row.FontSize, fontWeight = row.FontWeight.Weight, selected = row.IsSelected, enabled = row.IsEnabled,
                fill = (row.Background as SolidColorBrush)?.Color.ToString(), ink = (row.Foreground as SolidColorBrush)?.Color.ToString(),
            }),
        }, new System.Text.Json.JsonSerializerOptions { WriteIndented = true }));
        Check(Math.Abs(border.ActualWidth - combo.Input.ActualWidth) < .5
            && border.Padding == new Thickness(theme.Number("spacing.1"))
            && border.BorderThickness == new Thickness(theme.Number("stroke.hairline"))
            && border.CornerRadius == new CornerRadius(theme.Number("radius.large"))
            && ((SolidColorBrush)border.Background).Color == theme.Color("color.surface-overlay")
            && ((SolidColorBrush)border.BorderBrush).Color == theme.Color("color.border-strong"),
            $"{name}: suggestion sheet lost the shared popup box (width={border.ActualWidth}, field={combo.Input.ActualWidth}, padding={border.Padding}, radius={border.CornerRadius}).");
        Check(Math.Abs(gap - theme.Number("spacing.1")) < .5,
            $"{name}: suggestion sheet gap differs from the React positioner (actual={gap}, expected={theme.Number("spacing.1")}).");
        Check(list.Margin == new Thickness(0) && list.Padding == new Thickness(0) && list.BorderThickness == new Thickness(0),
            $"{name}: native suggestions retained extra list insets.");
        Check(shadowHost is not null && popup.Child.Shadow is null && shadowHost.Shadow is null && border.Shadow is null
            && shadowVisual is Microsoft.UI.Composition.ContainerVisual { Children.Count: > 0 },
            $"{name}: suggestion sheet lost its lg lift or retained native elevation (rootShadow={popup.Child.Shadow?.GetType().Name}, hostShadow={shadowHost?.Shadow?.GetType().Name}, borderShadow={border.Shadow?.GetType().Name}, visual={shadowVisual?.GetType().Name}).");
        foreach (var row in rows)
        {
            var heading = Equals(row.Tag, UI.QueryPopupSkin.RowKind.Heading);
            var role = empty ? "bodySmall" : heading ? "captionSmall" : "labelQuiet";
            var expected = theme.Text("", role);
            var expectedHeight = empty ? expected.LineHeight + 2 * theme.Number("menu.item-padding")
                : heading ? expected.FontSize + theme.Number("spacing.1.5") + theme.Number("spacing.1")
                : Math.Max(expected.FontSize, theme.Number("icon.small")) + 2 * theme.Number("menu.item-padding");
            var label = UI.QueryPopupSkin.Walk(row).OfType<TextBlock>().Single(text => text.Text == row.Content?.ToString());
            var check = UI.QueryPopupSkin.Walk(row).OfType<ContentPresenter>().Single(part => part.Name == "DazzSelectionCheck");
            Check(Math.Abs(row.ActualHeight - expectedHeight) < .5 && Math.Abs(label.FontSize - expected.FontSize) < .01
                && label.FontWeight == (heading ? theme.Weight("labelSmall") : expected.FontWeight)
                && Math.Abs(label.LineHeight - (empty ? expected.LineHeight : expected.FontSize)) < .01,
                $"{name}/{row.Content}: suggestion row lost React typography or height (height={row.ActualHeight}, expected={expectedHeight}, font={label.FontSize}, line={label.LineHeight}, weight={label.FontWeight.Weight}).");
            if (heading || empty)
                Check(check.Visibility == Visibility.Collapsed && !row.IsEnabled, $"{name}/{row.Content}: non-option content exposed a selection check.");
            else
                Check(Math.Abs(check.ActualWidth - theme.Number("icon.small")) < .01
                    && check.Opacity == (row.Tag is UI.SelectOption option && option.Value == combo.Value ? 1 : 0)
                    && check.Content is UI.Icon { GlyphVariant: FluentIcons.Common.IconVariant.Filled }
                    && row.Padding == new Thickness(theme.Number("spacing.2"), theme.Number("menu.item-padding"), theme.Number("spacing.2"), theme.Number("menu.item-padding"))
                    && row.CornerRadius == new CornerRadius(theme.Number("radius.large") - theme.Number("spacing.1")),
                    $"{name}/{row.Content}: suggestion row lost its inset, concentric corner or committed check column.");
        }
    }

    private static async Task Edit(Shell shell, UI.Autocomplete field, string text)
    {
        var editor = UI.QueryPopupSkin.Editor(field.Input)
            ?? throw new InvalidOperationException("The AutoSuggestBox native TextBox was not materialized.");
        Check(editor.Focus(FocusState.Programmatic), "Native query editor could not receive focus.");
        await Settle(shell);
        Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), editor),
            $"Native query focus did not settle on the editor (focused={FocusManager.GetFocusedElement(shell.XamlRoot)?.GetType().Name}, enabled={editor.IsEnabled}, readOnly={editor.IsReadOnly}).");
        // The native AutoSuggestBox debounces TextChanged for 150 ms. Wait for
        // the actual user-input notification, not just the TextBox's text DP.
        var changed = new TaskCompletionSource<bool>();
        var reasons = new List<string>();
        void OnTextChanged(AutoSuggestBox sender, AutoSuggestBoxTextChangedEventArgs args)
        {
            reasons.Add($"{args.Reason}:{sender.Text}");
            if (args.Reason == AutoSuggestionBoxTextChangeReason.UserInput && sender.Text == text && args.CheckCurrent())
                changed.TrySetResult(true);
        }
        field.Input.TextChanged += OnTextChanged;
        try
        {
            NativeKeyboard.ReplaceText(shell, text);
            await changed.Task.WaitAsync(TimeSpan.FromSeconds(3));
        }
        catch (TimeoutException)
        {
            throw new InvalidOperationException($"Native query input did not notify the expected text '{text}' (text={field.Text}, events={string.Join(";", reasons)}).");
        }
        finally { field.Input.TextChanged -= OnTextChanged; }
        await Settle(shell);
    }

    public static async Task RunAsync(Shell shell, string output)
    {
        foreach (var name in new[] { "bright-light", "studio-dark", "graphite-light" })
        {
            var theme = new UI.Theme(name);
            var combo = new UI.Combobox { Theme = theme, Width = 280, Empty = "No country by that name" };
            combo.Options.Add(new("jp", "Japan", "Asia"));
            combo.Options.Add(new("cn", "China", "Asia", Disabled: true));
            combo.Options.Add(new("de", "Germany", "Europe"));
            combo.Value = "jp";
            var proposals = new List<string?>();
            combo.ValueChanged += (_, value) => proposals.Add(value);
            var submitted = new List<string>();
            combo.Input.QuerySubmitted += (_, args) => submitted.Add(Describe(args.ChosenSuggestion));
            var outside = new UI.Button { Content = "Outside" };
            var host = new StackPanel { Spacing = 16, Children = { combo, outside } };
            shell.ShowFixture(host);
            await Settle(shell);
            await NativeKeyboard.ActivateAsync(shell);
            Check(combo.Value == "jp" && combo.Text == "Japan" && proposals.Count == 0,
                $"{name}: the selected value and displayed label were not initialized together.");
            var originalEditor = UI.QueryPopupSkin.Editor(combo.Input)!;
            Check(Action(combo, "Clear").Visibility == Visibility.Visible,
                $"{name}: selected combobox did not expose its clear action while unfocused.");
            await Edit(shell, combo, "not a country");
            Check(combo.Value == "jp" && combo.Text == "not a country" && proposals.Count == 0,
                $"{name}: typing a query committed it as an option or replaced it before selection.");
            var emptyRows = (combo.Input.ItemsSource as IEnumerable<ListViewItem>)?.ToArray() ?? [];
            Check(combo.IsSuggestionListOpen && emptyRows.Length == 1
                && emptyRows[0].Content?.ToString() == "No country by that name" && !emptyRows[0].IsEnabled,
                $"{name}: an unmatched query did not show its nonselectable empty message (open={combo.IsSuggestionListOpen}, text={combo.Text}, rows={string.Join(";", emptyRows.Select(row => $"{row.Content}/{row.Tag}/enabled={row.IsEnabled}"))}).");
            await CapturePopup(shell, combo, output, name + "-query-empty", empty: true);
            outside.Focus(FocusState.Programmatic);
            await Settle(shell);
            Check(combo.Value == "jp" && combo.Text == "Japan" && proposals.Count == 0,
                $"{name}: leaving an uncommitted query did not restore its selected label.");

            await Edit(shell, combo, "Ger");
            NativeKeyboard.Press(shell, 0x28);
            await Settle(shell);
            Check(combo.Value == "jp" && combo.Text == "Ger" && proposals.Count == 0,
                $"{name}: highlighting a native suggestion committed it or replaced the query.");
            Check(UI.QueryPopupSkin.List(combo.Input)?.SelectedItem is ListViewItem { Tag: UI.SelectOption { Value: "de" } },
                $"{name}: Down did not skip the filtered group heading (selected={Describe(UI.QueryPopupSkin.List(combo.Input)?.SelectedItem)}).");
            NativeKeyboard.Press(shell, 0x0D);
            await Settle(shell);
            Check(combo.Value == "de" && combo.Text == "Germany" && proposals.SequenceEqual(new string?[] { "de" }),
                $"{name}: Enter did not commit the option value exactly once (value={combo.Value}, text={combo.Text}, events={string.Join(",", proposals)}, submitted={string.Join(";", submitted)}).");
            var openBeforeClear = combo.IsSuggestionListOpen;
            Invoke(Action(combo, "Clear"));
            await Settle(shell);
            Check(combo.Value is null && combo.Text == "" && proposals.Count == 2 && proposals[1] is null,
                $"{name}: clear did not reset both selected value and query exactly once.");
            Check(combo.IsSuggestionListOpen == openBeforeClear,
                $"{name}: clear changed the popup's open state (before={openBeforeClear}, after={combo.IsSuggestionListOpen}).");
            combo.Value = "jp";
            combo.Clearable = false;
            await Settle(shell);
            Check(Action(combo, "Clear").Visibility == Visibility.Collapsed,
                $"{name}: clearable=false left a clear action.");
            var openBeforeChevron = combo.IsSuggestionListOpen;
            Invoke(Action(combo, "Open"));
            await Settle(shell);
            var rows = ((IEnumerable<ListViewItem>)combo.Input.ItemsSource).ToArray();
            Check(rows.Count(row => Equals(row.Tag, UI.QueryPopupSkin.RowKind.Heading)) == 2 && rows.Count(row => row.Tag is UI.SelectOption && !row.IsEnabled) == 1,
                $"{name}: grouped options lost their headings or disabled state.");
            Check(!openBeforeChevron && combo.IsSuggestionListOpen,
                $"{name}: the chevron did not open the native list (before={openBeforeChevron}, after={combo.IsSuggestionListOpen}).");
            NativeKeyboard.Press(shell, 0x28);
            await Settle(shell);
            var nativeList = UI.QueryPopupSkin.List(combo.Input)!;
            Check(nativeList.SelectedItem is ListViewItem { Tag: UI.SelectOption { Value: "jp" } },
                $"{name}: opening grouped suggestions did not navigate past the first heading (selected={Describe(nativeList.SelectedItem)}).");
            NativeKeyboard.Press(shell, 0x28);
            await Settle(shell);
            Check(nativeList.SelectedItem is ListViewItem { Tag: UI.SelectOption { Value: "cn", Disabled: true } },
                $"{name}: a disabled option was not discoverable with the arrow keys (selected={Describe(nativeList.SelectedItem)}).");
            var beforeDisabledEnter = proposals.Count;
            NativeKeyboard.Press(shell, 0x0D);
            await Settle(shell);
            Check(combo.IsSuggestionListOpen && combo.Value == "jp" && proposals.Count == beforeDisabledEnter
                && nativeList.SelectedItem is ListViewItem { Tag: UI.SelectOption { Value: "cn" } },
                $"{name}: Enter on a disabled suggestion submitted, closed or moved the list (open={combo.IsSuggestionListOpen}, value={combo.Value}, selected={Describe(nativeList.SelectedItem)}).");
            NativeKeyboard.Press(shell, 0x28);
            await Settle(shell);
            Check(nativeList.SelectedItem is ListViewItem { Tag: UI.SelectOption { Value: "de" } },
                $"{name}: Down did not skip the next group heading (selected={Describe(nativeList.SelectedItem)}).");
            NativeKeyboard.Press(shell, 0x26);
            await Settle(shell);
            Check(nativeList.SelectedItem is ListViewItem { Tag: UI.SelectOption { Value: "cn" } },
                $"{name}: Up did not navigate back over the group heading to its disabled option (selected={Describe(nativeList.SelectedItem)}).");
            await CapturePopup(shell, combo, output, name + "-query-grouped");
            var highlighted = rows.FirstOrDefault(row => row.IsSelected)
                ?? throw new InvalidOperationException($"{name}: native suggestion navigation did not highlight an option.");
            var source = combo.Input.ItemsSource;
            var nextTheme = new UI.Theme(theme.IsDark ? "studio-light" : "studio-dark");
            combo.Theme = nextTheme;
            await Settle(shell);
            Check(combo.IsSuggestionListOpen && ReferenceEquals(source, combo.Input.ItemsSource) && highlighted.IsSelected
                && ReferenceEquals(originalEditor, UI.QueryPopupSkin.Editor(combo.Input)) && combo.Value == "jp",
                $"{name}: live query theme change replaced native rows/editor, lost highlight or closed the list.");
            Check(((SolidColorBrush)highlighted.Foreground).Color == nextTheme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "content", "disabled").Color,
                $"{name}: an open suggestion retained the old theme ink.");
            var surface = UI.QueryPopupSkin.Walk(combo.Input).OfType<Border>().Single(part => part.Name == "SuggestionsContainer");
            Check(((SolidColorBrush)surface.Background).Color == nextTheme.Color("color.surface-overlay"),
                $"{name}: an open suggestion sheet retained the old palette.");
            combo.Theme = theme;
            await Settle(shell);
            combo.IsSuggestionListOpen = false;
            combo.ReadOnly = true;
            await Settle(shell);
            Check(originalEditor.IsReadOnly && !Action(combo, "Open").IsEnabled,
                $"{name}: readOnly left editing or popup activation enabled.");
            combo.ReadOnly = false;
            combo.IsEnabled = false;
            await Settle(shell);
            Check(!combo.Input.IsEnabled && !originalEditor.IsEnabled,
                $"{name}: disabled state did not reach the native input.");
            var disabledAction = Action(combo, "Open");
            Check(!disabledAction.IsEnabled && disabledAction.Width == theme.Number("spacing.5")
                && disabledAction.Height == theme.Number("spacing.5") && disabledAction.Content is UI.Icon glyph
                && glyph.GlyphSize == theme.Number("icon.small")
                && ((SolidColorBrush)glyph.Foreground).Color == theme.Color("color.content-subtle"),
                $"{name}: a disabled query action lost the React action geometry or subtle glyph ink.");
            Check(ReferenceEquals(originalEditor, UI.QueryPopupSkin.Editor(combo.Input)),
                $"{name}: updating query field state replaced its native editor.");
            await SmokeTest.SaveAsync(host, output, name + "-query-fields.png");
        }

        var bottom = new UI.Combobox
        {
            Width = 280, Theme = new UI.Theme("bright-light"),
            HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center,
        };
        for (var index = 0; index < 10; index++) bottom.Options.Add(new(index.ToString(), $"Option {index}"));
        bottom.Value = "0";
        shell.ShowFixture(new Grid { Width = 560, Height = 400, Children = { bottom } });
        await Settle(shell);
        var anchor = bottom.TransformToVisual(shell.XamlRoot.Content).TransformPoint(default);
        bottom.RenderTransform = new TranslateTransform
        {
            Y = shell.XamlRoot.Size.Height - anchor.Y - bottom.ActualHeight - bottom.Theme.Number("spacing.3"),
        };
        await Settle(shell);
        bottom.IsSuggestionListOpen = false;
        Invoke(Action(bottom, "Open"));
        await Settle(shell);
        try
        {
            var popup = UI.QueryPopupSkin.Walk(bottom.Input).OfType<Popup>().Single(part => part.Name == "SuggestionsPopup");
            Check(bottom.IsSuggestionListOpen && popup.VerticalOffset < 0,
                $"A bottom-edge query did not flip its native suggestion list above (open={bottom.IsSuggestionListOpen}, offset={popup.VerticalOffset}).");
            await CapturePopup(shell, bottom, output, "bright-light-query-above");
        }
        finally { bottom.IsSuggestionListOpen = false; bottom.RenderTransform = null; }

        var auto = new UI.Autocomplete { Width = 280, Text = "custom family" };
        auto.Suggestions.Add("Inter");
        auto.Suggestions.Add("JetBrains Mono");
        var edits = new List<string>();
        auto.ValueChanged += (_, value) => edits.Add(value);
        var blur = new UI.Button { Content = "Outside" };
        shell.ShowFixture(new StackPanel { Spacing = 16, Children = { auto, blur } });
        await Settle(shell);
        await Edit(shell, auto, "Unlisted family");
        blur.Focus(FocusState.Programmatic);
        await Settle(shell);
        Check(auto.Text == "Unlisted family" && edits.LastOrDefault() == "Unlisted family",
            "Autocomplete rejected a free-form value on blur.");
        Check(UI.QueryPopupSkin.Walk(auto.Input).OfType<Microsoft.UI.Xaml.Controls.Button>()
            .Where(button => button.Name is "DeleteButton" or "QueryButton").All(button => button.Width == 0 && !button.IsHitTestVisible),
            "Autocomplete exposed a native clear button or chevron absent from React.");

        var state = new SceneState(new JsonObject { ["country"] = "jp" });
        var controlled = (UI.Combobox)Nodes.Build(JsonNode.Parse("""
            {"$el":"Combobox","props":{"value":{"$state":"country"},"options":[{"value":"jp","label":"Japan"},{"value":"de","label":"Germany","disabled":true}],"required":true}}
            """), new Context(state))!;
        shell.ShowFixture(controlled);
        await Settle(shell);
        Invoke(Action(controlled, "Clear"));
        await Settle(shell);
        Check(controlled.Value == "jp" && controlled.Text == "Japan",
            "A controlled combobox accepted a clear that its owner did not commit.");
        state.Write("country", JsonValue.Create("de"));
        await Settle(shell);
        Check(controlled.Value == "de" && controlled.Text == "Germany" && controlled.Options[1].Disabled
            && AutomationProperties.GetIsRequiredForForm(controlled.Input),
            "Scene mapping lost a controlled value, disabled option or required metadata.");

        var retired = new UI.Autocomplete { Text = "Initial" };
        var lateEdits = 0;
        retired.ValueChanged += (_, _) => lateEdits++;
        shell.ShowFixture(retired);
        await Settle(shell);
        retired.Text = "Pending native notification";
        var replacement = new UI.Button { Content = "Replacement fixture" };
        shell.ShowFixture(replacement);
        await Task.Delay(300);
        shell.UpdateLayout();
        Check(!retired.IsLoaded && replacement.IsLoaded && lateEdits == 0,
            "An unloaded query field processed its deferred native text callback.");
    }
}
