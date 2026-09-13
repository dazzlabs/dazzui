import 'dart:async';

import 'package:dazzui/dazzui.dart';

import '../generated/expression.dart';
import '../generated/glyphs.dart';
import '../generated/spacing.dart';
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
    if (expression.containsKey('not')) return _value(expression['not']) != true;
    if (expression.containsKey('and')) {
      return (expression['and'] as List).every((v) => _value(v) == true);
    }
    if (expression.containsKey('or')) {
      return (expression['or'] as List).any((v) => _value(v) == true);
    }
    if (expression.containsKey('text')) {
      final num value = _value(expression['text']);
      return value == value.roundToDouble() ? '${value.round()}' : '$value';
    }
    if (expression.containsKey('lower')) {
      return (_value(expression['lower']) as String).toLowerCase();
    }
    if (expression.containsKey('length')) {
      return (_value(expression['length']) as String).runes.length;
    }
    if (expression.containsKey('contains')) {
      final list = expression['contains'] as List;
      return (_value(list[0]) as String).contains(_value(list[1]) as String);
    }
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
    // List components read their item nodes as data rather than widgets.
    final itemList = const {
      'SegmentedControl',
      'RadioGroup',
      'PillTabs',
      'ToggleGroup',
    }.contains(node['component']);
    final children = [
      if (!itemList)
        for (final child in _shown(node['children'] ?? []))
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
            height: _length(context, props['height']),
            width:
                _length(context, props['width']) ??
                (props['grow'] == true ? double.infinity : null),
            constraints: props['maxWidth'] == null
                ? null
                : BoxConstraints(
                    maxWidth: _length(context, props['maxWidth'])!,
                  ),
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
            child: props['display'] != null && props['wrap'] == true
                ? Wrap(
                    spacing: _spacing(context, props['gap']) ?? 0,
                    runSpacing: _spacing(context, props['gap']) ?? 0,
                    children: children,
                  )
                : props['display'] != null
                ? Row(
                    mainAxisAlignment: props['justify'] == 'between'
                        ? MainAxisAlignment.spaceBetween
                        : MainAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    spacing: _spacing(context, props['gap']) ?? 0,
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
      case 'RadioGroup':
        final bound = binding['value'];
        return RadioGroup<String>(
          key: key,
          semanticsLabel: props['label'],
          options: [
            for (final option in node['children'] ?? [])
              RadioItem<String>(
                value: option['props']['value'],
                label: Text(option['props']['label']),
                enabled: option['props']['disabled'] != true,
              ),
          ],
          value: bound == null ? props['value'] : values[bound],
          onChanged: (value) {
            if (bound != null && value != null) _assign(bound, value);
          },
          size: _size(props['size'] ?? 'medium'),
          tint: RadioTint.values.byName(props['tint'] ?? 'primary'),
          direction: props['direction'] == 'row'
              ? Axis.horizontal
              : Axis.vertical,
          spacing: _spacing(context, props['gap']),
        );
      case 'PillTabs':
        final bound = binding['value'];
        return Semantics(
          label: props['label'],
          child: PillTabs<String>(
            key: key,
            items: [
              for (final item in node['children'] ?? [])
                PillTabItem<String>(
                  value: item['props']['value'],
                  label: item['props']['label'],
                  count: item['props']['count'],
                  enabled: item['props']['disabled'] != true,
                ),
            ],
            value: bound == null ? props['value'] : values[bound],
            onChanged: (value) {
              if (bound != null) _assign(bound, value);
            },
            tint: PillTabsTint.values.byName(props['tint'] ?? 'primary'),
          ),
        );
      case 'ToggleGroup':
        final items = [
          for (final item in node['children'] ?? []) item as Map,
        ];
        bool pressed(Map item) => item['bind']?['pressed'] != null
            ? values[item['bind']['pressed']] == true
            : item['props']['pressed'] == true;
        final single = props['multiple'] == false;
        final bound = binding['value'];
        return Semantics(
          label: props['label'],
          child: ToggleGroup<String>(
            key: key,
            items: [
              for (final item in items)
                ToggleItem<String>(
                  value: item['props']['value'],
                  label: item['children'] == null
                      ? Text(item['props']['label'])
                      : _node((item['children'] as List).single, context),
                  semanticsLabel: item['props']['label'],
                  enabled: item['props']['disabled'] != true,
                ),
            ],
            value: single
                ? [if (bound != null && values[bound] != '') values[bound]]
                : [
                    for (final item in items)
                      if (pressed(item)) item['props']['value'] as String,
                  ],
            multiple: !single,
            onChanged: (next) {
              if (single) {
                if (bound != null)
                  _assign(bound, next.isEmpty ? '' : next.first);
                return;
              }
              for (final item in items) {
                final key = item['bind']?['pressed'];
                if (key != null) {
                  _assign(key, next.contains(item['props']['value']));
                }
              }
            },
            size: _size(props['size'] ?? 'small'),
            variant: ToggleVariant.values.byName(
              props['variant'] ?? 'recessed',
            ),
            tint: ToggleTint.values.byName(props['tint'] ?? 'primary'),
          ),
        );
      case 'OptionCard':
        return OptionCard(
          key: key,
          title: props['title'] ?? '',
          titleContent: slots['title'],
          description: props['description'],
          selected: props['selected'] ?? false,
          onPressed: props['disabled'] == true
              ? null
              : _action(node, 'press') ?? () {},
          tint: OptionCardTint.values.byName(props['tint'] ?? 'primary'),
        );
      case 'Slider':
        final valueKey = binding['value'];
        final endKey = binding['end'];
        final double first =
            ((valueKey == null ? props['value'] : values[valueKey]) as num? ??
                    0)
                .toDouble();
        final end = endKey == null ? props['end'] : values[endKey];
        final sliderValues = [
          first,
          if (end != null) (end as num).toDouble(),
        ];
        String number(double value) =>
            value == value.roundToDouble() ? '${value.round()}' : '$value';
        return Slider(
          key: key,
          values: sliderValues,
          min: (props['min'] as num? ?? 0).toDouble(),
          max: (props['max'] as num? ?? 100).toDouble(),
          step: (props['step'] as num? ?? 1).toDouble(),
          size: _size(props['size'] ?? 'medium'),
          tint: SliderTint.values.byName(props['tint'] ?? 'primary'),
          semanticsLabel: props['label'],
          // React prints the reading, a range as "a – b", a unit after one value.
          valueLabel: props['showValue'] != true
              ? null
              : props['unit'] != null
              ? '${number(first)}${props['unit']}'
              : sliderValues.map(number).join(' – '),
          onChanged: props['disabled'] == true
              ? null
              : (next) {
                  if (valueKey != null) _assign(valueKey, next.first);
                  if (endKey != null && next.length > 1) {
                    _assign(endKey, next[1]);
                  }
                },
        );
      case 'NumberField':
        final bound = binding['value'];
        final String text = bound == null
            ? props['value'] ?? ''
            : values[bound];
        return NumberField(
          key: key,
          value: text.isEmpty ? null : double.parse(text),
          min: (props['min'] as num?)?.toDouble(),
          max: (props['max'] as num?)?.toDouble(),
          step: (props['step'] as num? ?? 1).toDouble(),
          size: _size(props['size'] ?? 'medium'),
          state: props['state'] == 'error'
              ? TextFieldState.error
              : TextFieldState.normal,
          tint: TextFieldTint.values.byName(props['tint'] ?? 'primary'),
          placeholder: props['placeholder'],
          unit: props['unit'],
          onChanged: props['disabled'] == true
              ? null
              : (next) {
                  if (bound == null) return;
                  _assign(
                    bound,
                    next == null
                        ? ''
                        : next == next.roundToDouble()
                        ? '${next.round()}'
                        : '$next',
                  );
                },
        );
      case 'SearchField':
        final bound = binding['value'];
        return SearchField(
          key: key,
          value: bound == null ? '' : values[bound],
          placeholder: props['placeholder'],
          size: _size(props['size'] ?? 'medium'),
          tint: TextFieldTint.values.byName(props['tint'] ?? 'primary'),
          hint: slots['hint'],
          clearLabel: props['clearLabel'] ?? 'Clear search',
          onChanged: (value) {
            if (bound != null) _assign(bound, value);
          },
          onDismiss: _action(node, 'dismiss'),
        );
      case 'OtpField':
        final bound = binding['value'];
        return OtpField(
          key: key,
          length: props['length'] ?? 6,
          value: bound == null ? props['value'] ?? '' : values[bound],
          groupAfter: props['groupAfter'],
          mask: props['mask'] ?? false,
          size: _size(props['size'] ?? 'medium'),
          state: props['state'] == 'error'
              ? TextFieldState.error
              : TextFieldState.normal,
          tint: TextFieldTint.values.byName(props['tint'] ?? 'primary'),
          onChanged: props['disabled'] == true
              ? null
              : (value) {
                  if (bound != null) _assign(bound, value);
                },
        );
      case 'SegmentedControl':
        final bound = binding['value'];
        return Semantics(
          label: props['label'],
          child: SegmentedControl<String>(
            key: key,
            items: [
              for (final item in node['children'] ?? [])
                SegmentedItem<String>(
                  value: item['props']['value'],
                  label: item['props']['label'],
                  enabled: item['props']['disabled'] != true,
                ),
            ],
            value: bound == null ? props['value'] : values[bound],
            // An unbound control still presses; there is simply nothing to set.
            onChanged: (value) {
              if (bound != null) _assign(bound, value);
            },
            size: _size(props['size'] ?? 'medium'),
            tint: SegmentedTint.values.byName(props['tint'] ?? 'primary'),
            activeStyle: SegmentedActiveStyle.values.byName(
              props['activeStyle'] ?? 'raised',
            ),
            stretch: props['stretch'] ?? false,
          ),
        );
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
        final tone = switch (props['tone']) {
          'muted' => Theme.of(context).vars.colorContentMuted,
          'subtle' => Theme.of(context).vars.colorContentSubtle,
          _ => null,
        };
        return KeyedSubtree(
          key: key,
          child: DefaultTextStyle.merge(
            style: TextStyle(
              color: tone,
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
            // A glyph rides the same tone as the text beside it.
            child: IconTheme.merge(
              data: IconThemeData(color: tone),
              child: Wrap(
                crossAxisAlignment: WrapCrossAlignment.center,
                children: children,
              ),
            ),
          ),
        );
      case 'Icon':
        final size = props['size'];
        return Icon(
          sceneGlyphs[props['name']]!,
          size: size is num ? size.toDouble() : _spacing(context, size),
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
          spacing: _spacing(context, props['gap']),
          children: children,
        );
      case 'Col':
        return Col(
          width: _length(context, props['width']),
          spacing: _spacing(context, props['gap']),
          children: children,
        );
      case 'Tag':
        final selectedBinding = binding['selected'];
        final selectable =
            selectedBinding != null || node['on']?['select'] != null;
        final remove = _action(node, 'remove');
        final select = _action(node, 'select');
        return Tag(
          key: key,
          label: Text((node['children'] as List).whereType<String>().join()),
          icon: node['slots']?['icon'] == null
              ? null
              : sceneGlyphs[node['slots']['icon']['props']['name']],
          size: _size(props['size'] ?? 'medium'),
          variant: TagVariant.values.byName(props['variant'] ?? 'tinted'),
          tint: TagTint.values.byName(props['tint'] ?? 'primary'),
          selected: selectedBinding != null
              ? values[selectedBinding]
              : selectable
              ? props['selected'] ?? false
              : null,
          onSelectedChanged: !selectable
              ? null
              : (value) {
                  if (selectedBinding != null) _assign(selectedBinding, value);
                  select?.call();
                },
          onRemove: remove,
          removeSemanticsLabel: props['removeLabel'] ?? 'Remove',
          enabled: props['disabled'] != true,
        );
      case 'Link':
        return Link(
          key: key,
          label: (node['children'] as List).whereType<String>().join(),
          onPressed: _action(node, 'press') ?? () {},
          tint: LinkTint.values.byName(props['tint'] ?? 'primary'),
          underline: LinkUnderline.values.byName(props['underline'] ?? 'hover'),
          external: props['external'] ?? false,
          enabled: props['disabled'] != true,
        );
      case 'Meter':
        final double value = (props['value'] as num).toDouble();
        final double min = (props['min'] as num? ?? 0).toDouble();
        final double max = (props['max'] as num? ?? 100).toDouble();
        return Meter(
          key: key,
          value: value,
          min: min,
          max: max,
          size: _size(props['size'] ?? 'medium'),
          tint: MeterTint.values.byName(props['tint'] ?? 'primary'),
          label: props['label'] == null ? null : Text(props['label']),
          // React prints the reading as a whole percent of the range.
          valueLabel: props['showValue'] == true
              ? '${((value - min) / (max - min) * 100).round()}%'
              : null,
        );
      case 'Skeleton':
        final width = props['width'];
        final skeleton = Skeleton(
          key: key,
          shape: SkeletonShape.values.byName(props['shape'] ?? 'text'),
          width: width is String && width.endsWith('%')
              ? null
              : _length(context, width),
          height: _length(context, props['height']),
        );
        return width is String && width.endsWith('%')
            ? FractionallySizedBox(
                alignment: Alignment.centerLeft,
                widthFactor: double.parse(width.replaceAll('%', '')) / 100,
                child: skeleton,
              )
            : skeleton;
      case 'SkeletonText':
        return SkeletonText(key: key, lines: props['lines'] ?? 3);
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
                  if (action?['set'] != null) {
                    _assign(action['set'], action['value']);
                  }
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
      case 'IconButton':
        final press = node['on']?['press'];
        return IconButton(
          key: key,
          icon: children.single,
          semanticsLabel: props['label'],
          size: _size(props['size'] ?? 'tiny'),
          variant: IconButtonVariant.values.byName(props['variant'] ?? 'plain'),
          tint: IconButtonTint.values.byName(props['tint'] ?? 'neutral'),
          onPressed: props['disabled'] == true
              ? null
              : () {
                  if (press?['set'] != null) {
                    _assign(press['set'], press['value']);
                  }
                },
        );
      case 'Toggle':
        final bound = binding['pressed'];
        return Toggle(
          key: key,
          semanticsLabel: props['label'],
          pressed: bound == null ? props['pressed'] ?? false : values[bound],
          onPressedChanged: (value) {
            if (bound != null) _assign(bound, value);
          },
          enabled: props['disabled'] != true,
          size: _size(props['size'] ?? 'tiny'),
          variant: ToggleVariant.values.byName(props['variant'] ?? 'plain'),
          tint: ToggleTint.values.byName(props['tint'] ?? 'neutral'),
          child: children.length == 1
              ? children.single
              : Row(mainAxisSize: MainAxisSize.min, children: children),
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

  /// A spacing step named by the declaration, or null for the default.
  double? _spacing(BuildContext context, String? name) =>
      name == null ? null : sceneSpacing(Theme.of(context).vars, name);

  /// A declared length: logical pixels or a spacing step.
  double? _length(BuildContext context, dynamic value) =>
      value is num ? value.toDouble() : _spacing(context, value);

  /// Children with every `Show` resolved in place, so a hidden branch takes
  /// no slot (and no gap) in the flex it sits in.
  List<dynamic> _shown(List<dynamic> nodes) => [
    for (final child in nodes)
      if (child is Map && child['component'] == 'Show')
        if (_value(child['value']) == true)
          ..._shown(child['children'] ?? [])
        else
          ...[]
      else
        child,
  ];

  /// An event handler for a declared action; an empty action is live but
  /// assigns nothing, and an undeclared event leaves the control inert.
  VoidCallback? _action(dynamic node, String event) {
    final action = node['on']?[event];
    if (action == null) return null;
    return () {
      if (action['set'] != null) _assign(action['set'], action['value']);
    };
  }

  WidgetSize _size(String name) => switch (name) {
    'tiny' => WidgetSize.tiny,
    'small' => WidgetSize.small,
    'medium' => WidgetSize.medium,
    'large' => WidgetSize.large,
    _ => throw StateError('Unsupported size: $name'),
  };

  @override
  Widget build(BuildContext context) {
    final node = widget.scene['content'];
    if (node is Map && node['component'] == 'Dialog') {
      return const SizedBox.shrink();
    }
    final shown = _shown([node]);
    return shown.isEmpty
        ? const SizedBox.shrink()
        : _node(shown.single, context);
  }
}
