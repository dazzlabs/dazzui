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
    /// <summary>Applies an inherited theme and follows its runtime sizing.</summary>
    public static void Follow(FrameworkElement element, Action<Theme> apply) => Appearance.Follow(element, apply);
}
