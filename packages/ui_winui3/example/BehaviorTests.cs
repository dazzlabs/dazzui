using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using UI = DazzUI;

namespace DazzUI.Playground;

/// <summary>Exercise the public components in a real window, including their native automation contracts.</summary>
internal static class BehaviorTests
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
    private static T Pattern<T>(FrameworkElement element, PatternInterface pattern) where T : class
    {
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(element);
        var result = peer?.GetPattern(pattern);
        return result as T ?? throw new InvalidOperationException($"Missing {pattern} on {element.GetType().Name}: peer={peer?.GetType().FullName}, result={result?.GetType().FullName}, loaded={element.IsLoaded}, enabled={(element as Control)?.IsEnabled}.");
    }
    private static async Task Settle(PlaygroundShell shell)
    {
        await Task.Delay(80);
        shell.UpdateLayout();
        await new RenderTargetBitmap().RenderAsync(shell);
    }
    public static async Task RunAsync(PlaygroundShell shell, string output)
    {
        File.WriteAllText(Path.Combine(output, "progress.txt"), "Interactive controls, inherited themes and popup behavior");
        var checkbox = new UI.Checkbox { Content = "Keep formatting" };
        var radioA = new UI.Radio { Content = "Automatic", Tag = "auto", IsChecked = true };
        var radioB = new UI.Radio { Content = "Manual", Tag = "manual" };
        var group = new UI.RadioGroup(); group.Items.Add(radioA); group.Items.Add(radioB);
        var text = new UI.TextField { Width = 240 };
        var field = new UI.FormField { Label = "Name", Required = true, Field = text };
        var form = new UI.Form { Validate = () => string.IsNullOrWhiteSpace(text.Text) ? new Dictionary<string, string> { ["Name"] = "Enter a name" } : new Dictionary<string, string>() };
        form.Fields.Add(field);
        var otp = new UI.OtpField { Value = "12" };
        var range = new UI.RangeSlider { Width = 240 };
        var action = new UI.Button { Content = "Open" };
        var scope = new UI.ThemeScope { Theme = new("studio-light"), Content = new StackPanel { Spacing = 12, Children = { checkbox, group, form, otp, range, action } } };
        shell.ShowFixture(scope); await Settle(shell);

        Pattern<IToggleProvider>(checkbox, PatternInterface.Toggle).Toggle();
        Check(checkbox.IsChecked == true, "Checkbox automation did not toggle the checked state.");
        Pattern<ISelectionItemProvider>(radioB, PatternInterface.SelectionItem).Select();
        Check(radioB.IsChecked == true && radioA.IsChecked == false, "Radio group did not retain exclusive selection.");
        Check(!form.Submit() && field.Error is not null, "Form did not reject a missing required field.");
        await Settle(shell);
        // WinUI implements TextBox's Value/Text providers in native code, not
        // as projected Microsoft.UI.Xaml.Automation.Provider interfaces.
        // https://learn.microsoft.com/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.automation.peers.textboxautomationpeer
        Check(FrameworkElementAutomationPeer.CreatePeerForElement(text).GetAutomationControlType() == AutomationControlType.Edit, "TextField lost its native edit automation role.");
        text.Text = "Handbook";
        var submitted = false; form.Submitted += (_, _) => submitted = true;
        Check(form.Submit() && submitted && field.Error is null, "Form failed to submit valid input or clear its error.");

        var completions = 0; otp.Completed += (_, _) => completions++;
        var firstDigit = Walk(otp).OfType<UI.TextField>().First();
        firstDigit.Text = "987654";
        await Settle(shell);
        Check(otp.Value == "987654" && completions == 1, $"OTP paste did not distribute digits exactly once: value={otp.Value}, events={completions}.");
        Pattern<IRangeValueProvider>(range.Lower, PatternInterface.RangeValue).SetValue(25);
        Pattern<IRangeValueProvider>(range.Upper, PatternInterface.RangeValue).SetValue(75);
        Pattern<IRangeValueProvider>(range.Lower, PatternInterface.RangeValue).SetValue(90);
        Check(range.Lower.Value == 75 && range.Upper.Value == 75, "Range handles crossed each other.");
        range.Lower.Value = 25;
        Check(Math.Abs(Canvas.GetTop(range.Lower) - Canvas.GetTop(range.Upper)) < .1, "Range handles do not share one track.");

        VisualStateManager.GoToState(action, "PointerOver", false);
        scope.Theme = new("bright-dark"); await Settle(shell);
        Check(checkbox.IsChecked == true && text.Text == "Handbook" && otp.Value == "987654", "Changing theme reset editing or selection state.");
        Check(action.Theme.Name == "bright-dark", "A nested control did not inherit the new theme.");
        var presenter = Walk(action).OfType<ContentPresenter>().First();
        Check(((SolidColorBrush)presenter.Background).Color == action.Theme.Recipe(action.Variant, action.Tint, "surface", "hovered").Color, "A live theme change left the active button state stale.");
        await SmokeTest.SaveAsync(shell, output, "interactive.png");

        var menuInvoked = false;
        var menu = new UI.Menu(); menu.Items.Add(new("Run", () => menuInvoked = true));
        menu.Show(action); await Settle(shell);
        Check(menu.Flyout.IsOpen, "Menu did not open.");
        Pattern<IInvokeProvider>((FrameworkElement)menu.Flyout.Items[0], PatternInterface.Invoke).Invoke();
        await Settle(shell); Check(menuInvoked, "Menu action did not invoke."); menu.Flyout.Hide();
        var popover = new UI.Popover { Content = new UI.TextField { PlaceholderText = "Rename" } };
        popover.Show(action); await Settle(shell);
        Check(popover.IsOpen && UI.Appearance.GetTheme(popover.Content!).Name == "bright-dark", "Popover did not open with its anchor's theme.");
        popover.Hide(); await Settle(shell);

        var dialog = new UI.Dialog { Title = "Confirm export", Content = new TextBlock { Text = "Export the document?" } };
        var shown = dialog.ShowAsync(action); await Settle(shell);
        var primary = Walk(dialog.Native).OfType<Microsoft.UI.Xaml.Controls.Button>().FirstOrDefault(button => button.Name == "PrimaryButton");
        Check(primary is not null, "Dialog primary action was not rendered.");
        await SmokeTest.SaveAsync(dialog.Native, output, "dialog.png");
        Pattern<IInvokeProvider>(primary!, PatternInterface.Invoke).Invoke();
        Check(await shown == ContentDialogResult.Primary, "Dialog did not return its primary action result.");
        await Settle(shell);
        foreach (var side in Enum.GetValues<UI.DrawerSide>())
        {
            var drawer = new UI.Drawer { Side = side, Title = "Settings", Content = new UI.Checkbox { Content = "Remember this choice" } };
            var opened = drawer.ShowAsync(action); await Settle(shell);
            var panel = Walk(drawer.Native).OfType<FrameworkElement>().First(element => element.Name == "BackgroundElement");
            var size = action.XamlRoot.Size;
            Check(side is UI.DrawerSide.Left or UI.DrawerSide.Right ? Math.Abs(panel.ActualHeight - size.Height) < 1 : Math.Abs(panel.ActualWidth - size.Width) < 1, $"{side} drawer did not span its window edge.");
            await SmokeTest.SaveAsync(drawer.Native, output, $"drawer-{side}.png");
            drawer.Hide(); await opened; await Settle(shell);
        }
        var masked = new UI.OtpField { Masked = true, Value = "654321" };
        shell.ShowFixture(masked); await Settle(shell);
        Check(Walk(masked).OfType<PasswordBox>().Count() == 6 && masked.Value == "654321", "Masked OTP did not use native password inputs.");
        await File.WriteAllTextAsync(Path.Combine(output, "behavior.txt"), "PASS: checkbox, exclusive radio selection, form validation/submission, native edit role, OTP paste/masking, range automation, theme/state preservation, menu action, popover, dialog primary result, four drawer edges.\n");
    }
}
