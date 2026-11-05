using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Documents;
using Microsoft.UI.Xaml.Media;

namespace DazzUI.Example;

/// <summary>Keep the same controls alive through palette, sizing and native state changes.</summary>
internal static class AppearanceTests
{
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }

    internal static async Task RunAsync(Shell shell, string output, bool comparePages = false)
    {
        var failures = new List<string>();
        void Check(bool condition, string message) { if (!condition) failures.Add(message); }
        var link = new Link { Content = "Documentation" };
        var check = new Checkbox { Content = "Remember", Note = "this device", IsChecked = true };
        var action = new Button { Content = "Save", Variant = ControlVariant.Filled };
        var iconAction = new Button
        {
            Content = new StackPanel { Orientation = Orientation.Horizontal, Children =
            {
                new Icon { Glyph = FluentIcons.Common.Icon.Save },
                new TextBlock { Text = "Save with icon", VerticalAlignment = VerticalAlignment.Center },
            } },
            Variant = ControlVariant.Filled,
        };
        var field = new TextField { Text = "Keep selection", Width = 240 };
        var formField = new FormField { Label = "Project", Field = field };
        var disabled = new TextField { Text = "Disabled", Width = 240, IsEnabled = false };
        var select = new Select { Width = 240 };
        var secure = new SecureTextField { Password = "Secret", Width = 240 };
        var autocomplete = new Autocomplete { Text = "Query", Width = 240 };
        var combobox = new Combobox { Text = "Query", Width = 240 };
        var number = new NumberField { Value = 42, Width = 240 };
        select.SetOptions([new SelectOption("one", "First option")]);
        select.SelectedIndex = 0;
        var panel = new StackPanel { Spacing = 12, Children = { link, check, action, iconAction, formField, disabled, select, secure, autocomplete, combobox, number } };
        var scope = new ThemeScope { Content = panel, HorizontalAlignment = HorizontalAlignment.Left };
        shell.ShowFixture(scope);
        await Task.Delay(80);
        shell.UpdateLayout();
        field.Select(2, 4);
        Check(field.Focus(FocusState.Programmatic), "The native editor could not receive focus.");
        foreach (var sizing in new[] { ThemeSizing.Design, ThemeSizing.System, ThemeSizing.Design })
        foreach (var name in new[] { "studio-light", "studio-dark", "windows11-light", "bright-dark", "bright-light" })
        {
            var theme = new Theme(name, sizing);
            scope.Theme = theme;
            await Task.Delay(40);
            shell.UpdateLayout();
            var context = $"{name}/{sizing}";
            scope.Background = theme.Brush("color.surface");
            foreach (var composite in new Surface[] { autocomplete, combobox, number })
            {
                var native = Walk(composite).OfType<TextBox>().First();
                foreach (var state in new[] { "PointerOver", "Focused", "Normal" })
                {
                    VisualStateManager.GoToState(native, state, false);
                    await Task.Delay(10);
                    var content = Walk(native).OfType<ScrollViewer>().First(item => item.Name == "ContentElement");
                    Check((content.Foreground as SolidColorBrush)?.Color == theme.Color("color.content"), $"{context}: {composite.GetType().Name} {state} kept native editor ink");
                    if (composite is NumberField)
                    {
                        var edge = Walk(native).OfType<Border>().First(item => item.Name == "BorderElement");
                        Check((edge.Background as SolidColorBrush)?.Color.A == 0, $"{context}: the numeric editor painted over its group in {state}");
                    }
                }
            }
            var linkText = Walk(link).OfType<TextBlock>().First(text => text.Text == "Documentation");
            Check(Math.Abs(linkText.FontSize - link.FontSize) < .001, $"{context}: link retained font {linkText.FontSize}, expected {link.FontSize}");
            Check(Math.Abs(linkText.LineHeight - theme.Text("", "bodyMedium").LineHeight) < .001, $"{context}: link retained its old leading");
            foreach (var (state, recipe) in new[] { ("PointerOver", "hovered"), ("Pressed", "pressed"), ("Disabled", "disabled"), ("Normal", "normal") })
            {
                VisualStateManager.GoToState(link, state, false);
                await Task.Delay(10);
                Check((linkText.Foreground as SolidColorBrush)?.Color == theme.Recipe(ControlVariant.Plain, link.Tint, "content", recipe).Color, $"{context}: link {state} has the wrong ink");
                Check(Math.Abs(linkText.FontSize - link.FontSize) < .001, $"{context}: link {state} changed its font size");
            }
            var noteText = (TextBlock)check.Content;
            IEnumerable<Run> Runs(IEnumerable<Inline> inlines) => inlines.SelectMany(inline => inline is Span span ? Runs(span.Inlines) : inline is Run run ? [run] : Array.Empty<Run>());
            var note = Runs(noteText.Inlines).Last();
            Check((note.Foreground as SolidColorBrush)?.Color == theme.Color("color.content-faint"), $"{context}: checkbox note retained its old ink");
            Check(Math.Abs(noteText.LineHeight - theme.Text("", "labelQuiet").LineHeight) < .001, $"{context}: checkbox note retained its old leading");
            Check(field.Text == "Keep selection" && field.SelectionStart == 2 && field.SelectionLength == 4, $"{context}: native selection changed");
            Check(field.FocusState != FocusState.Unfocused, $"{context}: the form editor lost focus");
            formField.SetError("Keep editing");
            formField.SetError(null);
            Check(field.FocusState != FocusState.Unfocused && field.SelectionStart == 2 && field.SelectionLength == 4, $"{context}: validation detached the native editor");
            foreach (var enabled in new[] { false, true, false })
            {
                disabled.IsEnabled = enabled;
                await Task.Delay(10);
                var disabledBorder = Walk(disabled).OfType<Border>().First(item => item.Name == "BorderElement");
                var expected = enabled ? theme.Color("color.surface-muted") : theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "surface", "disabled").Color;
                Check((disabledBorder.Background as SolidColorBrush)?.Color == expected, $"{context}: changing enabled={enabled} retained an old field surface");
                var editor = Walk(disabled).OfType<ScrollViewer>().First(item => item.Name == "ContentElement");
                var ink = enabled ? theme.Color("color.content") : theme.Recipe(ControlVariant.Normal, WidgetTint.Neutral, "content", "disabled").Color;
                Check((editor.Foreground as SolidColorBrush)?.Color == ink, $"{context}: changing enabled={enabled} retained old editor ink");
            }
            foreach (var button in new[] { action, iconAction })
            {
                var label = Walk(button).OfType<TextBlock>().First(text => text.Text.StartsWith("Save", StringComparison.Ordinal));
                Check(Math.Abs(label.FontSize - button.FontSize) < .001, $"{context}: button label size differs from its control");
                VisualStateManager.GoToState(button, "PointerOver", false);
                await Task.Delay(10);
                var presenter = Walk(button).OfType<ContentPresenter>().First(item => item.Name == "Presenter");
                Check((presenter.Foreground as SolidColorBrush)?.Color == theme.Recipe(button.Variant, button.Tint, "content", "hovered").Color, $"{context}: button hover retained its old ink");
                Check(Math.Abs(label.FontSize - button.FontSize) < .001, $"{context}: hovering changed button label size");
            }
            VisualStateManager.GoToState(field, "Focused", false);
            await Task.Delay(10);
            var border = Walk(field).OfType<Border>().First(item => item.Name == "BorderElement");
            Check((border.Background as SolidColorBrush)?.Color == theme.Color("color.surface"), $"{context}: focused field retained its old surface ({(border.Background as SolidColorBrush)?.Color})");
            var fieldEditor = Walk(field).OfType<ScrollViewer>().First(item => item.Name == "ContentElement");
            Check((fieldEditor.Foreground as SolidColorBrush)?.Color == theme.Color("color.content"), $"{context}: focused editor has the wrong ink");
            foreach (var state in new[] { "PointerOver", "Pressed", "Focused", "Normal" })
            {
                VisualStateManager.GoToState(select, state, false);
                VisualStateManager.GoToState(secure.Input, state == "Pressed" ? "PointerOver" : state, false);
                await Task.Delay(10);
                var value = Walk(select).OfType<ContentPresenter>().First(item => item.Name == "ContentPresenter");
                Check((value.Foreground as SolidColorBrush)?.Color == theme.Color("color.content"), $"{context}: select {state} has the wrong ink");
                var chevron = Walk(select).OfType<IconElement>().First(item => item.Name == "DropDownGlyph");
                Check((chevron.Foreground as SolidColorBrush)?.Color == theme.Color("color.content-subtle"), $"{context}: select {state} kept native chevron ink");
                var password = Walk(secure.Input).OfType<Control>().First(item => item.Name == "ContentElement");
                Check((password.Foreground as SolidColorBrush)?.Color == theme.Color("color.content"), $"{context}: password {state} has the wrong ink");
            }
            var highlight = Walk(select).OfType<Border>().First(item => item.Name == "HighlightBackground");
            Check((highlight.BorderBrush as SolidColorBrush)?.Color == ((SolidColorBrush)select.FocusVisualPrimaryBrush).Color, $"{context}: select kept a native focus border");
            var pill = Walk(select).OfType<Microsoft.UI.Xaml.Shapes.Rectangle>().First(item => item.Name == "Pill");
            Check((pill.Fill as SolidColorBrush)?.Color == theme.Color("color.primary.600"), $"{context}: select kept the Windows accent");
            field.Invalid = true;
            await Task.Delay(10);
            Check((fieldEditor.Foreground as SolidColorBrush)?.Color == theme.Color("color.danger.800"), $"{context}: focused invalid editor lost its danger ink");
            Check((border.Background as SolidColorBrush)?.Color == theme.Color("color.danger.50"), $"{context}: focused invalid editor lost its danger wash");
            field.Invalid = false;
            await SmokeTest.SaveAsync(scope, output, $"appearance-{name}-{sizing}.png");
        }
        var pages = Environment.GetEnvironmentVariable("DAZZUI_SMOKE_PAGES")?.Split(',', StringSplitOptions.RemoveEmptyEntries);
        var comparedPages = 0;
        foreach (var story in Stories.All.Where(story => comparePages && (pages is null || pages.Contains(story.Id))))
        {
            comparedPages++;
            File.WriteAllText(Path.Combine(output, "progress.txt"), $"Live appearance: {story.Id}");
            try
            {
                shell.ChooseSizing(ThemeSizing.Design);
                shell.Choose(new ThemeChoice("bright"));
                shell.Open(story.Id);
                await Settle(shell);
                foreach (var (choice, sizing) in new[]
                {
                    (new ThemeChoice("windows11"), ThemeSizing.System),
                    (new ThemeChoice("studio", true), ThemeSizing.System),
                    (new ThemeChoice("bright"), ThemeSizing.Design),
                })
                {
                    shell.ChooseSizing(sizing);
                    shell.Choose(choice);
                    await Settle(shell);
                    var changed = TextSnapshot(shell.CurrentPage!);
                    shell.Open(story.Id);
                    await Settle(shell);
                    var fresh = TextSnapshot(shell.CurrentPage!);
                    Check(changed.SequenceEqual(fresh), $"{story.Id}/{choice.Id}/{sizing}: live text differs from a fresh page:\n" +
                        string.Join('\n', changed.Except(fresh).Select(text => "live: " + text).Concat(fresh.Except(changed).Select(text => "fresh: " + text))));
                }
            }
            catch (Exception error)
            {
                failures.Add($"{story.Id}: {error}");
                shell.ShowFixture(new Grid());
                await Settle(shell);
            }
            File.WriteAllLines(Path.Combine(output, "appearance.txt"), failures);
        }
        File.WriteAllLines(Path.Combine(output, "appearance.txt"), failures.Count == 0 ?
            [$"PASS: live typography, leading, palette, hover/focus and native selection; {comparedPages} gallery pages compared with fresh rendering."] : failures);
        if (failures.Count > 0) throw new InvalidOperationException(string.Join('\n', failures.Distinct().Take(40)));
    }

    private static async Task Settle(Shell shell)
    {
        await Task.Delay(40);
        shell.UpdateLayout();
        // A theme can change scroll anchoring. Compare the same viewport on
        // both pages, after nested templates have handled their Loaded events.
        if (shell.CurrentPage is DependencyObject page)
            foreach (var scroll in Walk(page).OfType<ScrollViewer>()) scroll.ChangeView(0, 0, null, true);
        for (var pass = 0; pass < 3; pass++)
        {
            await Task.Delay(40);
            shell.UpdateLayout();
        }
    }

    private static bool Visible(TextBlock text, DependencyObject root)
    {
        if (root is not FrameworkElement frame || text.ActualWidth <= 0 || text.ActualHeight <= 0) return false;
        for (DependencyObject? current = text; current is not null; current = VisualTreeHelper.GetParent(current))
        {
            if (current is UIElement visual && (visual.Visibility == Visibility.Collapsed || visual.Opacity == 0)) return false;
            if (current is FrameworkElement ancestor && current != text)
            {
                var clip = ancestor.Clip is RectangleGeometry geometry ? geometry.Rect
                    : new Windows.Foundation.Rect(0, 0, ancestor.ActualWidth, ancestor.ActualHeight);
                var inside = text.TransformToVisual(ancestor).TransformBounds(new Windows.Foundation.Rect(0, 0, text.ActualWidth, text.ActualHeight));
                if (inside.Right <= clip.Left || inside.Bottom <= clip.Top || inside.Left >= clip.Right || inside.Top >= clip.Bottom) return false;
            }
            if (ReferenceEquals(current, root)) break;
        }
        var box = text.TransformToVisual(frame).TransformBounds(new Windows.Foundation.Rect(0, 0, text.ActualWidth, text.ActualHeight));
        return box.Right > 0 && box.Bottom > 0 && box.Left < frame.ActualWidth && box.Top < frame.ActualHeight;
    }

    private static string[] TextSnapshot(DependencyObject root) => Walk(root).OfType<TextBlock>()
        .Where(text => !string.IsNullOrEmpty(text.Text) && Visible(text, root))
        .Select(text => $"{text.Text}: {text.FontFamily.Source}, {text.FontSize:0.###}, {text.FontWeight.Weight}, {text.LineHeight:0.###}, {(text.Foreground as SolidColorBrush)?.Color}, scale={text.IsTextScaleFactorEnabled}")
        .OrderBy(text => text, StringComparer.Ordinal).ToArray();
}
