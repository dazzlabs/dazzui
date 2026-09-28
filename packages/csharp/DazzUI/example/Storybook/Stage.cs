using DazzUI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI.Playground.Storybook;

// Bare-story arrangement; no documentation or decoration.
public static class Stage
{
    public static StackPanel Column(Theme theme, params UIElement[] children) => Arrange(theme, Orientation.Vertical, children);
    public static Panel Row(Theme theme, params UIElement[] children)
    {
        var panel = new FlowPanel { Gap = theme.Number("spacing.3") };
        foreach (var child in children) panel.Children.Add(child);
        return panel;
    }
    private static StackPanel Arrange(Theme theme, Orientation orientation, UIElement[] children)
    {
        var panel = new StackPanel { Orientation = orientation, Spacing = theme.Number("spacing.3") };
        foreach (var child in children) panel.Children.Add(child);
        return panel;
    }
}

// Specimens wrap at the available window width; no fixed gallery breakpoint.
internal sealed class FlowPanel : Panel
{
    public double Gap { get; init; }
    protected override Windows.Foundation.Size MeasureOverride(Windows.Foundation.Size availableSize)
    {
        foreach (var child in Children) child.Measure(availableSize);
        return Layout(availableSize.Width, false);
    }
    protected override Windows.Foundation.Size ArrangeOverride(Windows.Foundation.Size finalSize)
    {
        Layout(finalSize.Width, true);
        return finalSize;
    }
    private Windows.Foundation.Size Layout(double width, bool arrange)
    {
        double x = 0, y = 0, rowHeight = 0, usedWidth = 0;
        foreach (var child in Children)
        {
            var size = child.DesiredSize;
            if (x > 0 && x + size.Width > width)
            {
                x = 0;
                y += rowHeight + Gap;
                rowHeight = 0;
            }
            if (arrange) child.Arrange(new Windows.Foundation.Rect(x, y, size.Width, size.Height));
            usedWidth = Math.Max(usedWidth, x + size.Width);
            x += size.Width + Gap;
            rowHeight = Math.Max(rowHeight, size.Height);
        }
        return new Windows.Foundation.Size(usedWidth, y + rowHeight);
    }
}
