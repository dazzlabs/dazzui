using System.Runtime.InteropServices;
using System.Runtime.InteropServices.WindowsRuntime;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.Graphics.Imaging;
using Windows.Storage.Streams;

namespace SettingsExample;

/// <summary>Clicks through the generated page with real mouse input — so
/// the XAML's Click wiring, the generated handler, the C ABI, Rust's update
/// and the x:Bind that shows the next snapshot are all on the path — and
/// checks the state Rust hands back after each click.</summary>
internal static class SmokeTest
{
    internal static string Output => Environment.GetEnvironmentVariable("DAZZUI_SMOKE_OUTPUT")
        ?? Path.Combine(Path.GetTempPath(), "SettingsExample");

    private static readonly List<string> log = [];

    public static async Task RunAsync(Window window, FrameworkElement root, SettingsViewModel model)
    {
        Directory.CreateDirectory(Output);
        try
        {
            await Settle();
            await SaveAsync(root, "initial.png");
            Check(model.State.Saved && model.State.Status == "No changes", "the first snapshot is Rust's default");
            Check(Texts(root).Contains("Spell check") && !Texts(root).Contains("Grammar"), "the for loop shows the visible plugins only");
            Check(Texts(root).Contains("Disabled plugins are hidden"), "the if shows the hidden-plugins note");

            var switches = Descendants<DazzUI.Switch>(root).ToList();
            Check(switches.Count == 3, $"three switches ({switches.Count})");

            await Click(window, switches[0]);
            Check(model.State.LaunchAtStartup && !model.State.Saved, "a click on the first switch reached Rust");
            Check(switches[0].IsChecked == true, "the switch shows Rust's value");
            Check(Texts(root).Contains("Unsaved changes"), "the status text follows the snapshot");

            await Click(window, Button(root, "Save"));
            Check(model.State.Saves == 1 && model.State.Status == "Saved", "Save reached Rust");
            Check(!Button(root, "Save").IsEnabled, "Save is disabled again once saved");

            await Click(window, switches[2]);
            Check(model.State.ShowDisabledPlugins && model.State.VisiblePlugins.Length == 3, "the selector recomputed");
            Check(Texts(root).Contains("Grammar"), "the for loop shows the disabled plugin");
            Check(!Texts(root).Contains("Disabled plugins are hidden"), "the if hides the note");

            await Click(window, Button(root, "Enable"));
            Check(model.State.Plugins.Single(p => p.Id == 2).Enabled, "a click in the loop sent its item's id");

            await SaveAsync(root, "after.png");
            await File.WriteAllLinesAsync(Path.Combine(Output, "result.txt"), log.Prepend("PASS"));
            Environment.Exit(0);
        }
        catch (Exception error)
        {
            await File.WriteAllLinesAsync(Path.Combine(Output, "result.txt"), log.Prepend("FAIL").Append(error.ToString()));
            await SaveAsync(root, "failure.png");
            Environment.Exit(1);
        }
    }

    private static void Check(bool ok, string what)
    {
        log.Add($"{(ok ? "ok  " : "FAIL")} {what}");
        if (!ok) throw new Exception($"check failed: {what}");
    }

    private static IEnumerable<T> Descendants<T>(DependencyObject parent) where T : DependencyObject
    {
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(parent); i++)
        {
            var child = VisualTreeHelper.GetChild(parent, i);
            if (child is T match) yield return match;
            foreach (var deeper in Descendants<T>(child)) yield return deeper;
        }
    }

    /// <summary>The visible texts, as a reader sees them: collapsed ones left out.</summary>
    private static HashSet<string> Texts(FrameworkElement root) =>
        Descendants<TextBlock>(root).Where(Shown).Select(t => t.Text).ToHashSet();

    private static bool Shown(FrameworkElement element)
    {
        for (DependencyObject? e = element; e is not null; e = VisualTreeHelper.GetParent(e))
            if (e is UIElement u && u.Visibility != Visibility.Visible) return false;
        return element.ActualWidth > 0;
    }

    private static Microsoft.UI.Xaml.Controls.Button Button(FrameworkElement root, string content) =>
        Descendants<Microsoft.UI.Xaml.Controls.Button>(root).First(b => Equals(b.Content, content) && Shown(b));

    private static async Task Settle()
    {
        for (var i = 0; i < 6; i++) await Task.Delay(100);
    }

    /// <summary>A real left click at the middle of the element.</summary>
    private static async Task Click(Window window, FrameworkElement element)
    {
        element.StartBringIntoView();
        await Settle();
        var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(window);
        var scale = element.XamlRoot.RasterizationScale;
        var origin = element.TransformToVisual(null).TransformPoint(new Windows.Foundation.Point(0, 0));
        var point = new POINT
        {
            X = (int)((origin.X + element.ActualWidth / 2) * scale),
            Y = (int)((origin.Y + element.ActualHeight / 2) * scale),
        };
        ClientToScreen(hwnd, ref point);
        SetForegroundWindow(hwnd);
        SetCursorPos(point.X, point.Y);
        mouse_event(0x0002, 0, 0, 0, 0); // left down
        mouse_event(0x0004, 0, 0, 0, 0); // left up
        await Settle();
    }

    private static async Task SaveAsync(UIElement element, string name)
    {
        var bitmap = new RenderTargetBitmap();
        await bitmap.RenderAsync(element);
        var pixels = await bitmap.GetPixelsAsync();
        var path = Path.Combine(Output, name);
        using var stream = new InMemoryRandomAccessStream();
        var encoder = await BitmapEncoder.CreateAsync(BitmapEncoder.PngEncoderId, stream);
        var scale = element.XamlRoot.RasterizationScale;
        encoder.SetPixelData(BitmapPixelFormat.Bgra8, BitmapAlphaMode.Premultiplied,
            (uint)bitmap.PixelWidth, (uint)bitmap.PixelHeight, 96 * scale, 96 * scale, pixels.ToArray());
        await encoder.FlushAsync();
        var bytes = new byte[stream.Size];
        stream.Seek(0);
        await stream.ReadAsync(bytes.AsBuffer(), (uint)bytes.Length, InputStreamOptions.None);
        await File.WriteAllBytesAsync(path, bytes);
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct POINT { public int X; public int Y; }

    [DllImport("user32.dll")] private static extern bool ClientToScreen(IntPtr hwnd, ref POINT point);
    [DllImport("user32.dll")] private static extern bool SetForegroundWindow(IntPtr hwnd);
    [DllImport("user32.dll")] private static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] private static extern void mouse_event(uint flags, uint dx, uint dy, uint data, UIntPtr extra);
}
