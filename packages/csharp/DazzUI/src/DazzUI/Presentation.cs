using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.ApplicationModel.DataTransfer;

namespace DazzUI;

public sealed class Divider : Surface
{
    public Orientation Orientation { get; set; }
    public bool Muted { get; set; }
    protected override void RefreshAppearance()
    {
        Background = Theme.Brush(Muted ? "color.border-muted" : "color.border");
        if (Orientation == Orientation.Horizontal) Height = Theme.Number("stroke.hairline");
        else Width = Theme.Number("stroke.hairline");
    }
}
public sealed class SectionLabel : Surface
{
    public SectionLabel() { }
    public SectionLabel(string text) => Content = text;
    /// <summary>Set in a tint's ink rather than the subtle one.</summary>
    public WidgetTint? Accent { get; set; }
    /// <summary>Quieter still: the faint ink.</summary>
    public bool Faint { get; set; }
    protected override void RefreshAppearance()
    {
        Foreground = Accent is WidgetTint accent ? Theme.Recipe(ControlVariant.Plain, accent, "content")
            : Theme.Brush(Faint ? "color.content-faint" : "color.content-subtle");
        Theme.Typography(this, "labelSmall");
        Padding = new Thickness(Theme.Number("spacing.3"), Theme.Number("spacing.2"), 0, Theme.Number("spacing.1"));
    }
}
public enum AvatarShape { Circle, Square }
public sealed class Avatar : Surface
{
    public string NameText { get; set; } = "DazzUI";
    public Uri? Source { get; set; }
    public AvatarShape Shape { get; set; }
    protected override void RefreshAppearance()
    {
        Width = Height = Theme.Number($"control.{Theme.Key(Size)}.size") + Theme.Number("spacing.2");
        Skin.SetCorner(this, Theme.Number(Shape == AvatarShape.Circle ? "radius.full" : "radius.medium"));
        Background = Theme.Recipe(ControlVariant.Tinted, Tint, "surface");
        Foreground = Theme.Recipe(ControlVariant.Tinted, Tint, "content");
        HorizontalContentAlignment = HorizontalAlignment.Center;
        VerticalContentAlignment = VerticalAlignment.Center;
        Theme.Typography(this, "labelMedium");
        var initials = string.Concat(NameText.Split(' ', StringSplitOptions.RemoveEmptyEntries).Take(2).Select(n => n[0])).ToUpperInvariant();
        Content = initials;
        if (Source is not null)
        {
            var image = new Image { Source = new BitmapImage(Source), Stretch = Stretch.UniformToFill, Width = Width, Height = Height };
            image.ImageFailed += (_, _) => Content = initials;
            Content = image;
        }
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, NameText);
    }
}
public sealed class AvatarGroup : Surface
{
    public IList<Avatar> Avatars { get; } = new List<Avatar>();
    protected override void RefreshAppearance() { Content = Skin.Row(Theme, Avatars.Cast<UIElement>().ToArray()); }
}
public enum KeyCapVariant { Hint, Strong, Key }
public sealed class KeyCap : Surface
{
    public KeyCapVariant Variant { get; set; }
    public bool Raised { get => Variant == KeyCapVariant.Key; set => Variant = value ? KeyCapVariant.Key : KeyCapVariant.Hint; }
    public KeyCap() { HorizontalAlignment = HorizontalAlignment.Left; }
    public KeyCap(string text) : this() => Content = text;
    protected override void RefreshAppearance()
    {
        MinHeight = 0;
        Padding = Raised ? new Thickness(Theme.Number("spacing.2.5"), 0, Theme.Number("spacing.2.5"), 0) : new Thickness(0);
        Background = Raised ? Theme.Brush("color.surface-inset") : null;
        Foreground = Theme.Brush(Variant == KeyCapVariant.Hint ? "color.content-faint" : Variant == KeyCapVariant.Strong ? "color.content-subtle" : "color.content");
        BorderThickness = new Thickness(0);
        Skin.SetCorner(this, Theme.Number("radius.tiny"));
        Theme.Typography(this, Size == ControlSize.Large ? "labelMedium" : "labelSmall");
        Height = Raised ? Theme.Number("spacing.5") + Theme.Number("spacing.0.5") : double.NaN;
        VerticalContentAlignment = VerticalAlignment.Center;
    }
}
public sealed class Code : Surface
{
    public Code() { HorizontalAlignment = HorizontalAlignment.Left; }
    public Code(string text) : this() => Content = text;
    protected override void RefreshAppearance()
    {
        base.RefreshAppearance();
        Background = Theme.Brush("color.surface-sunken");
        Skin.SetCorner(this, Theme.Number("radius.tiny"));
        Padding = new Thickness(Theme.Number("spacing.1"), Theme.Number("spacing.0.5"), Theme.Number("spacing.1"), Theme.Number("spacing.0.5"));
        Theme.Typography(this, "bodySmall");
        FontFamily = new FontFamily("Cascadia Mono, Consolas"); // Windows monospace fallback.
    }
}
public sealed class CodeBlock : Surface
{
    public string Text { get; set; } = "";
    public string? Title { get; set; }
    public bool LineNumbers { get; set; }
    public bool Copyable { get; set; } = true;
    public bool Wrap { get; set; }
    public UIElement? Actions { get; set; }
    protected override void RefreshAppearance()
    {
        var value = LineNumbers ? string.Join('\n', Text.Split('\n').Select((line, i) => $"{i + 1,3}  {line}")) : Text;
        var text = Theme.Text(value, "bodySmall");
        text.FontFamily = new FontFamily("Cascadia Mono, Consolas");
        text.IsTextSelectionEnabled = true;
        text.TextWrapping = Wrap ? TextWrapping.Wrap : TextWrapping.NoWrap;
        var body = Skin.Column(Theme);
        if (Title is not null) body.Children.Add(Theme.Text(Title, "labelSmall"));
        body.Children.Add(new ScrollViewer { Content = text, HorizontalScrollBarVisibility = Wrap ? ScrollBarVisibility.Disabled : ScrollBarVisibility.Auto });
        if (Copyable)
        {
            var copy = new Button { Content = "Copy", Variant = ControlVariant.Plain, HorizontalAlignment = HorizontalAlignment.Right };
            copy.Click += (_, _) => { var data = new DataPackage(); data.SetText(Text); Clipboard.SetContent(data); copy.Content = "Copied"; };
            body.Children.Add(copy);
        }
        if (Actions is not null) { Skin.Detach(Actions); body.Children.Add(Actions); }
        Content = new Card(body, Theme);
    }
}
public sealed class EmptyState : Surface
{
    public string Title { get; set; } = "Nothing here yet";
    public string? Description { get; set; }
    public UIElement? Icon { get; set; }
    public UIElement? Actions { get; set; }
    protected override void RefreshAppearance()
    {
        var body = Skin.Column(Theme);
        body.HorizontalAlignment = HorizontalAlignment.Center;
        if (Icon is not null) { Skin.Detach(Icon); body.Children.Add(Icon); }
        body.Children.Add(Theme.Text(Title, "titleLarge"));
        if (Description is not null) body.Children.Add(Theme.Text(Description));
        if (Actions is not null) { Skin.Detach(Actions); body.Children.Add(Actions); }
        Padding = new Thickness(Theme.Number("spacing.8"));
        Content = body;
    }
}
public sealed class Callout : Surface
{
    public string? Title { get; set; }
    public string Description { get; set; } = "";
    public UIElement? Icon { get; set; }
    public UIElement? Action { get; set; }
    protected override void RefreshAppearance()
    {
        var body = Skin.Column(Theme);
        if (Title is not null) body.Children.Add(Theme.Text(Title, "labelMedium"));
        body.Children.Add(Theme.Text(Description));
        if (Action is not null) { Skin.Detach(Action); body.Children.Add(Action); }
        Content = Icon is null ? body : Skin.Row(Theme, Icon, body);
        Background = Theme.Recipe(ControlVariant.Tinted, Tint, "surface");
        Foreground = Theme.Recipe(ControlVariant.Tinted, Tint, "content");
        foreach (var text in body.Children.OfType<TextBlock>()) text.Foreground = Foreground;
        BorderBrush = Theme.Alpha($"color.{Theme.Key(Tint)}.{Theme.Number("focus.ring.shade")}", Theme.Number("wash.edge"));
        BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        Skin.SetCorner(this, Theme.Number(Size == ControlSize.Small ? "radius.medium" : "radius.large"));
        Padding = new Thickness(Theme.Number(Size == ControlSize.Small ? "spacing.3" : "spacing.4"));
    }
}
