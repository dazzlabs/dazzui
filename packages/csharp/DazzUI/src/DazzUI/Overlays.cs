using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;

namespace DazzUI;

public sealed record MenuItem(string Label, Action? Action = null, bool Disabled = false,
    bool? Checked = null, IReadOnlyList<MenuItem>? Children = null, string? Shortcut = null, bool Separator = false,
    Glyph? Glyph = null);

public class Menu
{
    public IList<MenuItem> Items { get; } = new List<MenuItem>();
    public MenuFlyout Flyout { get; } = new();
    public void Attach(FrameworkElement target) { Build(Appearance.GetTheme(target)); if (target is Microsoft.UI.Xaml.Controls.Button button) button.Flyout = Flyout; }
    public void Show(FrameworkElement target) { Build(Appearance.GetTheme(target)); Flyout.ShowAt(target); }
    protected void Build(Theme theme)
    {
        Flyout.Items.Clear();
        foreach (var item in Items) Flyout.Items.Add(BuildItem(item, theme));
        var style = new Style(typeof(MenuFlyoutPresenter));
        style.Setters.Add(new Setter(Control.BackgroundProperty, theme.Brush("color.surface-overlay")));
        style.Setters.Add(new Setter(Control.ForegroundProperty, theme.Brush("color.content")));
        style.Setters.Add(new Setter(Control.BorderBrushProperty, theme.Brush("color.border-strong")));
        style.Setters.Add(new Setter(Control.CornerRadiusProperty, new CornerRadius(theme.Number("radius.large"))));
        style.Setters.Add(new Setter(FrameworkElement.MinWidthProperty, theme.Number("menu.min-width")));
        Flyout.MenuFlyoutPresenterStyle = style;
    }
    internal static MenuFlyoutItemBase BuildItem(MenuItem item, Theme theme)
    {
        if (item.Separator) return new MenuFlyoutSeparator();
        if (item.Children is not null)
        {
            var parent = new MenuFlyoutSubItem { CornerRadius = new CornerRadius(theme.Number("radius.small")), Text = item.Label, IsEnabled = !item.Disabled };
            Skin.RoundCorners(parent);
            foreach (var child in item.Children) parent.Items.Add(BuildItem(child, theme));
            return parent;
        }
        if (item.Checked is bool check)
        {
            var toggle = new ToggleMenuFlyoutItem { CornerRadius = new CornerRadius(theme.Number("radius.small")), Text = item.Label, IsChecked = check, IsEnabled = !item.Disabled };
            Skin.RoundCorners(toggle);
            toggle.Click += (_, _) => item.Action?.Invoke(); return toggle;
        }
        var result = new MenuFlyoutItem { CornerRadius = new CornerRadius(theme.Number("radius.small")), Text = item.Label, IsEnabled = !item.Disabled, KeyboardAcceleratorTextOverride = item.Shortcut ?? "", Foreground = theme.Brush("color.content") };
        if (item.Glyph is Glyph glyph) result.Icon = Icon.Element(glyph);
        Skin.RoundCorners(result);
        result.Click += (_, _) => item.Action?.Invoke(); return result;
    }
}
public sealed class ContextMenu : Menu
{
    public new void Attach(FrameworkElement target) { Build(Appearance.GetTheme(target)); target.ContextFlyout = Flyout; }
}
/// <summary>
/// A row of menu triggers. The native <c>MenuBar</c> floors its row at forty
/// and measures each title wider than React's, so the bar is drawn as the
/// kit's own plain buttons and keeps the keyboard movement a menubar has.
/// </summary>
public sealed class Menubar : Surface
{
    public IList<(string Label, Menu Menu)> Menus { get; } = new List<(string, Menu)>();
    protected override void RefreshAppearance()
    {
        var row = new StackPanel { Orientation = Orientation.Horizontal, Spacing = Theme.Number("spacing.0.5") };
        var triggers = new List<MenubarItem>();
        foreach (var (label, menu) in Menus)
        {
            var trigger = new MenubarItem
            {
                Content = label,
                AccessKey = label[..1],
                Size = ControlSize.Small,
                Variant = ControlVariant.Plain,
                Tint = WidgetTint.Neutral,
            };
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(trigger, label);
            // The flyout is built from the trigger's theme, and the trigger
            // is not in the tree yet.
            Appearance.SetTheme(trigger, Theme);
            menu.Attach(trigger);
            trigger.KeyDown += (_, args) =>
            {
                var step = args.Key is Windows.System.VirtualKey.Left or Windows.System.VirtualKey.Up ? -1
                    : args.Key is Windows.System.VirtualKey.Right or Windows.System.VirtualKey.Down ? 1 : 0;
                if (step == 0) return;
                var at = triggers.IndexOf(trigger);
                triggers[(at + step + triggers.Count) % triggers.Count].Focus(FocusState.Keyboard);
                args.Handled = true;
            };
            triggers.Add(trigger);
            row.Children.Add(trigger);
        }
        Content = row;
    }
}
/// <summary>A menubar trigger: a plain button at the menubar's own eight-pixel inline inset.</summary>
public sealed class MenubarItem : Button
{
    protected override void LayContent()
    {
        base.LayContent();
        var pad = Theme.Number("spacing.2");
        Padding = new Thickness(pad, 0, pad, 0);
    }
    private protected override bool Raised => false;
}
public class Popover
{
    public Flyout Flyout { get; } = new();
    public UIElement? Content { get => Flyout.Content; set => Flyout.Content = value; }
    public FlyoutPlacementMode Placement { get => Flyout.Placement; set => Flyout.Placement = value; }
    public bool IsOpen => Flyout.IsOpen;
    public void Show(FrameworkElement anchor)
    {
        var theme = Appearance.GetTheme(anchor);
        var style = new Style(typeof(FlyoutPresenter));
        style.Setters.Add(new Setter(Control.BackgroundProperty, theme.Brush("color.surface-overlay")));
        style.Setters.Add(new Setter(Control.ForegroundProperty, theme.Brush("color.content")));
        style.Setters.Add(new Setter(Control.BorderBrushProperty, theme.Brush("color.border-strong")));
        style.Setters.Add(new Setter(Control.CornerRadiusProperty, new CornerRadius(theme.Number("radius.large"))));
        Flyout.FlyoutPresenterStyle = style;
        if (Content is not null) Appearance.SetTheme(Content, theme);
        Flyout.ShowAt(anchor);
    }
    public void Hide() => Flyout.Hide();
}
public sealed class PreviewCard : Popover
{
    public void Attach(FrameworkElement anchor)
    {
        var timer = new DispatcherTimer { Interval = TimeSpan.FromMilliseconds(350) }; // Intent delay, not geometry.
        timer.Tick += (_, _) => { timer.Stop(); Show(anchor); };
        anchor.PointerEntered += (_, _) => timer.Start();
        anchor.PointerExited += (_, _) => timer.Stop();
        anchor.Unloaded += (_, _) => { timer.Stop(); Hide(); };
        anchor.GotFocus += (_, _) => Show(anchor);
    }
}
public static class Tooltip
{
    public static void Set(DependencyObject target, string text, PlacementMode placement = PlacementMode.Top)
    {
        var theme = Appearance.GetTheme(target);
        var tooltip = new ToolTip { Content = text, Placement = placement, Background = theme.Brush("color.surface-overlay"), Foreground = theme.Brush("color.content"), CornerRadius = new CornerRadius(theme.Number("radius.small")) };
        Skin.RoundCorners(tooltip);
        ToolTipService.SetToolTip(target, tooltip);
    }
}
public class Dialog
{
    public ContentDialog Native { get; } = new();
    public string Title { get => Native.Title?.ToString() ?? ""; set => Native.Title = value; }
    public UIElement? Content { get => Native.Content as UIElement; set => Native.Content = value; }
    public string PrimaryText { get => Native.PrimaryButtonText; set => Native.PrimaryButtonText = value; }
    public string CloseText { get => Native.CloseButtonText; set => Native.CloseButtonText = value; }
    public bool Danger { get; set; }
    /// <summary>The inset around the content; <c>spacing.6</c> when unset. Zero lets content that draws its own header and footer run to the edge.</summary>
    public Thickness? Padding { get; set; }
    protected Theme ActiveTheme { get; private set; } = Theme.Default;
    public Dialog()
    {
        Native.CloseButtonText = "Cancel"; Native.PrimaryButtonText = "Continue";
        Native.DefaultButton = ContentDialogButton.Primary;
        Native.Opened += (_, _) => ArrangePanel();
        Native.SizeChanged += (_, _) => ArrangePanel();
    }
    protected virtual void ArrangePanel()
    {
        foreach (var element in Descendants(Native))
        {
            if (element is Control { Name: "Title" } title) ActiveTheme.Typography(title, "titleLarge");
            if (element is ScrollViewer { Name: "ContentScrollViewer" } scroll) scroll.VerticalContentAlignment = VerticalAlignment.Top;
            if (element is ContentPresenter { Name: "Content" } content) content.VerticalAlignment = VerticalAlignment.Top;
            if (element is Microsoft.UI.Xaml.Controls.Button button && button.Name is "PrimaryButton" or "SecondaryButton" or "CloseButton")
            {
                Skin.Control(button, ActiveTheme, ControlSize.Medium);
                button.Height = ActiveTheme.Number("control.medium.size");
            }
        }
    }
    private static IEnumerable<DependencyObject> Descendants(DependencyObject root)
    {
        for (var i = 0; i < Microsoft.UI.Xaml.Media.VisualTreeHelper.GetChildrenCount(root); i++)
        {
            var child = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetChild(root, i);
            yield return child;
            foreach (var descendant in Descendants(child)) yield return descendant;
        }
    }
    protected FrameworkElement? Panel => FindPanel(Native);
    private static FrameworkElement? FindPanel(DependencyObject root)
    {
        if (root is FrameworkElement { Name: "BackgroundElement" } panel) return panel;
        for (var i = 0; i < Microsoft.UI.Xaml.Media.VisualTreeHelper.GetChildrenCount(root); i++)
        {
            var child = FindPanel(Microsoft.UI.Xaml.Media.VisualTreeHelper.GetChild(root, i));
            if (child is not null) return child;
        }
        return null;
    }
    public virtual async Task<ContentDialogResult> ShowAsync(FrameworkElement owner)
    {
        var theme = Appearance.GetTheme(owner);
        ActiveTheme = theme;
        Native.XamlRoot = owner.XamlRoot ?? throw new InvalidOperationException("Show a dialog after its owner is loaded.");
        Native.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
        Native.Resources["ContentDialogBackground"] = theme.Brush("color.surface-overlay");
        Native.Resources["ContentDialogTopOverlay"] = theme.Brush("color.surface-overlay");
        Native.Resources["ContentDialogForeground"] = theme.Brush("color.content");
        Native.Resources["ContentDialogMaxWidth"] = theme.Number("dialog.width");
        Native.Resources["ContentDialogBorderBrush"] = theme.Brush("color.border-strong");
        Native.Resources["ContentDialogSmokeFill"] = theme.Alpha("dialog.scrim.color", theme.Number("dialog.scrim.alpha"));
        Native.Resources["ContentDialogPadding"] = Padding ?? new Thickness(theme.Number("spacing.6"));
        Native.Resources["ContentDialogCornerRadius"] = new CornerRadius(theme.Number("frame.window.radius"));
        Native.Resources["AccentButtonBackground"] = theme.Recipe(ControlVariant.Filled, Danger ? WidgetTint.Danger : WidgetTint.Primary, "surface");
        if (Content is not null) Appearance.SetTheme(Content, theme);
        return await Native.ShowAsync();
    }
    public void Hide() => Native.Hide();
}
public enum DrawerSide { Left, Right, Top, Bottom }
public sealed class Drawer : Dialog
{
    public DrawerSide Side { get; set; } = DrawerSide.Right;
    public double? Extent { get; set; }
    private double extent;
    protected override void ArrangePanel()
    {
        base.ArrangePanel();
        if (Panel is not FrameworkElement panel || Native.XamlRoot is null) return;
        var bounds = Native.XamlRoot.Size;
        var horizontal = Side is DrawerSide.Left or DrawerSide.Right;
        panel.MinWidth = panel.MinHeight = 0;
        panel.MaxWidth = bounds.Width; panel.MaxHeight = bounds.Height;
        panel.Width = horizontal ? Math.Min(extent, bounds.Width) : bounds.Width;
        panel.Height = horizontal ? bounds.Height : Math.Min(extent, bounds.Height);
        panel.HorizontalAlignment = Side == DrawerSide.Left ? HorizontalAlignment.Left : Side == DrawerSide.Right ? HorizontalAlignment.Right : HorizontalAlignment.Stretch;
        panel.VerticalAlignment = Side == DrawerSide.Top ? VerticalAlignment.Top : Side == DrawerSide.Bottom ? VerticalAlignment.Bottom : VerticalAlignment.Stretch;
        if (panel is Border border) border.CornerRadius = new CornerRadius(0);
    }
    public override Task<ContentDialogResult> ShowAsync(FrameworkElement owner)
    {
        var theme = Appearance.GetTheme(owner);
        extent = Extent ?? theme.Number("drawer.size");
        Native.HorizontalAlignment = Side == DrawerSide.Left ? HorizontalAlignment.Left : Side == DrawerSide.Right ? HorizontalAlignment.Right : HorizontalAlignment.Stretch;
        Native.VerticalAlignment = Side == DrawerSide.Top ? VerticalAlignment.Top : Side == DrawerSide.Bottom ? VerticalAlignment.Bottom : VerticalAlignment.Stretch;
        Native.MinWidth = Side is DrawerSide.Left or DrawerSide.Right ? theme.Number("drawer.size") : 0;
        return base.ShowAsync(owner);
    }
}
public sealed record ToastOptions(string Title, string? Description = null, WidgetTint Tint = WidgetTint.Neutral,
    string? ActionLabel = null, Action? Action = null, TimeSpan? Duration = null,
    Glyph? Leading = null, bool? Dismissible = null, string DismissLabel = "Dismiss");
public sealed class Toast : Surface
{
    public ToastOptions Options { get; set; } = new("Saved");
    public event EventHandler? Dismissed;
    protected override void RefreshAppearance()
    {
        var body = new Grid { ColumnSpacing = Theme.Number("spacing.2.5"), VerticalAlignment = VerticalAlignment.Center };
        void Add(FrameworkElement child, bool flexible = false)
        {
            body.ColumnDefinitions.Add(new ColumnDefinition { Width = flexible ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
            Grid.SetColumn(child, body.ColumnDefinitions.Count - 1); body.Children.Add(child);
        }
        if (Options.Leading is Glyph glyph) Add(new Icon { Glyph = glyph, Foreground = Theme.Recipe(ControlVariant.Plain, Options.Tint, "content"), VerticalAlignment = VerticalAlignment.Center });
        var message = Theme.Text(Options.Title, "labelMedium");
        message.Foreground = Theme.Brush("color.content"); message.TextTrimming = TextTrimming.CharacterEllipsis;
        message.VerticalAlignment = VerticalAlignment.Center;
        if (Options.Description is null) Add(message, true);
        else Add(Skin.Column(Theme, message, Theme.Text(Options.Description, "bodySmall")), true);
        if (Options.ActionLabel is not null)
        {
            var action = new Button { Content = Options.ActionLabel, Variant = ControlVariant.Plain, Size = ControlSize.Tiny };
            action.Click += (_, _) => Options.Action?.Invoke(); Add(action);
        }
        if (Options.Dismissible ?? (Options.Duration <= TimeSpan.Zero || Options.Duration is null && Options.Tint == WidgetTint.Danger))
        {
            var close = new IconButton { Glyph = FluentIcons.Common.Icon.Dismiss, Label = Options.DismissLabel };
            close.Click += (_, _) => Dismissed?.Invoke(this, EventArgs.Empty); Add(close);
        }
        Content = body;
        HorizontalAlignment = HorizontalAlignment.Center; VerticalContentAlignment = VerticalAlignment.Center;
        Padding = new Thickness(Theme.Number("spacing.3"), Theme.Number("spacing.1.5"), Theme.Number("spacing.1.5"), Theme.Number("spacing.1.5"));
        MinHeight = Theme.Number("spacing.9");
        Background = Theme.Brush("color.surface-overlay"); BorderBrush = Theme.Brush("color.border-strong");
        BorderThickness = new Thickness(Theme.Number("stroke.hairline")); CornerRadius = new CornerRadius(Theme.Number("radius.big"));
        MaxWidth = Theme.Number("toast.max-width");
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetLiveSetting(this, Microsoft.UI.Xaml.Automation.Peers.AutomationLiveSetting.Polite);
    }
}
public sealed class ToastViewport : Surface
{
    private readonly StackPanel stack = new();
    private readonly Dictionary<Toast, DispatcherTimer> timers = [];
    public ToastViewport() { Content = stack; Unloaded += (_, _) => { foreach (var timer in timers.Values) timer.Stop(); timers.Clear(); stack.Children.Clear(); }; }
    public Toast Add(ToastOptions options)
    {
        var toast = new Toast { Options = options };
        toast.Dismissed += (_, _) => Remove(toast); stack.Children.Add(toast);
        var duration = options.Duration ?? (options.Tint == WidgetTint.Danger ? TimeSpan.Zero : TimeSpan.FromSeconds(options.ActionLabel is null ? 4 : 6));
        if (duration > TimeSpan.Zero)
        {
            var timer = new DispatcherTimer { Interval = duration };
            timer.Tick += (_, _) => Remove(toast); timers[toast] = timer; timer.Start();
            toast.PointerEntered += (_, _) => timer.Stop(); toast.PointerExited += (_, _) => timer.Start();
        }
        return toast;
    }
    public void Remove(Toast toast) { if (timers.Remove(toast, out var timer)) timer.Stop(); stack.Children.Remove(toast); }
    protected override void RefreshAppearance() { stack.Spacing = Theme.Number("spacing.3"); }
}
