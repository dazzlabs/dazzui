using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;

namespace DazzUI.Example;

/// <summary>
/// The name, and which of its implementations this is. Every port has an
/// example of its own, and a window of one looks like a window of the next.
/// </summary>
public sealed class Brand : Surface
{
    private readonly Border mark = new();
    private readonly StackPanel line;

    public Brand(Action action)
    {
        line = new StackPanel
        {
            Orientation = Orientation.Horizontal,
            Children =
            {
                mark,
                new Styled("DazzUI", "titleSmall") { VerticalAlignment = VerticalAlignment.Center },
                new Badge { Content = "WinUI 3", Size = ControlSize.Small, VerticalAlignment = VerticalAlignment.Center },
            },
        };
        Content = line;
        HorizontalAlignment = HorizontalAlignment.Left;
        IsTabStop = true;
        UseSystemFocusVisuals = true;
        AutomationProperties.SetName(this, "DazzUI");
        Tapped += (_, _) => action();
    }

    protected override void RefreshAppearance()
    {
        line.Spacing = Theme.Number("spacing.2");
        mark.Width = mark.Height = Theme.Number("spacing.5");
        mark.CornerRadius = new CornerRadius(Theme.Number("radius.small"));
        mark.Background = Theme.Brush("color.primary.500");
    }
}
