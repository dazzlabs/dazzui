using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI.Example;

/// <summary>
/// A theme in miniature: a window of its own, drawn in that theme's colours
/// — the column, the paper, two lines of its ink, a button in its accent.
/// </summary>
public sealed class ThemePreview : Grid
{
    private readonly Border sidebar = new();
    private readonly Grid paper = new();
    private readonly Border ink = new();
    private readonly Border faint = new() { Opacity = 0.4, HorizontalAlignment = HorizontalAlignment.Stretch };
    private readonly Border accent = new() { HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Bottom };
    private readonly Grid lines = new() { RowDefinitions = { new RowDefinition { Height = GridLength.Auto }, new RowDefinition { Height = GridLength.Auto } } };
    private ThemeChoice theme = ThemeChoice.Default;
    private double corner;

    public ThemePreview(ThemeChoice theme)
    {
        this.theme = theme;
        ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(28, GridUnitType.Star) });
        ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(72, GridUnitType.Star) });
        Children.Add(sidebar);
        // The short line is two thirds of the paper in a star grid, so it
        // follows a width change without a SizeChanged pass per preview.
        var faintRow = new Grid { ColumnDefinitions = { new ColumnDefinition { Width = new GridLength(6, GridUnitType.Star) }, new ColumnDefinition { Width = new GridLength(4, GridUnitType.Star) } } };
        faintRow.Children.Add(faint);
        lines.VerticalAlignment = VerticalAlignment.Top;
        lines.Children.Add(ink);
        Grid.SetRow(faintRow, 1);
        lines.Children.Add(faintRow);
        paper.Children.Add(lines);
        paper.Children.Add(accent);
        SetColumn(paper, 1);
        Children.Add(paper);
        Loaded += (_, _) => Paint();
    }

    /// <summary>A framed preview has its own border and corner; an unframed one runs to its container's edge and draws only the rule under it.</summary>
    public bool Framed { get; init; } = true;

    public ThemeChoice Theme { get => theme; set { theme = value; if (IsLoaded) Paint(); } }

    /// <summary>The corner of the card it runs to the edge of, which its top corners follow.</summary>
    public double Corner { get => corner; set { corner = value; if (IsLoaded) Paint(); } }

    private void Paint()
    {
        var outer = Appearance.GetTheme(this);
        var vars = Theme.Theme;
        var hairline = outer.Number("stroke.hairline");
        Background = vars.Brush("color.surface");
        sidebar.Background = vars.Brush("color.surface-column");
        sidebar.BorderBrush = vars.Brush("color.border-strong");
        sidebar.BorderThickness = new Thickness(0, 0, hairline, 0);
        var pad = outer.Number(Framed ? "spacing.2" : "spacing.1.5");
        paper.Padding = new Thickness(pad);
        ink.Height = faint.Height = outer.Number("spacing.0.5");
        ink.Background = faint.Background = vars.Brush("color.content");
        ink.CornerRadius = faint.CornerRadius = new CornerRadius(1);
        lines.RowSpacing = outer.Number("spacing.1");
        accent.Width = outer.Number("spacing.6");
        accent.Height = outer.Number("spacing.2.5");
        accent.CornerRadius = new CornerRadius(outer.Number("spacing.1"));
        accent.Background = vars.Brush("color.primary.500");
        if (Framed)
        {
            var radius = outer.Number("spacing.1.5");
            CornerRadius = new CornerRadius(radius);
            sidebar.CornerRadius = new CornerRadius(radius, 0, 0, radius);
            BorderBrush = vars.Brush("color.border-strong");
            BorderThickness = new Thickness(hairline);
        }
        else
        {
            CornerRadius = new CornerRadius(corner, corner, 0, 0);
            sidebar.CornerRadius = new CornerRadius(corner, 0, 0, 0);
            BorderBrush = outer.Brush("color.border");
            BorderThickness = new Thickness(0, 0, 0, hairline);
        }
    }
}
