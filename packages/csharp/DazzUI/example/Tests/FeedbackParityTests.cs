using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI.Example;

/// <summary>Real native animation values through resizing, reload and accessibility changes.</summary>
internal static class FeedbackParityTests
{
    internal static async Task RunAsync(Shell shell)
    {
        static void Check(bool pass, string message)
        {
            if (!pass) throw new InvalidOperationException(message);
        }
        static Border Fill(Progress progress) => ((Grid)progress.Content).Children.OfType<Border>().Single();
        static TranslateTransform Sweep(Progress progress) => (TranslateTransform)Fill(progress).RenderTransform;
        static TranslateTransform Glint(Skeleton skeleton) => (TranslateTransform)((LinearGradientBrush)((Border)skeleton.Content).Background).RelativeTransform;

        try
        {
            FeedbackMotion.SetEnabledForTest(true);
            var progress = new Progress { Width = 200 };
            var measured = new Progress { Width = 200, Value = 20 };
            var skeleton = new Skeleton { Width = 200 };
            var spinner = new Spinner();
            var meter = new Meter { Width = 200, Value = 20, Label = "Quota", ShowValue = true };
            var paragraph = new SkeletonText { Width = 200, Lines = 3 };
            var panel = new StackPanel { Spacing = 16, HorizontalAlignment = HorizontalAlignment.Left, Children = { progress, measured, skeleton, spinner, meter, paragraph } };
            var scope = new ThemeScope { Theme = new Theme("studio-light"), Content = panel };
            shell.ShowFixture(scope);
            await Task.Delay(120); shell.UpdateLayout();
            Check(Math.Abs(Fill(progress).ActualWidth - 80) < .1, "Indeterminate progress must occupy forty percent of its track.");
            var initial = Sweep(progress).X;
            await Task.Delay(200);
            Check(Math.Abs(Sweep(progress).X - initial) > 1, "An indeterminate progress bar must sweep while mounted.");

            // Resize late in the sweep, when stale endpoints are distinguishable
            // from the relative transform React preserves across layout changes.
            await Task.Delay(480);
            var phaseBefore = Sweep(progress).X / progress.ActualWidth;
            progress.Width = 400;
            shell.UpdateLayout();
            var phaseAfter = Sweep(progress).X / progress.ActualWidth;
            Check(Math.Abs(Fill(progress).ActualWidth - 160) < .1 && Math.Abs(phaseAfter - phaseBefore) < .18,
                $"Resizing progress must update its fill and preserve sweep phase ({phaseBefore:0.###} -> {phaseAfter:0.###}).");
            panel.Children.Remove(progress);
            await Task.Delay(50);
            Check(Sweep(progress).X == 0, "Unmounting progress must stop its animation.");
            panel.Children.Insert(0, progress);
            await Task.Delay(100); shell.UpdateLayout();
            Check(Sweep(progress).X < 0, "Reloading progress at the same width must start a fresh sweep.");

            measured.Value = 80;
            await Task.Delay(60); shell.UpdateLayout();
            Check(Fill(measured).ActualWidth > 40 && Fill(measured).ActualWidth < 160,
                "Determinate progress must interpolate rather than jump to the new reading.");
            await Task.Delay((int)scope.Theme.Milliseconds("motion.duration") * 2 + 80); shell.UpdateLayout();
            Check(Math.Abs(Fill(measured).ActualWidth - 160) < .1, "Progress must reach its new width after twice the shared motion duration.");

            meter.Value = 80;
            shell.UpdateLayout();
            var meterBody = (StackPanel)meter.Content;
            var meterHead = (Grid)meterBody.Children[0];
            var meterTrack = (Grid)meterBody.Children[1];
            Check(meterHead.Children.OfType<TextBlock>().Any(text => text.Text == "80%") && Math.Abs(meterTrack.Children.OfType<Border>().Single().ActualWidth - 160) < .1,
                "A meter must update its printed reading and fill immediately, without progress animation.");

            var brush = (LinearGradientBrush)((Border)skeleton.Content).Background;
            Check(brush.StartPoint.X == 0 && brush.EndPoint.X == 2 && brush.GradientStops[1].Color == scope.Theme.Color("color.surface-subtle"),
                "Skeleton shimmer must paint a double-width gradient using the shared highlight.");
            var shimmerBefore = Glint(skeleton).X;
            await Task.Delay(180);
            Check(Math.Abs(Glint(skeleton).X - shimmerBefore) > .01, "Skeleton highlight must move while animations are enabled.");
            var paragraphGrid = (Grid)paragraph.Content;
            var bars = paragraphGrid.Children.OfType<Skeleton>().ToArray();
            Check(Math.Abs(bars[^1].ActualWidth / bars[0].ActualWidth - scope.Theme.Number("skeleton.last-line-width")) < .01,
                "The paragraph's last bar must use the shared width proportion.");

            scope.Theme = new Theme("studio-dark");
            await Task.Delay(100); shell.UpdateLayout();
            Check(brush.GradientStops[1].Color == scope.Theme.Color("color.surface-subtle"), "A live theme change must recolor an existing shimmer.");

            await Task.Run(() => FeedbackMotion.SetEnabledForTest(false));
            await Task.Delay(100); shell.UpdateLayout();
            Check(Sweep(progress).X == 0 && Glint(skeleton).X == 0, "Reduced motion must stop decorative progress and skeleton sweeps.");
            measured.Value = 30;
            shell.UpdateLayout();
            Check(Math.Abs(Fill(measured).ActualWidth - 60) < .1, "Reduced motion must apply determinate progress without a transition.");
            var rotation = (RotateTransform)((Grid)spinner.Content).RenderTransform;
            var angle = rotation.Angle;
            await Task.Delay(200);
            var travel = (rotation.Angle - angle + 360) % 360;
            Check(travel > 10 && travel < 60, $"Reduced motion must retain the slower spinner turn (travel {travel:0.##} degrees).");

            FeedbackMotion.SetEnabledForTest(true);
            await Task.Delay(120);
            Check(Sweep(progress).X < 0 && Glint(skeleton).X < 0, "Re-enabling animation must resume mounted feedback.");
            panel.Children.Remove(skeleton);
            await Task.Delay(50);
            Check(Glint(skeleton).X == 0, "Unmounting a skeleton must release its animation.");
            panel.Children.Add(skeleton);
            await Task.Delay(120);
            Check(Glint(skeleton).X < 0, "Reloading a skeleton must restart its shimmer.");
        }
        finally { FeedbackMotion.SetEnabledForTest(null); }
    }
}
