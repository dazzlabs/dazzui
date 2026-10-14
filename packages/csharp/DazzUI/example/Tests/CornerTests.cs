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
        var circle = new UI.Skeleton { Shape = UI.SkeletonShape.Circle, Width = 32, Height = 80 };
        var rectangle = new UI.Skeleton { Shape = UI.SkeletonShape.Rect, Width = 64, Height = 40 };
        var link = new UI.Link { Content = "Read more" };
        var card = new UI.Card { Content = new TextBlock { Text = "Card" } };
        var badge = new UI.Badge("Badge");
        var hint = new UI.KeyCap("Ctrl+K");
        var tag = new UI.Tag { Text = "Status" };
        var button = new UI.Button { Content = "Primary action" };
        var segments = new UI.SegmentedControl { Size = UI.ControlSize.Medium }; segments.Items.Add("One"); segments.Items.Add("Two"); segments.SelectedIndex = 0;
        var disclosure = new UI.Collapsible { Header = "Disclosure" };
        var meter = new UI.Meter { Value = 50 };
        var progress = new UI.Progress { Value = 50, Size = UI.ControlSize.Medium };
        var spinner = new UI.Spinner();
        var number = new UI.NumberField { Value = 42, Unit = "px", Width = 160 };
        var choice = new UI.Select { Width = 160 };
        choice.SetOptions([new UI.SelectOption("first", "First")]); choice.Value = "first";
        var preference = new UI.PreferenceRow { Label = "Open settings" };
        preference.Opened += (_, _) => { };
        var fraction = new Grid { Width = 160, HorizontalAlignment = HorizontalAlignment.Left,
            ColumnDefinitions = { new ColumnDefinition { Width = new GridLength(4, GridUnitType.Star) }, new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) } } };
        var fractionBone = new UI.Skeleton(); fraction.Children.Add(fractionBone);
        var body = new StackPanel { Width = 420, Spacing = 8, Children = { button, single, multiline, option, small, medium, large, skeleton, circle, rectangle, link, card, badge, hint, tag, segments, disclosure, meter, progress, spinner, number, choice, preference, fraction } };
        var scope = new UI.ThemeScope { Content = body, HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
        shell.ShowFixture(scope);
        foreach (var name in UI.Theme.Names)
        {
            var theme = new UI.Theme(name); scope.Theme = theme;
            if (theme.Number("control.small.radius") != theme.Number("radius.small"))
                throw new InvalidOperationException($"{name}: generated control radius bypassed its semantic alias.");
            await Task.Delay(80); shell.UpdateLayout(); await new RenderTargetBitmap().RenderAsync(shell).AsTask().WaitAsync(TimeSpan.FromSeconds(30));
            foreach (var (control, token) in new (Control, string)[] {
                (button, "radius.small"), (single, "control.field.radius"), (multiline, "radius.large"), (option, "radius.large"),
                (small, "control.container.radius"), (medium, "radius.large"), (large, "radius.large"),
                (skeleton, "radius.tiny"), (circle, "radius.full"), (rectangle, "radius.small"), (link, "radius.tiny"), (card, "radius.large"), (badge, "radius.tiny") })
            {
                var expected = new CornerRadius(Math.Min(theme.Number(token), Math.Min(control.ActualWidth, control.ActualHeight) / 2));
                if (control.CornerRadius != expected)
                    throw new InvalidOperationException($"{name}: {control.GetType().Name} must use {token} ({expected}), got {control.CornerRadius}.");
                var border = Descendants(control).OfType<Border>().FirstOrDefault();
                if (control is UI.Surface && border is not null && border.CornerRadius != expected)
                    throw new InvalidOperationException($"{name}: {control.GetType().Name} template did not receive its corner radius.");
            }
            var item = segments.Segments[0];
            if (circle.ActualHeight != circle.ActualWidth) throw new InvalidOperationException($"{name}: a circle skeleton must derive its height from width.");
            var track = Math.Min(theme.Number("radius.medium"), segments.ActualHeight / 2);
            if (segments.CornerRadius != new CornerRadius(track)) throw new InvalidOperationException($"{name}: segmented track ignored the container corner.");
            var inner = Math.Max(0, track - theme.Number("segmented-control.inset"));
            if (item.CornerRadius != new CornerRadius(inner)) throw new InvalidOperationException($"{name}: segmented inner corner mismatch.");
            double Read(FrameworkElement element, string property) => Measure.Read(element, new HashSet<DependencyObject>())["values"]![property]!.GetValue<double>();
            bool Near(double actual, double expected) => Math.Abs(actual - expected) < .005;
            if (!Near(Read(single, "paddingX"), single.Padding.Left))
                throw new InvalidOperationException($"{name}: native editor viewport lost its allocated inset: {Measure.Read(single, new HashSet<DependencyObject>())}.");
            if (!Near(Read(choice, "paddingX"), theme.Number("spacing.3")))
                throw new InvalidOperationException($"{name}: select lost the shared field inset: {Measure.Read(choice, new HashSet<DependencyObject>())}.");
            if (!Near(Read(number, "fontSize"), number.Input.FontSize) || !Near(Read(number, "paddingX"), theme.Number("spacing.3")) || !Near(Read(number, "paddingY"), theme.Number("control.medium.padding.block")) || !Near(Read(number, "gap"), theme.Number("spacing.1")))
                throw new InvalidOperationException($"{name}: number field measured its unit or lost its shared field insets: {Measure.Read(number, new HashSet<DependencyObject>())}.");
            var outline = Descendants(option).OfType<Border>().First(border => border.Name == "SelectionOutline");
            if (Read(option, "stroke") != theme.Number("stroke.hairline") || Read(option, "paddingX") != theme.Number("spacing.3") || Read(option, "paddingY") != theme.Number("spacing.3.5") || Read(option, "gap") != theme.Number("spacing.1.5") || outline.Visibility != Visibility.Visible || outline.BorderThickness.Top != theme.Number("stroke.control"))
                throw new InvalidOperationException($"{name}: selected option card must add its painted edge without changing its layout border.");
            if (Read(hint, "corner") != 0 || Math.Abs(hint.ActualHeight - hint.FontSize) >= .5)
                throw new InvalidOperationException($"{name}: a bare key hint must have no corner and one line of type.");
            if (Read(badge, "lineHeight") != Math.Round(badge.FontSize * 1.4, 2) || Read(tag, "lineHeight") != Math.Round(tag.Selection.FontSize * 1.4, 2))
                throw new InvalidOperationException($"{name}: annotation leading must follow React's 1.4 line box.");
            if (Read(disclosure, "corner") != 0 || Read(meter, "corner") != 0)
                throw new InvalidOperationException($"{name}: a clear layout borrowed its child's corner.");
            if (Math.Abs(disclosure.ActualHeight - theme.Text("", "labelMedium").LineHeight - 2 * theme.Number("spacing.2")) >= .5)
                throw new InvalidOperationException($"{name}: disclosure retained native minimum height or text leading ({disclosure.ActualHeight}).");
            if (!Equals(disclosure.Header, "Disclosure")) throw new InvalidOperationException("Disclosure changed its public Header value.");
            if (Read(progress, "corner") != progress.ActualHeight / 2 || Read(spinner, "corner") != spinner.ActualHeight / 2 || Read(spinner, "stroke") != theme.Number("spacing.0.5"))
                throw new InvalidOperationException($"{name}: a native track or ring lost its measured geometry.");
            if (Read(preference, "corner") != Math.Min(theme.Number("control.container.radius"), preference.ActualHeight / 2))
                throw new InvalidOperationException($"{name}: a transparent preference row lost its corner.");
            if (!ReferenceEquals(Collector.PaintedRoot(fraction, "Skeleton"), fractionBone) || fractionBone.ActualWidth != 128)
                throw new InvalidOperationException($"{name}: percentage skeleton measured its layout wrapper.");
            if (name.StartsWith("bright-")) await SmokeTest.SaveAsync(shell, output, name + "-corners.png");
            if (name.StartsWith("bright-"))
            {
                foreach (var height in new[] { 60d, 28d })
                {
                    single.Height = height;
                    await Task.Delay(40); shell.UpdateLayout(); await new RenderTargetBitmap().RenderAsync(shell).AsTask().WaitAsync(TimeSpan.FromSeconds(30));
                    if (single.CornerRadius.TopLeft != height / 2) throw new InvalidOperationException("Pill corners did not follow a resize.");
                }
            }
        }
        File.WriteAllText(Path.Combine(output, "corners.txt"), "PASS: control/container corner roles, rendered template borders and conformance surface boundaries in all registered themes, including live theme changes.");
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
