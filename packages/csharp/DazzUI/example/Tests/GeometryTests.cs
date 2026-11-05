using Microsoft.UI.Composition;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Documents;
using Microsoft.UI.Xaml.Hosting;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;
using Windows.Graphics.Imaging;
using Windows.Storage;

namespace DazzUI.Example;

/// <summary>Actual layout and native drawing after live theme changes.</summary>
internal static class GeometryTests
{
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }

    internal static async Task RunAsync(Shell shell, string output)
    {
        var failures = new List<string>();
        void Check(bool pass, string message) { if (!pass) failures.Add(message); }
        var button = new Button { Content = "primary", Variant = ControlVariant.Filled };
        var disabled = new Button { Content = "Unavailable", IsEnabled = false };
        var group = new AvatarGroup { Overflow = 2 };
        group.Avatars.Add(new Avatar { NameText = "Ada Lovelace" });
        group.Avatars.Add(new Avatar { NameText = "Grace Hopper", Shape = AvatarShape.Square });
        group.Avatars.Add(new Avatar { NameText = "Alan Turing" });
        var keys = new KeyCap("⌘↵ to send");
        var pills = new PillTabs { SelectedIndex = 0 };
        pills.Items.Add("All"); pills.Counts[0] = 24;
        pills.Items.Add("Unread"); pills.Counts[1] = 3;
        var inline = new Link { Content = "@ada", IsInline = true };
        var select = new Select { HorizontalAlignment = HorizontalAlignment.Left };
        select.SetOptions([new SelectOption("one", "First")]); select.Value = "one";
        var panel = new StackPanel { Spacing = 20, Children = { button, disabled, group, keys, pills, inline, select } };
        var scope = new ThemeScope { Content = panel, Padding = new Thickness(24), HorizontalAlignment = HorizontalAlignment.Left };
        shell.ShowFixture(scope);
        foreach (var name in new[] { "bright-light", "studio-dark", "windows11-light", "bright-dark", "bright-light" })
        {
            var theme = new Theme(name);
            scope.Theme = theme;
            scope.Background = theme.Brush("color.surface");
            await Task.Delay(180); shell.UpdateLayout();
            var label = Walk(button).OfType<TextBlock>().First(text => text.Text == "primary");
            var expected = new TextBlock { FontFamily = label.FontFamily, FontSize = label.FontSize, FontWeight = label.FontWeight, UseLayoutRounding = false, IsTextScaleFactorEnabled = false };
            var span = new Span { Inlines = { new Run { Text = label.Text } } };
            Typography.SetKerning(span, true); expected.Inlines.Add(span);
            expected.Measure(new Size(double.PositiveInfinity, double.PositiveInfinity));
            Check(Math.Abs(label.ActualWidth - expected.DesiredSize.Width) < .1, $"{name}: button skipped OpenType kerning");
            Check(Measure.Read(button, new HashSet<DependencyObject>())["values"]?["fontWeight"]?.GetValue<int>() == theme.Weight("labelStrong").Weight, $"{name}: font-face matching changed the declared weight");
            var disabledLabel = Walk(disabled).OfType<TextBlock>().First(text => text.Text == "Unavailable");
            Check((disabledLabel.Foreground as SolidColorBrush)?.Color == theme.Recipe(disabled.Variant, disabled.Tint, "content", "disabled").Color, $"{name}: formatted text lost disabled ink: {Measure.Hex(((SolidColorBrush)disabledLabel.Foreground).Color)}, expected {Measure.Hex(theme.Recipe(disabled.Variant, disabled.Tint, "content", "disabled").Color)}");
            var faces = group.Avatars.Cast<FrameworkElement>().ToArray();
            var first = faces[0].TransformToVisual(group).TransformPoint(default);
            var row = (StackPanel)group.Content;
            var last = (Border)((Border)row.Children.Last()).Child;
            var end = last.TransformToVisual(group).TransformPoint(new Windows.Foundation.Point(last.ActualWidth, 0));
            Check(Math.Abs(first.X) < .01 && Math.Abs(end.X - group.ActualWidth) < .01, $"{name}: avatar ring changed group width");
            foreach (var avatar in group.Avatars)
            {
                var ring = (Border)avatar.Parent;
                Check((ring.Background as SolidColorBrush)?.Color == theme.Color("color.surface"), $"{name}: avatar ring retained old ground");
                Check(Measure.Read(avatar, new HashSet<DependencyObject>())["values"]?["shadow"]?.GetValue<string>() is { } shadow && shadow != "none", $"{name}: painted avatar ring was not measured");
            }
            Check(inline.ActualHeight < theme.Text("", "bodyMedium").LineHeight, $"{name}: inline link took the paragraph line box");
            var count = Walk(pills).OfType<TextBlock>().First(text => text.Text == "24");
            Check(Typography.GetNumeralAlignment(count) == FontNumeralAlignment.Tabular, $"{name}: pill count lost tabular figures");
            var lift = Walk(button).OfType<FrameworkElement>().First(element => element.Name == "Lift");
            Check(Walk(button).OfType<ContentPresenter>().First(element => element.Name == "Presenter").BackgroundSizing == BackgroundSizing.OuterBorderEdge, $"{name}: button fill stopped inside its border");
            Check(ElementCompositionPreview.GetElementChildVisual(lift) is ContainerVisual { Children.Count: > 0 }, $"{name}: elevated button has no native shadow drawing");
            await SmokeTest.SaveAsync(scope, output, $"geometry-{name}.png");
            var box = button.TransformToVisual(scope).TransformBounds(new Windows.Foundation.Rect(0, 0, button.ActualWidth, button.ActualHeight));
            await File.WriteAllTextAsync(Path.Combine(output, $"geometry-{name}.json"), System.Text.Json.JsonSerializer.Serialize(new
            {
                width = scope.ActualWidth, height = scope.ActualHeight, scale = scope.XamlRoot.RasterizationScale,
                x = box.X, y = box.Y, buttonWidth = box.Width, buttonHeight = box.Height, surface = Measure.Hex(theme.Color("color.surface")),
            }));
        }
        var changing = new Theme("bright-light").Text("AV", "labelStrong");
        changing.Text = "primary";
        Check(changing.Text == "primary" && changing.Inlines.Single() is Span, "Updating a formatted label discarded its words or OpenType formatting");
        var sizing = new Border { Width = 172 };
        panel.Children.Add(sizing); await Task.Delay(40); shell.UpdateLayout();
        var animation = new WidthAnimation(sizing);
        animation.AnimateTo(172, Theme.Default);
        Check(!animation.Animating && sizing.Width == 172, "Reapplying the initial sidebar width started an animation");
        animation.AnimateTo(260, Theme.Default);
        await Task.Delay(60); shell.UpdateLayout();
        animation.AnimateTo(172, Theme.Default);
        Check(animation.Animating, "Reversing a sidebar animation jumped to its initial width");
        animation.Set(172);
        var pixels = new Canvas { Width = 256, Height = 180, Background = Theme.Default.Brush("color.surface"), HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Top };
        var shadowHost = new Border { Width = 160, Height = 40 };
        var face = new Surface { Width = 160, Height = 40, CornerRadius = new CornerRadius(8), Background = Theme.Default.Brush("color.content") };
        foreach (var element in new FrameworkElement[] { shadowHost, face })
        {
            Canvas.SetLeft(element, 48); Canvas.SetTop(element, 48); pixels.Children.Add(element);
        }
        shell.ShowFixture(pixels); await Task.Delay(60); shell.UpdateLayout();
        foreach (var token in new[] { "2xs", "sm" })
        {
            Elevation.Draw(face, shadowHost, Theme.Default, token);
            await Task.Delay(40);
            await SmokeTest.SaveAsync(pixels, output, $"shadow-{token}.png");
            var layer = Theme.Default.Shadow("shadow." + token).Single();
            using var stream = await (await StorageFile.GetFileFromPathAsync(Path.GetFullPath(Path.Combine(output, $"shadow-{token}.png")))).OpenReadAsync();
            var decoder = await BitmapDecoder.CreateAsync(stream);
            var data = (await decoder.GetPixelDataAsync(BitmapPixelFormat.Rgba8, BitmapAlphaMode.Ignore, new BitmapTransform(), ExifOrientationMode.IgnoreExifOrientation, ColorManagementMode.DoNotColorManage)).DetachPixelData();
            var scale = decoder.PixelWidth / pixels.ActualWidth;
            var bottom = 88 * scale;
            var offset = Math.Round(layer.Y * scale, MidpointRounding.AwayFromZero);
            var sigma = layer.Blur * scale / 2;
            var ground = Theme.Default.Color("color.surface");
            var background = new[] { ground.R, ground.G, ground.B };
            var ink = new[] { layer.Color.R, layer.Color.G, layer.Color.B };
            double error = 0;
            for (var y = (int)Math.Ceiling(bottom); y < Math.Min(decoder.PixelHeight, bottom + offset + sigma * 4); y++)
            {
                var alpha = layer.Color.A / 255d * Normal((bottom + offset - y - .5) / sigma);
                var index = ((int)(128 * scale) + y * (int)decoder.PixelWidth) * 4;
                for (var channel = 0; channel < 3; channel++)
                    error = Math.Max(error, Math.Abs(data[index + channel] - (background[channel] + (ink[channel] - background[channel]) * alpha)));
            }
            // Native and browser Gaussian kernels quantize to eight-bit sRGB.
            Check(error <= 3, $"{token}: shadow pixels differ from CSS Gaussian decay by {error:0.##} levels");
            await File.AppendAllTextAsync(Path.Combine(output, "shadow-pixels.txt"), $"{token}: max channel error {error:0.###}/255 at {scale:0.##}x\n");
        }
        if (failures.Count > 0) throw new InvalidOperationException(string.Join('\n', failures));
        await File.WriteAllTextAsync(Path.Combine(output, "geometry.txt"), "PASS: kerned text and live edits, declared weights, disabled ink, avatar outsets, inline link boxes, pill counts, native shadows and initial sidebar width.\n");
    }

    private static double Normal(double value)
    {
        var x = Math.Abs(value);
        var t = 1 / (1 + .2316419 * x);
        var tail = Math.Exp(-x * x / 2) / Math.Sqrt(2 * Math.PI)
            * t * (.319381530 + t * (-.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
        return value >= 0 ? 1 - tail : tail;
    }
}
