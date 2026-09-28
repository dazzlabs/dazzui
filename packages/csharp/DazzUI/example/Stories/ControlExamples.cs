using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Playground.Stories;

public static partial class ComponentExamples
{
    public static UIElement TextField(UI.Theme t, string s) => Example(t, "TextField", s, "Text entry with placeholder, validation, read-only and multiline states.", o =>
    {
        var input = Input(o, s is "Placeholder" or "Default" ? "" : "Quarterly report");
        input.Mono = s.Contains("mono", StringComparison.OrdinalIgnoreCase);
        input.IsReadOnly = s.Contains("read-only", StringComparison.OrdinalIgnoreCase);
        input.AcceptsReturn = s.Contains("Multiline", StringComparison.OrdinalIgnoreCase);
        if (input.AcceptsReturn) { input.Text = "Keep terminology consistent.\nPreserve the document layout.\nReview names before exporting."; input.MinHeight = t.Number("spacing.20"); }
        if (s == "Beside a button") return Stage.Row(t, input, new UI.Button { Content = "Save" });
        return input;
    });
    public static UIElement SearchField(UI.Theme t, string s) => Example(t, "SearchField", s, "Find content in a collection. Escape clears the current query.", o =>
    {
        var search = Configure(new UI.SearchField { Text = s == "With a query" ? "invoice" : "", Hint = s == "With a hint" ? "Ctrl+F" : null, Width = t.Number("spacing.20") * 3 }, o);
        if (s != "Filtering a list") return search;
        var results = new StackPanel();
        void Refresh() { results.Children.Clear(); foreach (var name in new[] { "Annual report", "Invoice September", "Invoice August", "Product handbook" }.Where(name => name.Contains(search.Text, StringComparison.OrdinalIgnoreCase))) results.Children.Add(t.Text(name)); }
        search.TextChanged += (_, _) => Refresh(); Refresh(); return Stage.Column(t, search, results);
    });
    public static UIElement Select(UI.Theme t, string s) => Example(t, "Select", s, "Choose one option from a keyboard-accessible list, with grouped and disabled options.", o => LanguageSelect(o));
    public static UIElement Autocomplete(UI.Theme t, string s) => Example(t, "Autocomplete", s, "Type freely while suggestions help complete a known value.", o =>
    {
        var input = Configure(new UI.Autocomplete { PlaceholderText = "Search terminology", Mono = s == "Mono", Invalid = o.Invalid, Width = t.Number("spacing.20") * 3 }, o);
        foreach (var value in new[] { "translation", "translation memory", "translator", "glossary", "terminology", "document" }) input.Suggestions.Add(value);
        return input;
    });
    public static UIElement Combobox(UI.Theme t, string s) => Example(t, "Combobox", s, "Search a collection and select an option without losing keyboard navigation.", o =>
    {
        var input = Configure(new UI.Combobox { PlaceholderText = "Choose a provider", Invalid = o.Invalid, Width = t.Number("spacing.20") * 3 }, o);
        foreach (var value in new[] { "OpenAI", "Anthropic", "Google", "Microsoft", "DeepL", "Local model" }) input.Suggestions.Add(value);
        if (s == "Grouped")
            foreach (var value in input.Suggestions) input.Options.Add(new(value, value, value == "Local model" ? "On device" : "Cloud providers"));
        return input;
    });
    public static UIElement NumberField(UI.Theme t, string s) => Example(t, "NumberField", s, "A bounded numeric value with direct editing, arrow keys and step buttons.", o => Configure(new UI.NumberField { Value = 12, Minimum = 1, Maximum = 100, SmallChange = 1, LargeChange = 10, Unit = s == "With unit" ? "pages" : null, Invalid = o.Invalid, Width = t.Number("spacing.20") * 2 }, o));
    public static UIElement OtpField(UI.Theme t, string s) => Example(t, "OtpField", s, "Enter a verification code. Pasting distributes digits and keyboard navigation moves between positions.", o =>
    {
        var otp = Configure(new UI.OtpField { Length = s == "Grouped" ? 8 : 6, GroupSize = s == "Grouped" ? 4 : 0, Masked = s == "Masked" }, o);
        if (s == "Filled") otp.Loaded += (_, _) => otp.Value = "123456";
        var status = t.Text("Enter the code sent to your device.", "bodySmall");
        otp.Completed += (_, code) => status.Text = $"Code entered: {code}";
        return Stage.Column(t, otp, status);
    });
    public static UIElement FormField(UI.Theme t, string s) => Example(t, "FormField", s, "A label, description and validation message associated with an input.", o =>
    {
        if (s == "A full form") return Form(t, "Bare");
        UIElement field = s == "Over a select" ? LanguageSelect(o) : s == "Over any control" ? new UI.Switch() : s == "With a field value" ? new UI.FieldValue { Text = "Workspace default" } : Input(o);
        if (s == "Over a multiline field" && field is UI.TextField input) { input.AcceptsReturn = true; input.MinHeight = t.Number("spacing.20"); }
        return new UI.FormField { Label = "Document name", Hint = s == "Without a hint" ? null : "Choose a name you can recognize later.", Error = o.Invalid ? "A document name is required." : null, Required = true, Field = field };
    });
    public static UIElement Form(UI.Theme t, string s) => Example(t, "Form", s, "Group editable fields, validate their values and submit only when they are ready.", o =>
    {
        var name = Input(o); var field = new UI.FormField { Label = "Name", Required = true, Field = name };
        var form = new UI.Form(); form.Fields.Add(field); form.Fields.Add(new UI.FormField { Label = "Language", Field = LanguageSelect(o) });
        var status = t.Text("", "bodySmall");
        form.Validate = () => string.IsNullOrWhiteSpace(name.Text) ? new Dictionary<string, string> { ["Name"] = "Enter a name." } : s == "Server errors" ? new Dictionary<string, string> { ["Name"] = "This name is already in use." } : new Dictionary<string, string>();
        form.Submitted += (_, _) => status.Text = "Settings saved.";
        form.Footer = Stage.Row(t, Action("Save", () => form.Submit()), status);
        if (s == "With fieldsets") { var set = new UI.Fieldset { Legend = "Workspace settings" }; set.Fields.Add(form); return set; }
        return form;
    });
    public static UIElement Checkbox(UI.Theme t, string s) => Example(t, "Checkbox", s, "An independent choice, including checked, unchecked and mixed states.", o =>
    {
        if (s is "A group" or "A checklist")
        {
            var group = new UI.CheckboxGroup();
            foreach (var label in new[] { "Keep formatting", "Include comments", "Use glossary" }) group.Items.Add(new UI.Checkbox { Content = label, IsChecked = label != "Include comments" });
            return group;
        }
        var checkbox = Configure(new UI.Checkbox { Content = s == "With a note" ? "Include images (2.4 MB)" : "Keep original formatting", IsChecked = o.Selected || s == "Overview" }, o);
        if (s == "Interactive") { var status = t.Text("Unchecked"); checkbox.Checked += (_, _) => status.Text = "Checked"; checkbox.Unchecked += (_, _) => status.Text = "Unchecked"; return Stage.Column(t, checkbox, status); }
        return checkbox;
    });
    public static UIElement Radio(UI.Theme t, string s) => Example(t, "Radio", s, "One choice within a named group. Arrow keys move among enabled options.", o => s is "A group" or "In a form" ? ChoiceGroup(t, o.Disabled) : Configure(new UI.Radio { Content = "Translate automatically", IsChecked = o.Selected || s == "Overview" }, o));
    public static UIElement Switch(UI.Theme t, string s) => Example(t, "Switch", s, "A setting that takes effect immediately, with a clear on and off position.", o =>
    {
        var toggle = Configure(new UI.Switch { IsChecked = o.Selected || s == "Overview" }, o);
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(toggle, "Automatic translation");
        if (s == "Against a checkbox") return Stage.Row(t, toggle, new UI.Checkbox { Content = "Automatic translation" });
        if (s == "Interactive") { var status = t.Text("Off"); toggle.Checked += (_, _) => status.Text = "On"; toggle.Unchecked += (_, _) => status.Text = "Off"; return Stage.Row(t, toggle, status); }
        return toggle;
    });
    public static UIElement Toggle(UI.Theme t, string s) => Example(t, "Toggle", s, "A button that retains its selected state, for formatting and filters.", o => Configure(new UI.Toggle { Content = "Bold", IsChecked = o.Selected, Variant = o.Variant }, o), true);
    public static UIElement ToggleGroup(UI.Theme t, string s) => Example(t, "ToggleGroup", s, "Related toggle actions with single or multiple selection and arrow-key navigation.", o =>
    {
        var group = new UI.ToggleGroup { Multiple = s != "Single" };
        foreach (var label in new[] { "Bold", "Italic", "Underline" }) group.Items.Add(Configure(new UI.Toggle { Content = label, Variant = o.Variant, IsChecked = label == "Bold" }, o));
        return group;
    }, true);
    public static UIElement SegmentedControl(UI.Theme t, string s) => Example(t, "SegmentedControl", s, "A compact choice among closely related views, in a raised or filled selection style.", o =>
    {
        var segments = Configure(new UI.SegmentedControl { Filled = s is "Filled" or "Filled tints", SelectedIndex = 0 }, o);
        foreach (var label in s == "Two segments" ? new[] { "Source", "Translation" } : new[] { "All", "Active", "Archived" }) segments.Items.Add(label);
        segments.SelectedIndex = 0; return segments;
    });
    public static UIElement PillTabs(UI.Theme t, string s) => Example(t, "PillTabs", s, "Switch between sibling views, with optional counts.", o =>
    {
        var tabs = Configure(new UI.PillTabs(), o);
        foreach (var label in s == "With counts" ? new[] { "All 24", "Active 8", "Archived 16" } : new[] { "Overview", "Activity", "Settings" }) tabs.Items.Add(label);
        tabs.SelectedIndex = 0; return tabs;
    });
    public static UIElement OptionCard(UI.Theme t, string s) => Example(t, "OptionCard", s, "A radio choice with enough room to explain what each option means.", o =>
    {
        UI.OptionCard Make(string name) => Configure(new UI.OptionCard { Title = name, Description = s == "Without a description" ? "" : "Keep the original document structure.", IsChecked = o.Selected || name == "DOCX", GroupName = "export-format" }, o);
        return s is "A group" or "In a grid" ? Stage.Row(t, Make("DOCX"), Make("PDF"), Make("Markdown")) : Make("DOCX");
    });
    public static UIElement Tag(UI.Theme t, string s) => Example(t, "Tag", s, "A selectable or removable label for filters and chosen items.", o =>
    {
        var tag = Configure(new UI.Tag { Text = "English", Removable = s is "Removable" or "A filter bar", IsSelected = true }, o);
        tag.Removed += (_, _) => tag.Visibility = Visibility.Collapsed;
        return s == "A filter bar" ? Stage.Row(t, tag, new UI.Tag { Text = "Reviewed", IsSelected = true }) : tag;
    });
    public static UIElement ShortcutRecorder(UI.Theme t, string s) => Example(t, "ShortcutRecorder", s, "Record a keyboard shortcut, clear it with Backspace, or cancel with Escape.", o => Configure(new UI.ShortcutRecorder
    {
        Shortcut = s == "Unbound" ? "" : s == "Word keys" ? "Ctrl+Enter" : s == "Modified" ? "Ctrl+Shift+K" : "Ctrl+K",
        Validate = s == "Conflicts" ? value => value == "Ctrl+S" ? "Already used by Save" : null : null,
    }, o));
}
