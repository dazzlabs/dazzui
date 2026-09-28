using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Playground.Stories;

public static partial class ComponentExamples
{
    public static UIElement Button(UI.Theme t, string s) => Example(t, "Button", s, "Six control variants, four sizes and six tints. Use one filled action to give a view a clear next step.", o =>
    {
        var button = new UI.Button { Content = s is "All variants" or "Normal" or "Recessed" ? o.Variant.ToString() : "Save changes", Theme = t, Tint = o.Tint, Size = o.Size, Variant = s == "Destructive" ? UI.ControlVariant.Filled : o.Variant, IsEnabled = !o.Disabled && s != "Busy" };
        if (s == "Destructive") { button.Tint = UI.WidgetTint.Danger; button.Content = "Delete glossary"; }
        if (s == "With an icon") button.Content = Stage.Row(t, new UI.Icon { Glyph = UI.IconGlyph.ArrowDownload }, t.Text("Export", "labelMedium"));
        if (s == "With a shortcut") button.Content = Stage.Row(t, t.Text("Translate", "labelMedium"), new UI.KeyCap("Ctrl+S"));
        if (s == "Busy") button.Content = Stage.Row(t, new UI.Spinner { Size = UI.ControlSize.Small }, t.Text("Translating…"));
        if (s == "Full width") { button.HorizontalAlignment = HorizontalAlignment.Stretch; button.Width = t.Number("spacing.20") * 4; }
        button.Click += (_, _) => button.Content = "Saved";
        return button;
    }, true);
    public static UIElement Badge(UI.Theme t, string s) => Example(t, "Badge", s, "A compact, non-interactive annotation. Its tint conveys status without suggesting an action.", o =>
    {
        UI.Badge Make(UI.BadgeVariant variant) => Configure(new UI.Badge(s == "Counts" ? "128" : "Ready", t, o.Tint) { Variant = variant }, o);
        if (s == "All variants") return Stage.Row(t, Enum.GetValues<UI.BadgeVariant>().Select(v => (UIElement)Make(v)).ToArray());
        var badge = Make(Enum.TryParse<UI.BadgeVariant>(s, true, out var v) ? v : UI.BadgeVariant.Tinted);
        if (s == "With a glyph") badge.Content = Stage.Row(t, new UI.Icon { Glyph = UI.IconGlyph.Checkmark }, t.Text("Ready", "labelSmall"));
        if (s == "In a table") return SampleTable(t, badge);
        return badge;
    });
    public static UIElement Card(UI.Theme t, string s) => Example(t, "Card", s, "A surface around related content. Raised, sunken, outlined and tinted cards share one corner and spacing vocabulary.", o =>
    {
        UI.Card Make(UI.CardVariant variant) => Configure(new UI.Card(Stage.Column(t, t.Text("Project glossary", "titleSmall"), t.Text("Shared terminology for your workspace.")), t) { Variant = variant }, o);
        if (s == "Variants") return Stage.Row(t, Enum.GetValues<UI.CardVariant>().Select(v => (UIElement)Make(v)).ToArray());
        if (s == "As a grid") return Stage.Row(t, Make(UI.CardVariant.Raised), Make(UI.CardVariant.Sunken), Make(UI.CardVariant.Outlined));
        var card = Make(Enum.TryParse<UI.CardVariant>(s, true, out var variant) ? variant : UI.CardVariant.Sunken);
        if (s == "With an action") card.Content = Stage.Column(t, t.Text("Glossary", "titleSmall"), new UI.Button { Content = "Open glossary" });
        if (s == "Nested") card.Content = Make(UI.CardVariant.Raised);
        if (s == "As a well") card.Content = new UI.EmptyState { Title = "No files yet", Description = "Drop a document here to begin." };
        return card;
    });
    public static UIElement Avatar(UI.Theme t, string s) => Example(t, "Avatar", s, "A person or team represented by initials or an image, with a fallback when an image cannot load.", o =>
    {
        UI.Avatar Make(string name) => Configure(new UI.Avatar { NameText = name, Shape = s == "Square" ? UI.AvatarShape.Square : UI.AvatarShape.Circle }, o);
        if (s == "Groups") { var group = new UI.AvatarGroup(); foreach (var name in new[] { "Avery Chen", "Sam Lee", "Morgan Kim" }) group.Avatars.Add(Make(name)); return group; }
        var avatar = Make("Avery Chen");
        if (s == "Image") avatar.Source = new Uri("ms-appx:///Assets/avatar.png");
        return avatar;
    });
    public static UIElement Callout(UI.Theme t, string s) => Example(t, "Callout", s, "Context that helps a person understand the current state or decide what to do next.", o => Configure(new UI.Callout
    {
        Title = s == "Without a title" ? null : "Translation memory is syncing",
        Description = "Your documents remain available while the latest terms are downloaded.",
        Icon = s == "With an icon" ? new UI.Icon { Glyph = UI.IconGlyph.Info } : null,
        Action = s == "With an action" ? Action("Retry", () => { }) : null,
    }, o));
    public static UIElement Divider(UI.Theme t, string s) => Example(t, "Divider", s, "A quiet rule between related regions. Its tone and direction follow the layout.", o =>
    {
        if (s == "Tones") return Stage.Column(t, new UI.Divider { Width = t.Number("spacing.20") * 3 }, new UI.Divider { Muted = true });
        if (s == "Vertical") return Stage.Row(t, t.Text("Source"), new UI.Divider { Orientation = Orientation.Vertical, Height = t.Number("spacing.8") }, t.Text("Translation"));
        return Stage.Column(t, t.Text("Original document"), new UI.Divider { Muted = s == "Muted", Width = t.Number("spacing.20") * 3 }, t.Text("Translated document"));
    });
    public static UIElement SectionLabel(UI.Theme t, string s) => Example(t, "SectionLabel", s, "A small heading that groups rows without competing with the content.", o => Configure(new UI.SectionLabel("WORKSPACE"), o));
    public static UIElement Code(UI.Theme t, string s) => Example(t, "Code", s, "A short code fragment that remains readable inside prose.", o => s == "In prose" ? Stage.Row(t, t.Text("Install with"), new UI.Code("pnpm install"), t.Text("then open the playground.")) : new UI.Code("dazzui.translate()"));
    public static UIElement CodeBlock(UI.Theme t, string s) => Example(t, "CodeBlock", s, "A selectable code sample with an optional heading, line numbers and a copy action.", o => new UI.CodeBlock
    {
        Text = "const job = await translate({\n  source: 'English',\n  target: 'Chinese',\n  preserveLayout: true,\n})\nconsole.log(job.status)",
        Title = s is "With head" or "With actions" ? "translate.ts" : null,
        LineNumbers = s == "Line numbers",
        Wrap = s == "Wrapped",
        Copyable = s is "Copyable" or "Overview" or "With actions",
        Actions = s == "With actions" ? new UI.Button { Content = "Run sample" } : null,
    });
    public static UIElement KeyCap(UI.Theme t, string s) => Example(t, "KeyCap", s, "A keyboard key or chord printed as a compact hint.", o =>
    {
        if (s.Contains("chord", StringComparison.OrdinalIgnoreCase)) return Stage.Row(t, new UI.KeyCap("Ctrl"), new UI.KeyCap("Shift"), new UI.KeyCap("K"));
        if (s == "All variants") return Stage.Row(t, Enum.GetValues<UI.KeyCapVariant>().Select(variant => (UIElement)new UI.KeyCap("Ctrl+K") { Variant = variant }).ToArray());
        return Configure(new UI.KeyCap(s == "Word keys" ? "Backspace" : "Ctrl+K") { Variant = Enum.TryParse<UI.KeyCapVariant>(s, out var variant) ? variant : UI.KeyCapVariant.Hint }, o);
    });
    public static UIElement Icon(UI.Theme t, string s) => Example(t, "Icon", s, "Fluent System Icons, shared with the React and other native libraries.", o =>
    {
        if (s == "A set") return Stage.Row(t, Enum.GetValues<UI.IconGlyph>().Select(g => (UIElement)new UI.Icon { Glyph = g }).ToArray());
        var icon = Configure(new UI.Icon { Glyph = UI.IconGlyph.Folder, Foreground = s == "Inherited colour" ? t.Recipe(UI.ControlVariant.Plain, UI.WidgetTint.Success, "content") : null }, o);
        if (s == "In a button") return new UI.Button { Content = icon };
        return icon;
    });
    public static UIElement IconButton(UI.Theme t, string s) => Example(t, "IconButton", s, "A compact action with an accessible label and a tooltip.", o => new UI.IconButton { Glyph = UI.IconGlyph.Settings, Label = "Settings", Size = o.Size, Tint = o.Tint, Variant = o.Variant, IsEnabled = !o.Disabled }, true);
    public static UIElement Link(UI.Theme t, string s) => Example(t, "Link", s, "A text link that navigates or opens a related destination.", o => Configure(new UI.Link { Content = s == "External" ? "DazzUI documentation ↗" : "View documentation", NavigateUri = new Uri("https://dazzui.dev") }, o));
    public static UIElement EmptyState(UI.Theme t, string s) => Example(t, "EmptyState", s, "Explain why a region is empty and offer a useful next action.", o => new UI.EmptyState
    {
        Title = s == "Nothing matched" ? "No matching documents" : "No documents yet",
        Description = s == "Nothing matched" ? "Try a different search." : "Add a document to start translating.",
        Actions = s == "With two actions" ? Stage.Row(t, new UI.Button { Content = "Import" }, new UI.Button { Content = "New document" }) : s == "With an action" ? new UI.Button { Content = "Import document" } : null,
    });
}
