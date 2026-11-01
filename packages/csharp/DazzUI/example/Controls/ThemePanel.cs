using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Windows.System;

namespace DazzUI.Example;

/// <summary>The theme, and nothing else: which half of it, then which family.</summary>
public sealed class ThemePanel : Surface
{
    private readonly Shell shell;
    private readonly Toggle mode = new() { Variant = ControlVariant.Plain, Size = ControlSize.Tiny };
    private readonly Grid head = new();
    private readonly Toggle systemSizing = new() { Content = "Follow system sizing", Size = ControlSize.Small };
    private readonly List<(Grid Grid, int Count)> grids = [];
    private readonly List<ThemeTile> tiles = [];

    public ThemePanel(Shell shell)
    {
        this.shell = shell;
        head.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        head.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        head.Children.Add(new Styled("Theme", "bodyMedium") { Weight = "labelStrong", VerticalAlignment = VerticalAlignment.Center });
        mode.Click += (_, _) => shell.Choose(shell.Theme with { Dark = !shell.Theme.Dark });
        Grid.SetColumn(mode, 1);
        head.Children.Add(mode);

        var list = new Stack(Orientation.Vertical, "spacing.0", theme => new Thickness(0, theme.Number("spacing.2"), 0, theme.Number("spacing.2")));
        systemSizing.Click += (_, _) => shell.ChooseSizing(systemSizing.IsChecked == true ? ThemeSizing.System : ThemeSizing.Design);
        list.Add(systemSizing);
        foreach (var group in Themes.Groups)
        {
            var section = new Stack(Orientation.Vertical, "spacing.2", theme => new Thickness(theme.Number("spacing.4"), theme.Number("spacing.2"), theme.Number("spacing.4"), theme.Number("spacing.2")));
            var label = new SectionLabel(group.Title) { FontSize = 11 };
            section.Add(label);
            var grid = new Grid();
            for (var i = 0; i < 3; i++) grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
            for (var i = 0; i < group.Families.Count; i++)
            {
                if (i % 3 == 0) grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
                var tile = new ThemeTile(group.Families[i], family => shell.Choose(shell.Theme with { Family = family }));
                Grid.SetRow(tile, i / 3);
                Grid.SetColumn(tile, i % 3);
                grid.Children.Add(tile);
                tiles.Add(tile);
            }
            section.Add(grid);
            grids.Add((grid, group.Families.Count));
            list.Add(section);
        }

        var column = new Grid { RowDefinitions = { new RowDefinition { Height = GridLength.Auto }, new RowDefinition { Height = new GridLength(1, GridUnitType.Star) } } };
        column.Children.Add(head);
        var scroll = new ScrollViewer { Content = list, HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled };
        Grid.SetRow(scroll, 1);
        column.Children.Add(scroll);
        Content = column;
        VerticalContentAlignment = VerticalAlignment.Stretch;
    }

    /// <summary>Marks the theme the window is in.</summary>
    public void Refresh()
    {
        systemSizing.IsChecked = shell.Sizing == ThemeSizing.System;
        var dark = shell.Theme.Dark;
        mode.IsChecked = dark;
        mode.Content = Glyphs.Draw(dark ? "WeatherMoon20Regular" : "WeatherSunny20Regular", 16);
        var tip = dark ? "Switch to light" : "Switch to dark";
        AutomationProperties.SetName(mode, tip);
        if (mode.IsLoaded) Tooltip.Set(mode, tip);
        else mode.Loaded += (_, _) => Tooltip.Set(mode, tip);
        foreach (var tile in tiles) tile.Show(shell.Theme);
    }

    protected override void RefreshAppearance()
    {
        Background = Theme.Brush("color.surface-column");
        BorderBrush = Theme.Brush("color.border");
        BorderThickness = new Thickness(Theme.Number("stroke.hairline"), 0, 0, 0);
        head.Height = Theme.Number("frame.titlebar.size");
        head.Padding = new Thickness(Theme.Number("spacing.4"), 0, Theme.Number("spacing.4"), 0);
        head.BorderBrush = Theme.Brush("color.border");
        head.BorderThickness = new Thickness(0, 0, 0, Theme.Number("stroke.hairline"));
        foreach (var (grid, _) in grids) grid.ColumnSpacing = grid.RowSpacing = Theme.Number("spacing.2");
    }
}

/// <summary>A family, as a card: the theme in miniature over its name.</summary>
internal sealed class ThemeTile : Surface
{
    private readonly ThemeFamily family;
    private readonly ThemePreview preview;
    private readonly Styled name;
    private bool selected;
    private bool hovered;
    private bool focused;

    public ThemeTile(ThemeFamily family, Action<string> choose)
    {
        this.family = family;
        preview = new ThemePreview(new ThemeChoice(family.Value)) { Framed = false };
        name = new Styled(family.Title, "labelSmall") { Trimmed = true, HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
        var column = new Grid { RowDefinitions = { new RowDefinition { Height = GridLength.Auto }, new RowDefinition { Height = GridLength.Auto } } };
        column.Children.Add(preview);
        Grid.SetRow(name, 1);
        column.Children.Add(name);
        Content = column;
        IsTabStop = true;
        // A tile is a twelve-pixel card; the platform's focus visual would
        // give it a capsule, so the card marks focus in its own border.
        UseSystemFocusVisuals = false;
        GotFocus += (_, _) => { focused = true; Paint(); };
        LostFocus += (_, _) => { focused = false; Paint(); };
        AutomationProperties.SetName(this, family.Title);
        Tapped += (_, _) => choose(family.Value);
        KeyDown += (_, args) =>
        {
            if (args.Key is not (VirtualKey.Enter or VirtualKey.Space)) return;
            choose(family.Value);
            args.Handled = true;
        };
        PointerEntered += (_, _) => { hovered = true; Paint(); };
        PointerExited += (_, _) => { hovered = false; Paint(); };
    }

    public void Show(ThemeChoice current)
    {
        selected = current.Family == family.Value;
        preview.Theme = current with { Family = family.Value };
        if (IsLoaded) Paint();
    }

    protected override void RefreshAppearance()
    {
        name.Height = Theme.Number("spacing.6");
        name.Padding = new Thickness(Theme.Number("spacing.1.5"), 0, Theme.Number("spacing.1.5"), 0);
        preview.Height = Theme.Number("spacing.10");
        Paint();
    }

    /// <summary>The option card's recipe: tinted and outlined in the accent when chosen.</summary>
    private void Paint()
    {
        // One radius for every family, whatever the family's own scale says.
        const double radius = 8;
        var hairline = Theme.Number("stroke.control");
        CornerRadius = new CornerRadius(radius);
        BorderThickness = new Thickness(hairline);
        var state = hovered ? "hovered" : "normal";
        Background = Theme.Recipe(selected ? ControlVariant.Tinted : ControlVariant.Normal, WidgetTint.Primary, "surface", state);
        BorderBrush = focused
            ? Theme.Alpha($"color.primary.{Theme.Number("focus.ring.shade")}", Theme.Number("focus.ring.alpha"))
            : selected ? Theme.Recipe(ControlVariant.Outlined, WidgetTint.Primary, "border", state) : Theme.Brush("color.border");
        preview.Corner = Math.Max(0, radius - hairline);
    }
}
