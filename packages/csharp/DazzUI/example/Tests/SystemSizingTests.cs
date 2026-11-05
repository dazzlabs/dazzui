using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;

namespace DazzUI.Example;

internal static class SystemSizingTests
{
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }

    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }

    private static async Task Settle(Shell shell)
    {
        await Task.Delay(120);
        shell.UpdateLayout();
        await SmokeTest.CaptureAsync(shell);
    }

    internal static async Task RunAsync(Shell shell, string output)
    {
        File.WriteAllText(Path.Combine(output, "progress.txt"), "System typography and control sizing at 100%, 150%, 225%");
        var log = new List<string>();
        try
        {
            foreach (var name in new[] { "studio-light", "studio-dark", "windows11-light", "ember-dark" })
            {
                var design = new Theme(name);
                var system = new Theme(name, ThemeSizing.System);
                var field = new TextField { Text = "Keep this selection", Size = ControlSize.Medium, Width = 280 };
                var buttons = Enum.GetValues<ControlSize>().Select(size => new Button { Content = "Save changes 保存", Size = size }).ToArray();
                var check = new Checkbox { Content = "Preserve checked state", IsChecked = true };
                var panel = new StackPanel { Spacing = 8, Children = { field, check } };
                foreach (var button in buttons) panel.Children.Add(button);
                var hostUpdates = 0;
                Appearance.Follow(panel, theme => { panel.Spacing = theme.Number("spacing.2"); hostUpdates++; });
                // A nested design scope retains its own geometry and scaling policy.
                var fixedButton = new Button { Content = "Design", Theme = design };
                panel.Children.Add(fixedButton);
                var scope = new ThemeScope { Theme = system, Content = panel };
                var inheritedText = new TextBlock { Text = "Inherited system typography" };
                panel.Children.Add(inheritedText);
                shell.ShowFixture(scope);
                await Settle(shell);
                field.Select(5, 4);
                double initialHeight = 0;
                foreach (var scale in new[] { 1.0, 1.5, 2.25, 1.0 })
                {
                    var previousUpdates = hostUpdates;
                    await Task.Run(() => SystemStyle.SetTextScaleForTest(scale));
                    await Settle(shell);
                    Check(hostUpdates > previousUpdates && Math.Abs(panel.Spacing - system.Number("spacing.2")) < .001,
                        "A host-owned panel did not follow a background settings notification.");
                    var expected = SystemStyle.FontSize * scale;
                    Check(Math.Abs(inheritedText.FontSize - expected) < .001 && !inheritedText.IsTextScaleFactorEnabled,
                        "Plain host text lost the scope's resolved typography.");
                    Check(Math.Abs(field.FontSize - expected) < .001, $"{name} {scale}: field font did not follow Windows: {field.FontSize}, expected {expected}.");
                    Check(!field.IsTextScaleFactorEnabled, "The native editor would apply accessibility scaling twice.");
                    Check(field.Text == "Keep this selection" && field.SelectionStart == 5 && field.SelectionLength == 4,
                        "A system setting change reset native editing or selection state.");
                    Check(check.IsChecked == true, "A system setting change reset checkbox state.");
                    if (initialHeight == 0) { initialHeight = field.ActualHeight; }
                    var pixel = 1 / field.XamlRoot.RasterizationScale;
                    Check(Math.Abs(field.ActualHeight - initialHeight * scale) <= pixel + .01, "The editor's box did not scale with its text.");
                    Check(Math.Abs(fixedButton.Height - design.Number("control.small.size")) < .001,
                        "A nested design theme followed the outer system scale.");
                    fixedButton.Theme = system;
                    Check(!fixedButton.IsTextScaleFactorEnabled, "A direct button theme change kept native text scaling enabled.");
                    fixedButton.Theme = design;
                    foreach (var button in buttons)
                    {
                        Check(Math.Abs(button.FontSize - system.Text("", $"control.{button.Size.ToString().ToLowerInvariant()}.content").FontSize) < .001, "A button retained design typography in system mode.");
                        var label = Walk(button).OfType<TextBlock>().FirstOrDefault(text => text.Text.Contains("Save changes"));
                        Check(label is not null && !label.IsTextScaleFactorEnabled, "A template label would scale twice.");
                        Check(label!.ActualHeight <= button.ActualHeight + pixel, "A scaled button clipped its label vertically.");
                    }
                    if (name == "studio-light" && scale != 1)
                        await SmokeTest.SaveAsync(scope, output, $"system-sizing-{scale:0.00}.png");
                    Check(system.Color("color.content") == design.Color("color.content"), "System sizing changed the palette.");
                    Check(system.Number("stroke.hairline") == design.Number("stroke.hairline"), "System sizing enlarged hairlines.");
                    Check(system.Number("radius.medium") == design.Number("radius.medium"), "System sizing changed corner roles.");
                    log.Add($"{name} {scale:P0}: font={field.FontSize}, height={field.ActualHeight}, selection preserved");
                }
                Appearance.SetTheme(scope, design);
                await Settle(shell);
                Check(Math.Abs(field.FontSize - design.Text("", "control.medium.content").FontSize) < .001 && field.Text == "Keep this selection",
                    "Switching back to design sizing lost typography or editing state.");
            }
            shell.ChooseSizing(ThemeSizing.System);
            SystemStyle.SetTextScaleForTest(2.25);
            foreach (var story in Stories.All)
            {
                File.WriteAllText(Path.Combine(output, "progress.txt"), $"System sizing at 225%: {story.Id}");
                shell.Open(story.Id);
                await Settle(shell);
            }
            log.Add($"All {Stories.All.Count} gallery pages rendered in system sizing at 225%.");
            File.WriteAllLines(Path.Combine(output, "system-sizing.txt"), log);
        }
        finally
        {
            SystemStyle.SetTextScaleForTest(null);
            shell.ChooseSizing(ThemeSizing.Design);
        }
    }
}
