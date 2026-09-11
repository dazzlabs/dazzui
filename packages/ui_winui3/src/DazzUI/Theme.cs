using System.Text.Json;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Windows.UI;
using Windows.UI.Text;

namespace DazzUI;

public enum WidgetTint { Neutral, Primary, Success, Warning, Danger, Info }
public enum ControlVariant { Normal, Recessed, Filled, Tinted, Outlined, Plain }
public enum ControlSize { Tiny, Small, Medium, Large }

/// <summary>A complete, immutable snapshot from the shared ds token graph.</summary>
public sealed class Theme
{
    private static readonly JsonElement Themes = Load();
    private readonly JsonElement values;
    public string Name { get; }
    public bool IsDark => Name.EndsWith("-dark", StringComparison.Ordinal);
    public static IReadOnlyList<string> Names { get; } = Themes.EnumerateObject().Select(p => p.Name).ToArray();
    public static Theme Default { get; } = new("studio-light");

    public Theme(string name)
    {
        if (!Themes.TryGetProperty(name, out values))
            throw new ArgumentException($"Unknown DazzUI theme: {name}", nameof(name));
        Name = name;
    }

    private static JsonElement Load()
    {
        using var stream = typeof(Theme).Assembly.GetManifestResourceStream("DazzUI.Generated.tokens.json")
            ?? throw new InvalidOperationException("The generated design tokens are missing.");
        using var document = JsonDocument.Parse(stream);
        return document.RootElement.Clone();
    }

    public double Number(string token) => Number(values.GetProperty(token));
    private static double Number(JsonElement value)
    {
        if (value.ValueKind == JsonValueKind.Number) return value.GetDouble();
        var unit = value.GetProperty("unit").GetString();
        var number = value.GetProperty("value").GetDouble();
        // CSS rem is resolved against the same 16px root as the native ports.
        return unit switch { "px" => number, "rem" => number * 16, _ => throw new NotSupportedException($"Unsupported dimension: {unit}") };
    }

    public Color Color(string token) => ReadColor(values.GetProperty(token));
    private static Color ReadColor(JsonElement value) => Windows.UI.Color.FromArgb(
        value[0].GetByte(), value[1].GetByte(), value[2].GetByte(), value[3].GetByte());
    public SolidColorBrush Brush(string token) => new(Color(token));
    public SolidColorBrush Alpha(string token, double opacity)
    {
        var color = Color(token);
        color.A = (byte)Math.Round(color.A * opacity);
        return new SolidColorBrush(color);
    }
    public IEnumerable<string> TokenNames => values.EnumerateObject().Select(p => p.Name);
    public JsonElement Token(string name) => values.GetProperty(name);

    public SolidColorBrush Recipe(ControlVariant variant, WidgetTint tint, string part, string state = "normal")
    {
        var prefix = $"control.color.{Key(variant)}.{part}.{state}";
        var color = values.TryGetProperty(prefix + ".color", out var literal)
            ? ReadColor(literal)
            : Color($"color.{Key(tint)}.{Number(prefix + ".shade")}");
        if (values.TryGetProperty(prefix + ".alpha", out var alpha))
            color.A = (byte)Math.Round(color.A * alpha.GetDouble());
        return new SolidColorBrush(color);
    }

    public void Typography(Control control, string token)
    {
        var style = values.GetProperty(token);
        control.FontSize = Number(style.GetProperty("fontSize"));
        control.FontWeight = new FontWeight { Weight = (ushort)style.GetProperty("fontWeight").GetInt32() };
        // Native Windows UI font; the token's platform-system font has no literal family.
    }

    public TextBlock Text(string text, string token = "bodyMedium")
    {
        var style = values.GetProperty(token);
        return new TextBlock
        {
            Text = text,
            FontSize = Number(style.GetProperty("fontSize")),
            FontWeight = new FontWeight { Weight = (ushort)style.GetProperty("fontWeight").GetInt32() },
            LineHeight = Number(style.GetProperty("lineHeight")),
            LineStackingStrategy = Microsoft.UI.Xaml.LineStackingStrategy.BlockLineHeight,
            Foreground = Brush("color.content"),
            TextWrapping = Microsoft.UI.Xaml.TextWrapping.Wrap,
        };
    }

    internal static string Key<T>(T value) where T : Enum => value.ToString().ToLowerInvariant();
}
