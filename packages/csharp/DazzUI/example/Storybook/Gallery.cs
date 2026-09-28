using DazzUI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI.Playground.Storybook;

// Documentation furniture is only used by Overview stories.
public static class Gallery
{
    public static UIElement Overview(Theme theme, string title, string description, UIElement specimen, params UIElement[] sections)
    {
        var page = Stage.Column(theme, theme.Text(title, "headlineMedium"), theme.Text(description), Section(theme, "Preview", specimen));
        page.Spacing = theme.Number("spacing.6");
        foreach (var section in sections) page.Children.Add(section);
        return page;
    }
    public static UIElement Section(Theme theme, string title, UIElement specimen)
    {
        var presenter = new ContentControl { Content = specimen, HorizontalContentAlignment = HorizontalAlignment.Left, IsTabStop = false };
        return Stage.Column(theme, theme.Text(title, "titleSmall"), new Divider(), new Card(presenter, theme));
    }
}
