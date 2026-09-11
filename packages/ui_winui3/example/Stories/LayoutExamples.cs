using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Playground.Stories;

public static partial class ComponentExamples
{
    public static UIElement ActionBar(UI.Theme t, string s) => Example(t, "ActionBar", s, "Align related actions at the start, end or opposite edges of a region.", o =>
    {
        var bar = Configure(new UI.ActionBar { Alignment = s == "Start" ? UI.ActionBarAlignment.Start : s == "Between" ? UI.ActionBarAlignment.Between : UI.ActionBarAlignment.End, Width = t.Number("spacing.20") * 5 }, o);
        bar.Items.Add(new UI.Button { Content = "Cancel", Variant = UI.ControlVariant.Recessed });
        if (s == "Mixed controls") bar.Items.Add(new UI.Switch { IsChecked = true });
        if (s == "With a key hint") bar.Items.Add(new UI.KeyCap("Ctrl+Enter"));
        bar.Items.Add(new UI.Button { Content = "Export", Variant = UI.ControlVariant.Filled }); return bar;
    });
    public static UIElement Accordion(UI.Theme t, string s) => Example(t, "Accordion", s, "Expandable sections that allow either one or several sections to remain open.", o =>
    {
        var accordion = new UI.Accordion { Multiple = s == "Multiple", Divided = s == "Divided", Width = t.Number("spacing.20") * 5 };
        foreach (var (title, description) in new[] { ("Where do the values come from?", "The shared design token graph defines colors, spacing and typography."), ("How many themes are there?", "Six families, each with light and dark appearances."), ("What keeps the platforms in line?", "The same tokens and component stories are used across platforms.") })
            accordion.Items.Add(new UI.AccordionItem { Header = title, Body = t.Text(description), IsEnabled = !o.Disabled, IsExpanded = title.StartsWith("Where") });
        return accordion;
    });
    public static UIElement Collapsible(UI.Theme t, string s) => Example(t, "Collapsible", s, "A disclosure that keeps secondary content available without showing it all the time.", o => new UI.Collapsible
    {
        Header = s == "With meta" ? "Advanced options · 3 settings" : "Advanced options",
        Body = Stage.Column(t, new UI.Checkbox { Content = "Keep comments", IsChecked = true }, new UI.Checkbox { Content = "Include source text" }),
        IsExpanded = s is "Open" or "Overview",
        IsEnabled = !o.Disabled,
        Width = t.Number("spacing.20") * 4,
    });
    public static UIElement Preferences(UI.Theme t, string s) => Example(t, "Preferences", s, "Settings grouped into sections, with explanatory labels and controls kept together.", o =>
    {
        var group = new UI.PreferenceGroup();
        group.Rows.Add(new UI.PreferenceRow { Label = "Automatic translation", Description = "Translate new documents when they are added.", Control = new UI.Switch { IsChecked = true } });
        group.Rows.Add(new UI.PreferenceRow { Label = "Target language", Control = LanguageSelect(o) });
        group.Rows.Add(new UI.PreferenceRow { Label = "Parallel requests", Control = new UI.NumberField { Value = 3, Minimum = 1, Maximum = 8 } });
        if (s == "A group") return group;
        var preferences = new UI.Preferences();
        preferences.Sections.Add(new UI.PreferenceSection { Title = s == "Unlabelled" ? "" : "Translation", Description = "Defaults for this workspace.", Body = group });
        if (s == "A settings page") preferences.Sections.Add(new UI.PreferenceSection { Title = "Keyboard", Body = new UI.PreferenceRow { Label = "Quick translate", Control = new UI.ShortcutRecorder { Shortcut = "Ctrl+Shift+T" } } });
        return preferences;
    });
    public static UIElement Sidebar(UI.Theme t, string s) => Example(t, "Sidebar", s, "A navigation column with optional header, footer, grouped content and collapse behavior.", o =>
    {
        var sidebar = MakeSidebar(t);
        sidebar.Height = t.Number("spacing.20") * 5;
        if (s == "With a header") sidebar.Header = new UI.SearchField { PlaceholderText = "Find a project" };
        if (s == "With a card") sidebar.Footer = new UI.Card(t.Text("Local workspace"), t);
        if (s == "With an action") sidebar.Footer = new UI.Button { Content = "New project" };
        if (s == "Collapsing") { sidebar.Collapsible = UI.SidebarCollapsible.Icons; return Stage.Column(t, Action("Toggle sidebar", () => sidebar.IsCollapsed = !sidebar.IsCollapsed), sidebar); }
        if (s == "Resizable") { var splitter = new UI.Splitter { Width = t.Number("spacing.20") * 7, Height = sidebar.Height }; splitter.Panels.Add(new(sidebar)); splitter.Panels.Add(new UI.SplitterPanel(t.Text("Project canvas"))); return splitter; }
        return sidebar;
    });
    private static UI.Sidebar MakeSidebar(UI.Theme t)
    {
        var sidebar = new UI.Sidebar();
        foreach (var section in new[] { "Workspace", "Projects" })
        {
            var group = new UI.SidebarGroup { Label = section };
            foreach (var label in section == "Workspace" ? new[] { "Overview", "Activity", "Settings" } : new[] { "Handbook", "Website", "Release notes" })
                group.Items.Add(new UI.NavItem { Content = label, IsChecked = label == "Overview" });
            sidebar.Items.Add(group);
        }
        return sidebar;
    }
    public static UIElement NavItem(UI.Theme t, string s) => Example(t, "NavItem", s, "A navigation row that distinguishes the current destination from an action.", o =>
    {
        if (s == "In a sidebar") return MakeSidebar(t);
        var row = Configure(new UI.NavItem { Content = s == "With an icon" ? Stage.Row(t, new UI.Icon { Glyph = UI.IconGlyph.Folder }, t.Text("Documents")) : "Documents", IsChecked = o.Selected, Width = t.Number("frame.sidebar.width") }, o);
        return row;
    });
    public static UIElement Aside(UI.Theme t, string s) => Example(t, "Aside", s, "An inspector or supporting column beside the main content.", o =>
    {
        var aside = new UI.Aside { Content = Stage.Column(t, new UI.SectionLabel("DOCUMENT"), t.Text("Product handbook", "titleSmall"), new UI.PreferenceRow { Label = "Language", Control = LanguageSelect(o) }) };
        if (s == "With an action") aside.Content = Stage.Column(t, t.Text("Selection", "titleSmall"), new UI.Button { Content = "Inspect" });
        if (s == "In a three-column window") return Stage.Row(t, MakeSidebar(t), new UI.Card(t.Text("Document canvas"), t), aside);
        return Stage.Row(t, new UI.Card(t.Text("Document canvas"), t), aside);
    });
    public static UIElement Rail(UI.Theme t, string s) => Example(t, "Rail", s, "A narrow column of icon actions for top-level destinations.", o =>
    {
        var rail = new UI.Rail();
        foreach (var glyph in new[] { UI.IconGlyph.Folder, UI.IconGlyph.Document, UI.IconGlyph.Person, UI.IconGlyph.Settings }) rail.Items.Add(new UI.IconButton { Glyph = glyph, Label = glyph.ToString(), Variant = UI.ControlVariant.Plain });
        if (s == "With an action") rail.Items.Add(new UI.IconButton { Glyph = UI.IconGlyph.Add, Label = "New project" });
        return s == "Beside a sidebar" ? Stage.Row(t, rail, MakeSidebar(t)) : rail;
    });
    public static UIElement Splitter(UI.Theme t, string s) => Example(t, "Splitter", s, "Resize adjacent panels with the pointer or arrow keys while respecting their limits.", o =>
    {
        var split = new UI.Splitter { Orientation = s == "Vertical" ? Orientation.Vertical : Orientation.Horizontal, Width = t.Number("spacing.20") * 6, Height = t.Number("spacing.20") * 4 };
        for (var i = 0; i < (s == "Three panels" ? 3 : 2); i++) split.Panels.Add(new(new UI.Card(t.Text($"Panel {i + 1}"), t), Minimum: s == "Constrained" ? t.Number("spacing.20") : t.Number("spacing.8")));
        if (s == "Nested") { var inner = new UI.Splitter { Orientation = Orientation.Vertical }; inner.Panels.Add(new(t.Text("Top"))); inner.Panels.Add(new(t.Text("Bottom"))); split.Panels[1] = new(inner); }
        return split;
    });
    public static UIElement Tree(UI.Theme t, string s) => Example(t, "Tree", s, "A hierarchical collection with native expansion, keyboard navigation and selection.", o =>
    {
        var tree = Configure(new UI.Tree { Width = t.Number("spacing.20") * 4 }, o);
        tree.Nodes.Add(new("project", "dazzui-ui", [new("src", "src", [new("button", "Button.cs"), new("theme", "Theme.cs")], Expanded: s != "Collapsed"), new("readme", "README.md", Meta: s == "With meta" ? "Modified" : null)], Expanded: s != "Collapsed"));
        return tree;
    });
    private static UI.Table SampleTable(UI.Theme t, UIElement? embedded = null, bool selectable = true, bool empty = false)
    {
        var table = new UI.Table { Width = t.Number("spacing.20") * 6, Selectable = selectable };
        table.Columns.Add(new("name", "Document", 3)); table.Columns.Add(new("status", "Status", 2)); table.Columns.Add(new("pages", "Pages", 1, TextAlignment.Right));
        if (!empty)
            foreach (var (name, index) in new[] { "Product handbook", "Release notes", "Invoice September" }.Select((name, index) => (name, index)))
                table.Rows.Add(new(name, new Dictionary<string, object?> { ["name"] = name, ["status"] = index == 0 && embedded is not null ? embedded : index == 2 ? "In progress" : "Ready", ["pages"] = (index + 1) * 4 }));
        return table;
    }
    public static UIElement Table(UI.Theme t, string s) => Example(t, "Table", s, "Aligned rows and columns, with optional selection and controls within cells.", o => Configure(SampleTable(t, s == "With controls" ? new UI.Checkbox { Content = "Reviewed" } : null, s != "Inert", s == "Empty"), o));
}
