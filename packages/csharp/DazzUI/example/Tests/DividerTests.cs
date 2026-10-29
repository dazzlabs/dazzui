using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class DividerTests
{
    public static async Task RunAsync(Shell shell)
    {
        var horizontal = new UI.Divider();
        var vertical = new UI.Divider { Orientation = Orientation.Vertical };
        var grid = new Grid { Width = 260, Height = 100 };
        grid.Children.Add(horizontal); grid.Children.Add(vertical);
        shell.ShowFixture(grid); await Task.Delay(100); shell.UpdateLayout();
        if (horizontal.Orientation != Orientation.Horizontal || horizontal.ActualWidth < 200 || horizontal.ActualHeight > 2)
            throw new InvalidOperationException("The default divider is not a horizontal hairline.");
        if (vertical.ActualWidth > 2 || vertical.ActualHeight < 90)
            throw new InvalidOperationException("The explicitly vertical divider did not stretch vertically.");
    }
}
