using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI;

public sealed record TreeNode(string Id, string Label, IReadOnlyList<TreeNode>? Children = null, string? Meta = null, bool Expanded = true, bool Disabled = false)
{
    public override string ToString() => Meta is null ? Label : $"{Label}   {Meta}";
}
public sealed class Tree : Surface
{
    private double itemFontSize;
    public TreeView View { get; } = new() { SelectionMode = TreeViewSelectionMode.Single, CanDragItems = false, CanReorderItems = false };
    public IList<TreeNode> Nodes { get; } = new List<TreeNode>();
    public TreeNode? SelectedNode => View.SelectedNode?.Content as TreeNode;
    public event EventHandler<TreeNode>? Selected;
    public Tree()
    {
        Content = View;
        View.SelectionChanged += (_, _) => { if (SelectedNode is not null) Selected?.Invoke(this, SelectedNode); };
        View.ItemInvoked += (_, args) => { if (args.InvokedItem is TreeViewNode node && node.Content is TreeNode { Disabled: true }) View.SelectedNode = null; };
        View.LayoutUpdated += (_, _) => UpdateContainers(View.RootNodes);
    }
    private void UpdateContainers(IEnumerable<TreeViewNode> nodes)
    {
        foreach (var node in nodes)
        {
            if (View.ContainerFromNode(node) is TreeViewItem item && node.Content is TreeNode model)
            {
                if (item.IsEnabled == model.Disabled) item.IsEnabled = !model.Disabled;
                if (itemFontSize > 0 && item.FontSize != itemFontSize) item.FontSize = itemFontSize;
            }
            if (node.IsExpanded) UpdateContainers(node.Children);
        }
    }
    protected override void RefreshAppearance()
    {
        View.Foreground = Theme.Brush("color.content");
        View.Background = null;
        Theme.Typography(View, "bodySmall");
        itemFontSize = View.FontSize;
        View.Resources["TreeViewItemMinHeight"] = Theme.Number($"control.{Theme.Key(Size)}.size");
        View.Resources["TreeViewItemContentHeight"] = Theme.Number($"control.{Theme.Key(Size)}.size");
        View.Resources["TreeViewItemPresenterPadding"] = new Thickness(0);
        View.Resources["TreeViewItemBackgroundSelected"] = Theme.Recipe(ControlVariant.Tinted, Tint, "surface");
        View.Resources["TreeViewItemForegroundSelected"] = Theme.Recipe(ControlVariant.Tinted, Tint, "content");
        if (View.RootNodes.Count == 0) foreach (var node in Nodes) View.RootNodes.Add(Build(node));
    }
    private static TreeViewNode Build(TreeNode node)
    {
        var result = new TreeViewNode { Content = node, IsExpanded = node.Expanded };
        foreach (var child in node.Children ?? []) result.Children.Add(Build(child));
        return result;
    }
}
public sealed record TableColumn(string Key, string Label, double Weight = 1, TextAlignment Alignment = TextAlignment.Left);
public sealed record TableRow(string Id, IReadOnlyDictionary<string, object?> Cells);
public sealed class Table : Surface
{
    public IList<TableColumn> Columns { get; } = new List<TableColumn>();
    public IList<TableRow> Rows { get; } = new List<TableRow>();
    public ListView View { get; } = new();
    public bool Selectable { get; set; } = true;
    public bool Multiple { get; set; }
    public IReadOnlyList<string> SelectedIds => View.SelectedItems.OfType<ListViewItem>().Select(item => ((TableRow)item.Tag).Id).ToArray();
    protected override void RefreshAppearance()
    {
        View.SelectionMode = !Selectable ? ListViewSelectionMode.None : Multiple ? ListViewSelectionMode.Multiple : ListViewSelectionMode.Single;
        View.Foreground = Theme.Brush("color.content"); View.Background = Theme.Brush("color.surface");
        View.Resources["ListViewItemBackgroundSelected"] = Theme.Recipe(ControlVariant.Tinted, Tint, "surface");
        View.Resources["ListViewItemForegroundSelected"] = Theme.Recipe(ControlVariant.Tinted, Tint, "content");
        var selection = SelectedIds;
        View.Items.Clear();
        View.Header = MakeRow(null);
        foreach (var row in Rows)
        {
            var item = new ListViewItem { Content = MakeRow(row), Tag = row, HorizontalContentAlignment = HorizontalAlignment.Stretch, Padding = new Thickness(0) };
            View.Items.Add(item);
            if (selection.Contains(row.Id)) item.IsSelected = true;
        }
        Content = View;
    }
    private Grid MakeRow(TableRow? row)
    {
        var grid = new Grid { ColumnSpacing = Theme.Number("spacing.4"), Padding = new Thickness(Theme.Number("spacing.3")) };
        foreach (var (column, index) in Columns.Select((column, index) => (column, index)))
        {
            grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(column.Weight, GridUnitType.Star) });
            var value = row is null ? column.Label : row.Cells.GetValueOrDefault(column.Key);
            var cell = value as UIElement ?? Theme.Text(value?.ToString() ?? "", row is null ? "labelSmall" : "bodyMedium");
            if (cell is TextBlock text) text.TextAlignment = column.Alignment;
            Skin.Detach(cell); cell.SetValue(Grid.ColumnProperty, index); grid.Children.Add(cell);
        }
        return grid;
    }
}
