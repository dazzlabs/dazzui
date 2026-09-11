using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using UI = DazzUI;

namespace DazzUI.Playground.Stories;

public static partial class ComponentExamples
{
    private static UI.Menu SampleMenu(bool checks = false, bool disabled = false)
    {
        var menu = new UI.Menu();
        menu.Items.Add(new("Open", Shortcut: "Ctrl+O")); menu.Items.Add(new("Export", Shortcut: "Ctrl+E"));
        menu.Items.Add(new("", Separator: true));
        menu.Items.Add(new("Show source", Checked: checks ? true : null));
        menu.Items.Add(new("Advanced", Children: [new("Copy identifier"), new("Reveal in folder")]));
        menu.Items.Add(new("Delete", Disabled: disabled)); return menu;
    }
    public static UIElement Menu(UI.Theme t, string s) => Example(t, "Menu", s, "Actions in a popup with native keyboard navigation, checks and nested submenus.", o =>
    {
        var menu = SampleMenu(s == "Checked", s == "Disabled");
        var button = s == "From an icon button" ? new UI.IconButton { Glyph = UI.IconGlyph.MoreHorizontal, Label = "Document actions" } : new UI.Button { Content = "Document actions" };
        button.Click += (_, _) => menu.Show(button); return button;
    });
    public static UIElement ContextMenu(UI.Theme t, string s) => Example(t, "ContextMenu", s, "Contextual actions available with a secondary click or the keyboard menu key.", o =>
    {
        var menu = new UI.ContextMenu(); foreach (var item in SampleMenu(s == "Checked").Items) menu.Items.Add(item);
        var target = new UI.Button { Content = "Right-click this document", Variant = UI.ControlVariant.Normal };
        target.Loaded += (_, _) => menu.Attach(target); return target;
    });
    public static UIElement Menubar(UI.Theme t, string s) => Example(t, "Menubar", s, "Top-level action menus for a document or workspace.", o =>
    {
        var bar = new UI.Menubar();
        foreach (var label in new[] { "File", "Edit", "View" }) bar.Menus.Add((label, SampleMenu(s == "With checks", o.Disabled)));
        return Configure(bar, o);
    });
    public static UIElement Popover(UI.Theme t, string s) => Example(t, "Popover", s, "A lightweight anchored panel that dismisses when focus moves away.", o =>
    {
        UI.Button Make(FlyoutPlacementMode side)
        {
            var button = new UI.Button { Content = side.ToString() };
            var popover = new UI.Popover { Placement = side, Content = s is "A form" or "With controls" ? Stage.Column(t, new UI.FormField { Label = "Document name", Field = Input(o) }, new UI.Checkbox { Content = "Keep formatting" }) : Stage.Column(t, t.Text("Document details", "titleSmall"), t.Text("12 pages · English → Chinese")) };
            button.Click += (_, _) => popover.Show(button); return button;
        }
        return s == "Sides" ? Stage.Row(t, Make(FlyoutPlacementMode.Top), Make(FlyoutPlacementMode.Bottom), Make(FlyoutPlacementMode.Left), Make(FlyoutPlacementMode.Right)) : Make(FlyoutPlacementMode.Bottom);
    });
    public static UIElement PreviewCard(UI.Theme t, string s) => Example(t, "PreviewCard", s, "Preview a destination on hover or keyboard focus before opening it.", o =>
    {
        var link = new UI.Link { Content = s == "A repository" ? "dazzlabs/dazzui" : "Product handbook" };
        var preview = new UI.PreviewCard { Content = Stage.Column(t, t.Text("Product handbook", "titleSmall"), t.Text("The shared source of truth for product terminology.")) };
        preview.Attach(link); return link;
    });
    public static UIElement Tooltip(UI.Theme t, string s) => Example(t, "Tooltip", s, "A concise explanation attached to an action, available on hover and focus.", o =>
    {
        if (s == "Sides") return Stage.Row(t, new[] { PlacementMode.Top, PlacementMode.Bottom, PlacementMode.Left, PlacementMode.Right }.Select(side =>
        {
            var anchor = new UI.Button { Content = side.ToString() };
            anchor.Loaded += (_, _) => UI.Tooltip.Set(anchor, $"Tooltip on the {side.ToString().ToLowerInvariant()}", side);
            return (UIElement)anchor;
        }).ToArray());
        var button = new UI.IconButton { Glyph = UI.IconGlyph.ArrowDownload, Label = s == "A long label" ? "Export the translated document with the original formatting preserved" : "Export document" };
        button.Loaded += (_, _) => UI.Tooltip.Set(button, button.Label);
        if (s == "Open") button.Loaded += (_, _) => { if (ToolTipService.GetToolTip(button) is ToolTip tooltip) tooltip.IsOpen = true; };
        return button;
    });
    public static UIElement Dialog(UI.Theme t, string s) => Example(t, "Dialog", s, "A modal decision or focused task. Native focus management keeps interaction inside the dialog.", o =>
    {
        var button = new UI.Button { Content = s == "Danger" ? "Delete glossary…" : "Open dialog" };
        var dialog = new UI.Dialog { Title = s == "Body only" ? "" : s == "Danger" ? "Delete this glossary?" : "Export document", Danger = s == "Danger", PrimaryText = s == "Danger" ? "Delete" : "Export", CloseText = s == "An alert" ? "Close" : "Cancel" };
        if (s == "An alert") dialog.PrimaryText = "";
        dialog.Content = s == "A form" ? new UI.FormField { Label = "File name", Field = Input(o, "handbook") } : Stage.Column(t, t.Text(s == "Danger" ? "This glossary will be removed from the workspace." : "Choose where to save the translated document."), ChoiceGroup(t));
        button.Click += async (_, _) => { var result = await dialog.ShowAsync(button); button.Content = result == ContentDialogResult.Primary ? "Confirmed" : "Open dialog"; };
        return button;
    });
    public static UIElement Drawer(UI.Theme t, string s) => Example(t, "Drawer", s, "A modal supporting panel that keeps its relationship to an edge of the window.", o =>
    {
        UI.Button Make(UI.DrawerSide side)
        {
            var button = new UI.Button { Content = $"Open {side.ToString().ToLowerInvariant()} drawer" };
            var drawer = new UI.Drawer { Title = "Document settings", Side = side, Extent = s == "Wide" ? t.Number("dialog.width") : null, Content = Preferences(t, "A section"), PrimaryText = "Done" };
            button.Click += async (_, _) => await drawer.ShowAsync(button); return button;
        }
        return s == "Sides" ? Stage.Column(t, Enum.GetValues<UI.DrawerSide>().Select(side => (UIElement)Make(side)).ToArray()) : Make(UI.DrawerSide.Right);
    });
}
