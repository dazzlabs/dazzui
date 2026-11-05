using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Input;
using UI = DazzUI;

namespace DazzUI.Example;

internal static class PopupParityTests
{
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }

    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }

    private static async Task Settle(Shell shell)
    {
        await Task.Delay(100);
        shell.UpdateLayout();
    }

    private static async Task Press(Shell shell, byte key)
    {
        NativeKeyboard.Press(shell, key);
        await Settle(shell);
    }

    private static string FocusDescription(Shell shell, UI.Menu menu) =>
        $"focused={FocusManager.GetFocusedElement(shell.XamlRoot)?.GetType().Name ?? "null"}, states={string.Join(",", menu.Flyout.Items.Select((item, index) => $"{index}:{item.FocusState}"))}";

    private static async Task NavigateTo(Shell shell, UI.Menu menu, int index, string theme)
    {
        // Drive the native presenter's own navigation index, not just XAML
        // focus on a row. A focus request can succeed before the presenter
        // finishes choosing its first keyboard destination.
        await Press(shell, 0x24);
        Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), menu.Flyout.Items[0]),
            $"{theme}: Home did not establish focus on the first native menu row ({FocusDescription(shell, menu)}).");
        for (var step = 1; step <= index; step++)
        {
            await Press(shell, 0x28);
            Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), menu.Flyout.Items[step]),
                $"{theme}: Down did not reach native menu row {step} ({FocusDescription(shell, menu)}).");
        }
    }

    public static async Task RunAsync(Shell shell, string output)
    {
        foreach (var name in new[] { "studio-light", "studio-dark", "bright-light", "bright-dark" })
        {
            var theme = new UI.Theme(name);
            var anchor = new UI.Button { Theme = theme, Content = "Open popup", HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
            var activated = 0;
            var checkedActions = 0;
            var uncheckedActions = 0;
            var disabledActions = 0;
            var nestedActions = 0;
            var menu = new UI.Menu();
            menu.Items.Add(new("New file", () => activated++, Shortcut: "Ctrl+N"));
            menu.Items.Add(new("Open folder", Glyph: FluentIcons.Common.Icon.Folder));
            menu.Items.Add(new("Selected", () => checkedActions++, Checked: true));
            menu.Items.Add(new("Unselected", () => uncheckedActions++, Checked: false));
            menu.Items.Add(new("Unavailable", () => disabledActions++, Disabled: true));
            menu.Items.Add(new("After disabled"));
            menu.Items.Add(new("More", Children: [new UI.MenuItem("Nested action", () => nestedActions++)]));
            menu.Items.Add(new("Unavailable selected", Checked: true, Disabled: true));
            shell.ShowFixture(anchor);
            await Settle(shell);
            await NativeKeyboard.ActivateAsync(shell);
            menu.Show(anchor);
            await Settle(shell);
            try
            {
                Check(menu.Flyout.IsOpen, $"{name}: native menu did not open.");
                var presenter = VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
                    .SelectMany(popup => Walk(popup.Child)).OfType<MenuFlyoutPresenter>().Single();
                Check(Math.Abs(presenter.Padding.Left - theme.Number("spacing.1")) < .01
                    && Math.Abs(presenter.BorderThickness.Left - theme.Number("stroke.hairline")) < .01,
                    $"{name}: popup sheet did not use shared padding and border.");
                Check(UI.Elevation.GetShadow(presenter) == "lg", $"{name}: popup lost the shared large shadow.");

                foreach (var item in menu.Flyout.Items)
                {
                    var label = UI.PopupSkin.Part<TextBlock>(item, "TextBlock")
                        ?? throw new InvalidOperationException($"{name}: native item has no painted label.");
                    var iconHeight = item is MenuFlyoutItem { Icon: not null } ? theme.Number("icon.medium") : 0;
                    var checkHeight = item is ToggleMenuFlyoutItem ? theme.Number("icon.small") : 0;
                    var expectedHeight = Math.Max(label.FontSize, Math.Max(iconHeight, checkHeight)) + 2 * theme.Number("menu.item-padding");
                    Check(Math.Abs(item.ActualHeight - expectedHeight) < .5,
                        $"{name}: popup row measured {item.ActualHeight}, expected {expectedHeight}.");
                    Check(Math.Abs(label.LineHeight - label.FontSize) < .01 && label.FontWeight == theme.Text("", "labelQuiet").FontWeight,
                        $"{name}: popup label lost quiet weight or solid leading.");
                    Check(Math.Abs(item.CornerRadius.TopLeft - Math.Min(theme.Number("radius.large") - theme.Number("spacing.1"), item.ActualHeight / 2)) < .01,
                        $"{name}: popup highlight corner is not concentric with the sheet.");
                    Check(item.Background is SolidColorBrush { Color.A: 0 },
                        $"{name}: resting menu rows paint a surface over the popup sheet.");
                    if (!item.IsEnabled)
                        Check(((SolidColorBrush)label.Foreground).Color == theme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "content", "disabled").Color,
                            $"{name}: disabled menu text did not use the normal disabled recipe.");
                }

                var checkedMark = UI.PopupSkin.Part<ContentPresenter>(menu.Flyout.Items[2], "CheckGlyph")!;
                var uncheckedMark = UI.PopupSkin.Part<ContentPresenter>(menu.Flyout.Items[3], "CheckGlyph")!;
                Check(checkedMark.Opacity == 1 && uncheckedMark.Opacity == 0 && checkedMark.ActualWidth == uncheckedMark.ActualWidth,
                    $"{name}: checkable rows did not retain an aligned check column.");
                var disabledMark = UI.PopupSkin.Part<ContentPresenter>(menu.Flyout.Items[7], "CheckGlyph")!;
                Check(disabledMark.Opacity == 0,
                    $"{name}: more than the first checked item rendered a radio check.");
                Check(disabledMark.Content is UI.Icon mark && ((SolidColorBrush)mark.Foreground).Color == theme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "content", "disabled").Color,
                    $"{name}: disabled checkmark kept the enabled ink.");
                var plainLabel = UI.PopupSkin.Part<TextBlock>(menu.Flyout.Items[0], "TextBlock")!;
                var plainOrigin = plainLabel.TransformToVisual(menu.Flyout.Items[0]).TransformPoint(new Windows.Foundation.Point());
                Check(Math.Abs(plainOrigin.X - theme.Number("spacing.2")) < .5,
                    $"{name}: an ordinary row borrowed another row's check/icon placeholder.");
                var shortcut = UI.PopupSkin.Part<TextBlock>(menu.Flyout.Items[0], "KeyboardAcceleratorTextBlock")!;
                Check(shortcut.Text == "Ctrl+N" && ((SolidColorBrush)shortcut.Foreground).Color == theme.Color("color.content-faint"),
                    $"{name}: shortcut hint lost its text or faint ink.");
                await NativeSnapshot.SaveAsync(presenter, output, name + "-popup-rows.png");

                await NavigateTo(shell, menu, 3, name);
                await Press(shell, 0x28);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), menu.Flyout.Items[4]),
                    $"{name}: native Down traversal skipped the disabled row instead of keeping React's focusable disabled option ({FocusDescription(shell, menu)}).");
                var disabledRow = UI.PopupSkin.Part<Grid>(menu.Flyout.Items[4], "LayoutRoot")!;
                Check(((SolidColorBrush)disabledRow.Background).Color == theme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "surface", "hovered").Color,
                    $"{name}: the disabled keyboard destination has no neutral highlight.");
                await Press(shell, 0x0D);
                Check(menu.Flyout.IsOpen && disabledActions == 0,
                    $"{name}: confirming a disabled row invoked its action or dismissed the menu (open={menu.Flyout.IsOpen}, actions={disabledActions}, {FocusDescription(shell, menu)}).");
                await Press(shell, 0x20);
                Check(menu.Flyout.IsOpen && disabledActions == 0,
                    $"{name}: Space on a disabled row invoked its action or dismissed the menu (open={menu.Flyout.IsOpen}, actions={disabledActions}).");
                await Press(shell, 0x28);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), menu.Flyout.Items[5]),
                    $"{name}: native Down traversal could not leave the disabled row ({FocusDescription(shell, menu)}).");
                var focusedRow = UI.PopupSkin.Part<Grid>(menu.Flyout.Items[5], "LayoutRoot")!;
                Check(((SolidColorBrush)focusedRow.Background).Color == theme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "surface", "hovered").Color,
                    $"{name}: the keyboard destination has no neutral highlight.");
                await Press(shell, 0x23);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), menu.Flyout.Items[7]),
                    $"{name}: End did not reach the last focusable disabled row ({FocusDescription(shell, menu)}).");
                await Press(shell, 0x26);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), menu.Flyout.Items[6]),
                    $"{name}: native Up did not continue from the End destination ({FocusDescription(shell, menu)}).");
                var selected = (ToggleMenuFlyoutItem)menu.Flyout.Items[2];
                await NavigateTo(shell, menu, 2, name);
                await Press(shell, 0x0D);
                Check(selected.IsChecked && checkedActions == 0 && !menu.Flyout.IsOpen,
                    $"{name}: reactivating the selected radio row changed its state, repeated its action or lost dismissal.");

                menu.Show(anchor);
                await Settle(shell);
                var proposal = (ToggleMenuFlyoutItem)menu.Flyout.Items[3];
                await NavigateTo(shell, menu, 3, name);
                await Press(shell, 0x0D);
                Check(!proposal.IsChecked && uncheckedActions == 1 && !menu.Flyout.IsOpen,
                    $"{name}: a radio proposal did not invoke once or changed the caller-owned selection.");
                menu.Show(anchor);
                await Settle(shell);
                Check(((ToggleMenuFlyoutItem)menu.Flyout.Items[2]).IsChecked && !((ToggleMenuFlyoutItem)menu.Flyout.Items[3]).IsChecked,
                    $"{name}: reopening a refused radio proposal changed selection.");
                menu.Flyout.Hide();
                await Settle(shell);
                menu.Items[2] = menu.Items[2] with { Checked = false };
                menu.Items[3] = menu.Items[3] with { Checked = true };

                menu.Show(anchor);
                await Settle(shell);
                Check(!((ToggleMenuFlyoutItem)menu.Flyout.Items[2]).IsChecked && ((ToggleMenuFlyoutItem)menu.Flyout.Items[3]).IsChecked,
                    $"{name}: reopening ignored the caller's updated radio selection.");
                var branch = (MenuFlyoutSubItem)menu.Flyout.Items[6];
                await NavigateTo(shell, menu, 6, name);
                await Press(shell, 0x27);
                var nested = branch.Items[0];
                Check(nested.IsLoaded && nested.ActualHeight > 0,
                    $"{name}: Right did not open the native submenu.");
                var nestedPeer = FrameworkElementAutomationPeer.CreatePeerForElement(nested);
                ((IInvokeProvider)nestedPeer!.GetPattern(PatternInterface.Invoke)!).Invoke();
                await Settle(shell);
                Check(nestedActions == 1, $"{name}: native submenu action was lost.");

                menu.Show(anchor);
                await Settle(shell);
                var peer = FrameworkElementAutomationPeer.CreatePeerForElement(menu.Flyout.Items[0]);
                ((IInvokeProvider)peer!.GetPattern(PatternInterface.Invoke)!).Invoke();
                await Settle(shell);
                Check(activated == 1, $"{name}: styling replaced native menu activation.");

                menu.Items[3] = menu.Items[3] with { Checked = false };
                menu.Show(anchor);
                await Settle(shell);
                disabledMark = UI.PopupSkin.Part<ContentPresenter>(menu.Flyout.Items[7], "CheckGlyph")!;
                Check(disabledMark.Opacity == 1 && disabledMark.Content is UI.Icon disabledGlyph
                    && ((SolidColorBrush)disabledGlyph.Foreground).Color == theme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "content", "disabled").Color,
                    $"{name}: the selected disabled radio check did not paint disabled ink.");
                await NavigateTo(shell, menu, 3, name);
                var liveRow = menu.Flyout.Items[3];
                var livePresenter = VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
                    .SelectMany(popup => Walk(popup.Child)).OfType<MenuFlyoutPresenter>()
                    .Single(p => p.Items.Contains(liveRow));
                var nextTheme = new UI.Theme(theme.IsDark ? "bright-light" : "studio-dark");
                anchor.Theme = nextTheme;
                await Settle(shell);
                Check(ReferenceEquals(menu.Flyout.Items[3], liveRow)
                    && ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), liveRow),
                    $"{name}: live menu theming replaced a row or lost its keyboard destination.");
                Check(((SolidColorBrush)livePresenter.Background).Color == nextTheme.Color("color.surface-overlay")
                    && ((SolidColorBrush)liveRow.Foreground).Color == nextTheme.Color("color.content"),
                    $"{name}: open menu did not follow its owner's live palette.");
                var liveRoot = UI.PopupSkin.Part<Grid>(liveRow, "LayoutRoot")!;
                Check(((SolidColorBrush)liveRoot.Background).Color == nextTheme.Recipe(UI.ControlVariant.Normal, UI.WidgetTint.Neutral, "surface", "hovered").Color,
                    $"{name}: live menu kept its previous focused-row brush.");
                await Press(shell, 0x28);
                Check(ReferenceEquals(FocusManager.GetFocusedElement(shell.XamlRoot), menu.Flyout.Items[4]),
                    $"{name}: native navigation stopped working after live menu theming.");
            }
            finally
            {
                menu.Flyout.Hide();
                await Settle(shell);
            }
        }
    }
}
