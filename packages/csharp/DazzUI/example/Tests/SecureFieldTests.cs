using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using System.Runtime.InteropServices;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class SecureFieldTests
{
    [DllImport("user32.dll")]
    private static extern void keybd_event(byte key, byte scan, uint flags, UIntPtr extraInfo);

    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }

    public static async Task RunAsync(Shell shell)
    {
        var field = new UI.SecureTextField { Password = "initial-secret", PlaceholderText = "API key", Width = 280 };
        var form = new UI.FormField { Label = "API key", Required = true, Field = field };
        var changes = 0;
        field.PasswordChanged += (_, _) => changes++;
        shell.ShowFixture(form);
        await Task.Delay(100); shell.UpdateLayout();
        Check(field.Input.PasswordRevealMode == PasswordRevealMode.Hidden && field.Input.Password == "initial-secret", "The native input did not retain its masked value.");
        Check(AutomationProperties.GetLabeledBy(field.Input) is not null && AutomationProperties.GetIsRequiredForForm(field.Input), "The native password input lost its form label or required state.");
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(field.Input);
        Check(peer?.IsPassword() == true, "The native input did not expose the password automation role.");
        var exposedValue = "";
        try { exposedValue = (peer?.GetPattern(PatternInterface.Value) as IValueProvider)?.Value ?? ""; }
        catch (Exception error) when (error is InvalidOperationException or UnauthorizedAccessException or COMException) { }
        Check(!exposedValue.Contains("initial-secret", StringComparison.Ordinal), "Password text leaked through the UI Automation value pattern.");
        field.Input.Password = "changed-secret";
        for (var attempt = 0; attempt < 40 && changes == 0; attempt++) await Task.Delay(25);
        Check(field.Password == "changed-secret" && changes > 0, $"Editing the native password did not update the component (value matches: {field.Password == "changed-secret"}, events: {changes}).");
        field.Input.Password = "";
        Check(field.Input.Focus(Microsoft.UI.Xaml.FocusState.Programmatic), "The secure input could not receive keyboard focus.");
        await Task.Delay(100);
        keybd_event(0x41, 0, 0, UIntPtr.Zero); keybd_event(0x41, 0, 2, UIntPtr.Zero);
        for (var attempt = 0; attempt < 40 && field.Password.Length == 0; attempt++) await Task.Delay(25);
        Check(field.Password.Length == 1, "The secure input did not accept a real keyboard event.");
        field.Password = "changed-secret";
        field.Invalid = true; field.Theme = new UI.Theme("studio-dark"); await Task.Delay(50); shell.UpdateLayout();
        Check(field.Password == "changed-secret" && ((SolidColorBrush)field.Input.BorderBrush).Color == field.Theme.Color("color.danger.600"), "Theme or validation changes lost the value or error styling.");
        field.IsEnabled = false; await Task.Delay(50);
        Check(!field.Input.IsEnabled, "Disabled state did not reach the native editor.");
    }
}
