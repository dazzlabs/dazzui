using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using UI = DazzUI;

namespace DazzUI.Example;

/// <summary>The titlebar band and its caption controls, as React defines them.</summary>
internal static class WindowChromeTests
{
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }
    private static Microsoft.UI.Xaml.Controls.Button Caption(UI.WindowTitlebar bar, string name) =>
        Walk(bar).OfType<Microsoft.UI.Xaml.Controls.Button>().Single(button => AutomationProperties.GetName(button) == name);

    public static async Task RunAsync(Shell shell, string output)
    {
        var bar = new UI.WindowTitlebar
        {
            Title = "Project",
            Subtitle = "Quarterly report",
            Controls = UI.WindowControls.All,
        };
        shell.ShowFixture(bar);
        await Task.Delay(80);
        shell.UpdateLayout();
        Check(Math.Abs(bar.ActualHeight - 52) < .5, $"A titlebar measured {bar.ActualHeight}px instead of the 52px band.");
        await SmokeTest.SaveAsync(bar, output, "window-titlebar.png");
        var buttons = Walk(bar).OfType<Microsoft.UI.Xaml.Controls.Button>().ToArray();
        Check(buttons.Length == 3, $"A full caption strip has {buttons.Length} controls.");
        Check(buttons.All(button => Math.Abs(button.Width - 46) < .1 && Math.Abs(button.Height - 52) < .1),
            "A caption button is not 46 by 52.");
        var closed = 0;
        bar.CloseRequested += (_, _) => closed++;
        var close = Caption(bar, "Close");
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(close);
        ((IInvokeProvider)peer.GetPattern(PatternInterface.Invoke)!).Invoke();
        Check(closed == 1, "The close caption did not raise CloseRequested once.");

        var closeOnly = new UI.WindowTitlebar { Title = "About", Controls = UI.WindowControls.CloseOnly, Compact = true };
        shell.ShowFixture(closeOnly);
        await Task.Delay(60);
        shell.UpdateLayout();
        Check(Math.Abs(closeOnly.ActualHeight - 32) < .5, $"A compact titlebar measured {closeOnly.ActualHeight}px instead of 32px.");
        Check(Walk(closeOnly).OfType<Microsoft.UI.Xaml.Controls.Button>().Count() == 1,
            "CloseOnly did not leave exactly one caption control.");

        var none = new UI.WindowTitlebar { Title = "No chrome", Controls = UI.WindowControls.None };
        shell.ShowFixture(none);
        await Task.Delay(60);
        shell.UpdateLayout();
        Check(!Walk(none).OfType<Microsoft.UI.Xaml.Controls.Button>().Any(), "All-hidden controls drew a caption button.");

        var inactive = new UI.WindowTitlebar { Title = "Inactive", Inactive = true };
        shell.ShowFixture(inactive);
        await Task.Delay(60);
        shell.UpdateLayout();
        var title = Walk(inactive).OfType<TextBlock>().Single(text => text.Text == "Inactive");
        Check(((SolidColorBrush)title.Foreground).Color == inactive.Theme.Color("color.content-faint"),
            "An inactive titlebar did not ink its title with the faint colour.");

        var toolbar = new UI.WindowTitlebar { Title = "Toolbar", Toolbar = new TextBlock { Text = "Actions" }, Compact = true };
        shell.ShowFixture(toolbar);
        await Task.Delay(60);
        shell.UpdateLayout();
        Check(Math.Abs(toolbar.ActualHeight - 52) < .5, "A toolbar did not force the 52px band.");
    }
}
