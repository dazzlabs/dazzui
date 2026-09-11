using System.Runtime.InteropServices.WindowsRuntime;
using DazzUI.Playground.Storybook;
using DazzUI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.Graphics.Imaging;
using Windows.Storage;
using Button = DazzUI.Button;

namespace DazzUI.Playground;

/// <summary>Runs in a real WinUI window: layout, rendering and native Invoke behavior.</summary>
internal static class SmokeTest
{
    public static async Task RunAsync(Window window, PlaygroundShell shell)
    {
        var output = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_OUTPUT")
            ?? Path.Combine(AppContext.BaseDirectory, "smoke-results");
        Directory.CreateDirectory(output);
        try
        {
            var count = 0;
            var storyCount = PlaygroundShell.Entries.Count;
            var themes = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_BEHAVIOR_ONLY") == "1" ? Array.Empty<string>() : Theme.Names;
            await File.WriteAllTextAsync(Path.Combine(output, "stories.json"), System.Text.Json.JsonSerializer.Serialize(PlaygroundShell.Entries.Select(entry => new { group = entry.Page.Group, page = entry.Page.Name, name = entry.Story.Name, id = entry.Id })));
            foreach (var name in themes)
            {
                var theme = new Theme(name);
                // Ensure every state/variant/tint is backed by a valid recipe.
                foreach (var tint in Enum.GetValues<WidgetTint>())
                    foreach (var variant in Enum.GetValues<ControlVariant>())
                        foreach (var state in new[] { "normal", "hovered", "pressed", "disabled" })
                            foreach (var part in new[] { "surface", "content", "border" })
                                _ = theme.Recipe(variant, tint, part, state);
                for (var i = 0; i < storyCount; i++)
                {
                    var entry = PlaygroundShell.Entries[i];
                    File.WriteAllText(Path.Combine(output, "progress.txt"), $"{name} · {entry.Page.Name} / {entry.Story.Name} ({count + 1}/{Theme.Names.Count * storyCount})");
                    shell.Show(name, i);
                    // Loaded applies inherited themes once the subtree is attached to the window.
                    var bitmap = new RenderTargetBitmap();
                    await bitmap.RenderAsync(shell);
                    if (bitmap.PixelWidth == 0 || bitmap.PixelHeight == 0)
                        throw new InvalidOperationException($"Empty render: {name}, story {i}");
                    if (entry.Page.Name == "Button" && entry.Story.Name == "Overview")
                        await SaveAsync(shell, output, name + ".png");
                    foreach (var button in Descendants(shell.CurrentSpecimen!).OfType<Button>().Where(button => button.GetType() == typeof(Button)))
                    {
                        var expected = button.Theme.Number($"control.{button.Size.ToString().ToLowerInvariant()}.size");
                        if (Math.Abs(button.ActualHeight - expected) > 0.1)
                            throw new InvalidOperationException($"{name}: {button.Size} height {button.ActualHeight}, expected {expected}");
                        var presenter = Descendants(button).OfType<Microsoft.UI.Xaml.Controls.ContentPresenter>().First();
                        foreach (var (visualState, recipeState) in new[] { ("PointerOver", "hovered"), ("Pressed", "pressed"), ("Disabled", "disabled"), ("Normal", "normal") })
                        {
                            if (!VisualStateManager.GoToState(button, visualState, false))
                                throw new InvalidOperationException($"Missing button visual state: {visualState}");
                            foreach (var (part, brush) in new[] { ("surface", presenter.Background), ("content", presenter.Foreground), ("border", presenter.BorderBrush) })
                            {
                                var actual = (SolidColorBrush)brush;
                                var recipe = button.Theme.Recipe(button.Variant, button.Tint, part, recipeState);
                                if (actual.Color != recipe.Color)
                                    throw new InvalidOperationException($"{name}, {entry.Page.Name}/{entry.Story.Name}: {button.Theme.Name} {button.Variant} {visualState} {part}: {actual.Color}, expected {recipe.Color}.");
                            }
                        }
                        VisualStateManager.GoToState(button, button.IsEnabled ? "Normal" : "Disabled", false);
                    }
                    count++;
                }
            }

            await BehaviorTests.RunAsync(shell, output);
            shell.Show("studio-light", PlaygroundShell.Find("Button", "Normal"));
            await Task.Delay(50);
            var counter = Descendants(shell.CurrentSpecimen!).OfType<Button>().Single();
            var invoked = false;
            counter.Click += (_, _) => invoked = true;
            var peer = new ButtonAutomationPeer(counter);
            ((IInvokeProvider)peer.GetPattern(PatternInterface.Invoke)).Invoke();
            // Invoke is queued through the WinUI dispatcher.
            await Task.Delay(100);
            if (!invoked) throw new InvalidOperationException("Native button Invoke did not raise Click.");
            if (!Equals(counter.Content, "Saved"))
                throw new InvalidOperationException("The button story did not update after invocation.");

            VisualStateManager.GoToState(counter, "PointerOver", false);
            counter.Tint = WidgetTint.Danger;
            var counterPresenter = Descendants(counter).OfType<Microsoft.UI.Xaml.Controls.ContentPresenter>().First();
            if (((SolidColorBrush)counterPresenter.Background).Color != counter.Theme.Recipe(counter.Variant, counter.Tint, "surface", "hovered").Color)
                throw new InvalidOperationException($"Changing Tint did not update an active visual state: {((SolidColorBrush)counterPresenter.Background).Color}; expected {counter.Theme.Recipe(counter.Variant, counter.Tint, "surface", "hovered").Color}; state {VisualStateManager.GetVisualStateGroups(counterPresenter)[0].CurrentState.Name}.");

            foreach (var name in new[] { "studio-light", "studio-dark" })
            {
                shell.Show(name, PlaygroundShell.Find("Button", "Overview"));
                await SaveAsync(shell, output, name + ".png");
            }
            await File.WriteAllTextAsync(Path.Combine(output, "result.txt"),
                $"PASS: {Registry.Pages.Count} pages, {themes.Count} rendered themes, {storyCount} stories, {count} renders; button heights and state recipes; live Tint change; native Invoke; component and popup interactions (behavior.txt).\n");
            window.Close();
            Environment.Exit(0);
        }
        catch (Exception error)
        {
            await File.WriteAllTextAsync(Path.Combine(output, "result.txt"), error.ToString());
            window.Close();
            Environment.Exit(1);
        }
    }

    private static IEnumerable<DependencyObject> Descendants(DependencyObject root)
    {
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
        {
            var child = VisualTreeHelper.GetChild(root, i);
            yield return child;
            foreach (var descendant in Descendants(child)) yield return descendant;
        }
    }

    internal static async Task SaveAsync(UIElement element, string directory, string name)
    {
        var bitmap = new RenderTargetBitmap();
        await bitmap.RenderAsync(element);
        var pixels = (await bitmap.GetPixelsAsync()).ToArray();
        var folder = await StorageFolder.GetFolderFromPathAsync(Path.GetFullPath(directory));
        var file = await folder.CreateFileAsync(name, CreationCollisionOption.ReplaceExisting);
        using var stream = await file.OpenAsync(FileAccessMode.ReadWrite);
        var encoder = await BitmapEncoder.CreateAsync(BitmapEncoder.PngEncoderId, stream);
        encoder.SetPixelData(BitmapPixelFormat.Bgra8, BitmapAlphaMode.Premultiplied,
            (uint)bitmap.PixelWidth, (uint)bitmap.PixelHeight, 96, 96, pixels);
        await encoder.FlushAsync();
    }
}
