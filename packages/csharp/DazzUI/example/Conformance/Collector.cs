using System.Runtime.InteropServices.WindowsRuntime;
using System.Text.Json;
using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.Foundation;
using Windows.Graphics.Imaging;
using Windows.Storage.Streams;

namespace DazzUI.Example;

/// <summary>
/// The WinUI collector, run with <c>--conformance</c>: draws every scene in
/// scenes.json in a real window, in every theme asked for, and writes
/// readings.json, the scene images and a crop per specimen. See
/// specs/conformance.md. It is driven by these, which
/// scripts/conformance.sh sets:
/// <list type="bullet">
/// <item><c>DAZZUI_CONFORMANCE_OUTPUT</c> — where the readings go;</item>
/// <item><c>DAZZUI_CONFORMANCE_CONTRACT</c> — scenes.json;</item>
/// <item><c>DAZZUI_CONFORMANCE_THEMES</c> — <c>drawn</c>, <c>all</c> or a list;</item>
/// <item><c>DAZZUI_CONFORMANCE_SCENES</c> — stories to keep, by id;</item>
/// <item><c>DAZZUI_CONFORMANCE_IMAGES</c> — <c>false</c> to skip pictures;</item>
/// <item><c>DAZZUI_CONFORMANCE_COMMIT</c> — the commit being measured.</item>
/// </list>
/// </summary>
internal static class Collector
{
    private const double Scale = 2;

    internal static FrameworkElement PaintedRoot(FrameworkElement root, string component) => root switch
    {
        Autocomplete query => query.Input,
        ShortcutRecorder recorder => recorder.Field,
        Grid room when component == "Skeleton" && room.Children.Count == 1 && room.Children[0] is Skeleton bone => bone,
        _ => root,
    };

    public static async Task RunAsync(Window window, ContentControl host)
    {
        using var display = new DisplayWakeLock();
        var output = Environment.GetEnvironmentVariable("DAZZUI_CONFORMANCE_OUTPUT")
            ?? Path.Combine(AppContext.BaseDirectory, "conformance");
        Directory.CreateDirectory(output);
        var finishing = false;
        window.Closed += (_, _) =>
        {
            if (finishing) return;
            File.WriteAllText(Path.Combine(output, "failures.txt"), "The collection window closed before measurement finished.");
            Environment.Exit(1);
        };
        try
        {
            await CollectAsync(host, output);
            finishing = true;
            window.Close();
            Environment.Exit(0);
        }
        catch (Exception error)
        {
            await File.WriteAllTextAsync(Path.Combine(output, "failures.txt"), error.ToString());
            finishing = true;
            window.Close();
            Environment.Exit(1);
        }
    }

    private static async Task CollectAsync(ContentControl host, string output)
    {
        string? Env(string name) => Environment.GetEnvironmentVariable($"DAZZUI_CONFORMANCE_{name}") is { Length: > 0 } value ? value : null;
        var contract = JsonNode.Parse(await File.ReadAllTextAsync(Env("CONTRACT") ?? Path.Combine(AppContext.BaseDirectory, "scenes.json")))!;
        var only = Env("SCENES")?.Split(',');
        var images = Env("IMAGES") != "false";

        ThemeChoice[] Halves(IEnumerable<ThemeFamily> families) =>
            families.SelectMany(family => new[] { new ThemeChoice(family.Value), new ThemeChoice(family.Value, true) }).ToArray();
        var themes = (Env("THEMES") ?? "drawn") switch
        {
            "drawn" => Halves(Themes.Groups[0].Families),
            "all" => Halves(Themes.Groups.SelectMany(group => group.Families)),
            var list => list.Split(',').Select(id => new ThemeChoice(id[..id.LastIndexOf('-')], id.EndsWith("-dark"))).ToArray(),
        };

        var scenes = contract["stories"].Items()
            .SelectMany(story => story!["scenes"].Items())
            .Where(scene => only is null || only.Any(id => scene!["id"].Text() == id || scene["id"].Text().StartsWith(id + "/", StringComparison.Ordinal)))
            .ToList();
        var drawable = new Dictionary<string, Scene>();
        foreach (var story in Stories.All)
        {
            drawable[$"{story.Id}/preview"] = story.Preview;
            for (var i = 0; i < story.Examples.Count; i++) drawable[$"{story.Id}/{i}"] = story.Examples[i].Demo;
        }

        var readingsThemes = new JsonObject();
        var readingsScenes = new JsonObject();
        var failures = new List<string>();

        // The frame every collector draws a scene in: 40rem wide, spacing-6
        // around it, on the surface, its content centred.
        var frame = new Grid { Width = 640, HorizontalAlignment = HorizontalAlignment.Left, VerticalAlignment = VerticalAlignment.Top };
        host.Content = frame;

        foreach (var choice in themes)
        {
            var theme = choice.Theme;
            host.RequestedTheme = theme.IsDark ? ElementTheme.Dark : ElementTheme.Light;
            Appearance.SetTheme(host, theme);
            host.Foreground = theme.Brush("color.content");
            theme.Typography(host, "bodyMedium");
            frame.Background = theme.Brush("color.surface");
            frame.Padding = new Thickness(theme.Number("spacing.6"));

            var readingsOf = new JsonObject();
            var scenesOf = new JsonObject();
            readingsThemes[choice.Id] = readingsOf;
            readingsScenes[choice.Id] = scenesOf;
            var imageDir = Path.Combine(output, "images", choice.Id);
            if (images) Directory.CreateDirectory(imageDir);

            foreach (var contracted in scenes)
            {
                var id = contracted!["id"].Text();
                File.WriteAllText(Path.Combine(output, "progress.txt"), $"{choice.Id} · {id}");
                if (!drawable.TryGetValue(id, out var scene))
                {
                    failures.Add($"{id}: the example has no such scene");
                    continue;
                }
                var nodes = Addresses.SpecimensOf(scene);
                var specimens = contracted["specimens"].Items();
                var want = string.Join(' ', specimens.Select(s => s!["component"].Text()));
                var got = string.Join(' ', nodes.Select(n => Addresses.Kebab(n.Element()!)));
                if (want != got)
                {
                    failures.Add($"{id}: codegen addressed {want}, the walk found {got}");
                    continue;
                }

                // Each node leads to the elements built from it, in the order they were built.
                var built = new Dictionary<JsonNode, Queue<UIElement>>(ReferenceEqualityComparer.Instance);
                void Record(JsonNode node, UIElement element)
                {
                    if (!built.TryGetValue(node, out var queue)) built[node] = queue = new();
                    queue.Enqueue(element);
                }
                Nodes.Built += Record;
                try
                {
                    frame.Children.Clear();
                    frame.Children.Add(new SceneView(scene, align: true));
                    await Settle(host);
                }
                finally { Nodes.Built -= Record; }

                FrameworkElement? Attached(JsonNode node)
                {
                    if (!built.TryGetValue(node, out var queue)) return null;
                    while (queue.TryDequeue(out var element))
                        if (element is FrameworkElement root && root.XamlRoot is not null && IsInside(root, frame)) return PaintedRoot(root, node.Element()!);
                    return null;
                }
                var roots = nodes
                    .Select(Attached)
                    .ToArray();
                // A provider, or a wrapper lending its trigger a behaviour,
                // draws nothing of its own: its element is a specimen inside it.
                for (var i = 0; i < roots.Length; i++)
                    for (var j = i + 1; j < roots.Length; j++)
                        if (roots[i] is not null && ReferenceEquals(roots[i], roots[j])) roots[i] = null;
                // One that is not in the window — a popup's, before it opens — has no reading.
                for (var i = 0; i < roots.Length; i++)
                    if (roots[i] is { } root && (root.XamlRoot is null || !IsInside(root, frame) || root.ActualWidth == 0 && root.ActualHeight == 0))
                        roots[i] = null;

                var file = id.Replace('/', '-');
                var crops = new string?[roots.Length];
                if (images)
                {
                    var rects = roots.Select(root => root is null ? (Rect?)null
                        : root.TransformToVisual(frame).TransformBounds(new Rect(0, 0, root.ActualWidth, root.ActualHeight))).ToArray();
                    crops = await DrawAsync(frame, rects, imageDir, file);
                    scenesOf[id] = $"images/{choice.Id}/{file}.png";
                }

                var nested = new HashSet<DependencyObject>(roots.OfType<FrameworkElement>());
                for (var i = 0; i < roots.Length; i++)
                {
                    if (roots[i] is not { } root) continue;
                    var reading = Measure.Read(root, nested);
                    if (crops[i] is { } crop) reading["image"] = $"images/{choice.Id}/{crop}";
                    readingsOf[specimens[i]!["address"].Text()] = reading;
                }
            }
        }

        var readings = new JsonObject
        {
            ["platform"] = "winui",
            ["title"] = "WinUI",
            ["role"] = "implementation",
            ["collected"] = DateTime.UtcNow.ToString("yyyy-MM-ddTHH:mm:ss.fffZ"),
            ["commit"] = Env("COMMIT") ?? "",
            ["scale"] = Scale,
            // System fonts are host-specific; no cross-platform test font.
            ["font"] = null,
            ["themes"] = readingsThemes,
            ["scenes"] = readingsScenes,
        };
        await File.WriteAllTextAsync(Path.Combine(output, "readings.json"), readings.ToJsonString());
        if (failures.Count > 0)
            await File.WriteAllTextAsync(Path.Combine(output, "failures.txt"), string.Join('\n', failures) + "\n");
    }

    private static bool IsInside(DependencyObject element, DependencyObject ancestor)
    {
        for (var item = element; item is not null; item = Microsoft.UI.Xaml.Media.VisualTreeHelper.GetParent(item))
            if (item == ancestor) return true;
        return false;
    }

    private static async Task Settle(FrameworkElement host)
    {
        // Loaded handlers apply the theme; a second pass lays out what they changed.
        await Task.Delay(30);
        host.UpdateLayout();
        await Task.Delay(30);
        host.UpdateLayout();
    }

    /// <summary>Writes the scene as a PNG, and a crop of it per rectangle; answers the crops' names.</summary>
    private static async Task<string?[]> DrawAsync(FrameworkElement frame, Rect?[] rects, string dir, string file)
    {
        var bitmap = new RenderTargetBitmap();
        await bitmap.RenderAsync(frame, (int)Math.Round(frame.ActualWidth * Scale), (int)Math.Round(frame.ActualHeight * Scale)).AsTask().WaitAsync(TimeSpan.FromSeconds(30));
        var pixels = (await bitmap.GetPixelsAsync()).ToArray();
        int width = bitmap.PixelWidth, height = bitmap.PixelHeight;
        if (width == 0 || height == 0) return new string?[rects.Length];
        await WritePngAsync(Path.Combine(dir, $"{file}.png"), pixels, width, height);
        // What the window actually drew at, which is what the crops are cut at.
        var scale = width / frame.ActualWidth;
        var names = new string?[rects.Length];
        for (var i = 0; i < rects.Length; i++)
        {
            if (rects[i] is not { } rect) continue;
            int x = Math.Max(0, (int)Math.Round(rect.X * scale)), y = Math.Max(0, (int)Math.Round(rect.Y * scale));
            int w = Math.Min(width - x, (int)Math.Round(rect.Width * scale)), h = Math.Min(height - y, (int)Math.Round(rect.Height * scale));
            if (w < 1 || h < 1) continue;
            var crop = new byte[w * h * 4];
            for (var row = 0; row < h; row++)
                System.Buffer.BlockCopy(pixels, ((y + row) * width + x) * 4, crop, row * w * 4, w * 4);
            names[i] = $"{file}-{i}.png";
            await WritePngAsync(Path.Combine(dir, names[i]!), crop, w, h);
        }
        return names;
    }

    private static async Task WritePngAsync(string path, byte[] pixels, int width, int height)
    {
        using var stream = new InMemoryRandomAccessStream();
        var encoder = await BitmapEncoder.CreateAsync(BitmapEncoder.PngEncoderId, stream);
        encoder.SetPixelData(BitmapPixelFormat.Bgra8, BitmapAlphaMode.Premultiplied, (uint)width, (uint)height, 96, 96, pixels);
        await encoder.FlushAsync();
        var bytes = new byte[stream.Size];
        stream.Seek(0);
        await stream.ReadAsync(bytes.AsBuffer(), (uint)bytes.Length, InputStreamOptions.None);
        await File.WriteAllBytesAsync(path, bytes);
    }
}
