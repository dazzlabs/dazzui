using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

public enum ActionBarAlignment { Start, End, Between }
public sealed class ActionBar : Surface
{
    public IList<UIElement> Items { get; } = new List<UIElement>();
    public ActionBarAlignment Alignment { get; set; } = ActionBarAlignment.End;
    protected override void RefreshAppearance()
    {
        var gap = Theme.Number("control.small.gap");
        if (Alignment != ActionBarAlignment.Between)
        {
            var row = Skin.Row(Theme, Items.ToArray());
            row.Spacing = gap;
            row.HorizontalAlignment = Alignment == ActionBarAlignment.Start ? HorizontalAlignment.Left : HorizontalAlignment.Right;
            Content = row;
        }
        else
        {
            var grid = new Grid { ColumnSpacing = gap };
            for (var i = 0; i < Items.Count; i++)
            {
                grid.ColumnDefinitions.Add(new ColumnDefinition { Width = i == 0 ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
                Skin.Detach(Items[i]); Items[i].SetValue(Grid.ColumnProperty, i); grid.Children.Add(Items[i]);
                // The first item keeps its own width at the start; the rest sit at the end.
                if (i == 0 && Items[i] is FrameworkElement first) first.HorizontalAlignment = HorizontalAlignment.Left;
            }
            Content = grid;
        }
    }
}
/// <summary>
/// A disclosure: a row that opens a panel under itself. The row is a toggle
/// with a chevron that turns when it is open, the title, and any meta at its
/// end; the panel is set in so its text lines up under the title.
/// </summary>
public class Collapsible : Surface
{
    private const string TriggerTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='ToggleButton'><Border Background='{TemplateBinding Background}' CornerRadius='{TemplateBinding CornerRadius}' Padding='{TemplateBinding Padding}'><ContentPresenter Content='{TemplateBinding Content}' HorizontalContentAlignment='Stretch' VerticalContentAlignment='Center'/></Border></ControlTemplate>";
    private readonly Microsoft.UI.Xaml.Controls.Primitives.ToggleButton trigger = new() { HorizontalAlignment = HorizontalAlignment.Stretch, HorizontalContentAlignment = HorizontalAlignment.Stretch };
    private readonly Grid row = new();
    private readonly Icon chevron = new() { Glyph = FluentIcons.Common.Icon.ChevronRight, VerticalAlignment = VerticalAlignment.Center };
    private readonly RotateTransform turn = new();
    private readonly ContentPresenter title = new() { VerticalAlignment = VerticalAlignment.Center };
    private readonly ContentPresenter meta = new() { VerticalAlignment = VerticalAlignment.Center };
    private readonly Border panel = new() { Visibility = Visibility.Collapsed };
    private bool hovered;
    /// <summary>Raised when the panel opens or closes.</summary>
    public event EventHandler? IsExpandedChanged;
    private object? header;
    public object? Header
    {
        get => header;
        set { header = value; title.Content = value is string text && IsLoaded ? Theme.Text(text, "labelMedium") : value; }
    }
    /// <summary>What sits at the row's end — a count, a badge — in the quiet ink.</summary>
    public UIElement? Meta { get => meta.Content as UIElement; set => meta.Content = value; }
    public UIElement? Body { get => panel.Child; set => panel.Child = value; }
    public bool IsExpanded { get => trigger.IsChecked == true; set => trigger.IsChecked = value; }
    public Collapsible()
    {
        trigger.Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(TriggerTemplate);
        trigger.MinWidth = trigger.MinHeight = 0;
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        chevron.RenderTransform = turn;
        chevron.RenderTransformOrigin = new Windows.Foundation.Point(.5, .5);
        Grid.SetColumn(title, 1);
        Grid.SetColumn(meta, 2);
        row.Children.Add(chevron);
        row.Children.Add(title);
        row.Children.Add(meta);
        trigger.Content = row;
        trigger.Checked += (_, _) => Changed();
        trigger.Unchecked += (_, _) => Changed();
        trigger.PointerEntered += (_, _) => { hovered = true; Paint(); };
        trigger.PointerExited += (_, _) => { hovered = false; Paint(); };
        Content = new StackPanel { Children = { trigger, panel } };
    }
    private void Changed()
    {
        panel.Visibility = IsExpanded ? Visibility.Visible : Visibility.Collapsed;
        turn.Angle = IsExpanded ? 90 : 0;
        IsExpandedChanged?.Invoke(this, EventArgs.Empty);
    }
    private void Paint()
    {
        if (!IsLoaded) return;
        trigger.Background = hovered && trigger.IsEnabled ? Theme.Recipe(ControlVariant.Normal, Tint, "surface", "hovered") : new SolidColorBrush(Microsoft.UI.Colors.Transparent);
    }
    protected override void RefreshAppearance()
    {
        var gap = Theme.Number("spacing.2");
        var small = Theme.Number("icon.small");
        trigger.Padding = new Thickness(gap);
        trigger.CornerRadius = new CornerRadius(Theme.Number("control.container.radius"));
        Skin.Focus(trigger, Theme);
        Theme.Typography(trigger, "labelMedium");
        if (header is string words)
        {
            if (title.Content is TextBlock headerText) Theme.Text("", "labelMedium").CopyTo(headerText);
            else title.Content = Theme.Text(words, "labelMedium");
        }
        trigger.Foreground = Theme.Brush(trigger.IsEnabled ? "color.content" : "color.content-faint");
        row.ColumnSpacing = gap;
        chevron.GlyphSize = small;
        chevron.Foreground = Theme.Brush("color.content-subtle");
        meta.Foreground = Theme.Brush("color.content-faint");
        panel.Padding = new Thickness(gap + small + gap, Theme.Number("spacing.1"), gap, Theme.Number("spacing.3"));
        if (panel.Child is TextBlock text)
        {
            Theme.Text("", "bodySmall").CopyTo(text);
            text.Foreground = Theme.Brush("color.content-secondary");
        }
        turn.Angle = IsExpanded ? 90 : 0;
        Paint();
    }
}
public sealed class AccordionItem : Collapsible { }
/// <summary>Disclosures stacked, one open at a time unless <see cref="Multiple"/>, ruled apart when <see cref="Divided"/>.</summary>
public sealed class Accordion : Surface
{
    private readonly StackPanel stack = new();
    public IList<AccordionItem> Items { get; } = new List<AccordionItem>();
    public bool Multiple { get; set; }
    public bool Divided { get; set; }
    public Accordion() { Content = stack; }
    protected override void RefreshAppearance()
    {
        if (stack.Children.Count == 0)
            foreach (var item in Items)
            {
                item.IsExpandedChanged += (_, _) =>
                {
                    if (!item.IsExpanded || Multiple) return;
                    foreach (var other in Items.Where(other => other != item)) other.IsExpanded = false;
                };
                stack.Children.Add(item);
            }
        var first = true;
        foreach (var rule in stack.Children.OfType<AccordionItem>())
        {
            rule.BorderBrush = Theme.Brush("color.border");
            rule.BorderThickness = new Thickness(0, Divided && !first ? Theme.Number("stroke.hairline") : 0, 0, 0);
            first = false;
        }
    }
}
/// <summary>
/// One setting: its title over an optional subtitle, and the control at the
/// row's end. The row's height is a floor plus a pad on its text, never a
/// height, so it grows for a second line and never for a control. A row that
/// opens something washes under the pointer and ends in a chevron.
/// </summary>
public sealed class PreferenceRow : Surface
{
    private const string ActionTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='Button'><Border Background='Transparent' CornerRadius='{TemplateBinding CornerRadius}' Padding='{TemplateBinding Padding}'><ContentPresenter Content='{TemplateBinding Content}' HorizontalContentAlignment='Stretch' VerticalContentAlignment='Center'/></Border></ControlTemplate>";
    private readonly Microsoft.UI.Xaml.Controls.Button action = new() { HorizontalAlignment = HorizontalAlignment.Stretch, HorizontalContentAlignment = HorizontalAlignment.Stretch, MinHeight = 0, MinWidth = 0 };
    private bool hovered;
    public string Label { get; set; } = "";
    public string? Description { get; set; }
    public UIElement? Control { get; set; }
    public UIElement? Leading { get; set; }
    /// <summary>Raised when an interactive row is pressed; subscribing makes the row interactive.</summary>
    public event EventHandler? Opened
    {
        add { opened += value; Interactive = true; if (IsLoaded) RefreshAppearance(); }
        remove { opened -= value; Interactive = opened is not null; if (IsLoaded) RefreshAppearance(); }
    }
    private event EventHandler? opened;
    private bool Interactive { get; set; }
    public PreferenceRow()
    {
        Tint = WidgetTint.Neutral;
        HorizontalAlignment = HorizontalAlignment.Stretch;
        action.Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(ActionTemplate);
        action.Click += (_, _) => opened?.Invoke(this, EventArgs.Empty);
        PointerEntered += (_, _) => { hovered = true; Paint(); };
        PointerExited += (_, _) => { hovered = false; Paint(); };
    }
    private void Paint()
    {
        if (!IsLoaded) return;
        Background = Interactive && hovered ? Theme.Recipe(ControlVariant.Plain, Tint, "surface", "hovered") : null;
    }
    protected override void RefreshAppearance()
    {
        // The gap sits only between what is there: a row without a glyph starts on the heading's edge.
        var gap = Theme.Number("spacing.2.5");
        var grid = new Grid { ColumnSpacing = gap };
        if (Leading is not null) grid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        var title = Theme.Text(Label, "labelQuiet");
        title.TextTrimming = TextTrimming.CharacterEllipsis;
        title.TextWrapping = TextWrapping.NoWrap;
        var text = new StackPanel { Spacing = Theme.Number("spacing.1"), VerticalAlignment = VerticalAlignment.Center, Padding = new Thickness(0, Theme.Number("preferences.row-padding"), 0, Theme.Number("preferences.row-padding")), Children = { title } };
        if (Description is not null)
        {
            var subtitle = Theme.Text(Description, "captionSmall");
            subtitle.Foreground = Theme.Brush("color.content-subtle");
            subtitle.TextTrimming = TextTrimming.CharacterEllipsis;
            subtitle.TextWrapping = TextWrapping.NoWrap;
            text.Children.Add(subtitle);
        }
        Grid.SetColumn(text, Leading is null ? 0 : 1); grid.Children.Add(text);
        if (Leading is not null) { Skin.Detach(Leading); Leading.SetValue(Grid.ColumnProperty, 0); grid.Children.Add(Leading); }
        var trailing = Control;
        if (Interactive)
        {
            var chevron = new Icon { Glyph = FluentIcons.Common.Icon.ChevronRight, GlyphSize = Theme.Number("icon.small"), Foreground = Theme.Brush("color.content-subtle"), VerticalAlignment = VerticalAlignment.Center };
            trailing = trailing is null ? chevron : new StackPanel { Orientation = Orientation.Horizontal, Spacing = gap, Children = { Detached(trailing), chevron } };
        }
        if (trailing is FrameworkElement end)
        {
            grid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
            Skin.Detach(end); end.VerticalAlignment = VerticalAlignment.Center;
            Grid.SetColumn(end, grid.ColumnDefinitions.Count - 1); grid.Children.Add(end);
        }
        MinHeight = Theme.Number("control.medium.size");
        var inset = Interactive ? Theme.Number("spacing.2") : 0;
        Padding = new Thickness(0);
        Margin = new Thickness(-inset, 0, -inset, 0);
        Skin.SetCorner(this, Interactive ? Theme.Number("control.container.radius") : 0);
        VerticalContentAlignment = VerticalAlignment.Center;
        if (Interactive)
        {
            action.Content = grid;
            action.Padding = new Thickness(inset, 0, inset, 0);
            Skin.SetCorner(action, Theme.Number("control.container.radius"));
            Skin.Focus(action, Theme);
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(action, Label);
            Content = action;
        }
        else { action.Content = null; Content = grid; }
        Paint();
    }
    private static UIElement Detached(UIElement element) { Skin.Detach(element); return element; }
}/// <summary>A section's rows, close together: the settings column's tightest step.</summary>
public sealed class PreferenceGroup : Surface
{
    public IList<PreferenceRow> Rows { get; } = new List<PreferenceRow>();
    protected override void RefreshAppearance()
    {
        var rows = new StackPanel { Spacing = Theme.Number("preferences.row-gap") };
        foreach (var row in Rows) { Skin.Detach(row); rows.Children.Add(row); }
        Content = rows;
    }
}/// <summary>A heading over its rows, and an optional footer under them in the caption face.</summary>
public sealed class PreferenceSection : Surface
{
    public string Title { get; set; } = "";
    public string? Description { get; set; }
    public UIElement? Body { get; set; }
    /// <summary>A line under the rows: what the setting does not say for itself.</summary>
    public string? Footer { get; set; }
    protected override void RefreshAppearance()
    {
        var column = new StackPanel();
        if (Title.Length > 0) column.Children.Add(new SectionLabel(Title) { Margin = new Thickness(0, 0, 0, Theme.Number("preferences.heading-gap")) });
        if (Description is not null) column.Children.Add(Theme.Text(Description, "bodySmall"));
        if (Body is not null) { Skin.Detach(Body); column.Children.Add(Body); }
        if (Footer is not null)
        {
            var footer = Theme.Text(Footer, "captionSmall");
            footer.Foreground = Theme.Brush("color.content-subtle");
            footer.LineHeight = footer.FontSize * 1.7;
            footer.Margin = new Thickness(0, Theme.Number("preferences.footer-gap"), 0, 0);
            column.Children.Add(footer);
        }
        Content = column;
    }
}/// <summary>The settings column: groups of sections at the widest step, no wider than its width token.</summary>
public sealed class Preferences : Surface
{
    public IList<UIElement> Sections { get; } = new List<UIElement>();
    protected override void RefreshAppearance()
    {
        // As the web writes it: the settings width, capped by the room it is
        // given — a preferences column in a narrow pane is that pane's width.
        Width = double.NaN;
        MaxWidth = Theme.Number("preferences.width");
        HorizontalAlignment = HorizontalAlignment.Stretch;
        var column = new StackPanel { Spacing = Theme.Number("preferences.group-gap") };
        foreach (var section in Sections) { Skin.Detach(section); column.Children.Add(section); }
        Content = column;
    }
}public class NavItem : Toggle
{
    public string? Count { get; set; }
    /// <summary>The element at the row's end — a count badge, an action.</summary>
    public UIElement? Trailing { get; set; }
    /// <summary>The leading glyph, or none.</summary>
    public Glyph? Glyph { get; set; } = FluentIcons.Common.Icon.Folder;
    /// <summary>
    /// The leading element itself — a story's <see cref="Icon"/> node, drawn
    /// and addressed as its own specimen — in place of <see cref="Glyph"/>.
    /// </summary>
    public UIElement? Leading { get; set; }
    public bool IsIconOnly { get; set; }
    public NavItem() { DefaultStyleKey = typeof(NavItem); Size = ControlSize.Medium; HorizontalContentAlignment = HorizontalAlignment.Left; HorizontalAlignment = HorizontalAlignment.Stretch; Variant = ControlVariant.Normal; }
    protected override void OnApplyTemplate() { base.OnApplyTemplate(); Apply(); }
    protected override void Apply()
    {
        base.Apply();
        // A row rests transparent in the navigation ink; the page it is on is
        // filled, and the pointer previews that in the plain wash.
        var onPage = IsChecked == true;
        var state = RecipeState;
        Background = onPage ? Theme.Recipe(ControlVariant.Filled, Tint, "surface", IsEnabled ? "normal" : "disabled")
            : state is "hovered" or "pressed" ? Theme.Recipe(ControlVariant.Plain, Tint, "surface", "hovered")
            : new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent);
        Foreground = onPage ? Theme.Recipe(ControlVariant.Filled, Tint, "content", IsEnabled ? "normal" : "disabled")
            : Theme.Brush(!IsEnabled ? "color.content-faint" : state is "hovered" or "pressed" ? "color.content" : "color.content-nav");
        BorderThickness = new Thickness(0);
        Skin.SetCorner(this, Theme.Number("radius.small"));
        var type = IsChecked == true ? "labelMedium" : "labelQuiet";
        Theme.Typography(this, type);
        // The row's eight-pixel block inset leaves only the label's own line;
        // the platform's default line box would be clipped by it.
        Skin.TextLine(this, Theme.Text("", type).LineHeight);
        if (GetTemplateChild("Label") is ContentPresenter label) label.Visibility = IsIconOnly ? Visibility.Collapsed : Visibility.Visible;
        // The row's own inset, not the button profile's: a navigation row is
        // ten inline and eight block, with a six-pixel gap to its glyph.
        Padding = IsIconOnly
            ? new Thickness(Theme.Number("spacing.1.5"))
            : new Thickness(Theme.Number("spacing.2.5"), Theme.Number("spacing.2"), Theme.Number("spacing.2.5"), Theme.Number("spacing.2"));
        Width = IsIconOnly ? Height : double.NaN;
        HorizontalContentAlignment = IsIconOnly ? HorizontalAlignment.Center : HorizontalAlignment.Left;
        if (GetTemplateChild("Leading") is ContentControl leading)
        {
            if (Leading is not null) { Skin.Detach(Leading); leading.Content = Leading; }
            else if (Glyph is Glyph glyph) { if (leading.Content is not Icon current || current.Glyph != glyph) leading.Content = new Icon { Glyph = glyph }; }
            else leading.Content = null;
            leading.Margin = new Thickness(0, 0, IsIconOnly || leading.Content is null ? 0 : Theme.Number("spacing.1.5"), 0);
        }
        if (GetTemplateChild("Trailing") is ContentControl trailing)
        {
            if (Trailing is not null) { Skin.Detach(Trailing); trailing.Content = Trailing; }
            else if (Count is not null)
            {
                var count = Theme.Text(Count, "captionSmall");
                count.FontWeight = Theme.Text("", "labelMedium").FontWeight;
                trailing.Content = count;
            }
            else trailing.Content = null;
            trailing.Visibility = IsIconOnly || trailing.Content is null ? Visibility.Collapsed : Visibility.Visible;
        }
        // A count follows the row's own ink: white on the filled page, the
        // navigation grey otherwise.
        if (Trailing is SidebarBadge badge) { badge.Ink = Foreground; Appearance.Refresh(badge); }
        var labelText = Content as string ?? (Content as TextBlock)?.Text;
        if (IsIconOnly && labelText is not null) Tooltip.Set(this, labelText);
    }
}
public enum SidebarCollapsible { None, Offcanvas, Icons }
public class Sidebar : Surface
{
    private bool collapsed;
    private readonly HashSet<NavItem> wired = [];
    public bool IsCollapsed { get => collapsed; set { collapsed = value; RefreshAppearance(); } }
    public SidebarCollapsible Collapsible { get; set; }
    public UIElement? Header { get; set; }
    public UIElement? Footer { get; set; }
    public IList<UIElement> Items { get; } = new List<UIElement>();
    protected override void RefreshAppearance()
    {
        Width = IsCollapsed && Collapsible == SidebarCollapsible.Offcanvas ? 0 : Theme.Number(IsCollapsed && Collapsible == SidebarCollapsible.Icons ? "frame.sidebar.icon-width" : "frame.sidebar.width");
        Background = Theme.Brush("color.surface-column");
        var icons = IsCollapsed && Collapsible == SidebarCollapsible.Icons;
        foreach (var group in Items.OfType<SidebarGroup>()) group.IsIconOnly = icons;
        var navigation = Items.OfType<NavItem>().Concat(Items.OfType<SidebarGroup>().SelectMany(group => group.Items.OfType<NavItem>())).ToArray();
        foreach (var item in navigation)
        {
            item.IsIconOnly = icons;
            if (wired.Add(item)) item.Checked += (_, _) => { foreach (var other in navigation.Where(other => other != item)) other.IsChecked = false; };
        }
        var grid = new Grid();
        grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        grid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });
        grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        if (Header is not null && !icons) { Skin.Detach(Header); grid.Children.Add(Header); }
        var scroll = new ScrollViewer { Content = Skin.Column(Theme, Items.ToArray()), HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled };
        Grid.SetRow(scroll, 1); grid.Children.Add(scroll);
        if (Footer is not null && !icons) { Skin.Detach(Footer); Footer.SetValue(Grid.RowProperty, 2); grid.Children.Add(Footer); }
        Padding = new Thickness(Theme.Number("spacing.2")); Content = grid;
    }
}
public sealed class SidebarGroup : Surface
{
    internal bool IsIconOnly { get; set; }
    public string Label { get; set; } = "";
    public IList<UIElement> Items { get; } = new List<UIElement>();
    protected override void RefreshAppearance() { Content = Skin.Column(Theme, (IsIconOnly ? Array.Empty<UIElement>() : new UIElement[] { new SectionLabel(Label) }).Concat(Items).ToArray()); }
}
/// <summary>
/// A row's count at the row's end: a twenty-pixel chip in the navigation
/// ink, the caption at the label's weight, tabular figures, and the small
/// corner — the web's <c>sidebar-menu-badge</c>.
/// </summary>
public sealed class SidebarBadge : Surface
{
    public string Text { get; set; } = "";
    /// <summary>The row's ink, when the badge follows a selected row; null for the navigation ink.</summary>
    public Microsoft.UI.Xaml.Media.Brush? Ink { get; set; }
    public SidebarBadge() { HorizontalAlignment = HorizontalAlignment.Left; VerticalAlignment = VerticalAlignment.Center; }
    protected override void RefreshAppearance()
    {
        Height = Theme.Number("spacing.5");
        MinWidth = Theme.Number("spacing.5");
        Padding = new Thickness(Theme.Number("spacing.1"), 0, Theme.Number("spacing.1"), 0);
        Skin.SetCorner(this, Theme.Number("radius.small"));
        Foreground = Ink ?? Theme.Brush("color.content-nav");
        Theme.Typography(this, "captionSmall");
        FontWeight = Theme.Text("", "labelMedium").FontWeight;
        Skin.TextLine(this, Theme.Number("spacing.5"));
        Content = Text;
    }
}
/// <summary>
/// A group's heading, on the rows' own inset: the faint section label with
/// the sidebar's four-block, ten-inline padding around it, which is what
/// gives the heading its nineteen-pixel row.
/// </summary>
public sealed class SidebarHeading : Surface
{
    public string Label { get; set; } = "";
    protected override void RefreshAppearance()
    {
        Padding = new Thickness(Theme.Number("spacing.2.5"), Theme.Number("spacing.1"), Theme.Number("spacing.2.5"), Theme.Number("spacing.1"));
        Foreground = Theme.Brush("color.content-faint");
        Theme.Typography(this, "labelSmall");
        Skin.TextLine(this, Theme.Text("", "labelSmall").LineHeight);
        Content = Label;
    }
}
public sealed class Aside : Card
{
    protected override void RefreshAppearance() { base.RefreshAppearance(); Width = Theme.Number("frame.aside.width"); Background = Theme.Brush("color.surface-column"); }
}
public sealed class Rail : Surface
{
    public IList<UIElement> Items { get; } = new List<UIElement>();
    protected override void RefreshAppearance()
    {
        Width = Theme.Number("frame.rail.width"); Background = Theme.Brush("color.surface-column");
        Content = Skin.Column(Theme, Items.ToArray()); Padding = new Thickness(Theme.Number("spacing.2"));
    }
}
