using DazzUI.Playground.Storybook;
using DazzUI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI.Playground;

public partial class App : Application
{
    private Window? window;
    public App()
    {
        UnhandledException += (_, args) =>
        {
            var directory = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_OUTPUT")
                ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "DazzUI", "Playground");
            Directory.CreateDirectory(directory);
            File.WriteAllText(Path.Combine(directory, "crash.txt"), args.Exception.ToString());
        };
        InitializeComponent();
    }

    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        window = new Window { Title = "DazzUI · WinUI 3 Playground" };
        // Initial window size is host policy, not a design-system metric.
        window.AppWindow.Resize(new Windows.Graphics.SizeInt32(1100, 760));
        var shell = new PlaygroundShell();
        window.Content = shell;
        shell.Loaded += async (_, _) =>
        {
            // Match the native playground's logical size while respecting the
            // display's DPI and work area (AppWindow.Resize uses physical pixels).
            var scale = shell.XamlRoot.RasterizationScale;
            var work = Microsoft.UI.Windowing.DisplayArea.GetFromWindowId(window.AppWindow.Id, Microsoft.UI.Windowing.DisplayAreaFallback.Primary).WorkArea;
            window.AppWindow.Resize(new Windows.Graphics.SizeInt32(Math.Min((int)(1180 * scale), work.Width), Math.Min((int)(780 * scale), work.Height)));
            shell.UpdateLayout();
            if (Environment.GetCommandLineArgs().Contains("--smoke-test")) await SmokeTest.RunAsync(window, shell);
        };
        window.Activate();
    }
}
