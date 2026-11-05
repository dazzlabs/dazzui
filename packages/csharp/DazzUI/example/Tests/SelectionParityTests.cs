using System.Text.Json.Nodes;
using FluentIcons.WinUI;
using Microsoft.UI.Composition;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Hosting;
using Microsoft.UI.Xaml.Media;

namespace DazzUI.Example;

/// <summary>Selection state changes must preserve the same drawing as the React controls.</summary>
internal static class SelectionParityTests
{
    private static void Check(bool pass, string message)
    {
        if (!pass) throw new InvalidOperationException(message);
    }
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }
    private static async Task Settle(Shell shell)
    {
        await Task.Delay(80);
        shell.UpdateLayout();
    }
    private static void Select(RadioButton control)
    {
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(control);
        Check(peer?.GetPattern(PatternInterface.SelectionItem) is ISelectionItemProvider, "A native selection item lost its automation pattern.");
        ((ISelectionItemProvider)peer!.GetPattern(PatternInterface.SelectionItem)).Select();
    }

    internal static async Task RunAsync(Shell shell, string output)
    {
        File.WriteAllText(Path.Combine(output, "progress.txt"), "Selection drawing, disabled options, native activation and controlled proposals");
        var toggle = new Toggle { Content = "Pinned", IsChecked = true };
        var toggleChanges = new List<string>();
        string ToggleState() => $"checked={toggle.IsChecked}, enabled={toggle.IsEnabled}, weight={toggle.FontWeight.Weight}, size={toggle.Size}, theme={toggle.Theme.Name}, loaded={toggle.IsLoaded}, focus={toggle.FocusState}";
        void Trace(string reason) => toggleChanges.Add($"{reason}: {ToggleState()}");
        toggle.Checked += (_, _) => Trace("checked");
        toggle.Unchecked += (_, _) => Trace("unchecked");
        toggle.IsEnabledChanged += (_, _) => Trace("enabled changed");
        toggle.RegisterPropertyChangedCallback(Control.FontWeightProperty, (_, _) => Trace("weight changed"));
        var glyphToggle = new Toggle { Content = new Icon { Glyph = FluentIcons.Common.Icon.Pin } };
        var check = new Checkbox { Content = "Include files", IsChecked = true };
        var mixed = new Checkbox { Content = "Some files", IsThreeState = true, IsChecked = null };
        var segments = new SegmentedControl { SelectedIndex = 0, Stretch = true, Width = 360 };
        segments.Items.Add("First"); segments.Items.Add("Unavailable"); segments.Items.Add("Last");
        segments.DisabledItems.Add(1);
        var pills = new PillTabs { SelectedIndex = 0 };
        pills.Items.Add("All"); pills.Items.Add("Unavailable"); pills.Items.Add("Unread");
        pills.DisabledItems.Add(1); pills.Counts[0] = 24;
        var panel = new StackPanel { Spacing = 12, Children = { toggle, glyphToggle, check, mixed, segments, pills } };
        // These fixtures inspect normal/disabled drawing and use native
        // automation for activation. A stationary desktop pointer must not
        // press or hover a control that moves beneath it during layout.
        var scope = new ThemeScope { Content = panel, Padding = new Thickness(24), HorizontalAlignment = HorizontalAlignment.Left, IsHitTestVisible = false };
        shell.ShowFixture(scope);
        foreach (var name in new[] { "studio-light", "bright-dark", "graphite-light", "windows11-dark" })
        {
            var theme = new Theme(name);
            scope.Theme = theme;
            scope.Background = theme.Brush("color.surface");
            foreach (var size in new[] { ControlSize.Small, ControlSize.Medium, ControlSize.Large })
            {
                toggle.Size = glyphToggle.Size = check.Size = mixed.Size = segments.Size = size;
                toggle.IsEnabled = true;
                await Settle(shell);
                Check(toggle.IsChecked == true && toggle.IsEnabled && toggle.FontWeight == theme.Weight("labelStrong"), $"{name}/{size}: enabled latched toggle lost its strong label: {ToggleState()}, expectedWeight={theme.Weight("labelStrong").Weight}; recent=[{string.Join("; ", toggleChanges.TakeLast(12))}].");
                toggle.IsEnabled = false;
                await Settle(shell);
                Check(toggle.IsChecked == true && !toggle.IsEnabled && toggle.FontWeight == theme.Weight($"control.{Theme.Key(size)}.content"), $"{name}/{size}: disabled toggle changed selection or retained the strong label: {ToggleState()}, expectedWeight={theme.Weight($"control.{Theme.Key(size)}.content").Weight}; recent=[{string.Join("; ", toggleChanges.TakeLast(12))}].");
                Check(((SolidColorBrush)toggle.Foreground).Color == theme.Recipe(ControlVariant.Tinted, toggle.Tint, "content", "disabled").Color, $"{name}/{size}: disabled toggle retained live ink.");
                var glyph = (Icon)glyphToggle.Content;
                Check(Math.Abs(glyph.ActualWidth - theme.Number($"control.{Theme.Key(size)}.icon-size")) < .1, $"{name}/{size}: glyph-only toggle ignored its control profile.");
                foreach (var boxControl in new[] { check, mixed })
                {
                    var box = Walk(boxControl).OfType<Border>().Single(element => element.Name == "Box");
                    var mark = box.Child as FluentIcon;
                    var available = box.ActualWidth - box.BorderThickness.Left - box.BorderThickness.Right;
                    Check(mark is not null && Math.Abs(mark.ActualWidth - Math.Min(theme.Number("icon.small"), available)) < .1, $"{name}/{size}: checkbox mark exceeded its content box or icon scale.");
                    Check(mark!.IconVariant == FluentIcons.Common.IconVariant.Filled && mark.IconSize == FluentIcons.Common.IconSize.Size12, $"{name}/{size}: checkbox changed the reference's filled twelve-pixel glyph.");
                    Check(mark.Icon == (boxControl == mixed ? FluentIcons.Common.Icon.Subtract : FluentIcons.Common.Icon.Checkmark), $"{name}/{size}: checkbox mixed state drew the wrong mark.");
                }
                Check(!segments.Segments[1].IsEnabled && segments.Segments[1].Opacity == .5, $"{name}/{size}: disabled segment remained live.");
                Check(segments.Segments.Max(item => item.ActualWidth) - segments.Segments.Min(item => item.ActualWidth) < .1, $"{name}/{size}: stretched segments did not divide the track equally: track={segments.ActualWidth:0.###}, inset={segments.Padding.Left:0.###}, segments=[{string.Join(", ", segments.Segments.Select(item => $"{item.ActualWidth:0.###} ({item.HorizontalAlignment})"))}].");
                foreach (var segment in segments.Segments)
                {
                    var label = Walk(segment).OfType<TextBlock>().First();
                    Check(Math.Abs(label.LineHeight - segment.FontSize) < .1, $"{name}/{size}: segment retained the control's leading instead of line-height one.");
                    var lift = Walk(segment).OfType<FrameworkElement>().Single(element => element.Name == "Lift");
                    var painted = ElementCompositionPreview.GetElementChildVisual(lift) is ContainerVisual { Children.Count: > 0 };
                    Check(painted == (segment.IsChecked == true), $"{name}/{size}: segment shadow did not follow selection.");
                }
            }
            Select(segments.Segments[2]); Select(pills.Tabs[2]); await Settle(shell);
            Check(segments.SelectedIndex == 2 && pills.SelectedIndex == 2, $"{name}: native activation did not update selection.");
            Check(!pills.Tabs[1].IsEnabled && pills.Tabs[1].Opacity == .5, $"{name}: disabled pill lost its disabled drawing.");
            Check(pills.Tabs[2].FontWeight == theme.Weight("labelStrong"), $"{name}: active pill lost its strong label.");
            await SmokeTest.SaveAsync(scope, output, $"selection-{name}.png");
        }
        var originalTab = pills.Tabs[0];
        pills.Items[0] = "Everything"; pills.Counts.Remove(0); Appearance.Refresh(pills);
        var originalSegment = segments.Segments[0];
        segments.Items[0] = "Renamed"; Appearance.Refresh(segments);
        await Settle(shell);
        Check(ReferenceEquals(originalTab, pills.Tabs[0]) && Walk(pills.Tabs[0]).OfType<TextBlock>().Any(text => text.Text == "Everything") && !Walk(pills.Tabs[0]).OfType<TextBlock>().Any(text => text.Text == "24"), "Updating a pill left a stale label/count or replaced the native selection item.");
        Check(ReferenceEquals(originalSegment, segments.Segments[0]) && Walk(segments.Segments[0]).OfType<TextBlock>().Any(text => text.Text == "Renamed"), "Renaming a segment left a stale label or replaced the native selection item.");

        foreach (var component in new[] { "PillTabs", "SegmentedControl" })
        {
            var state = new SceneState(new JsonObject { ["selected"] = "first" });
            var json = new JsonObject
            {
                ["$el"] = component,
                ["props"] = new JsonObject
                {
                    ["value"] = new JsonObject { ["$state"] = "selected" },
                    ["items"] = JsonNode.Parse("""[{"value":"first","label":"First"},{"value":"disabled","label":"Disabled","disabled":true},{"value":"last","label":"Last"}]"""),
                    ["stretch"] = true,
                },
            };
            var control = (FrameworkElement)Nodes.Build(json, new Context(state))!;
            control.IsHitTestVisible = false;
            shell.ShowFixture(control); await Settle(shell);
            IReadOnlyList<RadioButton> Options() => control is PillTabs tabs ? tabs.Tabs : ((SegmentedControl)control).Segments;
            Check(!Options()[1].IsEnabled, $"{component}: scene mapping discarded an item's disabled flag.");
            Select(Options()[2]); await Settle(shell);
            Check(Options()[0].IsChecked == true && Options()[2].IsChecked == false, $"{component}: a refused controlled proposal changed the native selection.");
            state.Write("selected", JsonValue.Create("last")); await Settle(shell);
            Check(Options()[2].IsChecked == true, $"{component}: an external controlled value did not reach the existing control.");
            if (control is SegmentedControl segment) Check(segment.Stretch, "SegmentedControl mapping discarded stretch.");
        }
    }
}
