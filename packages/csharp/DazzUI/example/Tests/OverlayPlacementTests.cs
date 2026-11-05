using System.Text.Json;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;

namespace DazzUI.Example;

/// <summary>Native placement must align the painted popup, not just its declared mode.</summary>
internal static class OverlayPlacementTests
{
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }
    private static Rect Bounds(FrameworkElement element) => element.TransformToVisual(element.XamlRoot.Content)
        .TransformBounds(new Rect(0, 0, element.ActualWidth, element.ActualHeight));
    private static async Task Settle(Shell shell)
    {
        await Task.Delay(140);
        shell.UpdateLayout();
    }
    private static T Presenter<T>(Shell shell) where T : FrameworkElement => VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
        .SelectMany(popup => OverlaySkin.Walk(popup.Child)).OfType<T>().Single();
    private static void Record(string output, string name, FrameworkElement trigger, FrameworkElement popup)
    {
        static object Box(Rect rect) => new { rect.X, rect.Y, rect.Width, rect.Height };
        File.AppendAllText(Path.Combine(output, "overlay-placement.jsonl"), JsonSerializer.Serialize(new
        {
            name, trigger = Box(Bounds(trigger)), popup = Box(Bounds(popup)),
            root = new { trigger.XamlRoot.Size.Width, trigger.XamlRoot.Size.Height }, scale = trigger.XamlRoot.RasterizationScale,
        }) + Environment.NewLine);
    }
    private static void AssertBelow(string name, FrameworkElement trigger, FrameworkElement popup, PopupAlignment align)
    {
        var anchor = Bounds(trigger);
        var panel = Bounds(popup);
        var gap = Appearance.GetTheme(trigger).Number("spacing.1");
        var offset = align switch
        {
            PopupAlignment.Start => panel.Left - anchor.Left,
            PopupAlignment.End => panel.Right - anchor.Right,
            _ => (panel.Left + panel.Width / 2) - (anchor.Left + anchor.Width / 2),
        };
        Check(Math.Abs(panel.Top - anchor.Bottom - gap) <= 1 && Math.Abs(offset) <= 1,
            $"{name}: native popup did not resolve below/{align} with spacing.1 (anchor={anchor}, popup={panel}, actualGap={panel.Top - anchor.Bottom}, expectedGap={gap}, alignmentOffset={offset}).");
    }

    internal static async Task RunAsync(Shell shell, string output)
    {
        var trigger = new Button { Content = "Placement", Width = 100, HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
        var scope = new ThemeScope { Theme = new Theme("bright-light"), Content = new Grid { Width = 560, Height = 400, Children = { trigger } } };
        shell.ShowFixture(scope);
        await Settle(shell);
        await NativeKeyboard.ActivateAsync(shell);
        File.WriteAllText(Path.Combine(output, "overlay-placement.jsonl"), "");

        var menu = new Menu();
        menu.Items.Add(new("First action")); menu.Items.Add(new("Second action"));
        Check(menu.Side == PopupSide.Bottom && menu.Align == PopupAlignment.End, "Menu defaults must be bottom/end.");
        foreach (var align in new[] { PopupAlignment.End, PopupAlignment.Start, PopupAlignment.Center })
        {
            menu.Align = align;
            menu.Show(trigger); await Settle(shell);
            try
            {
                var panel = Presenter<MenuFlyoutPresenter>(shell);
                Record(output, $"menu-{align}", trigger, panel);
                AssertBelow($"Menu/{align}", trigger, panel, align);
            }
            finally { menu.Flyout.Hide(); await Settle(shell); }
        }

        var popover = new Popover { Content = scope.Theme.Text("Anchored content", "bodySmall"), Width = 240 };
        Check(popover.Side == PopupSide.Bottom && popover.Align == PopupAlignment.Center, "Popover defaults must be bottom/center.");
        popover.Show(trigger); await Settle(shell);
        try
        {
            var panel = Presenter<FlyoutPresenter>(shell);
            Record(output, "popover-default", trigger, panel);
            AssertBelow("Popover", trigger, panel, PopupAlignment.Center);
        }
        finally { popover.Hide(); await Settle(shell); }

        var preview = new PreviewCard { Content = scope.Theme.Text("Preview content", "bodySmall"), Width = 240 };
        Check(preview.Side == PopupSide.Bottom && preview.Align == PopupAlignment.Center, "PreviewCard defaults must be bottom/center.");
        preview.Show(trigger); await Settle(shell);
        try
        {
            var panel = Presenter<FlyoutPresenter>(shell);
            Record(output, "preview-default", trigger, panel);
            AssertBelow("PreviewCard", trigger, panel, PopupAlignment.Center);
        }
        finally { preview.Hide(); await Settle(shell); }

        foreach (var align in new[] { PopupAlignment.Center, PopupAlignment.Start, PopupAlignment.End })
        {
            Tooltip.Set(trigger, "Placement hint", PlacementMode.Top, align);
            var hint = (ToolTip)ToolTipService.GetToolTip(trigger);
            hint.IsOpen = true; await Settle(shell);
            try
            {
                Record(output, $"tooltip-{align}", trigger, hint);
                var anchor = Bounds(trigger); var panel = Bounds(hint);
                var offset = align == PopupAlignment.Start ? panel.Left - anchor.Left : align == PopupAlignment.End
                    ? panel.Right - anchor.Right : panel.Left + panel.Width / 2 - anchor.Left - anchor.Width / 2;
                Check(Math.Abs(anchor.Top - panel.Bottom - scope.Theme.Number("spacing.1")) <= 1 && Math.Abs(offset) <= 1,
                    $"Tooltip/{align}: native hint lost top alignment/gap (anchor={anchor}, popup={panel}, offset={offset}).");
            }
            finally { hint.IsOpen = false; await Settle(shell); }
        }

        // Move the same live anchor next to the root's lower/right edges. No
        // popup coordinates are supplied: native collision handling must flip
        // and constrain the preferred bottom-centered placement itself.
        var original = Bounds(trigger);
        trigger.RenderTransform = new TranslateTransform
        {
            X = shell.XamlRoot.Size.Width - original.Right - 12,
            Y = shell.XamlRoot.Size.Height - original.Bottom - 12,
        };
        await Settle(shell);
        popover.Show(trigger); await Settle(shell);
        try
        {
            var panel = Presenter<FlyoutPresenter>(shell);
            Record(output, "popover-bottom-right-collision", trigger, panel);
            var anchor = Bounds(trigger); var box = Bounds(panel);
            Check(box.Bottom <= anchor.Top - scope.Theme.Number("spacing.1") + 1
                && box.Left >= -1 && box.Top >= -1 && box.Right <= shell.XamlRoot.Size.Width + 1,
                $"Native collision handling failed to flip or constrain a bottom-right popover (anchor={anchor}, popup={box}, root={shell.XamlRoot.Size}).");
        }
        finally { popover.Hide(); await Settle(shell); trigger.RenderTransform = null; }
    }
}
