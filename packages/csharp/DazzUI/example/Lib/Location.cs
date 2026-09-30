namespace DazzUI.Example;

/// <summary>
/// The page and theme the app was launched on, the way the React example
/// reads them from its address: <c>--page button --theme ember-dark</c> on
/// the command line, or nothing for the defaults.
/// </summary>
public static class Location
{
    public static (string? Page, ThemeChoice Theme) Read(IReadOnlyList<string> args)
    {
        string? Option(string name)
        {
            var at = args.ToList().IndexOf(name);
            return at >= 0 && at + 1 < args.Count ? args[at + 1] : null;
        }
        var page = Option("--page") is { Length: > 0 } id ? id : null;
        var theme = Option("--theme") ?? "";
        var cut = theme.LastIndexOf('-');
        if (cut < 0) return (page, ThemeChoice.Default);
        var family = theme[..cut];
        var mode = theme[(cut + 1)..];
        var known = Themes.Groups.Any(group => group.Families.Any(item => item.Value == family));
        if (!known || mode is not ("light" or "dark")) return (page, ThemeChoice.Default);
        return (page, new ThemeChoice(family, mode == "dark"));
    }
}
