using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Example;

/// <summary>A rail and a group must measure complete before they are attached, and on every row change.</summary>
internal static class SidebarTests
{
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }

    public static async Task RunAsync(Shell shell)
    {
        var group = new UI.SidebarGroup { Label = "Workspace" };
        group.Items.Add(new UI.NavItem { Content = "Translate" });
        group.Items.Add(new UI.NavItem { Content = "History" });
        group.Prepare();
        shell.ShowFixture(group);
        await Task.Delay(80);
        shell.UpdateLayout();
        Check(group.Content is StackPanel panel && panel.Children.OfType<UI.SectionLabel>().Any(),
            "A sidebar group did not build its section heading for its first measure.");
        var first = group.ActualHeight;
        Check(first > 2 * 28, $"A sidebar group measured {first}px; its heading and gaps were missing.");
        DazzUI.Appearance.Refresh(group);
        shell.UpdateLayout();
        Check(Math.Abs(group.ActualHeight - first) < 1,
            $"A sidebar group changed height from {first}px to {group.ActualHeight}px after a theme refresh.");

        group.Items.Clear();
        group.Items.Add(new UI.NavItem { Content = "Glossaries" });
        shell.UpdateLayout();
        Check(group.ActualHeight > 0, "Clearing and refilling a sidebar group left it with no height.");

        var rail = new UI.Rail();
        rail.Items.Add(new UI.NavItem { Content = "One" });
        rail.Prepare();
        shell.ShowFixture(rail);
        await Task.Delay(80);
        shell.UpdateLayout();
        var before = rail.ActualHeight;
        Check(before > 0, "A rail measured no height before its first theme pass.");

        rail.Items.Clear();
        rail.Items.Add(new UI.NavItem { Content = "Two" });
        shell.UpdateLayout();
        Check(rail.ActualHeight > 0 && Math.Abs(rail.ActualHeight - before) < 1,
            $"Replacing a rail's rows changed its height from {before}px to {rail.ActualHeight}px.");
    }
}
