using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI.Example;

/// <summary>
/// The layer over the whole window: where toasts stand, centred on its top
/// or bottom edge, as the web's fixed viewport places them.
/// </summary>
public static class Overlay
{
    private static readonly Dictionary<ToastViewport, FrameworkElement> owners = [];
    public static Grid Layer { get; } = new();

    public static void Attach(FrameworkElement owner, ToastViewport viewport)
    {
        owners[viewport] = owner;
        if (!Layer.Children.Contains(viewport)) Layer.Children.Add(viewport);
    }

    public static void Detach(ToastViewport viewport)
    {
        owners.Remove(viewport);
        Layer.Children.Remove(viewport);
    }

    /// <summary>The real out-of-flow viewport belongs to the provider in this scene.</summary>
    internal static bool BelongsTo(FrameworkElement element, DependencyObject scene)
    {
        if (element is not ToastViewport viewport || !owners.TryGetValue(viewport, out var owner)) return false;
        for (DependencyObject? node = owner; node is not null; node = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(node))
            if (ReferenceEquals(node, scene)) return true;
        return false;
    }
}
