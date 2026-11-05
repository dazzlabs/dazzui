using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Markup;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

/// <summary>The prose sheet and passive hint, over native popup lifecycles.</summary>
internal static class OverlaySkin
{
    internal static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }

    internal static Style Prose(Theme theme, double maximumWidth, double? width)
    {
        var style = new Style(typeof(FlyoutPresenter));
        void Set(DependencyProperty property, object value) => style.Setters.Add(new Setter(property, value));
        Set(Control.TemplateProperty, (ControlTemplate)XamlReader.Load(ProseTemplate));
        Set(FlyoutPresenter.IsDefaultShadowEnabledProperty, false);
        Set(Control.BackgroundProperty, theme.Brush("color.surface-overlay"));
        Set(Control.ForegroundProperty, theme.Brush("color.content"));
        Set(Control.BorderBrushProperty, theme.Brush("color.border-strong"));
        Set(Control.BorderThicknessProperty, new Thickness(theme.Number("stroke.hairline")));
        Set(Control.CornerRadiusProperty, new CornerRadius(theme.Number("radius.large")));
        Set(Control.PaddingProperty, new Thickness(theme.Number("spacing.4") + theme.Number("spacing.0.5")));
        // FlyoutBase supplies a local Width when it places the presenter.
        // Bounds keep an explicit CSS border-box width through that native pass.
        var resolvedWidth = width is double specified ? Math.Min(specified, maximumWidth) : (double?)null;
        Set(FrameworkElement.MinWidthProperty, resolvedWidth ?? 0d);
        Set(FrameworkElement.MinHeightProperty, 0d);
        Set(FrameworkElement.MaxWidthProperty, resolvedWidth ?? maximumWidth);
        if (resolvedWidth is double requested) Set(FrameworkElement.WidthProperty, requested);
        var text = theme.Text("", "bodySmall");
        Set(Control.FontFamilyProperty, text.FontFamily);
        Set(Control.FontSizeProperty, text.FontSize);
        Set(Control.FontWeightProperty, theme.Weight("bodySmall"));
        Set(Control.IsTextScaleFactorEnabledProperty, theme.Sizing != ThemeSizing.System);
        Set(Control.HorizontalContentAlignmentProperty, HorizontalAlignment.Stretch);
        Set(Control.VerticalContentAlignmentProperty, VerticalAlignment.Stretch);
        Set(Control.IsTabStopProperty, false);
        Set(FrameworkElement.UseLayoutRoundingProperty, false);
        return style;
    }

    internal static void Lift(Control owner, Theme theme, string token)
    {
        owner.ApplyTemplate();
        owner.Shadow = null;
        if (Walk(owner).OfType<FrameworkElement>().FirstOrDefault(element => element.Name == "Lift") is { } lift)
            Elevation.Draw(owner, lift, theme, token);
    }

    internal static void UpdateProse(FlyoutPresenter presenter, Theme theme, double maximumWidth, double? width)
    {
        // WinRT Setter.Value can rebox enum/struct values for its XAML
        // metadata. Use the typed properties when updating a live presenter.
        var resolved = width is double specified ? Math.Min(specified, maximumWidth) : (double?)null;
        presenter.Background = theme.Brush("color.surface-overlay");
        presenter.Foreground = theme.Brush("color.content");
        presenter.BorderBrush = theme.Brush("color.border-strong");
        presenter.BorderThickness = new Thickness(theme.Number("stroke.hairline"));
        Skin.SetCorner(presenter, theme.Number("radius.large"));
        presenter.Padding = new Thickness(theme.Number("spacing.4") + theme.Number("spacing.0.5"));
        presenter.MinWidth = resolved ?? 0;
        presenter.MinHeight = 0;
        presenter.MaxWidth = resolved ?? maximumWidth;
        presenter.Width = resolved ?? double.NaN;
        var text = theme.Text("", "bodySmall");
        presenter.FontFamily = text.FontFamily;
        presenter.FontSize = text.FontSize;
        presenter.FontWeight = theme.Weight("bodySmall");
        presenter.IsTextScaleFactorEnabled = theme.Sizing != ThemeSizing.System;
        presenter.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
    }

    private const string ProseTemplate = """
        <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="FlyoutPresenter">
          <Grid>
            <Border x:Name="Lift" IsHitTestVisible="False" />
            <Border BackgroundSizing="OuterBorderEdge" Background="{TemplateBinding Background}"
                BorderBrush="{TemplateBinding BorderBrush}" BorderThickness="{TemplateBinding BorderThickness}"
                CornerRadius="{TemplateBinding CornerRadius}">
              <ScrollViewer x:Name="ScrollViewer" ZoomMode="Disabled"
                  HorizontalScrollMode="Disabled" HorizontalScrollBarVisibility="Disabled"
                  VerticalScrollMode="Auto" VerticalScrollBarVisibility="Auto">
                <ContentPresenter Content="{TemplateBinding Content}" ContentTemplate="{TemplateBinding ContentTemplate}"
                    Padding="{TemplateBinding Padding}" HorizontalContentAlignment="{TemplateBinding HorizontalContentAlignment}"
                    VerticalContentAlignment="{TemplateBinding VerticalContentAlignment}" />
              </ScrollViewer>
            </Border>
          </Grid>
        </ControlTemplate>
        """;

    internal sealed class Hint : ToolTip
    {
        private readonly DependencyObject target;
        private readonly TextBlock label = new() { TextWrapping = TextWrapping.Wrap };
        private Theme? applied;
        private double appliedScale;

        internal Hint(DependencyObject target, string text)
        {
            this.target = target;
            label.Text = text;
            Content = label;
            IsHitTestVisible = false;
            UseLayoutRounding = false;
            Template = (ControlTemplate)XamlReader.Load(HintTemplate);
            Opened += (_, _) =>
            {
                LayoutUpdated -= FollowAppearance;
                LayoutUpdated += FollowAppearance;
                Apply();
            };
            Closed += (_, _) => LayoutUpdated -= FollowAppearance;
            Apply();
            // Skin clamps full-radius themes after layout. Keep the native
            // shadow's silhouette on that same resolved radius and size.
            RegisterPropertyChangedCallback(CornerRadiusProperty, (_, _) => DrawShadow());
            SizeChanged += (_, _) => DrawShadow();
        }

        private void FollowAppearance(object? sender, object args)
        {
            var theme = Appearance.GetTheme(target);
            if (IsOpen && (!ReferenceEquals(theme, applied) || appliedScale != theme.LayoutScale)) Apply();
        }

        protected override void OnApplyTemplate()
        {
            base.OnApplyTemplate();
            Apply();
        }

        private void Apply()
        {
            var theme = Appearance.GetTheme(target);
            applied = theme;
            appliedScale = theme.LayoutScale;
            theme.Typography(this, "labelSmall");
            theme.Text("", "labelSmall").CopyTo(label);
            label.Foreground = Foreground = theme.Brush("color.content");
            Background = theme.Brush("color.surface-raised");
            BorderBrush = theme.Brush("color.border");
            BorderThickness = new Thickness(theme.Number("stroke.hairline"));
            Skin.SetCorner(this, theme.Number("radius.small"));
            Padding = new Thickness(theme.Number("spacing.2"), theme.Number("spacing.1") + theme.Number("spacing.0.5"),
                theme.Number("spacing.2"), theme.Number("spacing.1") + theme.Number("spacing.0.5"));
            MaxWidth = theme.Number("toast.max-width");
            RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
            DrawShadow();
            Shadow = null;
        }

        private void DrawShadow()
        {
            if (applied is Theme theme && GetTemplateChild("Lift") is FrameworkElement lift)
                Elevation.Draw(this, lift, theme, "sm");
        }

        private const string HintTemplate = """
            <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
                xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="ToolTip">
              <Grid>
                <Border x:Name="Lift" IsHitTestVisible="False" />
                <ContentPresenter BackgroundSizing="OuterBorderEdge" Background="{TemplateBinding Background}"
                    BorderBrush="{TemplateBinding BorderBrush}" BorderThickness="{TemplateBinding BorderThickness}"
                    CornerRadius="{TemplateBinding CornerRadius}" Padding="{TemplateBinding Padding}"
                    Content="{TemplateBinding Content}" ContentTemplate="{TemplateBinding ContentTemplate}" />
              </Grid>
            </ControlTemplate>
            """;
    }
}
