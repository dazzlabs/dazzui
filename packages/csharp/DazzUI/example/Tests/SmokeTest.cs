using System.Runtime.InteropServices.WindowsRuntime;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.Graphics.Imaging;
using Windows.Storage;

namespace DazzUI.Example;

/// <summary>
/// Runs in a real WinUI window: every page in every theme, laid out and
/// rendered; a scene's live state; then the kit's own behavior and corner
/// fixtures.
/// </summary>
internal static class SmokeTest
{
    public static async Task RunAsync(Window window, Shell shell)
    {
        var output = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_OUTPUT")
            ?? Path.Combine(AppContext.BaseDirectory, "smoke-results");
        Directory.CreateDirectory(output);
        try
        {
            await File.WriteAllTextAsync(Path.Combine(output, "stories.json"), System.Text.Json.JsonSerializer.Serialize(
                Stories.All.Select(story => new { id = story.Id, name = story.Name, examples = story.Examples.Select(example => example.Title) })));
            // DAZZUI_SMOKE_PAGES narrows the run to a few pages, by id.
            var pageIds = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_PAGES")?.Split(',', StringSplitOptions.RemoveEmptyEntries);
            var pages = new string?[] { null }.Concat(Stories.All.Select(story => story.Id))
                .Where(page => pageIds is null || pageIds.Contains(page ?? "overview")).ToArray();
            // DAZZUI_SMOKE_THEMES narrows the run to a few themes, by name.
            var only = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_THEMES")?.Split(',', StringSplitOptions.RemoveEmptyEntries);
            ThemeChoice[] Halves(IEnumerable<ThemeFamily> families) => families
                .SelectMany(family => new[] { new ThemeChoice(family.Value), new ThemeChoice(family.Value, true) })
                .Where(theme => only is null || only.Contains(theme.Id)).ToArray();
            // Every page in the twelve themes DazzUI draws; the overview, which
            // holds every component, in each theme imported from elsewhere.
            var drawn = Halves(Themes.Groups[0].Families);
            var imported = Halves(Themes.Groups.Skip(1).SelectMany(group => group.Families));
            var themes = drawn.Concat(imported).ToArray();
            var total = drawn.Length * pages.Length + imported.Length;
            // DAZZUI_SMOKE_SHOTS=all keeps a picture of every page in the first theme.
            var shots = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_SHOTS") == "all";
            var renders = 0;
            foreach (var choice in themes)
            {
                var theme = choice.Theme;
                // Every state, variant and tint must be backed by a recipe.
                foreach (var tint in Enum.GetValues<WidgetTint>())
                    foreach (var variant in Enum.GetValues<ControlVariant>())
                        foreach (var state in new[] { "normal", "hovered", "pressed", "disabled" })
                            foreach (var part in new[] { "surface", "content", "border" })
                                _ = theme.Recipe(variant, tint, part, state);
                shell.Choose(choice);
                foreach (var page in drawn.Contains(choice) ? pages : [null])
                {
                    File.WriteAllText(Path.Combine(output, "progress.txt"), $"{choice.Id} · {page ?? "overview"} ({renders + 1}/{total})");
                    var clock = System.Diagnostics.Stopwatch.StartNew();
                    shell.Open(page);
                    var opened = clock.ElapsedMilliseconds;
                    await Settle(shell);
                    var settled = clock.ElapsedMilliseconds;
                    var bitmap = new RenderTargetBitmap();
                    await bitmap.RenderAsync(shell);
                    File.AppendAllText(Path.Combine(output, "timings.txt"), $"{choice.Id} {page ?? "overview"} open={opened} settle={settled - opened} render={clock.ElapsedMilliseconds - settled} gc={GC.CollectionCount(2)}\n");
                    if (bitmap.PixelWidth == 0 || bitmap.PixelHeight == 0)
                        throw new InvalidOperationException($"Empty render: {choice.Id}, {page}");
                    CheckButtons(shell, choice.Id, page);
                    if (shots && choice == themes[0] || page is null or "button" && choice.Family == "studio")
                        await SaveAsync(shell, output, $"{choice.Id}-{page ?? "overview"}.png");
                    renders++;
                }
            }

            await LiveState(shell);
            await BehaviorTests.RunAsync(shell, output);
            await CornerTests.RunAsync(shell, output);
            await File.WriteAllTextAsync(Path.Combine(output, "result.txt"),
                $"PASS: {Stories.All.Count} stories; every page in {drawn.Length} drawn themes and the overview in {imported.Length} imported ones, {renders} renders; button heights; live scene state; component and popup interactions (behavior.txt).\n");
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

    /// <summary>Every kit button on the page is the height its size says.</summary>
    private static void CheckButtons(Shell shell, string theme, string? page)
    {
        if (shell.CurrentPage is not DependencyObject root) return;
        foreach (var button in Descendants(root).OfType<Button>().Where(button => button.GetType() == typeof(Button) && button.ActualHeight > 0))
        {
            var expected = button.Theme.Number($"control.{button.Size.ToString().ToLowerInvariant()}.size");
            // Layout rounds to whole device pixels: a 19px button at 150% is 28.5 of them.
            var pixel = 1 / (button.XamlRoot?.RasterizationScale ?? 1);
            if (Math.Abs(button.ActualHeight - expected) > pixel + 0.01)
                throw new InvalidOperationException($"{theme}, {page}: a {button.Size} button is {button.ActualHeight} tall, expected {expected}.");
        }
    }

    /// <summary>A scene keeps its slots live: choosing a sidebar row renames the page it heads.</summary>
    private static async Task LiveState(Shell shell)
    {
        shell.Choose(ThemeChoice.Default);
        shell.Open("sidebar");
        await Settle(shell);
        var rows = Descendants(shell.CurrentPage!).OfType<NavItem>().ToArray();
        var glossaries = rows.FirstOrDefault(row => Equals(row.Content, "Glossaries"))
            ?? throw new InvalidOperationException("The sidebar story drew no Glossaries row.");
        ((IToggleProvider)new ToggleButtonAutomationPeer(glossaries).GetPattern(PatternInterface.Toggle)).Toggle();
        await Settle(shell);
        var heading = Descendants(shell.CurrentPage!).OfType<StageText>().Any(text => text.Children.FirstOrDefault() is TextBlock { Text: "Glossaries" });
        var documents = rows.First(row => Equals(row.Content, "Documents"));
        if (!heading || glossaries.IsChecked != true || documents.IsChecked != false)
            throw new InvalidOperationException("Choosing a sidebar row did not update the scene's slot.");
    }

    internal static async Task Settle(Shell shell)
    {
        // Let the new page's Loaded handlers run before layout.
        await Task.Delay(10);
        shell.UpdateLayout();
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
