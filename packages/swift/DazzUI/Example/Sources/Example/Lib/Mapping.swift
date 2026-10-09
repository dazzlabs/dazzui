import DazzUI
import FluentIcons
import SwiftUI

/*
 * How each React component a story uses is drawn on SwiftUI.
 *
 * Every entry takes the props as the React story wrote them and hands them
 * to the kit's own view, passing React's default wherever the story leaves
 * one out. `tools/codegen/swiftui.mjs` lists the same props; a story that
 * sets one missing from both fails generation.
 */

typealias ComponentBuilder = @MainActor (Node) -> AnyView

@MainActor
let components: [String: ComponentBuilder] = [
  "Row": stageRow,
  "Col": stageCol,
  "Frame": stageFrame,
  "Text": stageText,

  "Accordion": { n in
    AnyView(
      Accordion(
        items: n.children.map { child in
          let item = Node(child, n.context)
          return AccordionItem(item.string("value") ?? "", title: item.string("title") ?? "") {
            Text(item.text)
          }
        },
        initiallyOpen: Set(strings(n["defaultValue"])),
        multiple: n.flag("multiple"),
        divided: n.flag("divided")
      )
    )
  },
  "ActionBar": { n in
    AnyView(ActionBar(align: n.option("align", ActionBarAlign.start)) { n.kids })
  },
  "Autocomplete": { n in
    AnyView(
      Held("") { text in
        Autocomplete(
          items: strings(n["items"]),
          text: text,
          placeholder: n.string("placeholder") ?? "",
          size: n.option("size", TextFieldSize.medium)
        )
      }
    )
  },
  "Avatar": { n in
    AnyView(
      Avatar(
        n.string("name") ?? "",
        size: n.option("size", AvatarSize.medium),
        shape: n.option("shape", AvatarShape.circle)
      )
    )
  },
  "AvatarGroup": { n in
    let max = n.number("max").map(Int.init) ?? n.children.count
    let shown = Array(n.children.prefix(max))
    return AnyView(
      AvatarGroup(
        size: n.option("size", AvatarSize.medium),
        overflow: n.children.count - shown.count
      ) {
        ForEach(Array(shown.enumerated()), id: \.offset) { NodeView($0.element) }
      }
    )
  },
  "Badge": { n in
    AnyView(
      Badge(
        size: n.option("size", BadgeSize.medium),
        variant: n.option("variant", BadgeVariant.tinted),
        tint: n.option("tint", Tint.primary)
      ) { Text(n.text) }
    )
  },
  "Button": { n in
    // A glyph at either end of a labelled button is the button's to size and
    // place, so it leaves the label for the `leading` or `trailing` slot.
    let ends = GlyphEnds(n)
    // A shortcut written as an element — a `KeyCap` — is drawn as one, in
    // the button's shortcut slot; a string is the string.
    if n.props["shortcut"]?["$el"] != nil {
      return AnyView(
        Button(
          size: n.option("size", ControlSize.small),
          variant: n.option("variant", ControlVariant.recessed),
          tint: n.option("tint", Tint.primary),
          fullWidth: n.flag("fullWidth"),
          leading: ends.leading,
          trailing: ends.trailing,
          enabled: !n.flag("disabled"),
          action: n.press()
        ) { InlineKids(node: ends.label) } shortcut: { n.element("shortcut") }
      )
    }
    return AnyView(
      Button(
        size: n.option("size", ControlSize.small),
        variant: n.option("variant", ControlVariant.recessed),
        tint: n.option("tint", Tint.primary),
        fullWidth: n.flag("fullWidth"),
        leading: ends.leading,
        trailing: ends.trailing,
        shortcut: keyText(n.props["shortcut"]),
        enabled: !n.flag("disabled"),
        action: n.press()
      ) { InlineKids(node: ends.label) }
    )
  },
  "Callout": { n in
    AnyView(
      Callout(
        size: n.option("size", CalloutSize.medium),
        tint: n.option("tint", Tint.neutral),
        title: n.string("title"),
        leading: { EmptyView() },
        description: { if !n.children.isEmpty { Text(n.text) } },
        trailing: { n.element("action") }
      )
    )
  },
  "Card": { n in
    AnyView(
      Card(size: n.option("size", CardSize.medium), variant: n.option("variant", CardVariant.raised)) {
        n.kids
      }
    )
  },
  "Checkbox": { n in
    AnyView(
      Held(n.flag("defaultChecked")) { checked in
        Checkbox(
          checked: checked.wrappedValue,
          indeterminate: n.flag("indeterminate"),
          onCheckedChange: { checked.wrappedValue = $0 },
          size: n.option("size", CheckboxSize.medium),
          tint: n.option("tint", Tint.primary),
          note: n.string("note"),
          enabled: !n.flag("disabled")
        ) { Text(n.text) }
      }
    )
  },
  "Code": { n in AnyView(Code(n.text)) },
  "CodeBlock": { n in
    AnyView(
      CodeBlock(
        code: n.string("code") ?? "",
        title: n.string("title"),
        language: n.string("language"),
        lineNumbers: n.flag("lineNumbers"),
        copyable: n.flag("copyable")
      ) { EmptyView() }
    )
  },
  "Collapsible": { n in
    AnyView(
      Collapsible(
        title: n.string("title") ?? "",
        initiallyOpen: n.flag("defaultOpen"),
        meta: { n.element("meta") },
        content: { n.kids }
      )
    )
  },
  "Combobox": { n in AnyView(ComboboxNode(node: n)) },
  "ContextMenu": { n in
    AnyView(ContextMenu(items: menuItems(n, n["items"])) { n.kids })
  },
  "Dialog": { n in AnyView(DialogNode(node: n)) },
  "DialogBody": { n in AnyView(DialogBody { n.kids }) },
  "DialogClose": { n in AnyView(CloseNode(node: n)) },
  "DialogFooter": { n in AnyView(DialogFooter { n.kids }) },
  "DialogHeader": { n in
    AnyView(DialogHeader(title: n.string("title") ?? "", subtitle: n.string("subtitle")) { EmptyView() })
  },
  "Divider": { n in
    AnyView(
      DazzUI.Divider(
        tone: n.option("tone", DividerTone.default),
        orientation: n.option("orientation", DividerOrientation.horizontal)
      )
    )
  },
  "Drawer": { n in AnyView(DrawerNode(node: n)) },
  "DrawerClose": { n in AnyView(CloseNode(node: n)) },
  "EmptyState": { n in
    // No action is no row at all, not an empty one with its gap above it.
    guard n.has("action") else { return AnyView(EmptyState(n.string("title") ?? "")) }
    return AnyView(EmptyState(title: n.string("title") ?? "") { n.element("action") })
  },
  "Fieldset": { n in AnyView(Fieldset(legend: n.string("legend")) { n.kids }) },
  "Form": { n in AnyView(DazzUI.Form { n.kids }) },
  "FormField": { n in
    AnyView(
      FormField(
        label: n.string("label") ?? "",
        hint: n.string("hint"),
        state: n.option("state", TextFieldState.default)
      ) { n.kids }
    )
  },
  "Icon": { n in
    AnyView(Icon(glyph(n.props["icon"] ?? .null), size: n.number("size").map { CGFloat($0) }))
  },
  "IconButton": { n in
    AnyView(
      IconButton(
        label: n.string("label") ?? "",
        size: n.option("size", ControlSize.tiny),
        variant: n.option("variant", ControlVariant.plain),
        tint: n.option("tint", Tint.neutral),
        enabled: !n.flag("disabled"),
        action: n.press()
      ) { n.kids }
    )
  },
  "KeyCap": { n in
    AnyView(
      KeyCap(
        size: n.option("size", KeyCapSize.medium),
        variant: n.option("variant", KeyCapVariant.hint)
      ) { Text(n.text) }
    )
  },
  "Link": { n in
    AnyView(
      DazzUI.Link(
        n.text,
        tint: n.option("tint", Tint.primary),
        underline: n.option("underline", LinkUnderline.hover),
        external: n.flag("external"),
        action: n.press()
      )
    )
  },
  "Menu": { n in
    AnyView(
      DazzUI.Menu(items: menuItems(n, n["items"])) { state in
        n.element("trigger").environment(\.triggerPress, state.toggle)
      }
    )
  },
  "Menubar": { n in
    AnyView(
      Menubar(
        menus: (n["menus"].array ?? []).map { menu in
          MenubarMenu(menu["label"]?.text ?? "", items: menuItems(n, menu["items"] ?? .null))
        }
      )
    )
  },
  "Meter": { n in
    let value = n.number("value") ?? 0
    return AnyView(
      Meter(
        value: value,
        size: n.option("size", MeterSize.medium),
        tint: n.option("tint", Tint.primary),
        label: n.string("label"),
        valueLabel: n.flag("showValue") ? "\(Int(value.rounded()))%" : nil
      )
    )
  },
  "NumberField": { n in
    AnyView(
      Held(n.number("defaultValue")) { value in
        NumberField(
          value: value,
          minimum: n.number("min"),
          maximum: n.number("max"),
          step: n.number("step") ?? 1,
          unit: n.string("unit"),
          size: n.option("size", TextFieldSize.medium)
        )
      }
    )
  },
  "OptionCard": { n in
    AnyView(
      OptionCard(
        selected: n.flag("selected"),
        enabled: !n.flag("disabled"),
        action: n.press(),
        title: { Text(n.string("title") ?? "") },
        description: { if let text = n.string("description") { Text(text) } }
      )
    )
  },
  "OtpField": { n in
    AnyView(
      Held("") { value in
        OtpField(
          value: value,
          length: n.number("length").map(Int.init) ?? 6,
          groupAfter: n.number("groupAfter").map(Int.init),
          size: n.option("size", TextFieldSize.medium),
          state: n.option("state", TextFieldState.default)
        )
      }
    )
  },
  "PillTabs": { n in
    AnyView(
      Held(n.string("defaultValue") ?? "") { value in
        PillTabs(
          items: (n["items"].array ?? []).map { item in
            PillTabItem(
              value: item["value"]?.text ?? "",
              label: item["label"]?.text ?? "",
              count: item["count"]?.number.map(Int.init)
            )
          },
          value: value.wrappedValue,
          onValueChange: { value.wrappedValue = $0 }
        )
      }
    )
  },
  "Popover": { n in AnyView(PopoverNode(node: n)) },
  "PopoverClose": { n in AnyView(CloseNode(node: n)) },
  "PreferenceGroup": { n in
    AnyView(PreferenceGroup(title: n.string("title") ?? "", description: n.string("description")) { n.kids })
  },
  "PreferenceRow": { n in
    AnyView(
      PreferenceRow(
        title: n.string("title") ?? "",
        subtitle: n.string("subtitle"),
        onOpen: n.action("onOpen"),
        icon: { EmptyView() },
        trailing: { n.kids }
      )
    )
  },
  "PreferenceSection": { n in
    AnyView(PreferenceSection(label: n.string("label"), footer: n.string("footer")) { n.kids })
  },
  "Preferences": { n in AnyView(Preferences { n.kids }) },
  "PreviewCard": { n in
    AnyView(
      PreviewCard(width: Length(n["width"])?.points) {
        n.element("trigger")
      } content: {
        n.kids
      }
    )
  },
  "Progress": { n in
    AnyView(
      DazzUI.Progress(
        value: n.number("value") ?? 0,
        size: n.option("size", ProgressSize.small),
        tint: n.option("tint", Tint.primary),
        gradient: n.flag("gradient")
      )
    )
  },
  "RadioGroup": { n in
    AnyView(
      Held(n.string("defaultValue") ?? "") { value in
        RadioGroup(
          options: (n["options"].array ?? []).map {
            RadioGroupOption(value: $0["value"]?.text ?? "", label: $0["label"]?.text ?? "")
          },
          value: value.wrappedValue,
          onValueChange: { value.wrappedValue = $0 },
          size: n.option("size", RadioSize.medium)
        )
      }
    )
  },
  "SearchField": { n in
    bound(n, onChange: "onValueChange", initial: "", read: { $0.text }, write: Value.string) { value in
      SearchField(
        value: value.wrappedValue,
        onValueChange: { value.wrappedValue = $0 },
        placeholder: n.string("placeholder") ?? "Search",
        accessibilityLabel: n.string("aria-label")
      ) { EmptyView() }
    }
  },
  "SectionLabel": { n in
    AnyView(
      SectionLabel(tint: n.string("tint").flatMap(Tint.init(rawValue:)), faint: n.flag("faint")) {
        Text(n.text)
      }
    )
  },
  "SegmentedControl": { n in
    bound(n, onChange: "onValueChange", initial: "", read: { $0.text }, write: Value.string) { value in
      SegmentedControl(
        items: (n["items"].array ?? []).map {
          SegmentedItem(value: $0["value"]?.text ?? "", label: $0["label"]?.text ?? "")
        },
        value: value.wrappedValue,
        onValueChange: { value.wrappedValue = $0 },
        size: n.option("size", SegmentedControlSize.medium)
      )
    }
  },
  "Select": { n in
    AnyView(
      Held(n.string("defaultValue") ?? "") { value in
        Select(
          // An entry with `options` of its own is a group under a heading.
          options: (n["options"].array ?? []).map { entry -> SelectEntry<String> in
            let option = { (o: Value) in
              SelectOption(value: o["value"]?.text ?? "", label: o["label"]?.text ?? "")
            }
            if let grouped = entry["options"]?.array {
              return .group(entry["label"]?.text ?? "", options: grouped.map(option))
            }
            return .option(option(entry))
          },
          selection: value,
          placeholder: n.string("placeholder") ?? "Choose…",
          size: n.option("size", TextFieldSize.medium),
          enabled: !n.flag("disabled")
        )
      }
    )
  },
  "ShortcutRecorder": { n in
    bound(n, onChange: "onValueChange", initial: "", read: { $0.text }, write: Value.string) { value in
      ShortcutRecorder(
        value: value.wrappedValue,
        onValueChange: { value.wrappedValue = $0 },
        onClear: n.action("onClear"),
        state: n.option("state", ShortcutRecorderState.default),
        disabled: n.flag("disabled"),
        accessibilityLabel: n.string("aria-label")
      )
    }
  },
  "Sidebar": { n in AnyView(SidebarColumn(node: n)) },
  "SidebarContent": { n in
    AnyView(
      ScrollView {
        VStack(alignment: .leading, spacing: 0) { n.kids }
          .frame(maxWidth: .infinity, alignment: .leading)
      }
      .frame(maxHeight: .infinity)
    )
  },
  "SidebarGroup": { n in AnyView(SidebarGroupNode(node: n)) },
  "SidebarGroupContent": { n in AnyView(VStack(alignment: .leading, spacing: 0) { n.kids }) },
  "SidebarGroupLabel": { n in AnyView(SidebarLabelNode(node: n)) },
  "SidebarHeader": { n in AnyView(SidebarHeaderNode(node: n)) },
  "SidebarInset": { n in AnyView(SidebarInsetNode(node: n)) },
  "SidebarMenu": { n in AnyView(SidebarMenuNode(node: n)) },
  "SidebarMenuBadge": { n in AnyView(SidebarMenuBadge(n.text)) },
  "SidebarMenuButton": { n in AnyView(SidebarRow(node: n)) },
  "SidebarMenuItem": { n in AnyView(SidebarItem(node: n)) },
  "SidebarProvider": { n in AnyView(SidebarProviderNode(node: n)) },
  "SidebarTrigger": { _ in AnyView(SidebarTriggerNode()) },
  "Skeleton": { n in
    let width = Length(n["width"])
    let bone = Skeleton(
      shape: n.option("shape", SkeletonShape.text),
      width: width?.points,
      height: n.number("height").map { CGFloat($0) }
    )
    if case .fraction = width {
      return AnyView(bone.layout(width: n["width"]))
    }
    return AnyView(bone)
  },
  "SkeletonText": { n in AnyView(SkeletonText(lines: n.number("lines").map(Int.init) ?? 3)) },
  "Slider": { n in
    let start = n["defaultValue"]
    let values = start.array?.compactMap(\.number) ?? [start.number ?? 0]
    return AnyView(
      Held(values) { values in
        DazzUI.Slider(
          values: values,
          size: n.option("size", SliderSize.medium),
          valueLabel: n.flag("showValue")
            ? values.wrappedValue.map { String(Int($0.rounded())) }.joined(separator: " – ")
            : nil
        )
      }
    )
  },
  "Spinner": { n in
    AnyView(Spinner(size: n.option("size", SpinnerSize.medium), onAccent: n.flag("onAccent")))
  },
  "Splitter": { n in
    AnyView(
      Splitter(
        orientation: n.option("orientation", SplitterOrientation.horizontal),
        panels: n.children.map { child in
          let panel = Node(child, n.context)
          return SplitterPanel(
            defaultSize: panel.number("defaultSize") ?? 1,
            minSize: panel.number("minSize") ?? 0,
            maxSize: panel.number("maxSize") ?? 100
          ) {
            // A panel is a box of its own, as React's is: it is the specimen,
            // and its content takes the room it is given.
            panel.kids
              .frame(maxWidth: .infinity, maxHeight: .infinity)
              .specimen(child["$at"]?.number.map { Int($0) })
          }
        }
      )
    )
  },
  "Step": { n in
    AnyView(
      Step(status: n.option("status", StepStatus.idle)) {
        Text(n.string("label") ?? "")
      } meta: {
        if let meta = n.string("meta") { Text(meta) }
      }
    )
  },
  "StepList": { n in AnyView(StepList { n.kids }) },
  "Switch": { n in
    AnyView(
      Held(n.flag("defaultChecked")) { checked in
        Switch(
          checked: checked.wrappedValue,
          onCheckedChange: { checked.wrappedValue = $0 },
          size: n.option("size", SwitchSize.medium),
          tint: n.option("tint", Tint.primary),
          enabled: !n.flag("disabled")
        )
        .accessibilityLabel(n.string("aria-label") ?? "")
      }
    )
  },
  "Table": { n in AnyView(DazzUI.Table { n.kids }) },
  "TableCell": { n in
    AnyView(
      TableCell(head: n.flag("head"), align: n.option("align", TableCellAlignment.start)) {
        InlineKids(node: n)
      }
    )
  },
  "TableHead": { n in AnyView(TableHead { n.kids }) },
  "TableRow": { n in AnyView(DazzUI.TableRow { n.kids }) },
  "Tag": { n in
    AnyView(
      Tag(
        n.text,
        tint: n.option("tint", Tint.primary),
        selected: n.has("selected") ? n.flag("selected") : nil,
        onSelect: n.handler("onSelectedChange").map { handle in { handle(.bool($0)) } },
        onRemove: n.action("onRemove")
      )
    )
  },
  "TextField": { n in
    AnyView(
      Held(n.string("defaultValue") ?? "") { text in
        DazzUI.TextField(
          text: n.flag("readOnly") ? .constant(text.wrappedValue) : text,
          placeholder: n.string("placeholder") ?? "",
          size: n.option("size", TextFieldSize.medium),
          state: n.option("state", TextFieldState.default),
          mono: n.flag("mono"),
          enabled: !n.flag("disabled"),
          multiline: n.flag("multiline"),
          rows: n.number("rows").map { Int($0) } ?? 2
        )
      }
    )
  },
  "ToastProvider": { n in AnyView(ToastProvider { HStack(spacing: 0) { n.kids } }) },
  "ToastViewport": { n in
    AnyView(ToasterViewport(placement: n.option("placement", ToastPlacement.bottom)))
  },
  "Toggle": { n in
    let toggle = Held(n.flag("defaultPressed")) { pressed in
      DazzUI.Toggle(
        pressed: pressed.wrappedValue,
        size: n.option("size", ControlSize.tiny),
        variant: n.option("variant", ControlVariant.plain),
        glyphOnly: n.children.allSatisfy { $0["$el"]?.string == "Icon" },
        accessibilityName: n.string("label"),
        action: { pressed.wrappedValue.toggle() }
      ) { n.kids }
    }
    guard let label = n.string("label") else { return AnyView(toggle) }
    return AnyView(Tooltip { Text(label) } content: { toggle })
  },
  "ToggleGroup": { n in
    bound(
      n,
      onChange: "onValueChange",
      initial: [String](),
      read: { strings($0) },
      write: { .array($0.map(Value.string)) }
    ) { value in
      ToggleGroup(
        items: (n["items"].array ?? []).map { item in
          ToggleItem(
            value: item["value"]?.text ?? "",
            label: item["hint"]?.text ?? "",
            icon: item["label"].flatMap(iconOf)
          )
        },
        value: value.wrappedValue,
        multiple: n.has("multiple") ? n.flag("multiple") : true,
        size: n.option("size", ControlSize.small),
        onChange: { value.wrappedValue = $0 }
      )
    }
  },
  "Tooltip": { n in
    AnyView(
      Tooltip(side: n.option("side", TooltipSide.top)) {
        Text(n.string("label") ?? "")
      } content: {
        n.kids
      }
    )
  },
  "Tree": { n in AnyView(TreeNodeView(node: n)) },
]

/* Helpers ---------------------------------------------------------------- */

func strings(_ value: Value) -> [String] {
  (value.array ?? []).map(\.text)
}

/// The glyph an icon element or a bare `$icon` names.
func iconOf(_ value: Value) -> FluentIcon? {
  if value["$icon"] != nil { return glyph(value) }
  if let icon = value["props"]?["icon"] { return glyph(icon) }
  return nil
}

/// A KeyCap element's label, which is all a button or a menu row keeps.
func keyText(_ value: Value?) -> String? {
  guard let value else { return nil }
  if let text = value.string { return text }
  return (value["children"]?.array ?? []).map(\.text).joined()
}

@MainActor
func menuItems(_ n: Node, _ items: Value) -> [MenuItem] {
  (items.array ?? []).map { item in
    MenuItem(
      item["label"]?.text ?? "",
      icon: item["icon"].flatMap(iconOf),
      shortcut: keyText(item["shortcut"]),
      checked: item["checked"].flatMap { $0 == .null ? nil : $0.truthy },
      disabled: item["disabled"]?.truthy ?? false,
      onSelect: { n.context.run(item["onSelect"] ?? .null) }
    )
  }
}

/// A labelled button's children split the way React's `glyphEnds` reads
/// them: an `Icon` first or last, beside at least one other child, is a
/// glyph at that end; what is left is the label. A lone glyph stays in the
/// label — that button is an `IconButton` in all but name. A glyph is drawn
/// through the scene, so it is still the specimen its `$at` names.
@MainActor
struct GlyphEnds {
  let leading: NodeView?
  let trailing: NodeView?
  let label: Node

  init(_ node: Node) {
    func isIcon(_ value: Value?) -> Bool { value?["$el"]?.string == "Icon" }
    var children = node.children
    let paired = children.count >= 2
    leading = paired && isIcon(children.first) ? NodeView(children.removeFirst()) : nil
    trailing = paired && isIcon(children.last) ? NodeView(children.removeLast()) : nil
    label = node.with(children: children)
  }
}

/// A button's children in a line: a glyph beside its label.
struct InlineKids: View {
  let node: Node
  @Environment(\.theme) private var theme

  var body: some View {
    if node.children.count == 1 {
      node.kids
    } else {
      HStack(spacing: theme.vars.spacing15) { node.kids }
    }
  }
}

/* Presented --------------------------------------------------------------- */

/// What a presented layer must carry with it, since it is drawn at the
/// window rather than where the scene stands.
private struct Carried: ViewModifier {
  let scene: SceneState
  let toaster: Toaster?

  func body(content: Content) -> some View {
    content.environmentObject(scene).environment(\.toaster, toaster)
  }
}

private struct DialogNode: View {
  let node: Node
  @State private var open = false

  var body: some View {
    node.element("trigger")
      .environment(\.triggerPress) { open = true }
      .dialog(isPresented: $open) {
        DazzUI.Dialog(
          width: Length(node["width"])?.points,
          tone: node.option("tone", DialogTone.default)
        ) { node.kids }
          .modifier(Carried(scene: node.context.scene, toaster: node.context.toaster))
      }
  }
}

private struct DrawerNode: View {
  let node: Node
  @State private var open = false

  var body: some View {
    let side: DrawerSide =
      switch node.string("side") {
      case "left": .leading
      case "top": .top
      case "bottom": .bottom
      default: .trailing
      }
    node.element("trigger")
      .environment(\.triggerPress) { open = true }
      .drawer(isPresented: $open) {
        Group {
          if node.has("footer") {
            Drawer(
              side: side,
              title: node.string("title"),
              description: node.string("description"),
              scrolls: true
            ) {
              node.element("footer")
            } content: {
              node.kids
            }
          } else {
            Drawer(
              side: side,
              title: node.string("title"),
              description: node.string("description"),
              scrolls: true
            ) {
              node.kids
            }
          }
        }
        .modifier(Carried(scene: node.context.scene, toaster: node.context.toaster))
      }
  }
}

/// A close inside a dialog, drawer or popover: its trigger, which dismisses
/// the layer it is drawn in.
private struct CloseNode: View {
  let node: Node
  @Environment(\.closeOverlay) private var close

  var body: some View {
    node.element("render").environment(\.triggerPress) { close?() }
  }
}

/// Holds the toggle a popover's trigger was handed, so its content can
/// close it — React's `PopoverClose`.
private final class PopoverToggle {
  var toggle: (() -> Void)?
}

private struct PopoverNode: View {
  let node: Node
  @State private var holder = PopoverToggle()

  var body: some View {
    let side: PopoverSide =
      switch node.string("side") {
      case "top": .top
      case "left": .leading
      case "right": .trailing
      default: .bottom
      }
    Popover(
      title: node.string("title"),
      description: node.string("description"),
      side: side,
      width: Length(node["width"])?.points
    ) { state in
      let _ = { holder.toggle = state.toggle }()
      node.element("trigger").environment(\.triggerPress, state.toggle)
    } content: {
      node.kids.environment(\.closeOverlay, CloseOverlayAction { holder.toggle?() })
    }
  }
}

private struct ComboboxNode: View {
  let node: Node
  @State private var value: String?
  @State private var query = ""

  init(node: Node) {
    self.node = node
    _value = State(initialValue: node.string("defaultValue"))
  }

  var body: some View {
    let options: [ComboboxOption<String>] = (node["options"].array ?? []).flatMap { entry in
      if let group = entry["options"]?.array {
        return group.map {
          ComboboxOption($0["value"]?.text ?? "", label: $0["label"]?.text ?? "", group: entry["label"]?.text)
        }
      }
      return [ComboboxOption(entry["value"]?.text ?? "", label: entry["label"]?.text ?? "")]
    }
    Combobox(
      options: options,
      value: $value,
      query: $query,
      placeholder: node.string("placeholder") ?? "Search…",
      emptyLabel: node.string("empty") ?? "No matches",
      clearable: node.has("clearable") ? node.flag("clearable") : true
    )
  }
}

private struct TreeNodeView: View {
  let node: Node
  @State private var expanded: [String]
  @State private var own: String?

  init(node: Node) {
    self.node = node
    _expanded = State(initialValue: strings(node["defaultExpanded"]))
  }

  private func items(_ value: Value) -> [TreeNode] {
    (value.array ?? []).map { item in
      TreeNode(
        id: item["id"]?.text ?? "",
        label: item["label"]?.text ?? "",
        icon: item["icon"].flatMap(iconOf),
        meta: item["meta"].map(keyText) ?? nil,
        children: item["children"].map(items),
        enabled: !(item["disabled"]?.truthy ?? false)
      )
    }
  }

  var body: some View {
    let selected = node.has("selected") ? node.string("selected") : own
    Tree(
      items: items(node["items"]),
      expanded: expanded,
      selected: selected,
      size: node.option("size", TreeSize.medium),
      onSelect: { id in
        own = id
        node.handler("onSelectedChange")?(.string(id))
      },
      onToggle: { id in
        if let index = expanded.firstIndex(of: id) {
          expanded.remove(at: index)
        } else {
          expanded.append(id)
        }
      }
    )
  }
}

/* Sidebar ----------------------------------------------------------------- */

private struct SidebarState {
  var open = true
  var toggle: () -> Void = {}
  var inset = false
  var showsLabels = true
  var badge: Value?
}

private struct SidebarKey: EnvironmentKey {
  static let defaultValue = SidebarState()
}

extension EnvironmentValues {
  fileprivate var sidebar: SidebarState {
    get { self[SidebarKey.self] }
    set { self[SidebarKey.self] = newValue }
  }
}

private struct SidebarProviderNode: View {
  let node: Node
  @State private var open = true
  @Environment(\.theme) private var theme

  var body: some View {
    let inset = node.children.contains {
      $0["$el"]?.string == "Sidebar" && $0["props"]?["variant"]?.string == "inset"
    }
    HStack(spacing: 0) { node.kids }
      .background(inset ? theme.vars.colorSurfaceColumn : .clear)
      .environment(\.sidebar, SidebarState(open: open, toggle: { open.toggle() }, inset: inset))
  }
}

private struct SidebarColumn: View {
  let node: Node
  @Environment(\.sidebar) private var sidebar
  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = theme.vars
    let variant = node.string("variant") ?? "sidebar"
    let collapsible = node.string("collapsible") ?? "offcanvas"
    let framed = variant != "sidebar"
    let icons = collapsible == "icon" && !sidebar.open
    let pad = framed ? vars.spacing2 : 0
    let width: CGFloat =
      switch collapsible {
      case "none": vars.frameRailWidth
      case "offcanvas" where !sidebar.open: 0
      case "icon" where !sidebar.open: vars.frameSidebarIconWidth + (framed ? vars.spacing4 : 0)
      default: vars.frameSidebarWidth
      }
    let full = collapsible == "none" ? vars.frameRailWidth : vars.frameSidebarWidth
    let surface = collapsible == "none" ? vars.colorSurface : vars.colorSurfaceColumn
    var state = sidebar
    state.showsLabels = !icons

    // A plain sidebar's edge is a border, and a border takes its width from
    // the content box.
    // A floating card's own hairline is on both sides of it.
    let edge = variant == "sidebar" ? hairline : variant == "floating" ? hairline * 2 : 0
    let column = VStack(alignment: .leading, spacing: 0) { node.kids }
      .frame(width: max(0, (icons ? width : full) - pad * 2 - edge), alignment: .leading)
      .frame(maxHeight: .infinity, alignment: .top)
      .environment(\.sidebar, state)

    return Group {
      if variant == "floating" {
        column
          .background(surface)
          .clipShape(RoundedRectangle(cornerRadius: vars.radiusLarge))
          .border(vars.colorBorder, width: hairline, cornerRadius: vars.radiusLarge)
          .tokenShadow(vars.shadowSm)
      } else {
        column
      }
    }
    .frame(width: max(0, width - pad * 2), alignment: .leading)
    .clipped()
    .padding(pad)
    .background(variant == "sidebar" ? surface : .clear)
    .overlay(alignment: .trailing) {
      if variant == "sidebar" && width > 0 {
        Rectangle().fill(vars.colorBorder).frame(width: hairline)
      }
    }
    .animation(vars.motion, value: width)
  }
}

private struct SidebarInsetNode: View {
  let node: Node
  @Environment(\.sidebar) private var sidebar
  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = theme.vars
    let content = VStack(alignment: .leading, spacing: 0) {
      ForEach(Array(node.children.enumerated()), id: \.offset) { _, child in
        NodeView(child)
          .frame(maxWidth: .infinity, alignment: .leading)
          .overlay(alignment: .bottom) {
            Rectangle().fill(vars.colorBorder).frame(height: hairline)
          }
      }
    }
    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
    .background(vars.colorSurface)

    if sidebar.inset {
      content
        .clipShape(RoundedRectangle(cornerRadius: vars.radiusLarge))
        .tokenShadow(vars.shadowSm)
        .padding(.vertical, vars.spacing2)
        .padding(.trailing, vars.spacing2)
        .padding(.leading, sidebar.open ? 0 : vars.spacing2)
    } else {
      content
    }
  }
}

private struct SidebarHeaderNode: View {
  let node: Node
  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    VStack(alignment: .leading, spacing: vars.spacing2) { node.kids }
      .padding(.vertical, vars.spacing2)
      .padding(.horizontal, vars.spacing25)
      .frame(maxWidth: .infinity, minHeight: vars.frameTitlebarSize, alignment: .leading)
  }
}

private struct SidebarGroupNode: View {
  let node: Node
  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    VStack(alignment: .leading, spacing: 0) { node.kids }
      .padding(.vertical, vars.spacing15)
      .padding(.horizontal, vars.spacing25)
      .frame(maxWidth: .infinity, alignment: .leading)
  }
}

private struct SidebarLabelNode: View {
  let node: Node
  @Environment(\.sidebar) private var sidebar
  @Environment(\.theme) private var theme

  var body: some View {
    if sidebar.showsLabels {
      // `SidebarGroupLabel` is the faint section label, stretched to the
      // group the way a flex row is.
      HStack(spacing: 0) {
        SectionLabel(faint: true) { Text(node.text) }
        Spacer(minLength: 0)
      }
      .padding(.vertical, theme.vars.spacing1)
      .padding(.horizontal, theme.vars.spacing25)
    }
  }
}

private struct SidebarMenuNode: View {
  let node: Node
  @Environment(\.theme) private var theme

  var body: some View {
    VStack(alignment: .leading, spacing: theme.vars.frameNavGap) { node.kids }
      .frame(maxWidth: .infinity, alignment: .leading)
  }
}

/// An item: its row, with the badge the item carries drawn at its end.
private struct SidebarItem: View {
  let node: Node
  @Environment(\.sidebar) private var sidebar

  var body: some View {
    let badge = node.children.first { $0["$el"]?.string == "SidebarMenuBadge" }
    let rows = node.children.filter { $0["$el"]?.string != "SidebarMenuBadge" }
    var state = sidebar
    state.badge = sidebar.showsLabels ? badge : nil
    return VStack(alignment: .leading, spacing: 0) {
      ForEach(Array(rows.enumerated()), id: \.offset) { NodeView($0.element) }
    }
    .environment(\.sidebar, state)
  }
}

private struct SidebarRow: View {
  let node: Node
  @Environment(\.sidebar) private var sidebar
  @Environment(\.theme) private var theme

  var body: some View {
    if let tooltip = node.string("tooltip"), !sidebar.showsLabels {
      Tooltip(side: .right) { Text(tooltip) } content: { row }
    } else {
      row
    }
  }

  /// The row, made with only the slots it fills: an absent glyph or count
  /// takes no room, so they are left out rather than passed empty.
  private var row: AnyView {
    let active = node.flag("active")
    let size = node.string("size").flatMap(SidebarMenuButtonSize.init(rawValue:)) ?? .medium
    let variant = node.string("variant").flatMap(SidebarMenuButtonVariant.init(rawValue:)) ?? .filled
    let tint = node.string("tint").flatMap(Tint.init(rawValue:)) ?? .primary
    let press = node.press()
    let text = sidebar.showsLabels ? node.text : ""
    // Drawn through the scene, so the glyph and the count are specimens.
    switch (node.props["icon"], sidebar.badge) {
    case (let icon?, let badge?):
      return AnyView(
        SidebarMenuButton(active: active, size: size, variant: variant, tint: tint, action: press) {
          NodeView(icon)
        } label: { Text(text) } badge: { NodeView(badge) })
    case (let icon?, nil):
      return AnyView(
        SidebarMenuButton(active: active, size: size, variant: variant, tint: tint, action: press) {
          NodeView(icon)
        } label: { Text(text) })
    case (nil, let badge?):
      return AnyView(
        SidebarMenuButton(active: active, size: size, variant: variant, tint: tint, action: press) {
          Text(text)
        } badge: { NodeView(badge) })
    case (nil, nil):
      return AnyView(
        SidebarMenuButton(active: active, size: size, variant: variant, tint: tint, action: press) {
          Text(text)
        })
    }
  }
}

private struct SidebarTriggerNode: View {
  @Environment(\.sidebar) private var sidebar

  var body: some View {
    IconButton(label: "Toggle sidebar", action: sidebar.toggle) {
      Icon(.panelLeft20Regular)
    }
  }
}
