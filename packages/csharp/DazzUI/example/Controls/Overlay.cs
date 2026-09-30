using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI.Example;

/// <summary>
/// The layer over the whole window: where toasts stand, centred on its top
/// or bottom edge, as the web's fixed viewport places them.
/// </summary>
public static class Overlay
{
    private static ToastViewport? top;
    private static ToastViewport? bottom;

    public static Grid Layer { get; } = new();

    public static void Toast(ToastOptions options, bool atTop)
    {
        var viewport = atTop ? top ??= Viewport(VerticalAlignment.Top) : bottom ??= Viewport(VerticalAlignment.Bottom);
        viewport.Add(options);
    }

    private static ToastViewport Viewport(VerticalAlignment edge)
    {
        var viewport = new ToastViewport { HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = edge };
        viewport.Loaded += (_, _) =>
        {
            var gap = viewport.Theme.Number("spacing.4");
            viewport.Margin = new Thickness(gap);
        };
        Layer.Children.Add(viewport);
        return viewport;
    }
}
