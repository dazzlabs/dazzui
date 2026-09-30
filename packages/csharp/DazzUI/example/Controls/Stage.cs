using System.Globalization;
using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;

namespace DazzUI.Example;

/*
 * The four stage components a story arranges with — Row, Col, Frame, Text —
 * read the way `example/src/components/stage.tsx` draws them on the web.
 */

/// <summary>A length as the web writes it: pixels, <c>rem</c> against the 16px root every port shares, or a share of the room there is.</summary>
public readonly record struct Length(double Value, bool IsFraction)
{
    public static Length? Parse(JsonNode? value)
    {
        if (value.Number() is double number) return new(number, false);
        if (value.String() is not string text) return null;
        static double Read(string digits) => double.Parse(digits, CultureInfo.InvariantCulture);
        if (text.EndsWith("rem", StringComparison.Ordinal)) return new(Read(text[..^3]) * 16, false);
        if (text.EndsWith("px", StringComparison.Ordinal)) return new(Read(text[..^2]), false);
        if (text.EndsWith('%')) return new(Read(text[..^1]) / 100, true);
        return double.TryParse(text, NumberStyles.Float, CultureInfo.InvariantCulture, out var plain) ? new(plain, false) : null;
    }

    /// <summary>The length in the room offered: never more than the room, as <c>max-width: 100%</c> keeps it.</summary>
    public double Resolve(double room) =>
        IsFraction ? (double.IsInfinity(room) ? 0 : room * Value) : Math.Min(Value, room);

    /// <summary>The length in points, when it is one.</summary>
    public double? Points => IsFraction ? null : Value;
}

public static class Stage
{
    /// <summary>A spacing step as the web writes it (<c>"3"</c>, <c>"0-5"</c>), in this theme.</summary>
    public static double Spacing(Theme theme, string? step) => step switch
    {
        null => theme.Number("spacing.3"),
        "px" => theme.Number("spacing.px"),
        _ => theme.Number("spacing." + step.Replace('-', '.')),
    };

    /// <summary>A run of plain text, in whatever ink and size it stands in.</summary>
    public static TextBlock Words(string text) => new() { Text = text, TextWrapping = TextWrapping.Wrap };

    public static UIElement Row(Node node) => new StagePanel(node, StageLayout.Row);

    public static UIElement Col(Node node) => new StagePanel(node, StageLayout.Col);

    public static UIElement Frame(Node node) => new StageFrame(node);

    /// <summary>A run of text that may hold components inline, as a paragraph would.</summary>
    public static UIElement Text(Node node) => new StageText(node);
}

public enum StageLayout { Row, Col }

/// <summary>
/// A <c>Row</c> or a <c>Col</c>: its children laid out as the web's flex box
/// lays them — a row wraps, or with <c>justify="between"</c> keeps to one
/// line with the ends at the edges; a column stacks, stretching its
/// children unless told otherwise, and is as wide as its widest child.
/// </summary>
public sealed class StagePanel : Panel
{
    private readonly StageLayout layout;
    private readonly string align;
    private readonly bool between;
    private readonly string? step;
    private readonly Length? width;

    public StagePanel(Node node, StageLayout layout)
    {
        this.layout = layout;
        align = node.String("align") ?? (layout == StageLayout.Row ? "center" : "stretch");
        between = node.String("justify") == "between";
        step = node.String("gap");
        width = Length.Parse(node["width"]);
        foreach (var child in node.Kids()) Children.Add(child);
        // A theme can move the spacing scale.
        Themed.Follow(this, _ => InvalidateMeasure());
    }

    /// <summary>The gap, in the theme the panel stands in now.</summary>
    private double Gap => Stage.Spacing(Appearance.GetTheme(this), step);

    private double Room(Size available) => width?.Resolve(available.Width) ?? available.Width;

    protected override Size MeasureOverride(Size available)
    {
        var room = Room(available);
        if (layout == StageLayout.Col)
        {
            double wide = 0, tall = 0;
            foreach (var child in Children)
            {
                child.Measure(new Size(room, double.PositiveInfinity));
                wide = Math.Max(wide, child.DesiredSize.Width);
                tall += child.DesiredSize.Height;
            }
            tall += Gap * Math.Max(0, Children.Count - 1);
            return new Size(width is null ? Math.Min(wide, room) : room, tall);
        }
        if (between) return Between(room, double.NaN, arrange: false);
        return Flow(room, arrange: false);
    }

    protected override Size ArrangeOverride(Size final)
    {
        var room = width is null ? final.Width : Math.Min(Room(final), final.Width);
        if (layout == StageLayout.Col)
        {
            double y = 0;
            foreach (var child in Children)
            {
                var size = child.DesiredSize;
                var w = align == "stretch" ? room : Math.Min(size.Width, room);
                var x = align switch { "center" => (room - w) / 2, "end" => room - w, _ => 0 };
                child.Arrange(new Rect(x, y, w, size.Height));
                y += size.Height + Gap;
            }
            return final;
        }
        if (between) Between(room, final.Height, arrange: true);
        else Flow(room, arrange: true);
        return final;
    }

    /// <summary>The web's <c>flex-wrap: wrap</c>: side by side, onto the next line when the row runs out.</summary>
    private Size Flow(double room, bool arrange)
    {
        var gap = Gap;
        var lines = new List<List<UIElement>> { new() };
        double x = 0;
        foreach (var child in Children)
        {
            if (!arrange) child.Measure(new Size(room, double.PositiveInfinity));
            var w = child.DesiredSize.Width;
            if (x > 0 && x + w > room) { lines.Add([]); x = 0; }
            lines[^1].Add(child);
            x += w + gap;
        }
        double y = 0, used = 0;
        foreach (var line in lines)
        {
            var tall = line.Count == 0 ? 0 : line.Max(child => child.DesiredSize.Height);
            double at = 0;
            foreach (var child in line)
            {
                var size = child.DesiredSize;
                if (arrange)
                {
                    // A rule between items runs the height of the line.
                    var h = align == "stretch" || child is Divider { Orientation: Orientation.Vertical } ? tall : size.Height;
                    var top = align switch { "start" or "stretch" => 0, "end" => tall - h, _ => (tall - h) / 2 };
                    child.Arrange(new Rect(at, y + top, Math.Min(size.Width, room), h));
                }
                at += size.Width + gap;
            }
            used = Math.Max(used, at - gap);
            y += tall + gap;
        }
        return new Size(width is null ? Math.Min(Math.Max(0, used), room) : room, Math.Max(0, y - gap));
    }

    /// <summary>The web's <c>justify-content: space-between</c> on one line, each child shrinking in proportion when the line is short.</summary>
    private Size Between(double room, double height, bool arrange)
    {
        var gap = Gap;
        var count = Children.Count;
        if (!arrange)
        {
            foreach (var child in Children) child.Measure(new Size(double.PositiveInfinity, double.PositiveInfinity));
            var natural = Children.Select(child => child.DesiredSize.Width).ToArray();
            var sum = natural.Sum();
            var over = sum + gap * Math.Max(0, count - 1) - room;
            if (over > 0 && sum > 0)
                for (var i = 0; i < count; i++)
                    Children[i].Measure(new Size(Math.Max(0, natural[i] - over * natural[i] / sum), double.PositiveInfinity));
        }
        var widths = Children.Select(child => child.DesiredSize.Width).ToArray();
        var tall = count == 0 ? 0 : Children.Max(child => child.DesiredSize.Height);
        if (arrange)
        {
            var line = double.IsNaN(height) ? tall : Math.Max(tall, height);
            var spare = Math.Max(0, room - widths.Sum());
            var space = count > 1 ? Math.Max(gap, spare / (count - 1)) : 0;
            double x = 0;
            for (var i = 0; i < count; i++)
            {
                var child = Children[i];
                var h = align == "stretch" ? line : child.DesiredSize.Height;
                var top = align switch { "start" or "stretch" => 0, "end" => line - h, _ => (line - h) / 2 };
                child.Arrange(new Rect(x, top, widths[i], h));
                x += widths[i] + space;
            }
        }
        var used = widths.Sum() + gap * Math.Max(0, count - 1);
        return new Size(width is null ? Math.Min(used, room) : room, tall);
    }
}

/// <summary>A window, for what only makes sense inside one: a sidebar, a splitter.</summary>
public sealed class StageFrame : Grid
{
    private readonly Length? width;
    private readonly Length? height;

    public StageFrame(Node node)
    {
        width = Length.Parse(node.Has("width") ? node["width"] : "36rem");
        height = Length.Parse(node.Has("height") ? node["height"] : "16rem");
        foreach (var child in node.Kids())
        {
            ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
            SetColumn((FrameworkElement)child, Children.Count);
            Children.Add(child);
        }
        Themed.Follow(this, theme =>
        {
            Background = theme.Brush("color.surface");
            BorderBrush = theme.Brush("color.border");
            BorderThickness = new Thickness(theme.Number("stroke.hairline"));
            CornerRadius = new CornerRadius(theme.Number("radius.large"));
        });
    }

    protected override Size MeasureOverride(Size available)
    {
        var w = width?.Resolve(available.Width) ?? available.Width;
        var h = height?.Resolve(available.Height) ?? available.Height;
        if (double.IsInfinity(h)) h = height?.Points ?? 0;
        base.MeasureOverride(new Size(w, h));
        return new Size(w, h);
    }

    protected override Size ArrangeOverride(Size final)
    {
        var w = Math.Min(final.Width, width?.Resolve(final.Width) ?? final.Width);
        var h = Math.Min(final.Height, height?.Resolve(final.Height) ?? final.Height);
        base.ArrangeOverride(new Size(w, h));
        return new Size(w, h);
    }
}

/// <summary>A run of text: its words, and any component set in the line with them.</summary>
public sealed class StageText : Grid
{
    private readonly Node node;
    private readonly List<(TextBlock Words, JsonNode? Source)> runs = [];
    private readonly List<Control> inline = [];

    public StageText(Node node)
    {
        this.node = node;
        HorizontalAlignment = HorizontalAlignment.Left;
        var line = new StackPanel { Orientation = Orientation.Horizontal };
        foreach (var child in node.Children)
        {
            if (child.String() is not null || Nodes.IsLive(child))
            {
                var words = Stage.Words("");
                runs.Add((words, child));
                line.Children.Add(words);
            }
            else if (Nodes.Build(child, node.Context) is UIElement element)
            {
                if (element is FrameworkElement framed) framed.VerticalAlignment = VerticalAlignment.Center;
                if (element is Control control) inline.Add(control);
                line.Children.Add(element);
            }
        }
        // A run of words alone wraps; one holding a component keeps to a line.
        if (line.Children.Count == 1 && runs.Count == 1)
        {
            line.Children.Clear();
            Children.Add(runs[0].Words);
        }
        else Children.Add(line);
        node.Live(this, () => { foreach (var (words, source) in runs) words.Text = node.Context.Resolve(source).Text(); });
        Themed.Follow(this, Apply);
    }

    /// <summary>The run's ink and weight; a glyph inside takes the ink too.</summary>
    private void Apply(Theme theme)
    {
        var tint = node.String("tint");
        var ink = tint is not null ? theme.Brush($"color.{tint}.500")
            : node.String("tone") == "secondary" ? theme.Brush("color.content-secondary")
            : null;
        foreach (var (words, _) in runs)
        {
            if (ink is null) words.ClearValue(TextBlock.ForegroundProperty);
            else words.Foreground = ink;
            if (node.Flag("strong"))
                words.FontWeight = new Windows.UI.Text.FontWeight { Weight = (ushort)theme.Token("labelStrong").GetProperty("fontWeight").GetInt32() };
        }
        if (ink is not null) foreach (var control in inline.OfType<Icon>()) control.Foreground = ink;
    }
}
