using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Input;

namespace DazzUI;

public sealed record MenuItem(string Label, Action? Action = null, bool Disabled = false,
    bool? Checked = null, IReadOnlyList<MenuItem>? Children = null, string? Shortcut = null, bool Separator = false,
    Glyph? Glyph = null);

public class Menu
{
    public IList<MenuItem> Items { get; } = new List<MenuItem>();
    public MenuFlyout Flyout { get; } = new() { AreOpenCloseAnimationsEnabled = false, Placement = FlyoutPlacementMode.BottomEdgeAlignedRight };
    public FlyoutPlacementMode Placement { get => Flyout.Placement; set => Flyout.Placement = value; }
    public PopupSide Side { get => OverlayPlacement.Side(Placement); set => Placement = OverlayPlacement.Resolve(value, Align); }
    public PopupAlignment Align { get => OverlayPlacement.Align(Placement); set => Placement = OverlayPlacement.Resolve(Side, value); }
    protected FrameworkElement? Owner { get; set; }
    private Theme? appliedTheme;
    private double appliedScale;
    public Menu()
    {
        Flyout.Opening += (_, _) => { if (Owner is not null) Build(Appearance.GetTheme(Owner)); };
        Flyout.Opened += (_, _) =>
        {
            if (Owner is null) return;
            Owner.LayoutUpdated += FollowAppearance;
            Owner.Unloaded += OwnerUnloaded;
        };
        Flyout.Closed += (_, _) =>
        {
            if (Owner is null) return;
            Owner.LayoutUpdated -= FollowAppearance;
            Owner.Unloaded -= OwnerUnloaded;
        };
    }
    private void OwnerUnloaded(object sender, RoutedEventArgs args) => Flyout.Hide();
    private void FollowAppearance(object? sender, object args)
    {
        if (Owner is null || !Flyout.IsOpen) return;
        var theme = Appearance.GetTheme(Owner);
        if (ReferenceEquals(theme, appliedTheme) && appliedScale == theme.LayoutScale) return;
        appliedTheme = theme;
        appliedScale = theme.LayoutScale;
        var presenters = new HashSet<MenuFlyoutPresenter>();
        void Paint(IEnumerable<MenuFlyoutItemBase> items)
        {
            foreach (var item in items)
            {
                PopupSkin.Repaint(item, theme);
                for (DependencyObject? parent = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(item); parent is not null; parent = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(parent))
                    if (parent is MenuFlyoutPresenter presenter) { presenters.Add(presenter); break; }
                if (item is MenuFlyoutSubItem branch) Paint(branch.Items);
            }
        }
        Paint(Flyout.Items);
        foreach (var presenter in presenters) PopupSkin.Repaint(presenter, theme);
    }
    public void Attach(FrameworkElement target) { Owner = target; Build(Appearance.GetTheme(target)); if (target is Microsoft.UI.Xaml.Controls.Button button) button.Flyout = Flyout; }
    public void Show(FrameworkElement target) { Owner = target; OverlayPlacement.Show(Flyout, target); }
    protected void Build(Theme theme)
    {
        appliedTheme = theme;
        appliedScale = theme.LayoutScale;
        Flyout.Items.Clear();
        BuildItems(Flyout.Items, () => Items.ToArray(), theme);
        Flyout.MenuFlyoutPresenterStyle = PopupSkin.Presenter(theme);
    }
    private static void BuildItems(IList<MenuFlyoutItemBase> target, Func<IReadOnlyList<MenuItem>> source, Theme theme)
    {
        var items = source();
        int Selected()
        {
            var current = source();
            for (var i = 0; i < current.Count; i++) if (current[i].Checked == true) return i;
            return -1;
        }
        void Sync()
        {
            var selected = Selected();
            for (var i = 0; i < target.Count; i++)
                if (target[i] is ToggleMenuFlyoutItem toggle) toggle.IsChecked = i == selected;
        }
        for (var i = 0; i < items.Count; i++)
        {
            var index = i;
            var built = BuildItem(items[i], theme);
            target.Add(built);
            if (built is MenuFlyoutSubItem submenu)
                BuildItems(submenu.Items, () => source().ElementAtOrDefault(index)?.Children ?? [], theme);
            if (built is ToggleMenuFlyoutItem toggle)
                toggle.Click += (_, _) =>
                {
                    // React's checks form one controlled radio group. Native
                    // activation still dismisses, but a proposal cannot toggle
                    // the selected item off or outrun the caller-owned model.
                    if (index != Selected()) source().ElementAtOrDefault(index)?.Action?.Invoke();
                    Sync();
                };
        }
        Sync();
    }
    private static MenuFlyoutItemBase BuildItem(MenuItem item, Theme theme)
    {
        if (item.Separator)
        {
            var separator = new MenuFlyoutSeparator();
            PopupSkin.Item(separator, theme);
            return separator;
        }
        if (item.Children is not null)
        {
            var parent = new MenuFlyoutSubItem { CornerRadius = new CornerRadius(theme.Number("radius.small")), Text = item.Label, IsEnabled = !item.Disabled };
            if (item.Glyph is Glyph parentGlyph) parent.Icon = Icon.Element(parentGlyph);
            Skin.RoundCorners(parent);
            PopupSkin.Item(parent, theme);
            return parent;
        }
        if (item.Checked is bool check)
        {
            var toggle = new ToggleMenuFlyoutItem { CornerRadius = new CornerRadius(theme.Number("radius.small")), Text = item.Label, IsChecked = check, IsEnabled = !item.Disabled };
            toggle.KeyboardAcceleratorTextOverride = item.Shortcut ?? "";
            if (item.Glyph is Glyph toggleGlyph) toggle.Icon = Icon.Element(toggleGlyph);
            Skin.RoundCorners(toggle);
            PopupSkin.Item(toggle, theme);
            return toggle;
        }
        var result = new MenuFlyoutItem { CornerRadius = new CornerRadius(theme.Number("radius.small")), Text = item.Label, IsEnabled = !item.Disabled, KeyboardAcceleratorTextOverride = item.Shortcut ?? "", Foreground = theme.Brush("color.content") };
        if (item.Glyph is Glyph glyph) result.Icon = Icon.Element(glyph);
        Skin.RoundCorners(result);
        PopupSkin.Item(result, theme);
        result.Click += (_, _) => item.Action?.Invoke(); return result;
    }
}
public sealed class ContextMenu : Menu
{
    public new void Attach(FrameworkElement target) { Owner = target; Build(Appearance.GetTheme(target)); target.ContextFlyout = Flyout; }
}
/// <summary>
/// A row of menu triggers. The native <c>MenuBar</c> floors its row at forty
/// and measures each title wider than React's, so the bar is drawn as the
/// kit's own plain buttons and keeps the keyboard movement a menubar has.
/// </summary>
public sealed class Menubar : Surface
{
    private readonly StackPanel row = new() { Orientation = Orientation.Horizontal };
    private readonly List<Entry> entries = [];
    private int navigation;
    private sealed class Entry(Menubar owner, Menu menu, MenubarItem trigger)
    {
        internal Menu Menu { get; } = menu;
        internal MenubarItem Trigger { get; } = trigger;
        private readonly List<MenuFlyoutItemBase> watched = [];
        internal bool FocusLast { get; set; }
        internal void Opened(object? sender, object args)
        {
            Trigger.PopupOpen = true;
            foreach (var item in Menu.Flyout.Items)
            {
                item.PreviewKeyDown += Navigate;
                item.PreviewKeyUp += FinishNavigation;
                watched.Add(item);
            }
            if (FocusLast)
            {
                FocusLast = false;
                Menu.Flyout.Items.LastOrDefault(item => item is not MenuFlyoutSeparator && item.Visibility == Visibility.Visible)?.Focus(FocusState.Keyboard);
            }
        }
        private void Navigate(object sender, KeyRoutedEventArgs args)
        {
            if (args.Key is not (Windows.System.VirtualKey.Left or Windows.System.VirtualKey.Right)) return;
            // Nested rows keep the native submenu's Left/Right behavior. A
            // root branch also keeps Right so it can open its own submenu.
            if (!ReferenceEquals(FocusManager.GetFocusedElement(Trigger.XamlRoot), sender)
                || args.Key == Windows.System.VirtualKey.Right && sender is MenuFlyoutSubItem) return;
            owner.Switch(this, args.Key == Windows.System.VirtualKey.Left ? -1 : 1, FocusState.Keyboard);
            args.Handled = true;
        }
        private void FinishNavigation(object sender, KeyRoutedEventArgs args)
        {
            // The release can arrive at the newly opened native presenter.
            // Keep it from applying Left a second time and closing that menu.
            if (args.Key is Windows.System.VirtualKey.Left or Windows.System.VirtualKey.Right
                && ReferenceEquals(FocusManager.GetFocusedElement(Trigger.XamlRoot), sender)) args.Handled = true;
        }
        internal void Closed(object? sender, object args)
        {
            Trigger.PopupOpen = false;
            foreach (var item in watched) { item.PreviewKeyDown -= Navigate; item.PreviewKeyUp -= FinishNavigation; }
            watched.Clear();
        }
        internal void Detach()
        {
            Menu.Flyout.Opened -= Opened;
            Menu.Flyout.Closed -= Closed;
            Closed(null, EventArgs.Empty);
            Menu.Flyout.Hide();
            Trigger.Flyout = null;
        }
    }
    public IList<(string Label, Menu Menu)> Menus { get; } = new List<(string, Menu)>();
    public Menubar()
    {
        Content = row;
        Unloaded += (_, _) =>
        {
            navigation++;
            foreach (var entry in entries) entry.Menu.Flyout.Hide();
        };
    }
    private void Switch(Entry source, int delta, FocusState focus)
    {
        var at = entries.IndexOf(source);
        if (at < 0 || entries.Count < 2) return;
        Open(entries[(at + delta + entries.Count) % entries.Count], focus);
    }
    private void Open(Entry destination, FocusState focus)
    {
        var ticket = ++navigation;
        foreach (var entry in entries) if (!ReferenceEquals(entry, destination)) entry.Menu.Flyout.Hide();
        // Let the old flyout return focus before the new native presenter
        // chooses its row. Only the latest traversal can open a destination.
        DispatcherQueue.TryEnqueue(() =>
        {
            if (ticket != navigation || !IsLoaded || !entries.Contains(destination)) return;
            destination.Trigger.IsTabStop = true;
            destination.Trigger.Focus(focus);
            destination.Menu.Show(destination.Trigger);
        });
    }
    protected override void RefreshAppearance()
    {
        row.Spacing = Theme.Number("spacing.0.5");
        if (entries.Count != Menus.Count || entries.Where((entry, index) => !ReferenceEquals(entry.Menu, Menus[index].Menu)).Any())
        {
            navigation++;
            foreach (var entry in entries) entry.Detach();
            entries.Clear();
            row.Children.Clear();
            foreach (var (label, menu) in Menus)
            {
                menu.Side = PopupSide.Bottom;
                menu.Align = PopupAlignment.Start;
                var trigger = new MenubarItem { Content = label, Size = ControlSize.Small, Variant = ControlVariant.Normal, Tint = WidgetTint.Neutral };
                Appearance.SetTheme(trigger, Theme);
                menu.Attach(trigger);
                var entry = new Entry(this, menu, trigger);
                menu.Flyout.Opened += entry.Opened;
                menu.Flyout.Closed += entry.Closed;
                trigger.IsTabStop = entries.Count == 0;
                trigger.GotFocus += (_, _) =>
                {
                    foreach (var item in entries) item.Trigger.IsTabStop = ReferenceEquals(item.Trigger, trigger);
                };
                trigger.PointerEntered += (_, _) =>
                {
                    if (entries.Any(item => !ReferenceEquals(item, entry) && item.Menu.Flyout.IsOpen)) Open(entry, FocusState.Pointer);
                };
                menu.Flyout.OverlayInputPassThroughElement = row;
                trigger.KeyDown += (_, args) =>
                {
                    if (args.Key is Windows.System.VirtualKey.Up or Windows.System.VirtualKey.Down)
                    {
                        entry.FocusLast = args.Key == Windows.System.VirtualKey.Up;
                        menu.Show(trigger);
                        args.Handled = true;
                        return;
                    }
                    var at = entries.IndexOf(entry);
                    var next = args.Key switch
                    {
                        Windows.System.VirtualKey.Left => (at + entries.Count - 1) % entries.Count,
                        Windows.System.VirtualKey.Right => (at + 1) % entries.Count,
                        Windows.System.VirtualKey.Home => 0,
                        Windows.System.VirtualKey.End => entries.Count - 1,
                        _ => -1,
                    };
                    if (next < 0) return;
                    var destination = entries[next].Trigger;
                    var wasTabStop = destination.IsTabStop;
                    // WinUI refuses keyboard focus on a non-tab-stop button.
                    // Promote the destination first; GotFocus retires its peers.
                    destination.IsTabStop = true;
                    if (!destination.Focus(FocusState.Keyboard)) destination.IsTabStop = wasTabStop;
                    args.Handled = true;
                };
                entries.Add(entry);
                row.Children.Add(trigger);
            }
        }
        for (var i = 0; i < entries.Count; i++)
        {
            var trigger = entries[i].Trigger;
            var label = Menus[i].Label;
            trigger.Content = label;
            trigger.AccessKey = label.Length == 0 ? "" : label[..1];
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(trigger, label);
            Appearance.SetTheme(trigger, Theme);
        }
    }
}
/// <summary>A menubar trigger: a plain button at the menubar's own eight-pixel inline inset.</summary>
public sealed class MenubarItem : Button
{
    private bool popupOpen;
    internal bool PopupOpen { get => popupOpen; set { popupOpen = value; Appearance.Refresh(this); } }
    protected override void Tailor()
    {
        BorderThickness = new Thickness(0);
        Skin.SetCorner(this, Theme.Number("radius.small"));
        if (popupOpen) Repaint("ButtonBackground", Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered").Color);
    }
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
    public Flyout Flyout { get; } = new() { AreOpenCloseAnimationsEnabled = false, Placement = FlyoutPlacementMode.Bottom };
    public UIElement? Content { get => Flyout.Content; set => Flyout.Content = value; }
    public FlyoutPlacementMode Placement { get => Flyout.Placement; set => Flyout.Placement = value; }
    public PopupSide Side { get => OverlayPlacement.Side(Placement); set => Placement = OverlayPlacement.Resolve(value, Align); }
    public PopupAlignment Align { get => OverlayPlacement.Align(Placement); set => Placement = OverlayPlacement.Resolve(Side, value); }
    public bool IsOpen => Flyout.IsOpen;
    /// <summary>The full border-box width, including the shared prose inset.</summary>
    public double? Width { get; set; }
    protected virtual double MaximumWidth(Theme theme, FrameworkElement anchor) => anchor.XamlRoot.Size.Width;
    protected FlyoutPresenter? Presenter
    {
        get
        {
            for (DependencyObject? current = Content; current is not null; current = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(current))
                if (current is FlyoutPresenter presenter) return presenter;
            return null;
        }
    }
    private FrameworkElement? owner;
    private Theme? appliedTheme;
    private double appliedScale;
    private double appliedMaximum;
    private double? appliedWidth;
    public Popover()
    {
        Flyout.Opened += (_, _) =>
        {
            if (owner is null) return;
            owner.LayoutUpdated += FollowAppearance;
            owner.Unloaded += OwnerUnloaded;
            if (Presenter is FlyoutPresenter presenter) OverlaySkin.Lift(presenter, Appearance.GetTheme(owner), "lg");
        };
        Flyout.Closed += (_, _) =>
        {
            if (owner is null) return;
            owner.LayoutUpdated -= FollowAppearance;
            owner.Unloaded -= OwnerUnloaded;
        };
    }
    private void OwnerUnloaded(object sender, RoutedEventArgs args) => Hide();
    private void FollowAppearance(object? sender, object args)
    {
        if (owner is null || !IsOpen || Presenter is not FlyoutPresenter presenter) return;
        var theme = Appearance.GetTheme(owner);
        var maximum = MaximumWidth(theme, owner);
        if (ReferenceEquals(theme, appliedTheme) && appliedScale == theme.LayoutScale && maximum == appliedMaximum && Width == appliedWidth) return;
        RememberAppearance(theme, maximum);
        // Apply palette and geometry without replacing the native template or
        // its editor descendants, so focus and in-progress input survive.
        OverlaySkin.UpdateProse(presenter, theme, maximum, Width);
        if (Content is not null) Appearance.SetTheme(Content, theme);
        OverlaySkin.Lift(presenter, theme, "lg");
    }
    private void RememberAppearance(Theme theme, double maximum)
    {
        appliedTheme = theme;
        appliedScale = theme.LayoutScale;
        appliedMaximum = maximum;
        appliedWidth = Width;
    }
    public void Show(FrameworkElement anchor)
    {
        if (IsOpen) return;
        owner = anchor;
        var theme = Appearance.GetTheme(anchor);
        var maximum = MaximumWidth(theme, anchor);
        RememberAppearance(theme, maximum);
        Flyout.FlyoutPresenterStyle = OverlaySkin.Prose(theme, maximum, Width);
        if (Content is not null) Appearance.SetTheme(Content, theme);
        OverlayPlacement.Show(Flyout, anchor);
    }
    public void Hide() => Flyout.Hide();
}
public sealed class PreviewCard : Popover
{
    private Action? detach;
    protected override double MaximumWidth(Theme theme, FrameworkElement anchor) =>
        Math.Min(base.MaximumWidth(theme, anchor), theme.Number("preview-card.width"));
    public void Attach(FrameworkElement anchor)
    {
        detach?.Invoke();
        var timer = new DispatcherTimer();
        var closing = new DispatcherTimer();
        FlyoutPresenter? panel = null;
        var overAnchor = false;
        var overPanel = false;
        var interactedInside = false;
        var attached = true;
        Flyout.ShowMode = FlyoutShowMode.Transient;
        bool HasFocus()
        {
            if (anchor.XamlRoot is null) return false;
            var focused = FocusManager.GetFocusedElement(anchor.XamlRoot) as DependencyObject;
            for (var current = focused; current is not null; current = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(current))
                if (ReferenceEquals(current, anchor) || ReferenceEquals(current, panel)) return true;
            return false;
        }
        void CancelClose() => closing.Stop();
        void CloseSoon()
        {
            timer.Stop();
            if (overAnchor || overPanel || interactedInside || HasFocus()) return;
            closing.Interval = TimeSpan.FromMilliseconds(Appearance.GetTheme(anchor).Milliseconds("preview-card.close-delay"));
            closing.Start();
        }
        closing.Tick += (_, _) =>
        {
            closing.Stop();
            if (!overAnchor && !overPanel && !interactedInside && !HasFocus()) Hide();
        };
        timer.Tick += (_, _) => { timer.Stop(); if (anchor.IsLoaded) Show(anchor); };
        void AnchorEntered(object sender, PointerRoutedEventArgs args)
        {
            overAnchor = true;
            CancelClose();
            timer.Interval = TimeSpan.FromMilliseconds(Appearance.GetTheme(anchor).Milliseconds("preview-card.open-delay"));
            timer.Start();
        }
        void AnchorExited(object sender, PointerRoutedEventArgs args) { overAnchor = false; CloseSoon(); }
        void PanelEntered(object sender, PointerRoutedEventArgs args) { overPanel = true; CancelClose(); }
        void PanelExited(object sender, PointerRoutedEventArgs args) { overPanel = false; CloseSoon(); }
        void PanelPressed(object sender, PointerRoutedEventArgs args) { interactedInside = true; CancelClose(); }
        void AnchorFocused(object sender, RoutedEventArgs args) { timer.Stop(); CancelClose(); Show(anchor); }
        void PanelFocused(object sender, RoutedEventArgs args) => CancelClose();
        void FocusLeft(object sender, RoutedEventArgs args)
        {
            // GotFocus follows LostFocus when moving between children. Inspect
            // the settled focus tree before dismissing the containing card.
            anchor.DispatcherQueue.TryEnqueue(() =>
            {
                if (!attached || !anchor.IsLoaded) return;
                if (!HasFocus()) { interactedInside = false; CloseSoon(); }
            });
        }
        var pressHandler = new PointerEventHandler(PanelPressed);
        void WatchPanel(FlyoutPresenter? next)
        {
            if (ReferenceEquals(panel, next)) return;
            if (panel is not null)
            {
                panel.PointerEntered -= PanelEntered; panel.PointerExited -= PanelExited;
                panel.GotFocus -= PanelFocused; panel.LostFocus -= FocusLeft;
                panel.RemoveHandler(UIElement.PointerPressedEvent, pressHandler);
            }
            panel = next;
            overPanel = false;
            if (panel is null) return;
            panel.PointerEntered += PanelEntered; panel.PointerExited += PanelExited;
            panel.GotFocus += PanelFocused; panel.LostFocus += FocusLeft;
            panel.AddHandler(UIElement.PointerPressedEvent, pressHandler, true);
        }
        void Opened(object? sender, object args) => WatchPanel(Presenter);
        void Closed(object? sender, object args) { timer.Stop(); closing.Stop(); interactedInside = false; WatchPanel(null); }
        void Unloaded(object sender, RoutedEventArgs args) { timer.Stop(); closing.Stop(); Hide(); WatchPanel(null); }
        anchor.PointerEntered += AnchorEntered; anchor.PointerExited += AnchorExited;
        anchor.GotFocus += AnchorFocused; anchor.LostFocus += FocusLeft; anchor.Unloaded += Unloaded;
        Flyout.Opened += Opened; Flyout.Closed += Closed;
        detach = () =>
        {
            attached = false;
            timer.Stop(); closing.Stop(); Hide(); WatchPanel(null);
            anchor.PointerEntered -= AnchorEntered; anchor.PointerExited -= AnchorExited;
            anchor.GotFocus -= AnchorFocused; anchor.LostFocus -= FocusLeft; anchor.Unloaded -= Unloaded;
            Flyout.Opened -= Opened; Flyout.Closed -= Closed;
        };
    }
}
public static class Tooltip
{
    public static void Set(DependencyObject target, string text, PlacementMode placement = PlacementMode.Top, PopupAlignment align = PopupAlignment.Center)
    {
        var tooltip = new OverlaySkin.Hint(target, text) { Placement = placement };
        if (target is FrameworkElement anchor)
        {
            void Position() => OverlayPlacement.Hint(tooltip, anchor, placement, align);
            Position();
            tooltip.Opened += (_, _) => Position();
            tooltip.SizeChanged += (_, _) => Position();
            // The hint owns these subscriptions, so replacing a hint does not
            // accumulate callbacks on a long-lived toolbar trigger.
            tooltip.Loaded += (_, _) => Position();
        }
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
            if (element is ContentControl { Name: "Title" } title)
            {
                // The sheet draws its own header; the template's title row
                // would otherwise keep its twelve-pixel gap above it.
                title.Visibility = string.IsNullOrEmpty(title.Content?.ToString()) ? Visibility.Collapsed : Visibility.Visible;
                ActiveTheme.Typography(title, "titleSmall");
            }
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
        var border = theme.Number("stroke.hairline");
        var maxWidth = theme.Number("dialog.width");
        if (Content is FrameworkElement sheet && !double.IsNaN(sheet.Width))
            maxWidth = Math.Max(maxWidth, sheet.Width + border * 2);
        Native.Resources["ContentDialogMaxWidth"] = maxWidth;
        Native.Resources["ContentDialogBorderBrush"] = Danger
            ? theme.Alpha("color.danger.600", theme.Number("dialog.danger-border-alpha"))
            : theme.Brush("color.border-strong");
        Native.Resources["ContentDialogSmokeFill"] = theme.Alpha("dialog.scrim.color", theme.Number("dialog.scrim.alpha"));
        Native.Resources["ContentDialogPadding"] = Padding ?? new Thickness(theme.Number("spacing.6"));
        Native.Resources["ContentDialogTitleMargin"] = new Thickness(0);
        Native.Resources["ContentDialogSeparatorThickness"] = new Thickness(0);
        Native.Resources["ContentDialogContentMargin"] = new Thickness(0);
        Native.Resources["ContentDialogContentScrollViewerMargin"] = new Thickness(0);
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
    Glyph? Leading = null, bool? Dismissible = null, string DismissLabel = "Dismiss",
    string? Id = null, bool? HighPriority = null, Action? OnClose = null, UIElement? Icon = null);
public sealed class Toast : Surface
{
    private readonly Grid body = new();
    private readonly TextBlock message = new() { TextWrapping = TextWrapping.NoWrap, TextTrimming = TextTrimming.CharacterEllipsis, VerticalAlignment = VerticalAlignment.Center };
    private readonly Button action = new() { Variant = ControlVariant.Plain, Size = ControlSize.Small };
    private readonly ToastDismiss close = new();
    private readonly Icon leading = new();
    private ToastOptions options = new("Saved");
    private bool limited;
    private Microsoft.UI.Composition.CompositionScopedBatch? motion;
    private Action? motionCompleted;
    private readonly Microsoft.UI.Xaml.Media.CompositeTransform transform = new();
    public ToastOptions Options { get => options; set { options = value; if (IsLoaded) RefreshAppearance(); } }
    public event EventHandler? Dismissed;

    public Toast()
    {
        Content = body;
        IsTabStop = true;
        HorizontalAlignment = HorizontalAlignment.Center;
        VerticalContentAlignment = VerticalAlignment.Center;
        RenderTransform = transform;
        RenderTransformOrigin = new Windows.Foundation.Point(.5, .5);
        Template = (Microsoft.UI.Xaml.Controls.ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(ToastTemplate);
        action.Click += (_, _) => { if (limited) return; Options.Action?.Invoke(); Dismissed?.Invoke(this, EventArgs.Empty); };
        close.Click += (_, _) => { if (!limited) Dismissed?.Invoke(this, EventArgs.Empty); };
        RegisterPropertyChangedCallback(CornerRadiusProperty, (_, _) => DrawShadow());
        SizeChanged += (_, _) => DrawShadow();
        Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.SetIsTranslationEnabled(this, true);
        FeedbackMotion.Follow(this, () => { if (!FeedbackMotion.Enabled) StopMotion(finish: true); }, () => StopMotion());
        ManipulationMode = ManipulationModes.TranslateX | ManipulationModes.TranslateY;
        ManipulationStarting += (_, args) =>
        {
            for (var node = args.OriginalSource as DependencyObject; node is not null && node != this; node = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(node))
                if (node is ButtonBase) { args.Mode = ManipulationModes.System; return; }
        };
        ManipulationDelta += (_, args) =>
        {
            // The native gesture recognizer owns touch capture. Base UI's
            // default dismiss directions are down and right.
            transform.TranslateX = Math.Max(0, args.Cumulative.Translation.X);
            transform.TranslateY = Math.Max(0, args.Cumulative.Translation.Y);
        };
        ManipulationCompleted += (_, args) =>
        {
            if (args.Cumulative.Translation.X > 40 || args.Cumulative.Translation.Y > 40)
                Dismissed?.Invoke(this, EventArgs.Empty);
            else { transform.TranslateX = 0; transform.TranslateY = 0; }
        };
    }

    protected override void OnApplyTemplate() { base.OnApplyTemplate(); DrawShadow(); }
    protected override void OnKeyDown(KeyRoutedEventArgs args)
    {
        base.OnKeyDown(args);
        if (!limited && !args.Handled && args.Key == Windows.System.VirtualKey.Escape)
        {
            Dismissed?.Invoke(this, EventArgs.Empty);
            args.Handled = true;
        }
    }

    private void DrawShadow()
    {
        if (GetTemplateChild("Lift") is FrameworkElement lift) Elevation.Draw(this, lift, Theme, "md");
    }

    internal void SetLimited(bool value)
    {
        limited = value;
        // Base UI's limit makes the oldest rows inert; its toast stylesheet
        // keeps their drawing. Preserve the same queue and painted ordering.
        IsHitTestVisible = IsTabStop = action.IsTabStop = close.IsTabStop = !limited;
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetAccessibilityView(this, limited
            ? Microsoft.UI.Xaml.Automation.Peers.AccessibilityView.Raw : Microsoft.UI.Xaml.Automation.Peers.AccessibilityView.Content);
    }

    internal void Transition(bool ending, Action? complete = null)
    {
        StopMotion();
        if (!FeedbackMotion.Enabled || !IsLoaded)
        {
            complete?.Invoke();
            return;
        }
        var visual = Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.GetElementVisual(this);
        var compositor = visual.Compositor;
        var duration = TimeSpan.FromMilliseconds(Theme.Milliseconds("toast.transition.duration"));
        // Composition accepts the shared CSS curve's overshoot (y2=1.05),
        // unlike XAML KeySpline's documented normalized control points.
        var curve = compositor.CreateCubicBezierEasingFunction(
            new System.Numerics.Vector2((float)Theme.Number("toast.transition.x1"), (float)Theme.Number("toast.transition.y1")),
            new System.Numerics.Vector2((float)Theme.Number("toast.transition.x2"), (float)Theme.Number("toast.transition.y2")));
        var batch = compositor.CreateScopedBatch(Microsoft.UI.Composition.CompositionBatchTypes.Animation);
        void Animate(string property, double from, double to)
        {
            var animation = compositor.CreateScalarKeyFrameAnimation();
            animation.Duration = duration;
            animation.InsertKeyFrame(0, (float)from);
            animation.InsertKeyFrame(1, (float)to, curve);
            visual.StartAnimation(property, animation);
        }
        visual.CenterPoint = new System.Numerics.Vector3((float)ActualWidth / 2, (float)ActualHeight / 2, 0);
        Animate("Opacity", ending ? 1 : 0, ending ? 0 : 1);
        Animate("Translation.Y", ending ? 0 : Theme.Number("toast.transition.offset"), ending ? Theme.Number("toast.transition.offset") : 0);
        Animate("Scale.X", ending ? 1 : Theme.Number("toast.transition.scale"), ending ? Theme.Number("toast.transition.scale") : 1);
        Animate("Scale.Y", ending ? 1 : Theme.Number("toast.transition.scale"), ending ? Theme.Number("toast.transition.scale") : 1);
        motion = batch; motionCompleted = complete;
        batch.Completed += (_, _) => { if (ReferenceEquals(motion, batch)) StopMotion(finish: true); };
        batch.End();
    }

    private void StopMotion(bool finish = false)
    {
        var complete = finish ? motionCompleted : null;
        motionCompleted = null;
        motion?.Dispose(); motion = null;
        var visual = Microsoft.UI.Xaml.Hosting.ElementCompositionPreview.GetElementVisual(this);
        foreach (var property in new[] { "Opacity", "Translation.Y", "Scale.X", "Scale.Y" }) visual.StopAnimation(property);
        visual.Opacity = 1; visual.Scale = System.Numerics.Vector3.One;
        visual.Properties.InsertVector3("Translation", System.Numerics.Vector3.Zero);
        transform.TranslateX = transform.TranslateY = 0;
        complete?.Invoke();
    }

    protected override void RefreshAppearance()
    {
        body.ColumnSpacing = Theme.Number("spacing.2.5");
        var type = Theme.Text("", "labelMedium");
        message.Text = Options.Title;
        message.FontFamily = type.FontFamily; message.FontSize = type.FontSize;
        TextStyle.SetWeight(message, Theme.Weight("labelMedium"));
        message.IsTextScaleFactorEnabled = type.IsTextScaleFactorEnabled;
        message.LineHeight = message.FontSize * Theme.Number("toast.message-line-height");
        message.LineStackingStrategy = LineStackingStrategy.BlockLineHeight;
        message.Padding = new Thickness(0, Theme.Number("spacing.0.5"), 0, Theme.Number("spacing.0.5"));
        message.Foreground = Theme.Brush("color.content");
        if (Options.Leading is Glyph glyph) leading.Glyph = glyph;
        leading.GlyphSize = Theme.Number("icon.medium");
        leading.Foreground = Theme.Brush($"color.{Theme.Key(Options.Tint)}.600");
        action.Content = Options.ActionLabel; action.Tint = Options.Tint;
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(close, Options.DismissLabel);
        var parts = new List<UIElement>();
        if (Options.Icon is UIElement icon)
        {
            if (icon is Control control) control.Foreground = leading.Foreground;
            parts.Add(icon);
        }
        else if (Options.Leading is not null) parts.Add(leading);
        parts.Add(message);
        if (Options.ActionLabel is not null) parts.Add(action);
        if (Options.Dismissible ?? true) parts.Add(close);
        if (!body.Children.SequenceEqual(parts))
        {
            // Optional slots can appear in an ID update. Insert/remove only
            // those slots so an existing dismiss/action retains native focus.
            foreach (var old in body.Children.Where(child => !parts.Contains(child)).ToArray()) body.Children.Remove(old);
            body.ColumnDefinitions.Clear();
            for (var i = 0; i < parts.Count; i++)
            {
                var part = parts[i];
                body.ColumnDefinitions.Add(new ColumnDefinition { Width = ReferenceEquals(part, message) ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
                part.SetValue(Grid.ColumnProperty, i);
                if (i >= body.Children.Count || !ReferenceEquals(body.Children[i], part))
                {
                    if (body.Children.Contains(part)) body.Children.Remove(part);
                    body.Children.Insert(i, part);
                }
            }
        }
        Padding = new Thickness(Theme.Number("spacing.3"), Theme.Number("spacing.1.5"), Theme.Number("spacing.1.5"), Theme.Number("spacing.1.5"));
        MinHeight = Theme.Number("spacing.9");
        Background = Theme.Brush("color.surface-overlay"); BorderBrush = Theme.Brush("color.border-strong");
        BorderThickness = new Thickness(Theme.Number("stroke.hairline")); Skin.SetCorner(this, Theme.Number("radius.big"));
        MaxWidth = Theme.Number("toast.max-width");
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, Options.Title);
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetHelpText(this, Options.Description ?? "");
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetLiveSetting(this, (Options.HighPriority ?? Options.Tint == WidgetTint.Danger)
            ? Microsoft.UI.Xaml.Automation.Peers.AutomationLiveSetting.Assertive : Microsoft.UI.Xaml.Automation.Peers.AutomationLiveSetting.Polite);
        SetLimited(limited);
        if (leading.IsLoaded) Appearance.Refresh(leading);
        Skin.Focus(this, Theme);
        DrawShadow();
    }

    private sealed class ToastDismiss : Button
    {
        private readonly Viewbox box = new();
        internal ToastDismiss()
        {
            Variant = ControlVariant.Plain; Tint = WidgetTint.Neutral; Size = ControlSize.Tiny;
            var glyph = Icon.Element(FluentIcons.Common.Icon.Dismiss);
            glyph.IconSize = FluentIcons.Common.IconSize.Size12; glyph.FontSize = 12;
            glyph.SetBinding(IconElement.ForegroundProperty, new Microsoft.UI.Xaml.Data.Binding { Source = this, Path = new PropertyPath(nameof(Foreground)) });
            box.Child = glyph; Content = box;
        }
        protected override void LayContent() => Padding = new Thickness(0);
        protected override void Tailor()
        {
            Width = Height = Theme.Number("control.tiny.size");
            BorderThickness = new Thickness(0);
            Skin.SetCorner(this, Theme.Number("radius.small"));
            if (box is not null) box.Width = box.Height = Theme.Number("icon.small");
            Repaint("ButtonForeground", Theme.Color("color.content-subtle"));
            Repaint("ButtonForegroundPointerOver", Theme.Color("color.content"));
            Repaint("ButtonForegroundPressed", Theme.Color("color.content"));
            Repaint("ButtonBackgroundPointerOver", Theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered").Color);
        }
    }

    private const string ToastTemplate = """
        <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="ContentControl">
          <Grid>
            <Border x:Name="Lift" IsHitTestVisible="False" />
            <ContentPresenter BackgroundSizing="OuterBorderEdge" Background="{TemplateBinding Background}"
                BorderBrush="{TemplateBinding BorderBrush}" BorderThickness="{TemplateBinding BorderThickness}"
                CornerRadius="{TemplateBinding CornerRadius}" Padding="{TemplateBinding Padding}"
                HorizontalContentAlignment="{TemplateBinding HorizontalContentAlignment}"
                VerticalContentAlignment="{TemplateBinding VerticalContentAlignment}"
                Content="{TemplateBinding Content}" ContentTemplate="{TemplateBinding ContentTemplate}" />
          </Grid>
        </ControlTemplate>
        """;
}
public sealed class ToastViewport : Surface
{
    // One column, native children in source order. Grid rows reverse only the
    // visual order at the top edge, preserving focused controls and Tab order.
    private readonly Grid stack = new() { ColumnDefinitions = { new ColumnDefinition() } };
    private sealed class Entry(Toast toast)
    {
        internal readonly Toast Toast = toast;
        internal readonly DispatcherTimer Timer = new();
        internal readonly System.Diagnostics.Stopwatch Clock = new();
        internal TimeSpan Remaining;
        internal bool Closing;
    }
    private readonly List<Entry> entries = [];
    private bool hovered, focused, touching, inactive;
    private int limit = 3;
    private bool atTop;
    private UIElement? root;
    private readonly KeyEventHandler rootKeyDown;
    private Control? previousFocus;
    private Microsoft.UI.Input.InputActivationListener? activation;

    /// <summary>Top reverses the newest-first store, as the React viewport does.</summary>
    public bool AtTop { get => atTop; set { atTop = value; VerticalAlignment = value ? VerticalAlignment.Top : VerticalAlignment.Bottom; ArrangeToasts(); } }
    public int Limit { get => limit; set { limit = Math.Max(0, value); ApplyLimit(); } }
    public IReadOnlyList<Toast> Toasts => entries.Where(entry => !entry.Closing).Select(entry => entry.Toast).ToArray();

    internal string TimingState
    {
        get
        {
            var current = XamlRoot is null ? null : FocusManager.GetFocusedElement(XamlRoot);
            var focus = current is Control control ? $"{control.GetType().Name}/{control.FocusState}/{Microsoft.UI.Xaml.Automation.AutomationProperties.GetName(control)}"
                : current?.GetType().Name ?? "none";
            return $"loaded={IsLoaded}, hostVisible={XamlRoot?.IsHostVisible}, hovered={hovered}, focused={focused}, actualFocus={focus}, focusInside={Inside(current as DependencyObject, this)}, touching={touching}, inactive={inactive}, activation={activation?.State}; "
                + string.Join("; ", entries.Select(entry => $"'{entry.Toast.Options.Title}': closing={entry.Closing}, remaining={entry.Remaining.TotalMilliseconds:F0}ms, elapsed={entry.Clock.Elapsed.TotalMilliseconds:F0}ms, clockRunning={entry.Clock.IsRunning}, timerEnabled={entry.Timer.IsEnabled}, interval={entry.Timer.Interval.TotalMilliseconds:F0}ms"));
        }
    }

    public ToastViewport()
    {
        rootKeyDown = RootKeyDown;
        Content = stack;
        HorizontalAlignment = HorizontalAlignment.Stretch;
        VerticalAlignment = VerticalAlignment.Bottom;
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, "Notifications");
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetLiveSetting(this, Microsoft.UI.Xaml.Automation.Peers.AutomationLiveSetting.Polite);
        Loaded += (_, _) => Connect();
        Unloaded += (_, _) => Disconnect();
        PointerEntered += (_, _) => { hovered = true; UpdateTimers(); };
        PointerMoved += (_, _) => { if (!hovered) { hovered = true; UpdateTimers(); } };
        PointerExited += (_, _) => { hovered = false; UpdateTimers(); };
        AddHandler(PointerPressedEvent, new PointerEventHandler((_, args) =>
        {
            if (args.Pointer.PointerDeviceType == Microsoft.UI.Input.PointerDeviceType.Touch) { touching = true; UpdateTimers(); }
        }), true);
        void TouchEnded(object sender, PointerRoutedEventArgs args)
        {
            if (args.Pointer.PointerDeviceType == Microsoft.UI.Input.PointerDeviceType.Touch) { touching = false; UpdateTimers(); }
        }
        AddHandler(PointerReleasedEvent, new PointerEventHandler(TouchEnded), true);
        AddHandler(PointerCanceledEvent, new PointerEventHandler(TouchEnded), true);
        GotFocus += (_, _) =>
        {
            // Mouse focus is already protected by hover; only keyboard and
            // explicit programmatic focus keep the queue paused after exit.
            focused = FocusManager.GetFocusedElement(XamlRoot) is Control { FocusState: not FocusState.Pointer };
            UpdateTimers();
        };
        LostFocus += (_, _) => DispatcherQueue.TryEnqueue(() =>
        {
            if (!IsLoaded) return;
            focused = Inside(FocusManager.GetFocusedElement(XamlRoot) as DependencyObject, this);
            if (!focused) IsTabStop = false;
            UpdateTimers();
        });
        KeyDown += (_, args) =>
        {
            if (args.Key == Windows.System.VirtualKey.Tab && ReferenceEquals(FocusManager.GetFocusedElement(XamlRoot), this)
                && Microsoft.UI.Input.InputKeyboardSource.GetKeyStateForCurrentThread(Windows.System.VirtualKey.Shift).HasFlag(Windows.UI.Core.CoreVirtualKeyStates.Down))
            {
                if (previousFocus?.Focus(FocusState.Keyboard) == true) args.Handled = true;
            }
        };
    }

    private static bool Inside(DependencyObject? child, DependencyObject parent)
    {
        for (var node = child; node is not null; node = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(node))
            if (ReferenceEquals(node, parent)) return true;
        return false;
    }

    private void Connect()
    {
        root = XamlRoot.Content;
        root.AddHandler(KeyDownEvent, rootKeyDown, true);
        activation = Microsoft.UI.Input.InputActivationListener.GetForWindowId(XamlRoot.ContentIslandEnvironment.AppWindowId);
        if (activation is not null)
        {
            activation.InputActivationChanged += ActivationChanged;
            inactive = activation.State == Microsoft.UI.Input.InputActivationState.Deactivated;
        }
        foreach (var entry in entries) ConfigureTimer(entry);
        UpdateTimers();
    }

    private void ActivationChanged(Microsoft.UI.Input.InputActivationListener sender, Microsoft.UI.Input.InputActivationListenerActivationChangedEventArgs args)
    {
        inactive = sender.State == Microsoft.UI.Input.InputActivationState.Deactivated;
        UpdateTimers();
    }

    private void RootKeyDown(object sender, KeyRoutedEventArgs args)
    {
        if (args.Handled || args.Key != Windows.System.VirtualKey.F6 || !entries.Any(entry => entry.Toast.IsTabStop) || Inside(FocusManager.GetFocusedElement(XamlRoot) as DependencyObject, this)) return;
        previousFocus = FocusManager.GetFocusedElement(XamlRoot) as Control;
        IsTabStop = true;
        args.Handled = Focus(FocusState.Keyboard);
    }

    private void Disconnect()
    {
        if (root is not null) root.RemoveHandler(KeyDownEvent, rootKeyDown);
        root = null;
        if (activation is not null) activation.InputActivationChanged -= ActivationChanged;
        activation = null;
        foreach (var entry in entries) { entry.Timer.Stop(); entry.Clock.Reset(); }
        entries.Clear(); stack.Children.Clear(); stack.RowDefinitions.Clear();
        hovered = focused = touching = inactive = false;
        previousFocus = null;
    }

    public Toast Add(ToastOptions options)
    {
        // Reusing an ID while its old row exits replaces that ending row
        // immediately, without another close callback or two copies on screen.
        foreach (var ending in entries.Where(entry => options.Id is not null && entry.Closing && entry.Toast.Options.Id == options.Id).ToArray())
        {
            entries.Remove(ending); stack.Children.Remove(ending.Toast);
        }
        ArrangeToasts();
        if (options.Id is not null && entries.FirstOrDefault(entry => !entry.Closing && entry.Toast.Options.Id == options.Id) is Entry existing)
        {
            existing.Toast.Options = options;
            ConfigureTimer(existing);
            return existing.Toast;
        }
        var toast = new Toast { Options = options };
        var entry = new Entry(toast);
        toast.Dismissed += (_, _) => Remove(toast);
        toast.Loaded += (_, _) => { if (!entry.Closing) toast.Transition(false); };
        entry.Timer.Tick += (_, _) => Remove(toast);
        entries.Insert(0, entry);
        stack.Children.Insert(0, toast);
        ArrangeToasts();
        ApplyLimit();
        if (IsLoaded) ConfigureTimer(entry);
        return toast;
    }

    private void ConfigureTimer(Entry entry)
    {
        entry.Timer.Stop(); entry.Clock.Reset();
        var options = entry.Toast.Options;
        entry.Remaining = options.Duration ?? (options.Tint == WidgetTint.Danger ? TimeSpan.Zero
            : TimeSpan.FromMilliseconds(Theme.Milliseconds(options.ActionLabel is null ? "toast.notice-duration" : "toast.action-duration")));
        if (!entry.Closing && !Paused && IsLoaded && entry.Remaining > TimeSpan.Zero) Start(entry);
    }

    private bool Paused => hovered || focused || touching || inactive;
    private static void Start(Entry entry)
    {
        entry.Timer.Interval = entry.Remaining > TimeSpan.Zero ? entry.Remaining : TimeSpan.FromMilliseconds(1);
        entry.Clock.Restart(); entry.Timer.Start();
    }
    private void UpdateTimers()
    {
        foreach (var entry in entries)
        {
            if (entry.Closing) continue;
            if (Paused && entry.Clock.IsRunning)
            {
                entry.Timer.Stop();
                entry.Remaining = TimeSpan.FromTicks(Math.Max(1, (entry.Remaining - entry.Clock.Elapsed).Ticks));
                entry.Clock.Reset();
            }
            else if (!Paused && IsLoaded && !entry.Clock.IsRunning && entry.Remaining > TimeSpan.Zero) Start(entry);
        }
    }

    public void Close(string? id = null)
    {
        foreach (var entry in entries.Where(entry => id is null || entry.Toast.Options.Id == id).ToArray()) Remove(entry.Toast);
    }

    public void Remove(Toast toast)
    {
        var entry = entries.FirstOrDefault(entry => ReferenceEquals(entry.Toast, toast));
        if (entry is null || entry.Closing) return;
        entry.Closing = true; entry.Timer.Stop(); entry.Clock.Reset();
        toast.SetLimited(true);
        if (XamlRoot is not null && Inside(FocusManager.GetFocusedElement(XamlRoot) as DependencyObject, toast))
        {
            var next = entries.FirstOrDefault(other => !other.Closing)?.Toast;
            if (next is null) previousFocus?.Focus(FocusState.Keyboard);
            else { next.SetLimited(false); next.Focus(FocusState.Keyboard); }
        }
        ApplyLimit();
        toast.Options.OnClose?.Invoke();
        toast.Transition(true, () =>
        {
            entries.Remove(entry); stack.Children.Remove(toast); ArrangeToasts();
            ApplyLimit();
        });
    }

    private void ApplyLimit()
    {
        var index = 0;
        foreach (var entry in entries) entry.Toast.SetLimited(entry.Closing || index++ >= limit);
        if (XamlRoot is null) return;
        var focus = FocusManager.GetFocusedElement(XamlRoot) as DependencyObject;
        if (entries.Any(entry => !entry.Toast.IsTabStop && Inside(focus, entry.Toast)))
        {
            var next = entries.FirstOrDefault(entry => entry.Toast.IsTabStop)?.Toast;
            if (next is not null) next.Focus(FocusState.Keyboard);
            else previousFocus?.Focus(FocusState.Keyboard);
        }
    }

    private void ArrangeToasts()
    {
        while (stack.RowDefinitions.Count < entries.Count) stack.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
        while (stack.RowDefinitions.Count > entries.Count) stack.RowDefinitions.RemoveAt(stack.RowDefinitions.Count - 1);
        for (var i = 0; i < entries.Count; i++) Grid.SetRow(entries[i].Toast, atTop ? entries.Count - i - 1 : i);
    }

    protected override void RefreshAppearance()
    {
        stack.RowSpacing = Theme.Number("spacing.2");
        Padding = new Thickness(Theme.Number("spacing.4"));
        // A null background keeps the real, full-width viewport transparent
        // to input between notifications. The content template still owns padding.
        Background = null;
        BorderThickness = new Thickness(0);
    }
}
