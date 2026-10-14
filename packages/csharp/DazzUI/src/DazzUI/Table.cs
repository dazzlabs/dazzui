using System.Collections.ObjectModel;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Markup;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Input;

namespace DazzUI;

/// <summary>A table composed from TableHead, TableRow and TableCell, as in React.</summary>
[ContentProperty(Name = nameof(Children))]
public sealed class Table : Surface
{
    private readonly StackPanel rows = new();
    public ObservableCollection<UIElement> Children { get; } = [];
    public Table()
    {
        Content = rows;
        Children.CollectionChanged += (_, _) => Rebuild();
    }
    private void Rebuild()
    {
        TableLayout.Sync(rows.Children, Children.ToArray());
    }
    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        Background = null;
        CornerRadius = new CornerRadius(0);
        Padding = new Thickness(0);
    }
    protected override AutomationPeer OnCreateAutomationPeer() => new TablePeer(this);
    private sealed class TablePeer(Table owner) : FrameworkElementAutomationPeer(owner), IGridProvider, ITableProvider, ISelectionProvider
    {
        private TableRow[] Rows => owner.Children.OfType<TableRow>().ToArray();
        private TableHead? Head => owner.Children.OfType<TableHead>().FirstOrDefault();
        public int RowCount => Rows.Length;
        public int ColumnCount => Math.Max(Head?.Children.OfType<TableCell>().Count() ?? 0, Rows.Select(row => row.Children.OfType<TableCell>().Count()).DefaultIfEmpty().Max());
        public RowOrColumnMajor RowOrColumnMajor => RowOrColumnMajor.RowMajor;
        public IRawElementProviderSimple GetItem(int row, int column)
        {
            var rows = Rows;
            if (row < 0 || row >= rows.Length) throw new ArgumentOutOfRangeException(nameof(row));
            var cells = rows[row].Children.OfType<TableCell>().ToArray();
            if (column < 0 || column >= cells.Length)
                throw new ArgumentOutOfRangeException();
            return ProviderFromPeer(CreatePeerForElement(cells[column]));
        }
        public IRawElementProviderSimple[] GetColumnHeaders() => (Head?.Children ?? [])
            .OfType<TableCell>().Where(cell => cell.Head).Select(cell => ProviderFromPeer(CreatePeerForElement(cell))).ToArray();
        public IRawElementProviderSimple[] GetRowHeaders() => [];
        public bool CanSelectMultiple => true;
        public bool IsSelectionRequired => false;
        public IRawElementProviderSimple[] GetSelection() => Rows.Where(row => row.Active && row.Interactive)
            .Select(row => ProviderFromPeer(CreatePeerForElement(row))).ToArray();
        protected override object GetPatternCore(PatternInterface pattern) => pattern is PatternInterface.Grid or PatternInterface.Table or PatternInterface.Selection ? this : base.GetPatternCore(pattern);
        protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.DataGrid;
    }
}

/// <summary>The chrome band containing independently addressable heading cells.</summary>
[ContentProperty(Name = nameof(Children))]
public sealed class TableHead : Surface
{
    private readonly Grid cells = new();
    public ObservableCollection<UIElement> Children { get; } = [];
    public TableHead()
    {
        Content = cells;
        Children.CollectionChanged += (_, _) => TableLayout.Fill(cells, Children);
    }
    protected override void RefreshAppearance()
    {
        Background = Theme.Brush("color.surface-chrome");
        BorderBrush = Theme.Brush("color.border");
        BorderThickness = new Thickness(0, 0, 0, Theme.Number("stroke.hairline"));
        CornerRadius = new CornerRadius(0);
        Padding = new Thickness(Theme.Number("spacing.5"), 0, Theme.Number("spacing.5"), 0);
    }
    protected override AutomationPeer OnCreateAutomationPeer() => new HeadPeer(this);
    private sealed class HeadPeer(TableHead owner) : FrameworkElementAutomationPeer(owner)
    {
        protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.Header;
    }
}

/// <summary>A ruled row. Active is controlled by the caller; a press never toggles it.</summary>
[ContentProperty(Name = nameof(Children))]
public sealed class TableRow : Microsoft.UI.Xaml.Controls.Button
{
    private readonly Grid cells = new();
    private bool hovered;
    public ObservableCollection<UIElement> Children { get; } = [];
    public static readonly DependencyProperty ActiveProperty = DependencyProperty.Register(nameof(Active), typeof(bool), typeof(TableRow), new PropertyMetadata(false, Changed));
    public static readonly DependencyProperty InteractiveProperty = DependencyProperty.Register(nameof(Interactive), typeof(bool), typeof(TableRow), new PropertyMetadata(false, Changed));
    public bool Active { get => (bool)GetValue(ActiveProperty); set => SetValue(ActiveProperty, value); }
    public bool Interactive { get => (bool)GetValue(InteractiveProperty); set => SetValue(InteractiveProperty, value); }
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public TableRow()
    {
        DefaultStyleKey = typeof(TableRow);
        UseLayoutRounding = false;
        UseSystemFocusVisuals = true;
        MinHeight = MinWidth = 0;
        HorizontalContentAlignment = HorizontalAlignment.Stretch;
        VerticalContentAlignment = VerticalAlignment.Center;
        Content = cells;
        Children.CollectionChanged += (_, _) => TableLayout.Fill(cells, Children);
        Appearance.Bind(this, RefreshAppearance);
        PointerEntered += (_, _) => { hovered = true; Paint(); };
        PointerExited += (_, _) => { hovered = false; Paint(); };
    }
    private static void Changed(DependencyObject sender, DependencyPropertyChangedEventArgs _) => ((TableRow)sender).RefreshAppearance();
    private WidgetTint TableTint => TableLayout.Parent<Table>(this)?.Tint ?? WidgetTint.Primary;
    private void Paint()
    {
        Background = Interactive && hovered
            ? Theme.Recipe(ControlVariant.Normal, TableTint, "surface", "hovered")
            : Active ? Theme.Alpha($"color.{Theme.Key(TableTint)}.600", Theme.Number("wash.surface"))
            : Theme.Alpha("color.surface", 0);
    }
    private void RefreshAppearance()
    {
        Foreground = Theme.Brush("color.content");
        Theme.Typography(this, "bodyMedium");
        Skin.Focus(this, Theme);
        BorderBrush = Theme.Brush("color.border");
        BorderThickness = new Thickness(0, 0, 0, Theme.Number("stroke.hairline"));
        CornerRadius = new CornerRadius(0);
        Padding = new Thickness(Theme.Number("spacing.5"), Theme.Number("spacing.3"), Theme.Number("spacing.5"), Theme.Number("spacing.3"));
        IsTabStop = AllowFocusOnInteraction = Interactive;
        ProtectedCursor = InputSystemCursor.Create(Interactive ? InputSystemCursorShape.Hand : InputSystemCursorShape.Arrow);
        Paint();
    }
    protected override void OnKeyDown(KeyRoutedEventArgs args)
    {
        if (Interactive) base.OnKeyDown(args);
    }
    protected override AutomationPeer OnCreateAutomationPeer() => new RowPeer(this);
    private sealed class RowPeer(TableRow owner) : ButtonAutomationPeer(owner), ISelectionItemProvider
    {
        public bool IsSelected => owner.Active;
        public IRawElementProviderSimple SelectionContainer => TableLayout.Parent<Table>(owner) is { } table
            ? ProviderFromPeer(CreatePeerForElement(table)) : null!;
        private void RequestSelection(bool selected)
        {
            if (owner.Active == selected) return;
            if (!owner.Interactive) throw new InvalidOperationException("The table row is not interactive.");
            ((IInvokeProvider)base.GetPatternCore(PatternInterface.Invoke)).Invoke();
        }
        public void AddToSelection() => RequestSelection(true);
        public void RemoveFromSelection() => RequestSelection(false);
        public void Select() => RequestSelection(true);
        protected override string GetNameCore() => AutomationProperties.GetName(owner) is { Length: > 0 } name
            ? name : string.Join(" ", owner.Children.Select(TableLayout.Text));
        protected override object GetPatternCore(PatternInterface pattern) => pattern == PatternInterface.SelectionItem ? owner.Interactive ? this : null!
            : pattern == PatternInterface.Invoke && !owner.Interactive ? null! : base.GetPatternCore(pattern);
        protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.DataItem;
    }
}

/// <summary>An equal share of its row, holding words or rich component content.</summary>
public sealed class TableCell : Surface
{
    public static readonly DependencyProperty HeadProperty = DependencyProperty.Register(nameof(Head), typeof(bool), typeof(TableCell), new PropertyMetadata(false, Changed));
    public static readonly DependencyProperty AlignProperty = DependencyProperty.Register(nameof(Align), typeof(TextAlignment), typeof(TableCell), new PropertyMetadata(TextAlignment.Left, Changed));
    public bool Head { get => (bool)GetValue(HeadProperty); set => SetValue(HeadProperty, value); }
    public TextAlignment Align { get => (TextAlignment)GetValue(AlignProperty); set => SetValue(AlignProperty, value); }
    private double inlineTop, inlineBottom;
    public TableCell()
    {
        MinWidth = 0;
        VerticalAlignment = VerticalAlignment.Center;
        LayoutUpdated += (_, _) => UpdateInlineBox();
    }
    private void UpdateInlineBox()
    {
        double top = 0, bottom = 0;
        // A rich inline control participates in a line with the cell's own
        // font strut. Block panels already own their layout and need no strut.
        if (Content is FrameworkElement content && content is not TextBlock and not Panel && content.ActualHeight > 0)
        {
            var height = content.ActualHeight;
            var baseline = height;
            TextBlock? FirstText(DependencyObject root)
            {
                if (root is TextBlock text) return text;
                for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
                    if (FirstText(VisualTreeHelper.GetChild(root, i)) is { } found) return found;
                return null;
            }
            if (FirstText(content) is { } words && words.ActualHeight > 0)
            {
                var leading = words.LineHeight > 0 ? words.LineHeight : words.ActualHeight;
                // TransformToVisual includes the label's baseline translation.
                baseline = words.TransformToVisual(content).TransformPoint(default).Y + words.BaselineOffset;
                if (words.RenderTransform is not TranslateTransform)
                    baseline += PlatformFonts.LineBaseline(words.FontFamily, words.FontSize, words.FontWeight, leading) - words.BaselineOffset;
            }
            var lineHeight = Theme.Text("", Head ? "labelSmall" : "bodyMedium").LineHeight;
            var strut = PlatformFonts.LineBaseline(FontFamily, FontSize, PlatformFonts.MatchWeight(FontFamily, FontWeight), lineHeight);
            top = Math.Max(0, strut - baseline);
            bottom = Math.Max(0, lineHeight - strut - (height - baseline));
        }
        if (Math.Abs(inlineTop - top) < 0.001 && Math.Abs(inlineBottom - bottom) < 0.001) return;
        inlineTop = top;
        inlineBottom = bottom;
        InvalidateMeasure();
    }
    protected override Windows.Foundation.Size MeasureOverride(Windows.Foundation.Size availableSize)
    {
        var desired = base.MeasureOverride(availableSize);
        return new Windows.Foundation.Size(desired.Width, desired.Height + inlineTop + inlineBottom);
    }
    protected override Windows.Foundation.Size ArrangeOverride(Windows.Foundation.Size finalSize)
    {
        base.ArrangeOverride(finalSize);
        if ((inlineTop != 0 || inlineBottom != 0) && VisualTreeHelper.GetChildrenCount(this) > 0
            && VisualTreeHelper.GetChild(this, 0) is FrameworkElement root)
            root.Arrange(new Windows.Foundation.Rect(0, inlineTop, finalSize.Width, Math.Max(0, finalSize.Height - inlineTop - inlineBottom)));
        return finalSize;
    }
    private static void Changed(DependencyObject sender, DependencyPropertyChangedEventArgs _) => Appearance.Refresh(sender);
    protected override void RefreshAppearance()
    {
        var type = Head ? "labelSmall" : "bodyMedium";
        Theme.Typography(this, type);
        Foreground = Theme.Brush(Head ? "color.content-faint" : "color.content");
        Background = null;
        BorderThickness = new Thickness(0);
        CornerRadius = new CornerRadius(0);
        Padding = Head ? new Thickness(0, Theme.Number("spacing.2"), 0, Theme.Number("spacing.2")) : new Thickness(0);
        HorizontalContentAlignment = Align == TextAlignment.Right ? HorizontalAlignment.Right : Align == TextAlignment.Center ? HorizontalAlignment.Center : HorizontalAlignment.Left;
        Skin.TextLine(this, Theme.Text("", type).LineHeight, wrap: true);
        if (Content is TextBlock text) text.TextAlignment = Align;
    }
    protected override AutomationPeer OnCreateAutomationPeer() => new CellPeer(this);
    private sealed class CellPeer(TableCell owner) : FrameworkElementAutomationPeer(owner), IGridItemProvider, ITableItemProvider
    {
        protected override string GetNameCore() => AutomationProperties.GetName(owner) is { Length: > 0 } name
            ? name : TableLayout.Text(owner.Content);
        private Table? Table => TableLayout.Parent<Table>(owner);
        public int Row => Table?.Children.OfType<TableRow>().ToList().IndexOf(TableLayout.Parent<TableRow>(owner)!) ?? -1;
        public int Column => TableLayout.Parent<TableRow>(owner)?.Children.OfType<TableCell>().ToList().IndexOf(owner)
            ?? TableLayout.Parent<TableHead>(owner)?.Children.OfType<TableCell>().ToList().IndexOf(owner) ?? -1;
        public int RowSpan => 1;
        public int ColumnSpan => 1;
        public IRawElementProviderSimple ContainingGrid => Table is { } table ? ProviderFromPeer(CreatePeerForElement(table)) : null!;
        public IRawElementProviderSimple[] GetColumnHeaderItems() => Table?.Children.OfType<TableHead>().FirstOrDefault()?.Children
            .OfType<TableCell>()
            .Where((cell, index) => index == Column && cell.Head).Select(cell => ProviderFromPeer(CreatePeerForElement(cell))).ToArray() ?? [];
        public IRawElementProviderSimple[] GetRowHeaderItems() => [];
        protected override object GetPatternCore(PatternInterface pattern) => pattern is PatternInterface.GridItem or PatternInterface.TableItem ? this : base.GetPatternCore(pattern);
        protected override AutomationControlType GetAutomationControlTypeCore() => owner.Head ? AutomationControlType.HeaderItem : AutomationControlType.DataItem;
    }
}

internal static class TableLayout
{
    internal static string Text(object? content) => content switch
    {
        string text => text,
        TextBlock text => text.Text,
        ContentControl control => Text(control.Content),
        Panel panel => string.Join(" ", panel.Children.Select(child => Text(child))),
        _ => "",
    };
    internal static T? Parent<T>(DependencyObject element) where T : DependencyObject
    {
        for (var parent = VisualTreeHelper.GetParent(element); parent is not null; parent = VisualTreeHelper.GetParent(parent))
            if (parent is T found) return found;
        return null;
    }
    internal static void Fill(Grid grid, IEnumerable<UIElement> cells)
    {
        var children = cells.ToArray();
        Sync(grid.Children, children);
        grid.ColumnDefinitions.Clear();
        foreach (var cell in children)
        {
            cell.SetValue(Grid.ColumnProperty, grid.ColumnDefinitions.Count);
            grid.ColumnDefinitions.Add(new ColumnDefinition { Width = cell is TableCell ? new GridLength(1, GridUnitType.Star) : GridLength.Auto, MinWidth = 0 });
        }
    }
    internal static void Sync(UIElementCollection target, IReadOnlyList<UIElement> children)
    {
        foreach (var previous in target.ToArray())
            if (!children.Contains(previous)) target.Remove(previous);
        for (var index = 0; index < children.Count; index++)
        {
            var child = children[index];
            var previous = target.IndexOf(child);
            if (previous == index) continue;
            if (previous >= 0) target.RemoveAt(previous);
            else Skin.Detach(child);
            target.Insert(index, child);
        }
    }
}
