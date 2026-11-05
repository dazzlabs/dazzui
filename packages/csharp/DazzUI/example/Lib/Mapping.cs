using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;

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
                    accordion.Items.Add(Nodes.Drawn(child!, new AccordionItem
                    {
                        Header = item.String("title") ?? "",
                        Body = Block(item),
                        IsExpanded = open.Contains(item.String("value")),
                    }));
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
                    Content = Inline(n, n.Props["shortcut"], owned: true),
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
            ["Calendar"] = n =>
            {
                var range = n.String("mode") == "range";
                var calendar = new Calendar
                {
                    Mode = range ? CalendarMode.Range : CalendarMode.Single,
                    Size = n.Option("size", ControlSize.Medium),
                    Months = (int?)n.Number("months") ?? 1,
                    Min = ReadDay(n["min"]),
                    Max = ReadDay(n["max"]),
                    WeekStartsOn = (DayOfWeek)((int?)n.Number("weekStartsOn") ?? 1),
                    Today = ReadDay(n["today"]),
                    Navigation = !n.Has("navigation") || n.Flag("navigation"),
                    Tint = n.Option("tint", WidgetTint.Primary),
                };
                var changed = n.Handler("onValueChange");
                if (range)
                {
                    calendar.RangeChanged += (_, value) => changed?.Invoke(DateRangeJson(value));
                    n.Live(calendar, () => calendar.Range = ReadDateRange(n["value"] ?? n["defaultValue"]));
                }
                else
                {
                    calendar.ValueChanged += (_, value) => changed?.Invoke(DayJson(value));
                    n.Live(calendar, () => calendar.Value = ReadDay(n["value"] ?? n["defaultValue"]));
                }
                Name(calendar, n.String("aria-label"));
                return calendar;
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
                foreach (var child in n.Children)
                {
                    // A bare string is the card's own body copy, at the type
                    // step its stylesheet gives the card.
                    if (child.String() is string words) body.Add(new Styled(words, "bodyMedium"));
                    else if (Nodes.Build(child, n.Context) is UIElement kid) body.Add(kid);
                }
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
            ["DateRangePicker"] = n =>
            {
                var picker = new DateRangePicker
                {
                    TimeStep = (int?)n.Number("timeStep") ?? 15,
                    Months = (int?)n.Number("months") ?? 1,
                    WeekStartsOn = (DayOfWeek)((int?)n.Number("weekStartsOn") ?? 1),
                    Today = ReadDay(n["today"]),
                    Size = n.Option("size", ControlSize.Medium),
                    Invalid = n.String("state") == "error",
                    Tint = n.Option("tint", WidgetTint.Primary),
                    IsEnabled = !n.Flag("disabled"),
                };
                // A preset is data here: a label and two whole-day offsets from today.
                if (n.Has("presets"))
                {
                    picker.Presets.Clear();
                    foreach (var preset in n["presets"].Items())
                        picker.Presets.Add(DateRangePreset.Days(preset?["label"].Text() ?? "", (int)(preset?["days"]?[0].Number() ?? 0), (int)(preset?["days"]?[1].Number() ?? 0)));
                }
                if (n.Has("durations"))
                {
                    picker.Durations.Clear();
                    foreach (var duration in n["durations"].Items())
                        picker.Durations.Add(new DateRangeDuration(duration?["label"].Text() ?? "", (int)(duration?["minutes"].Number() ?? 60)));
                }
                var changed = n.Handler("onValueChange");
                picker.ValueChanged += (_, value) => changed?.Invoke(DateRangeValueJson(value));
                n.Live(picker, () => picker.Value = ReadDateRangeValue(n["value"] ?? n["defaultValue"]));
                Name(picker, n.String("aria-label"));
                return picker;
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
            // No size is React's own default: the theme's `icon.medium`.
            ["Icon"] = n => Glyphs.Draw(Nodes.GlyphOf(n.Props["icon"]) ?? "", n.Number("size")),
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
                // The story's `<Icon>` child is the button's face, and its
                // own specimen: draw it here rather than naming a glyph.
                if (n.Children.FirstOrDefault(child => child.Element() == "Icon") is JsonNode glyphNode)
                    button.Content = Nodes.Drawn(glyphNode, Glyphs.Draw(Nodes.GlyphOf(glyphNode) ?? ""));
                else if (n.Children.Select(Nodes.GlyphOf).OfType<string>().FirstOrDefault() is string name)
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
                    Content = n.Text,
                    External = n.Flag("external"),
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
                    Minimum = n.Number("min") ?? 0,
                    Maximum = n.Number("max") ?? 100,
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
                    Minimum = n.Number("min") ?? double.MinValue,
                    Maximum = n.Number("max") ?? double.MaxValue,
                    Value = n.Number("defaultValue") ?? double.NaN,
                    SmallChange = n.Number("step") ?? 1,
                    Unit = n.String("unit"),
                    Size = n.Option("size", ControlSize.Medium),
                    Tint = n.Option("tint", WidgetTint.Primary),
                    Invalid = n.String("state") == "error",
                    IsEnabled = !n.Flag("disabled"),
                    PlaceholderText = n.String("placeholder") ?? "",
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
                {
                    if (item?["count"] is JsonNode count) tabs.Counts[tabs.Items.Count] = count.GetValue<int>();
                    tabs.Items.Add(item?["label"].Text());
                }
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
                // The group's description sets its caption at 1.7 leading.
                if (n.String("description") is string description) titles.Add(new Styled(description, "captionSmall", "color.content-subtle") { LeadingScale = 1.7 });
                var sections = new Stack(Orientation.Vertical, "preferences.section-gap");
                foreach (var kid in n.Kids()) sections.Add(kid);
                var group = new Stack(Orientation.Vertical, "preferences.title-gap");
                group.Add(titles);
                group.Add(sections);
                return group;
            },
            ["PreferenceRow"] = n =>
            {
                var row = new PreferenceRow { Label = n.String("title") ?? "", Description = n.String("subtitle"), Leading = n.Element("icon"), Tint = n.Option("tint", WidgetTint.Neutral), Control = n.Children.Count == 0 ? null : Kid(n) };
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
            ["PreferenceThemeSwatches"] = n =>
            {
                var control = new PreferenceThemeSwatches();
                foreach (var option in n["options"].Items())
                    control.Options.Add(new PreferenceThemeOption(option?["value"].Text() ?? "",
                        option?["label"].Text() ?? "", option?["theme"] is JsonNode palette ? palette.Text() : null));
                control.Value = n.String("value") ?? n.String("defaultValue");
                var changed = n.Handler("onValueChange");
                control.ValueChanged += (_, value) =>
                {
                    changed?.Invoke(JsonValue.Create(value));
                    if (n.Has("value")) control.Value = n.String("value");
                };
                if (n.Has("value")) n.Live(control, () => control.Value = n.String("value"));
                Name(control, n.String("aria-label"));
                return control;
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
                    IsIndeterminate = n.Number("value") is null,
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
                var control = new SegmentedControl
                {
                    Size = n.Option("size", ControlSize.Medium),
                    IconOnly = n.Flag("iconOnly"),
                    Filled = n.String("activeStyle") == "filled",
                };
                var values = n["items"].Items().Select(item => item?["value"].Text()).ToList();
                foreach (var item in n["items"].Items())
                {
                    control.Items.Add(item?["label"].Text());
                    control.Icons.Add(Nodes.GlyphOf(item?["icon"]) is string glyph ? Glyphs.All[glyph].Glyph : null);
                }
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
                    Invalid = n.String("state") == "error",
                    Mono = n.Flag("mono"),
                    Tint = n.Option("tint", WidgetTint.Primary),
                };
                select.SetOptions(n["options"].Items().SelectMany(entry =>
                    entry?["options"] is JsonArray group
                        ? group.Select(option => new SelectOption(option?["value"].Text() ?? "", option?["label"].Text() ?? "", entry?["label"].Text(), option?["disabled"]?.GetValue<bool>() ?? false))
                        : [new SelectOption(entry?["value"].Text() ?? "", entry?["label"].Text() ?? "", Disabled: entry?["disabled"]?.GetValue<bool>() ?? false)]));
                var changing = false;
                var changed = n.Handler("onValueChange");
                select.SelectionChanged += (_, _) => { if (!changing && select.Value is string value) changed?.Invoke(JsonValue.Create(value)); };
                n.Live(select, () =>
                {
                    changing = true;
                    select.Value = n.String("value") ?? n.String("defaultValue");
                    changing = false;
                });
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
                var scroll = new ScrollViewer { Content = content, HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled };
                Themed.Follow(scroll, theme => scroll.Padding = new Thickness(0, theme.Number(n.Context.Sidebar?.Collapsible == "none" ? "spacing.2.5" : "spacing.2"), 0, theme.Number(n.Context.Sidebar?.Collapsible == "none" ? "spacing.2.5" : "spacing.2")));
                return scroll;
            },
            ["SidebarGroup"] = n =>
            {
                var group = new Stack(Orientation.Vertical, "spacing.0", theme =>
                {
                    if (n.Context.Sidebar?.Collapsible == "none")
                    {
                        var siblings = n.Source.Parent as JsonArray;
                        var index = siblings?.IndexOf(n.Source) ?? -1;
                        return new Thickness(theme.Number("spacing.2.5"), index == 0 ? 0 : theme.Number("spacing.1.5"), theme.Number("spacing.2.5"), index == siblings?.Count - 1 ? 0 : theme.Number("spacing.1.5"));
                    }
                    return Inset(theme, "spacing.1.5", "spacing.2.5");
                });
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
                // The group heading is the faint section label on the rows'
                // inset — its own padded row, not a margin above one.
                var label = new SidebarHeading { Label = n.Text };
                n.Context.Sidebar?.Follow(label, sidebar => label.Visibility = sidebar.ShowsLabels ? Visibility.Visible : Visibility.Collapsed);
                return label;
            },
            ["SidebarHeader"] = n =>
            {
                var header = new Stack(Orientation.Vertical, "spacing.2", theme => Inset(theme, "spacing.2", "spacing.2.5"))
                {
                    VerticalContentAlignment = VerticalAlignment.Center,
                    MinHeightToken = "frame.titlebar.size",
                    NoWrap = true,
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
            ["SidebarMenuBadge"] = n => new SidebarBadge { Text = n.Text },
            ["SidebarMenuButton"] = n => SidebarRow(n),
            ["SidebarMenuItem"] = n =>
            {
                // The badge is a specimen of its own at the row's end, not
                // text folded into the button.
                var badge = n.Children.FirstOrDefault(child => child.Element() == "SidebarMenuBadge");
                var trailing = badge is null ? null : Nodes.Build(badge, n.Context);
                var item = new Stack(Orientation.Vertical, "spacing.0");
                foreach (var child in n.Children.Where(child => child.Element() != "SidebarMenuBadge"))
                    if ((child.Element() == "SidebarMenuButton" ? Nodes.Drawn(child!, SidebarRow(new Node(child!, n.Context), trailing)) : Nodes.Build(child, n.Context)) is UIElement row)
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
                    splitter.Panels.Add(new SplitterPanel(Nodes.Drawn(child!, body), panel.Number("defaultSize") ?? 1));
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
                var table = new Table { Tint = n.Option("tint", WidgetTint.Primary) };
                foreach (var child in n.Kids()) table.Children.Add(child);
                Name(table, n.String("aria-label"));
                n.Live(table, () => table.Tint = n.Option("tint", WidgetTint.Primary));
                return table;
            },
            ["TableHead"] = n =>
            {
                var head = new TableHead();
                foreach (var child in n.Kids()) head.Children.Add(child);
                Name(head, n.String("aria-label"));
                return head;
            },
            ["TableRow"] = n =>
            {
                var row = new TableRow { Active = n.Flag("active"), Interactive = n.Flag("interactive") };
                foreach (var child in n.Kids()) row.Children.Add(child);
                row.Click += (_, _) => n.Press()();
                Name(row, n.String("aria-label"));
                n.Live(row, () => { row.Active = n.Flag("active"); row.Interactive = n.Flag("interactive"); });
                return row;
            },
            ["TableCell"] = n =>
            {
                var words = n.Children.All(child => child.String() is not null || Nodes.IsLive(child));
                var cell = new TableCell { Head = n.Flag("head"), Align = Align(n.String("align")), Content = words ? n.Text : Kid(n) };
                Name(cell, n.String("aria-label"));
                n.Live(cell, () =>
                {
                    cell.Head = n.Flag("head");
                    cell.Align = Align(n.String("align"));
                    if (words) cell.Content = n.Text;
                });
                return cell;
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
                    Rows = (int)(n.Number("rows") ?? 2),
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
                    // A label that is a glyph is a specimen of its own, so
                    // the element drawn for it is reported as that node.
                    toggle.Content = glyph is null ? item?["label"].Text()
                        : item?["label"] is JsonNode label ? Nodes.Drawn(label, Glyphs.Draw(glyph))
                        : Glyphs.Draw(glyph);
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

    /// <summary>
    /// A button's children in a line: a glyph beside its label, and the shortcut
    /// after both. Where the control <paramref name="owned"/> its glyphs, as
    /// Button does, the line is a bare StackPanel and the glyphs unsized: the
    /// button sizes them, spaces the line and trims the padding at a glyph's end.
    /// </summary>
    private static object? Inline(Node n, JsonNode? shortcut, bool owned = false)
    {
        double? size = null;
        if (shortcut is null && n.Children.Count == 1 && n.Children[0].String() is string only) return only;
        // A glyph alone is the control's whole face.
        if (shortcut is null && n.Children.Count == 1 && n.Children[0].Element() == "Icon")
            return Nodes.Drawn(n.Children[0]!, Glyphs.Draw(Nodes.GlyphOf(n.Children[0]) ?? "", size));
        var stack = owned ? null : new Stack(Orientation.Horizontal, "spacing.1.5") { VerticalContentAlignment = VerticalAlignment.Center };
        var panel = owned ? new StackPanel { Orientation = Orientation.Horizontal } : null;
        void Add(UIElement element)
        {
            if (element is FrameworkElement framed) framed.VerticalAlignment = VerticalAlignment.Center;
            if (stack is not null) stack.Add(element); else panel!.Children.Add(element);
        }
        foreach (var child in n.Children)
        {
            var element = child.Element() == "Icon"
                ? Nodes.Drawn(child!, Glyphs.Draw(Nodes.GlyphOf(child) ?? "", size))
                : Nodes.Build(child, n.Context);
            if (element is not null) Add(element);
        }
        if (shortcut is not null)
            Add(Nodes.Drawn(shortcut, new KeyCap(KeyText(shortcut)) { Variant = KeyCapVariant.Hint }));
        return (object?)stack ?? panel;
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

    /// <summary>
    /// A slider's reading: after the track, in the code face at the caption
    /// step, in the secondary ink, right-aligned in a column of its own.
    /// </summary>
    private static TextBlock Reading()
    {
        var reading = new TextBlock { TextAlignment = TextAlignment.Right, MinWidth = 40, VerticalAlignment = VerticalAlignment.Center, LineStackingStrategy = LineStackingStrategy.BlockLineHeight };
        Themed.Follow(reading, theme =>
        {
            var caption = theme.Text("", "captionSmall");
            reading.FontFamily = theme.CodeFont;
            reading.FontSize = caption.FontSize;
            reading.LineHeight = caption.FontSize;
            reading.Foreground = theme.Brush("color.content-muted");
        });
        return reading;
    }

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

    private static FlyoutPlacementMode Side(string? side, FlyoutPlacementMode fallback) => side switch
    {
        "top" => FlyoutPlacementMode.Top,
        "bottom" => FlyoutPlacementMode.Bottom,
        "left" => FlyoutPlacementMode.Left,
        "right" => FlyoutPlacementMode.Right,
        _ => fallback,
    };

    /* Dates ----------------------------------------------------------------
     * A story writes a date as local `YYYY-MM-DDTHH:MM`, the way codegen
     * writes `new Date(2026, 9, 16, 14, 30)`, and a slot a calendar writes
     * back takes the same text, so a round trip through a scene's state reads
     * as it was written. */

    private const string DateFormat = "yyyy-MM-dd'T'HH:mm";

    private static DateTime? ReadMoment(JsonNode? value) =>
        value.String() is string text && DateTime.TryParseExact(text, DateFormat, System.Globalization.CultureInfo.InvariantCulture, System.Globalization.DateTimeStyles.None, out var at) ? at : null;

    private static DateOnly? ReadDay(JsonNode? value) => ReadMoment(value) is DateTime at ? DateOnly.FromDateTime(at) : null;

    private static JsonNode MomentJson(DateTime at) => JsonValue.Create(at.ToString(DateFormat, System.Globalization.CultureInfo.InvariantCulture));

    private static JsonNode DayJson(DateOnly day) => MomentJson(day.ToDateTime(TimeOnly.MinValue));

    private static DateRange ReadDateRange(JsonNode? value) => new(ReadDay(value?["start"]), ReadDay(value?["end"]));

    private static JsonNode DateRangeJson(DateRange range) => new JsonObject
    {
        ["start"] = range.Start is DateOnly start ? DayJson(start) : null,
        ["end"] = range.End is DateOnly end ? DayJson(end) : null,
    };

    private static DateRangeValue? ReadDateRangeValue(JsonNode? value) =>
        ReadMoment(value?["start"]) is DateTime start && ReadMoment(value?["end"]) is DateTime end
            ? new DateRangeValue(start, end, value?["allDay"].Truthy() ?? false)
            : null;

    private static JsonNode DateRangeValueJson(DateRangeValue value) => new JsonObject
    {
        ["start"] = MomentJson(value.Start),
        ["end"] = MomentJson(value.End),
        ["allDay"] = JsonValue.Create(value.AllDay),
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
        // The web's header is floored at 44px, and its title is one line at
        // leading 1 rather than the title step's own 20px line.
        var head = new Stack(Orientation.Vertical, "spacing.1", theme => Inset(theme, "spacing.3", "spacing.5")) { BorderBottom = true, MinHeightToken = "spacing.11" };
        head.Add(new Styled(title, "titleSmall") { Weight = "labelStrong", LeadingScale = 1 });
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
                BorderTop = true,
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
                        var bar = new Stack(Orientation.Vertical, "spacing.0", theme => Inset(theme, "spacing.3", "spacing.5")) { BackgroundToken = "color.surface-chrome", BorderTop = true };
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
            MetaView: item?["meta"] is JsonNode view && view.Element() is not null ? () => Nodes.Build(view, n.Context)! : null,
            IconView: item?["icon"] is JsonNode icon ? () => Nodes.Build(icon, n.Context)! : null,
            LabelView: item?["label"] is JsonNode label && label.Element() is not null ? () => Nodes.Build(label, n.Context)! : null);
        var tree = new Tree { Size = n.Option("size", ControlSize.Medium), Tint = n.Option("tint", WidgetTint.Primary) };
        foreach (var item in n["items"].Items()) tree.Nodes.Add(Item(item));
        var changed = n.Handler("onSelectedChange");
        tree.Selected += (_, node) => changed?.Invoke(JsonValue.Create(node.Id));
        var opened = n.Handler("onExpandedChange");
        tree.ExpandedChanged += (_, ids) => opened?.Invoke(new JsonArray(ids.Select(id => (JsonNode?)JsonValue.Create(id)).ToArray()));
        n.Live(tree, () => { tree.SelectedId = n.String("selected"); if (n.Has("expanded")) tree.ExpandedIds = n["expanded"].Strings(); });
        Name(tree, n.String("aria-label"));
        return tree;
    }

    /* Sidebar ------------------------------------------------------------- */

    /// <summary>A sidebar row: a navigation item, with the element its item carries at its end.</summary>
    private static UIElement SidebarRow(Node n, UIElement? trailing = null)
    {
        // Draw the story's icon node as an element of its own, so the
        // collector reads it where React draws it.
        var leading = n.Props["icon"] is JsonNode iconNode ? Nodes.Build(iconNode, n.Context) : null;
        var glyph = leading is null ? Nodes.GlyphOf(n.Props["icon"]) : null;
        var row = new NavItem
        {
            Content = n.Text,
            Leading = leading,
            Glyph = glyph is null ? null : Glyphs.All[glyph].Glyph,
            Trailing = trailing,
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

    /// <summary>A hairline along its top edge, as a dialog's footer draws.</summary>
    public bool BorderTop { get; init; }

    /// <summary>A hairline along its bottom edge, as a dialog's header draws.</summary>
    public bool BorderBottom { get; init; }

    /// <summary>Keep a label on one line, as a narrow sidebar heading is cut.</summary>
    public bool NoWrap { get; init; }

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
        BorderBrush = theme.Brush("color.border");
        var hairline = theme.Number("stroke.hairline");
        BorderThickness = new Thickness(0, BorderTop ? hairline : 0, 0, BorderBottom ? hairline : 0);
        if (NoWrap)
            foreach (var words in Descendants(this).OfType<TextBlock>())
                words.TextWrapping = TextWrapping.NoWrap;
    }

    private static IEnumerable<DependencyObject> Descendants(DependencyObject root)
    {
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
        {
            var child = VisualTreeHelper.GetChild(root, i);
            yield return child;
            foreach (var inner in Descendants(child)) yield return inner;
        }
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

    /// <summary>A leading as a multiple of the style's own, where the web sets one.</summary>
    public double? LeadingScale { get; init; }

    private void Apply(Theme theme)
    {
        var styled = theme.Text("", style);
        block.FontSize = styled.FontSize;
        block.LineHeight = LeadingScale is double scale ? styled.FontSize * scale : styled.LineHeight;
        block.LineStackingStrategy = LineStackingStrategy.BlockLineHeight;
        block.FontFamily = styled.FontFamily;
        TextStyle.SetWeight(block, Weight is null ? TextStyle.RequestedWeight(styled) : theme.Weight(Weight));
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
    private readonly WidthAnimation widthTween;
    private readonly List<FrameworkElement> chrome = [];

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
            if (child.Element() is "SidebarHeader" or "SidebarFooter") chrome.Add(kid);
        }
        Children.Add(frame);
        widthTween = new WidthAnimation(this);
        SizeChanged += (_, args) => Clip = new RectangleGeometry
        {
            Rect = new Windows.Foundation.Rect(0, 0, args.NewSize.Width, args.NewSize.Height),
        };
        Themed.Follow(this, theme => Apply(theme, animate: false));
        sidebar.Follow(this, _ => { if (IsLoaded) Apply(Appearance.GetTheme(this), animate: true); });
    }

    private void Apply(Theme theme, bool animate)
    {
        var framed = variant != "sidebar";
        var collapsible = sidebar.Collapsible;
        var icons = collapsible == "icon" && !sidebar.Open;
        // A header or footer has no room in a forty-eight-pixel rail; the
        // web cuts its text at the edge, which reads as a clipped word.
        foreach (var element in chrome) element.Visibility = icons ? Visibility.Collapsed : Visibility.Visible;
        var pad = framed ? theme.Number("spacing.2") : 0;
        var full = theme.Number(collapsible == "none" ? "frame.rail.width" : "frame.sidebar.width");
        var width = collapsible == "none" ? full
            : !sidebar.Open && collapsible == "offcanvas" ? 0
            : icons ? theme.Number("frame.sidebar.icon-width") + (framed ? theme.Number("spacing.4") : 0)
            : full;
        if (animate) widthTween.AnimateTo(width, theme);
        else widthTween.Set(width);
        Padding = new Thickness(pad);
        // The edge is the column's, so the content inside it gives up the
        // hairline: React's inner column is 171 in a 172 column.
        var edge = variant == "sidebar" && width > 0 ? theme.Number("stroke.hairline") : 0;
        // Closing slides the column shut over content that keeps its width;
        // the layout clips what no longer fits.
        // In icon mode the frame keeps the full icon width: its 28px rows and
        // the group's ten-pixel insets need all forty-eight, and the column's
        // hairline paints over the frame's last pixel rather than eating it.
        frame.Width = Math.Max(0, (icons ? width : full) - pad * 2 - (icons ? 0 : edge));
        var surface = theme.Brush(collapsible == "none" ? "color.surface" : "color.surface-column");
        Background = variant == "sidebar" ? surface : null;
        BorderBrush = theme.Brush("color.border");
        BorderThickness = new Thickness(0, 0, edge, 0);
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
        // The web sets the sheet in by a margin, so the column's ground shows
        // around it and the sheet itself is a step narrower and shorter.
        Margin = new Thickness(sidebar.Open ? 0 : gap, gap, gap, gap);
        Padding = new Thickness(0);
    }
}
