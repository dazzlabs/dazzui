using System.Collections.ObjectModel;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Markup;
using Windows.Foundation;

namespace DazzUI;

/// <summary>A value and accessible name, optionally previewing another theme.</summary>
public sealed record PreferenceThemeOption(string Value, string Label, string? Theme = null);

/// <summary>A wrapping run of theme previews. Applying the choice belongs to the host.</summary>
public sealed class PreferenceThemeSwatches : Surface
{
    private readonly SwatchPanel panel = new();
    private readonly List<(PreferenceThemeOption Option, ThemeSwatch Swatch)> swatches = [];
    private readonly string groupName = Guid.NewGuid().ToString("N");
    private bool syncing;

    public ObservableCollection<PreferenceThemeOption> Options { get; } = [];
    public static readonly DependencyProperty ValueProperty = DependencyProperty.Register(
        nameof(Value), typeof(string), typeof(PreferenceThemeSwatches),
        new PropertyMetadata(null, (sender, _) => ((PreferenceThemeSwatches)sender).Sync()));
    public string? Value { get => (string?)GetValue(ValueProperty); set => SetValue(ValueProperty, value); }
    public event EventHandler<string>? ValueChanged;

    public PreferenceThemeSwatches()
    {
        panel.Spacing = Theme.Number("spacing.1.5");
        Content = panel;
        HorizontalAlignment = HorizontalAlignment.Right;
        Options.CollectionChanged += (_, _) => Rebuild();
    }

    private void Rebuild()
    {
        panel.Children.Clear();
        swatches.Clear();
        foreach (var option in Options)
        {
            var swatch = new ThemeSwatch(option) { GroupName = groupName };
            swatch.Checked += (_, _) =>
            {
                if (syncing) return;
                Value = option.Value;
                ValueChanged?.Invoke(this, option.Value);
                Sync();
            };
            swatches.Add((option, swatch));
            panel.Children.Add(swatch);
        }
        Sync();
    }

    private void Sync()
    {
        if (syncing) return;
        syncing = true;
        try { foreach (var (option, swatch) in swatches) swatch.IsChecked = option.Value == Value; }
        finally { syncing = false; }
    }

    protected override void RefreshAppearance()
    {
        panel.Spacing = Theme.Number("spacing.1.5");
        foreach (var (_, swatch) in swatches) swatch.Paint();
    }
}

internal sealed class ThemeSwatch : RadioButton
{
    private const string SwatchTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='RadioButton'><ContentPresenter Content='{TemplateBinding Content}' HorizontalContentAlignment='Stretch' VerticalContentAlignment='Stretch'/></ControlTemplate>";
    private readonly PreferenceThemeOption option;
    private readonly Border window = new();
    private readonly Border ring = new() { IsHitTestVisible = false };
    private readonly Border gap = new() { IsHitTestVisible = false };

    internal ThemeSwatch(PreferenceThemeOption option)
    {
        this.option = option;
        Template = (ControlTemplate)XamlReader.Load(SwatchTemplate);
        MinWidth = MinHeight = 0;
        Padding = new Thickness(0);
        UseLayoutRounding = false;
        Content = new Grid { Children = { gap, ring, window } };
        AutomationProperties.SetName(this, option.Label);
        Appearance.Bind(this, Paint);
        Loaded += (_, _) => { Tooltip.Set(this, option.Label); Paint(); };
        Checked += (_, _) => Paint();
        Unchecked += (_, _) => Paint();
        Paint();
    }

    internal void Paint()
    {
        var outer = Appearance.GetTheme(this);
        var id = option.Theme ?? option.Value;
        var vars = new Theme(id.Length == 0 ? "studio-light" : id);
        var rule = vars.Number("stroke.hairline");
        Width = vars.Number("control.small.size") * 1.5;
        Height = vars.Number("control.small.size");
        Skin.Focus(this, outer);
        CornerRadius = new CornerRadius(outer.Number("radius.small"));
        window.Background = vars.Brush("color.surface");
        window.BorderBrush = vars.Brush("color.border-strong");
        window.BorderThickness = new Thickness(rule);
        window.CornerRadius = new CornerRadius(vars.Number("radius.small"));
        var body = new Grid();
        body.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(30, GridUnitType.Star) });
        body.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(70, GridUnitType.Star) });
        body.Children.Add(new Border { Background = vars.Brush("color.surface-column"),
            BorderBrush = vars.Brush("color.border-strong"), BorderThickness = new Thickness(0, 0, rule, 0),
            CornerRadius = new CornerRadius(Math.Max(0, vars.Number("radius.small") - rule), 0, 0, Math.Max(0, vars.Number("radius.small") - rule)) });
        var pad = vars.Number("spacing.1");
        var paper = new Grid { Margin = new Thickness(pad) };
        paper.Children.Add(new Border { Height = vars.Number("spacing.0.5"), Background = vars.Brush("color.content"),
            CornerRadius = new CornerRadius(vars.Number("spacing.px")), VerticalAlignment = VerticalAlignment.Top });
        paper.Children.Add(new Border { Width = Math.Max(0, ((Width - rule * 2) * 0.7 - pad * 2) * 0.6),
            Height = vars.Number("spacing.1.5"), Background = vars.Brush("color.primary.500"),
            CornerRadius = new CornerRadius(vars.Number("spacing.px")), HorizontalAlignment = HorizontalAlignment.Left,
            VerticalAlignment = VerticalAlignment.Bottom });
        Grid.SetColumn(paper, 1);
        body.Children.Add(paper);
        window.Child = body;
        var space = outer.Number("stroke.hairline");
        var stroke = outer.Number("stroke.control");
        ring.Margin = new Thickness(-space - stroke);
        ring.BorderThickness = new Thickness(stroke);
        ring.BorderBrush = outer.Recipe(ControlVariant.Outlined, WidgetTint.Primary, "border");
        ring.CornerRadius = new CornerRadius(outer.Number("radius.small") + space + stroke);
        gap.Margin = new Thickness(-space);
        gap.BorderThickness = new Thickness(space);
        gap.BorderBrush = outer.Brush("color.surface");
        gap.CornerRadius = new CornerRadius(outer.Number("radius.small") + space);
        ring.Visibility = gap.Visibility = IsChecked == true ? Visibility.Visible : Visibility.Collapsed;
    }
}

internal sealed class SwatchPanel : Panel
{
    private double spacing;
    internal double Spacing
    {
        get => spacing;
        set { if (spacing == value) return; spacing = value; InvalidateMeasure(); }
    }

    private List<List<UIElement>> Rows(double width)
    {
        var rows = new List<List<UIElement>>();
        var row = new List<UIElement>();
        double run = 0;
        foreach (var child in Children)
        {
            if (row.Count > 0 && run + Spacing + child.DesiredSize.Width > width)
            { rows.Add(row); row = []; run = 0; }
            run += (row.Count > 0 ? Spacing : 0) + child.DesiredSize.Width;
            row.Add(child);
        }
        if (row.Count > 0) rows.Add(row);
        return rows;
    }

    protected override Size MeasureOverride(Size available)
    {
        foreach (var child in Children) child.Measure(new Size(double.PositiveInfinity, double.PositiveInfinity));
        var rows = Rows(available.Width);
        return new Size(rows.Select(row => row.Sum(child => child.DesiredSize.Width) + (row.Count - 1) * Spacing).DefaultIfEmpty().Max(),
            rows.Sum(row => row.Max(child => child.DesiredSize.Height)) + Math.Max(0, rows.Count - 1) * Spacing);
    }

    protected override Size ArrangeOverride(Size final)
    {
        double y = 0;
        foreach (var row in Rows(final.Width))
        {
            double x = final.Width - row.Sum(child => child.DesiredSize.Width) - (row.Count - 1) * Spacing;
            foreach (var child in row)
            {
                child.Arrange(new Rect(x, y, child.DesiredSize.Width, child.DesiredSize.Height));
                x += child.DesiredSize.Width + Spacing;
            }
            y += row.Max(child => child.DesiredSize.Height) + Spacing;
        }
        return final;
    }
}
