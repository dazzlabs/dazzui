using System.Runtime.InteropServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class ToastTests
{
    private static void Check(bool pass, string message)
    {
        if (!pass) throw new InvalidOperationException(message);
    }
    private static void Invoke(UI.Button button) => ((IInvokeProvider)new ButtonAutomationPeer(button).GetPattern(PatternInterface.Invoke)).Invoke();
    private static UI.Button Dismiss(UI.Toast toast) => ((Grid)toast.Content).Children.OfType<UI.Button>()
        .Single(button => AutomationProperties.GetName(button) == toast.Options.DismissLabel);
    private static async Task Settle(Shell shell, int milliseconds = 80) { await Task.Delay(milliseconds); shell.UpdateLayout(); }

    public static async Task RunAsync(Shell shell)
    {
        using var pointer = new NativePointer();
        var viewport = new UI.ToastViewport();
        var outside = new UI.Button { Content = "Outside notifications", HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Top };
        var scope = new UI.ThemeScope { Content = new Grid { Width = 560, Height = 320, Children = { outside, viewport } }, HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Top };
        var history = new Queue<string>();
        var elapsed = System.Diagnostics.Stopwatch.StartNew();
        void Record(string reason)
        {
            history.Enqueue($"{elapsed.Elapsed.TotalMilliseconds:F0}ms {reason}: {viewport.TimingState}");
            while (history.Count > 25) history.Dequeue();
        }
        string Timing()
        {
            GetCursorPos(out var cursor);
            return $"\n{elapsed.Elapsed.TotalMilliseconds:F0}ms now: {viewport.TimingState}; cursor=({cursor.X},{cursor.Y}), foreground=0x{GetForegroundWindow().ToInt64():X}\n" + string.Join("\n", history);
        }
        viewport.GotFocus += (_, _) => Record("viewport GotFocus");
        viewport.LostFocus += (_, _) => Record("viewport LostFocus");
        viewport.PointerEntered += (_, _) => Record("viewport PointerEntered");
        viewport.PointerExited += (_, _) => Record("viewport PointerExited");
        outside.GotFocus += (_, _) => Record("outside GotFocus");
        shell.ShowFixture(scope);
        await Settle(shell);
        await NativeKeyboard.ActivateAsync(shell);
        pointer.MoveTo(outside); outside.Focus(FocusState.Keyboard);
        UI.FeedbackMotion.SetEnabledForTest(false);
        try
        {
            foreach (var name in new[] { "studio-light", "bright-dark", "graphite-light" })
            {
                var theme = scope.Theme = new UI.Theme(name);
                var closed = 0; var invoked = 0;
                var toast = viewport.Add(new UI.ToastOptions(new string('W', 200), Tint: UI.WidgetTint.Success,
                    ActionLabel: "Undo", Action: () => invoked++, Duration: TimeSpan.Zero, Leading: FluentIcons.Common.Icon.Checkmark, OnClose: () => closed++));
                await Settle(shell);
                var row = (Grid)toast.Content;
                var message = row.Children.OfType<TextBlock>().Single();
                var dismiss = Dismiss(toast);
                var action = row.Children.OfType<UI.Button>().Single(button => !ReferenceEquals(button, dismiss));
                Check(message.TextWrapping == TextWrapping.NoWrap && message.TextTrimming == TextTrimming.CharacterEllipsis, $"{name}: toast message is not one ellipsized line");
                Check(Math.Abs(message.LineHeight - message.FontSize * theme.Number("toast.message-line-height")) < .01 && message.Padding.Top == theme.Number("spacing.0.5"), $"{name}: wrong message line height or inset");
                Check(toast.ActualWidth <= theme.Number("toast.max-width") + .1 && message.ActualWidth > 0 && action.ActualWidth > 0 && dismiss.ActualWidth > 0, $"{name}: long title crowded out toast actions");
                Check(action.Size == UI.ControlSize.Small && action.Tint == UI.WidgetTint.Success && action.Variant == UI.ControlVariant.Plain, $"{name}: wrong toast action profile");
                Check(dismiss.ActualWidth == theme.Number("control.tiny.size") && dismiss.BorderThickness.Left == 0 && dismiss.Content is Viewbox box && box.Width == theme.Number("icon.small"), $"{name}: wrong dismiss glyph or hit target");
                Check(UI.Elevation.GetShadow(toast) == "md" && row.ColumnSpacing == theme.Number("spacing.2.5"), $"{name}: missing shadow or content gap");
                Check(((SolidColorBrush)row.Children.OfType<UI.Icon>().Single().Foreground).Color == theme.Color("color.success.600"), $"{name}: leading icon is not tint-600");
                Check(((Grid)viewport.Content).RowSpacing == theme.Number("spacing.2") && viewport.Padding == new Thickness(theme.Number("spacing.4")), $"{name}: viewport gap or inset differs");
                Check(action.Focus(FocusState.Keyboard), $"{name}: native toast action refused focus");
                scope.Theme = new UI.Theme(name.EndsWith("-dark") ? "studio-light" : "studio-dark");
                await Settle(shell);
                Check(ReferenceEquals(toast.Content, row) && ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), action) && row.Children.Contains(dismiss), $"{name}: theme update rebuilt native actions or lost focus");
                Invoke(action); await Settle(shell);
                Check(invoked == 1 && closed == 1 && !viewport.Toasts.Contains(toast), $"{name}: action did not invoke and close once");
                viewport.Remove(toast); Check(closed == 1, "Repeated close invoked onClose again");
                outside.Focus(FocusState.Keyboard);
            }
            var persistent = viewport.Add(new UI.ToastOptions("Failed", Tint: UI.WidgetTint.Danger, DismissLabel: "Close notice"));
            await Settle(shell);
            Check(AutomationProperties.GetLiveSetting(persistent) == AutomationLiveSetting.Assertive, "Danger toast lost its high-priority announcement");
            Invoke(Dismiss(persistent)); await Settle(shell);
            Check(!viewport.Toasts.Contains(persistent), "Localized native dismiss button did not close its toast");
            var hiddenDismiss = viewport.Add(new UI.ToastOptions("No close", Duration: TimeSpan.Zero, Dismissible: false));
            await Settle(shell);
            Check(!((Grid)hiddenDismiss.Content).Children.OfType<UI.Button>().Any(), "Explicit dismissable=false was ignored");
            viewport.Remove(hiddenDismiss);

            var first = viewport.Add(new UI.ToastOptions("First", Duration: TimeSpan.Zero));
            await Settle(shell);
            Check(Dismiss(first).Focus(FocusState.Keyboard), "Oldest toast action refused initial focus");
            var second = viewport.Add(new UI.ToastOptions("Second", Duration: TimeSpan.Zero));
            var third = viewport.Add(new UI.ToastOptions("Third", Duration: TimeSpan.Zero));
            var fourth = viewport.Add(new UI.ToastOptions("Fourth", Duration: TimeSpan.Zero));
            await Settle(shell);
            var stack = (Grid)viewport.Content;
            Check(stack.Children.SequenceEqual(new UIElement[] { fourth, third, second, first }), "Bottom store is not newest-first");
            Check(!first.IsHitTestVisible && !first.IsTabStop && first.ActualHeight > 0 && second.IsHitTestVisible, "Queue limit did not make only the oldest toast inert while retaining its drawing");
            Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), fourth), "Queue limit retained keyboard focus on an inert toast");
            Invoke(Dismiss(first));
            Check(viewport.Toasts.Contains(first), "An inert toast still accepted activation");
            var gap = second.TransformToVisual(first).TransformPoint(new Point()).Y + second.ActualHeight;
            Check(Math.Abs(gap + viewport.Theme.Number("spacing.2")) < .5, $"Actual toast gap differs: {gap}");
            viewport.AtTop = true; await Settle(shell);
            Check(stack.Children.SequenceEqual(new UIElement[] { fourth, third, second, first })
                && first.TransformToVisual(fourth).TransformPoint(new Point()).Y < 0
                && ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), fourth), "Top viewport changed source/Tab order or lost focus while reversing its layout");
            Invoke(Dismiss(fourth)); await Settle(shell);
            Check(first.IsHitTestVisible && first.IsTabStop, "Closing a newer toast did not release the queue limit");
            viewport.Close(); viewport.AtTop = false;
            var replacementClosed = 0;
            var replace = viewport.Add(new UI.ToastOptions("Uploading", Id: "upload", Duration: TimeSpan.Zero));
            await Settle(shell);
            var originalDismiss = Dismiss(replace);
            Check(originalDismiss.Focus(FocusState.Keyboard), "Toast dismiss refused focus before replacement");
            var same = viewport.Add(new UI.ToastOptions("Uploaded", Id: "upload", Tint: UI.WidgetTint.Success, ActionLabel: "Undo", Duration: TimeSpan.Zero, OnClose: () => replacementClosed++));
            await Settle(shell);
            Check(ReferenceEquals(replace, same) && viewport.Toasts.Count == 1 && ((Grid)same.Content).Children.OfType<TextBlock>().Single().Text == "Uploaded", "Same ID did not update the existing native row");
            Check(ReferenceEquals(Dismiss(same), originalDismiss) && ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), originalDismiss), "Adding a slot in an ID update rebuilt the focused dismiss action");
            viewport.Close("upload"); Check(replacementClosed == 1 && viewport.Toasts.Count == 0, "Closing an ID did not close the replacement once");

            // Verify real elapsed default lifetimes, not just timer properties.
            outside.Focus(FocusState.Keyboard); pointer.MoveTo(outside);
            Record("before defaults");
            var notice = viewport.Add(new UI.ToastOptions("Default notice", OnClose: () => Record("default notice closed")));
            var actionNotice = viewport.Add(new UI.ToastOptions("Default action", ActionLabel: "Undo", OnClose: () => Record("default action closed")));
            await Settle(shell);
            Record("defaults settled");
            Check(Dismiss(notice).IsLoaded && Dismiss(actionNotice).IsLoaded, "Timed notices lack their default dismiss buttons");
            await Task.Delay(4200);
            Check(viewport.Toasts.Contains(notice) && viewport.Toasts.Contains(actionNotice), $"Default notice expired at the obsolete 4s lifetime; noticePresent={viewport.Toasts.Contains(notice)}, actionPresent={viewport.Toasts.Contains(actionNotice)}" + Timing());
            await Task.Delay(1200);
            Check(!viewport.Toasts.Contains(notice) && viewport.Toasts.Contains(actionNotice), $"Notice/action lifetimes did not separate at 5s; noticePresent={viewport.Toasts.Contains(notice)}, actionPresent={viewport.Toasts.Contains(actionNotice)}" + Timing());
            await Task.Delay(1200);
            Check(viewport.Toasts.Contains(actionNotice), "Action toast expired at the obsolete 6s lifetime" + Timing());
            await Task.Delay(1800);
            Check(!viewport.Toasts.Contains(actionNotice), "Action toast did not expire at 8s" + Timing());
            var paused = viewport.Add(new UI.ToastOptions("Hover keeps remaining time", Duration: TimeSpan.FromMilliseconds(1200)));
            var entered = new TaskCompletionSource(TaskCreationOptions.RunContinuationsAsynchronously);
            var exited = new TaskCompletionSource(TaskCreationOptions.RunContinuationsAsynchronously);
            void Entered(object sender, PointerRoutedEventArgs args) => entered.TrySetResult();
            void Exited(object sender, PointerRoutedEventArgs args) => exited.TrySetResult();
            viewport.PointerEntered += Entered;
            viewport.PointerExited += Exited;
            try
            {
                await Settle(shell, 400);
                pointer.MoveTo(paused);
                await entered.Task.WaitAsync(TimeSpan.FromSeconds(1));
                Record("actual hover entered");
                await Task.Delay(1300); Check(viewport.Toasts.Contains(paused), "Hovered toast expired" + Timing());
                pointer.MoveTo(outside);
                await exited.Task.WaitAsync(TimeSpan.FromSeconds(1));
                Record("actual hover exited");
                await Task.Delay(450); Check(viewport.Toasts.Contains(paused), "Hover resume lost the remaining lifetime" + Timing());
                await Task.Delay(550); Check(!viewport.Toasts.Contains(paused), "Hover resume restarted the full lifetime" + Timing());
            }
            catch (TimeoutException error) { throw new InvalidOperationException("Native mouse input did not reach the expected toast viewport boundary" + Timing(), error); }
            finally { viewport.PointerEntered -= Entered; viewport.PointerExited -= Exited; }
            var keyboard = viewport.Add(new UI.ToastOptions("Keyboard pause", Duration: TimeSpan.FromMilliseconds(600)));
            await Settle(shell); outside.Focus(FocusState.Keyboard); NativeKeyboard.Press(shell, 0x75);
            await Settle(shell);
            Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), viewport), "F6 did not focus the actual viewport");
            await Task.Delay(700); Check(viewport.Toasts.Contains(keyboard), "Keyboard-focused viewport did not pause timers");
            NativeKeyboard.Press(shell, 0x09); await Settle(shell);
            Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), keyboard), "Tab did not reach the first native toast");
            NativeKeyboard.Press(shell, 0x1B); await Settle(shell);
            Check(!viewport.Toasts.Contains(keyboard) && ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), outside), "Escape did not close and restore the F6 origin");

            var blurred = viewport.Add(new UI.ToastOptions("Inactive window pauses", Duration: TimeSpan.FromMilliseconds(900)));
            await Settle(shell, 150);
            var otherWindow = new Window { Title = "Toast activation fixture", Content = new Grid() };
            try
            {
                otherWindow.Activate();
                await Task.Delay(1100);
                Check(viewport.Toasts.Contains(blurred), "A deactivated window did not pause the toast lifetime");
            }
            finally
            {
                otherWindow.Close();
                await NativeKeyboard.ActivateAsync(shell);
                outside.Focus(FocusState.Keyboard); pointer.MoveTo(outside);
            }
            await Task.Delay(950);
            Check(!viewport.Toasts.Contains(blurred), "Reactivating the window did not resume the remaining toast lifetime");

            await ProviderAsync(shell);
            shell.ShowFixture(scope); await Settle(shell);
            UI.FeedbackMotion.SetEnabledForTest(true);
            var animated = viewport.Add(new UI.ToastOptions("Animated receipt", Duration: TimeSpan.Zero));
            await Settle(shell, 350); Check(animated.Opacity == 1 && animated.IsLoaded, "Toast entrance did not settle");
            viewport.Remove(animated); await Settle(shell, 350);
            Check(!((Grid)viewport.Content).Children.Contains(animated), "Exit transition did not remove the toast");
        }
        finally { UI.FeedbackMotion.SetEnabledForTest(null); viewport.Close(); }
    }

    private static async Task ProviderAsync(Shell shell)
    {
        var scene = new SceneView(Stories.Find("toast")!.Preview, align: true);
        shell.ShowFixture(scene); await Settle(shell);
        var actual = Overlay.Layer.Children.OfType<UI.ToastViewport>().Single(viewport => Overlay.BelongsTo(viewport, scene));
        Check(Math.Abs(actual.ActualWidth - Overlay.Layer.ActualWidth) < .5 && actual.ActualWidth > 640, $"Empty viewport is not window-wide: actual={actual.ActualWidth}, window={Overlay.Layer.ActualWidth}");
        Check(Math.Abs(actual.ActualHeight - actual.Theme.Number("spacing.4") * 2) < .5, $"Empty viewport lost its padded box: {actual.ActualHeight}");
        var reading = Measure.Read(actual, new HashSet<DependencyObject> { actual });
        Check(reading["values"]!["paddingX"]!.GetValue<double>() == actual.Theme.Number("spacing.4") && reading["values"]!["gap"]!.GetValue<double>() == actual.Theme.Number("spacing.2"), "Conformance could not measure the actual empty viewport");
        Invoke(Walk(scene).OfType<UI.Button>().Single()); await Settle(shell);
        Check(actual.Toasts.Count == 1, "Generated action did not use its own provider viewport");
        shell.ShowFixture(new SceneView(Stories.Find("toast")!.Examples[2].Demo, align: true)); await Settle(shell);
        Check(!Overlay.Layer.Children.Contains(actual) && actual.Toasts.Count == 0, "Unmounting a provider leaked its viewport or notifications");
        var top = Overlay.Layer.Children.OfType<UI.ToastViewport>().Single();
        Check(top.AtTop && top.Toasts.Count == 0, "New top provider inherited another provider's queue");
    }
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }
    [StructLayout(LayoutKind.Sequential)] private struct NativePoint { public int X; public int Y; }
    [DllImport("user32.dll")][return: MarshalAs(UnmanagedType.Bool)] private static extern bool GetCursorPos(out NativePoint point);
    [DllImport("user32.dll")] private static extern IntPtr GetForegroundWindow();
}
