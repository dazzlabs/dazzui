using System.Runtime.CompilerServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Markup;
using Microsoft.UI.Xaml.Media;

namespace DazzUI;

/// <summary>Paints the native AutoSuggestBox parts without replacing its editor or list.</summary>
internal static class QueryPopupSkin
{
    internal enum RowKind { Heading, Empty }
    private static readonly ConditionalWeakTable<Autocomplete, PopupPosition> Positions = new();

    internal static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        if (root is Popup { Child: not null } popup)
        {
            foreach (var child in Walk(popup.Child)) yield return child;
            yield break;
        }
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }

    internal static TextBox? Editor(AutoSuggestBox input) => Walk(input).OfType<TextBox>().FirstOrDefault(box => box.Name == "TextBox");
    internal static ListView? List(AutoSuggestBox input) => Walk(input).OfType<ListView>().FirstOrDefault(list => list.Name == "SuggestionsList");

    internal static bool ContainsFocus(Autocomplete owner)
    {
        if (!owner.IsMounted || owner.XamlRoot is null) return false;
        var focused = FocusManager.GetFocusedElement(owner.XamlRoot);
        return Walk(owner).Any(element => ReferenceEquals(element, focused));
    }

    internal static void HideNativeActions(TextBox editor)
    {
        foreach (var button in Walk(editor).OfType<Microsoft.UI.Xaml.Controls.Button>().Where(button => button.Name is "DeleteButton" or "QueryButton"))
        {
            button.Visibility = Visibility.Collapsed;
            button.Opacity = 0;
            button.Width = button.MinWidth = 0;
            button.Padding = button.Margin = new Thickness(0);
            button.IsHitTestVisible = false;
            button.IsTabStop = false;
            if (VisualTreeHelper.GetParent(button) is Grid { ColumnDefinitions.Count: 4 } row)
                row.ColumnDefinitions[3].Width = new GridLength(0);
        }
    }

    internal static void Sheet(Autocomplete owner)
    {
        if (!owner.IsMounted) return;
        var popup = Walk(owner.Input).OfType<Popup>().FirstOrDefault(part => part.Name == "SuggestionsPopup");
        if (popup?.Child is null) return;
        var surface = Walk(popup.Child).OfType<Border>().FirstOrDefault(part => part.Name == "SuggestionsContainer");
        if (surface is null) return;
        var list = Walk(surface).OfType<ListView>().FirstOrDefault(part => part.Name == "SuggestionsList");
        if (list is not null)
        {
            list.Margin = list.Padding = list.BorderThickness = new Thickness(0);
            list.MinWidth = list.MinHeight = 0;
            list.Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent);
            if (owner.XamlRoot is not null) list.MaxHeight = Math.Max(0, owner.XamlRoot.Size.Height - owner.Theme.Number("spacing.1") * 2);
        }
        var host = SelectionPopupSkin.Sheet(popup, surface, owner.Theme, owner.Input.ActualWidth);
        Positions.GetValue(owner, value => new PopupPosition(value)).Apply(popup, host);
    }

    internal static void Detach(Autocomplete owner)
    {
        if (Positions.TryGetValue(owner, out var position)) position.Detach();
    }

    private sealed class PopupPosition(Autocomplete owner)
    {
        private Popup? popup;
        private FrameworkElement? host;
        private long offsetToken;

        internal void Apply(Popup value, FrameworkElement content)
        {
            if (!ReferenceEquals(popup, value))
            {
                Detach();
                popup = value;
                offsetToken = popup.RegisterPropertyChangedCallback(Popup.VerticalOffsetProperty, (_, _) => Position());
            }
            host = content;
            Position();
        }

        private void Position()
        {
            if (!owner.IsMounted || popup is null || host is null) return;
            // ASB anchors at the editor's inner edge, one hairline inside the
            // field. Keep the shared gap outside that edge in either native
            // placement; a bottom margin alone cannot move an upward popup.
            var offset = owner.Theme.Number("spacing.1") + owner.Theme.Number("stroke.hairline");
            var above = popup.VerticalOffset < 0;
            host.Margin = above ? new Thickness(0, 0, 0, offset) : new Thickness(0, offset, 0, 0);
            host.RenderTransform = new TranslateTransform { Y = above ? -offset : 0 };
        }

        internal void Detach()
        {
            if (popup is not null) popup.UnregisterPropertyChangedCallback(Popup.VerticalOffsetProperty, offsetToken);
            popup = null;
            host = null;
        }
    }

    internal sealed class ActionButton : Microsoft.UI.Xaml.Controls.Button
    {
        private readonly Icon glyph;
        private Theme theme = Theme.Default;
        private bool hovered;

        internal ActionButton(Glyph icon, string label)
        {
            glyph = new Icon { Glyph = icon, IsHitTestVisible = false };
            Content = glyph;
            MinWidth = MinHeight = 0;
            Padding = BorderThickness = new Thickness(0);
            IsTabStop = false;
            AllowFocusOnInteraction = false;
            UseLayoutRounding = false;
            Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(this, label);
            Template = (ControlTemplate)XamlReader.Load("""
                <ControlTemplate xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" TargetType="Button">
                  <Border Background="{TemplateBinding Background}" CornerRadius="{TemplateBinding CornerRadius}">
                    <ContentPresenter Content="{TemplateBinding Content}" HorizontalAlignment="Center" VerticalAlignment="Center" />
                  </Border>
                </ControlTemplate>
                """);
            PointerEntered += (_, _) => { hovered = true; Ink(); };
            PointerExited += (_, _) => { hovered = false; Ink(); };
        }

        internal void Apply(Theme appearance, bool enabled)
        {
            theme = appearance;
            IsEnabled = enabled;
            Width = Height = theme.Number("spacing.5");
            Skin.SetCorner(this, theme.Number("radius.small"));
            Skin.Focus(this, theme);
            FocusVisualMargin = new Thickness(0);
            glyph.Theme = theme;
            glyph.GlyphSize = theme.Number("icon.small");
            Appearance.Refresh(glyph);
            Ink();
        }

        private void Ink()
        {
            Background = IsEnabled && hovered ? theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "hovered") : new SolidColorBrush(Microsoft.UI.Colors.Transparent);
            glyph.Foreground = theme.Brush(IsEnabled && hovered ? "color.content" : "color.content-subtle");
        }
    }
}
