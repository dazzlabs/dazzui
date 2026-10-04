// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

using DazzUI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;

namespace Settings;

public sealed partial class SettingsPage : Page
{
    public SettingsViewModel ViewModel { get; }

    public SettingsPage(SettingsViewModel viewModel)
    {
        ViewModel = viewModel;
        InitializeComponent();
        Loaded += (_, _) =>
        {
            SetFace(Text1, "captionMedium");
            SetFace(Text2, "captionMedium");
        };
    }

    private void OnSetLaunchAtStartupClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.SetLaunchAtStartup(((ToggleButton)sender).IsChecked == true));

    private void OnSetAutoUpdateClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.SetAutoUpdate(((ToggleButton)sender).IsChecked == true));

    private void OnSetShowDisabledPluginsClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.SetShowDisabledPlugins(((ToggleButton)sender).IsChecked == true));

    private void OnTogglePluginClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.TogglePlugin(((Plugin)((FrameworkElement)sender).DataContext).Id));

    private void OnSaveClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.Save());

    private void OnResetClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.Reset());

    public static bool Not(bool value) => !value;

    public static Visibility Visible1(bool c0) => c0 ? Visibility.Visible : Visibility.Collapsed;

    public static Visibility Visible2(bool c0) => !c0 ? Visibility.Visible : Visibility.Collapsed;

    private void SetFace(TextBlock text, string token)
    {
        var face = Appearance.GetTheme(this).Text("", token);
        text.FontSize = face.FontSize;
        text.FontWeight = face.FontWeight;
        text.LineHeight = face.LineHeight;
        text.LineStackingStrategy = face.LineStackingStrategy;
    }
}
