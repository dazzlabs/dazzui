using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI;

public enum ActionBarAlignment { Start, End, Between }
public sealed class ActionBar : Surface
{
    public IList<UIElement> Items { get; } = new List<UIElement>();
    public ActionBarAlignment Alignment { get; set; } = ActionBarAlignment.End;
    protected override void RefreshAppearance()
    {
        if (Alignment != ActionBarAlignment.Between)
        {
            var row = Skin.Row(Theme, Items.ToArray());
            row.HorizontalAlignment = Alignment == ActionBarAlignment.Start ? HorizontalAlignment.Left : HorizontalAlignment.Right;
            Content = row;
        }
        else
        {
            var grid = new Grid { ColumnSpacing = Theme.Number("spacing.3") };
            for (var i = 0; i < Items.Count; i++)
            {
                grid.ColumnDefinitions.Add(new ColumnDefinition { Width = i == 0 ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
                Skin.Detach(Items[i]); Items[i].SetValue(Grid.ColumnProperty, i); grid.Children.Add(Items[i]);
            }
            Content = grid;
        }
    }
}
public class Collapsible : Surface
{
    public Expander Expander { get; } = new() { HorizontalAlignment = HorizontalAlignment.Stretch, HorizontalContentAlignment = HorizontalAlignment.Stretch };
    public object Header { get => Expander.Header; set => Expander.Header = value; }
    public UIElement? Body { get => Expander.Content as UIElement; set => Expander.Content = value; }
    public bool IsExpanded { get => Expander.IsExpanded; set => Expander.IsExpanded = value; }
    public Collapsible() { Content = Expander; }
    protected override void RefreshAppearance()
    {
        Expander.Background = Theme.Brush("color.surface");
        Expander.Foreground = Theme.Brush("color.content");
        Expander.BorderBrush = Theme.Brush("color.border");
        Expander.BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        Expander.CornerRadius = new CornerRadius(Theme.Number("radius.medium"));
    }
}
public sealed class AccordionItem : Collapsible { }
public sealed class Accordion : Surface
{
    public IList<AccordionItem> Items { get; } = new List<AccordionItem>();
    public bool Multiple { get; set; }
    public bool Divided { get; set; }
    protected override void RefreshAppearance()
    {
        if (Content is null)
            foreach (var item in Items)
                item.Expander.Expanding += (_, _) => { if (!Multiple) foreach (var other in Items.Where(other => other != item)) other.IsExpanded = false; };
        var children = new List<UIElement>();
        foreach (var item in Items) { if (Divided && children.Count > 0) children.Add(new Divider()); children.Add(item); }
        Content = Skin.Column(Theme, children.ToArray());
    }
}
public sealed class PreferenceRow : Surface
{
    public string Label { get; set; } = "";
    public string? Description { get; set; }
    public UIElement? Control { get; set; }
    public UIElement? Leading { get; set; }
    protected override void RefreshAppearance()
    {
        var grid = new Grid { ColumnSpacing = Theme.Number("spacing.3") };
        grid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        grid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        var text = Skin.Column(Theme, Theme.Text(Label, "labelMedium"));
        if (Description is not null) text.Children.Add(Theme.Text(Description, "bodySmall"));
        Grid.SetColumn(text, 1); grid.Children.Add(text);
        if (Leading is not null) { Skin.Detach(Leading); grid.Children.Add(Leading); }
        if (Control is not null) { Skin.Detach(Control); Control.SetValue(Grid.ColumnProperty, 2); grid.Children.Add(Control); }
        Padding = new Thickness(Theme.Number("spacing.4") + Theme.Number("spacing.0.5"));
        Content = grid;
    }
}
public sealed class PreferenceGroup : Surface
{
    public IList<PreferenceRow> Rows { get; } = new List<PreferenceRow>();
    protected override void RefreshAppearance()
    {
        var rows = new List<UIElement>();
        foreach (var row in Rows) { if (rows.Count > 0) rows.Add(new Divider()); rows.Add(row); }
        Content = new Card(Skin.Column(Theme, rows.ToArray()), Theme);
    }
}
public sealed class PreferenceSection : Surface
{
    public string Title { get; set; } = "";
    public string? Description { get; set; }
    public UIElement? Body { get; set; }
    protected override void RefreshAppearance()
    {
        var children = new List<UIElement> { Theme.Text(Title, "titleSmall") };
        if (Description is not null) children.Add(Theme.Text(Description, "bodySmall"));
        if (Body is not null) children.Add(Body);
        Content = Skin.Column(Theme, children.ToArray());
    }
}
public sealed class Preferences : Surface
{
    public IList<PreferenceSection> Sections { get; } = new List<PreferenceSection>();
    protected override void RefreshAppearance()
    {
        MaxWidth = Theme.Number("preferences.width");
        Content = Skin.Column(Theme, Sections.Cast<UIElement>().ToArray());
    }
}
public class NavItem : Toggle
{
    public string? Count { get; set; }
    public IconGlyph Glyph { get; set; } = IconGlyph.Folder;
    public bool IsIconOnly { get; set; }
    public NavItem() { DefaultStyleKey = typeof(NavItem); Size = ControlSize.Medium; HorizontalContentAlignment = HorizontalAlignment.Left; HorizontalAlignment = HorizontalAlignment.Stretch; Variant = ControlVariant.Normal; }
    protected override void OnApplyTemplate() { base.OnApplyTemplate(); Apply(); }
    protected override void Apply()
    {
        base.Apply();
        Theme.Typography(this, IsChecked == true ? "labelMedium" : "labelQuiet");
        if (GetTemplateChild("Label") is ContentPresenter label) label.Visibility = IsIconOnly ? Visibility.Collapsed : Visibility.Visible;
        if (GetTemplateChild("Leading") is ContentControl leading)
        {
            if (leading.Content is not Icon) leading.Content = new Icon { Glyph = Glyph };
            leading.Margin = new Thickness(0, 0, IsIconOnly ? 0 : Theme.Number("spacing.2"), 0);
        }
        if (GetTemplateChild("Count") is TextBlock count)
        {
            count.Text = Count ?? ""; count.Visibility = IsIconOnly || Count is null ? Visibility.Collapsed : Visibility.Visible;
        }
        if (IsIconOnly && Content is string text) Tooltip.Set(this, text);
    }
}
public enum SidebarCollapsible { None, Offcanvas, Icons }
public class Sidebar : Surface
{
    private bool collapsed;
    private readonly HashSet<NavItem> wired = [];
    public bool IsCollapsed { get => collapsed; set { collapsed = value; RefreshAppearance(); } }
    public SidebarCollapsible Collapsible { get; set; }
    public UIElement? Header { get; set; }
    public UIElement? Footer { get; set; }
    public IList<UIElement> Items { get; } = new List<UIElement>();
    protected override void RefreshAppearance()
    {
        Width = IsCollapsed && Collapsible == SidebarCollapsible.Offcanvas ? 0 : Theme.Number(IsCollapsed && Collapsible == SidebarCollapsible.Icons ? "frame.sidebar.icon-width" : "frame.sidebar.width");
        Background = Theme.Brush("color.surface-column");
        var icons = IsCollapsed && Collapsible == SidebarCollapsible.Icons;
        foreach (var group in Items.OfType<SidebarGroup>()) group.IsIconOnly = icons;
        var navigation = Items.OfType<NavItem>().Concat(Items.OfType<SidebarGroup>().SelectMany(group => group.Items.OfType<NavItem>())).ToArray();
        foreach (var item in navigation)
        {
            item.IsIconOnly = icons;
            if (wired.Add(item)) item.Checked += (_, _) => { foreach (var other in navigation.Where(other => other != item)) other.IsChecked = false; };
        }
        var grid = new Grid();
        grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        grid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });
        grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        if (Header is not null && !icons) { Skin.Detach(Header); grid.Children.Add(Header); }
        var scroll = new ScrollViewer { Content = Skin.Column(Theme, Items.ToArray()), HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled };
        Grid.SetRow(scroll, 1); grid.Children.Add(scroll);
        if (Footer is not null && !icons) { Skin.Detach(Footer); Footer.SetValue(Grid.RowProperty, 2); grid.Children.Add(Footer); }
        Padding = new Thickness(Theme.Number("spacing.2")); Content = grid;
    }
}
public sealed class SidebarGroup : Surface
{
    internal bool IsIconOnly { get; set; }
    public string Label { get; set; } = "";
    public IList<UIElement> Items { get; } = new List<UIElement>();
    protected override void RefreshAppearance() { Content = Skin.Column(Theme, (IsIconOnly ? Array.Empty<UIElement>() : new UIElement[] { new SectionLabel(Label) }).Concat(Items).ToArray()); }
}
public sealed class Aside : Card
{
    protected override void RefreshAppearance() { base.RefreshAppearance(); Width = Theme.Number("frame.aside.width"); Background = Theme.Brush("color.surface-column"); }
}
public sealed class Rail : Surface
{
    public IList<UIElement> Items { get; } = new List<UIElement>();
    protected override void RefreshAppearance()
    {
        Width = Theme.Number("frame.rail.width"); Background = Theme.Brush("color.surface-column");
        Content = Skin.Column(Theme, Items.ToArray()); Padding = new Thickness(Theme.Number("spacing.2"));
    }
}
