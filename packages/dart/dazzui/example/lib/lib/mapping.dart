import 'dart:math' as math;

import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../widgets/layout.dart';
import '../widgets/stage.dart';
import 'scene.dart';

/*
 * How each React component a story uses is drawn on Flutter.
 *
 * Every entry takes the props as the React story wrote them and hands them
 * to the kit's own widget, passing React's default wherever the story leaves
 * one out — the two kits do not always default alike, and the story was
 * written against React's. `tools/codegen/flutter.mjs` lists the same props;
 * a story that sets one missing from both fails generation.
 */

typedef ComponentBuilder = Widget Function(Node node);

final Map<String, ComponentBuilder> components = {
  'Row': stageRow,
  'Col': stageCol,
  'Frame': stageFrame,
  'Text': stageText,
  'Accordion': _accordion,
  'ActionBar': (n) => ActionBar(
    align: n.option(ActionBarAlign.values, 'align', ActionBarAlign.start),
    children: n.kids,
  ),
  'Autocomplete': (n) => Held<String>(
    initial: '',
    builder: (context, value, set) => Autocomplete(
      items: _strings(n['items']),
      value: value,
      onChanged: set,
      placeholder: n.string('placeholder'),
      size: _size(n, 'medium'),
    ),
  ),
  'Avatar': (n) => Avatar(
    name: n.string('name'),
    size: _size(n, 'medium'),
    shape: n.option(AvatarShape.values, 'shape', AvatarShape.circle),
  ),
  'AvatarGroup': (n) => AvatarGroup(
    max: (n['max'] as num?)?.toInt(),
    size: _size(n, 'medium'),
    children: n.kids,
  ),
  'Badge': (n) => Badge(
    variant: n.option(BadgeVariant.values, 'variant', BadgeVariant.tinted),
    tint: n.option(BadgeTint.values, 'tint', BadgeTint.primary),
    size: _size(n, 'medium'),
    child: Text(n.text),
  ),
  'Button': _button,
  'Callout': (n) => Callout(
    tint: n.option(CalloutTint.values, 'tint', CalloutTint.neutral),
    size: _size(n, 'medium'),
    title: n.has('title') ? Text(n.string('title')!) : null,
    message: n.children.isEmpty ? null : Text(n.text),
    actions: n.has('action') ? [n.element('action')!] : null,
  ),
  'Card': (n) {
    final ThemeVariables vars = Theme.of(n.context).vars;
    final WidgetSize size = _size(n, 'medium');
    // A card is a column on the web, its gap stepping with its size.
    final double gap = switch (size.namedSize) {
      NamedSize.small => vars.spacing2,
      NamedSize.large => vars.spacing4,
      _ => vars.spacing3,
    };
    return Card(
      variant: n.option(CardVariant.values, 'variant', CardVariant.raised),
      size: size,
      child: StretchColumn(gap: gap, children: n.kids),
    );
  },
  'Checkbox': (n) => Held<bool>(
    initial: n.flag('defaultChecked'),
    builder: (context, value, set) => Checkbox(
      value: value,
      onChanged: n.flag('disabled') ? null : set,
      indeterminate: n.flag('indeterminate'),
      label: n.children.isEmpty ? null : Text(n.text),
      note: n.has('note') ? Text(n.string('note')!) : null,
      tint: n.option(CheckboxTint.values, 'tint', CheckboxTint.primary),
      size: _size(n, 'medium'),
    ),
  ),
  'Code': (n) => Code(n.text),
  'CodeBlock': (n) => CodeBlock(
    code: n.string('code')!,
    title: n.string('title'),
    language: n.string('language'),
    lineNumbers: n.flag('lineNumbers'),
    copyable: n.flag('copyable'),
  ),
  'Collapsible': (n) => Collapsible(
    title: Text(n.string('title')!),
    meta: n.element('meta'),
    initiallyOpen: n.flag('defaultOpen'),
    child: _one(n),
  ),
  'Combobox': _combobox,
  'ContextMenu': (n) =>
      ContextMenu(items: _menuItems(n, n['items']), child: _one(n)),
  'Dialog': _dialog,
  'DialogBody': (n) => DialogBody(children: n.kids),
  'DialogClose': _close,
  'DialogFooter': (n) => DialogFooter(children: n.kids),
  'DialogHeader': (n) =>
      DialogHeader(title: n.string('title')!, subtitle: n.string('subtitle')),
  'Divider': (n) {
    final DividerTone tone = n.string('tone') == 'muted'
        ? DividerTone.muted
        : DividerTone.normal;
    return n.string('orientation') == 'vertical'
        ? VerticalDivider(tone: tone)
        : Divider(tone: tone);
  },
  'Drawer': _drawer,
  'DrawerClose': _close,
  'EmptyState': (n) => EmptyState(
    title: n.string('title')!,
    actions: n.has('action') ? [n.element('action')!] : const [],
  ),
  'Fieldset': (n) => Fieldset(legend: n.string('legend'), children: n.kids),
  'Form': (n) => Form(children: n.kids),
  'FormField': (n) => FormField(
    label: n.string('label'),
    hint: n.string('hint'),
    invalid: n.string('state') == 'error',
    child: _one(n),
  ),
  'Icon': (n) => Icon(glyphOf(n.props['icon']), size: n.number('size')),
  'IconButton': (n) => IconButton(
    icon: _one(n),
    semanticsLabel: n.string('label'),
    variant: n.option(
      IconButtonVariant.values,
      'variant',
      IconButtonVariant.plain,
    ),
    tint: n.option(IconButtonTint.values, 'tint', IconButtonTint.neutral),
    size: _size(n, 'tiny'),
    onPressed: _press(n, 'onClick'),
  ),
  'KeyCap': (n) => KeyCap(
    n.text,
    variant: n.option(KeyCapVariant.values, 'variant', KeyCapVariant.hint),
    size: _size(n, 'medium'),
  ),
  'Link': (n) => Link(
    label: n.text,
    onPressed: _press(n, 'onClick'),
    tint: n.option(LinkTint.values, 'tint', LinkTint.primary),
    underline: n.option(LinkUnderline.values, 'underline', LinkUnderline.hover),
    external: n.flag('external'),
  ),
  'Menu': (n) => Menu(
    trigger: (context, state) =>
        TriggerPress(onPressed: state.toggle, child: n.element('trigger')!),
    items: _menuItems(n, n['items']),
  ),
  'Menubar': (n) => Menubar(
    menus: [
      for (final menu in n['menus'] as List<Object?>)
        MenubarMenu(
          label: '${(menu as Map)['label']}',
          items: _menuItems(n, menu['items']),
        ),
    ],
  ),
  'Meter': (n) {
    final double value = n.number('value') ?? 0;
    return Meter(
      value: value / 100,
      size: _size(n, 'medium'),
      tint: n.option(MeterTint.values, 'tint', MeterTint.primary),
      label: n.has('label') ? Text(n.string('label')!) : null,
      valueLabel: n.flag('showValue') ? '${value.round()}%' : null,
      semanticsLabel: n.string('aria-label'),
    );
  },
  'NumberField': (n) => Held<double?>(
    initial: n.number('defaultValue'),
    builder: (context, value, set) => NumberField(
      value: value,
      onChanged: set,
      min: n.number('min'),
      max: n.number('max'),
      step: n.number('step') ?? 1,
      unit: n.string('unit'),
      size: _size(n, 'medium'),
    ),
  ),
  'OptionCard': (n) => OptionCard(
    title: n.string('title')!,
    description: n.string('description'),
    selected: n.flag('selected'),
    onPressed: n.flag('disabled') ? null : (n.action('onClick') ?? () {}),
  ),
  'OtpField': (n) => Held<String>(
    initial: '',
    builder: (context, value, set) => OtpField(
      length: (n['length'] as num).toInt(),
      value: value,
      onChanged: set,
      groupAfter: (n['groupAfter'] as num?)?.toInt(),
      size: _size(n, 'medium'),
      state: _state(n),
    ),
  ),
  'PillTabs': (n) => Held<String?>(
    initial: n.string('defaultValue'),
    builder: (context, value, set) => PillTabs<String>(
      items: [
        for (final item in n['items'] as List<Object?>)
          PillTabItem(
            value: '${(item as Map)['value']}',
            label: '${item['label']}',
            count: (item['count'] as num?)?.toInt(),
          ),
      ],
      value: value,
      onChanged: set,
    ),
  ),
  'Popover': (n) => Popover(
    trigger: (context, state) =>
        TriggerPress(onPressed: state.toggle, child: n.element('trigger')!),
    title: n.has('title') ? Text(n.string('title')!) : null,
    description: n.has('description') ? Text(n.string('description')!) : null,
    side: n.option(PopoverSide.values, 'side', PopoverSide.bottom),
    width: (Length.parse(n['width']) as Px?)?.value,
    child: n.children.isEmpty ? null : _one(n),
  ),
  'PopoverClose': _close,
  'PreferenceGroup': (n) => PreferenceGroup(
    title: n.string('title')!,
    description: n.string('description'),
    children: n.kids,
  ),
  'PreferenceRow': (n) => PreferenceRow(
    title: n.string('title')!,
    subtitle: n.string('subtitle'),
    trailing: n.children.isEmpty ? null : _one(n),
    onPressed: n.action('onOpen'),
  ),
  'PreferenceSection': (n) => PreferenceSection(
    label: n.string('label'),
    footer: n.string('footer'),
    children: n.kids,
  ),
  'Preferences': (n) => Preferences(children: n.kids),
  'PreviewCard': (n) => PreviewCard(
    trigger: n.element('trigger')!,
    width: (Length.parse(n['width']) as Px?)?.value,
    child: _one(n),
  ),
  'Progress': (n) => Progress(
    value: (n.number('value') ?? 0) / 100,
    size: _size(n, 'small'),
    tint: n.option(ProgressTint.values, 'tint', ProgressTint.primary),
    gradient: n.flag('gradient'),
    semanticsLabel: n.string('aria-label'),
  ),
  'RadioGroup': (n) => Held<String?>(
    initial: n.string('defaultValue'),
    builder: (context, value, set) => RadioGroup<String>(
      options: [
        for (final option in n['options'] as List<Object?>)
          RadioItem(
            value: '${(option as Map)['value']}',
            label: Text('${option['label']}'),
          ),
      ],
      value: value,
      onChanged: set,
      size: _size(n, 'medium'),
      semanticsLabel: n.string('aria-label'),
    ),
  ),
  'SearchField': (n) => bound<String>(
    n,
    onChange: 'onValueChange',
    initial: '',
    read: (raw) => '${raw ?? ''}',
    builder: (value, set) => SearchField(
      value: value,
      onChanged: set,
      placeholder: n.string('placeholder') ?? 'Search',
      size: _size(n, 'medium'),
    ),
  ),
  'SectionLabel': (n) => SectionLabel(
    n.text,
    tone: n.flag('faint') ? SectionLabelTone.faint : SectionLabelTone.normal,
    tint: n.has('tint')
        ? SectionLabelTint.values.byName(n.string('tint')!)
        : null,
  ),
  'SegmentedControl': (n) => bound<String?>(
    n,
    onChange: 'onValueChange',
    initial: null,
    read: (raw) => raw as String?,
    builder: (value, set) => SegmentedControl<String>(
      items: [
        for (final item in n['items'] as List<Object?>)
          SegmentedItem(
            value: '${(item as Map)['value']}',
            label: '${item['label']}',
          ),
      ],
      value: value,
      onChanged: set,
      size: _size(n, 'medium'),
    ),
  ),
  'Select': (n) => Held<String?>(
    initial: n.string('defaultValue'),
    builder: (context, value, set) => Select<String>(
      options: [
        for (final option in n['options'] as List<Object?>)
          SelectOption(
            value: '${(option as Map)['value']}',
            label: '${option['label']}',
          ),
      ],
      value: value,
      onChanged: n.flag('disabled') ? null : set,
      placeholder: n.string('placeholder') ?? 'Choose…',
      size: _size(n, 'medium'),
    ),
  ),
  'ShortcutRecorder': (n) => bound<String>(
    n,
    onChange: 'onValueChange',
    initial: '',
    read: (raw) => '${raw ?? ''}',
    builder: (value, set) => ShortcutRecorder(
      value: value,
      onChanged: set,
      onClear: n.action('onClear'),
      state: _state(n),
      enabled: !n.flag('disabled'),
      semanticsLabel: n.string('aria-label'),
    ),
  ),
  'Sidebar': _sidebar,
  'SidebarContent': (n) => Expanded(
    child: SingleChildScrollView(
      padding: EdgeInsets.symmetric(
        vertical: Theme.of(n.context).vars.spacing2,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: n.kids,
      ),
    ),
  ),
  'SidebarGroup': (n) {
    final ThemeVariables vars = Theme.of(n.context).vars;
    return Padding(
      padding: EdgeInsets.symmetric(
        vertical: vars.spacing15,
        horizontal: vars.spacing25,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: n.kids,
      ),
    );
  },
  'SidebarGroupContent': (n) => Column(
    crossAxisAlignment: CrossAxisAlignment.stretch,
    children: n.kids,
  ),
  'SidebarGroupLabel': (n) {
    if (!_SidebarColumn.of(n.context).showsLabels) {
      return const SizedBox.shrink();
    }
    final ThemeVariables vars = Theme.of(n.context).vars;
    return Padding(
      padding: EdgeInsets.symmetric(
        vertical: vars.spacing1,
        horizontal: vars.spacing25,
      ),
      child: SectionLabel(n.text),
    );
  },
  'SidebarHeader': (n) {
    final ThemeVariables vars = Theme.of(n.context).vars;
    return ConstrainedBox(
      constraints: BoxConstraints(minHeight: vars.frameTitlebarSize),
      child: Padding(
        padding: EdgeInsets.symmetric(
          vertical: vars.spacing2,
          horizontal: vars.spacing25,
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.start,
          spacing: vars.spacing2,
          children: n.kids,
        ),
      ),
    );
  },
  'SidebarInset': _inset,
  'SidebarMenu': (n) => Column(
    crossAxisAlignment: CrossAxisAlignment.stretch,
    spacing: Theme.of(n.context).vars.frameNavGap,
    children: n.kids,
  ),
  'SidebarMenuBadge': (n) => Text(n.text),
  'SidebarMenuButton': (n) {
    final Map<Object?, Object?>? icon =
        n.props['icon'] as Map<Object?, Object?>?;
    final IconData? glyph = icon == null ? null : _iconData(icon);
    final Widget item = NavItem(
      label: _SidebarColumn.of(n.context).showsLabels ? n.text : '',
      icon: glyph,
      trailing: _SidebarBadge.of(n.context),
      current: n.flag('active'),
      onPressed: n.action('onClick') ?? () {},
    );
    final String? tooltip = n.string('tooltip');
    return tooltip != null && !_SidebarColumn.of(n.context).showsLabels
        ? Tooltip(label: tooltip, side: PopoverSide.right, child: item)
        : item;
  },
  'SidebarMenuItem': (n) {
    // A badge is the item's, and the row draws it at its end.
    final List<Object?> badges = [
      for (final child in n.children)
        if (child is Map && child[r'$el'] == 'SidebarMenuBadge') child,
    ];
    final List<Object?> rows = [
      for (final child in n.children)
        if (!badges.contains(child)) child,
    ];
    return _SidebarBadge(
      badge: badges.isEmpty || !_SidebarColumn.of(n.context).showsLabels
          ? null
          : buildNode(n.context, badges.single),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [for (final row in rows) buildNode(n.context, row)],
      ),
    );
  },
  'SidebarProvider': (n) => _SidebarProvider(node: n),
  'SidebarTrigger': (n) {
    final _SidebarState sidebar = _SidebarState.of(n.context);
    return IconButton(
      icon: const Icon(FluentIcons.panel_left_20_regular),
      semanticsLabel: 'Toggle sidebar',
      onPressed: sidebar.toggle,
    );
  },
  'Skeleton': (n) {
    final Length? width = Length.parse(n['width']);
    final Widget bone = Skeleton(
      shape: n.option(SkeletonShape.values, 'shape', SkeletonShape.text),
      width: width is Px ? width.value : null,
      height: n.number('height'),
    );
    return width is Fraction
        ? FractionallySizedBox(
            widthFactor: width.value,
            alignment: AlignmentDirectional.centerStart,
            child: bone,
          )
        : bone;
  },
  'SkeletonText': (n) =>
      SkeletonText(lines: (n['lines'] as num?)?.toInt() ?? 3),
  'Slider': (n) {
    final Object? start = n['defaultValue'];
    return Held<List<double>>(
      initial: start is List
          ? [for (final v in start) (v as num).toDouble()]
          : [((start as num?) ?? 0).toDouble()],
      builder: (context, values, set) => Slider(
        values: values,
        onChanged: set,
        size: _size(n, 'medium'),
        valueLabel: n.flag('showValue')
            ? values.map((v) => v.round()).join(' – ')
            : null,
        semanticsLabel: n.string('aria-label'),
      ),
    );
  },
  'Spinner': (n) =>
      Spinner(size: _size(n, 'medium'), onAccent: n.flag('onAccent')),
  'Splitter': (n) => Splitter(
    orientation: n.option(
      SplitterOrientation.values,
      'orientation',
      SplitterOrientation.horizontal,
    ),
    panels: [
      for (final child in n.children)
        _panel(Node(n.context, child! as Map<Object?, Object?>)),
    ],
  ),
  'Step': (n) => Step(
    status: StepStatus.values.byName(n.string('status')!),
    label: n.string('label')!,
    meta: n.string('meta'),
  ),
  'StepList': (n) => StepList(children: n.kids),
  'Switch': (n) => Held<bool>(
    initial: n.flag('defaultChecked'),
    builder: (context, value, set) => Switch(
      value: value,
      onChanged: n.flag('disabled') ? null : set,
      size: _size(n, 'medium'),
      tint: n.option(SwitchTint.values, 'tint', SwitchTint.primary),
    ),
  ),
  'Table': (n) => Table(children: n.kids),
  'TableCell': (n) => TableCell(
    head: n.flag('head'),
    align: n.option(TableCellAlign.values, 'align', TableCellAlign.start),
    child: _one(n),
  ),
  'TableHead': (n) => TableHead(children: n.kids),
  'TableRow': (n) => TableRow(children: n.kids),
  'Tag': (n) => Tag(
    label: Text(n.text),
    tint: n.option(TagTint.values, 'tint', TagTint.primary),
    selected: n.has('selected') ? n.flag('selected') : null,
    onSelectedChanged: n.handler<bool>('onSelectedChange'),
    onRemove: n.action('onRemove'),
  ),
  'TextField': _textField,
  'ToastProvider': (n) => ToastProvider(
    child: Row(mainAxisSize: MainAxisSize.min, children: n.kids),
  ),
  'ToastViewport': (n) => ToastViewport(
    placement: n.string('placement') == 'top'
        ? ToastPlacement.top
        : ToastPlacement.bottom,
  ),
  'Toggle': (n) {
    final Widget toggle = Toggle(
      initiallyPressed: n.flag('defaultPressed'),
      size: _size(n, 'tiny'),
      variant: n.option(ToggleVariant.values, 'variant', ToggleVariant.plain),
      semanticsLabel: n.string('label'),
      child: _one(n),
    );
    final String? label = n.string('label');
    return label == null ? toggle : Tooltip(label: label, child: toggle);
  },
  'ToggleGroup': (n) => bound<List<String>>(
    n,
    onChange: 'onValueChange',
    initial: const [],
    read: (raw) => [for (final v in raw as List<Object?>) '$v'],
    builder: (value, set) => ToggleGroup<String>(
      items: [
        for (final item in n['items'] as List<Object?>)
          ToggleItem(
            value: '${(item as Map)['value']}',
            label: buildNode(n.context, item['label']),
            semanticsLabel: item['hint'] as String?,
          ),
      ],
      value: value,
      onChanged: set,
      multiple: n.has('multiple') ? n.flag('multiple') : true,
      size: _size(n, 'small'),
    ),
  ),
  'Tooltip': (n) => Tooltip(
    label: n.string('label')!,
    side: n.option(PopoverSide.values, 'side', PopoverSide.top),
    child: _one(n),
  ),
  'Tree': (n) => bound<String?>(
    n,
    value: 'selected',
    onChange: 'onSelectedChange',
    initial: null,
    read: (raw) => raw as String?,
    builder: (value, set) => Tree<String>(
      items: _treeNodes(n, n['items']),
      initiallyExpanded: _strings(n['defaultExpanded']),
      selected: value,
      onSelectedChanged: set,
      size: _size(n, 'medium'),
      semanticsLabel: n.string('aria-label'),
    ),
  ),
};

/* Helpers ---------------------------------------------------------------- */

WidgetSize _size(Node n, String fallback) =>
    switch (n.string('size') ?? fallback) {
      'tiny' => WidgetSize.tiny,
      'small' => WidgetSize.small,
      'large' => WidgetSize.large,
      _ => WidgetSize.medium,
    };

TextFieldState _state(Node n) =>
    n.string('state') == 'error' ? TextFieldState.error : TextFieldState.normal;

List<String> _strings(Object? value) => [
  for (final item in (value as List<Object?>?) ?? const []) '$item',
];

/// The one child a component wraps.
Widget _one(Node n) {
  final List<Widget> kids = n.kids;
  if (kids.length == 1) return kids.single;
  return Row(mainAxisSize: MainAxisSize.min, children: kids);
}

/// What a press does: a trigger's own, the story's event, or nothing — a
/// React button with no handler is still a button.
VoidCallback? _press(Node n, String event) {
  if (n.flag('disabled')) return null;
  return TriggerPress.of(n.context) ?? n.action(event) ?? () {};
}

/// The glyph an icon element or a bare `$icon` names.
IconData? _iconData(Object? value) {
  if (value is! Map) return null;
  if (value.containsKey(r'$icon')) return glyphOf(value);
  final Map<Object?, Object?>? props = value['props'] as Map<Object?, Object?>?;
  return props == null ? null : glyphOf(props['icon']);
}

/// A KeyCap element's label, which is all a menu row keeps of it.
String? _keyText(Object? value) {
  if (value is String) return value;
  if (value is! Map) return null;
  return ((value['children'] as List<Object?>?) ?? const []).join();
}

List<MenuItem> _menuItems(Node n, Object? items) => [
  for (final item in (items as List<Object?>?) ?? const [])
    MenuItem(
      label: '${(item as Map)['label']}',
      icon: _iconData(item['icon']),
      shortcut: _keyText(item['shortcut']),
      checked: item['checked'] as bool?,
      enabled: item['disabled'] != true,
      onSelect: () => run(n.context, item['onSelect']),
    ),
];

List<TreeNode<String>> _treeNodes(Node n, Object? items) => [
  for (final item in (items as List<Object?>?) ?? const [])
    TreeNode<String>(
      id: '${(item as Map)['id']}',
      label: buildNode(n.context, item['label']),
      icon: _iconData(item['icon']),
      meta: item['meta'] == null ? null : buildNode(n.context, item['meta']),
      enabled: item['disabled'] != true,
      children: item['children'] == null
          ? null
          : _treeNodes(n, item['children']),
    ),
];

SplitterPanel _panel(Node n) => SplitterPanel(
  key: NodeKey(n.json),
  defaultSize: n.number('defaultSize') ?? 1,
  minSize: n.number('minSize') ?? 0,
  maxSize: n.number('maxSize') ?? 100,
  child: _one(n),
);

Widget _button(Node n) {
  final List<Widget> kids = n.kids;
  return Button(
    variant: n.option(ButtonVariant.values, 'variant', ButtonVariant.recessed),
    tint: n.option(ButtonTint.values, 'tint', ButtonTint.primary),
    size: _size(n, 'small'),
    shortcut: n.element('shortcut'),
    expand: n.flag('fullWidth'),
    onPressed: _press(n, 'onClick'),
    child: kids.length == 1
        ? kids.single
        : Row(
            mainAxisSize: MainAxisSize.min,
            spacing: Theme.of(n.context).vars.spacing15,
            children: kids,
          ),
  );
}

Widget _accordion(Node n) => Accordion(
  initiallyOpen: _strings(n['defaultValue']),
  multiple: n.flag('multiple'),
  divided: n.flag('divided'),
  items: [
    for (final child in n.children)
      _accordionItem(Node(n.context, child! as Map<Object?, Object?>)),
  ],
);

AccordionItem _accordionItem(Node n) => AccordionItem(
  key: NodeKey(n.json),
  value: n.string('value')!,
  title: Text(n.string('title')!),
  child: Text(n.text),
);

Widget _combobox(Node n) {
  final List<Object?> options = n['options'] as List<Object?>;
  final bool grouped =
      options.isNotEmpty && (options.first as Map).containsKey('options');
  ComboboxOption<String> option(Object? raw) => ComboboxOption(
    value: '${(raw as Map)['value']}',
    label: '${raw['label']}',
  );
  return Held<String?>(
    initial: n.string('defaultValue'),
    builder: (context, value, set) => Combobox<String>(
      options: grouped ? const [] : [for (final raw in options) option(raw)],
      groups: grouped
          ? [
              for (final group in options)
                ComboboxGroup(
                  label: '${(group as Map)['label']}',
                  options: [
                    for (final raw in group['options'] as List<Object?>)
                      option(raw),
                  ],
                ),
            ]
          : const [],
      value: value,
      onChanged: set,
      placeholder: n.string('placeholder') ?? 'Search…',
      emptyLabel: n.string('empty') ?? 'No matches',
      clearable: n.has('clearable') ? n.flag('clearable') : true,
    ),
  );
}

Widget _textField(Node n) => _Field(n);

/// A text field that owns the controller its `defaultValue` starts in.
class _Field extends StatefulWidget {
  const _Field(this.node);

  final Node node;

  @override
  State<_Field> createState() => _FieldState();
}

class _FieldState extends State<_Field> {
  late final TextEditingController _controller = TextEditingController(
    text: widget.node.string('defaultValue'),
  );

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final Node n = widget.node;
    final bool multiline = n.flag('multiline');
    final int rows = (n['rows'] as num?)?.toInt() ?? 2;
    return TextField(
      controller: _controller,
      placeholder: n.string('placeholder'),
      size: _size(n, 'medium'),
      state: _state(n),
      mono: n.flag('mono'),
      readOnly: n.flag('readOnly'),
      enabled: !n.flag('disabled'),
      obscureText: n.string('type') == 'password',
      keyboardType: n.string('type') == 'email'
          ? TextInputType.emailAddress
          : multiline
          ? TextInputType.multiline
          : TextInputType.text,
      maxLines: multiline ? rows : 1,
      minLines: multiline ? rows : null,
    );
  }
}

/// A close inside a dialog, drawer or popover: its trigger, which dismisses
/// the route it is drawn in.
Widget _close(Node n) => Builder(
  builder: (context) => TriggerPress(
    onPressed: () => Navigator.of(context).maybePop(),
    child: n.element('render')!,
  ),
);

Widget _dialog(Node n) {
  final ThemeData theme = Theme.of(n.context);
  final List<Object?> content = n.children;
  final Length? width = Length.parse(n['width']);
  return Builder(
    builder: (context) => TriggerPress(
      onPressed: () => showDialog<void>(
        context: context,
        builder: (dialog) => SceneScope.carry(
          context,
          Theme(
            data: theme,
            child: sized(
              Dialog(
                tone: n.string('tone') == 'danger'
                    ? DialogTone.danger
                    : DialogTone.normal,
                children: [
                  for (final child in content) buildNode(context, child),
                ],
              ),
              width: width,
            ),
          ),
        ),
      ),
      child: n.element('trigger')!,
    ),
  );
}

Widget _drawer(Node n) {
  final ThemeData theme = Theme.of(n.context);
  final DrawerSide side = switch (n.string('side')) {
    'left' => DrawerSide.start,
    'top' => DrawerSide.top,
    'bottom' => DrawerSide.bottom,
    _ => DrawerSide.end,
  };
  return Builder(
    builder: (context) => TriggerPress(
      onPressed: () => showDrawer<void>(
        context: context,
        side: side,
        builder: (drawer) => SceneScope.carry(
          context,
          Theme(
            data: theme,
            child: Drawer(
              side: side,
              title: n.string('title'),
              description: n.string('description'),
              footer: n.has('footer') ? [n.element('footer')!] : null,
              child: _one(n),
            ),
          ),
        ),
      ),
      child: n.element('trigger')!,
    ),
  );
}

/* Sidebar ----------------------------------------------------------------- */

/// Whether the window's sidebar is open, and the way to toggle it.
class _SidebarState extends InheritedWidget {
  const _SidebarState({
    required this.open,
    required this.toggle,
    required super.child,
  });

  final bool open;
  final VoidCallback toggle;

  static _SidebarState of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<_SidebarState>()!;

  @override
  bool updateShouldNotify(_SidebarState old) => old.open != open;
}

class _SidebarProvider extends StatefulWidget {
  const _SidebarProvider({required this.node});

  final Node node;

  @override
  State<_SidebarProvider> createState() => _SidebarProviderState();
}

class _SidebarProviderState extends State<_SidebarProvider> {
  bool _open = true;

  @override
  Widget build(BuildContext context) {
    final Node n = widget.node;
    final bool inset = n.children.any(
      (child) =>
          child is Map &&
          child[r'$el'] == 'Sidebar' &&
          (child['props'] as Map?)?['variant'] == 'inset',
    );
    return _SidebarState(
      open: _open,
      toggle: () => setState(() => _open = !_open),
      child: ColoredBox(
        color: inset
            ? Theme.of(context).vars.colorSurfaceColumn
            : const Color(0x00000000),
        child: _InsetVariant(
          inset: inset,
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: n.kids,
          ),
        ),
      ),
    );
  }
}

class _InsetVariant extends InheritedWidget {
  const _InsetVariant({required this.inset, required super.child});

  final bool inset;

  static bool of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<_InsetVariant>()?.inset ??
      false;

  @override
  bool updateShouldNotify(_InsetVariant old) => old.inset != inset;
}

/// What a column is showing: every label, or only the glyphs.
class _SidebarColumn extends InheritedWidget {
  const _SidebarColumn({required this.showsLabels, required super.child});

  final bool showsLabels;

  static _SidebarColumn of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<_SidebarColumn>() ??
      const _SidebarColumn(showsLabels: true, child: SizedBox.shrink());

  @override
  bool updateShouldNotify(_SidebarColumn old) => old.showsLabels != showsLabels;
}

class _SidebarBadge extends InheritedWidget {
  const _SidebarBadge({required this.badge, required super.child});

  final Widget? badge;

  static Widget? of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<_SidebarBadge>()?.badge;

  @override
  bool updateShouldNotify(_SidebarBadge old) => old.badge != badge;
}

Widget _sidebar(Node n) {
  final ThemeVariables vars = Theme.of(n.context).vars;
  final bool open = _SidebarState.of(n.context).open;
  final String variant = n.string('variant') ?? 'sidebar';
  final String collapsible = n.string('collapsible') ?? 'offcanvas';
  final bool framed = variant != 'sidebar';
  final bool icons = collapsible == 'icon' && !open;
  final double pad = framed ? vars.spacing2 : 0;

  final double width = switch (collapsible) {
    'none' => vars.frameRailWidth,
    'offcanvas' when !open => 0,
    'icon' when !open =>
      vars.frameSidebarIconWidth + (framed ? vars.spacing4 : 0),
    _ => vars.frameSidebarWidth,
  };
  final Color surface = collapsible == 'none'
      ? vars.colorSurface
      : vars.colorSurfaceColumn;
  final BorderSide edge = BorderSide(
    color: vars.colorBorder,
    width: n.context.hairlineWidth,
  );

  Widget column = Column(
    crossAxisAlignment: CrossAxisAlignment.stretch,
    children: n.kids,
  );
  column = _SidebarColumn(showsLabels: !icons, child: column);

  // The column's content keeps its open width and is cut by the column's
  // edge as it closes, the way the web's container slides under it.
  final double full = collapsible == 'none'
      ? vars.frameRailWidth
      : vars.frameSidebarWidth;
  final double content = math.max(0, (icons ? width : full) - pad * 2);

  return AnimatedContainer(
    duration: vars.motionDuration,
    curve: vars.motionEasing,
    width: width,
    padding: EdgeInsets.all(pad),
    decoration: variant == 'sidebar'
        ? BoxDecoration(
            color: surface,
            border: BorderDirectional(end: edge),
          )
        : const BoxDecoration(),
    child: ClipRect(
      child: OverflowBox(
        alignment: AlignmentDirectional.centerStart,
        minWidth: content,
        maxWidth: content,
        child: variant == 'floating'
            ? DecoratedBox(
                decoration: BoxDecoration(
                  color: surface,
                  border: Border.fromBorderSide(edge),
                  borderRadius: BorderRadius.circular(vars.radiusLarge),
                  boxShadow: vars.shadowSm,
                ),
                child: column,
              )
            : column,
      ),
    ),
  );
}

Widget _inset(Node n) {
  final ThemeVariables vars = Theme.of(n.context).vars;
  final bool inset = _InsetVariant.of(n.context);
  final bool open = _SidebarState.of(n.context).open;
  final Widget content = Column(
    crossAxisAlignment: CrossAxisAlignment.stretch,
    children: [
      for (final child in n.children)
        DecoratedBox(
          decoration: BoxDecoration(
            border: Border(
              bottom: BorderSide(
                color: vars.colorBorder,
                width: n.context.hairlineWidth,
              ),
            ),
          ),
          child: buildNode(n.context, child),
        ),
    ],
  );
  return Expanded(
    child: inset
        ? Padding(
            padding: EdgeInsetsDirectional.only(
              top: vars.spacing2,
              bottom: vars.spacing2,
              end: vars.spacing2,
              start: open ? 0 : vars.spacing2,
            ),
            child: DecoratedBox(
              decoration: BoxDecoration(
                color: vars.colorSurface,
                borderRadius: BorderRadius.circular(vars.radiusLarge),
                boxShadow: vars.shadowSm,
              ),
              child: content,
            ),
          )
        : ColoredBox(color: vars.colorSurface, child: content),
  );
}
