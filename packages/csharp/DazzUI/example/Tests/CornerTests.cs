using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class CornerTests
{
    public static async Task RunAsync(Shell shell, string output)
    {
        var single = new UI.TextField { Text = "Single-line input" };
        var multiline = new UI.TextField { AcceptsReturn = true, Text = "Multiline input\nContainer corners stay finite" };
        var option = new UI.OptionCard { Title = "Option card", Description = "A container, not a pill", IsChecked = true };
        var small = new UI.Callout { Size = UI.ControlSize.Small, Description = "Small callout" };
        var medium = new UI.Callout { Title = "Medium callout", Description = "Container corner" };
        var large = new UI.Callout { Size = UI.ControlSize.Large, Title = "Large callout", Description = "Container corner" };
        var skeleton = new UI.Skeleton { Shape = UI.SkeletonShape.Text };
        var card = new UI.Card { Content = new TextBlock { Text = "Card" } };
        var badge = new UI.Badge("Badge");
        var button = new UI.Button { Content = "Primary action" };
        var segments = new UI.SegmentedControl { Size = UI.ControlSize.Medium }; segments.Items.Add("One"); segments.Items.Add("Two"); segments.SelectedIndex = 0;
        var body = new StackPanel { Width = 420, Spacing = 8, Children = { button, single, multiline, option, small, medium, large, skeleton, card, badge, segments } };
        var scope = new UI.ThemeScope { Content = body, HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
        shell.ShowFixture(scope);
        foreach (var name in UI.Theme.Names)
        {
            var theme = new UI.Theme(name); scope.Theme = theme;
            if (theme.Number("control.small.radius") != theme.Number("radius.small"))
                throw new InvalidOperationException($"{name}: generated control radius bypassed its semantic alias.");
            await Task.Delay(80); shell.UpdateLayout(); await new RenderTargetBitmap().RenderAsync(shell);
            foreach (var (control, token) in new (Control, string)[] {
                (button, "radius.small"), (single, "control.field.radius"), (multiline, "radius.large"), (option, "radius.large"),
                (small, "radius.medium"), (medium, "radius.large"), (large, "radius.large"),
                (skeleton, "radius.tiny"), (card, "radius.large"), (badge, "radius.tiny") })
            {
                var expected = new CornerRadius(Math.Min(theme.Number(token), Math.Min(control.ActualWidth, control.ActualHeight) / 2));
                if (control.CornerRadius != expected)
                    throw new InvalidOperationException($"{name}: {control.GetType().Name} must use {token} ({expected}), got {control.CornerRadius}.");
                var border = Descendants(control).OfType<Border>().FirstOrDefault();
                if (control is UI.Surface && border is not null && border.CornerRadius != expected)
                    throw new InvalidOperationException($"{name}: {control.GetType().Name} template did not receive its corner radius.");
            }
            var item = (ListBoxItem)segments.ContainerFromIndex(0);
            var segmentBorder = Descendants(segments).OfType<Border>().First(border => border.Name == "LayoutRoot");
            if (segmentBorder.CornerRadius != segments.CornerRadius) throw new InvalidOperationException($"{name}: segmented background ignored the control corner.");
            var inner = Math.Max(0, theme.Number("control.medium.radius") - theme.Number("segmented-control.inset"));
            if (item.CornerRadius != new CornerRadius(Math.Min(inner, Math.Min(item.ActualWidth, item.ActualHeight) / 2))) throw new InvalidOperationException($"{name}: segmented inner corner mismatch.");
            if (name.StartsWith("bright-")) await SmokeTest.SaveAsync(shell, output, name + "-corners.png");
            if (name.StartsWith("bright-"))
            {
                foreach (var height in new[] { 60d, 28d })
                {
                    single.Height = height;
                    await Task.Delay(40); shell.UpdateLayout(); await new RenderTargetBitmap().RenderAsync(shell);
                    if (single.CornerRadius.TopLeft != height / 2) throw new InvalidOperationException("Pill corners did not follow a resize.");
                }
            }
        }
        File.WriteAllText(Path.Combine(output, "corners.txt"), "PASS: control/container corner roles and rendered template borders in all twelve themes, including live theme changes.");
    }
    private static IEnumerable<DependencyObject> Descendants(DependencyObject root)
    {
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
        {
            var child = VisualTreeHelper.GetChild(root, i); yield return child;
            foreach (var nested in Descendants(child)) yield return nested;
        }
    }
}
