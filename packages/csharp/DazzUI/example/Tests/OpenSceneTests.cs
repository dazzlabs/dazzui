using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI.Example;

/// <summary>Open the generated React stories, so popup snapshots exercise the real mapping.</summary>
internal static class OpenSceneTests
{
    internal static async Task RunAsync(Shell shell, string output)
    {
        var report = new JsonObject();
        foreach (var name in new[] { "bright-light", "studio-dark", "graphite-light" })
        foreach (var id in new[] { "menu", "popover", "tooltip" })
        {
            var theme = new Theme(name);
            var story = Stories.All.Single(story => story.Id == id);
            var scene = new SceneView(story.Examples[0].Demo, align: true);
            var scope = new ThemeScope { Theme = theme, Content = scene, Width = 640, Padding = new Thickness(theme.Number("spacing.6")) };
            shell.ShowFixture(scope);
            await Task.Delay(100); shell.UpdateLayout();
            await NativeKeyboard.ActivateAsync(shell);
            var trigger = OverlaySkin.Walk(scene).OfType<Button>().First();
            Control popup;
            ToolTip? tooltip = null;
            if (id == "tooltip")
            {
                tooltip = ToolTipService.GetToolTip(trigger) as ToolTip ?? throw new InvalidOperationException("The story's tooltip is missing.");
                tooltip.IsOpen = true;
                popup = tooltip;
            }
            else
            {
                var peer = FrameworkElementAutomationPeer.CreatePeerForElement(trigger);
                ((IInvokeProvider)peer!.GetPattern(PatternInterface.Invoke)).Invoke();
                await Task.Delay(180); shell.UpdateLayout();
                popup = VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
                    .SelectMany(item => OverlaySkin.Walk(item.Child)).OfType<Control>()
                    .Single(control => id == "menu" ? control is MenuFlyoutPresenter : control is FlyoutPresenter);
            }
            await Task.Delay(100); shell.UpdateLayout();
            var reading = Measure.Read(popup, new HashSet<DependencyObject>());
            if (popup is MenuFlyoutPresenter menu)
                reading["rows"] = new JsonArray(menu.Items.OfType<MenuFlyoutItemBase>()
                    .Select(item => (JsonNode)Measure.Read(item, new HashSet<DependencyObject>())).ToArray());
            report[$"{name}/{id}"] = reading;
            await NativeSnapshot.SaveAsync(popup, output, $"{name}-{id}-opened.png", theme.Number("spacing.8"));
            if (tooltip is not null) tooltip.IsOpen = false;
            else NativeKeyboard.Press(shell, 0x1B);
            await Task.Delay(120);
        }
        await File.WriteAllTextAsync(Path.Combine(output, "open-scenes.json"), report.ToJsonString(new() { WriteIndented = true }));
    }
}
