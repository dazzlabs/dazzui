using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

public sealed record TreeNode(string Id, string Label, IReadOnlyList<TreeNode>? Children = null, string? Meta = null, bool Expanded = true, bool Disabled = false,
    Glyph? Icon = null, Func<UIElement>? MetaView = null,
    Func<UIElement>? IconView = null, Func<UIElement>? LabelView = null)
{
    public override string ToString() => Meta is null ? Label : $"{Label}   {Meta}";
}
/// <summary>
/// A column of rows that happen to be a hierarchy, drawn as a sidebar's rows
/// are: a twisty column kept on leaves too, the glyph, the label, and an
/// aside at the end. Selection is the tinted wash and the label weight; the
/// arrow keys walk it and open and close branches.
/// </summary>
public sealed class Tree : Surface
{
    private readonly StackPanel rows = new();
    private readonly HashSet<string> expanded = [];
    private readonly List<(TreeNode Node, Grid Row)> visible = [];
    private bool seeded;
    private string? selected;
    private string? hovered;
    private bool controlledExpansion;
    public IReadOnlyList<string>? ExpandedIds
    {
        get => expanded.ToArray();
        set { controlledExpansion = value is not null; if (value is null) return; expanded.Clear(); expanded.UnionWith(value); seeded = true; if (IsLoaded) Rebuild(); }
    }
    public event EventHandler<IReadOnlyList<string>>? ExpandedChanged;
    public IList<TreeNode> Nodes { get; } = new List<TreeNode>();
    public TreeNode? SelectedNode => Find(Nodes, selected);
    /// <summary>The selected row's id; setting it selects without raising <see cref="Selected"/>.</summary>
    public string? SelectedId { get => selected; set { selected = value; if (IsLoaded) Rebuild(); } }
    public event EventHandler<TreeNode>? Selected;
    public Tree()
    {
        Size = ControlSize.Medium;
        Content = rows;
        IsTabStop = true;
        UseSystemFocusVisuals = true;
        KeyDown += (_, args) => { if (Navigate(args.Key)) args.Handled = true; };
    }
    private static TreeNode? Find(IEnumerable<TreeNode> nodes, string? id)
    {
        foreach (var node in nodes)
        {
            if (node.Id == id) return node;
            if (Find(node.Children ?? [], id) is TreeNode match) return match;
        }
        return null;
    }
    private void Select(TreeNode node)
    {
        if (node.Disabled) return;
        selected = node.Id;
        Rebuild();
        Selected?.Invoke(this, node);
    }
    private void Toggle(TreeNode node)
    {
        if (node.Disabled || node.Children is null) return;
        var next = new HashSet<string>(expanded);
        if (!next.Remove(node.Id)) next.Add(node.Id);
        if (!controlledExpansion) { expanded.Clear(); expanded.UnionWith(next); Rebuild(); }
        ExpandedChanged?.Invoke(this, next.ToArray());
    }
    private bool Navigate(Windows.System.VirtualKey key)
    {
        var at = visible.FindIndex(pair => pair.Node.Id == selected);
        var node = at >= 0 ? visible[at].Node : null;
        switch (key)
        {
            case Windows.System.VirtualKey.Down when at + 1 < visible.Count: Select(visible[at + 1].Node); return true;
            case Windows.System.VirtualKey.Up when at > 0: Select(visible[at - 1].Node); return true;
            case Windows.System.VirtualKey.Right when node?.Children is not null && !expanded.Contains(node.Id): Toggle(node); return true;
            case Windows.System.VirtualKey.Left when node is not null && expanded.Contains(node.Id): Toggle(node); return true;
            case Windows.System.VirtualKey.Enter or Windows.System.VirtualKey.Space when node?.Children is not null:
                Toggle(node);
                return true;
        }
        return false;
    }
    protected override void RefreshAppearance()
    {
        if (!seeded)
        {
            seeded = true;
            void Seed(IEnumerable<TreeNode> nodes) { foreach (var node in nodes) { if (node.Expanded && node.Children is not null) expanded.Add(node.Id); Seed(node.Children ?? []); } }
            Seed(Nodes);
        }
        Rebuild();
    }
    private void Rebuild()
    {
        rows.Children.Clear();
        visible.Clear();
        void Add(TreeNode node, int depth)
        {
            rows.Children.Add(Row(node, depth));
            if (node.Children is { Count: > 0 } && expanded.Contains(node.Id))
                foreach (var child in node.Children) Add(child, depth + 1);
        }
        foreach (var node in Nodes) Add(node, 0);
    }
    private Grid Row(TreeNode node, int depth)
    {
        var small = Size is ControlSize.Small or ControlSize.Tiny;
        var chosen = node.Id == selected;
        var over = node.Id == hovered && !chosen && !node.Disabled;
        var ink = node.Disabled ? Theme.Brush("color.content-faint")
            : chosen ? Theme.Recipe(ControlVariant.Tinted, Tint, "content")
            : over ? Theme.Brush("color.content") : Theme.Brush("color.content-nav");
        var row = new Grid
        {
            Height = Theme.Number(small ? "spacing.6" : "spacing.7"),
            ColumnSpacing = Theme.Number("spacing.1"),
            Padding = new Thickness(Theme.Number("spacing.1") + depth * Theme.Number("spacing.4"), 0, Theme.Number("spacing.2"), 0),
            CornerRadius = new CornerRadius(Theme.Number("radius.small")),
            Background = chosen ? Theme.Recipe(ControlVariant.Tinted, Tint, "surface") : over ? Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered") : new SolidColorBrush(Microsoft.UI.Colors.Transparent),
        };
        // WinUI stretches the full-radius token into an ellipse unless it is
        // clamped to half the row's shorter side, including after a resize.
        var radius = Theme.Number("radius.small");
        row.SizeChanged += (_, _) =>
        {
            var corner = new CornerRadius(Math.Min(radius, Math.Min(row.ActualWidth, row.ActualHeight) / 2));
            if (row.CornerRadius != corner) row.CornerRadius = corner;
        };
        foreach (var width in new[] { GridLength.Auto, GridLength.Auto, new GridLength(1, GridUnitType.Star), GridLength.Auto })
            row.ColumnDefinitions.Add(new ColumnDefinition { Width = width });
        // The twisty keeps its column on a leaf, so a level's glyphs stay in line.
        var twisty = new Border { Width = Theme.Number("spacing.4"), Height = Theme.Number("spacing.4"), VerticalAlignment = VerticalAlignment.Center };
        if (node.Children is not null)
        {
            var open = expanded.Contains(node.Id);
            twisty.Child = new Icon
            {
                Glyph = FluentIcons.Common.Icon.ChevronRight,
                GlyphSize = Theme.Number("icon.small"),
                Foreground = Theme.Brush("color.content-faint"),
                HorizontalAlignment = HorizontalAlignment.Center,
                VerticalAlignment = VerticalAlignment.Center,
                RenderTransform = new RotateTransform { Angle = open ? 90 : 0 },
                RenderTransformOrigin = new Windows.Foundation.Point(.5, .5),
            };
            twisty.Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent);
            twisty.Tapped += (_, args) => { Toggle(node); args.Handled = true; };
        }
        row.Children.Add(twisty);
        UIElement? iconSlot = node.IconView?.Invoke();
        if (iconSlot is null && node.Icon is Glyph glyph)
            iconSlot = new Icon { Glyph = glyph, GlyphSize = Theme.Number("icon.medium") };
        if (iconSlot is FrameworkElement iconView)
        {
            if (iconView is Icon icon) { icon.GlyphSize = Theme.Number("icon.medium"); icon.Foreground = chosen ? ink : Theme.Brush("color.content-subtle"); }
            iconView.VerticalAlignment = VerticalAlignment.Center;
            Grid.SetColumn(iconView, 1); row.Children.Add(iconView);
        }
        UIElement labelContent = node.LabelView?.Invoke() ?? new TextBlock { Text = node.Label };
        FrameworkElement label = labelContent as FrameworkElement ?? new ContentPresenter { Content = labelContent };
        if (label is TextBlock textLabel)
        {
            textLabel.Foreground = ink;
            textLabel.IsTextScaleFactorEnabled = Theme.Sizing != ThemeSizing.System;
            textLabel.FontSize = Theme.Text("", small ? "labelSmall" : "labelQuiet").FontSize;
            TextStyle.SetWeight(textLabel, Theme.Weight(chosen ? "labelMedium" : "labelQuiet"));
            textLabel.TextTrimming = TextTrimming.CharacterEllipsis;
        }
        label.VerticalAlignment = VerticalAlignment.Center;
        Grid.SetColumn(label, 2); row.Children.Add(label);
        UIElement? meta = node.MetaView?.Invoke();
        if (meta is null && node.Meta is not null)
        {
            var text = Theme.Text(node.Meta, "captionSmall");
            text.Foreground = chosen ? ink : Theme.Brush("color.content-faint");
            meta = text;
        }
        if (meta is FrameworkElement aside)
        {
            aside.VerticalAlignment = VerticalAlignment.Center;
            aside.Margin = new Thickness(Theme.Number("spacing.2"), 0, 0, 0);
            Grid.SetColumn(aside, 3); row.Children.Add(aside);
        }
        row.Tapped += (_, _) => { if (node.Disabled) return; Toggle(node); Select(node); Focus(FocusState.Pointer); };
        row.PointerEntered += (_, _) => { hovered = node.Id; if (!chosen) Rebuild(); };
        row.PointerExited += (_, _) => { if (hovered == node.Id) { hovered = null; Rebuild(); } };
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(row, node.Label);
        visible.Add((node, row));
        return row;
    }
}
