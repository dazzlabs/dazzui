using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class PreferenceThemeSwatchesTests
{
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }
    internal static async Task RunAsync(Shell shell, string output)
    {
        var control = new UI.PreferenceThemeSwatches { Value = "paper" };
        control.Options.Add(new("paper", "Studio", ""));
        control.Options.Add(new("night", "Nocturne", "nocturne-dark"));
        var row = new UI.PreferenceRow { Label = "Theme", Control = control, Width = 320 };
        var scope = new UI.ThemeScope { Theme = new("studio-dark"), Content = row };
        shell.ShowFixture(scope);
        await Task.Delay(100);
        shell.UpdateLayout();
        var radios = Walk(control).OfType<RadioButton>().ToArray();
        Check(radios.Length == 2 && radios[0].IsChecked == true && radios[1].IsChecked == false,
            "Default theme selection or native radio roles are missing.");
        Check(AutomationProperties.GetName(radios[1]) == "Nocturne", "The swatch lost its accessible name.");
        var firstWindow = Walk(radios[0]).OfType<Border>().First(border => border.Child is Grid);
        var secondWindow = Walk(radios[1]).OfType<Border>().First(border => border.Child is Grid);
        var firstEdges = Walk(radios[0]).OfType<Border>().Take(3).ToArray();
        Check(firstEdges.Length == 3 && firstEdges.All(edge => edge.CornerRadius.TopLeft == radios[0].CornerRadius.TopLeft),
            "A selected theme swatch drew its ring, gap and window with different corner radii.");
        var light = new UI.Theme("studio-light");
        var dark = new UI.Theme("nocturne-dark");
        Check(((SolidColorBrush)firstWindow.Background).Color == light.Color("color.surface")
            && ((SolidColorBrush)secondWindow.Background).Color == dark.Color("color.surface"),
            "Swatches inherited the page's palette instead of their own.");
        var width = control.ActualWidth;
        string? changed = null;
        control.ValueChanged += (_, value) => changed = value;
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(radios[1]);
        ((ISelectionItemProvider)peer.GetPattern(PatternInterface.SelectionItem)).Select();
        await Task.Delay(50);
        Check(control.Value == "night" && changed == "night" && radios[0].IsChecked == false,
            "Native selection did not update the value exclusively.");
        scope.Theme = new UI.Theme("bright-light");
        await Task.Delay(80);
        shell.UpdateLayout();
        Check(control.Value == "night" && ReferenceEquals(radios[1], Walk(control).OfType<RadioButton>().Last()),
            "Changing the page theme replaced the controls or lost selection.");
        firstWindow = Walk(radios[0]).OfType<Border>().First(border => border.Child is Grid);
        Check(((SolidColorBrush)firstWindow.Background).Color == light.Color("color.surface"),
            $"The page theme changed the preview palette: {((SolidColorBrush)firstWindow.Background).Color} instead of {light.Color("color.surface")}.");
        Check(Math.Abs(width - control.ActualWidth) < 0.1,
            $"The page theme changed preview geometry: {width} to {control.ActualWidth}; swatches {radios[0].ActualWidth}, {radios[1].ActualWidth}.");
        control.Value = "paper";
        Check(radios[0].IsChecked == true && radios[1].IsChecked == false, "External value updates were ignored.");
        control.ValueChanged += (_, _) => control.Value = "paper";
        ((ISelectionItemProvider)peer.GetPattern(PatternInterface.SelectionItem)).Select();
        Check(control.Value == "paper" && radios[0].IsChecked == true, "A refused selection did not restore the native radios.");
        for (var i = 0; i < 4; i++) control.Options.Add(new($"extra-{i}", $"Theme {i}", "studio-light"));
        row.Width = 150;
        Appearance.Refresh(row);
        await Task.Delay(80);
        shell.UpdateLayout();
        Check(control.ActualHeight > light.Number("control.small.size"), "Narrow theme swatches did not wrap.");
        var label = Walk(row).OfType<TextBlock>().First(text => text.Text == "Theme");
        Check(label.ActualWidth > 0 && control.ActualWidth < row.ActualWidth,
            "The wrapped swatches displaced the preference label.");
        await SmokeTest.SaveAsync(scope, output, "theme-swatches-narrow.png");
        var example = Stories.Preferences.Examples.Single(example => example.Title == "Choosing a theme");
        var scene = new SceneView(example.Demo) { Width = 512 };
        shell.ShowFixture(scene);
        await Task.Delay(80);
        shell.UpdateLayout();
        var generated = Walk(scene).OfType<UI.PreferenceThemeSwatches>().Single();
        var generatedRadios = Walk(generated).OfType<RadioButton>().ToArray();
        Check(generated.Options.Count == 6 && generated.Value == "studio-light",
            "The generated React theme example lost its options or initial value.");
        for (var i = 0; i < generated.Options.Count; i++)
        {
            var option = generated.Options[i];
            var preview = Walk(generatedRadios[i]).OfType<Border>().First(border => border.Child is Grid);
            var id = option.Theme ?? option.Value;
            var expected = new UI.Theme(id.Length == 0 ? "studio-light" : id);
            Check(((SolidColorBrush)preview.Background).Color == expected.Color("color.surface"),
                $"The generated {option.Label} swatch did not resolve its own theme.");
        }
        await SmokeTest.SaveAsync(scene, output, "theme-swatches-story.png");
    }
}
