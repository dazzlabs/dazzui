// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

using DazzUI;
using System;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Controls.Primitives;

namespace SettingsExample;

public sealed partial class SettingsPage : Page
{
    public static readonly DependencyProperty StateProperty = DependencyProperty.Register(nameof(State), typeof(SettingsState), typeof(SettingsPage), new PropertyMetadata(null));

    /// <summary>What the page shows: set a new snapshot to show it.</summary>
    public SettingsState State
    {
        get => (SettingsState)GetValue(StateProperty);
        set => SetValue(StateProperty, value);
    }

    /// <summary>Whose methods the page's events call.</summary>
    public ISettingsActions? Model { get; set; }

    public SettingsPage()
    {
        InitializeComponent();
    }

    private void OnSetLaunchAtStartupClick(object sender, RoutedEventArgs e) => Model?.SetLaunchAtStartup(((ToggleButton)sender).IsChecked == true);

    private void OnSetAutoUpdateClick(object sender, RoutedEventArgs e) => Model?.SetAutoUpdate(((ToggleButton)sender).IsChecked == true);

    private void OnSetShowDisabledPluginsClick(object sender, RoutedEventArgs e) => Model?.SetShowDisabledPlugins(((ToggleButton)sender).IsChecked == true);

    private void OnTogglePluginClick(object sender, RoutedEventArgs e) => Model?.TogglePlugin(ItemOf<Plugin>(sender).Id);

    private void OnSaveClick(object sender, RoutedEventArgs e) => Model?.Save();

    private void OnResetClick(object sender, RoutedEventArgs e) => Model?.Reset();

    /// <summary>The element of the ItemsRepeater around <paramref name="sender"/> it was made for.</summary>
    private static T ItemOf<T>(object sender)
    {
        var child = (DependencyObject)sender;
        for (var parent = VisualTreeHelper.GetParent(child); parent is not null; child = parent, parent = VisualTreeHelper.GetParent(parent))
        {
            if (parent is ItemsRepeater repeater && child is UIElement element)
                return (T)repeater.ItemsSourceView.GetAt(repeater.GetElementIndex(element));
        }
        throw new InvalidOperationException("not inside an ItemsRepeater");
    }

    public static bool Not(bool value) => !value;

    public static Visibility Visible1(bool c0) => c0 ? Visibility.Visible : Visibility.Collapsed;

    public static Visibility Visible2(bool c0) => !c0 ? Visibility.Visible : Visibility.Collapsed;

    private void OnGap6Loaded(object sender, RoutedEventArgs e) => ((StackPanel)sender).Spacing = Appearance.GetTheme(this).Number("spacing.6");

    private void OnGap2Loaded(object sender, RoutedEventArgs e) => ((StackPanel)sender).Spacing = Appearance.GetTheme(this).Number("spacing.2");

    private void OnRepeaterGap2Loaded(object sender, RoutedEventArgs e) => ((StackLayout)((ItemsRepeater)sender).Layout).Spacing = Appearance.GetTheme(this).Number("spacing.2");

    private void OnText1Loaded(object sender, RoutedEventArgs e)
    {
        var text = (TextBlock)sender;
        var theme = Appearance.GetTheme(this);
        text.Foreground = theme.Brush("color.content-secondary");
    }
}
