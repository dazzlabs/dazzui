using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI.Example;

internal static class OverlayParityTests
{
    internal static async Task RunAsync(Shell shell, string output)
    {
        static void Check(bool condition, string message)
        {
            if (!condition) throw new InvalidOperationException(message);
        }
        var trigger = new Button { Content = "Details" };
        var scope = new ThemeScope { Content = trigger, Padding = new Thickness(40), HorizontalAlignment = HorizontalAlignment.Left };
        shell.ShowFixture(scope);
        await Task.Delay(100); shell.UpdateLayout();
        await NativeKeyboard.ActivateAsync(shell);
        Tooltip.Set(trigger, "Keyboard shortcut");
        var hint = (ToolTip)ToolTipService.GetToolTip(trigger);
        foreach (var name in new[] { "bright-light", "studio-dark", "graphite-light" })
        {
            var theme = scope.Theme = new Theme(name);
            hint.IsOpen = true;
            await Task.Delay(100); shell.UpdateLayout();
            var words = (TextBlock)hint.Content;
            var expected = theme.Text("", "labelSmall");
            Check(((SolidColorBrush)hint.Background).Color == theme.Color("color.surface-raised"), $"{name}: hint lost raised background");
            Check(((SolidColorBrush)hint.BorderBrush).Color == theme.Color("color.border") && hint.BorderThickness.Left == theme.Number("stroke.hairline"), $"{name}: hint border differs from React");
            Check(words.FontSize == expected.FontSize && words.LineHeight == expected.LineHeight && words.FontFamily.Source == expected.FontFamily.Source, $"{name}: hint lost label-small typography");
            Check(hint.Padding.Left == theme.Number("spacing.2") && hint.Padding.Top == theme.Number("spacing.1") + theme.Number("spacing.0.5"), $"{name}: hint has native default padding");
            Check(!hint.IsHitTestVisible && Elevation.GetShadow(hint) == "sm", $"{name}: hint intercepts input or has no shared shadow");
            Check(Math.Abs(hint.CornerRadius.TopLeft - Math.Min(theme.Number("radius.small"), Math.Min(hint.ActualWidth, hint.ActualHeight) / 2)) < .01,
                $"{name}: hint corner did not clamp to its painted box: radius={hint.CornerRadius.TopLeft}, box={hint.ActualWidth}×{hint.ActualHeight}");
            hint.IsOpen = false;

            var text = theme.Text("The complete popup includes its padding and border.", "bodySmall");
            var popover = new Popover { Content = text, Width = 240 };
            popover.Show(trigger);
            await Task.Delay(150); shell.UpdateLayout();
            var presenter = VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
                .SelectMany(popup => OverlaySkin.Walk(popup.Child)).OfType<FlyoutPresenter>()
                .Single(p => ReferenceEquals(p.Content, text));
            Check(Math.Abs(presenter.ActualWidth - 240) < .5, $"{name}: popup width is not its border box ({presenter.ActualWidth})");
            Check(presenter.Padding == new Thickness(theme.Number("spacing.4") + theme.Number("spacing.0.5")), $"{name}: popup retained native padding");
            Check(presenter.MinHeight == 0 && presenter.BorderThickness.Left == theme.Number("stroke.hairline"), $"{name}: popup retained native height floor or border");
            Check(Elevation.GetShadow(presenter) == "lg", $"{name}: popup has no shared shadow");
            await NativeSnapshot.SaveAsync(presenter, output, $"popover-{name}.png");
            popover.Hide();
            await Task.Delay(100);
        }

        hint.IsOpen = true;
        await Task.Delay(60);
        scope.Theme = new Theme("bright-dark");
        await Task.Delay(80); shell.UpdateLayout();
        Check(((SolidColorBrush)hint.Background).Color == scope.Theme.Color("color.surface-raised"), "An open hint retained its old palette");
        hint.IsOpen = false;

        var menubar = new Menubar();
        var file = new Menu(); file.Items.Add(new("New file"));
        file.Items.Add(new("Recent", Children: [new MenuItem("Document")]));
        var edit = new Menu(); edit.Items.Add(new("Copy"));
        menubar.Menus.Add(("File", file)); menubar.Menus.Add(("Edit", edit));
        scope.Content = menubar;
        await Task.Delay(80); shell.UpdateLayout();
        var original = OverlaySkin.Walk(menubar).OfType<MenubarItem>().ToArray();
        Check(original.Length == 2, $"Menubar created {original.Length} triggers instead of two");
        Check(original[0].Focus(FocusState.Keyboard), $"Menubar's initial tab stop refused native focus: loaded={original[0].IsLoaded}, visible={original[0].Visibility}, tabStop={original[0].IsTabStop}, box={original[0].ActualWidth}×{original[0].ActualHeight}");
        foreach (var (key, index) in new[] { ((byte)0x27, 1), ((byte)0x24, 0), ((byte)0x23, 1) })
        {
            NativeKeyboard.Press(shell, key);
            await Task.Delay(80); shell.UpdateLayout();
            Check(ReferenceEquals(Microsoft.UI.Xaml.Input.FocusManager.GetFocusedElement(shell.XamlRoot), original[index]) && original[index].IsTabStop && original.Count(item => item.IsTabStop) == 1,
                $"Menubar key 0x{key:X2} did not move its single native tab stop to {index}: focused={Microsoft.UI.Xaml.Input.FocusManager.GetFocusedElement(shell.XamlRoot)?.GetType().Name}, stops=[{string.Join(", ", original.Select(item => item.IsTabStop))}]");
        }
        foreach (var name in new[] { "studio-dark", "bright-light", "bright-dark" })
        {
            scope.Theme = new Theme(name);
            await Task.Delay(80); shell.UpdateLayout();
            var current = OverlaySkin.Walk(menubar).OfType<MenubarItem>().ToArray();
            Check(current.Length == 2 && current.Zip(original).All(pair => ReferenceEquals(pair.First, pair.Second)), $"{name}: theming rebuilt menu triggers and their flyout subscriptions");
            Check(ReferenceEquals(Microsoft.UI.Xaml.Input.FocusManager.GetFocusedElement(shell.XamlRoot), original[1]) && original[1].IsTabStop && !original[0].IsTabStop, $"{name}: theming reset the menubar's keyboard destination");
        }
        async Task Key(byte key)
        {
            NativeKeyboard.Press(shell, key);
            await Task.Delay(100); shell.UpdateLayout();
            var focused = Microsoft.UI.Xaml.Input.FocusManager.GetFocusedElement(shell.XamlRoot);
            await File.AppendAllTextAsync(Path.Combine(output, "menubar-keys.txt"),
                $"key={key:X2}, file={file.Flyout.IsOpen}, edit={edit.Flyout.IsOpen}, focus={focused?.GetType().Name}, fileRow={file.Flyout.Items.IndexOf(focused as MenuFlyoutItemBase)}, editRow={edit.Flyout.Items.IndexOf(focused as MenuFlyoutItemBase)}\n");
        }
        await Key(0x28);
        Check(edit.Flyout.IsOpen, "Down did not open the focused menubar trigger");
        await Key(0x27);
        Check(file.Flyout.IsOpen && !edit.Flyout.IsOpen && original[0].IsTabStop,
            "Right inside the last menu did not wrap to the first menu");
        await Key(0x27);
        Check(edit.Flyout.IsOpen && !file.Flyout.IsOpen && original[1].IsTabStop,
            "Right inside a root menu row did not move to the next menu");
        await Key(0x25);
        Check(file.Flyout.IsOpen && !edit.Flyout.IsOpen,
            "Left inside a root menu row did not move to the previous menu");
        await Key(0x23);
        await Key(0x27);
        var branch = (MenuFlyoutSubItem)file.Flyout.Items[1];
        Check(branch.Items[0].IsLoaded && file.Flyout.IsOpen && !edit.Flyout.IsOpen,
            "Menubar traversal stole Right from the native submenu");
        await Key(0x25);
        Check(file.Flyout.IsOpen && !edit.Flyout.IsOpen,
            "Left from a submenu changed the top-level menu");
        await Key(0x1B);
        Check(!file.Flyout.IsOpen && ReferenceEquals(Microsoft.UI.Xaml.Input.FocusManager.GetFocusedElement(shell.XamlRoot), original[0]),
            "Escape did not return focus to the traversed menu's trigger");
        await Key(0x26);
        Check(file.Flyout.IsOpen && ReferenceEquals(Microsoft.UI.Xaml.Input.FocusManager.GetFocusedElement(shell.XamlRoot), file.Flyout.Items[1]),
            "Up from a menubar trigger did not open at its last row");
        await Key(0x1B);
        using (var pointer = new NativePointer())
        {
            foreach (var item in original)
                item.PointerEntered += (_, _) => File.AppendAllText(Path.Combine(output, "menubar-keys.txt"), $"pointer entered {item.Content}\n");
            pointer.MoveTo(original[0]);
            await Key(0x28);
            pointer.MoveTo(original[1]);
            await Task.Delay(160); shell.UpdateLayout();
            Check(edit.Flyout.IsOpen && !file.Flyout.IsOpen,
                "Moving the pointer to an adjacent menubar trigger required a second click");
            pointer.MoveTo(original[0]);
            await Task.Delay(160); shell.UpdateLayout();
            Check(file.Flyout.IsOpen && !edit.Flyout.IsOpen,
                "Moving the pointer back did not switch the open menu");
            await Key(0x1B);
        }

        var leave = new Button { Content = "After preview" };
        var firstInput = new TextField { Text = "Keep selection", Width = 180 };
        var secondInput = new TextField { Text = "Second input", Width = 180 };
        var previewBody = new StackPanel { Spacing = 8, Children = { firstInput, secondInput } };
        var preview = new PreviewCard { Content = previewBody, Width = 240 };
        scope.Content = new StackPanel { Children = { trigger, leave } };
        preview.Attach(trigger);
        await Task.Delay(80); shell.UpdateLayout();
        preview.Show(trigger);
        await Task.Delay(100); shell.UpdateLayout();
        Check(firstInput.Focus(FocusState.Keyboard), "Preview editor could not receive native focus");
        firstInput.Select(2, 4);
        Check(secondInput.Focus(FocusState.Keyboard), "Focus could not move between preview descendants");
        var previewPresenter = VisualTreeHelper.GetOpenPopupsForXamlRoot(shell.XamlRoot)
            .SelectMany(popup => OverlaySkin.Walk(popup.Child)).OfType<FlyoutPresenter>()
            .Single(p => ReferenceEquals(p.Content, previewBody));
        scope.Theme = new Theme("studio-dark");
        await Task.Delay((int)scope.Theme.Milliseconds("preview-card.close-delay") + 80);
        Check(preview.IsOpen && ReferenceEquals(Microsoft.UI.Xaml.Input.FocusManager.GetFocusedElement(shell.XamlRoot), secondInput), "Moving focus within the preview dismissed it");
        Check(((SolidColorBrush)previewPresenter.Background).Color == scope.Theme.Color("color.surface-overlay"),
            "An open preview retained its old palette after the owner's theme changed");
        Check(firstInput.SelectionStart == 2 && firstInput.SelectionLength == 4, "Preview focus movement reset native editor selection");
        leave.Focus(FocusState.Keyboard);
        await Task.Delay((int)scope.Theme.Milliseconds("preview-card.close-delay") + 120);
        Check(!preview.IsOpen, "Preview remained open after focus left its trigger and complete panel");
    }
}
