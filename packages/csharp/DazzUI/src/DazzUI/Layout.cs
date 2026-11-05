using System.Collections.ObjectModel;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Animation;

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
            // Keep live children attached during appearance changes; moving a
            // focused native control between panels also resets its template.
            var row = Content as StackPanel;
            if (row is null || !row.Children.SequenceEqual(Items))
            {
                row = Skin.Row(Theme, Items.ToArray());
                Content = row;
            }
            row.Spacing = gap;
            row.HorizontalAlignment = Alignment == ActionBarAlignment.Start ? HorizontalAlignment.Left : HorizontalAlignment.Right;
        }
        else
        {
            var grid = Content as Grid;
            if (grid is null || !grid.Children.SequenceEqual(Items))
            {
                grid = new Grid();
                for (var i = 0; i < Items.Count; i++)
                {
                    grid.ColumnDefinitions.Add(new ColumnDefinition { Width = i == 0 ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
                    Skin.Detach(Items[i]); Items[i].SetValue(Grid.ColumnProperty, i); grid.Children.Add(Items[i]);
                    // The first item keeps its own width at the start; the rest sit at the end.
                    if (i == 0 && Items[i] is FrameworkElement first) first.HorizontalAlignment = HorizontalAlignment.Left;
                }
                Content = grid;
            }
            grid.ColumnSpacing = gap;
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
            // The trigger owns enabled/disabled ink; a text snapshot must not
            // pin the heading to the palette in which it was first created.
            ((TextBlock)title.Content).ClearValue(TextBlock.ForegroundProperty);
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
        var grid = (Interactive ? action.Content as Grid : Content as Grid) ?? new Grid();
        grid.ColumnSpacing = gap;
        grid.ColumnDefinitions.Clear();
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
        Grid.SetColumn(text, Leading is null ? 0 : 1);
        var children = new List<UIElement> { text };
        if (Leading is not null) { Leading.SetValue(Grid.ColumnProperty, 0); children.Add(Leading); }
        var trailing = Control;
        if (Interactive)
        {
            var chevron = new Icon { Glyph = FluentIcons.Common.Icon.ChevronRight, GlyphSize = Theme.Number("icon.small"), Foreground = Theme.Brush("color.content-subtle"), VerticalAlignment = VerticalAlignment.Center };
            trailing = trailing is null ? chevron : new StackPanel { Orientation = Orientation.Horizontal, Spacing = gap, Children = { Detached(trailing), chevron } };
        }
        if (trailing is FrameworkElement end)
        {
            var swatches = trailing is PreferenceThemeSwatches;
            if (swatches) grid.ColumnDefinitions[Leading is null ? 0 : 1].Width = GridLength.Auto;
            grid.ColumnDefinitions.Add(new ColumnDefinition { Width = swatches ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
            end.VerticalAlignment = VerticalAlignment.Center;
            Grid.SetColumn(end, grid.ColumnDefinitions.Count - 1); children.Add(end);
        }
        Skin.Children(grid, children);
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
        var rows = Skin.Stack(Theme, Orientation.Vertical, Rows.Cast<UIElement>().ToArray(), Content as StackPanel);
        rows.Spacing = Theme.Number("preferences.row-gap");
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
        var column = Content as StackPanel ?? new StackPanel();
        var children = new List<UIElement>();
        if (Title.Length > 0) children.Add(new SectionLabel(Title, Theme) { Margin = new Thickness(0, 0, 0, Theme.Number("preferences.heading-gap")) });
        if (Description is not null) children.Add(Theme.Text(Description, "bodySmall"));
        if (Body is not null) children.Add(Body);
        if (Footer is not null)
        {
            var footer = Theme.Text(Footer, "captionSmall");
            footer.Foreground = Theme.Brush("color.content-subtle");
            footer.LineHeight = footer.FontSize * 1.7;
            footer.Margin = new Thickness(0, Theme.Number("preferences.footer-gap"), 0, 0);
            children.Add(footer);
        }
        Skin.Children(column, children);
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
        var column = Skin.Stack(Theme, Orientation.Vertical, Sections.ToArray(), Content as StackPanel);
        column.Spacing = Theme.Number("preferences.group-gap");
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
        // The glyph and the label share a sixteen-pixel line, leaving six
        // pixels of block inset in the twenty-eight-pixel row. The web's line
        // is twelve pixels because the browser lets a glyph overflow; a
        // native line box clips it, so the row gives it the four more it needs.
        var line = Theme.Text("", type).FontSize + Theme.Number("spacing.1");
        Skin.TextLine(this, line);
        if (GetTemplateChild("Label") is ContentPresenter label) label.Visibility = IsIconOnly ? Visibility.Collapsed : Visibility.Visible;
        // The web row is its line plus the block inset, not the profile's
        // control height — a system-sizing profile would otherwise add four
        // pixels the sidebar's rows never had.
        Height = line + 2 * Theme.Number("spacing.1.5");
        MinHeight = 0;
        // The row's own inset, not the button profile's: a navigation row is
        // ten inline and six block, with a six-pixel gap to its glyph. Icon
        // mode keeps its six pixels on the sides only, as the web does.
        Padding = IsIconOnly
            ? new Thickness(Theme.Number("spacing.1.5"), 0, Theme.Number("spacing.1.5"), 0)
            : new Thickness(Theme.Number("spacing.2.5"), Theme.Number("spacing.1.5"), Theme.Number("spacing.2.5"), Theme.Number("spacing.1.5"));
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
                TextStyle.SetWeight(count, Theme.Weight("labelMedium"));
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
    private bool animatingWidth;
    private Storyboard? widthTween;
    private int widthVersion;
    private readonly HashSet<NavItem> wired = [];
    public bool IsCollapsed
    {
        get => collapsed;
        set
        {
            if (collapsed == value) return;
            var from = IsLoaded ? ActualWidth : double.NaN;
            collapsed = value;
            var to = TargetWidth;
            RefreshAppearance();
            if (IsLoaded) AnimateWidth(from, to);
        }
    }
    public SidebarCollapsible Collapsible { get; set; }
    public UIElement? Header { get; set; }
    public UIElement? Footer { get; set; }
    public IList<UIElement> Items { get; } = new List<UIElement>();
    public Sidebar()
    {
        // A collapsed column is narrower than its contents; the clip keeps
        // them from drawing over the page while they slide away.
        SizeChanged += (_, args) => Clip = new RectangleGeometry
        {
            Rect = new Windows.Foundation.Rect(0, 0, args.NewSize.Width, args.NewSize.Height),
        };
        Unloaded += (_, _) => { widthTween?.Stop(); widthTween = null; animatingWidth = false; };
    }
    private double TargetWidth => IsCollapsed && Collapsible == SidebarCollapsible.Offcanvas ? 0
        : Theme.Number(IsCollapsed && Collapsible == SidebarCollapsible.Icons ? "frame.sidebar.icon-width" : "frame.sidebar.width");
    private void AnimateWidth(double from, double to)
    {
        widthTween?.Stop();
        widthTween = null;
        animatingWidth = false;
        var current = ++widthVersion;
        if (double.IsNaN(from) || Math.Abs(from - to) < 0.5)
        {
            Width = to;
            return;
        }
        animatingWidth = true;
        Width = from;
        var animation = new DoubleAnimation
        {
            From = from,
            To = to,
            Duration = TimeSpan.FromMilliseconds(Theme.Milliseconds("motion.duration")),
            EasingFunction = new CubicEase { EasingMode = EasingMode.EaseInOut },
            EnableDependentAnimation = true,
        };
        Storyboard.SetTarget(animation, this);
        Storyboard.SetTargetProperty(animation, "Width");
        widthTween = new Storyboard { Children = { animation } };
        widthTween.Completed += (_, _) =>
        {
            if (current != widthVersion) return;
            widthTween = null;
            animatingWidth = false;
            Width = to;
        };
        widthTween.Begin();
    }
    protected override void RefreshAppearance()
    {
        var target = TargetWidth;
        if (!animatingWidth) Width = target;
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
/// <summary>
/// A run of navigation rows under a section heading. Its content is built
/// before the group is attached, and rebuilt as its label or rows change, so
/// the first measure already holds the heading and the row gaps.
/// </summary>
public sealed class SidebarGroup : Surface
{
    private readonly ObservableCollection<UIElement> items = [];
    private string label = "";
    private bool iconOnly;
    private bool dirty;
    internal bool IsIconOnly
    {
        get => iconOnly;
        set
        {
            if (iconOnly == value) return;
            iconOnly = value;
            Prepare();
        }
    }
    public string Label
    {
        get => label;
        set
        {
            if (label == value) return;
            label = value;
            Prepare();
        }
    }
    public IList<UIElement> Items => items;
    public SidebarGroup()
    {
        items.CollectionChanged += (_, _) => Prepare();
        Prepare();
    }
    /// <summary>
    /// Build the group's content before it is attached, or at its next measure
    /// when it has no XAML root yet — a native row cannot enter a tree before
    /// the framework has one for it.
    /// </summary>
    public void Prepare()
    {
        dirty = true;
        if (XamlRoot is not null)
        {
            dirty = false;
            RefreshAppearance();
        }
    }
    protected override Windows.Foundation.Size MeasureOverride(Windows.Foundation.Size availableSize)
    {
        if (dirty && XamlRoot is not null)
        {
            dirty = false;
            RefreshAppearance();
        }
        return base.MeasureOverride(availableSize);
    }
    protected override void RefreshAppearance()
    {
        dirty = false;
        Content = Skin.Column(Theme, (IsIconOnly ? Array.Empty<UIElement>() : new UIElement[] { new SectionLabel(label, Theme) }).Concat(items).ToArray());
    }
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
        FontWeight = Theme.Weight("labelMedium");
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
    private string label = "";
    private bool dirty = true;
    public string Label
    {
        get => label;
        set
        {
            if (label == value) return;
            label = value;
            dirty = true;
            if (XamlRoot is not null)
            {
                dirty = false;
                RefreshAppearance();
            }
        }
    }
    protected override Windows.Foundation.Size MeasureOverride(Windows.Foundation.Size availableSize)
    {
        if (dirty && XamlRoot is not null)
        {
            dirty = false;
            RefreshAppearance();
        }
        return base.MeasureOverride(availableSize);
    }
    protected override void RefreshAppearance()
    {
        dirty = false;
        Padding = new Thickness(Theme.Number("spacing.2.5"), Theme.Number("spacing.1"), Theme.Number("spacing.2.5"), Theme.Number("spacing.1"));
        Foreground = Theme.Brush("color.content-faint");
        Theme.Typography(this, "labelSmall");
        Skin.TextLine(this, Theme.Text("", "labelSmall").LineHeight);
        Content = label;
    }
}
public sealed class Aside : Card
{
    protected override void RefreshAppearance() { base.RefreshAppearance(); Width = Theme.Number("frame.aside.width"); Background = Theme.Brush("color.surface-column"); }
}
/// <summary>
/// The narrow column beside a page. Its content is built before attachment,
/// and rebuilt synchronously as its rows change, so replacing the rows never
/// leaves an empty intermediate measure.
/// </summary>
public sealed class Rail : Surface
{
    private readonly ObservableCollection<UIElement> items = [];
    private bool dirty;
    public IList<UIElement> Items => items;
    public Rail()
    {
        items.CollectionChanged += (_, _) => Prepare();
        Prepare();
    }
    /// <summary>
    /// Build the rail's content before it is attached, or at its next measure
    /// when it has no XAML root yet.
    /// </summary>
    public void Prepare()
    {
        dirty = true;
        if (XamlRoot is not null)
        {
            dirty = false;
            RefreshAppearance();
        }
    }
    protected override Windows.Foundation.Size MeasureOverride(Windows.Foundation.Size availableSize)
    {
        if (dirty && XamlRoot is not null)
        {
            dirty = false;
            RefreshAppearance();
        }
        return base.MeasureOverride(availableSize);
    }
    protected override void RefreshAppearance()
    {
        dirty = false;
        Width = Theme.Number("frame.rail.width"); Background = Theme.Brush("color.surface-column");
        Content = Skin.Column(Theme, items.ToArray()); Padding = new Thickness(Theme.Number("spacing.2.5"));
    }
}
