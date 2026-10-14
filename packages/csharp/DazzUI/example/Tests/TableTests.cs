using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class TableTests
{
    private static void Check(bool value, string message)
    {
        if (!value) throw new InvalidOperationException(message);
    }
    private static async Task Settle(Shell shell)
    {
        await Task.Delay(80);
        shell.UpdateLayout();
    }
    private static T Pattern<T>(FrameworkElement element, PatternInterface pattern) where T : class =>
        FrameworkElementAutomationPeer.CreatePeerForElement(element).GetPattern(pattern) as T
            ?? throw new InvalidOperationException($"Missing table {pattern} pattern.");

    internal static async Task RunAsync(Shell shell, string output)
    {
        var table = new UI.Table { Width = 320, Tint = UI.WidgetTint.Danger };
        var head = new UI.TableHead();
        var serviceHead = new UI.TableCell { Head = true, Content = "Service" };
        head.Children.Add(serviceHead);
        head.Children.Add(new UI.TableCell { Head = true, Align = TextAlignment.Right, Content = "Status" });
        var badge = new UI.Badge { Content = "Live", Tint = UI.WidgetTint.Success };
        var row = new UI.TableRow();
        var service = new UI.TableCell { Content = "web" };
        var status = new UI.TableCell { Align = TextAlignment.Right, Content = badge };
        row.Children.Add(service);
        row.Children.Add(status);
        table.Children.Add(head);
        table.Children.Add(row);
        shell.ShowFixture(table);
        await Settle(shell);

        Check(Math.Abs(head.ActualWidth - table.ActualWidth) < 0.5 && Math.Abs(row.ActualWidth - table.ActualWidth) < 0.5,
            "Table bands do not span the table.");
        Check(head.CornerRadius == new CornerRadius(0) && row.CornerRadius == new CornerRadius(0),
            "Table bands gained a control's rounded corners.");
        Check(Math.Abs(service.ActualWidth - status.ActualWidth) < 0.5,
            "Table columns are not equal shares.");
        Check(Math.Abs(service.ActualWidth * 2 + row.Padding.Left + row.Padding.Right - table.ActualWidth) < 0.5,
            "Table inserted a column gap that React does not have.");
        Check(ReferenceEquals(status.Content, badge), "Table flattened rich cell content.");
        var badgeWords = (TextBlock)badge.Content;
        if (badge.FontFamily.Source == "Microsoft YaHei UI")
        {
            Check(badge.FontWeight.Weight == 600 && badgeWords.FontWeight.Weight == 700,
                "The requested semibold label did not resolve to the family's bold face as in CSS.");
            Check(Math.Abs(status.ActualHeight - 22.39) < 0.1
                && Math.Abs(badge.TransformToVisual(status).TransformPoint(default).Y - 1) < 0.1,
                "The rich cell lost the browser's inline baseline and font strut.");
        }
        var sample = (TextBlock)serviceHead.Content;
        await File.WriteAllTextAsync(Path.Combine(output, "table-layout.txt"),
            $"Cell font: {serviceHead.FontFamily.Source}; text font: {sample.FontFamily.Source}; heading baseline: {sample.BaselineOffset}; heading height: {sample.ActualHeight}\n" +
            $"Body font: {service.FontFamily.Source}; body baseline: {((TextBlock)service.Content).BaselineOffset}; body height: {service.ActualHeight}; badge height: {badge.ActualHeight}\n" +
            $"Badge text: size={badgeWords.FontSize}, weight={badgeWords.FontWeight.Weight}, baseline={badgeWords.BaselineOffset}, height={badgeWords.ActualHeight}, y={badgeWords.TransformToVisual(badge).TransformPoint(default).Y}, lineHeight={badgeWords.LineHeight}\n");
        Check(!row.IsTabStop && FrameworkElementAutomationPeer.CreatePeerForElement(row).GetPattern(PatternInterface.Invoke) is null,
            "An inert table row became an action.");
        var grid = Pattern<IGridProvider>(table, PatternInterface.Grid);
        Check(grid.RowCount == 1 && grid.ColumnCount == 2 && grid.GetItem(0, 1) is not null,
            "Table automation lost its row and column coordinates.");
        Check(Pattern<ITableProvider>(table, PatternInterface.Table).GetColumnHeaders().Length == 2,
            "Table automation lost its column headers.");
        Check(FrameworkElementAutomationPeer.CreatePeerForElement(serviceHead).GetName() == "Service"
            && FrameworkElementAutomationPeer.CreatePeerForElement(status).GetName() == "Live",
            "Table automation lost the words in a header or rich cell.");
        var cell = Pattern<IGridItemProvider>(status, PatternInterface.GridItem);
        Check(cell.Row == 0 && cell.Column == 1 && cell.ContainingGrid is not null,
            "A cell does not identify its containing table.");
        Check(Pattern<ITableItemProvider>(status, PatternInterface.TableItem).GetColumnHeaderItems().Length == 1,
            "A body cell does not identify its column header.");

        var clicks = 0;
        row.Click += (_, _) => clicks++;
        row.Interactive = true;
        await Settle(shell);
        Check(row.IsTabStop, "An interactive row is not keyboard reachable.");
        Pattern<IInvokeProvider>(row, PatternInterface.Invoke).Invoke();
        await Settle(shell);
        Check(clicks == 1 && !row.Active, "Pressing a row toggled caller-owned selection or fired twice.");
        Pattern<ISelectionItemProvider>(row, PatternInterface.SelectionItem).Select();
        await Settle(shell);
        Check(clicks == 2 && !row.Active, "A selection request did not reach the caller or changed controlled state itself.");
        row.Active = true;
        await Settle(shell);
        Check(Pattern<ISelectionItemProvider>(row, PatternInterface.SelectionItem).IsSelected
            && Pattern<ISelectionProvider>(table, PatternInterface.Selection).GetSelection().Length == 1,
            "The controlled active row is not exposed as selected to automation.");
        var selected = (SolidColorBrush)row.Background;
        var expected = table.Theme.Alpha("color.danger.600", table.Theme.Number("wash.surface"));
        Check(selected.Color == expected.Color, "An active table row did not inherit the table's tint.");

        table.Theme = new UI.Theme("bright-dark");
        await Settle(shell);
        Check(row.Active && ReferenceEquals(status.Content, badge) && row.Theme.Name == "bright-dark",
            "A table theme change replaced content or lost controlled state.");
        var extra = new UI.TableRow { Interactive = true };
        extra.Children.Add(new UI.TableCell { Content = "api" });
        extra.Children.Add(new UI.TableCell { Content = "Building" });
        table.Children.Add(extra);
        await Settle(shell);
        Check(grid.RowCount == 2 && extra.ActualWidth > 0, "Rows added after load did not enter layout and automation.");
        table.Children.Remove(extra);
        row.Interactive = false;
        await Settle(shell);
        var inert = FrameworkElementAutomationPeer.CreatePeerForElement(row);
        Check(grid.RowCount == 1 && !row.IsTabStop && inert.GetPattern(PatternInterface.Invoke) is null
            && inert.GetPattern(PatternInterface.SelectionItem) is null
            && Pattern<ISelectionProvider>(table, PatternInterface.Selection).GetSelection().Length == 0,
            "Removing a row or its interactive state did not update automation.");
        Check(row.Active, "Disabling interaction changed the caller's active styling.");
        status.Content = new Grid { Height = 5, Width = 8 };
        await Settle(shell);
        Check(Math.Abs(status.ActualHeight - 5) < 0.1, "A block cell acquired an inline font strut.");
        status.Content = null;
        await Settle(shell);
        Check(status.ActualHeight == 0, "Removing cell content retained an old inline line box.");
        status.Content = badge;
        await Settle(shell);
        Check(status.ActualHeight > badge.ActualHeight, "Restoring inline content did not restore its font strut.");
        service.Content = "A longer sentence with several words";
        table.Width = 160;
        await Settle(shell);
        Check(service.ActualHeight > table.Theme.Text("", "bodyMedium").LineHeight,
            "A narrow table cell clipped its text instead of wrapping it.");
    }
}
