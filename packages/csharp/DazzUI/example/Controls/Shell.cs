using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Input;
using Windows.Foundation;
using Windows.System;

namespace DazzUI.Example;

/// <summary>
/// The example, as a desktop application's window: the component list on
/// the left, the document under its toolbar, and the theme panel on the
/// right — the same window the React example draws, column for column.
///
/// It holds the page and the theme — the two things the React example keeps
/// in its address — and the window's own state: the search, which columns
/// are open, and how wide.
/// </summary>
public sealed class Shell : UserControl
{
    private static readonly ColumnWidth ListWidth = new(272, 200, 420);
    private static readonly ColumnWidth ThemesWidth = new(336, 280, 560);

    private readonly Grid root = new();
    private readonly Grid window = new();
    private readonly ComponentList list;
    private readonly Toolbar toolbar;
    // The page is a panel's child, not a content control's: see Themed.
    private readonly Grid page = new();
    private readonly ThemePanel themes;
    private readonly ColumnDefinition listColumn = new();
    private readonly ColumnDefinition themesColumn = new();
    private readonly Resizer listResizer;
    private readonly Resizer themesResizer;
    private string query = "";

    public Shell(string? pageId, ThemeChoice theme)
    {
        Page = Stories.Find(pageId)?.Id;
        Theme = theme;
        list = new ComponentList(this);
        toolbar = new Toolbar(this);
        themes = new ThemePanel(this);
        listResizer = new Resizer(1, ListWidth, width => listColumn.Width = new GridLength(width), "Resize the component list") { HorizontalAlignment = HorizontalAlignment.Left };
        themesResizer = new Resizer(-1, ThemesWidth, width => themesColumn.Width = new GridLength(width), "Resize the theme panel") { HorizontalAlignment = HorizontalAlignment.Right };

        listColumn.Width = new GridLength(ListWidth.Initial);
        themesColumn.Width = new GridLength(ThemesWidth.Initial);
        window.ColumnDefinitions.Add(listColumn);
        window.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        window.ColumnDefinitions.Add(themesColumn);
        window.Children.Add(list);

        var main = new Grid { RowDefinitions = { new RowDefinition { Height = GridLength.Auto }, new RowDefinition { Height = new GridLength(1, GridUnitType.Star) } } };
        main.Children.Add(toolbar);
        Grid.SetRow(page, 1);
        main.Children.Add(page);
        Grid.SetColumn(main, 1);
        window.Children.Add(main);
        Grid.SetColumn(listResizer, 1);
        window.Children.Add(listResizer);

        Grid.SetColumn(themes, 2);
        window.Children.Add(themes);
        Grid.SetColumn(themesResizer, 1);
        window.Children.Add(themesResizer);

        // Toasts stand over the whole window, as the web's fixed viewport does.
        root.Children.Add(window);
        root.Children.Add(Overlay.Layer);
        Content = root;

        // Ctrl+B puts the list away, as the web's sidebar does with ⌘B.
        var toggle = new KeyboardAccelerator { Key = VirtualKey.B, Modifiers = VirtualKeyModifiers.Control };
        toggle.Invoked += (_, args) => { ToggleList(); args.Handled = true; };
        KeyboardAccelerators.Add(toggle);

        ApplyTheme();
        ShowPage();
    }

    /// <summary>The component being read, or null for the overview.</summary>
    public string? Page { get; private set; }

    public ThemeChoice Theme { get; private set; }

    public string Query
    {
        get => query;
        set { query = value; list.Refresh(); if (Page is null) ShowPage(); }
    }

    public bool ListOpen { get; private set; } = true;

    public bool ThemesOpen { get; private set; } = true;

    /// <summary>What the page shows now: the scenes a test can walk.</summary>
    public UIElement? CurrentPage => page.Children.FirstOrDefault();

    public void Open(string? id)
    {
        Page = Stories.Find(id)?.Id;
        list.Refresh();
        ShowPage();
    }

    public void Choose(ThemeChoice theme)
    {
        if (theme == Theme) return;
        Theme = theme;
        ApplyTheme();
    }

    public void ToggleList()
    {
        ListOpen = !ListOpen;
        list.Visibility = listResizer.Visibility = ListOpen ? Visibility.Visible : Visibility.Collapsed;
        listColumn.Width = new GridLength(ListOpen ? listResizer.Value : 0);
    }

    public void ToggleThemes()
    {
        ThemesOpen = !ThemesOpen;
        themes.Visibility = themesResizer.Visibility = ThemesOpen ? Visibility.Visible : Visibility.Collapsed;
        themesColumn.Width = new GridLength(ThemesOpen ? themesResizer.Value : 0);
    }

    /// <summary>Stands a test's own content where the page goes.</summary>
    internal void ShowFixture(UIElement content)
    {
        Show(content);
        UpdateLayout();
    }

    private void Show(UIElement content)
    {
        page.Children.Clear();
        page.Children.Add(content);
    }

    private void ApplyTheme()
    {
        var theme = Theme.Theme;
        RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
        Appearance.SetTheme(this, theme);
        // A UserControl paints no background of its own; its root does.
        root.Background = theme.Brush("color.surface");
        Foreground = theme.Brush("color.content");
        theme.Typography(this, "bodyMedium");
        themes.Refresh();
        Themed.Refresh();
    }

    private void ShowPage()
    {
        var story = Stories.Find(Page);
        toolbar.Show(story);
        Show(story is null ? new Overview(Stories.Matching(query), Open) : new Document(story));
    }
}

/// <summary>How wide a column may be, and how wide it starts.</summary>
public sealed record ColumnWidth(double Initial, double Min, double Max)
{
    public double Clamp(double width) => Math.Round(Math.Clamp(width, Min, Max));
}

/* The component list ------------------------------------------------------ */

internal sealed class ComponentList : Surface
{
    private readonly Shell shell;
    private readonly SearchField search = new() { PlaceholderText = "Search", Size = ControlSize.Small };
    private readonly NavItem overview;
    private readonly Styled count;
    private readonly Grid items = new();

    public ComponentList(Shell shell)
    {
        this.shell = shell;
        overview = new NavItem { Content = "Overview", Size = ControlSize.Small, Glyph = Glyph.Home };
        overview.Click += (_, _) => shell.Open(null);
        count = new Styled($"{Stories.All.Count}", "labelSmall") { IsHitTestVisible = false, HorizontalAlignment = HorizontalAlignment.Right, VerticalAlignment = VerticalAlignment.Center };
        search.TextChanged += (_, _) => shell.Query = search.Text;

        var header = new Stack(Orientation.Vertical, "spacing.0", theme => Inset(theme, "spacing.2", "spacing.2.5"))
        {
            MinHeightToken = "frame.titlebar.size",
            VerticalContentAlignment = VerticalAlignment.Center,
        };
        header.Add(new Brand(() => shell.Open(null)));

        var body = new Stack(Orientation.Vertical, "spacing.0", theme => new Thickness(0, 0, 0, theme.Number("spacing.2")));
        var find = new Stack(Orientation.Vertical, "spacing.0", theme => new Thickness(theme.Number("spacing.2.5"), 0, theme.Number("spacing.2.5"), theme.Number("spacing.1.5")));
        find.Add(search);
        body.Add(find);
        var home = new Grid { Children = { overview, count } };
        var homeGroup = new Stack(Orientation.Vertical, "spacing.0", theme => Inset(theme, "spacing.1.5", "spacing.2.5"));
        homeGroup.Add(home);
        body.Add(homeGroup);
        var group = new Stack(Orientation.Vertical, "frame.nav-gap", theme => Inset(theme, "spacing.1.5", "spacing.2.5"));
        var label = new SectionLabel("Components");
        label.Loaded += (_, _) => label.Margin = new Thickness(Appearance.GetTheme(label).Number("spacing.2.5"), Appearance.GetTheme(label).Number("spacing.1"), 0, Appearance.GetTheme(label).Number("spacing.1"));
        group.Add(label);
        group.Add(items);
        body.Add(group);

        var column = new Grid { RowDefinitions = { new RowDefinition { Height = GridLength.Auto }, new RowDefinition { Height = new GridLength(1, GridUnitType.Star) } } };
        column.Children.Add(header);
        var scroll = new ScrollViewer { Content = body, HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled };
        Grid.SetRow(scroll, 1);
        column.Children.Add(scroll);
        Content = column;
        VerticalContentAlignment = VerticalAlignment.Stretch;
        Refresh();
    }

    private static Thickness Inset(Theme theme, string block, string inline) =>
        new(theme.Number(inline), theme.Number(block), theme.Number(inline), theme.Number(block));

    /// <summary>Lists what the search leaves, marking the page being read.</summary>
    public void Refresh()
    {
        overview.IsChecked = shell.Page is null;
        var stories = Stories.Matching(shell.Query);
        var panel = new Stack(Orientation.Vertical, "frame.nav-gap");
        foreach (var story in stories)
        {
            var item = new NavItem { Content = story.Name, Size = ControlSize.Small, Glyph = null, IsChecked = story.Id == shell.Page };
            item.Click += (_, _) => shell.Open(story.Id);
            panel.Add(item);
        }
        items.Children.Clear();
        items.Children.Add(panel);
        if (IsLoaded) RefreshAppearance();
    }

    protected override void RefreshAppearance()
    {
        Background = Theme.Brush("color.surface-column");
        BorderBrush = Theme.Brush("color.border");
        BorderThickness = new Thickness(0, 0, Theme.Number("stroke.hairline"), 0);
        count.Ink = Theme.Brush(shell.Page is null ? "color.on-accent" : "color.content-subtle");
        count.Margin = new Thickness(0, 0, Theme.Number("spacing.2"), 0);
    }
}

/* The toolbar ------------------------------------------------------------- */

internal sealed class Toolbar : Surface
{
    private readonly Styled title = new("", "bodyMedium") { Weight = "labelStrong", VerticalAlignment = VerticalAlignment.Center };
    private readonly Styled subtitle = new("", "captionLarge", "color.content-secondary") { VerticalAlignment = VerticalAlignment.Center };

    public Toolbar(Shell shell)
    {
        var line = new Grid
        {
            ColumnDefinitions =
            {
                new ColumnDefinition { Width = GridLength.Auto },
                new ColumnDefinition { Width = GridLength.Auto },
                new ColumnDefinition { Width = GridLength.Auto },
                new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) },
                new ColumnDefinition { Width = GridLength.Auto },
            },
        };
        var toggleList = PanelButton(Glyph.PanelLeft, "Toggle sidebar", "Toggle sidebar  Ctrl+B", shell.ToggleList);
        var toggleThemes = PanelButton(Glyph.PanelRight, "Toggle theme panel", "Toggle theme panel", shell.ToggleThemes);
        line.Children.Add(toggleList);
        Grid.SetColumn(title, 1);
        line.Children.Add(title);
        Grid.SetColumn(subtitle, 2);
        line.Children.Add(subtitle);
        Grid.SetColumn(toggleThemes, 4);
        line.Children.Add(toggleThemes);
        Content = line;
        VerticalContentAlignment = VerticalAlignment.Center;
    }

    private static IconButton PanelButton(Glyph glyph, string label, string tip, Action action)
    {
        var button = new IconButton { Label = label, Variant = ControlVariant.Plain, Tint = WidgetTint.Neutral, Glyph = glyph };
        button.Click += (_, _) => action();
        button.Loaded += (_, _) => Tooltip.Set(button, tip, PlacementMode.Bottom);
        return button;
    }

    public void Show(Story? story)
    {
        title.Text = story?.Name ?? "Overview";
        subtitle.Text = story is null ? $"{Stories.All.Count} components" : $"{story.Examples.Count} examples";
    }

    protected override void RefreshAppearance()
    {
        Height = Theme.Number("frame.titlebar.size");
        Padding = new Thickness(Theme.Number("spacing.4"), 0, Theme.Number("spacing.4"), 0);
        BorderBrush = Theme.Brush("color.border");
        BorderThickness = new Thickness(0, 0, 0, Theme.Number("stroke.hairline"));
        title.Margin = new Thickness(Theme.Number("spacing.2") + Theme.Number("spacing.1"), 0, Theme.Number("spacing.2"), 0);
    }
}

/* The document ------------------------------------------------------------- */

/// <summary>A page's padding: the web's <c>padding: spacing-5 spacing-6 spacing-10</c>.</summary>
internal static class PageInset
{
    public static Thickness Of(Theme theme) =>
        new(theme.Number("spacing.6"), theme.Number("spacing.5"), theme.Number("spacing.6"), theme.Number("spacing.10"));
}

/// <summary>Every component at once, each by the one scene that stands for it.</summary>
internal sealed class Overview : Grid
{
    private readonly TileGrid grid = new();
    private readonly List<Grid> tiles = [];

    public Overview(IReadOnlyList<Story> stories, Action<string?> open)
    {
        foreach (var story in stories)
        {
            var name = new Button { Content = story.Name, Variant = ControlVariant.Plain, HorizontalAlignment = HorizontalAlignment.Stretch };
            name.Click += (_, _) => open(story.Id);
            // The stage takes the room the row has; the name keeps to the foot.
            var tile = new Grid { RowDefinitions = { new RowDefinition { Height = new GridLength(1, GridUnitType.Star) }, new RowDefinition { Height = GridLength.Auto } } };
            tile.Children.Add(new SceneStage(story.Preview, 8 * 16));
            SetRow(name, 1);
            tile.Children.Add(name);
            tiles.Add(tile);
            grid.Children.Add(new Card { Content = tile, Variant = CardVariant.Outlined, VerticalContentAlignment = VerticalAlignment.Stretch });
        }
        Children.Add(new ScrollViewer { Content = grid, HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled });
        Themed.Follow(this, theme =>
        {
            grid.Padding = PageInset.Of(theme);
            grid.Gap = theme.Number("spacing.3");
            foreach (var tile in tiles) tile.RowSpacing = theme.Number("spacing.2");
        });
    }
}

/// <summary>The web's <c>repeat(auto-fill, minmax(16rem, 1fr))</c>, every tile in a row as tall as the tallest.</summary>
internal sealed class TileGrid : Panel
{
    private double gap;

    public double Gap { get => gap; set { gap = value; InvalidateMeasure(); } }

    private Thickness padding;

    public Thickness Padding { get => padding; set { padding = value; InvalidateMeasure(); } }

    private int Columns(double room) => Math.Max(1, (int)((room + gap) / (16 * 16 + gap)));

    protected override Size MeasureOverride(Size available)
    {
        var room = Math.Max(0, available.Width - Padding.Left - Padding.Right);
        var columns = Columns(room);
        var width = Math.Max(0, (room - gap * (columns - 1)) / columns);
        double height = 0;
        for (var start = 0; start < Children.Count; start += columns)
        {
            double tall = 0;
            for (var i = start; i < Math.Min(start + columns, Children.Count); i++)
            {
                Children[i].Measure(new Size(width, double.PositiveInfinity));
                tall = Math.Max(tall, Children[i].DesiredSize.Height);
            }
            height += tall + gap;
        }
        return new Size(available.Width, Math.Max(0, height - gap) + Padding.Top + Padding.Bottom);
    }

    protected override Size ArrangeOverride(Size final)
    {
        var room = Math.Max(0, final.Width - Padding.Left - Padding.Right);
        var columns = Columns(room);
        var width = Math.Max(0, (room - gap * (columns - 1)) / columns);
        var y = Padding.Top;
        for (var start = 0; start < Children.Count; start += columns)
        {
            var end = Math.Min(start + columns, Children.Count);
            double tall = 0;
            for (var i = start; i < end; i++) tall = Math.Max(tall, Children[i].DesiredSize.Height);
            for (var i = start; i < end; i++)
                Children[i].Arrange(new Rect(Padding.Left + (i - start) * (width + gap), y, width, tall));
            y += tall + gap;
        }
        return final;
    }
}

/// <summary>One component: what it is for, then each example on a stage of its own.</summary>
internal sealed class Document : Grid
{
    /// <summary>Each example's stage, in order.</summary>
    public List<Card> Stages { get; } = [];

    public Document(Story story)
    {
        var body = new Stack(Orientation.Vertical, "spacing.6", PageInset.Of);
        body.Add(new Styled(story.Summary, "bodyMedium", "color.content-secondary") { MaxWidth = 640, HorizontalAlignment = HorizontalAlignment.Left });
        foreach (var example in story.Examples)
        {
            var section = new Stack(Orientation.Vertical, "spacing.2");
            section.Add(new SectionLabel(example.Title));
            // The web's `min-height: 10rem` is the card's own, its padding inside it.
            var stage = new Card { Content = new SceneStage(example.Demo, 0), Variant = CardVariant.Outlined, Size = ControlSize.Large, MinHeight = 10 * 16, VerticalContentAlignment = VerticalAlignment.Stretch };
            Stages.Add(stage);
            section.Add(stage);
            if (example.Note is string note) section.Add(new Styled(note, "captionLarge", "color.content-secondary"));
            body.Add(section);
        }
        Children.Add(new ScrollViewer { Content = body, HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled });
    }
}

/// <summary>A scene on its stage: centred in the room, at least as tall as the web's stage.</summary>
internal sealed class SceneStage : Grid
{
    public SceneStage(Scene scene, double minHeight)
    {
        MinHeight = minHeight;
        Children.Add(new SceneView(scene, align: true));
    }
}
