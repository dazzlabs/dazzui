using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace Settings;

/// <summary>Runs the generated page against the Rust app core. Each snapshot
/// the page gets is printed, so a change can be seen to have come from Rust.</summary>
public partial class App : Application
{
    private Window? window;

    public App()
    {
        UnhandledException += (_, args) =>
        {
            File.WriteAllText(Path.Combine(SmokeTest.Output, "crash.txt"), args.Exception.ToString());
        };
        InitializeComponent();
    }

    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        var model = new SettingsViewModel();
        model.PropertyChanged += (_, _) => Console.WriteLine($"state: {model.State}");
        var page = new SettingsPage(model);
        var root = new ScrollViewer { Content = new Border { Padding = new Thickness(24), Child = page } };
        window = new Window { Title = "Settings", Content = root };
        root.Loaded += async (_, _) =>
        {
            var scale = root.XamlRoot.RasterizationScale;
            var work = DisplayArea.GetFromWindowId(window.AppWindow.Id, DisplayAreaFallback.Primary).WorkArea;
            window.AppWindow.Resize(new Windows.Graphics.SizeInt32(Math.Min((int)(720 * scale), work.Width), Math.Min((int)(820 * scale), work.Height)));
            if (Environment.GetCommandLineArgs().Contains("--smoke-test")) await SmokeTest.RunAsync(window, root, model);
        };
        window.Activate();
    }
}
