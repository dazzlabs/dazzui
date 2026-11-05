using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.ApplicationModel.DataTransfer;

namespace DazzUI;

public sealed class Divider : Surface
{
    public Orientation Orientation { get; set; } = Orientation.Horizontal;
    public bool Muted { get; set; }
    protected override void RefreshAppearance()
    {
        // `muted` is the heavier rule — border-strong, a separator that holds against a filled surface.
        Background = Theme.Brush(Muted ? "color.border-strong" : "color.border");
        if (Orientation == Orientation.Horizontal) Height = Theme.Number("stroke.hairline");
        else Width = Theme.Number("stroke.hairline");
    }
}
public sealed class SectionLabel : Surface
{
    public SectionLabel() { }
    public SectionLabel(string text) => Content = text;
    /// <summary>A label already set in the theme it will be measured under.</summary>
    internal SectionLabel(string text, Theme theme)
    {
        Content = text;
        Theme = theme;
        RefreshAppearance();
    }
    /// <summary>Set in a tint's ink rather than the subtle one.</summary>
    public WidgetTint? Accent { get; set; }
    /// <summary>Quieter still: the faint ink.</summary>
    public bool Faint { get; set; }
    protected override void RefreshAppearance()
    {
        Foreground = Accent is WidgetTint accent ? Theme.Recipe(ControlVariant.Plain, accent, "content")
            : Theme.Brush(Faint ? "color.content-faint" : "color.content-subtle");
        Theme.Typography(this, "labelSmall");
        Skin.TextLine(this, Theme.Text("", "labelSmall").LineHeight);
    }
}
public enum AvatarShape { Circle, Square }
/// <summary>
/// The picture that stands for a person or a project, or its initials in
/// the tinted recipe's resting colours. Off the grid rather than the control
/// profile: small, medium and large are 20, 28 and 36px.
/// </summary>
public sealed class Avatar : Surface
{
    public string NameText { get; set; } = "DazzUI";
    public Uri? Source { get; set; }
    public AvatarShape Shape { get; set; }
    /// <summary>Whether it sits in a group, which lays each one on the ground colour it is cut out in.</summary>
    internal bool Grouped { get; set; }
    public Avatar() { Size = ControlSize.Medium; HorizontalAlignment = HorizontalAlignment.Left; VerticalAlignment = VerticalAlignment.Center; }
    internal static (string Box, string Type) Metrics(ControlSize size) => size switch
    {
        ControlSize.Tiny or ControlSize.Small => ("spacing.5", "captionSmall"),
        ControlSize.Large => ("spacing.9", "labelLarge"),
        _ => ("spacing.7", "labelMedium"),
    };
    protected override void RefreshAppearance()
    {
        var (box, type) = Metrics(Size);
        Width = Height = Theme.Number(box);
        var corner = Theme.Number(Shape == AvatarShape.Circle ? "radius.full" : "radius.medium");
        Skin.SetCorner(this, corner);
        // The avatar itself is clear; the tinted wash belongs to the
        // fallback, so a picture and a group ring do not double it.
        Background = Grouped ? Theme.Brush("color.surface") : null;
        Foreground = Theme.Recipe(ControlVariant.Tinted, Tint, "content");
        HorizontalContentAlignment = HorizontalAlignment.Center;
        VerticalContentAlignment = VerticalAlignment.Center;
        // Initials are read at a glance: the label's strong weight, one type step per size.
        var words = NameText.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);
        var monogram = words.Length == 0 ? "" : words[0][..1] + (words.Length > 1 ? words[^1][..1] : "");
        var initials = Theme.Text(monogram.ToUpperInvariant(), type);
        TextStyle.SetWeight(initials, Theme.Weight("labelStrong"));
        initials.Foreground = Foreground;
        initials.LineHeight = initials.FontSize;
        initials.TextLineBounds = TextLineBounds.Tight;
        initials.HorizontalAlignment = HorizontalAlignment.Center;
        initials.VerticalAlignment = VerticalAlignment.Center;
        var fallback = new Border
        {
            Background = Theme.Recipe(ControlVariant.Tinted, Tint, "surface"),
            CornerRadius = new CornerRadius(corner),
            Width = Width,
            Height = Height,
            Child = initials,
        };
        Content = fallback;
        if (Source is not null)
        {
            var image = new Image { Source = new BitmapImage(Source), Stretch = Stretch.UniformToFill, Width = Width, Height = Height };
            image.ImageFailed += (_, _) => Content = fallback;
            Content = image;
        }
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, NameText);
    }
}/// <summary>
/// Avatars overlapped by a third of the box, each cut out of its neighbour
/// by a ring in the ground colour, and the count of the rest in the same box.
/// </summary>
public sealed class AvatarGroup : Surface
{
    private readonly StackPanel row = new() { Orientation = Orientation.Horizontal, UseLayoutRounding = false };
    public IList<Avatar> Avatars { get; } = new List<Avatar>();
    /// <summary>How many more there are than are shown.</summary>
    public int Overflow { get; set; }
    public AvatarGroup() { Size = ControlSize.Medium; HorizontalAlignment = HorizontalAlignment.Left; Content = row; }
    protected override void RefreshAppearance()
    {
        // Let go of the avatars before the old rings go, so each can be ringed again.
        foreach (var ring0 in row.Children.OfType<Border>()) ring0.Child = null;
        row.Children.Clear();
        var overlap = Theme.Number(Size is ControlSize.Tiny or ControlSize.Small ? "spacing.1.5" : Size == ControlSize.Large ? "spacing.3" : "spacing.2");
        var ring = Theme.Number("stroke.control");
        var ground = Theme.Brush("color.surface");
        UIElement Ringed(FrameworkElement face) => new Border
        {
            UseLayoutRounding = false,
            Child = face,
            Background = ground,
            Padding = new Thickness(ring),
            CornerRadius = new CornerRadius(Math.Min(Theme.Number(face is Avatar { Shape: AvatarShape.Square } ? "radius.medium" : "radius.full"), Theme.Number(Avatar.Metrics(Size).Box) / 2) + ring),
            // The ring is an outset, as CSS box-shadow is. Both ends stay
            // outside layout so it never adds to the group's natural width.
            Margin = new Thickness(row.Children.Count == 0 ? -ring : -overlap - ring, -ring, -ring, -ring),
        };
        foreach (var avatar in Avatars)
        {
            avatar.Grouped = true;
            avatar.Size = Size;
            Skin.Detach(avatar);
            row.Children.Add(Ringed(avatar));
        }
        if (Overflow <= 0) return;
        var box = Theme.Number(Avatar.Metrics(Size).Box);
        var count = Theme.Text($"+{Overflow}", "captionSmall");
        TextStyle.SetWeight(count, Theme.Weight("labelStrong"));
        count.Foreground = Theme.Brush("color.content-muted");
        count.HorizontalAlignment = HorizontalAlignment.Center;
        count.VerticalAlignment = VerticalAlignment.Center;
        count.TextLineBounds = TextLineBounds.Tight;
        row.Children.Add(Ringed(new Border
        {
            Width = box,
            Height = box,
            CornerRadius = new CornerRadius(box / 2),
            Background = Theme.Brush("color.surface-sunken"),
            Child = count,
        }));
    }
}public enum KeyCapVariant { Hint, Strong, Key }
public sealed class KeyCap : Surface
{
    public KeyCapVariant Variant { get; set; }
    public bool Raised { get => Variant == KeyCapVariant.Key; set => Variant = value ? KeyCapVariant.Key : KeyCapVariant.Hint; }
    public KeyCap() { HorizontalAlignment = HorizontalAlignment.Left; VerticalAlignment = VerticalAlignment.Center; }
    public KeyCap(string text) : this() => Content = text;
    protected override void RefreshAppearance()
    {
        MinHeight = 0;
        Padding = Raised ? new Thickness(Theme.Number("spacing.2.5"), 0, Theme.Number("spacing.2.5"), 0) : new Thickness(0);
        Background = Raised ? Theme.Brush("color.surface-inset") : null;
        Foreground = Theme.Brush(Variant == KeyCapVariant.Hint ? "color.content-faint" : Variant == KeyCapVariant.Strong ? "color.content-subtle" : "color.content");
        BorderThickness = new Thickness(0);
        Skin.SetCorner(this, Raised ? Theme.Number("radius.tiny") : 0);
        Theme.Typography(this, Size == ControlSize.Large ? "labelMedium" : "labelSmall");
        FontWeight = Theme.Weight(Raised ? "labelStrong" : "labelMedium");
        Skin.TextLine(this, FontSize);
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
        Theme.Typography(this, "captionSmall");
        FontFamily = Theme.CodeFont;
    }
}
/// <summary>
/// A listing: an optional head — the file name in the code face, the
/// language in small caps and the copy action at its end — over the code,
/// with the line numbers in a gutter of their own.
/// </summary>
public sealed class CodeBlock : Surface
{
    private readonly IconButton copy = new() { Glyph = FluentIcons.Common.Icon.Copy, Label = "Copy" };
    private readonly DispatcherTimer copyFeedback = new();
    private bool copied;
    public string Text { get; set; } = "";
    public string? Title { get; set; }
    /// <summary>The listing's language, in small caps in the head.</summary>
    public string? Syntax { get; set; }
    public bool LineNumbers { get; set; }
    public bool Copyable { get; set; }
    public bool Wrap { get; set; }
    public UIElement? Actions { get; set; }
    public CodeBlock()
    {
        copy.Click += (_, _) => Copy(text =>
        {
            var data = new DataPackage();
            data.SetText(text);
            Clipboard.SetContent(data);
        });
        copyFeedback.Tick += (_, _) => ResetCopyFeedback();
        Unloaded += (_, _) => ResetCopyFeedback();
    }
    // A clipboard failure leaves the action unchanged. Keep the writer at the
    // boundary so native tests can verify success/refusal without replacing
    // the user's clipboard.
    internal void Copy(Action<string> write)
    {
        try { write(Text.TrimEnd('\n')); }
        catch (Exception) { return; }
        if (copied) return;
        copied = true;
        UpdateCopyFeedback();
        copyFeedback.Interval = TimeSpan.FromMilliseconds(Theme.Milliseconds("code-block.copy-feedback-duration"));
        copyFeedback.Start();
    }
    private void ResetCopyFeedback()
    {
        copyFeedback.Stop();
        copied = false;
        UpdateCopyFeedback();
    }
    private void UpdateCopyFeedback()
    {
        copy.Label = copied ? "Copied" : "Copy";
        copy.Glyph = copied ? FluentIcons.Common.Icon.Checkmark : FluentIcons.Common.Icon.Copy;
        if (copy.Content is Icon icon)
        {
            icon.Glyph = copy.Glyph;
            Appearance.Refresh(icon);
        }
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(copy, copy.Label);
        if (copy.IsLoaded) Tooltip.Set(copy, copy.Label);
    }
    protected override void RefreshAppearance()
    {
        var mono = Theme.CodeFont;
        var size = Theme.Text("", "captionSmall").FontSize;
        var leading = Theme.Text("", "bodySmall").LineHeight;
        var caption = Theme.Text("", "captionSmall").LineHeight;
        TextBlock Code(string text, double? line = null) => new() { IsTextScaleFactorEnabled = Theme.Sizing != ThemeSizing.System, Text = text, FontFamily = mono, FontSize = size, LineHeight = line ?? leading, LineStackingStrategy = LineStackingStrategy.BlockLineHeight };
        var column = new StackPanel();
        if (!string.IsNullOrEmpty(Title) || !string.IsNullOrEmpty(Syntax) || Copyable || Actions is not null)
        {
            var head = new Grid { ColumnSpacing = Theme.Number("spacing.2"), MinHeight = Theme.Number("spacing.8"), Padding = new Thickness(Theme.Number("spacing.3"), Theme.Number("spacing.1"), Theme.Number("spacing.3"), Theme.Number("spacing.1")), Background = Theme.Brush("color.surface-chrome"), BorderBrush = Theme.Brush("color.border"), BorderThickness = new Thickness(0, 0, 0, Theme.Number("stroke.hairline")) };
            head.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
            head.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
            head.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
            if (!string.IsNullOrEmpty(Title))
            {
                var title = Code(Title, caption);
                title.Foreground = Theme.Brush("color.content-secondary");
                title.VerticalAlignment = VerticalAlignment.Center;
                head.Children.Add(title);
            }
            if (!string.IsNullOrEmpty(Syntax))
            {
                var language = Theme.Text(Syntax.ToUpperInvariant(), "labelSmall");
                TextStyle.SetWeight(language, Theme.Weight("labelMedium"));
                language.Foreground = Theme.Brush("color.content-faint");
                language.VerticalAlignment = VerticalAlignment.Center;
                Grid.SetColumn(language, 1); head.Children.Add(language);
            }
            var actions = new StackPanel { Orientation = Orientation.Horizontal, Spacing = Theme.Number("spacing.0.5"), HorizontalAlignment = HorizontalAlignment.Right, VerticalAlignment = VerticalAlignment.Center };
            if (Actions is not null) { Skin.Detach(Actions); actions.Children.Add(Actions); }
            if (Copyable)
            {
                Skin.Detach(copy);
                UpdateCopyFeedback();
                actions.Children.Add(copy);
            }
            Grid.SetColumn(actions, 2); head.Children.Add(actions);
            column.Children.Add(head);
        }
        var source = Text.TrimEnd('\n');
        var lines = source.Split('\n');
        var body = new Grid { Padding = new Thickness(0, Theme.Number("spacing.2.5"), 0, Theme.Number("spacing.2.5")) };
        body.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        body.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        var code = Code(source);
        code.Foreground = Theme.Brush("color.content");
        code.IsTextSelectionEnabled = true;
        code.TextWrapping = Wrap ? TextWrapping.Wrap : TextWrapping.NoWrap;
        code.Margin = new Thickness(LineNumbers ? 0 : Theme.Number("spacing.3"), 0, Theme.Number("spacing.3"), 0);
        if (LineNumbers)
        {
            var gutter = Code(string.Join('\n', Enumerable.Range(1, lines.Length)));
            gutter.Foreground = Theme.Brush("color.content-faint");
            gutter.TextAlignment = TextAlignment.Right;
            gutter.MinWidth = Theme.Number("spacing.10");
            gutter.Padding = new Thickness(0, 0, Theme.Number("spacing.3"), 0);
            body.Children.Add(gutter);
        }
        Grid.SetColumn(code, 1); body.Children.Add(code);
        column.Children.Add(new ScrollViewer { Content = body, HorizontalScrollBarVisibility = Wrap ? ScrollBarVisibility.Disabled : ScrollBarVisibility.Auto, VerticalScrollBarVisibility = ScrollBarVisibility.Disabled });
        Background = Theme.Brush("color.surface-muted");
        BorderBrush = Theme.Brush("color.border");
        BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        Skin.SetCorner(this, Theme.Number("control.container.radius"));
        Padding = new Thickness(0);
        Content = column;
    }
}/// <summary>What a pane shows before it has content: one muted line and at most one action.</summary>
public sealed class EmptyState : Surface
{
    private readonly StackPanel body = new() { HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
    private readonly ContentPresenter iconHost = new();
    private readonly ContentPresenter actionsHost = new() { HorizontalAlignment = HorizontalAlignment.Center };
    private readonly TextBlock title = new() { TextAlignment = TextAlignment.Center, TextWrapping = TextWrapping.Wrap };
    private readonly TextBlock description = new() { TextAlignment = TextAlignment.Center, TextWrapping = TextWrapping.Wrap };
    public string Title { get; set; } = "Nothing here yet";
    public string? Description { get; set; }
    public UIElement? Icon { get; set; }
    public UIElement? Actions { get; set; }
    public EmptyState()
    {
        body.Children.Add(iconHost);
        body.Children.Add(title);
        body.Children.Add(description);
        body.Children.Add(actionsHost);
        Content = body;
    }
    protected override void RefreshAppearance()
    {
        body.Spacing = Theme.Number("spacing.3");
        Skin.Content(iconHost, Icon);
        Skin.Content(actionsHost, Actions);
        Theme.Text("", "bodyMedium").CopyTo(title);
        title.Text = Title;
        title.Foreground = Theme.Brush("color.content-subtle");
        Theme.Text("", "bodySmall").CopyTo(description);
        description.Text = Description ?? "";
        description.Foreground = Theme.Brush("color.content-subtle");
        description.Visibility = Description is null ? Visibility.Collapsed : Visibility.Visible;
        Padding = new Thickness(Theme.Number("spacing.8"), Theme.Number("spacing.14"), Theme.Number("spacing.8"), Theme.Number("spacing.14"));
    }
}/// <summary>
/// A tinted notice: an optional glyph, the title over its description, and
/// an action at the end. Fill and edge are washes of the tint, so it sits as
/// well on a card as on the paper; the type never changes with the size.
/// </summary>
public sealed class Callout : Surface
{
    private readonly Grid row = new();
    private readonly StackPanel body = new() { VerticalAlignment = VerticalAlignment.Center };
    private readonly ContentPresenter iconHost = new();
    private readonly ContentPresenter actionHost = new();
    private readonly TextBlock title = new() { TextWrapping = TextWrapping.Wrap };
    private readonly TextBlock description = new() { TextWrapping = TextWrapping.Wrap };
    public string? Title { get; set; }
    public string Description { get; set; } = "";
    public UIElement? Icon { get; set; }
    public UIElement? Action { get; set; }
    public Callout()
    {
        Size = ControlSize.Medium;
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        Grid.SetColumn(body, 1);
        Grid.SetColumn(actionHost, 2);
        body.Children.Add(title);
        body.Children.Add(description);
        row.Children.Add(iconHost);
        row.Children.Add(body);
        row.Children.Add(actionHost);
        Content = row;
    }
    protected override void RefreshAppearance()
    {
        row.ColumnSpacing = Theme.Number("spacing.2.5");
        body.Spacing = Theme.Number("spacing.1");
        var hasDescription = Description.Length > 0;
        var align = hasDescription ? VerticalAlignment.Top : VerticalAlignment.Center;
        Skin.Content(iconHost, Icon);
        Skin.Content(actionHost, Action);
        iconHost.VerticalAlignment = actionHost.VerticalAlignment = align;
        if (Icon is Control glyph) glyph.Foreground = Theme.Brush($"color.{Theme.Key(Tint)}.600");
        Theme.Text("", "labelMedium").CopyTo(title);
        title.Text = Title ?? "";
        title.Foreground = Theme.Recipe(ControlVariant.Tinted, Tint, "content", "hovered");
        title.LineHeight = title.FontSize * 1.4;
        title.Visibility = Title is null ? Visibility.Collapsed : Visibility.Visible;
        Theme.Text("", "labelQuiet").CopyTo(description);
        description.Text = Description;
        TextStyle.SetWeight(description, Theme.Weight("captionSmall"));
        description.Foreground = Theme.Recipe(ControlVariant.Tinted, Tint, "content");
        description.LineHeight = description.FontSize * 1.5;
        description.Visibility = hasDescription ? Visibility.Visible : Visibility.Collapsed;
        Background = Theme.Alpha($"color.{Theme.Key(Tint)}.600", Theme.Number("wash.surface"));
        BorderBrush = Theme.Alpha($"color.{Theme.Key(Tint)}.600", Theme.Number("wash.edge"));
        BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        var (block, inline) = Size switch
        {
            ControlSize.Tiny or ControlSize.Small => ("spacing.2", "spacing.3"),
            ControlSize.Large => ("spacing.4", "spacing.4"),
            _ => ("spacing.3", "spacing.3.5"),
        };
        Padding = new Thickness(Theme.Number(inline), Theme.Number(block), Theme.Number(inline), Theme.Number(block));
        Skin.SetCorner(this, Theme.Number(Size is ControlSize.Tiny or ControlSize.Small ? "control.container.radius" : "radius.large"));
    }
}
