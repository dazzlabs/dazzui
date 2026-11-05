using System.Runtime.CompilerServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Markup;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

/// <summary>The shared popup recipe over native selection containers and scrolling.</summary>
internal static class SelectionPopupSkin
{
    private static readonly ConditionalWeakTable<SelectorItem, RowState> Rows = new();
    private static readonly ConditionalWeakTable<Border, SheetHost> CanvasLifts = new();

    internal static FrameworkElement Sheet(Popup popup, Border border, Theme theme, double minimumWidth)
    {
        SheetHost host;
        if (VisualTreeHelper.GetParent(border) is Canvas canvas)
        {
            // ComboBox owns this Canvas, its light-dismiss region, and the
            // coordinates of PopupBorder. A sibling lift follows that actual
            // painted rectangle without changing the native popup hierarchy.
            host = CanvasLifts.GetValue(border, _ => new SheetHost { IsHitTestVisible = false });
            if (!ReferenceEquals(VisualTreeHelper.GetParent(host), canvas))
            {
                Skin.Detach(host);
                canvas.Children.Insert(canvas.Children.IndexOf(border), host);
            }
            host.Track(border);
        }
        else
        {
            var content = popup.Child;
            host = content is null ? new SheetHost() : OverlaySkin.Walk(content).OfType<SheetHost>().FirstOrDefault()!;
            if (host is null)
            {
                popup.Child = null;
                host = new SheetHost { Content = content };
                popup.Child = host;
            }
        }
        host.Apply(theme, minimumWidth);
        border.Background = theme.Brush("color.surface-overlay");
        border.BackgroundSizing = BackgroundSizing.OuterBorderEdge;
        border.BorderBrush = theme.Brush("color.border-strong");
        border.BorderThickness = new Thickness(theme.Number("stroke.hairline"));
        border.Padding = new Thickness(theme.Number("spacing.1"));
        border.CornerRadius = new CornerRadius(theme.Number("radius.large"));
        border.Margin = new Thickness(0);
        border.MinWidth = minimumWidth;
        border.UseLayoutRounding = false;
        border.Shadow = null;
        return host;
    }

    internal static void Row(SelectorItem item, Theme theme, bool? selected = null, bool check = true,
        bool heading = false, bool empty = false, bool groupGap = false)
    {
        if (!Rows.TryGetValue(item, out var row))
        {
            row = new RowState(item);
            Rows.Add(item, row);
        }
        row.Apply(theme, selected, check, heading, empty, groupGap);
    }

    private sealed class RowState
    {
        private readonly SelectorItem item;
        private readonly Icon mark = new() { Glyph = FluentIcons.Common.Icon.Checkmark, GlyphVariant = FluentIcons.Common.IconVariant.Filled };
        private Theme theme = Theme.Default;
        private bool? selected;
        private bool check;
        private bool heading;
        private bool empty;
        private bool hovered;
        private bool prepared;

        internal RowState(SelectorItem item)
        {
            this.item = item;
            item.Template = (ControlTemplate)XamlReader.Load(RowTemplate(item is ComboBoxItem ? "ComboBoxItem" : "ListViewItem"));
            item.Loaded += (_, _) => { Paint(); WatchPreparation(); };
            item.GotFocus += (_, _) => Ink();
            item.LostFocus += (_, _) => Ink();
            item.PointerEntered += (_, _) => { hovered = true; Ink(); };
            item.PointerExited += (_, _) => { hovered = false; Ink(); };
            item.Unloaded += (_, _) => { hovered = false; item.LayoutUpdated -= PrepareAfterLayout; };
            item.IsEnabledChanged += (_, _) => Ink();
            item.RegisterPropertyChangedCallback(SelectorItem.IsSelectedProperty, (_, _) => Ink());
            if (item is ComboBoxItem)
            {
                // Base UI Select includes disabled options in arrow focus,
                // but neither confirmation key commits or closes on them.
                // Keep WinUI's native navigation and disabled item semantics.
                void DisabledConfirmation(object sender, Microsoft.UI.Xaml.Input.KeyRoutedEventArgs args)
                {
                    if (!item.IsEnabled && args.Key is Windows.System.VirtualKey.Enter or Windows.System.VirtualKey.Space)
                        args.Handled = true;
                }
                item.PreviewKeyDown += DisabledConfirmation;
                item.PreviewKeyUp += DisabledConfirmation;
            }
        }

        private void PrepareAfterLayout(object? sender, object args) => Paint();
        private void WatchPreparation()
        {
            item.LayoutUpdated -= PrepareAfterLayout;
            // Wait only until the native presenter has materialized its text.
            // Closed/recycled popup rows must not keep global layout listeners.
            if (item.IsLoaded && !prepared) item.LayoutUpdated += PrepareAfterLayout;
        }

        internal void Apply(Theme value, bool? chosen, bool showCheck, bool group, bool noResults, bool groupGap)
        {
            theme = value; selected = chosen; check = showCheck && !group && !noResults;
            heading = group; empty = noResults; prepared = false;
            if (item is ComboBoxItem) item.AllowFocusWhenDisabled = !heading && !empty;
            Appearance.SetTheme(item, theme);
            item.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
            theme.Typography(item, heading ? "captionSmall" : empty ? "bodySmall" : "labelQuiet");
            if (heading) item.FontWeight = theme.Weight("labelSmall");
            item.UseLayoutRounding = false;
            item.UseSystemFocusVisuals = false;
            item.MinWidth = item.MinHeight = 0;
            item.BorderThickness = new Thickness(0);
            item.HorizontalContentAlignment = HorizontalAlignment.Stretch;
            item.VerticalContentAlignment = VerticalAlignment.Center;
            var top = heading ? theme.Number("spacing.1.5") : theme.Number("menu.item-padding");
            var bottom = heading ? theme.Number("spacing.1") : top;
            item.Padding = new Thickness(theme.Number("spacing.2"), top, theme.Number("spacing.2"), bottom);
            item.Margin = new Thickness(0, groupGap ? theme.Number("spacing.1") : 0, 0, 0);
            item.Height = top + bottom + (empty ? theme.Text("", "bodySmall").LineHeight : Math.Max(item.FontSize, check ? theme.Number("icon.small") : 0));
            item.CornerRadius = new CornerRadius(Math.Max(0, theme.Number("radius.large") - theme.Number("spacing.1")));
            item.ApplyTemplate();
            Paint();
            WatchPreparation();
        }

        private void Paint()
        {
            if (PopupSkin.Part<Grid>(item, "DazzSelectionRow") is not Grid row
                || PopupSkin.Part<ContentPresenter>(item, "DazzSelectionCheck") is not ContentPresenter slot
                || PopupSkin.Part<ContentPresenter>(item, "ContentPresenter") is not ContentPresenter content) return;
            row.ColumnSpacing = check ? theme.Number("spacing.2") : 0;
            slot.Visibility = check ? Visibility.Visible : Visibility.Collapsed;
            slot.Width = slot.Height = theme.Number("icon.small");
            if (!ReferenceEquals(slot.Content, mark)) slot.Content = mark;
            mark.GlyphSize = slot.Width;
            Appearance.SetTheme(mark, theme);
            Appearance.Refresh(mark);
            var labels = OverlaySkin.Walk(content).OfType<TextBlock>().ToArray();
            foreach (var label in labels)
            {
                theme.Text("", heading ? "captionSmall" : empty ? "bodySmall" : "labelQuiet").CopyTo(label);
                if (heading) label.FontWeight = theme.Weight("labelSmall");
                label.TextWrapping = TextWrapping.NoWrap;
                label.TextTrimming = TextTrimming.CharacterEllipsis;
                if (!empty) label.LineHeight = label.FontSize;
                label.Height = label.LineHeight;
                label.UseLayoutRounding = false;
                label.IsTextScaleFactorEnabled = theme.Sizing != ThemeSizing.System;
                Align(label);
            }
            prepared = labels.Length > 0 || item.Content is not string;
            if (prepared) item.LayoutUpdated -= PrepareAfterLayout;
            Ink();
        }

        private static readonly ConditionalWeakTable<TextBlock, LabelAlignment> Aligned = new();
        private static void Align(TextBlock label) => Aligned.GetValue(label, value => new LabelAlignment(value)).Request();
        private sealed class LabelAlignment
        {
            private readonly TextBlock label;
            internal LabelAlignment(TextBlock value)
            {
                label = value;
                label.Loaded += (_, _) => Request();
                label.SizeChanged += (_, _) => Request();
                label.Unloaded += (_, _) => label.LayoutUpdated -= AfterLayout;
            }
            internal void Request()
            {
                Position();
                label.LayoutUpdated -= AfterLayout;
                if (label.IsLoaded) label.LayoutUpdated += AfterLayout;
            }
            private void AfterLayout(object? sender, object args)
            {
                label.LayoutUpdated -= AfterLayout;
                Position();
            }
            private void Position()
            {
                var offset = PlatformFonts.LineBaseline(label.FontFamily, label.FontSize, label.FontWeight, label.Height) - label.BaselineOffset;
                if (label.RenderTransform is not TranslateTransform translation || Math.Abs(translation.Y - offset) > .001)
                    label.RenderTransform = new TranslateTransform { Y = offset };
            }
        }

        private void Ink()
        {
            // AutoSuggestBox keeps edit focus while its ListView selection
            // identifies the highlighted suggestion. A ComboBoxItem receives
            // native focus; its selected value only owns the checkmark.
            var highlighted = !heading && !empty && (hovered || item.FocusState != FocusState.Unfocused
                || item is ListViewItem && item.IsSelected);
            item.Background = highlighted ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered")
                : new SolidColorBrush(Microsoft.UI.Colors.Transparent);
            item.Foreground = empty ? theme.Brush("color.content-faint") : heading ? theme.Brush("color.content-subtle")
                : item.IsEnabled ? theme.Brush("color.content")
                : theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled");
            if (PopupSkin.Part<ContentPresenter>(item, "DazzSelectionCheck") is ContentPresenter slot)
                slot.Opacity = (selected ?? item.IsSelected) ? 1 : 0;
            mark.Foreground = item.Foreground;
            if (PopupSkin.Part<ContentPresenter>(item, "ContentPresenter") is ContentPresenter content)
                foreach (var label in OverlaySkin.Walk(content).OfType<TextBlock>()) label.Foreground = item.Foreground;
        }
    }

    private static string RowTemplate(string target) => $$"""
        <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
            xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="{{target}}">
          <Grid Background="{TemplateBinding Background}" Padding="{TemplateBinding Padding}" CornerRadius="{TemplateBinding CornerRadius}">
            <Grid x:Name="DazzSelectionRow" IsHitTestVisible="False">
              <Grid.ColumnDefinitions><ColumnDefinition Width="Auto" /><ColumnDefinition Width="*" /></Grid.ColumnDefinitions>
              <ContentPresenter x:Name="DazzSelectionCheck" VerticalAlignment="Center" Foreground="{TemplateBinding Foreground}" />
              <ContentPresenter x:Name="ContentPresenter" Grid.Column="1" Content="{TemplateBinding Content}"
                  ContentTemplate="{TemplateBinding ContentTemplate}" Foreground="{TemplateBinding Foreground}"
                  HorizontalAlignment="Stretch" VerticalAlignment="Center" />
            </Grid>
          </Grid>
        </ControlTemplate>
        """;

    private sealed class SheetHost : ContentControl
    {
        private Theme theme = Theme.Default;
        private Border? tracked;
        internal SheetHost()
        {
            IsTabStop = false;
            MinWidth = MinHeight = 0;
            Padding = new Thickness(0);
            HorizontalContentAlignment = HorizontalAlignment.Stretch;
            VerticalContentAlignment = VerticalAlignment.Stretch;
            Template = (ControlTemplate)XamlReader.Load("""
                <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
                    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml" TargetType="ContentControl">
                  <Grid>
                    <Border x:Name="DazzSelectionLift" IsHitTestVisible="False" />
                    <ContentPresenter Content="{TemplateBinding Content}" HorizontalContentAlignment="Stretch" VerticalContentAlignment="Stretch" />
                  </Grid>
                </ControlTemplate>
                """);
            Loaded += (_, _) => Lift();
        }
        internal void Track(Border border)
        {
            if (ReferenceEquals(tracked, border)) { FollowBounds(); return; }
            tracked = border;
            border.SizeChanged += (_, _) => FollowBounds();
            border.RegisterPropertyChangedCallback(Canvas.LeftProperty, (_, _) => FollowBounds());
            border.RegisterPropertyChangedCallback(Canvas.TopProperty, (_, _) => FollowBounds());
            border.RegisterPropertyChangedCallback(VisibilityProperty, (_, _) => FollowBounds());
            FollowBounds();
        }
        private void FollowBounds()
        {
            if (tracked is null) return;
            Width = tracked.ActualWidth;
            Height = tracked.ActualHeight;
            Canvas.SetLeft(this, Canvas.GetLeft(tracked));
            Canvas.SetTop(this, Canvas.GetTop(tracked));
            Visibility = tracked.Visibility;
        }
        internal void Apply(Theme value, double minimumWidth)
        {
            theme = value;
            Appearance.SetTheme(this, theme);
            RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
            MinWidth = minimumWidth;
            UseLayoutRounding = false;
            CornerRadius = new CornerRadius(theme.Number("radius.large"));
            ApplyTemplate();
            Lift();
        }
        private void Lift()
        {
            Shadow = null;
            if (GetTemplateChild("DazzSelectionLift") is FrameworkElement lift) Elevation.Draw(this, lift, theme, "lg");
        }
    }
}
