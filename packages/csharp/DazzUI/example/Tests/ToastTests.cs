using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class ToastTests
{
    public static async Task RunAsync(Shell shell)
    {
        var viewport = new UI.ToastViewport { HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
        shell.ShowFixture(viewport);
        var timed = viewport.Add(new UI.ToastOptions("Copied", Tint: UI.WidgetTint.Success, Duration: TimeSpan.FromMilliseconds(500)));
        await Task.Delay(100); shell.UpdateLayout();
        if (timed.Content is not Grid row || row.Children.Count != 1 || row.Children[0] is not TextBlock)
            throw new InvalidOperationException("A timed notice must be a single line without a dismiss control.");
        if (timed.ActualHeight > 45 || timed.ActualWidth <= 0) throw new InvalidOperationException("The timed notice does not use compact toast geometry.");
        await Task.Delay(600);
        if (((StackPanel)viewport.Content).Children.Contains(timed)) throw new InvalidOperationException("The timed notice did not expire.");
        var persistent = viewport.Add(new UI.ToastOptions("Failed", Tint: UI.WidgetTint.Danger, DismissLabel: "Close notice"));
        await Task.Delay(100); shell.UpdateLayout();
        if (persistent.Content is not Grid danger || !danger.Children.OfType<UI.IconButton>().Any(button => button.Label == "Close notice"))
            throw new InvalidOperationException("A persistent danger notice must expose its localized dismiss action.");
        viewport.Remove(persistent);
        var invoked = false;
        var longNotice = viewport.Add(new UI.ToastOptions(new string('W', 200), ActionLabel: "Undo", Action: () => invoked = true, Duration: TimeSpan.Zero));
        await Task.Delay(100); shell.UpdateLayout();
        var longRow = (Grid)longNotice.Content;
        var label = longRow.Children.OfType<TextBlock>().Single();
        var action = longRow.Children.OfType<UI.Button>().First(button => button is not UI.IconButton);
        if (longNotice.ActualWidth > longNotice.MaxWidth + 1 || label.ActualWidth <= 0 || label.ActualWidth >= longNotice.MaxWidth || action.ActualWidth <= 0 || label.TextTrimming != TextTrimming.CharacterEllipsis)
            throw new InvalidOperationException("A long toast must constrain its message while preserving its action.");
        ((Microsoft.UI.Xaml.Automation.Provider.IInvokeProvider)new Microsoft.UI.Xaml.Automation.Peers.ButtonAutomationPeer(action).GetPattern(Microsoft.UI.Xaml.Automation.Peers.PatternInterface.Invoke)).Invoke();
        if (!invoked) throw new InvalidOperationException("The toast action did not execute.");
        UI.Appearance.SetTheme(viewport, new UI.Theme("studio-dark")); await Task.Delay(100); shell.UpdateLayout();
        if (longNotice.Options.Title.Length != 200 || longNotice.Content is not Grid updated || !updated.Children.OfType<UI.Button>().Any(button => button is not UI.IconButton))
            throw new InvalidOperationException("Changing theme lost the toast's content or action.");
        viewport.Remove(longNotice);
    }
}
