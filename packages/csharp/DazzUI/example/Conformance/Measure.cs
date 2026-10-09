using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI.Example;

/// <summary>
/// A specimen's readings, in the conformance vocabulary rather than
/// XAML's: a <c>CornerRadius</c> is <c>corner</c>, a <c>Background</c> is
/// <c>fill</c>, and every colour is sRGB hex. See specs/conformance.md.
///
/// A component here is a control and the template it expands into. What
/// stands for the web's one element is the part of that tree the
/// component's own box is: the primitive the size of its root that paints
/// it, the paddings and borders down to its content, the first run of text
/// and the first stack inside it — never inside a component nested in it,
/// which is a specimen of its own. A control's own brushes and padding are
/// passed to its template, so only the primitives it expands into are read.
/// </summary>
internal static class Measure
{
    private static double Round(double n) => Math.Round(n * 100) / 100;

    public static string Hex(Windows.UI.Color colour, double opacity = 1)
    {
        var a = (int)Math.Round(colour.A * opacity);
        if (a == 0) return "#00000000";
        var rgb = $"#{colour.R:x2}{colour.G:x2}{colour.B:x2}";
        return a == 255 ? rgb : $"{rgb}{a:x2}";
    }

    /// <summary>The primitives a specimen owns: its subtree, less its nested specimens'.</summary>
    private static List<DependencyObject> Own(DependencyObject root, ISet<DependencyObject> nested)
    {
        var list = new List<DependencyObject>();
        void Visit(DependencyObject item)
        {
            if (item != root && nested.Contains(item)) return;
            list.Add(item);
            for (var i = 0; i < VisualTreeHelper.GetChildrenCount(item); i++) Visit(VisualTreeHelper.GetChild(item, i));
        }
        Visit(root);
        return list;
    }

    private sealed record Paint(Brush? Background, Brush? BorderBrush, Thickness Border, CornerRadius Corner, Thickness Padding);

    /// <summary>What a primitive paints and insets, when it is one that does.</summary>
    private static Paint? PaintOf(DependencyObject item) => item switch
    {
        Border b => new(b.Background, b.BorderBrush, b.BorderThickness, b.CornerRadius, b.Padding),
        ContentPresenter p => new(p.Background, p.BorderBrush, p.BorderThickness, p.CornerRadius, p.Padding),
        Grid g => new(g.Background, g.BorderBrush, g.BorderThickness, g.CornerRadius, g.Padding),
        StackPanel s => new(s.Background, s.BorderBrush, s.BorderThickness, s.CornerRadius, s.Padding),
        Panel p => new(p.Background, null, default, default, default),
        _ => null,
    };

    private static bool SameSize(DependencyObject item, FrameworkElement root) =>
        item is FrameworkElement e &&
        Math.Abs(e.ActualWidth - root.ActualWidth) < 0.5 &&
        Math.Abs(e.ActualHeight - root.ActualHeight) < 0.5;

    /// <summary>A glyph is a run of text in an icon face here and an image on the web: an ink, no type.</summary>
    private static bool Glyph(TextBlock text) =>
        text.FontFamily?.Source?.Contains("FluentSystemIcons", StringComparison.OrdinalIgnoreCase) == true ||
        text.FontFamily?.Source?.Contains("FluentIcons", StringComparison.OrdinalIgnoreCase) == true;

    public static JsonObject Read(FrameworkElement root, ISet<DependencyObject> nested)
    {
        var own = Own(root, nested);

        // The surface: the first primitive the size of the box that paints.
        Paint? surface = null;
        foreach (var item in own)
        {
            if (item is Control || !SameSize(item, root)) continue;
            if (PaintOf(item) is { } paint && (paint.Background is not null || paint.BorderBrush is not null))
            {
                surface = paint;
                break;
            }
        }
        // A brush that is not one colour cannot be written as one.
        var painted = surface is { Background: not (null or SolidColorBrush) } or { BorderBrush: not (null or SolidColorBrush) };

        var stroke = surface?.BorderBrush is null ? 0 : surface.Border.Top;
        var shortSide = Math.Min(root.ActualWidth, root.ActualHeight) / 2;
        var corner = Math.Min(surface?.Corner.TopLeft ?? 0, shortSide);
        string? Colour(Brush? brush) => brush is SolidColorBrush solid ? Hex(solid.Color, solid.Opacity) : null;

        var texts = own.OfType<TextBlock>().Where(t => t.ActualWidth > 0 || t.Text.Length > 0).ToList();
        var text = texts.FirstOrDefault(t => !Glyph(t) && t.Text.Trim().Length > 0);
        var ink = Colour((text ?? texts.FirstOrDefault())?.Foreground);
        var stack = own.OfType<StackPanel>().FirstOrDefault();

        // The padding is the inset from the box's edge to its content, less
        // the stroke — what CSS calls padding: every padding, border and
        // margin from the root down to where the content starts.
        double left = 0, top = 0;
        var content = (DependencyObject?)stack ?? text;
        for (var item = content; item is not null; item = VisualTreeHelper.GetParent(item))
        {
            if (item != content && item is not Control && PaintOf(item) is { } paint)
            {
                left += paint.Padding.Left + paint.Border.Left;
                top += paint.Padding.Top + paint.Border.Top;
            }
            if (item == root) break;
            if (item is FrameworkElement framed) { left += framed.Margin.Left; top += framed.Margin.Top; }
        }

        return new JsonObject
        {
            ["box"] = new JsonObject { ["width"] = Round(root.ActualWidth), ["height"] = Round(root.ActualHeight) },
            ["values"] = new JsonObject
            {
                ["corner"] = painted ? null : Round(corner),
                ["stroke"] = painted ? null : Round(stroke),
                ["strokeColor"] = painted || stroke == 0 ? null : Colour(surface!.BorderBrush),
                ["fill"] = painted ? null : Colour(surface?.Background) ?? "#00000000",
                ["ink"] = ink,
                ["fontSize"] = text is null ? null : Round(text.FontSize),
                ["fontWeight"] = text is null ? null : (int)text.FontWeight.Weight,
                ["lineHeight"] = text is null || text.LineHeight == 0 ? null : Round(text.LineHeight),
                ["paddingX"] = painted ? null : Round(Math.Max(0, left - stroke)),
                ["paddingY"] = painted ? null : Round(Math.Max(0, top - stroke)),
                ["gap"] = stack is null ? null : Round(stack.Spacing),
                // A token shadow the kit draws says its name; a ThemeShadow has none.
                ["shadow"] = painted ? null : Elevation.GetShadow(root) ?? (root.Shadow is null ? "none" : "theme"),
            },
        };
    }
}
