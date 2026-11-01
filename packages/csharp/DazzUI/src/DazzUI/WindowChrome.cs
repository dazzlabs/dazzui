using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Markup;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;
using Windows.UI;

namespace DazzUI;

/// <summary>One window control's state. Disabled and hidden controls are left out on WinUI.</summary>
public enum WindowControlState { Enabled, Disabled, Hidden }

/// <summary>The three caption controls, each with its own state.</summary>
public sealed class WindowControls
{
    public WindowControlState Minimize { get; set; } = WindowControlState.Enabled;
    public WindowControlState Maximize { get; set; } = WindowControlState.Enabled;
    public WindowControlState Close { get; set; } = WindowControlState.Enabled;
    public static WindowControls All => new();
    public static WindowControls CloseOnly => new() { Minimize = WindowControlState.Hidden, Maximize = WindowControlState.Hidden };
    public static WindowControls None => new() { Minimize = WindowControlState.Hidden, Maximize = WindowControlState.Hidden, Close = WindowControlState.Hidden };
    internal bool HasCaption => Minimize == WindowControlState.Enabled || Maximize == WindowControlState.Enabled || Close == WindowControlState.Enabled;
}

/// <summary>A pointer move over the titlebar, in the band's own coordinates.</summary>
public sealed class WindowDragEventArgs(double deltaX, double deltaY, bool wasMaximized) : EventArgs
{
    public double DeltaX { get; } = deltaX;
    public double DeltaY { get; } = deltaY;
    /// <summary>Whether the window was maximized when the drag began; the host restores first.</summary>
    public bool WasMaximized { get; } = wasMaximized;
}

/// <summary>
/// A window's titlebar band: the title and its context, the optional leading,
/// toolbar and trailing slots, and the platform's caption controls. The host
/// owns the window; the band raises minimize, maximize, close and drag.
/// </summary>
public sealed class WindowTitlebar : Surface
{
    private object? leading;
    private object? title;
    private string? subtitle;
    private object? toolbar;
    private object? trailing;
    private object? accessory;
    private WindowControls controls = WindowControls.All;
    private bool inactive;
    private bool fullSizeContent;
    private bool compact;
    private bool titleHidden;
    private bool windowMaximized;
    private bool dirty = true;

    public object? Leading { get => leading; set { leading = value; Invalidate(); } }
    public object? Title { get => title; set { title = value; Invalidate(); } }
    public string? Subtitle { get => subtitle; set { subtitle = value; Invalidate(); } }
    public object? Toolbar { get => toolbar; set { toolbar = value; Invalidate(); } }
    public object? Trailing { get => trailing; set { trailing = value; Invalidate(); } }
    public object? Accessory { get => accessory; set { accessory = value; Invalidate(); } }
    public WindowControls Controls { get => controls; set { controls = value ?? WindowControls.All; Invalidate(); } }
    public bool Inactive { get => inactive; set { inactive = value; Invalidate(); } }
    public bool FullSizeContent { get => fullSizeContent; set { fullSizeContent = value; Invalidate(); } }
    /// <summary>The platform's 32px band; the default is DazzUI's 52px toolbar band.</summary>
    public bool Compact { get => compact; set { compact = value; Invalidate(); } }
    public bool TitleHidden { get => titleHidden; set { titleHidden = value; Invalidate(); } }
    /// <summary>The host's current window state, passed with each drag.</summary>
    public bool IsWindowMaximized { get => windowMaximized; set => windowMaximized = value; }
    public string MinimizeLabel { get; set; } = "Minimize";
    public string MaximizeLabel { get; set; } = "Maximize";
    public string CloseLabel { get; set; } = "Close";

    public event EventHandler? MinimizeRequested;
    public event EventHandler? MaximizeRequested;
    public event EventHandler? CloseRequested;
    public event EventHandler<WindowDragEventArgs>? DragRequested;

    private bool dragging;
    private Point last;
    private bool dragWasMaximized;

    public WindowTitlebar()
    {
        IsTabStop = false;
        HorizontalContentAlignment = HorizontalAlignment.Stretch;
    }

    protected override Windows.Foundation.Size MeasureOverride(Windows.Foundation.Size availableSize)
    {
        if (dirty && XamlRoot is not null) Build();
        return base.MeasureOverride(availableSize);
    }

    protected override void RefreshAppearance()
    {
        dirty = false;
        Build();
    }

    private void Invalidate()
    {
        dirty = true;
        if (XamlRoot is not null)
        {
            dirty = false;
            Build();
        }
    }

    private void Build()
    {
        dirty = false;
        var band = FullSizeContent || !Compact || Toolbar is not null || Accessory is not null
            ? Theme.Number("frame.titlebar.size")
            : 32;
        Height = band;
        Padding = new Thickness(Theme.Number("spacing.4"), 0, Controls.HasCaption ? 0 : Theme.Number("spacing.4"), 0);
        BorderThickness = new Thickness(0);
        Background = null;

        var row = new Grid { Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent) };
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });

        if (Leading is not null)
        {
            var slot = Slot(Leading);
            row.Children.Add(slot);
            Grid.SetColumn(slot, 0);
        }

        if (Title is not null)
        {
            var titleRow = new StackPanel { Orientation = Orientation.Horizontal, Spacing = Theme.Number("spacing.2"), VerticalAlignment = VerticalAlignment.Center };
            if (Title is string words)
            {
                var text = Theme.Text(words, "captionMedium");
                text.Foreground = Theme.Brush(Inactive ? "color.content-faint" : "color.content");
                text.VerticalAlignment = VerticalAlignment.Center;
                text.TextTrimming = TextTrimming.CharacterEllipsis;
                text.TextWrapping = TextWrapping.NoWrap;
                titleRow.Children.Add(text);
            }
            else titleRow.Children.Add(Slot(Title));
            if (!string.IsNullOrEmpty(Subtitle))
            {
                var context = Theme.Text(Subtitle, "captionSmall");
                context.Foreground = Theme.Brush("color.content-subtle");
                context.VerticalAlignment = VerticalAlignment.Center;
                context.TextTrimming = TextTrimming.CharacterEllipsis;
                context.TextWrapping = TextWrapping.NoWrap;
                titleRow.Children.Add(context);
            }
            titleRow.Visibility = TitleHidden ? Visibility.Collapsed : Visibility.Visible;
            row.Children.Add(titleRow);
            Grid.SetColumn(titleRow, 1);
        }

        if (Toolbar is not null)
        {
            var slot = Slot(Toolbar);
            slot.HorizontalAlignment = HorizontalAlignment.Stretch;
            row.Children.Add(slot);
            Grid.SetColumn(slot, 2);
        }
        if (Trailing is not null)
        {
            var slot = Slot(Trailing);
            row.Children.Add(slot);
            Grid.SetColumn(slot, 3);
        }
        if (Controls.HasCaption)
        {
            var caption = Caption(band);
            row.Children.Add(caption);
            Grid.SetColumn(caption, 4);
        }
        if (Accessory is not null)
        {
            var slot = Slot(Accessory);
            slot.HorizontalAlignment = HorizontalAlignment.Center;
            slot.VerticalAlignment = VerticalAlignment.Center;
            slot.IsHitTestVisible = false;
            row.Children.Add(slot);
            Grid.SetColumnSpan(slot, 5);
        }

        row.PointerPressed += Pressed;
        row.PointerMoved += Moved;
        row.PointerReleased += Released;
        row.PointerCaptureLost += (_, _) => dragging = false;
        row.DoubleTapped += (_, args) =>
        {
            if (Controls.Maximize != WindowControlState.Enabled || InsideButton(args.OriginalSource as DependencyObject)) return;
            MaximizeRequested?.Invoke(this, EventArgs.Empty);
            args.Handled = true;
        };
        Content = row;
    }

    private FrameworkElement Slot(object content)
    {
        if (content is FrameworkElement element)
        {
            Skin.Detach(element);
            element.VerticalAlignment = VerticalAlignment.Center;
            return element;
        }
        if (content is UIElement other)
        {
            Skin.Detach(other);
            return new ContentControl
            {
                Content = other,
                VerticalAlignment = VerticalAlignment.Center,
                IsTabStop = false,
            };
        }
        var text = Theme.Text(content.ToString() ?? "", "captionMedium");
        text.VerticalAlignment = VerticalAlignment.Center;
        return text;
    }

    private FrameworkElement Caption(double band)
    {
        var row = new StackPanel { Orientation = Orientation.Horizontal, VerticalAlignment = VerticalAlignment.Stretch };
        if (Controls.Minimize == WindowControlState.Enabled)
            row.Children.Add(new CaptionButton(this, WindowCaptionKind.Minimize, FluentIcons.Common.Icon.Subtract, Theme, band, 12, MinimizeLabel));
        if (Controls.Maximize == WindowControlState.Enabled)
            row.Children.Add(new CaptionButton(this, WindowCaptionKind.Maximize, FluentIcons.Common.Icon.Square, Theme, band, 16, MaximizeLabel));
        if (Controls.Close == WindowControlState.Enabled)
            row.Children.Add(new CaptionButton(this, WindowCaptionKind.Close, FluentIcons.Common.Icon.Dismiss, Theme, band, 12, CloseLabel));
        return row;
    }

    private void Pressed(object sender, PointerRoutedEventArgs args)
    {
        if (args.GetCurrentPoint(this).Properties.IsLeftButtonPressed is false) return;
        if (InsideButton(args.OriginalSource as DependencyObject)) return;
        dragging = true;
        last = args.GetCurrentPoint(this).Position;
        dragWasMaximized = IsWindowMaximized;
        (Content as UIElement)?.CapturePointer(args.Pointer);
        args.Handled = true;
    }

    private void Moved(object sender, PointerRoutedEventArgs args)
    {
        if (!dragging) return;
        var point = args.GetCurrentPoint(this);
        if (!point.Properties.IsLeftButtonPressed) { dragging = false; return; }
        var now = point.Position;
        var dx = now.X - last.X;
        var dy = now.Y - last.Y;
        if (Math.Abs(dx) + Math.Abs(dy) < 0.01) return;
        last = now;
        DragRequested?.Invoke(this, new WindowDragEventArgs(dx, dy, dragWasMaximized));
        args.Handled = true;
    }

    private void Released(object sender, PointerRoutedEventArgs args)
    {
        if (!dragging) return;
        dragging = false;
        (Content as UIElement)?.ReleasePointerCapture(args.Pointer);
        args.Handled = true;
    }

    private static bool InsideButton(DependencyObject? source)
    {
        for (var current = source; current is not null; current = VisualTreeHelper.GetParent(current))
            if (current is ButtonBase) return true;
        return false;
    }

    private enum WindowCaptionKind { Minimize, Maximize, Close }

    private sealed class CaptionButton : Microsoft.UI.Xaml.Controls.Button
    {
        private const string CaptionTemplate = """
            <ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='Button'>
              <Border Background='{TemplateBinding Background}' Padding='{TemplateBinding Padding}'>
                <ContentPresenter Content='{TemplateBinding Content}' HorizontalContentAlignment='Center' VerticalContentAlignment='Center'/>
              </Border>
            </ControlTemplate>
            """;
        private readonly WindowTitlebar owner;
        private readonly WindowCaptionKind kind;
        private readonly Theme theme;
        private readonly Icon glyph;
        private bool hovered;
        private bool pressed;

        internal CaptionButton(WindowTitlebar owner, WindowCaptionKind kind, FluentIcons.Common.Icon icon, Theme theme, double band, double glyphSize, string label)
        {
            this.owner = owner;
            this.kind = kind;
            this.theme = theme;
            Width = 46;
            Height = band;
            MinWidth = MinHeight = 0;
            Padding = new Thickness(0);
            BorderThickness = new Thickness(0);
            CornerRadius = new CornerRadius(0);
            IsTabStop = false;
            Template = (ControlTemplate)XamlReader.Load(CaptionTemplate);
            glyph = new Icon { Glyph = icon, GlyphSize = glyphSize, IsHitTestVisible = false, HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
            Content = glyph;
            AutomationProperties.SetName(this, label);
            SetValue(ToolTipService.ToolTipProperty, label);
            PointerEntered += (_, _) => { hovered = true; Paint(); };
            PointerExited += (_, _) => { hovered = false; pressed = false; Paint(); };
            PointerPressed += (_, _) => { pressed = true; Paint(); };
            PointerReleased += (_, _) => { pressed = false; Paint(); };
            IsEnabledChanged += (_, _) => Paint();
            Click += (_, _) =>
            {
                switch (kind)
                {
                    case WindowCaptionKind.Minimize: owner.MinimizeRequested?.Invoke(owner, EventArgs.Empty); break;
                    case WindowCaptionKind.Maximize: owner.MaximizeRequested?.Invoke(owner, EventArgs.Empty); break;
                    default: owner.CloseRequested?.Invoke(owner, EventArgs.Empty); break;
                }
            };
            Paint();
        }

        private void Paint()
        {
            var faint = theme.Brush("color.content-faint");
            var ink = owner.Inactive ? faint : theme.Brush("color.content");
            Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent);
            if (kind is WindowCaptionKind.Close && hovered)
            {
                Background = new SolidColorBrush(Color.FromArgb(255, 196, 43, 28));
                ink = new SolidColorBrush(Microsoft.UI.Colors.White);
            }
            else if (hovered || pressed)
            {
                Background = theme.Brush("color.surface-subtle");
                if (pressed) ink = theme.Brush("color.content-subtle");
            }
            Foreground = ink;
            glyph.Foreground = ink;
        }
    }
}
