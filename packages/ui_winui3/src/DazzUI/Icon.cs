using System.Text.Json;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Markup;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

public enum IconGlyph { Add, Search, Dismiss, Settings, Folder, Document, Checkmark, ChevronRight, ChevronDown, ArrowDownload, MoreHorizontal, Info, Warning, ErrorCircle, Person, ArrowClockwise, Copy, Delete, ArrowLeft, ArrowRight, Eye, EyeOff, TextBold, TextItalic, Star }
public sealed class Icon : Surface
{
    private static readonly Dictionary<string, string> Paths = Load();
    public IconGlyph Glyph { get; set; }
    public double? GlyphSize { get; set; }
    private static Dictionary<string, string> Load()
    {
        using var stream = typeof(Icon).Assembly.GetManifestResourceStream("DazzUI.Assets.icon-paths.json")!;
        return JsonSerializer.Deserialize<Dictionary<string, string>>(stream)!;
    }
    protected override void RefreshAppearance()
    {
        var shape = (Microsoft.UI.Xaml.Shapes.Path)XamlReader.Load($"<Path xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' Data='{Paths[Glyph.ToString()]}' Stretch='Uniform' />");
        shape.SetBinding(Microsoft.UI.Xaml.Shapes.Shape.FillProperty, new Microsoft.UI.Xaml.Data.Binding { Source = this, Path = new PropertyPath(nameof(Foreground)) });
        Width = Height = GlyphSize ?? Theme.Number(Size == ControlSize.Small ? "spacing.3" : Size == ControlSize.Large ? "spacing.5" : "spacing.4");
        Content = shape;
    }
}
public sealed class IconButton : Button
{
    public IconGlyph Glyph { get; set; }
    public string Label { get; set; } = "Action";
    public IconButton()
    {
        Size = ControlSize.Tiny;
        Loaded += (_, _) =>
        {
            Content = new Icon { Glyph = Glyph, GlyphSize = Theme.Number($"control.{Theme.Key(Size)}.size") - Theme.Number("spacing.3") };
            Width = Height; Padding = new Thickness(0);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, Label);
            Tooltip.Set(this, Label);
        };
    }
}
public sealed class Link : HyperlinkButton
{
    public Theme Theme { get => Appearance.GetTheme(this); set => Appearance.SetTheme(this, value); }
    public WidgetTint Tint { get => Appearance.GetTint(this); set => Appearance.SetTint(this, value); }
    public Link()
    {
        Appearance.Bind(this, () => { Theme.Typography(this, "bodyMedium"); Foreground = Theme.Recipe(ControlVariant.Plain, Tint, "content"); Padding = new Thickness(0); Skin.Focus(this, Theme); });
    }
}
