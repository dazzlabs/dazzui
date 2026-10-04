// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace Settings;

public sealed partial class SettingsPage : Page
{
    public SettingsViewModel ViewModel { get; }

    public SettingsPage(SettingsViewModel viewModel)
    {
        ViewModel = viewModel;
        InitializeComponent();
    }

    private void OnSaveClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.Save());

    private void OnResetClick(object sender, RoutedEventArgs e) => ViewModel.Send(new SettingsAction.Reset());
}
