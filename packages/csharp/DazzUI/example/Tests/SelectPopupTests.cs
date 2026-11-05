using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class SelectPopupTests
{
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }
    private static async Task Settle(Shell shell)
    {
        await Task.Delay(120);
        shell.UpdateLayout();
    }
    private static T Pattern<T>(FrameworkElement owner, PatternInterface pattern) where T : class
    {
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(owner);
        return peer?.GetPattern(pattern) as T
            ?? throw new InvalidOperationException($"{owner.GetType().Name} lost native {pattern} (peer={peer?.GetType().FullName}).");
    }
    private static ContentPresenter CheckSlot(ComboBoxItem row) => Walk(row).OfType<ContentPresenter>()
        .Single(part => part.Name == "DazzSelectionCheck");

    internal static async Task RunAsync(Shell shell, string output)
    {
        var select = new UI.Select { Width = 320, HorizontalAlignment = HorizontalAlignment.Left };
        select.SetOptions([
            new("none", "Unassigned"),
            new("ada", "Ada", "People"),
            new("unavailable", "Unavailable", "People", Disabled: true),
            new("grace", "Grace", "People"),
            new("codex", "Codex", "Agents"),
        ]);
        select.Value = "ada";
        var frame = new Grid { Padding = new Thickness(60), VerticalAlignment = VerticalAlignment.Center, Children = { select } };
        shell.ShowFixture(frame);
        await Settle(shell);
        await NativeKeyboard.ActivateAsync(shell);
        var expand = Pattern<IExpandCollapseProvider>(select, PatternInterface.ExpandCollapse);
        var options = select.Items.OfType<ComboBoxItem>().Where(row => row.Tag is string).ToArray();
        var groups = select.Items.OfType<ComboBoxItem>().Where(row => row.Tag is null).ToArray();
        var ada = options.Single(row => Equals(row.Tag, "ada"));
        var disabled = options.Single(row => Equals(row.Tag, "unavailable"));
        var grace = options.Single(row => Equals(row.Tag, "grace"));
        foreach (var name in new[] { "bright-light", "studio-dark", "graphite-light" })
        {
            var theme = select.Theme = new UI.Theme(name);
            select.Value = "ada";
            Check(select.Focus(FocusState.Keyboard), $"{name}: Select could not receive native focus.");
            expand.Expand();
            await Settle(shell);
            try
            {
                Check(select.IsDropDownOpen, $"{name}: native Select popup did not open.");
                var popup = VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
                    .Single(candidate => Walk(candidate.Child).Contains(ada));
                var border = Walk(popup.Child).OfType<Border>().Single(part => part.Name == "PopupBorder");
                var scroll = Walk(border).OfType<ScrollViewer>().Single(part => part.Name == "ScrollViewer");
                Check(Math.Abs(border.ActualWidth - select.ActualWidth) < .5,
                    $"{name}: Select popup must use its trigger width (popup={border.ActualWidth}, trigger={select.ActualWidth}).");
                Check(border.Padding == new Thickness(theme.Number("spacing.1"))
                    && border.BorderThickness == new Thickness(theme.Number("stroke.hairline"))
                    && border.CornerRadius == new CornerRadius(theme.Number("radius.large"))
                    && border.Background is SolidColorBrush fill && fill.Color == theme.Color("color.surface-overlay")
                    && border.BorderBrush is SolidColorBrush edge && edge.Color == theme.Color("color.border-strong"),
                    $"{name}: Select popup did not use the shared sheet geometry and ink.");
                var shadowHost = Walk(popup.Child).OfType<Control>().FirstOrDefault(control => UI.Elevation.GetShadow(control) == "lg");
                var lift = shadowHost is null ? null : Walk(shadowHost).OfType<FrameworkElement>().FirstOrDefault(part => part.Name == "DazzSelectionLift");
                var shadowVisual = lift is null ? null : Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.GetElementChildVisual(lift);
                Check(shadowHost is not null && shadowHost.Shadow is null && border.Shadow is null
                    && shadowVisual is Microsoft.UI.Composition.ContainerVisual { Children.Count: > 0 },
                    $"{name}: Select popup lost the shared large shadow or retained Windows elevation (root={popup.Child.GetType().Name}, rootToken={UI.Elevation.GetShadow(popup.Child)}, rootShadow={popup.Child.Shadow?.GetType().Name ?? "null"}, host={shadowHost?.GetType().Name ?? "null"}, hostSize={shadowHost?.ActualWidth}x{shadowHost?.ActualHeight}, hostShadow={shadowHost?.Shadow?.GetType().Name ?? "null"}, borderShadow={border.Shadow?.GetType().Name ?? "null"}, lift={shadowVisual?.GetType().Name ?? "null"}).");
                Check(scroll.Padding == new Thickness(0) && scroll.Margin == new Thickness(0),
                    $"{name}: native scrolling retained an extra popup inset.");
                var expectedHeight = 2 * theme.Number("menu.item-padding") + Math.Max(theme.Text("", "labelQuiet").FontSize, theme.Number("icon.small"));
                foreach (var row in options)
                {
                    var slot = CheckSlot(row);
                    Check(Math.Abs(row.ActualHeight - expectedHeight) < .5
                        && row.Padding == new Thickness(theme.Number("spacing.2"), theme.Number("menu.item-padding"), theme.Number("spacing.2"), theme.Number("menu.item-padding"))
                        && row.CornerRadius == new CornerRadius(theme.Number("radius.large") - theme.Number("spacing.1")),
                        $"{name}/{row.Content}: native option retained Windows row geometry (height={row.ActualHeight}, expected={expectedHeight}).");
                    Check(Math.Abs(slot.ActualWidth - theme.Number("icon.small")) < .1
                        && slot.Opacity == (ReferenceEquals(row, ada) ? 1 : 0)
                        && slot.Content is UI.Icon { GlyphVariant: FluentIcons.Common.IconVariant.Filled },
                        $"{name}/{row.Content}: selected checkmark or its reserved column changed.");
                    var label = Walk(row).OfType<TextBlock>().Single(text => text.Text == row.Content?.ToString());
                    var expectedLabel = theme.Text("", "labelQuiet");
                    Check(label.FontWeight == expectedLabel.FontWeight && Math.Abs(label.FontSize - expectedLabel.FontSize) < .01
                        && Math.Abs(label.Height - label.FontSize) < .01 && Math.Abs(label.LineHeight - label.FontSize) < .01,
                        $"{name}/{row.Content}: option lost its quiet label or solid line height (font={label.FontFamily.Source}, size={label.FontSize}, weight={label.FontWeight.Weight}, line={label.LineHeight}, height={label.Height}, actual={label.ActualWidth}x{label.ActualHeight}, expectedSize={expectedLabel.FontSize}, expectedWeight={expectedLabel.FontWeight.Weight}, visual={label.GetType().Name}, parent={VisualTreeHelper.GetParent(label)?.GetType().Name}).");
                }
                Check(!disabled.IsEnabled && disabled.Foreground is SolidColorBrush disabledInk
                    && disabledInk.Color == theme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "content", "disabled").Color,
                    $"{name}: disabled option lost native disabling or its shared ink.");
                foreach (var group in groups)
                {
                    var groupHeight = theme.Text("", "captionSmall").FontSize + theme.Number("spacing.1.5") + theme.Number("spacing.1");
                    Check(!group.IsEnabled && Math.Abs(group.ActualHeight - groupHeight) < .5
                        && CheckSlot(group).Visibility == Visibility.Collapsed && group.FontWeight == theme.Weight("labelSmall"),
                        $"{name}: a group heading retained option geometry or selection affordances.");
                }
                Check(groups[0].Margin.Top == 0 && groups[1].Margin.Top == theme.Number("spacing.1"),
                    $"{name}: group spacing must appear only between adjacent groups.");
                await SmokeTest.SaveAsync(popup.Child, output, $"select-popup-{name}.png");

                // The React reference permits arrow focus on disabled options,
                // while headings stay out of navigation and confirmation on a
                // disabled option leaves both the value and popup unchanged.
                NativeKeyboard.Press(shell, 0x24); await Settle(shell);
                NativeKeyboard.Press(shell, 0x28); await Settle(shell);
                NativeKeyboard.Press(shell, 0x28); await Settle(shell);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), disabled)
                    && disabled.AllowFocusWhenDisabled
                    && disabled.Background is SolidColorBrush highlight
                    && highlight.Color == theme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "surface", "hovered").Color,
                    $"{name}: native arrows must highlight the disabled option and skip group headings (focus={(FocusManager.GetFocusedElement(shell.XamlRoot) as ComboBoxItem)?.Content}, disabledFocus={disabled.FocusState}, selected={select.Value}).");
                NativeKeyboard.Press(shell, 0x28); await Settle(shell);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), grace),
                    $"{name}: native Down could not leave the disabled option (focus={(FocusManager.GetFocusedElement(shell.XamlRoot) as ComboBoxItem)?.Content}).");
                NativeKeyboard.Press(shell, 0x26); await Settle(shell);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), disabled),
                    $"{name}: Up skipped the disabled option (focus={(FocusManager.GetFocusedElement(shell.XamlRoot) as ComboBoxItem)?.Content}).");
                foreach (var confirmation in new byte[] { 0x0D, 0x20 })
                {
                    NativeKeyboard.Press(shell, confirmation); await Settle(shell);
                    Check(select.IsDropDownOpen && select.Value == "ada"
                        && ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), disabled),
                        $"{name}: disabled confirmation 0x{confirmation:X2} changed the value, focus, or popup (open={select.IsDropDownOpen}, value={select.Value}, focus={(FocusManager.GetFocusedElement(shell.XamlRoot) as ComboBoxItem)?.Content}).");
                }
                NativeKeyboard.Press(shell, 0x28); await Settle(shell);
                NativeKeyboard.Press(shell, 0x0D); await Settle(shell);
                Check(!select.IsDropDownOpen && select.Value == "grace",
                    $"{name}: native navigation did not leave the disabled option and commit Grace (value={select.Value}, focus={FocusManager.GetFocusedElement(shell.XamlRoot)?.GetType().Name}).");
                expand.Expand(); await Settle(shell);
                Check(ReferenceEquals(select.SelectedItem, grace) && CheckSlot(grace).Opacity == 1 && CheckSlot(ada).Opacity == 0,
                    $"{name}: reopening lost the committed value or kept the old checkmark.");
                var nextTheme = new UI.Theme(name == "bright-light" ? "studio-dark" : "bright-light");
                select.Theme = nextTheme; await Settle(shell);
                Check(border.Background is SolidColorBrush live && live.Color == nextTheme.Color("color.surface-overlay")
                    && grace.Foreground is SolidColorBrush liveInk && liveInk.Color == nextTheme.Color("color.content"),
                    $"{name}: an open Select popup did not follow a live theme change.");
                NativeKeyboard.Press(shell, 0x1B); await Settle(shell);
                Check(!select.IsDropDownOpen && select.Value == "grace", $"{name}: Escape changed the committed value.");

                expand.Expand(); await Settle(shell);
                NativeKeyboard.Press(shell, 0x24); await Settle(shell);
                // React typeahead changes only the active option while open.
                // Repeated 'u' skips the disabled match; the longer disabled
                // prefix cannot focus or select Unavailable either.
                var typeahead = new List<object>();
                var matchedReference = true;
                foreach (var key in new byte[] { 0x55, 0x55, 0x4E, 0x41, 0x56 })
                {
                    NativeKeyboard.Press(shell, key); await Settle(shell);
                    var focused = FocusManager.GetFocusedElement(shell.XamlRoot) as ComboBoxItem;
                    typeahead.Add(new { key = (char)key, open = select.IsDropDownOpen, value = select.Value, focused = focused?.Content?.ToString() });
                    matchedReference &= select.IsDropDownOpen && select.Value == "grace" && ReferenceEquals(focused, options[0]);
                }
                var diagnostic = System.Text.Json.JsonSerializer.Serialize(new { theme = name, states = typeahead });
                await File.AppendAllTextAsync(Path.Combine(output, "select-typeahead.jsonl"), diagnostic + Environment.NewLine);
                Check(matchedReference, $"{name}: open Select typeahead must skip disabled options and leave the committed value unchanged: {diagnostic}");
                NativeKeyboard.Press(shell, 0x1B); await Settle(shell);
            }
            finally { expand.Collapse(); await Settle(shell); }
        }
        select.SetOptions(Enumerable.Range(0, 40).Select(index => new UI.SelectOption(index.ToString(), $"Option {index}")));
        select.Value = "0";
        select.MaxDropDownHeight = 160;
        expand.Expand(); await Settle(shell);
        try
        {
            var popup = VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
                .Single(candidate => Walk(candidate.Child).OfType<ComboBoxItem>().Any());
            var scroll = Walk(popup.Child).OfType<ScrollViewer>().Single(part => part.Name == "ScrollViewer");
            Check(scroll.ScrollableHeight > 0, "Long Select lists lost native constrained scrolling.");
            select.Value = null;
            Check(select.SelectedItem is null, "Clearing a Select value selected a group heading.");
        }
        finally { expand.Collapse(); await Settle(shell); }
    }
}
