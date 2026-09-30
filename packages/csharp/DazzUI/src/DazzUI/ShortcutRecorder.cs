using Microsoft.UI.Input;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Windows.System;
using Windows.UI.Core;

namespace DazzUI;

/// <summary>
/// A field that records a key chord: pressed, it listens; the next chord is
/// its value. Drawn as a field, not a key cap — the chord in the strong face,
/// the empty prompt faint and quiet — with a clear at its end once it holds
/// one. In error the whole chip is repainted in danger.
/// </summary>
public sealed class ShortcutRecorder : Surface
{
    private const string FieldTemplate = "<ControlTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='Button'><Border Background='{TemplateBinding Background}' BorderBrush='{TemplateBinding BorderBrush}' BorderThickness='{TemplateBinding BorderThickness}' CornerRadius='{TemplateBinding CornerRadius}' Padding='{TemplateBinding Padding}'><ContentPresenter Content='{TemplateBinding Content}' HorizontalAlignment='Center' VerticalAlignment='Center'/></Border></ControlTemplate>";
    private readonly Microsoft.UI.Xaml.Controls.Button field = new();
    private readonly Microsoft.UI.Xaml.Controls.Button clear = new() { Padding = new Thickness(0), MinWidth = 0, MinHeight = 0, BorderThickness = new Thickness(0), HorizontalAlignment = HorizontalAlignment.Right, VerticalAlignment = VerticalAlignment.Center };
    private string shortcut = "";
    private bool recording;
    public string Shortcut { get => shortcut; set { shortcut = value; if (IsLoaded) RefreshAppearance(); } }
    public Func<string, string?>? Validate { get; set; }
    public string? Error { get; private set; }
    /// <summary>A stored conflict: the chip is repainted in danger.</summary>
    public bool Invalid { get; set; }
    public event EventHandler<string>? ShortcutChanged;
    /// <summary>The field that listens, for automation and focus.</summary>
    public Microsoft.UI.Xaml.Controls.Button Field => field;
    public ShortcutRecorder()
    {
        field.Template = (ControlTemplate)Microsoft.UI.Xaml.Markup.XamlReader.Load(FieldTemplate);
        clear.Content = new Icon { Glyph = FluentIcons.Common.Icon.DismissCircle, GlyphVariant = FluentIcons.Common.IconVariant.Filled };
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(clear, "Clear shortcut");
        clear.Click += (_, _) => { Shortcut = ""; ShortcutChanged?.Invoke(this, Shortcut); };
        Content = new Grid { Children = { field, clear } };
        HorizontalAlignment = HorizontalAlignment.Left;
        field.Click += (_, _) => { recording = true; field.Focus(FocusState.Keyboard); RefreshAppearance(); };
        field.LostFocus += (_, _) => { recording = false; RefreshAppearance(); };
        IsEnabledChanged += (_, _) => RefreshAppearance();
        field.KeyDown += (_, args) =>
        {
            if (!recording) return;
            args.Handled = true;
            if (args.Key == VirtualKey.Escape) { recording = false; RefreshAppearance(); return; }
            if (args.Key is VirtualKey.Control or VirtualKey.Shift or VirtualKey.Menu or VirtualKey.LeftWindows or VirtualKey.RightWindows) return;
            var modifiers = new List<string>();
            if (Down(VirtualKey.Control)) modifiers.Add("Ctrl");
            if (Down(VirtualKey.Menu)) modifiers.Add("Alt");
            if (Down(VirtualKey.Shift)) modifiers.Add("Shift");
            if (Down(VirtualKey.LeftWindows) || Down(VirtualKey.RightWindows)) modifiers.Add("Win");
            if (args.Key is VirtualKey.Back or VirtualKey.Delete && modifiers.Count == 0) shortcut = "";
            else
            {
                modifiers.Add(args.Key.ToString()); var candidate = string.Join('+', modifiers);
                Error = Validate?.Invoke(candidate);
                if (Error is not null) { field.Content = Error; return; }
                shortcut = candidate;
            }
            recording = false; RefreshAppearance(); ShortcutChanged?.Invoke(this, Shortcut);
        };
    }
    private static bool Down(VirtualKey key) => InputKeyboardSource.GetKeyStateForCurrentThread(key).HasFlag(CoreVirtualKeyStates.Down);
    protected override void RefreshAppearance()
    {
        var empty = shortcut.Length == 0;
        var clearable = !empty && IsEnabled;
        field.Content = recording ? "Press shortcut…" : empty ? "Record shortcut" : shortcut;
        field.Height = Theme.Number("control.medium.size");
        field.MinWidth = Theme.Number("shortcut-recorder.width");
        field.Padding = new Thickness(Theme.Number(clearable ? "spacing.8" : "spacing.3"), 0, Theme.Number(clearable ? "spacing.8" : "spacing.3"), 0);
        field.BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
        field.CornerRadius = new CornerRadius(Math.Min(Theme.Number("control.field.radius"), field.Height / 2));
        Theme.Typography(field, empty ? "labelQuiet" : "labelStrong");
        var danger = Invalid && !recording;
        field.Background = recording ? Theme.Brush("color.surface") : danger ? Theme.Brush("color.danger.50") : Theme.Brush("color.surface-muted");
        field.BorderBrush = recording ? Theme.Brush($"color.{Theme.Key(Tint)}.600") : danger ? Theme.Brush("color.danger.600") : Theme.Brush("color.border-strong");
        field.Foreground = danger ? Theme.Brush("color.danger.800") : Theme.Brush(empty || !IsEnabled ? "color.content-faint" : "color.content");
        Skin.Focus(field, Theme);
        Opacity = IsEnabled ? 1 : 0.6;
        clear.Visibility = clearable ? Visibility.Visible : Visibility.Collapsed;
        clear.Margin = new Thickness(0, 0, Theme.Number("spacing.2"), 0);
        clear.Background = new SolidColorBrush(Microsoft.UI.Colors.Transparent);
        clear.Foreground = Theme.Brush("color.content-faint");
        if (clear.Content is Icon glyph) glyph.GlyphSize = Theme.Number("icon.small");
    }
}
