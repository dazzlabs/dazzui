using Microsoft.UI.Xaml;
using Windows.UI.ViewManagement;

namespace DazzUI;

/// <summary>Windows text accessibility and WinUI's own logical control metrics.</summary>
internal static class SystemStyle
{
    private static readonly UISettings Settings = new();
    private static double textScale = Settings.TextScaleFactor;
    private static int fontsDirty;
    internal static double TextScale => Volatile.Read(ref textScale);
    internal static event Action? Changed;

    static SystemStyle()
    {
        Settings.TextScaleFactorChanged += (_, _) =>
        {
            Volatile.Write(ref textScale, Settings.TextScaleFactor);
            Interlocked.Exchange(ref fontsDirty, 1);
            Changed?.Invoke();
        };
    }

    internal static void RefreshFonts()
    {
        if (Interlocked.Exchange(ref fontsDirty, 0) != 0) PlatformFonts.Refresh();
    }

    // Read theme resources on the UI thread. The committed Windows metrics are
    // the fallback when a host has not installed XamlControlsResources yet.
    internal static double FontSize => Resource("TextControlThemeFontSize", "bodyMedium", "fontSize");
    internal static double ControlHeight => Resource("TextControlThemeMinHeight", "control.medium.size");

    private static double Resource(string key, string token, string? member = null)
    {
        if (Application.Current?.Resources.TryGetValue(key, out var value) == true && value is double number && number > 0)
            return number;
        var theme = new Theme("windows11-light");
        if (member is null) return theme.Number(token);
        var dimension = theme.Token(token).GetProperty(member);
        return dimension.GetProperty("value").GetDouble() * (dimension.GetProperty("unit").GetString() == "rem" ? 16 : 1);
    }

    // Exercises the same notification/dispatcher path without changing the
    // signed-in user's Windows accessibility preferences.
    internal static void SetTextScaleForTest(double? scale)
    {
        Volatile.Write(ref textScale, scale ?? Settings.TextScaleFactor);
        Changed?.Invoke();
    }
}
