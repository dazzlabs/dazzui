using Microsoft.UI.Xaml;

namespace DazzUI.Example;

/// <summary>
/// Keeps a panel in the theme it stands in. The kit's <see cref="Surface"/>
/// does this for itself, but a surface is a content control, and WinUI walks
/// a content control's element twice when it leaves the window — once as the
/// property's value, once as the presenter's child — so a tree of them nested
/// deep takes twice as long to take down at every level. The example's own
/// containers are panels, which it walks once, themed from here.
/// </summary>
public static class Themed
{
    private static event Action? Changed;

    /// <summary>Tells every panel in the window that the theme has changed.</summary>
    public static void Refresh() => Changed?.Invoke();

    /// <summary>Applies the theme <paramref name="element"/> stands in when it is loaded, and again whenever the theme changes.</summary>
    public static void Follow(FrameworkElement element, Action<Theme> apply)
    {
        void Run() => apply(Appearance.GetTheme(element));
        element.Loaded += (_, _) => { Changed -= Run; Changed += Run; Run(); };
        element.Unloaded += (_, _) => Changed -= Run;
    }
}
