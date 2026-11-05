using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class SearchFieldTests
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
        await Task.Delay(80);
        shell.UpdateLayout();
    }

    private static async Task Escape(Shell shell)
    {
        NativeKeyboard.Press(shell, 0x1B);
        await Settle(shell);
    }

    public static async Task RunAsync(Shell shell)
    {
        var search = new UI.SearchField { Width = 320, Text = "matching query", Theme = new UI.Theme("bright-light") };
        var after = new Microsoft.UI.Xaml.Controls.Button { Content = "After search" };
        var host = new StackPanel { Children = { search, after } };
        var dismissals = 0;
        var escaped = 0;
        search.Dismissed += (_, _) => dismissals++;
        host.KeyDown += (_, args) => { if (args.Key == Windows.System.VirtualKey.Escape) escaped++; };
        shell.ShowFixture(host);
        await Settle(shell);
        await NativeKeyboard.ActivateAsync(shell);
        Check(search.Focus(FocusState.Programmatic), "Search could not receive keyboard focus.");
        await Settle(shell);
        await Escape(shell);
        Check(search.Text == "" && dismissals == 0,
            $"Escape must clear a populated search without dismissing it (text='{search.Text}', dismissals={dismissals}, escaped={escaped}).");
        Check(ReferenceEquals(FocusManager.GetFocusedElement(search.XamlRoot), search),
            "Clearing the search with Escape moved native focus.");
        await Escape(shell);
        Check(dismissals == 1 && escaped == 0,
            $"Escape on an empty search must dismiss once and remain handled (dismissals={dismissals}, escaped={escaped}).");

        var clear = Walk(search).OfType<Microsoft.UI.Xaml.Controls.Button>()
            .Single(button => AutomationProperties.GetName(button) == "Clear search");
        var clearIcon = (UI.Icon)clear.Content;
        Check(clear.Visibility == Visibility.Collapsed, "An empty search retained its clear action.");

        var keycap = Walk(search).OfType<UI.KeyCap>().Single();
        Check(keycap.Visibility == Visibility.Collapsed,
            "A search without a hint rendered an empty key cap.");
        search.Hint = "Ctrl+Shift+F";
        await Settle(shell);
        Check(keycap.Visibility == Visibility.Visible && Walk(keycap).OfType<TextBlock>().Any(text => text.Text == search.Hint),
            "A hint added after the native template loaded was not displayed.");
        Check(search.Padding.Right >= keycap.ActualWidth + search.Theme.Number("spacing.3") + search.Theme.Number("control.medium.gap") - .5,
            "The search editor did not reserve the full shortcut hint width.");
        search.Text = "query";
        await Settle(shell);
        Check(keycap.Visibility == Visibility.Collapsed && Math.Abs(search.Padding.Right - search.Theme.Number("spacing.3") - search.Theme.Number("icon.small") - search.Theme.Number("control.medium.gap")) < .01,
            "A populated search must replace the hint inset with its clear glyph and gap.");
        Check(after.Focus(FocusState.Programmatic), "The following control could not receive focus.");
        await Settle(shell);
        Check(clear.Visibility == Visibility.Visible && ReferenceEquals(FocusManager.GetFocusedElement(search.XamlRoot), after),
            "A populated search lost its clear action when the editor blurred.");
        // Temporarily isolate theme ink from the physical pointer. Restore
        // hit testing before inspecting the button's interactive UIA state.
        clear.IsHitTestVisible = false;
        foreach (var name in new[] { "bright-light", "studio-dark", "graphite-light" })
        {
            search.Theme = new UI.Theme(name);
            await Settle(shell);
            var iconSize = search.Theme.Number("icon.small");
            Check(Math.Abs(clear.ActualWidth - iconSize) < .01 && Math.Abs(clear.ActualHeight - iconSize) < .01
                && Math.Abs(clearIcon.ActualWidth - iconSize) < .01 && clearIcon.Glyph == FluentIcons.Common.Icon.Dismiss,
                $"{name}: the search clear action must use the unpadded icon.small dismiss glyph.");
            Check(clear.Foreground is SolidColorBrush ink && ink.Color == search.Theme.Color("color.content-faint")
                && clearIcon.Foreground is SolidColorBrush glyphInk && glyphInk.Color == ink.Color,
                $"{name}: the search clear action lost the faint theme ink.");
            Check(clear.UseSystemFocusVisuals && clear.FocusVisualPrimaryThickness == new Thickness(search.Theme.Number("focus.width"))
                && clear.FocusVisualMargin == new Thickness(search.Theme.Number("focus.offset")),
                $"{name}: the search clear action lost its token focus ring.");
        }
        clear.IsHitTestVisible = true;
        await Settle(shell);
        var clearPeer = FrameworkElementAutomationPeer.CreatePeerForElement(clear);
        var pattern = clearPeer?.GetPattern(PatternInterface.Invoke);
        Check(clearPeer?.GetAutomationControlType() == AutomationControlType.Button && pattern is IInvokeProvider,
            $"The visible search clear action must retain native button automation and Invoke (peer={clearPeer?.GetType().FullName ?? "null"}, class={clearPeer?.GetClassName() ?? "null"}, pattern={pattern?.GetType().FullName ?? "null"}, visible={clear.Visibility}, loaded={clear.IsLoaded}, hitTest={clear.IsHitTestVisible}, enabled={clear.IsEnabled}).");
        var invoke = (IInvokeProvider)pattern!;
        search.ClearLabel = "Clear results";
        Check(clearPeer!.GetName() == "Clear results", "Changing ClearLabel did not update the native accessible name.");
        Check(clear.Focus(FocusState.Keyboard), "The native clear action could not receive keyboard focus.");
        invoke.Invoke();
        await Settle(shell);
        Check(search.Text == "" && clear.Visibility == Visibility.Collapsed && keycap.Visibility == Visibility.Visible
            && ReferenceEquals(FocusManager.GetFocusedElement(search.XamlRoot), search),
            "Invoking clear must empty the query, restore the hint and return focus to the editor.");
        Check(dismissals == 1, "Invoking clear must not dismiss the search.");
        search.Text = "disabled query";
        search.IsEnabled = false;
        await Settle(shell);
        Check(!clear.IsEnabled && !clearPeer.IsEnabled(), "Disabling search left the clear action enabled.");
        try { invoke.Invoke(); }
        catch (Exception) when (!clear.IsEnabled) { /* Native automation may reject disabled invocation. */ }
        await Settle(shell);
        Check(search.Text == "disabled query", "Disabled search allowed its query to be cleared.");
        search.IsEnabled = true;
        search.Text = "";
        search.Theme = new UI.Theme("bright-dark");
        await Settle(shell);
        Check(keycap.Visibility == Visibility.Visible && Walk(keycap).OfType<TextBlock>().Any(text => text.Text == search.Hint),
            "Clearing a search or changing theme lost its hint.");
        search.Hint = null;
        await Settle(shell);
        Check(keycap.Visibility == Visibility.Collapsed && Math.Abs(search.Padding.Right - search.Theme.Number("spacing.3")) < .01,
            "Removing a live search hint left a key cap or extra inset.");
    }
}
