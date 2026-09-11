using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Playground;

public sealed class PlaygroundShell : Grid
{
    public sealed record Entry(StoryPage Page, Story Story)
    {
        public string Id => System.Text.RegularExpressions.Regex.Replace($"{Page.Group}-{Page.Name}", "[^a-zA-Z0-9]+", "-").ToLowerInvariant() + "--" + System.Text.RegularExpressions.Regex.Replace(Story.Name, "[^a-zA-Z0-9]+", "-").ToLowerInvariant().Trim('-');
    }
    public static IReadOnlyList<Entry> Entries { get; } = Registry.Pages.SelectMany(page => page.Stories.Select(story => new Entry(page, story))).ToArray();
    private readonly UI.SegmentedControl family = new() { Size = UI.ControlSize.Small };
    private readonly UI.SegmentedControl brightness = new() { Size = UI.ControlSize.Small };
    private readonly UI.SearchField search = new() { PlaceholderText = "Find a widget", Size = UI.ControlSize.Small };
    private readonly UI.Tree navigator = new() { Size = UI.ControlSize.Small };
    private readonly Border sidebar = new();
    private readonly Border canvas = new();
    private readonly Grid toolbar = new();
    private readonly TextBlock title = new() { Text = "DazzUI" };
    private readonly TextBlock breadcrumb = new();
    private readonly Border breadcrumbBar = new();
    private UI.Theme theme = UI.Theme.Default;
    private bool updating;
    private int selected = Entries.ToList().FindIndex(entry => entry.Page.Name == "Button" && entry.Story.Name == "Overview");
    public UIElement? CurrentSpecimen { get; private set; }
    public Entry CurrentEntry => Entries[selected];

    public PlaygroundShell()
    {
        RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });
        foreach (var label in new[] { "Studio", "Bright", "Frost", "Graphite", "Ember", "Nocturne" }) family.Items.Add(label);
        brightness.Items.Add("Light"); brightness.Items.Add("Dark");
        family.SelectedIndex = brightness.SelectedIndex = 0;
        toolbar.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        toolbar.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        var brand = new StackPanel { Orientation = Orientation.Horizontal, Spacing = theme.Number("spacing.3"), VerticalAlignment = VerticalAlignment.Center, Children = { title, new UI.Badge("WinUI 3") { Size = UI.ControlSize.Small } } };
        toolbar.Children.Add(brand);
        var pickers = new StackPanel { Orientation = Orientation.Horizontal, Spacing = theme.Number("spacing.3"), Children = { family, brightness } }; Grid.SetColumn(pickers, 1); toolbar.Children.Add(pickers);
        Children.Add(new ScrollViewer { Content = toolbar, HorizontalScrollBarVisibility = ScrollBarVisibility.Auto, VerticalScrollBarVisibility = ScrollBarVisibility.Disabled });

        var workspace = new Grid();
        workspace.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        workspace.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        Grid.SetRow(workspace, 1); Children.Add(workspace);
        var nav = new Grid(); nav.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto }); nav.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });
        nav.Children.Add(search); Grid.SetRow(navigator, 1); nav.Children.Add(navigator); sidebar.Child = nav; workspace.Children.Add(sidebar);
        var right = new Grid(); right.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto }); right.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });
        Grid.SetColumn(right, 1); workspace.Children.Add(right);
        breadcrumbBar.Child = breadcrumb; right.Children.Add(breadcrumbBar); Grid.SetRow(canvas, 1); right.Children.Add(canvas);
        search.TextChanged += (_, _) => Filter();
        navigator.Loaded += (_, _) => SyncNavigation();
        navigator.Selected += (_, node) => { if (!updating && int.TryParse(node.Id, out var index)) { selected = index; RenderStory(); } };
        family.SelectionChanged += (_, _) => ChangeTheme(); brightness.SelectionChanged += (_, _) => ChangeTheme();
        Filter(); ApplyTheme(); RenderStory();
    }
    private void ChangeTheme()
    {
        if (updating || family.SelectedItem is null || brightness.SelectedItem is null) return;
        theme = new UI.Theme($"{family.SelectedItem.ToString()!.ToLowerInvariant()}-{brightness.SelectedItem.ToString()!.ToLowerInvariant()}");
        ApplyTheme(); RenderStory();
    }
    private void ApplyTheme()
    {
        RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
        UI.Appearance.SetTheme(this, theme);
        Background = theme.Brush("color.canvas");
        toolbar.Background = theme.Brush("color.surface-muted");
        toolbar.Padding = new Thickness(theme.Number("spacing.4"), theme.Number("spacing.2"), theme.Number("spacing.4"), theme.Number("spacing.2"));
        toolbar.ColumnSpacing = theme.Number("spacing.4");
        title.Foreground = theme.Brush("color.content"); title.FontSize = theme.Text("", "labelMedium").FontSize;
        sidebar.Width = theme.Number("frame.sidebar.width");
        sidebar.Background = theme.Brush("color.surface-column");
        sidebar.Padding = new Thickness(theme.Number("spacing.2"));
        sidebar.BorderBrush = theme.Brush("color.border"); sidebar.BorderThickness = new Thickness(0, 0, theme.Number("stroke.hairline"), 0);
        search.Margin = new Thickness(0, 0, 0, theme.Number("spacing.3"));
        breadcrumb.Foreground = theme.Brush("color.content-muted");
        breadcrumb.FontSize = theme.Text("", "labelSmall").FontSize;
        breadcrumbBar.Padding = new Thickness(theme.Number("spacing.4"), theme.Number("spacing.3"), theme.Number("spacing.4"), theme.Number("spacing.3"));
        breadcrumbBar.Background = theme.Brush("color.surface"); breadcrumbBar.BorderBrush = theme.Brush("color.border"); breadcrumbBar.BorderThickness = new Thickness(0, 0, 0, theme.Number("stroke.hairline"));
        canvas.Background = theme.Brush("color.surface");
    }
    private void Filter()
    {
        var query = search.Text.Trim();
        navigator.Nodes.Clear(); navigator.View.RootNodes.Clear();
        foreach (var group in Registry.Pages.GroupBy(page => page.Group))
        {
            var pages = new List<UI.TreeNode>();
            foreach (var page in group)
            {
                var matchPage = page.Name.Contains(query, StringComparison.OrdinalIgnoreCase);
                var stories = Entries.Select((entry, index) => (entry, index)).Where(pair => pair.entry.Page == page && (matchPage || pair.entry.Story.Name.Contains(query, StringComparison.OrdinalIgnoreCase))).ToArray();
                if (stories.Length == 0) continue;
                pages.Add(new(stories[0].index.ToString(), page.Name, stories.Select(pair => new UI.TreeNode(pair.index.ToString(), pair.entry.Story.Name)).ToArray(), Expanded: query.Length > 0 || page.Name == CurrentEntry.Page.Name));
            }
            if (pages.Count > 0) navigator.Nodes.Add(new(group.Key, group.Key, pages));
        }
        UI.Appearance.Refresh(navigator);
        SyncNavigation();
    }
    private void SyncNavigation()
    {
        TreeViewNode? Find(IEnumerable<TreeViewNode> nodes)
        {
            foreach (var node in nodes)
            {
                if (node.Children.Count == 0 && node.Content is UI.TreeNode model && model.Id == selected.ToString()) return node;
                var match = Find(node.Children); if (match is not null) return match;
            }
            return null;
        }
        var match = Find(navigator.View.RootNodes);
        if (match is not null && navigator.View.SelectedNode != match)
        {
            var previous = updating; updating = true;
            navigator.View.SelectedNode = match;
            updating = previous;
        }
    }
    private void RenderStory()
    {
        var entry = CurrentEntry;
        breadcrumb.Text = $"{entry.Page.Group}  /  {entry.Page.Name}  /  {entry.Story.Name}";
        CurrentSpecimen = entry.Story.Build(theme);
        var presenter = new ContentControl { Content = CurrentSpecimen, HorizontalContentAlignment = entry.Story.Layout == StoryLayout.Centered ? HorizontalAlignment.Center : HorizontalAlignment.Stretch, VerticalContentAlignment = entry.Story.Layout == StoryLayout.Centered ? VerticalAlignment.Center : VerticalAlignment.Top, Padding = new Thickness(theme.Number("spacing.8")) };
        canvas.Child = entry.Story.Layout == StoryLayout.Fullscreen ? presenter : new ScrollViewer { Content = presenter, HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled, VerticalScrollBarVisibility = ScrollBarVisibility.Auto };
    }
    internal void ShowFixture(UIElement content)
    {
        CurrentSpecimen = content; canvas.Child = content; UpdateLayout();
    }
    public void Show(string themeName, int storyIndex)
    {
        selected = storyIndex;
        if (theme.Name != themeName)
        {
            updating = true;
            theme = new UI.Theme(themeName);
            family.SelectedItem = char.ToUpperInvariant(themeName[0]) + themeName.Split('-')[0][1..]; brightness.SelectedIndex = theme.IsDark ? 1 : 0;
            updating = false; ApplyTheme();
        }
        RenderStory(); UpdateLayout();
    }
    public static int Find(string page, string story) => Entries.ToList().FindIndex(entry => entry.Page.Name == page && entry.Story.Name.Equals(story, StringComparison.OrdinalIgnoreCase));
}
