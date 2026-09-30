using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;

namespace DazzUI.Example;

public partial class App : Application
{
    private Window? window;

    public App()
    {
        UnhandledException += (_, args) =>
        {
            var directory = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_OUTPUT")
                ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "DazzUI", "Example");
            Directory.CreateDirectory(directory);
            File.WriteAllText(Path.Combine(directory, "crash.txt"), args.Exception.ToString());
        };
        InitializeComponent();
    }

    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        var arguments = Environment.GetCommandLineArgs().Skip(1).ToArray();
        if (arguments.Contains("--conformance"))
        {
            // The collector draws scenes on their own, not in the window's shell.
            var host = new Microsoft.UI.Xaml.Controls.ContentControl
            {
                HorizontalAlignment = HorizontalAlignment.Left,
                VerticalAlignment = VerticalAlignment.Top,
                HorizontalContentAlignment = HorizontalAlignment.Left,
                VerticalContentAlignment = VerticalAlignment.Top,
            };
            window = new Window { Title = "DazzUI — conformance", Content = host };
            host.Loaded += async (_, _) =>
            {
                var scale = host.XamlRoot.RasterizationScale;
                var work = DisplayArea.GetFromWindowId(window.AppWindow.Id, DisplayAreaFallback.Primary).WorkArea;
                window.AppWindow.Resize(new Windows.Graphics.SizeInt32(Math.Min((int)(1024 * scale), work.Width), work.Height));
                await Collector.RunAsync(window, host);
            };
            window.Activate();
            return;
        }
        var (page, theme) = Location.Read(arguments);
        window = new Window { Title = "DazzUI — WinUI 3 example" };
        var shell = new Shell(page, theme);
        window.Content = shell;
        if (window.AppWindow.Presenter is OverlappedPresenter presenter)
        {
            // Logical sizes, as the other examples open at.
            presenter.PreferredMinimumWidth = 960;
            presenter.PreferredMinimumHeight = 600;
        }
        shell.Loaded += async (_, _) =>
        {
            // AppWindow sizes are physical pixels: scale, and keep to the work area.
            var scale = shell.XamlRoot.RasterizationScale;
            var work = DisplayArea.GetFromWindowId(window.AppWindow.Id, DisplayAreaFallback.Primary).WorkArea;
            window.AppWindow.Resize(new Windows.Graphics.SizeInt32(Math.Min((int)(1360 * scale), work.Width), Math.Min((int)(860 * scale), work.Height)));
            shell.UpdateLayout();
            if (arguments.Contains("--smoke-test")) await SmokeTest.RunAsync(window, shell);
        };
        window.Activate();
    }
}
