using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Windows.System;

namespace DazzUI.Example;

/// <summary>Presentation details and live states absent from initial-scene readings.</summary>
internal static class PresentationParityTests
{
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }

    internal static async Task RunAsync(Shell shell)
    {
        static void Check(bool pass, string message)
        {
            if (!pass) throw new InvalidOperationException(message);
        }

        var action = new Button { Content = "Continue" };
        var glyph = new IconButton { Label = "Settings", Glyph = FluentIcons.Common.Icon.Settings };
        var avatar = new Avatar { NameText = "  Ada\tByron\nLovelace  " };
        var empty = new Avatar { NameText = " \t " };
        var single = new Avatar { NameText = "Ada" };
        var plain = new CodeBlock { Text = "first\nsecond\n\n", LineNumbers = true };
        var copyable = new CodeBlock { Text = "copied text\n\n", Copyable = true };
        var applied = new List<double>();
        // This fixture checks keyboard focus independently of where the user's
        // mouse happens to rest over the native test window.
        var resizer = new Resizer(1, new ColumnWidth(272, 200, 420), applied.Add, "Resize list") { Height = 32, IsHitTestVisible = false };
        var reverse = new Resizer(-1, new ColumnWidth(336, 280, 560), _ => { }, "Resize themes") { Height = 32, IsHitTestVisible = false };
        var panel = new StackPanel { Spacing = 12, Children = { action, glyph, avatar, empty, single, plain, copyable, resizer, reverse } };
        var scope = new ThemeScope { Theme = new Theme("bright-light"), Content = panel, Width = 360 };
        shell.ShowFixture(scope);
        await Task.Delay(100); shell.UpdateLayout();

        Check(Walk(avatar).OfType<TextBlock>().Any(text => text.Text == "AL"), "Avatar initials must use the first and last whitespace-delimited words.");
        Check(Walk(empty).OfType<TextBlock>().All(text => text.Text.Length == 0), "Whitespace-only avatar names must have no initials.");
        Check(Walk(single).OfType<TextBlock>().Any(text => text.Text == "A"), "A one-word avatar must use one initial.");
        Check(((StackPanel)plain.Content).Children.Count == 1 && !Walk(plain).OfType<IconButton>().Any(), "A code block must not add a copy header by default.");
        Check(Walk(plain).OfType<TextBlock>().Any(text => text.Text == "first\nsecond") && Walk(plain).OfType<TextBlock>().Any(text => text.Text == "1\n2"), "Code and line numbers must omit trailing blank lines.");

        var copy = Walk(copyable).OfType<IconButton>().Single();
        copyable.Copy(_ => throw new InvalidOperationException("Clipboard unavailable"));
        Check(copy.Label == "Copy", "A refused clipboard write must not report success.");
        string? offered = null;
        copyable.Copy(text => offered = text);
        Check(offered == "copied text", "Copy must offer exactly the displayed source without trailing newlines.");
        Check(copy.Label == "Copied" && AutomationProperties.GetName(copy) == "Copied" && copy.Content is Icon { Glyph: FluentIcons.Common.Icon.Checkmark }, "Successful copying must update the visible glyph and accessible label.");

        foreach (var name in new[] { "studio-dark", "bright-light" })
        {
            scope.Theme = new Theme(name);
            await Task.Delay(80); shell.UpdateLayout();
            var theme = scope.Theme;
            foreach (var button in new Button[] { action, glyph })
                Check(button.FocusVisualMargin == new Thickness(theme.Number("focus.offset")), $"{name}: button focus ring lost its shared offset.");
            Check(copy.Label == "Copied" && ReferenceEquals(Walk(copyable).OfType<IconButton>().Single(), copy), $"{name}: a live theme change replaced the copy action or its feedback.");
            var line = resizer.Children.OfType<Border>().Single();
            Check((line.Background as SolidColorBrush)?.Color == theme.Color("color.primary.500"), $"{name}: resizer retained the previous theme's accent.");
        }

        await Task.Delay((int)scope.Theme.Milliseconds("code-block.copy-feedback-duration") + 80);
        Check(copy.Label == "Copy" && copy.Content is Icon { Glyph: FluentIcons.Common.Icon.Copy }, "Copy feedback must return to its resting state after the shared duration.");
        copyable.Copy(_ => { });
        panel.Children.Remove(copyable);
        await Task.Delay(30);
        panel.Children.Add(copyable);
        await Task.Delay(60); shell.UpdateLayout();
        Check(Walk(copyable).OfType<IconButton>().Single().Label == "Copy", "Unmounting must cancel pending copy feedback.");

        Check(resizer.ResizeKey(VirtualKey.Right, false) && resizer.Value == 280, "Resize arrow must move eight logical pixels.");
        Check(resizer.ResizeKey(VirtualKey.Right, true) && resizer.Value == 312, "Shift resize arrow must move thirty-two logical pixels.");
        Check(resizer.ResizeKey(VirtualKey.Home, false) && resizer.Value == 200, "Resize Home must reach the minimum.");
        Check(resizer.ResizeKey(VirtualKey.End, true) && resizer.Value == 420, "Resize End must reach the maximum.");
        resizer.ResizeKey(VirtualKey.Right, true);
        Check(resizer.Value == 420 && applied[^1] == 420 && !resizer.ResizeKey(VirtualKey.A, false), "Resize keys must clamp and leave unrelated commands untouched.");
        reverse.ResizeKey(VirtualKey.Right, true);
        Check(reverse.Value == 304, "The trailing column must reverse arrow direction.");
        reverse.ResizeKey(VirtualKey.Home, false);
        Check(reverse.Value == 280, "Home must select the trailing column minimum independently of direction.");

        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(resizer);
        Check(peer.GetAutomationControlType() == AutomationControlType.Separator && peer.GetOrientation() == AutomationOrientation.Vertical, "A resizer must expose a vertical separator role.");
        var range = (IRangeValueProvider)peer.GetPattern(PatternInterface.RangeValue);
        range.SetValue(310);
        Check(range.Value == 310 && range.Minimum == 200 && range.Maximum == 420 && range.SmallChange == 8 && range.LargeChange == 32, "Resizer automation must report and change its live range.");
        var focused = await FocusManager.TryFocusAsync(resizer, FocusState.Keyboard);
        Check(focused.Succeeded && ReferenceEquals(FocusManager.GetFocusedElement(resizer.XamlRoot), resizer), "Native keyboard focus must reach the resizer.");
        Check(resizer.Children.OfType<Border>().Single().Opacity == 1, "Keyboard focus must reveal the resize rule.");
        var moved = await FocusManager.TryFocusAsync(action, FocusState.Programmatic);
        // LostFocus is posted after the focus operation, unlike GettingFocus.
        await Task.Delay(40);
        Check(moved.Succeeded && ReferenceEquals(FocusManager.GetFocusedElement(action.XamlRoot), action), "Native focus must leave the resizer for the action.");
        Check(resizer.Children.OfType<Border>().Single().Opacity == 0, "Leaving keyboard focus must hide an idle resize rule.");
    }
}
