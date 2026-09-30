using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;

namespace DazzUI.Example;

/*
 * How each React component a story uses is drawn on WinUI.
 *
 * Every entry takes the props as the React story wrote them and hands them
 * to the kit's own control, passing React's default wherever the story
 * leaves one out. `tools/codegen/winui.mjs` lists the same props; a story
 * that sets one missing from both fails generation.
 */
public static class Mapping
{
    public static readonly IReadOnlyDictionary<string, Func<Node, UIElement?>> Components =
        new Dictionary<string, Func<Node, UIElement?>>
        {
            ["Row"] = Stage.Row,
            ["Col"] = Stage.Col,
            ["Frame"] = Stage.Frame,
            ["Text"] = Stage.Text,

            ["Accordion"] = n =>
            {
                var open = n["defaultValue"].Strings();
                var accordion = new Accordion { Multiple = n.Flag("multiple"), Divided = n.Flag("divided") };
                foreach (var child in n.Children)
                {
                    var item = new Node(child!, n.Context);
                    accordion.Items.Add(new AccordionItem
                    {
                        Header = item.String("title") ?? "",
                        Body = Block(item),
                        IsExpanded = open.Contains(item.String("value")),
                    });
                }
                return accordion;
            },
            ["ActionBar"] = n =>
            {
                var bar = new ActionBar { Alignment = Option(n.String("align"), ActionBarAlignment.Start) };
                foreach (var kid in n.Kids()) bar.Items.Add(kid);
                return bar;
            },
            ["Autocomplete"] = n =>
            {
                var field = new Autocomplete { PlaceholderText = n.String("placeholder") ?? "", Size = n.Option("size", ControlSize.Medium) };
                foreach (var item in n["items"].Strings()) field.Suggestions.Add(item);
                return field;
            },
            ["Avatar"] = n => new Avatar
            {
                NameText = n.String("name") ?? "",
                Size = n.Option("size", ControlSize.Medium),
                Shape = n.Option("shape", AvatarShape.Circle),
            },
            ["AvatarGroup"] = n =>
            {
                var max = (int?)n.Number("max") ?? n.Children.Count;
                var group = new AvatarGroup { Size = n.Option("size", ControlSize.Medium), Overflow = Math.Max(0, n.Children.Count - max) };
                foreach (var avatar in n.Children.Take(max).Select(child => Nodes.Build(child, n.Context)).OfType<Avatar>())
                    group.Avatars.Add(avatar);
                return group;
            },
            ["Badge"] = n => new Badge
            {
                Content = n.Text,
                Variant = n.Option("variant", BadgeVariant.Tinted),
                Tint = n.Option("tint", WidgetTint.Primary),
                Size = n.Option("size", ControlSize.Medium),
            },
            ["Button"] = n =>
            {
                var button = new Button
                {
                    Content = Inline(n, n.Props["shortcut"]),
                    Variant = n.Option("variant", ControlVariant.Recessed),
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Size = n.Option("size", ControlSize.Small),
                    IsEnabled = !n.Flag("disabled"),
                };
                if (n.Flag("fullWidth")) button.HorizontalAlignment = HorizontalAlignment.Stretch;
                var press = n.Press();
                var submit = n.String("type") == "submit" ? n.Context.Submit : null;
                button.Click += (_, _) => { press(); submit?.Invoke(); };
                return button;
            },
            ["Callout"] = n => new Callout
            {
                Title = n.String("title"),
                Description = n.Text,
                Action = n.Element("action"),
                Tint = n.Option("tint", WidgetTint.Neutral),
                Size = n.Option("size", ControlSize.Medium),
            },
            ["Card"] = n =>
            {
                var size = n.Option("size", ControlSize.Medium);
                var body = new Stack(Orientation.Vertical, size == ControlSize.Large ? "spacing.4" : size == ControlSize.Small ? "spacing.2" : "spacing.3");
                foreach (var kid in n.Kids()) body.Add(kid);
                return new Card { Content = body, Variant = n.Option("variant", CardVariant.Raised), Size = size };
            },
            ["Checkbox"] = n =>
            {
                var box = new Checkbox
                {
                    Content = n.Text,
                    Note = n.String("note"),
                    IsChecked = n.Flag("indeterminate") ? null : n.Flag("defaultChecked"),
                    IsThreeState = n.Flag("indeterminate"),
                    IsEnabled = !n.Flag("disabled"),
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Size = n.Option("size", ControlSize.Medium),
                };
                return box;
            },
            ["Code"] = n => new Code(n.Text),
            ["CodeBlock"] = n => new CodeBlock
            {
                Text = n.String("code") ?? "",
                Title = n.String("title"),
                Syntax = n.String("language"),
                LineNumbers = n.Flag("lineNumbers"),
                Copyable = n.Flag("copyable"),
            },
            ["Collapsible"] = n => new Collapsible
            {
                Header = n.String("title") ?? "",
                Meta = n.Element("meta"),
                Body = Block(n),
                IsExpanded = n.Flag("defaultOpen"),
            },
            ["Combobox"] = n =>
            {
                var field = new Combobox { PlaceholderText = n.String("placeholder") ?? "Search…" };
                foreach (var entry in n["options"].Items())
                {
                    if (entry?["options"] is JsonArray group)
                        foreach (var option in group)
                            field.Options.Add(new SelectOption(option!["value"].Text(), option["label"].Text(), entry["label"].Text()));
                    else
                        field.Options.Add(new SelectOption(entry!["value"].Text(), entry["label"].Text()));
                }
                var start = n.String("defaultValue");
                if (field.Options.FirstOrDefault(option => option.Value == start) is SelectOption chosen) field.Text = chosen.Label;
                return field;
            },
            ["ContextMenu"] = n =>
            {
                var target = Kid(n);
                var menu = new ContextMenu();
                foreach (var item in MenuItems(n, n["items"])) menu.Items.Add(item);
                // The menu takes its theme from where it is attached.
                if (target is FrameworkElement framed) framed.Loaded += (_, _) => menu.Attach(framed);
                return target;
            },
            ["Dialog"] = n => Present(n, drawer: false),
            ["DialogBody"] = n => Section(n, "body"),
            ["DialogClose"] = n => n.Element("render", n.Context with { Press = n.Context.Close }),
            ["DialogFooter"] = n => Section(n, "footer"),
            ["DialogHeader"] = n => Heading(n.String("title") ?? "", n.String("subtitle")),
            ["Divider"] = n => new Divider
            {
                Orientation = n.String("orientation") == "vertical" ? Orientation.Vertical : Orientation.Horizontal,
                Muted = n.String("tone") == "muted",
            },
            ["Drawer"] = n => Present(n, drawer: true),
            ["DrawerClose"] = n => n.Element("render", n.Context with { Press = n.Context.Close }),
            ["EmptyState"] = n => new EmptyState { Title = n.String("title") ?? "", Actions = n.Element("action") },
            ["Fieldset"] = n =>
            {
                var set = new Fieldset { Legend = n.String("legend") ?? "" };
                foreach (var kid in n.Kids()) set.Fields.Add(kid);
                return set;
            },
            ["Form"] = n =>
            {
                var submit = n.Action("onSubmit");
                var form = new Stack(Orientation.Vertical, "spacing.4");
                foreach (var kid in n.Kids(n.Context with { Submit = submit ?? (() => { }) })) form.Add(kid);
                return form;
            },
            ["FormField"] = n =>
            {
                var error = n.String("state") == "error";
                return new FormField
                {
                    Label = n.String("label") ?? "",
                    Hint = error ? null : n.String("hint"),
                    Error = error ? n.String("hint") ?? "" : null,
                    Field = Kid(n),
                };
            },
            ["Icon"] = n => Glyphs.Draw(Nodes.GlyphOf(n.Props["icon"]) ?? "", n.Number("size") ?? 20),
            ["IconButton"] = n =>
            {
                var button = new IconButton
                {
                    Label = n.String("label") ?? "",
                    Variant = n.Option("variant", ControlVariant.Plain),
                    Tint = n.Option("tint", WidgetTint.Neutral),
                    Size = n.Option("size", ControlSize.Tiny),
                    IsEnabled = !n.Flag("disabled"),
                };
                if (n.Children.Select(Nodes.GlyphOf).OfType<string>().FirstOrDefault() is string name)
                    (button.Glyph, button.GlyphVariant) = Glyphs.All[name];
                var press = n.Press();
                button.Click += (_, _) => press();
                return button;
            },
            ["KeyCap"] = n => new KeyCap(n.Text)
            {
                Variant = n.Option("variant", KeyCapVariant.Hint),
                Size = n.Option("size", ControlSize.Medium),
            },
            ["Link"] = n =>
            {
                var link = new Link
                {
                    Content = n.Flag("external") ? n.Text + " ↗" : n.Text,
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Underline = n.Option("underline", LinkUnderline.Hover),
                };
                var press = n.Press();
                link.Click += (_, _) => press();
                return link;
            },
            ["Menu"] = n =>
            {
                var menu = new Menu();
                foreach (var item in MenuItems(n, n["items"])) menu.Items.Add(item);
                FrameworkElement? anchor = null;
                anchor = n.Element("trigger", n.Context with { Press = () => { if (anchor is not null) menu.Show(anchor); } }) as FrameworkElement;
                return anchor;
            },
            ["Menubar"] = n =>
            {
                var bar = new Menubar { HorizontalAlignment = HorizontalAlignment.Left };
                foreach (var entry in n["menus"].Items())
                {
                    var menu = new Menu();
                    foreach (var item in MenuItems(n, entry?["items"])) menu.Items.Add(item);
                    bar.Menus.Add((entry?["label"].Text() ?? "", menu));
                }
                return bar;
            },
            ["Meter"] = n =>
            {
                var meter = new Meter
                {
                    Value = n.Number("value") ?? 0,
                    Label = n.String("label"),
                    ShowValue = n.Flag("showValue"),
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Size = n.Option("size", ControlSize.Medium),
                };
                Name(meter, n.String("aria-label") ?? n.String("label"));
                return meter;
            },
            ["NumberField"] = n =>
            {
                var field = new NumberField
                {
                    Value = n.Number("defaultValue") ?? double.NaN,
                    Minimum = n.Number("min") ?? double.MinValue,
                    Maximum = n.Number("max") ?? double.MaxValue,
                    SmallChange = n.Number("step") ?? 1,
                    Unit = n.String("unit"),
                    Size = n.Option("size", ControlSize.Medium),
                };
                Name(field, n.String("aria-label"));
                return field;
            },
            ["OptionCard"] = n =>
            {
                var card = new OptionCard
                {
                    Title = n.String("title") ?? "",
                    Description = n.String("description") ?? "",
                    IsEnabled = !n.Flag("disabled"),
                    // Each card is its own group: the story, not the radio, says which is chosen.
                    GroupName = Guid.NewGuid().ToString("N"),
                };
                var press = n.Press();
                // The radio checks itself; the story says whether it stays checked.
                card.Checked += (_, _) => { if (n.Flag("selected")) return; press(); card.IsChecked = n.Flag("selected"); };
                n.Live(card, () => card.IsChecked = n.Flag("selected"));
                return card;
            },
            ["OtpField"] = n =>
            {
                var field = new OtpField
                {
                    Length = (int?)n.Number("length") ?? 6,
                    GroupSize = (int?)n.Number("groupAfter") ?? 0,
                    Size = n.Option("size", ControlSize.Medium),
                };
                Name(field, n.String("aria-label"));
                return field;
            },
            ["PillTabs"] = n =>
            {
                var tabs = new PillTabs();
                var items = n["items"].Items();
                foreach (var item in items)
                    tabs.Items.Add(item?["count"] is JsonNode count ? $"{item["label"].Text()}  {count.Text()}" : item?["label"].Text());
                tabs.SelectedIndex = Math.Max(0, items.ToList().FindIndex(item => item?["value"].Text() == n.String("defaultValue")));
                Name(tabs, n.String("aria-label"));
                return tabs;
            },
            ["Popover"] = n =>
            {
                var popover = new Popover { Placement = Side(n.String("side"), FlyoutPlacementMode.Bottom) };
                FrameworkElement? anchor = null;
                anchor = n.Element("trigger", n.Context with
                {
                    Press = () =>
                    {
                        if (anchor is null) return;
                        popover.Content = PopoverPanel(n, popover.Hide);
                        popover.Show(anchor);
                    },
                }) as FrameworkElement;
                return anchor;
            },
            ["PopoverClose"] = n => n.Element("render", n.Context with { Press = n.Context.Close }),
            ["PreferenceGroup"] = n =>
            {
                var titles = new Stack(Orientation.Vertical, "spacing.0.5");
                titles.Add(new Styled(n.String("title") ?? "", "titleMedium"));
                if (n.String("description") is string description) titles.Add(new Styled(description, "captionSmall", "color.content-subtle"));
                var sections = new Stack(Orientation.Vertical, "preferences.section-gap");
                foreach (var kid in n.Kids()) sections.Add(kid);
                var group = new Stack(Orientation.Vertical, "preferences.title-gap");
                group.Add(titles);
                group.Add(sections);
                return group;
            },
            ["PreferenceRow"] = n =>
            {
                var row = new PreferenceRow { Label = n.String("title") ?? "", Description = n.String("subtitle"), Control = n.Children.Count == 0 ? null : Kid(n) };
                if (n.Action("onOpen") is Action open) row.Opened += (_, _) => open();
                return row;
            },
            ["PreferenceSection"] = n =>
            {
                var rows = new PreferenceGroup();
                foreach (var row in n.Kids().OfType<PreferenceRow>()) rows.Rows.Add(row);
                return new PreferenceSection { Title = n.String("label") ?? "", Body = rows, Footer = n.String("footer") };
            },
            ["Preferences"] = n =>
            {
                var column = new Preferences();
                foreach (var kid in n.Kids()) column.Sections.Add(kid);
                return column;
            },
            ["PreviewCard"] = n =>
            {
                var card = new PreviewCard { Placement = FlyoutPlacementMode.Bottom };
                var trigger = n.Element("trigger") as FrameworkElement;
                if (trigger is null) return null;
                var body = new Stack(Orientation.Vertical, "spacing.2");
                foreach (var kid in n.Kids()) body.Add(kid);
                if (Length.Parse(n["width"])?.Points is double width) body.Width = width;
                card.Content = body;
                card.Attach(trigger);
                return trigger;
            },
            ["Progress"] = n =>
            {
                var progress = new Progress
                {
                    Value = n.Number("value") ?? 0,
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Size = n.Option("size", ControlSize.Small),
                    Gradient = n.Flag("gradient"),
                };
                Name(progress, n.String("aria-label"));
                return progress;
            },
            ["RadioGroup"] = n =>
            {
                var group = new RadioGroup { Size = n.Option("size", ControlSize.Medium) };
                foreach (var option in n["options"].Items())
                    group.Items.Add(new Radio { Content = option?["label"].Text(), Tag = option?["value"].Text() });
                group.Value = n.String("defaultValue");
                Name(group, n.String("aria-label"));
                return group;
            },
            ["SearchField"] = n =>
            {
                var field = new SearchField { PlaceholderText = n.String("placeholder") ?? "Search" };
                var changed = n.Handler("onValueChange");
                field.TextChanged += (_, _) => changed?.Invoke(JsonValue.Create(field.Text));
                n.Live(field, () => { var value = n.String("value") ?? ""; if (field.Text != value) field.Text = value; });
                Name(field, n.String("aria-label"));
                return field;
            },
            ["SectionLabel"] = n => new SectionLabel(n.Text)
            {
                Accent = n.String("tint") is string tint ? Option(tint, WidgetTint.Primary) : null,
                Faint = n.Flag("faint"),
            },
            ["SegmentedControl"] = n =>
            {
                var control = new SegmentedControl { Size = n.Option("size", ControlSize.Medium) };
                var values = n["items"].Items().Select(item => item?["value"].Text()).ToList();
                foreach (var item in n["items"].Items()) control.Items.Add(item?["label"].Text());
                var changed = n.Handler("onValueChange");
                control.SelectionChanged += (_, _) =>
                {
                    if (control.SelectedIndex >= 0) changed?.Invoke(JsonValue.Create(values[control.SelectedIndex]));
                };
                n.Live(control, () => control.SelectedIndex = values.IndexOf(n.String("value")));
                Name(control, n.String("aria-label"));
                return control;
            },
            ["Select"] = n =>
            {
                var select = new Select
                {
                    PlaceholderText = n.String("placeholder") ?? "Choose…",
                    Size = n.Option("size", ControlSize.Medium),
                    IsEnabled = !n.Flag("disabled"),
                };
                select.SetOptions(n["options"].Items().Select(option => new SelectOption(option?["value"].Text() ?? "", option?["label"].Text() ?? "")));
                select.Value = n.String("defaultValue");
                Name(select, n.String("aria-label"));
                return select;
            },
            ["ShortcutRecorder"] = n =>
            {
                var recorder = new ShortcutRecorder { IsEnabled = !n.Flag("disabled"), Invalid = n.String("state") == "error" };
                var changed = n.Handler("onValueChange");
                var cleared = n.Action("onClear");
                recorder.ShortcutChanged += (_, value) =>
                {
                    changed?.Invoke(JsonValue.Create(value));
                    if (value.Length == 0) cleared?.Invoke();
                };
                n.Live(recorder, () => recorder.Shortcut = n.String("value") ?? "");
                Name(recorder, n.String("aria-label"));
                return recorder;
            },
            ["Sidebar"] = n => new SidebarColumn(n),
            ["SidebarContent"] = n =>
            {
                var content = new Stack(Orientation.Vertical, "spacing.0");
                foreach (var kid in n.Kids()) content.Add(kid);
                return new ScrollViewer { Content = content, HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled };
            },
            ["SidebarGroup"] = n =>
            {
                var group = new Stack(Orientation.Vertical, "spacing.0", theme => Inset(theme, "spacing.1.5", "spacing.2.5"));
                foreach (var kid in n.Kids()) group.Add(kid);
                return group;
            },
            ["SidebarGroupContent"] = n =>
            {
                var content = new Stack(Orientation.Vertical, "spacing.0");
                foreach (var kid in n.Kids()) content.Add(kid);
                return content;
            },
            ["SidebarGroupLabel"] = n =>
            {
                var label = new SectionLabel(n.Text);
                label.Loaded += (_, _) => label.Margin = new Thickness(Appearance.GetTheme(label).Number("spacing.2.5"), Appearance.GetTheme(label).Number("spacing.1"), 0, Appearance.GetTheme(label).Number("spacing.1"));
                n.Context.Sidebar?.Follow(label, sidebar => label.Visibility = sidebar.ShowsLabels ? Visibility.Visible : Visibility.Collapsed);
                return label;
            },
            ["SidebarHeader"] = n =>
            {
                var header = new Stack(Orientation.Vertical, "spacing.2", theme => Inset(theme, "spacing.2", "spacing.2.5"))
                {
                    VerticalContentAlignment = VerticalAlignment.Center,
                    MinHeightToken = "frame.titlebar.size",
                };
                foreach (var kid in n.Kids()) header.Add(kid);
                return header;
            },
            ["SidebarInset"] = n => new SidebarInset(n),
            ["SidebarMenu"] = n =>
            {
                var menu = new Stack(Orientation.Vertical, "frame.nav-gap");
                foreach (var kid in n.Kids()) menu.Add(kid);
                return menu;
            },
            ["SidebarMenuBadge"] = n => new Styled(n.Text, "labelSmall"),
            ["SidebarMenuButton"] = n => SidebarRow(n, null),
            ["SidebarMenuItem"] = n =>
            {
                var badge = n.Children.FirstOrDefault(child => child.Element() == "SidebarMenuBadge");
                var count = badge is null ? null : new Node(badge, n.Context).Text;
                var item = new Stack(Orientation.Vertical, "spacing.0");
                foreach (var child in n.Children.Where(child => child.Element() != "SidebarMenuBadge"))
                    if ((child.Element() == "SidebarMenuButton" ? SidebarRow(new Node(child!, n.Context), count) : Nodes.Build(child, n.Context)) is UIElement row)
                        item.Add(row);
                return item;
            },
            ["SidebarProvider"] = n => new SidebarProvider(n),
            ["SidebarTrigger"] = n =>
            {
                var trigger = new IconButton { Label = "Toggle sidebar", Variant = ControlVariant.Plain, Tint = WidgetTint.Neutral, Glyph = Glyph.PanelLeft };
                trigger.Click += (_, _) => n.Context.Sidebar?.Toggle();
                return trigger;
            },
            ["Skeleton"] = n =>
            {
                var bone = new Skeleton { Shape = n.Option("shape", SkeletonShape.Text) };
                if (n.Number("height") is double height) bone.Height = height;
                var width = Length.Parse(n["width"]);
                if (width?.Points is double points) bone.Width = points;
                if (width is { IsFraction: true } share)
                {
                    // A share of the room: the web's percentage width.
                    var room = new Grid { ColumnDefinitions = { new ColumnDefinition { Width = new GridLength(share.Value, GridUnitType.Star) }, new ColumnDefinition { Width = new GridLength(1 - share.Value, GridUnitType.Star) } } };
                    room.Children.Add(bone);
                    return room;
                }
                return bone;
            },
            ["SkeletonText"] = n => new SkeletonText { Lines = (int?)n.Number("lines") ?? 3 },
            ["Slider"] = n =>
            {
                var start = n["defaultValue"];
                var size = n.Option("size", ControlSize.Medium);
                if (start is JsonArray pair)
                {
                    var range = new RangeSlider { Size = size };
                    range.Lower.Value = pair[0].Number() ?? 0;
                    range.Upper.Value = pair.Count > 1 ? pair[1].Number() ?? 100 : 100;
                    Name(range, n.String("aria-label"));
                    if (!n.Flag("showValue")) return range;
                    var shown = Reading();
                    void Show() => shown.Text = $"{Math.Round(range.Lower.Value)} – {Math.Round(range.Upper.Value)}";
                    range.ValueChanged += (_, _) => Show();
                    Show();
                    return Beside(range, shown);
                }
                var slider = new Slider { Value = start.Number() ?? 0, Size = size };
                Name(slider, n.String("aria-label"));
                if (!n.Flag("showValue")) return slider;
                var value = Reading();
                slider.ValueChanged += (_, _) => value.Text = $"{Math.Round(slider.Value)}";
                value.Text = $"{Math.Round(slider.Value)}";
                return Beside(slider, value);
            },
            ["Spinner"] = n => new Spinner { Size = n.Option("size", ControlSize.Medium), OnAccent = n.Flag("onAccent") },
            ["Splitter"] = n =>
            {
                var splitter = new Splitter { Orientation = n.String("orientation") == "vertical" ? Orientation.Vertical : Orientation.Horizontal };
                foreach (var child in n.Children)
                {
                    var panel = new Node(child!, n.Context);
                    var body = new Stack(Orientation.Vertical, "spacing.2");
                    foreach (var kid in panel.Kids()) body.Add(kid);
                    splitter.Panels.Add(new SplitterPanel(body, panel.Number("defaultSize") ?? 1));
                }
                return splitter;
            },
            ["Step"] = n => new Step
            {
                Title = n.String("label") ?? "",
                Detail = n.String("meta"),
                Status = n.String("status") switch
                {
                    "active" => StepStatus.Active,
                    "done" => StepStatus.Complete,
                    "error" => StepStatus.Error,
                    _ => StepStatus.Pending,
                },
            },
            ["StepList"] = n =>
            {
                var list = new StepList();
                foreach (var step in n.Kids().OfType<Step>()) list.Steps.Add(step);
                return list;
            },
            ["Switch"] = n =>
            {
                var toggle = new Switch
                {
                    IsChecked = n.Flag("defaultChecked"),
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Size = n.Option("size", ControlSize.Medium),
                    IsEnabled = !n.Flag("disabled"),
                };
                Name(toggle, n.String("aria-label"));
                return toggle;
            },
            ["Table"] = n =>
            {
                var table = new Table { Selectable = false };
                var head = n.Children.FirstOrDefault(child => child.Element() == "TableHead");
                var heads = head?["children"].Items().Select(cell => new Node(cell!, n.Context)).ToArray() ?? [];
                for (var i = 0; i < heads.Length; i++)
                    table.Columns.Add(new TableColumn($"c{i}", heads[i].Text, 1, Align(heads[i].String("align"))));
                var index = 0;
                foreach (var row in n.Children.Where(child => child.Element() == "TableRow"))
                {
                    var cells = row!["children"].Items().Select(cell => new Node(cell!, n.Context)).ToArray();
                    table.Rows.Add(new TableRow($"r{index++}", cells.Select((cell, i) => (Key: $"c{i}", Value: Cell(cell)))
                        .ToDictionary(pair => pair.Key, pair => pair.Value)));
                }
                return table;
            },
            ["Tag"] = n =>
            {
                var tag = new Tag
                {
                    Text = n.Text,
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Removable = n.Has("onRemove"),
                };
                var changed = n.Handler("onSelectedChange");
                if (changed is null) tag.Selection.IsHitTestVisible = false;
                void Toggled()
                {
                    // Only a press moves the toggle away from what the story holds.
                    if (tag.IsSelected == n.Flag("selected")) return;
                    changed?.Invoke(JsonValue.Create(tag.IsSelected));
                    tag.IsSelected = n.Flag("selected");
                }
                tag.Selection.Checked += (_, _) => Toggled();
                tag.Selection.Unchecked += (_, _) => Toggled();
                if (n.Action("onRemove") is Action remove) tag.Removed += (_, _) => remove();
                n.Live(tag, () => tag.IsSelected = n.Flag("selected"));
                return tag;
            },
            ["TextField"] = n =>
            {
                var field = new TextField
                {
                    Text = n.String("defaultValue") ?? "",
                    PlaceholderText = n.String("placeholder") ?? "",
                    IsReadOnly = n.Flag("readOnly"),
                    Mono = n.Flag("mono"),
                    Invalid = n.String("state") == "error",
                    Size = n.Option("size", ControlSize.Medium),
                    IsEnabled = !n.Flag("disabled"),
                    AcceptsReturn = n.Flag("multiline"),
                };
                Name(field, n.String("aria-label"));
                return field;
            },
            ["ToastProvider"] = n =>
            {
                var line = new Stack(Orientation.Horizontal, "spacing.0");
                foreach (var kid in n.Kids(n.Context with { Toaster = new Toaster() })) line.Add(kid);
                return line;
            },
            ["ToastViewport"] = n =>
            {
                // The viewport is the window's edge; here it only says which one.
                if (n.Context.Toaster is Toaster toaster) toaster.Placement = n.String("placement") ?? "bottom";
                return null;
            },
            ["Toggle"] = n =>
            {
                var toggle = new Toggle
                {
                    Content = Inline(n, null),
                    IsChecked = n.Flag("defaultPressed"),
                    Variant = n.Option("variant", ControlVariant.Plain),
                    Size = n.Option("size", ControlSize.Tiny),
                    Tint = WidgetTint.Neutral,
                };
                if (n.String("label") is string label)
                {
                    Name(toggle, label);
                    toggle.Loaded += (_, _) => Tooltip.Set(toggle, label);
                }
                return toggle;
            },
            ["ToggleGroup"] = n =>
            {
                var group = new ToggleGroup
                {
                    Multiple = !n.Has("multiple") || n.Flag("multiple"),
                    Size = n.Option("size", ControlSize.Small),
                };
                var changed = n.Handler("onValueChange");
                foreach (var item in n["items"].Items())
                {
                    var toggle = new Toggle { Tag = item?["value"].Text() };
                    var glyph = Nodes.GlyphOf(item?["label"]);
                    toggle.Content = glyph is null ? item?["label"].Text() : Glyphs.Draw(glyph, 16);
                    if (item?["hint"].String() is string hint) { Name(toggle, hint); toggle.Loaded += (_, _) => Tooltip.Set(toggle, hint); }
                    toggle.Click += (_, _) =>
                    {
                        changed?.Invoke(new JsonArray(group.Values.Select(value => (JsonNode?)JsonValue.Create(value)).ToArray()));
                        if (n.Has("value")) Select(group, n["value"]);
                    };
                    group.Items.Add(toggle);
                }
                n.Live(group, () => Select(group, n["value"]));
                Name(group, n.String("aria-label"));
                return group;
            },
            ["Tooltip"] = n =>
            {
                var target = Kid(n);
                var label = n.String("label") ?? "";
                var side = n.String("side") switch
                {
                    "bottom" => PlacementMode.Bottom,
                    "left" => PlacementMode.Left,
                    "right" => PlacementMode.Right,
                    _ => PlacementMode.Top,
                };
                if (target is FrameworkElement framed) framed.Loaded += (_, _) => Tooltip.Set(framed, label, side);
                return target;
            },
            ["Tree"] = n => TreeOf(n),
        };

    /* Helpers ------------------------------------------------------------- */

    /// <summary>A string as one of a kit enum's cases — React writes them in lower case — or the fallback.</summary>
    public static T Option<T>(string? value, T fallback) where T : struct, Enum =>
        value is not null && System.Enum.TryParse<T>(value, ignoreCase: true, out var parsed) ? parsed : fallback;

    private static void Name(UIElement element, string? name)
    {
        if (name is not null) AutomationProperties.SetName(element, name);
    }

    /// <summary>The one child, or the children stacked when there are more.</summary>
    private static UIElement? Kid(Node n)
    {
        var kids = n.Kids().ToArray();
        if (kids.Length == 1) return kids[0];
        var stack = new Stack(Orientation.Vertical, "spacing.2");
        foreach (var kid in kids) stack.Add(kid);
        return stack;
    }

    /// <summary>The children as a block: their words when they are only words, else stacked.</summary>
    private static UIElement Block(Node n) =>
        n.Children.All(child => child.String() is not null || Nodes.IsLive(child))
            ? new Styled(n.Text, "bodySmall", "color.content-secondary")
            : Kid(n) ?? new Grid();

    /// <summary>A button's children in a line: a glyph beside its label, and the shortcut after both.</summary>
    private static object? Inline(Node n, JsonNode? shortcut)
    {
        if (shortcut is null && n.Children.Count == 1 && n.Children[0].String() is string only) return only;
        // A glyph alone is the control's whole face.
        if (shortcut is null && n.Children.Count == 1 && n.Children[0].Element() == "Icon")
            return Glyphs.Draw(Nodes.GlyphOf(n.Children[0]) ?? "", 16);
        var line = new Stack(Orientation.Horizontal, "spacing.1.5") { VerticalContentAlignment = VerticalAlignment.Center };
        foreach (var child in n.Children)
        {
            var element = child.Element() == "Icon"
                ? Glyphs.Draw(Nodes.GlyphOf(child) ?? "", 16)
                : Nodes.Build(child, n.Context);
            if (element is FrameworkElement framed) framed.VerticalAlignment = VerticalAlignment.Center;
            if (element is not null) line.Add(element);
        }
        if (shortcut is not null)
            line.Add(new KeyCap(KeyText(shortcut)) { Variant = KeyCapVariant.Hint, VerticalAlignment = VerticalAlignment.Center });
        return line;
    }

    /// <summary>A KeyCap element's label, which is all a button or a menu row keeps.</summary>
    private static string KeyText(JsonNode? value) =>
        value.String() ?? string.Concat(value?["children"].Items().Select(Json.Text) ?? []);

    /// <summary>Two things on a line, the second at its end.</summary>
    private static UIElement Line(UIElement first, UIElement second)
    {
        var line = new Stack(Orientation.Horizontal, "spacing.2") { VerticalContentAlignment = VerticalAlignment.Center };
        line.Add(first);
        line.Add(second);
        return line;
    }

    /// <summary>A slider's reading: after the track, in the code face, right-aligned in a column of its own.</summary>
    private static TextBlock Reading() => new()
    {
        FontFamily = new Microsoft.UI.Xaml.Media.FontFamily("Cascadia Mono, Consolas"),
        FontSize = 11,
        TextAlignment = TextAlignment.Right,
        MinWidth = 40,
        VerticalAlignment = VerticalAlignment.Center,
        Opacity = 0.7,
    };

    private static UIElement Beside(FrameworkElement track, TextBlock value)
    {
        var row = new Grid { ColumnSpacing = 12, ColumnDefinitions = { new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) }, new ColumnDefinition { Width = GridLength.Auto } } };
        track.VerticalAlignment = VerticalAlignment.Center;
        row.Children.Add(track);
        Grid.SetColumn(value, 1);
        row.Children.Add(value);
        return row;
    }

    private static Thickness Inset(Theme theme, string block, string inline) =>
        new(theme.Number(inline), theme.Number(block), theme.Number(inline), theme.Number(block));

    private static TextAlignment Align(string? align) => align switch
    {
        "end" => TextAlignment.Right,
        "center" => TextAlignment.Center,
        _ => TextAlignment.Left,
    };

    /// <summary>A table cell's content: its words, or the component it holds.</summary>
    private static object? Cell(Node cell)
    {
        if (cell.Children.All(child => child.String() is not null)) return cell.Text;
        var element = Kid(cell);
        if (element is FrameworkElement framed)
            framed.HorizontalAlignment = cell.String("align") == "end" ? HorizontalAlignment.Right : HorizontalAlignment.Left;
        return element;
    }

    private static FlyoutPlacementMode Side(string? side, FlyoutPlacementMode fallback) => side switch
    {
        "top" => FlyoutPlacementMode.Top,
        "bottom" => FlyoutPlacementMode.Bottom,
        "left" => FlyoutPlacementMode.Left,
        "right" => FlyoutPlacementMode.Right,
        _ => fallback,
    };

    private static void Select(ToggleGroup group, JsonNode? value)
    {
        var chosen = value.Strings();
        foreach (var toggle in group.Items) toggle.IsChecked = chosen.Contains(toggle.Tag?.ToString());
    }

    private static IEnumerable<MenuItem> MenuItems(Node n, JsonNode? items) =>
        items.Items().Select(item => new MenuItem(
            item?["label"].Text() ?? "",
            () => n.Context.Run(item?["onSelect"]),
            item?["disabled"].Truthy() ?? false,
            item?["checked"] is JsonNode check ? check.Truthy() : null,
            Shortcut: item?["shortcut"] is JsonNode shortcut ? KeyText(shortcut) : null,
            Glyph: Nodes.GlyphOf(item?["icon"]) is string glyph ? Glyphs.All[glyph].Glyph : null));

    /* Presented ----------------------------------------------------------- */

    /// <summary>A dialog's or a drawer's head: its title, and the line under it.</summary>
    private static UIElement Heading(string title, string? subtitle)
    {
        var head = new Stack(Orientation.Vertical, "spacing.1", theme => Inset(theme, "spacing.3", "spacing.5"));
        head.Add(new Styled(title, "titleSmall") { Weight = "labelStrong" });
        if (subtitle is not null) head.Add(new Styled(subtitle, "captionSmall", "color.content-subtle"));
        return head;
    }

    /// <summary>A dialog's body or footer: its children, in the padding the web gives them.</summary>
    private static UIElement Section(Node n, string part)
    {
        if (part == "footer")
        {
            var bar = new ActionBar { Alignment = ActionBarAlignment.End };
            foreach (var kid in n.Kids()) bar.Items.Add(kid);
            var footer = new Stack(Orientation.Vertical, "spacing.0", theme => Inset(theme, "spacing.3", "spacing.5"))
            {
                BackgroundToken = "color.surface-chrome",
            };
            footer.Add(bar);
            return footer;
        }
        var body = new Stack(Orientation.Vertical, "spacing.4", theme =>
            new Thickness(theme.Number("spacing.5"), theme.Number("spacing.4") + theme.Number("spacing.0.5"), theme.Number("spacing.5"), theme.Number("spacing.4") + theme.Number("spacing.0.5")));
        foreach (var kid in n.Kids()) body.Add(kid);
        return body;
    }

    /// <summary>A popover's panel: its head, then its children.</summary>
    private static UIElement PopoverPanel(Node n, Action close)
    {
        var panel = new Stack(Orientation.Vertical, "spacing.3");
        if (n.String("title") is string title || n.String("description") is not null)
        {
            var head = new Stack(Orientation.Vertical, "spacing.1");
            if (n.String("title") is string text) head.Add(new Styled(text, "labelLarge"));
            if (n.String("description") is string description) head.Add(new Styled(description, "bodySmall", "color.content-secondary"));
            panel.Add(head);
        }
        foreach (var kid in n.Kids(n.Context with { Close = close })) panel.Add(kid);
        if (Length.Parse(n["width"])?.Points is double width) panel.Width = width - 2 * 18;
        return panel;
    }

    /// <summary>A dialog or a drawer: its trigger, which presents it, built afresh each time as React mounts it.</summary>
    private static UIElement? Present(Node n, bool drawer)
    {
        FrameworkElement? anchor = null;
        anchor = n.Element("trigger", n.Context with
        {
            Press = () =>
            {
                if (anchor is null) return;
                Dialog dialog = drawer
                    ? new Drawer
                    {
                        Side = n.String("side") switch
                        {
                            "left" => DrawerSide.Left,
                            "top" => DrawerSide.Top,
                            "bottom" => DrawerSide.Bottom,
                            _ => DrawerSide.Right,
                        },
                    }
                    : new Dialog { Danger = n.String("tone") == "danger" };
                dialog.PrimaryText = "";
                dialog.CloseText = "";
                dialog.Padding = new Thickness(0);
                var inner = n.Context with { Close = dialog.Hide };
                var sheet = new Stack(Orientation.Vertical, "spacing.0");
                if (drawer)
                {
                    if (n.String("title") is string title) sheet.Add(Heading(title, n.String("description")));
                    var body = new Stack(Orientation.Vertical, "spacing.4", theme => Inset(theme, "spacing.4", "spacing.5"));
                    foreach (var kid in n.Kids(inner)) body.Add(kid);
                    sheet.Add(body);
                    if (n.Element("footer", inner) is UIElement footer)
                    {
                        var bar = new Stack(Orientation.Vertical, "spacing.0", theme => Inset(theme, "spacing.3", "spacing.5")) { BackgroundToken = "color.surface-chrome" };
                        bar.Add(footer);
                        sheet.Add(bar);
                    }
                }
                else
                {
                    foreach (var kid in n.Kids(inner)) sheet.Add(kid);
                    if (Length.Parse(n["width"])?.Points is double width) sheet.Width = width;
                }
                dialog.Content = sheet;
                _ = dialog.ShowAsync(anchor);
            },
        }) as FrameworkElement;
        return anchor;
    }

    /* Tree ---------------------------------------------------------------- */

    private static UIElement TreeOf(Node n)
    {
        var expanded = n["defaultExpanded"].Strings();
        TreeNode Item(JsonNode? item) => new(
            item?["id"].Text() ?? "",
            item?["label"].Text() ?? "",
            item?["children"] is JsonArray children ? children.Select(Item).ToArray() : null,
            item?["meta"] is JsonNode meta && meta.Element() is null ? meta.Text() : null,
            Expanded: expanded.Contains(item?["id"].Text()),
            Disabled: item?["disabled"].Truthy() ?? false,
            Icon: Nodes.GlyphOf(item?["icon"]) is string glyph ? Glyphs.All[glyph].Glyph : null,
            MetaView: item?["meta"] is JsonNode view && view.Element() is not null ? () => Nodes.Build(view, n.Context)! : null);
        var tree = new Tree { Size = n.Option("size", ControlSize.Medium) };
        foreach (var item in n["items"].Items()) tree.Nodes.Add(Item(item));
        var changed = n.Handler("onSelectedChange");
        tree.Selected += (_, node) => changed?.Invoke(JsonValue.Create(node.Id));
        n.Live(tree, () => tree.SelectedId = n.String("selected"));
        Name(tree, n.String("aria-label"));
        return tree;
    }

    /* Sidebar ------------------------------------------------------------- */

    /// <summary>A sidebar row: a navigation item, with the badge its item carries at its end.</summary>
    private static UIElement SidebarRow(Node n, string? badge)
    {
        var glyph = Nodes.GlyphOf(n.Props["icon"]);
        var row = new NavItem
        {
            Content = n.Text,
            Glyph = glyph is null ? null : Glyphs.All[glyph].Glyph,
            Count = badge,
        };
        var press = n.Press();
        void Toggled()
        {
            // Only a press moves the row away from what the story holds.
            if (row.IsChecked == n.Flag("active")) return;
            press();
            row.IsChecked = n.Flag("active");
        }
        row.Checked += (_, _) => Toggled();
        row.Unchecked += (_, _) => Toggled();
        n.Live(row, () => row.IsChecked = n.Flag("active"));
        n.Context.Sidebar?.Follow(row, sidebar =>
        {
            row.IsIconOnly = !sidebar.ShowsLabels;
            row.Content = n.Text;
            if (n.String("tooltip") is string tip && row.IsIconOnly) Tooltip.Set(row, tip, PlacementMode.Right);
            else ToolTipService.SetToolTip(row, null);
            Appearance.Refresh(row);
        });
        return row;
    }
}

/// <summary>Children in a line or a stack, the gap and inset read from the theme it stands in.</summary>
public sealed class Stack : Grid
{
    private readonly StackPanel panel;
    private readonly string gap;
    private readonly Func<Theme, Thickness>? inset;

    public Stack(Orientation orientation, string gap, Func<Theme, Thickness>? inset = null)
    {
        panel = new StackPanel { Orientation = orientation };
        this.gap = gap;
        this.inset = inset;
        Children.Add(panel);
        Themed.Follow(this, Apply);
    }

    /// <summary>The token its height starts at, if any.</summary>
    public string? MinHeightToken { get; init; }

    /// <summary>The token its ground is painted in, if any.</summary>
    public string? BackgroundToken { get; init; }

    /// <summary>Where the children sit across a line, or down a stack taller than they are.</summary>
    public VerticalAlignment VerticalContentAlignment
    {
        get => panel.VerticalAlignment;
        init => panel.VerticalAlignment = value;
    }

    public void Add(UIElement child)
    {
        if (panel.Orientation == Orientation.Horizontal && VerticalContentAlignment != VerticalAlignment.Stretch && child is FrameworkElement framed)
            framed.VerticalAlignment = VerticalContentAlignment;
        panel.Children.Add(child);
    }

    private void Apply(Theme theme)
    {
        panel.Spacing = theme.Number(gap);
        if (inset is not null) Padding = inset(theme);
        if (MinHeightToken is not null) MinHeight = theme.Number(MinHeightToken);
        if (BackgroundToken is not null) Background = theme.Brush(BackgroundToken);
    }
}

/// <summary>Text set in one of the theme's type styles and inks.</summary>
public sealed class Styled : Grid
{
    private readonly TextBlock block;
    private readonly string style;
    private readonly string? ink;

    public Styled(string text, string style, string? ink = null)
    {
        block = Stage.Words(text);
        block.VerticalAlignment = VerticalAlignment.Center;
        this.style = style;
        this.ink = ink;
        Children.Add(block);
        Themed.Follow(this, Apply);
    }

    public string Text { get => block.Text; set => block.Text = value; }

    /// <summary>The ink, in place of the one the style was given.</summary>
    public Microsoft.UI.Xaml.Media.Brush Ink { set => block.Foreground = value; }

    /// <summary>Keeps to one line, cutting what does not fit.</summary>
    public bool Trimmed
    {
        init
        {
            block.TextWrapping = value ? TextWrapping.NoWrap : TextWrapping.Wrap;
            block.TextTrimming = value ? TextTrimming.CharacterEllipsis : TextTrimming.None;
        }
    }

    /// <summary>A type style whose weight overrides the style's own.</summary>
    public string? Weight { get; init; }

    private void Apply(Theme theme)
    {
        var styled = theme.Text("", style);
        block.FontSize = styled.FontSize;
        block.LineHeight = styled.LineHeight;
        block.LineStackingStrategy = styled.LineStackingStrategy;
        block.FontWeight = Weight is null ? styled.FontWeight : theme.Text("", Weight).FontWeight;
        if (ink is not null) block.Foreground = theme.Brush(ink);
    }
}

/* Sidebar ------------------------------------------------------------------- */

/// <summary>
/// What a <c>SidebarProvider</c> shares with the parts inside it: whether
/// the sidebar is open, and — inside the sidebar's own column — whether
/// its rows show their labels.
/// </summary>
public sealed class SidebarContext
{
    private readonly SidebarContext? provider;
    private bool open = true;

    public SidebarContext() { }

    private SidebarContext(SidebarContext provider, string collapsible)
    {
        this.provider = provider;
        Collapsible = collapsible;
    }

    private event Action? Changed;

    public bool Open => provider?.Open ?? open;

    /// <summary>Whether the provider sets its content in, as the inset variant does.</summary>
    public bool Inset { get; init; }

    public string Collapsible { get; } = "offcanvas";

    /// <summary>Whether a row here shows its label: always, but in a sidebar collapsed to its icons.</summary>
    public bool ShowsLabels => provider is null || Collapsible != "icon" || Open;

    public void Toggle()
    {
        if (provider is not null) { provider.Toggle(); return; }
        open = !open;
        Changed?.Invoke();
    }

    /// <summary>The context inside a sidebar's own column.</summary>
    public SidebarContext Column(string collapsible) => new(this, collapsible);

    /// <summary>Runs <paramref name="update"/> now and whenever the sidebar opens or closes, while the element is in the window.</summary>
    public void Follow(FrameworkElement element, Action<SidebarContext> update)
    {
        var source = provider ?? this;
        void Changed() => update(this);
        update(this);
        source.Changed += Changed;
        element.Loaded += (_, _) => { source.Changed -= Changed; source.Changed += Changed; update(this); };
        element.Unloaded += (_, _) => source.Changed -= Changed;
    }
}

/// <summary>The sidebar and its content side by side, sharing whether the sidebar is open.</summary>
public sealed class SidebarProvider : Grid
{
    public SidebarProvider(Node n)
    {
        var inset = n.Children.Any(child => child.Element() == "Sidebar" && child?["props"]?["variant"].String() == "inset");
        var context = n.Context with { Sidebar = new SidebarContext { Inset = inset } };
        foreach (var kid in n.Kids(context))
        {
            ColumnDefinitions.Add(new ColumnDefinition { Width = kid is SidebarColumn ? GridLength.Auto : new GridLength(1, GridUnitType.Star) });
            SetColumn((FrameworkElement)kid, Children.Count);
            Children.Add(kid);
        }
        if (inset) Themed.Follow(this, theme => Background = theme.Brush("color.surface-column"));
    }
}

/// <summary>The sidebar's column: as wide as its collapsible state says, and framed as its variant says.</summary>
public sealed class SidebarColumn : Grid
{
    private readonly string variant;
    private readonly SidebarContext sidebar;
    private readonly Grid frame = new() { HorizontalAlignment = HorizontalAlignment.Left };

    public SidebarColumn(Node n)
    {
        variant = n.String("variant") ?? "sidebar";
        sidebar = (n.Context.Sidebar ?? new SidebarContext()).Column(n.String("collapsible") ?? "offcanvas");
        var context = n.Context with { Sidebar = sidebar };
        foreach (var child in n.Children)
        {
            if (Nodes.Build(child, context) is not FrameworkElement kid) continue;
            frame.RowDefinitions.Add(new RowDefinition { Height = child.Element() == "SidebarContent" ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
            SetRow(kid, frame.Children.Count);
            frame.Children.Add(kid);
        }
        Children.Add(frame);
        Themed.Follow(this, Apply);
        sidebar.Follow(this, _ => { if (IsLoaded) Apply(Appearance.GetTheme(this)); });
    }

    private void Apply(Theme theme)
    {
        var framed = variant != "sidebar";
        var collapsible = sidebar.Collapsible;
        var icons = collapsible == "icon" && !sidebar.Open;
        var pad = framed ? theme.Number("spacing.2") : 0;
        var full = theme.Number(collapsible == "none" ? "frame.rail.width" : "frame.sidebar.width");
        var width = collapsible == "none" ? full
            : !sidebar.Open && collapsible == "offcanvas" ? 0
            : icons ? theme.Number("frame.sidebar.icon-width") + (framed ? theme.Number("spacing.4") : 0)
            : full;
        Width = width;
        Padding = new Thickness(pad);
        // Closing slides the column shut over content that keeps its width;
        // the layout clips what no longer fits.
        frame.Width = Math.Max(0, (icons ? width : full) - pad * 2);
        var surface = theme.Brush(collapsible == "none" ? "color.surface" : "color.surface-column");
        Background = variant == "sidebar" ? surface : null;
        BorderBrush = theme.Brush("color.border");
        BorderThickness = new Thickness(0, 0, variant == "sidebar" && width > 0 ? theme.Number("stroke.hairline") : 0, 0);
        frame.Background = variant == "floating" ? surface : null;
        frame.BorderBrush = theme.Brush("color.border");
        frame.BorderThickness = new Thickness(variant == "floating" ? theme.Number("stroke.hairline") : 0);
        frame.CornerRadius = new CornerRadius(variant == "floating" ? theme.Number("radius.large") : 0);
    }
}

/// <summary>The content beside a sidebar: its rows ruled off, set in when the provider is inset.</summary>
public sealed class SidebarInset : Grid
{
    private readonly SidebarContext? sidebar;
    private readonly StackPanel paper = new();

    public SidebarInset(Node n)
    {
        sidebar = n.Context.Sidebar;
        foreach (var kid in n.Kids()) paper.Children.Add(new Border { Child = kid });
        Children.Add(paper);
        Themed.Follow(this, Apply);
        sidebar?.Follow(this, _ => { if (IsLoaded) Apply(Appearance.GetTheme(this)); });
    }

    private void Apply(Theme theme)
    {
        paper.Background = theme.Brush("color.surface");
        foreach (var rule in paper.Children.OfType<Border>())
        {
            rule.BorderBrush = theme.Brush("color.border");
            rule.BorderThickness = new Thickness(0, 0, 0, theme.Number("stroke.hairline"));
        }
        if (sidebar?.Inset != true) return;
        var gap = theme.Number("spacing.2");
        paper.CornerRadius = new CornerRadius(theme.Number("radius.large"));
        Padding = new Thickness(sidebar.Open ? 0 : gap, gap, gap, gap);
    }
}
