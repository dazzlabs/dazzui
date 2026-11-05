using System.Runtime.CompilerServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Markup;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

/// <summary>The shared popup sheet and rows, with native menu behavior.</summary>
internal static class PopupSkin
{
    private static readonly ConditionalWeakTable<FrameworkElement, object> Prepared = new();
    private static readonly DependencyProperty PresenterThemeProperty = DependencyProperty.RegisterAttached(
        "PresenterTheme", typeof(Theme), typeof(PopupSkin), new PropertyMetadata(null, PresenterChanged));

    private const string PresenterTemplate = """
        <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="MenuFlyoutPresenter">
          <Grid>
            <Border x:Name="DazzPopupLift" IsHitTestVisible="False" />
            <Border Background="{TemplateBinding Background}" BackgroundSizing="OuterBorderEdge"
                BorderBrush="{TemplateBinding BorderBrush}" BorderThickness="{TemplateBinding BorderThickness}"
                CornerRadius="{TemplateBinding CornerRadius}">
              <ScrollViewer x:Name="MenuFlyoutPresenterScrollViewer" Margin="{TemplateBinding Padding}"
                  HorizontalScrollMode="Disabled" HorizontalScrollBarVisibility="Disabled"
                  VerticalScrollMode="Auto" VerticalScrollBarVisibility="Auto"
                  ZoomMode="Disabled" IsTabStop="False" AutomationProperties.AccessibilityView="Raw">
                <ItemsPresenter />
              </ScrollViewer>
            </Border>
          </Grid>
        </ControlTemplate>
        """;

    internal static Style Presenter(Theme theme)
    {
        var style = new Style(typeof(MenuFlyoutPresenter));
        style.Setters.Add(new Setter(Control.TemplateProperty, (ControlTemplate)XamlReader.Load(PresenterTemplate)));
        style.Setters.Add(new Setter(Control.BackgroundProperty, theme.Brush("color.surface-overlay")));
        style.Setters.Add(new Setter(Control.ForegroundProperty, theme.Brush("color.content")));
        style.Setters.Add(new Setter(Control.BorderBrushProperty, theme.Brush("color.border-strong")));
        style.Setters.Add(new Setter(Control.BorderThicknessProperty, new Thickness(theme.Number("stroke.hairline"))));
        style.Setters.Add(new Setter(Control.PaddingProperty, new Thickness(theme.Number("spacing.1"))));
        style.Setters.Add(new Setter(Control.CornerRadiusProperty, new CornerRadius(theme.Number("radius.large"))));
        style.Setters.Add(new Setter(FrameworkElement.MinWidthProperty, theme.Number("menu.min-width")));
        style.Setters.Add(new Setter(FrameworkElement.MinHeightProperty, 0d));
        style.Setters.Add(new Setter(FrameworkElement.UseLayoutRoundingProperty, false));
        style.Setters.Add(new Setter(MenuFlyoutPresenter.IsDefaultShadowEnabledProperty, false));
        style.Setters.Add(new Setter(PresenterThemeProperty, theme));
        return style;
    }

    private static void PresenterChanged(DependencyObject sender, DependencyPropertyChangedEventArgs args)
    {
        if (sender is not MenuFlyoutPresenter presenter) return;
        void Paint()
        {
            if (presenter.GetValue(PresenterThemeProperty) is not Theme theme) return;
            presenter.ApplyTemplate();
            presenter.Shadow = null;
            Skin.SetCorner(presenter, theme.Number("radius.large"));
            if (Part<FrameworkElement>(presenter, "DazzPopupLift") is FrameworkElement lift)
                Elevation.Draw(presenter, lift, theme, "lg");
        }
        if (!Prepared.TryGetValue(presenter, out _))
        {
            Prepared.Add(presenter, new object());
            presenter.Loaded += (_, _) => Paint();
        }
        if (presenter.IsLoaded) Paint();
    }

    internal static void Item(MenuFlyoutItemBase item, Theme theme)
    {
        item.UseLayoutRounding = false;
        Appearance.SetTheme(item, theme);
        item.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
        item.MinWidth = item.MinHeight = 0;
        item.Margin = new Thickness(0);
        item.BorderThickness = new Thickness(0);
        item.UseSystemFocusVisuals = false;
        if (item is MenuFlyoutSeparator)
        {
            item.Template = (ControlTemplate)XamlReader.Load("""
                <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" TargetType="MenuFlyoutSeparator">
                  <Border Height="{TemplateBinding Height}" Background="{TemplateBinding Background}" />
                </ControlTemplate>
                """);
            item.Height = theme.Number("stroke.hairline");
            item.Background = theme.Brush("color.border");
            item.Margin = new Thickness(theme.Number("spacing.2"), theme.Number("spacing.1"), theme.Number("spacing.2"), theme.Number("spacing.1"));
            return;
        }

        // Base UI's menu keeps disabled rows in arrow navigation. WinUI can
        // retain native disabled activation while permitting that same focus.
        item.AllowFocusWhenDisabled = true;
        void DisabledConfirmation(object sender, Microsoft.UI.Xaml.Input.KeyRoutedEventArgs args)
        {
            // The native presenter otherwise dismisses for Enter even when
            // its focused row cannot invoke. Keep both halves of that key
            // inside the disabled row, as Base UI's disabled item does.
            if (!item.IsEnabled && args.Key is Windows.System.VirtualKey.Enter or Windows.System.VirtualKey.Space)
                args.Handled = true;
        }
        item.PreviewKeyDown += DisabledConfirmation;
        item.PreviewKeyUp += DisabledConfirmation;
        item.PreviewKeyDown += (_, args) =>
        {
            if (args.Key is not (Windows.System.VirtualKey.Home or Windows.System.VirtualKey.End)) return;
            if (Endpoint(item, args.Key == Windows.System.VirtualKey.End)) args.Handled = true;
        };
        item.PreviewKeyUp += (_, args) =>
        {
            if (args.Key is Windows.System.VirtualKey.Home or Windows.System.VirtualKey.End) args.Handled = true;
        };
        theme.Typography(item, "labelQuiet");
        item.Padding = new Thickness(theme.Number("spacing.2"), theme.Number("menu.item-padding"), theme.Number("spacing.2"), theme.Number("menu.item-padding"));
        Skin.SetCorner(item, Math.Max(0, theme.Number("radius.large") - theme.Number("spacing.1")));
        item.Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent);
        item.Foreground = theme.Brush("color.content");

        var checkedItem = item is ToggleMenuFlyoutItem;
        var submenu = item is MenuFlyoutSubItem;
        var icon = item switch { MenuFlyoutItem leaf => leaf.Icon, MenuFlyoutSubItem branch => branch.Icon, _ => null };
        var shortcut = item is MenuFlyoutItem action && !string.IsNullOrEmpty(action.KeyboardAcceleratorTextOverride);
        var prefix = checkedItem ? "ToggleMenuFlyoutItem" : submenu ? "MenuFlyoutSubItem" : "MenuFlyoutItem";
        item.Template = (ControlTemplate)XamlReader.Load(RowTemplate(prefix, checkedItem, icon is not null, shortcut, submenu, theme));
        item.Resources[typeof(MenuFlyoutPresenter)] = Presenter(theme);
        void FocusAppearance() => VisualStateManager.GoToState(item, item.FocusState == FocusState.Unfocused ? "DazzUnfocused" : "DazzFocused", false);
        item.GotFocus += (_, _) => FocusAppearance();
        item.LostFocus += (_, _) => FocusAppearance();

        void Paint()
        {
            theme = Appearance.GetTheme(item);
            if (Part<Grid>(item, "DazzMenuRow") is Grid row) row.ColumnSpacing = theme.Number("spacing.2");
            if (Part<TextBlock>(item, "TextBlock") is TextBlock label) Label(label, theme, "labelQuiet");
            if (Part<TextBlock>(item, "KeyboardAcceleratorTextBlock") is TextBlock hint) Label(hint, theme, "labelSmall");
            if (Part<ContentPresenter>(item, "CheckGlyph") is ContentPresenter check)
            {
                check.Width = check.Height = theme.Number("icon.small");
                var mark = new Icon { Glyph = FluentIcons.Common.Icon.Checkmark, GlyphVariant = FluentIcons.Common.IconVariant.Filled, GlyphSize = check.Width };
                mark.SetBinding(Control.ForegroundProperty, new Microsoft.UI.Xaml.Data.Binding { Source = check, Path = new PropertyPath("Foreground") });
                check.Content = mark;
            }
            if (Part<ContentPresenter>(item, "SubItemChevron") is ContentPresenter chevron)
            {
                chevron.Width = chevron.Height = theme.Number("icon.small");
                chevron.Content = new Icon { Glyph = FluentIcons.Common.Icon.ChevronRight, GlyphSize = chevron.Width, Foreground = theme.Brush("color.content-subtle") };
            }
            if (Part<ContentPresenter>(item, "IconContent") is ContentPresenter host)
            {
                host.Width = host.Height = theme.Number("icon.medium");
                if (icon is FluentIcons.WinUI.FluentIcon fluent)
                {
                    fluent.FontSize = host.Width;
                    fluent.IconSize = host.Width switch
                    {
                        12 => FluentIcons.Common.IconSize.Size12,
                        16 => FluentIcons.Common.IconSize.Size16,
                        20 => FluentIcons.Common.IconSize.Size20,
                        24 => FluentIcons.Common.IconSize.Size24,
                        _ => FluentIcons.Common.IconSize.Resizable,
                    };
                }
            }
            FocusAppearance();
        }
        item.Loaded += (_, _) => Paint();
        if (item.IsLoaded) Paint();
    }

    internal static void Repaint(MenuFlyoutItemBase item, Theme theme)
    {
        Appearance.SetTheme(item, theme);
        item.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
        if (item is MenuFlyoutSeparator)
        {
            item.Height = theme.Number("stroke.hairline");
            item.Background = theme.Brush("color.border");
            item.Margin = new Thickness(theme.Number("spacing.2"), theme.Number("spacing.1"), theme.Number("spacing.2"), theme.Number("spacing.1"));
            return;
        }
        theme.Typography(item, "labelQuiet");
        item.Padding = new Thickness(theme.Number("spacing.2"), theme.Number("menu.item-padding"), theme.Number("spacing.2"), theme.Number("menu.item-padding"));
        Skin.SetCorner(item, Math.Max(0, theme.Number("radius.large") - theme.Number("spacing.1")));
        item.Foreground = theme.Brush("color.content");
        item.Resources[typeof(MenuFlyoutPresenter)] = Presenter(theme);
        if (Part<Grid>(item, "LayoutRoot") is Grid root)
        {
            void Brush(string name, Windows.UI.Color color)
            {
                if (root.Resources.TryGetValue(name, out var resource) && resource is SolidColorBrush brush) brush.Color = color;
            }
            Brush("DazzMenuHighlight", theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered").Color);
            Brush("DazzMenuDisabledInk", theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled").Color);
            Brush("DazzMenuFaintInk", theme.Color("color.content-faint"));
            Brush("DazzMenuIconInk", theme.Color("color.content-muted"));
        }
        if (Part<Grid>(item, "DazzMenuRow") is Grid row) row.ColumnSpacing = theme.Number("spacing.2");
        if (Part<TextBlock>(item, "TextBlock") is TextBlock label) Label(label, theme, "labelQuiet");
        if (Part<TextBlock>(item, "KeyboardAcceleratorTextBlock") is TextBlock hint) Label(hint, theme, "labelSmall");
        foreach (var name in new[] { "CheckGlyph", "SubItemChevron", "IconContent" })
        {
            if (Part<ContentPresenter>(item, name) is not ContentPresenter host) continue;
            host.Width = host.Height = theme.Number(name == "IconContent" ? "icon.medium" : "icon.small");
            if (host.Content is Icon glyph)
            {
                glyph.GlyphSize = host.Width;
                if (name == "SubItemChevron") glyph.Foreground = theme.Brush("color.content-subtle");
                Appearance.Refresh(glyph);
            }
            else if (host.Content is FluentIcons.WinUI.FluentIcon fluent) fluent.FontSize = host.Width;
        }
    }

    internal static void Repaint(MenuFlyoutPresenter presenter, Theme theme)
    {
        foreach (Setter setter in Presenter(theme).Setters)
            if (setter.Property != Control.TemplateProperty) presenter.SetValue(setter.Property, setter.Value);
    }

    private static void Label(TextBlock label, Theme theme, string token)
    {
        theme.Text("", token).CopyTo(label);
        label.TextWrapping = TextWrapping.NoWrap;
        label.LineHeight = label.Height = label.FontSize;
        label.UseLayoutRounding = false;
        void Align()
        {
            var offset = PlatformFonts.LineBaseline(label.FontFamily, label.FontSize, label.FontWeight, label.Height) - label.BaselineOffset;
            if (label.RenderTransform is not TranslateTransform translation || Math.Abs(translation.Y - offset) > .001)
                label.RenderTransform = new TranslateTransform { Y = offset };
        }
        if (!Prepared.TryGetValue(label, out _))
        {
            Prepared.Add(label, new object());
            void Updated(object? sender, object args) => Align();
            label.Loaded += (_, _) => { label.LayoutUpdated -= Updated; label.LayoutUpdated += Updated; Align(); };
            label.Unloaded += (_, _) => label.LayoutUpdated -= Updated;
            if (label.IsLoaded) label.LayoutUpdated += Updated;
        }
        Align();
    }

    private static bool Endpoint(MenuFlyoutItemBase item, bool end)
    {
        for (DependencyObject? parent = VisualTreeHelper.GetParent(item); parent is not null; parent = VisualTreeHelper.GetParent(parent))
        {
            if (parent is not MenuFlyoutPresenter presenter) continue;
            var rows = presenter.Items.OfType<MenuFlyoutItemBase>().Where(row => row is not MenuFlyoutSeparator
                && row.Visibility == Visibility.Visible && (row.IsEnabled || row.AllowFocusWhenDisabled));
            var target = end ? rows.LastOrDefault() : rows.FirstOrDefault();
            if (target is null || !target.Focus(FocusState.Keyboard)) return false;
            target.StartBringIntoView();
            return true;
        }
        return false;
    }

    private static string RowTemplate(string target, bool check, bool icon, bool shortcut, bool submenu, Theme theme)
    {
        static string Brush(string name, SolidColorBrush brush)
        {
            var color = brush.Color;
            return $"<SolidColorBrush x:Key=\"{name}\" Color=\"#{color.A:X2}{color.R:X2}{color.G:X2}{color.B:X2}\" />";
        }
        // Flyout template loading is detached from the item's resource scope.
        // Keep its resolved palette in the template that owns these states.
        var brushes = Brush("DazzMenuHighlight", theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered"))
            + Brush("DazzMenuDisabledInk", theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled"))
            + Brush("DazzMenuFaintInk", theme.Brush("color.content-faint"))
            + Brush("DazzMenuIconInk", theme.Brush("color.content-muted"));
        var count = (check ? 1 : 0) + (icon ? 1 : 0);
        var columns = string.Concat(Enumerable.Repeat("<ColumnDefinition Width=\"Auto\" />", count))
            + "<ColumnDefinition Width=\"*\" />" + (shortcut || submenu ? "<ColumnDefinition Width=\"Auto\" />" : "");
        var checkStates = check ? """
            <VisualStateGroup x:Name="CheckStates">
              <VisualState x:Name="Unchecked" />
              <VisualState x:Name="UncheckedWithIcon" />
              <VisualState x:Name="Checked"><VisualState.Setters><Setter Target="CheckGlyph.Opacity" Value="1" /></VisualState.Setters></VisualState>
              <VisualState x:Name="CheckedWithIcon"><VisualState.Setters><Setter Target="CheckGlyph.Opacity" Value="1" /></VisualState.Setters></VisualState>
            </VisualStateGroup>
            """ : "";
        var checkPart = check ? "<ContentPresenter x:Name=\"CheckGlyph\" Grid.Column=\"0\" Opacity=\"0\" Foreground=\"{TemplateBinding Foreground}\" VerticalAlignment=\"Center\" />" : "";
        var disabledCheck = check ? "<Setter Target=\"CheckGlyph.Foreground\" Value=\"{StaticResource DazzMenuDisabledInk}\" />" : "";
        var iconPart = icon ? $"<ContentPresenter x:Name=\"IconContent\" Grid.Column=\"{(check ? 1 : 0)}\" Content=\"{{TemplateBinding Icon}}\" Foreground=\"{{StaticResource DazzMenuIconInk}}\" VerticalAlignment=\"Center\" />" : "";
        var shortcutPart = shortcut ? $"<TextBlock x:Name=\"KeyboardAcceleratorTextBlock\" Grid.Column=\"{count + 1}\" Text=\"{{TemplateBinding KeyboardAcceleratorTextOverride}}\" Foreground=\"{{StaticResource DazzMenuFaintInk}}\" VerticalAlignment=\"Center\" AutomationProperties.AccessibilityView=\"Raw\" />" : "";
        var chevronPart = submenu ? $"<ContentPresenter x:Name=\"SubItemChevron\" Grid.Column=\"{count + 1}\" VerticalAlignment=\"Center\" />" : "";
        return $$"""
            <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
                xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="{{target}}">
              <Grid x:Name="LayoutRoot" Background="{TemplateBinding Background}" Padding="{TemplateBinding Padding}"
                  CornerRadius="{TemplateBinding CornerRadius}">
                <Grid.Resources>{{brushes}}</Grid.Resources>
                <VisualStateManager.VisualStateGroups>
                  <VisualStateGroup x:Name="CommonStates">
                    <VisualState x:Name="Normal" />
                    <VisualState x:Name="PointerOver"><VisualState.Setters><Setter Target="LayoutRoot.Background" Value="{StaticResource DazzMenuHighlight}" /></VisualState.Setters></VisualState>
                    <VisualState x:Name="Pressed"><VisualState.Setters><Setter Target="LayoutRoot.Background" Value="{StaticResource DazzMenuHighlight}" /></VisualState.Setters></VisualState>
                    <VisualState x:Name="SubMenuOpened"><VisualState.Setters><Setter Target="LayoutRoot.Background" Value="{StaticResource DazzMenuHighlight}" /></VisualState.Setters></VisualState>
                    <VisualState x:Name="Disabled"><VisualState.Setters><Setter Target="TextBlock.Foreground" Value="{StaticResource DazzMenuDisabledInk}" />{{disabledCheck}}</VisualState.Setters></VisualState>
                  </VisualStateGroup>
                  <VisualStateGroup x:Name="DazzFocusStates">
                    <VisualState x:Name="DazzUnfocused" />
                    <VisualState x:Name="DazzFocused"><VisualState.Setters><Setter Target="LayoutRoot.Background" Value="{StaticResource DazzMenuHighlight}" /></VisualState.Setters></VisualState>
                  </VisualStateGroup>
                  {{checkStates}}
                </VisualStateManager.VisualStateGroups>
                <Grid x:Name="DazzMenuRow" IsHitTestVisible="False">
                  <Grid.ColumnDefinitions>{{columns}}</Grid.ColumnDefinitions>
                  {{checkPart}}
                  {{iconPart}}
                  <TextBlock x:Name="TextBlock" Grid.Column="{{count}}" Text="{TemplateBinding Text}"
                      Foreground="{TemplateBinding Foreground}" VerticalAlignment="Center" TextTrimming="CharacterEllipsis" />
                  {{shortcutPart}}
                  {{chevronPart}}
                </Grid>
              </Grid>
            </ControlTemplate>
            """;
    }

    internal static T? Part<T>(DependencyObject root, string name) where T : FrameworkElement
    {
        if (root is T part && part.Name == name) return part;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            if (Part<T>(VisualTreeHelper.GetChild(root, i), name) is T child) return child;
        return null;
    }
}
