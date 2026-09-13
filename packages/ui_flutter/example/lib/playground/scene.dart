import 'dart:async';

import 'package:dazzui/dazzui.dart';
import 'package:fluentui_system_icons/fluentui_system_icons.dart';

import '../generated/expression.dart';
import '../storybook/gallery.dart' as gallery;
import '../storybook/stage.dart';

/// The native adapter owns widget plumbing; scene content is generated.
class GeneratedScene extends StatefulWidget {
  const GeneratedScene({super.key, required this.scene});
  final Map<String, dynamic> scene;
  @override
  State<GeneratedScene> createState() => GeneratedSceneState();
}

class GeneratedSceneState extends State<GeneratedScene> {
  late final Map<String, dynamic> values = Map.of(widget.scene['state'] ?? {});
  final Map<String, TextEditingController> _controllers = {};
  final List<Timer> _timers = [];
  BuildContext? _dialogContext;
  Route<dynamic>? _route;
  NavigatorState? _navigator;
  Map<String, dynamic> get snapshot => Map.unmodifiable(values);

  @override
  void initState() {
    super.initState();
    for (final timer in widget.scene['timers'] ?? []) {
      _timers.add(
        Timer.periodic(Duration(milliseconds: timer['every']), (_) {
          if (mounted) _assign(timer['set'], _value(timer['value']));
        }),
      );
    }
    final node = widget.scene['content'];
    if (node is Map && node['component'] == 'Dialog') {
      WidgetsBinding.instance.addPostFrameCallback((_) async {
        if (!mounted || values[node['bind']['open']] != true) return;
        await showDialog<void>(
          context: context,
          builder: (context) {
            _dialogContext = context;
            _route = ModalRoute.of(context);
            _navigator = Navigator.of(context);
            return _node(node, context);
          },
        );
        _dialogContext = null;
        if (mounted) setState(() => values[node['bind']['open']] = false);
      });
    }
  }

  @override
  void dispose() {
    for (final timer in _timers) {
      timer.cancel();
    }
    final route = _route;
    final navigator = _navigator;
    if (route != null && navigator != null) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (navigator.mounted && route.isActive) navigator.removeRoute(route);
      });
    }
    for (final controller in _controllers.values) {
      controller.dispose();
    }
    super.dispose();
  }

  void _assign(String name, dynamic value) {
    setState(() => values[name] = value);
    final controller = _controllers[name];
    if (value is String && controller != null && controller.text != value) {
      controller.value = TextEditingValue(
        text: value,
        selection: TextSelection.collapsed(offset: value.length),
      );
    }
    if (value == false &&
        _dialogContext != null &&
        name == widget.scene['content']['bind']?['open'])
      Navigator.of(_dialogContext!).pop();
  }

  dynamic _value(dynamic expression) {
    if (expression is! Map) return expression;
    if (expression.containsKey('state')) return values[expression['state']];
    if (expression.containsKey('eq'))
      return _value(expression['eq'][0]) == _value(expression['eq'][1]);
    if (expression.containsKey('concat'))
      return (expression['concat'] as List).map(_value).join();
    if (expression.containsKey('multiply'))
      return (_value(expression['multiply'][0]) as num) *
          (_value(expression['multiply'][1]) as num);
    if (expression.containsKey('add'))
      return (_value(expression['add'][0]) as num) +
          (_value(expression['add'][1]) as num);
    if (expression.containsKey('gte'))
      return (_value(expression['gte'][0]) as num) >=
          (_value(expression['gte'][1]) as num);
    if (expression.containsKey('trim')) {
      final text = _value(expression['trim']) as String;
      var start = 0, end = text.length;
      while (start < end &&
          sceneTrimCodeUnits.contains(text.codeUnitAt(start))) {
        start++;
      }
      while (end > start &&
          sceneTrimCodeUnits.contains(text.codeUnitAt(end - 1))) {
        end--;
      }
      return text.substring(start, end);
    }
    return _value(
      _value(expression['if']) == true
          ? expression['then']
          : expression['else'],
    );
  }

  Widget _node(dynamic node, BuildContext context, [bool fillWidth = false]) {
    if (node is String) return Text(node);
    final props = Map<String, dynamic>.from(node['props'] ?? {});
    for (final entry in (node['derive'] as Map? ?? {}).entries) {
      props[entry.key] = _value(entry.value);
    }
    final slots = {
      for (final entry in (node['slots'] as Map? ?? {}).entries)
        entry.key: _node(entry.value, context),
    };
    final binding = Map<String, dynamic>.from(node['bind'] ?? {});
    final children = [
      for (final child in node['children'] ?? [])
        _node(
          child,
          context,
          (['Box', 'Col'].contains(node['component']) &&
                  props['display'] == null &&
                  (props['width'] != null || fillWidth)) ||
              (node['component'] == 'Card' && fillWidth) ||
              node['component'] == 'Grid',
        ),
    ];
    final key = node['id'] == null ? null : ValueKey<String>(node['id']);
    switch (node['component']) {
      case 'Grid':
        return gallery.TileGrid(
          key: key,
          min: (props['min'] ?? 256).toDouble(),
          children: children,
        );
      case 'Table':
        return Table(key: key, children: children);
      case 'TableHead':
        return TableHead(key: key, children: children);
      case 'TableRow':
        return TableRow(key: key, children: children);
      case 'TableCell':
        return TableCell(
          key: key,
          head: props['head'] ?? false,
          align: TableCellAlign.values.byName(props['align'] ?? 'start'),
          child: children.length == 1
              ? children.single
              : Row(mainAxisSize: MainAxisSize.min, children: children),
        );
      case 'Badge':
        return Badge(
          key: key,
          size: _size(props['size'] ?? 'medium'),
          variant: BadgeVariant.values.byName(props['variant'] ?? 'tinted'),
          tint: BadgeTint.values.byName(props['tint'] ?? 'primary'),
          child: children.length == 1
              ? children.single
              : Row(mainAxisSize: MainAxisSize.min, children: children),
        );
      case 'Box':
        final vars = Theme.of(context).vars;
        return DefaultTextStyle.merge(
          style: TextStyle(
            color: props['tone'] == 'muted' ? vars.colorContentMuted : null,
          ),
          child: Container(
            key: key,
            height: props['height']?.toDouble(),
            width:
                props['width']?.toDouble() ??
                (props['grow'] == true ? double.infinity : null),
            constraints: props['maxWidth'] == null
                ? null
                : BoxConstraints(maxWidth: props['maxWidth'].toDouble()),
            padding: props['padding'] == null
                ? null
                : EdgeInsets.all(
                    props['padding'] == 'spacing.3'
                        ? vars.spacing3
                        : vars.spacing2,
                  ),
            decoration: BoxDecoration(
              color: props['surface'] == null
                  ? null
                  : props['surface'] == 'color.surface.muted'
                  ? vars.colorSurfaceMuted
                  : vars.colorSurfaceChrome,
              borderRadius: props['radius'] == null
                  ? null
                  : BorderRadius.circular(
                      props['radius'] == 'radius.large'
                          ? vars.radiusLarge
                          : vars.radiusMedium,
                    ),
            ),
            child: props['display'] != null
                ? Row(
                    mainAxisAlignment: props['justify'] == 'between'
                        ? MainAxisAlignment.spaceBetween
                        : MainAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    spacing: props['gap'] == null
                        ? 0
                        : props['gap'] == 'spacing.1'
                        ? vars.spacing1
                        : props['gap'] == 'spacing.2'
                        ? vars.spacing2
                        : vars.spacing3,
                    children: children,
                  )
                : Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: children,
                  ),
          ),
        );
      case 'Value':
        return Text(_value(node['value']).toString());
      case 'Switch':
        return Semantics(
          label: props['label'],
          child: Switch(
            key: key,
            value: binding['checked'] == null
                ? props['checked'] ?? false
                : values[binding['checked']],
            tint: SwitchTint.values.byName(props['tint'] ?? 'primary'),
            size: _size(props['size'] ?? 'medium'),
            onChanged: props['disabled'] == true
                ? null
                : (value) {
                    if (binding['checked'] != null)
                      _assign(binding['checked'], value);
                  },
          ),
        );
      case 'Checkbox':
        return Checkbox(
          key: key,
          value: binding['checked'] == null
              ? props['checked'] ?? false
              : values[binding['checked']],
          tint: CheckboxTint.values.byName(props['tint'] ?? 'primary'),
          size: _size(props['size'] ?? 'medium'),
          label: children.isEmpty ? null : children.single,
          onChanged: props['disabled'] == true
              ? null
              : (value) {
                  if (binding['checked'] != null)
                    _assign(binding['checked'], value);
                },
        );
      case 'PreferenceRow':
        return PreferenceRow(
          title: props['title'],
          subtitle: props['subtitle'],
          trailing: children.single,
        );
      case 'PreferenceSection':
        return PreferenceSection(label: props['label'], children: children);
      case 'Page':
        return gallery.DocPage(
          props['title'],
          description: props['description'],
          children: children,
        );
      case 'Section':
        return gallery.DocSection(
          props['title'],
          description: props['description'],
          children: children,
        );
      case 'Stack':
        return gallery.Specimens(children: children);
      case 'Specimen':
        return gallery.Specimen(
          props['label'],
          children: [
            if (children.any((child) => child is VerticalDivider))
              IntrinsicHeight(
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  spacing: Theme.of(context).vars.spacing3,
                  children: children,
                ),
              )
            else
              ...children,
          ],
        );
      case 'Inline':
        return KeyedSubtree(
          key: key,
          child: DefaultTextStyle.merge(
            style: TextStyle(
              color: props['tone'] == 'muted'
                  ? Theme.of(context).vars.colorContentMuted
                  : null,
              fontSize: props['fontSize'] == null
                  ? null
                  : props['fontSize'] == 'title.large'
                  ? Theme.of(context).vars.titleLarge.fontSize
                  : Theme.of(context).vars.bodySmall.fontSize,
              fontFamily: props['mono'] == true
                  ? Theme.of(context).vars.fontCode.family
                  : null,
              fontFamilyFallback: props['mono'] == true
                  ? Theme.of(context).vars.fontCode.fallback
                  : null,
            ),
            child: Wrap(
              crossAxisAlignment: WrapCrossAlignment.center,
              children: children,
            ),
          ),
        );
      case 'Icon':
        return Icon(
          props['name'] == 'Circle20Filled'
              ? FluentIcons.circle_20_filled
              : FluentIcons.arrow_download_20_regular,
          size: (props['size'] as num?)?.toDouble(),
        );
      case 'StepList':
        return StepList(key: key, children: children);
      case 'Step':
        return Step(
          key: key,
          label: props['label'],
          status: StepStatus.values.byName(props['status']),
          tint: StepTint.values.byName(props['tint'] ?? 'primary'),
          meta: props['meta'] == '' ? null : props['meta'],
        );
      case 'Progress':
        return Progress(
          key: key,
          value: props['value'] == null
              ? null
              : ((props['value'] as num).clamp(0, 100) / 100),
          size: _size(props['size'] ?? 'small'),
          tint: ProgressTint.values.byName(props['tint'] ?? 'primary'),
          gradient: props['gradient'] ?? false,
        );
      case 'Divider':
        final tone = props['tone'] == 'muted'
            ? DividerTone.muted
            : DividerTone.normal;
        return props['orientation'] == 'vertical'
            ? VerticalDivider(key: key, tone: tone)
            : Divider(key: key, tone: tone);
      case 'EmptyState':
        return EmptyState(
          key: key,
          title: props['title'],
          actions: [if (slots['action'] != null) slots['action']!],
        );
      case 'Callout':
        return Callout(
          key: key,
          size: _size(props['size'] ?? 'medium'),
          tint: CalloutTint.values.byName(props['tint'] ?? 'neutral'),
          title: props['title'] == null ? null : Text(props['title']),
          message: children.isEmpty
              ? null
              : Text((node['children'] as List).join()),
          icon: slots['icon'],
          actions: slots['action'] == null ? null : [slots['action']!],
        );
      case 'Card':
        final vars = Theme.of(context).vars;
        final content = Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          spacing: props['size'] == 'small'
              ? vars.spacing2
              : props['size'] == 'large'
              ? vars.spacing4
              : vars.spacing3,
          children: children,
        );
        return Card(
          key: key,
          size: _size(props['size'] ?? 'medium'),
          variant: CardVariant.values.byName(props['variant'] ?? 'raised'),
          tint: CardTint.values.byName(props['tint'] ?? 'primary'),
          child: fillWidth ? content : IntrinsicWidth(child: content),
        );
      case 'SectionLabel':
        return SectionLabel(
          (node['children'] as List).join(),
          key: key,
          tone: props['faint'] == true
              ? SectionLabelTone.faint
              : SectionLabelTone.normal,
          tint: props['tint'] == null
              ? null
              : SectionLabelTint.values.byName(props['tint']),
        );
      case 'Spinner':
        return Spinner(
          size: _size(props['size'] ?? 'medium'),
          tint: SpinnerTint.values.byName(props['tint'] ?? 'primary'),
          onAccent: props['onAccent'] ?? false,
        );
      case 'KeyCap':
        return KeyCap(
          (node['children'] as List).join(),
          size: _size(props['size'] ?? 'medium'),
          variant: KeyCapVariant.values.byName(props['variant'] ?? 'hint'),
        );
      case 'Row':
        return RowWrap(
          spacing: switch (props['gap']) {
            'spacing.1' => Theme.of(context).vars.spacing1,
            'spacing.2' => Theme.of(context).vars.spacing2,
            'spacing.3' => Theme.of(context).vars.spacing3,
            'spacing.4' => Theme.of(context).vars.spacing4,
            'spacing.5' => Theme.of(context).vars.spacing5,
            _ => null,
          },
          children: children,
        );
      case 'Col':
        return Col(
          width: (props['width'] as num?)?.toDouble(),
          spacing: switch (props['gap']) {
            'spacing.1' => Theme.of(context).vars.spacing1,
            'spacing.2' => Theme.of(context).vars.spacing2,
            'spacing.3' => Theme.of(context).vars.spacing3,
            'spacing.4' => Theme.of(context).vars.spacing4,
            'spacing.5' => Theme.of(context).vars.spacing5,
            _ => null,
          },
          children: children,
        );
      case 'Button':
        final action = node['on']?['press'];
        return Button(
          key: key,
          variant: ButtonVariant.values.byName(props['variant'] ?? 'recessed'),
          tint: ButtonTint.values.byName(props['tint'] ?? 'primary'),
          size: _size(props['size'] ?? 'small'),
          expand: props['expand'] ?? false,
          shortcut: props['shortcut'] == null ? null : Text(props['shortcut']),
          onPressed: props['disabled'] == true
              ? null
              : () {
                  if (action != null) _assign(action['set'], action['value']);
                },
          child: children.length == 1
              ? children.first
              : Row(
                  mainAxisSize: MainAxisSize.min,
                  spacing: switch (props['size'] ?? 'small') {
                    'medium' => Theme.of(context).vars.controlMediumGap,
                    'large' => Theme.of(context).vars.controlLargeGap,
                    _ => Theme.of(context).vars.controlSmallGap,
                  },
                  children: children,
                ),
        );
      case 'FormField':
        return FormField(
          key: key,
          label: props['label'],
          hint: props['hint'],
          invalid: props['state'] == 'error',
          child: children.single,
        );
      case 'TextField':
        final name = binding['value'] as String;
        final controller = _controllers.putIfAbsent(
          name,
          () => TextEditingController(text: values[name]),
        );
        return TextField(
          key: key,
          controller: controller,
          placeholder: props['placeholder'],
          readOnly: props['readOnly'] ?? false,
          enabled: props['disabled'] != true,
          size: _size(props['size'] ?? 'medium'),
          tint: TextFieldTint.values.byName(props['tint'] ?? 'primary'),
          state: props['state'] == 'error'
              ? TextFieldState.error
              : TextFieldState.normal,
          mono: props['mono'] ?? false,
          onChanged: (value) => _assign(name, value),
        );
      case 'Text':
        final vars = Theme.of(context).vars;
        return DefaultTextStyle.merge(
          style: vars.bodyMedium.copyWith(color: vars.colorContentSecondary),
          child: children.single,
        );
      case 'Dialog':
        return SizedBox(
          width: (props['width'] as num?)?.toDouble(),
          child: Dialog(
            tone: props['tone'] == 'danger'
                ? DialogTone.danger
                : DialogTone.normal,
            children: children,
          ),
        );
      case 'DialogHeader':
        return DialogHeader(title: props['title'], subtitle: props['subtitle']);
      case 'DialogBody':
        return DialogBody(children: children);
      case 'DialogFooter':
        return DialogFooter(children: [const Spacer(), ...children]);
      case 'ActionBar':
        final bar = ActionBar(
          align: ActionBarAlign.values.byName(props['align'] ?? 'start'),
          children: [
            for (final child in children)
              if (child is TextField) Expanded(child: child) else child,
          ],
        );
        return children.any((child) => child is VerticalDivider)
            ? IntrinsicHeight(child: bar)
            : bar;
      default:
        throw StateError(
          'Unsupported generated component: ${node['component']}',
        );
    }
  }

  WidgetSize _size(String name) => switch (name) {
    'small' => WidgetSize.small,
    'medium' => WidgetSize.medium,
    'large' => WidgetSize.large,
    _ => throw StateError('Unsupported size: $name'),
  };

  @override
  Widget build(BuildContext context) {
    final node = widget.scene['content'];
    return node is Map && node['component'] == 'Dialog'
        ? const SizedBox.shrink()
        : _node(node, context);
  }
}
