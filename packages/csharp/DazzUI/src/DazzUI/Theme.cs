using System.Text.Json;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Windows.UI;
using Windows.UI.Text;

namespace DazzUI;

public enum WidgetTint { Neutral, Primary, Success, Warning, Danger, Info }
public enum ControlVariant { Normal, Recessed, Filled, Tinted, Outlined, Plain }
public enum ControlSize { Tiny, Small, Medium, Large }
public enum ThemeSizing { Design, System }

/// <summary>A shared ds token snapshot with an optional live Windows sizing policy.</summary>
public sealed class Theme
{
    private static readonly JsonElement Themes = Load();
    private readonly JsonElement values;
    public string Name { get; }
    public ThemeSizing Sizing { get; }
    /// <summary>Runtime font/layout scale in logical pixels. Display DPI remains XAML's responsibility.</summary>
    public double LayoutScale => Sizing == ThemeSizing.System
        ? SystemStyle.FontSize / Number(values.GetProperty("bodyMedium").GetProperty("fontSize")) * SystemStyle.TextScale
        : 1;
    public bool IsDark => Name.EndsWith("-dark", StringComparison.Ordinal);
    public FontFamily CodeFont => PlatformFonts.Code;
    public static IReadOnlyList<string> Names { get; } = Themes.EnumerateObject().Select(p => p.Name).ToArray();
    public static Theme Default { get; } = new("bright-light");

    public Theme(string name, ThemeSizing sizing = ThemeSizing.Design)
    {
        if (!Themes.TryGetProperty(name, out values))
            throw new ArgumentException($"Unknown DazzUI theme: {name}", nameof(name));
        Name = name;
        Sizing = sizing;
    }

    private static JsonElement Load()
    {
        using var stream = typeof(Theme).Assembly.GetManifestResourceStream("DazzUI.Generated.tokens.json")
            ?? throw new InvalidOperationException("The generated design tokens are missing.");
        using var document = JsonDocument.Parse(stream);
        return document.RootElement.Clone();
    }

    public double Number(string token)
    {
        var value = values.GetProperty(token);
        var number = Number(value);
        if (Sizing != ThemeSizing.System || value.ValueKind != JsonValueKind.Object
            || !value.TryGetProperty("unit", out var unit) || unit.GetString() is not ("px" or "rem")) return number;
        // Corners, hairlines, focus rings and animation timings retain their roles.
        if (token.StartsWith("radius.", StringComparison.Ordinal) || token.EndsWith(".radius", StringComparison.Ordinal)
            || token.StartsWith("stroke.", StringComparison.Ordinal) || token.StartsWith("focus.", StringComparison.Ordinal)
            || token.EndsWith(".thickness", StringComparison.Ordinal) || token.EndsWith(".edge-width", StringComparison.Ordinal)
            || token == "spacing.px") return number;
        if (token.StartsWith("control.", StringComparison.Ordinal) && token.EndsWith(".size", StringComparison.Ordinal))
            return number / Number(values.GetProperty("control.medium.size")) * SystemStyle.ControlHeight * SystemStyle.TextScale;
        return number * LayoutScale;
    }

    /// <summary>A duration token, in milliseconds — motion is not a length and never scales with text.</summary>
    public double Milliseconds(string token) => values.GetProperty(token).GetProperty("value").GetDouble();

    private double TypeScale(string token) => Sizing == ThemeSizing.System && token.StartsWith("control.", StringComparison.Ordinal)
        ? SystemStyle.FontSize / Number(values.GetProperty("control.medium.content").GetProperty("fontSize")) * SystemStyle.TextScale
        : LayoutScale;
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
        control.UseLayoutRounding = false;
        var style = values.GetProperty(token);
        control.IsTextScaleFactorEnabled = Sizing != ThemeSizing.System;
        control.FontSize = Number(style.GetProperty("fontSize")) * TypeScale(token);
        control.FontWeight = new FontWeight { Weight = (ushort)style.GetProperty("fontWeight").GetInt32() };
        control.FontFamily = PlatformFonts.Ui;
    }

    public TextBlock Text(string text, string token = "bodyMedium")
    {
        var style = values.GetProperty(token);
        return new TextBlock
        {
            UseLayoutRounding = false,
            Text = text,
            IsTextScaleFactorEnabled = Sizing != ThemeSizing.System,
            FontSize = Number(style.GetProperty("fontSize")) * TypeScale(token),
            FontFamily = PlatformFonts.Ui,
            FontWeight = new FontWeight { Weight = (ushort)style.GetProperty("fontWeight").GetInt32() },
            LineHeight = Number(style.GetProperty("lineHeight")) * TypeScale(token),
            LineStackingStrategy = Microsoft.UI.Xaml.LineStackingStrategy.BlockLineHeight,
            Foreground = Brush("color.content"),
            TextWrapping = Microsoft.UI.Xaml.TextWrapping.Wrap,
        };
    }

    /// <summary>A shadow token's layers, outermost first, in device-independent pixels.</summary>
    internal IReadOnlyList<(Color Color, double X, double Y, double Blur, double Spread, bool Inset)> Shadow(string token) =>
        values.GetProperty(token).EnumerateArray().Select(layer => (
            ReadColor(layer.GetProperty("color")),
            Number(layer.GetProperty("offsetX")),
            Number(layer.GetProperty("offsetY")),
            Number(layer.GetProperty("blur")),
            Number(layer.GetProperty("spread")),
            layer.TryGetProperty("inset", out var inset) && inset.GetBoolean())).ToArray();

    /// <summary>A type style's weight, as WinUI takes it.</summary>
    internal FontWeight Weight(string token) =>
        new() { Weight = (ushort)values.GetProperty(token).GetProperty("fontWeight").GetInt32() };

    internal static string Key<T>(T value) where T : Enum => value.ToString().ToLowerInvariant();
}

internal static class TextStyle
{
    /// <summary>Gives <paramref name="target"/> the face this styled text has, keeping its words and ink.</summary>
    public static void CopyTo(this TextBlock source, TextBlock target)
    {
        target.IsTextScaleFactorEnabled = source.IsTextScaleFactorEnabled;
        target.FontFamily = source.FontFamily;
        target.FontSize = source.FontSize;
        target.FontWeight = source.FontWeight;
        target.LineHeight = source.LineHeight;
        target.LineStackingStrategy = source.LineStackingStrategy;
    }
}
