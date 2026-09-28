using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Playground.Stories;

internal sealed record ExampleOptions(UI.ControlSize Size = UI.ControlSize.Medium,
    UI.WidgetTint Tint = UI.WidgetTint.Primary, UI.ControlVariant Variant = UI.ControlVariant.Recessed,
    bool Disabled = false, bool Selected = false, bool Invalid = false);

public static partial class ComponentExamples
{
    private static readonly UI.ControlSize[] Sizes = Enum.GetValues<UI.ControlSize>();
    private static readonly UI.WidgetTint[] Tints = Enum.GetValues<UI.WidgetTint>();
    private static readonly UI.ControlVariant[] Variants = Enum.GetValues<UI.ControlVariant>();
    private static UIElement Example(UI.Theme theme, string page, string story, string description,
        Func<ExampleOptions, UIElement> build, bool controlVariants = false)
    {
        var key = story.ToLowerInvariant();
        var options = new ExampleOptions(Size: page is "Button" or "ToggleGroup" ? UI.ControlSize.Small : page is "IconButton" or "Toggle" ? UI.ControlSize.Tiny : UI.ControlSize.Medium, Variant: controlVariants ? UI.ControlVariant.Filled : UI.ControlVariant.Recessed, Disabled: key == "disabled", Selected: key is "checked" or "selected" or "on" or "pressed" or "active", Invalid: key == "error state");
        if (Enum.TryParse<UI.ControlVariant>(story.Replace(" ", ""), true, out var variant)) options = options with { Variant = variant };
        if (Enum.TryParse<UI.WidgetTint>(story, true, out var tint)) options = options with { Tint = tint };
        UIElement content;
        if (key == "sizes") content = Stage.Row(theme, Sizes.Where(size => size != UI.ControlSize.Tiny || page is "Button" or "IconButton" or "Toggle" or "ToggleGroup").Select(size => build(options with { Size = size })).ToArray());
        else if (key is "tints" or "filled tints") content = Stage.Row(theme, Tints.Select(value => build(options with { Tint = value, Variant = key == "filled tints" ? UI.ControlVariant.Filled : options.Variant })).ToArray());
        else if (controlVariants && key is "variants" or "all variants") content = Stage.Row(theme, Variants.Select(value => build(options with { Variant = value })).ToArray());
        else if (key == "states") content = Stage.Row(theme, build(options), build(options with { Selected = true }), build(options with { Disabled = true }), build(options with { Invalid = true }));
        else content = build(options);
        if (key == "overview")
        {
            var axisStories = Registry.Pages.FirstOrDefault(entry => entry.Group == "Components" && entry.Name == page)?.Stories
                .Where(entry => entry.Name is "All variants" or "Variants" or "Sizes" or "Tints" or "States").ToArray() ?? [];
            var sections = axisStories.Select(entry => Gallery.Section(theme, entry.Name, entry.Build(theme))).ToArray();
            content = Gallery.Overview(theme, page, description, content, sections);
        }
        else if (key.Contains("settings") || key.Contains("preference"))
            content = new UI.PreferenceRow { Label = page, Description = "Choose how this workspace behaves.", Control = content, Width = theme.Number("preferences.width") };
        else if (key.Contains("form") && page is not "Form" and not "FormField" and not "Dialog" and not "Popover" and not "OtpField")
            content = Stage.Column(theme, new UI.FormField { Label = page, Hint = "This preference applies to the current workspace.", Field = content }, new UI.Button { Content = "Save changes", Variant = UI.ControlVariant.Filled, HorizontalAlignment = HorizontalAlignment.Left });
        else if ((key.Contains("toolbar") || key.Contains("action bar")) && page is not "ActionBar" and not "Menu" and not "Menubar")
        {
            var bar = new UI.ActionBar { Alignment = UI.ActionBarAlignment.Start }; bar.Items.Add(content); bar.Items.Add(new UI.Divider { Orientation = Orientation.Vertical, Height = theme.Number("control.medium.size") }); bar.Items.Add(new UI.Button { Content = "Done" }); content = bar;
        }
        else if ((key.Contains("pane") || key.Contains("panel") || key.Contains("card")) && page is not "Card" and not "Sidebar" and not "Splitter")
            content = new UI.Card(Stage.Column(theme, theme.Text("Workspace", "titleSmall"), content), theme);
        return new UI.ThemeScope { Theme = theme, Content = content };
    }
    private static T Configure<T>(T element, ExampleOptions options) where T : FrameworkElement
    {
        UI.Appearance.SetTint(element, options.Tint); UI.Appearance.SetSize(element, options.Size);
        if (element is Control control) control.IsEnabled = !options.Disabled;
        return element;
    }
    private static UI.TextField Input(ExampleOptions options, string text = "") => Configure(new UI.TextField { Text = text, PlaceholderText = "Enter a value", Invalid = options.Invalid, Width = UI.Theme.Default.Number("spacing.20") * 3 }, options);
    private static UI.Button Action(string label, Action action)
    {
        var button = new UI.Button { Content = label, HorizontalAlignment = HorizontalAlignment.Left };
        button.Click += (_, _) => action(); return button;
    }
    private static UIElement Caption(UI.Theme theme, string label, UIElement content) => Stage.Column(theme, theme.Text(label, "labelSmall"), content);
    private static UI.RadioGroup ChoiceGroup(UI.Theme theme, bool disabled = false)
    {
        var group = new UI.RadioGroup();
        foreach (var text in new[] { "Automatic", "Always", "Never" }) group.Items.Add(new UI.Radio { Content = text, Tag = text, IsChecked = text == "Automatic", IsEnabled = !disabled });
        return group;
    }
    private static UI.Select LanguageSelect(ExampleOptions options)
    {
        var select = Configure(new UI.Select { PlaceholderText = "Choose a language", Invalid = options.Invalid, Width = UI.Theme.Default.Number("spacing.20") * 3 }, options);
        select.SetOptions([new("en", "English", "Languages"), new("zh", "Chinese", "Languages"), new("fr", "French", "Languages"), new("auto", "Detect automatically", "Automatic")]);
        select.Value = "en"; return select;
    }
}
