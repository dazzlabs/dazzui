using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using UI = DazzUI;

namespace DazzUI.Playground.Stories;

public static class TokenExamples
{
    private static UIElement Page(UI.Theme theme, string title, string description, params UIElement[] rows) => new UI.ThemeScope { Theme = theme, Content = Gallery.Overview(theme, title, description, Stage.Column(theme, rows)) };
    public static UIElement Color(UI.Theme t, string story)
    {
        var roles = t.TokenNames.Where(name => name.StartsWith("color.") && (story == "Palette" ? char.IsDigit(name.Last()) : name.Count(c => c == '.') == 1));
        return Page(t, "Color", "Semantic roles and tint ramps resolved for the current theme.", roles.Select(name =>
            (UIElement)Stage.Row(t, new Border { Background = t.Brush(name), Width = t.Number("spacing.10"), Height = t.Number("spacing.6"), CornerRadius = new CornerRadius(t.Number("radius.small")), BorderBrush = t.Brush("color.border"), BorderThickness = new Thickness(t.Number("stroke.hairline")) }, t.Text(name, "bodySmall"), t.Text(t.Color(name).ToString(), "bodySmall"))).ToArray());
    }
    public static UIElement Typography(UI.Theme t, string story) => Page(t, "Typography", "The shared type scale, from compact labels to page headlines.",
        t.TokenNames.Where(name => t.Token(name).ValueKind == System.Text.Json.JsonValueKind.Object && t.Token(name).TryGetProperty("fontSize", out _) && !name.Contains('.')).Select(name => (UIElement)Stage.Column(t, t.Text(name, "labelSmall"), t.Text("The quick brown fox · 快速翻译", name))).ToArray());
    public static UIElement Spacing(UI.Theme t, string story) => Page(t, "Spacing", "The grid steps that set gaps, padding and proportions across components.",
        t.TokenNames.Where(name => name.StartsWith("spacing.") && t.Number(name) <= t.Number("spacing.20")).Select(name => (UIElement)Stage.Row(t, t.Text(name, "bodySmall"), new Border { Width = Math.Max(t.Number("stroke.hairline"), t.Number(name)), Height = t.Number("spacing.4"), Background = t.Recipe(UI.ControlVariant.Filled, UI.WidgetTint.Primary, "surface") }, t.Text($"{t.Number(name):0.##} px", "bodySmall"))).ToArray());
    public static UIElement Radius(UI.Theme t, string story) => Page(t, "Radius", "Corners belong to semantic roles and can change with the theme family.",
        t.TokenNames.Where(name => name.StartsWith("radius.")).Select(name => (UIElement)Stage.Row(t, new Border { Width = t.Number("spacing.16"), Height = t.Number("spacing.10"), CornerRadius = new CornerRadius(t.Number(name)), Background = t.Recipe(UI.ControlVariant.Tinted, UI.WidgetTint.Primary, "surface"), BorderBrush = t.Recipe(UI.ControlVariant.Outlined, UI.WidgetTint.Primary, "border"), BorderThickness = new Thickness(t.Number("stroke.hairline")) }, t.Text($"{name} · {t.Number(name)} px", "bodySmall"))).ToArray());
    public static UIElement Effect(UI.Theme t, string story) => Page(t, "Effect", "Elevation and shadows separate floating content from the page beneath it.",
        t.TokenNames.Where(name => name.StartsWith("effect.") || name.StartsWith("shadow.")).Select(name => (UIElement)new UI.Card(Stage.Column(t, t.Text(name, "labelMedium"), t.Text(t.Token(name).ToString(), "bodySmall")), t)).ToArray());
    public static UIElement Control(UI.Theme t, string story)
    {
        if (story == "Colors") return Page(t, "Control colors", "Every control recipe resolves its surface, content and border for the selected tint.", Enum.GetValues<UI.ControlVariant>().Select(variant => (UIElement)Stage.Row(t, Enum.GetValues<UI.WidgetTint>().Select(tint => (UIElement)new UI.Button { Content = variant + " · " + tint, Theme = t, Variant = variant, Tint = tint }).ToArray())).ToArray());
        if (story == "Focus") return Page(t, "Focus", "Use Tab to move between controls. The focus ring always uses the system accent.", Stage.Row(t, new UI.Button { Content = "First" }, new UI.Button { Content = "Second", Tint = UI.WidgetTint.Danger }, new UI.TextField { PlaceholderText = "Third" }));
        return Page(t, "Control sizing", "One height, corner, spacing and typography profile per control size.", Enum.GetValues<UI.ControlSize>().Select(size => (UIElement)Stage.Row(t, new UI.Button { Content = size.ToString(), Theme = t, Size = size }, t.Text($"{t.Number($"control.{size.ToString().ToLowerInvariant()}.size")} px", "bodySmall"))).ToArray());
    }
    public static UIElement Themes(UI.Theme t, string story) => Page(t, "Themes", "Six families, each with a light and dark appearance. Each subtree keeps its own theme.",
        UI.Theme.Names.Select(name =>
        {
            var theme = new UI.Theme(name);
            var preview = story == "Ramps"
                ? Stage.Column(theme, Enum.GetValues<UI.WidgetTint>().Select(tint => (UIElement)Stage.Row(theme, theme.Text(tint.ToString(), "labelSmall"), Stage.Row(theme, new[] { 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950 }.Select(shade => (UIElement)new Border { Width = theme.Number("spacing.6"), Height = theme.Number("spacing.6"), Background = theme.Brush($"color.{tint.ToString().ToLowerInvariant()}.{shade}") }).ToArray()))).ToArray())
                : Stage.Row(theme, new UI.Button { Theme = theme, Content = "Continue", Variant = UI.ControlVariant.Filled }, new UI.Badge("Ready", theme), new UI.Switch { IsChecked = true });
            return (UIElement)new UI.ThemeScope { Theme = theme, Content = new UI.Card(Stage.Column(theme, theme.Text(name, "titleSmall"), preview), theme) };
        }).ToArray());
}
