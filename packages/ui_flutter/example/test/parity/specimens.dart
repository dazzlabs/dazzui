import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' as material;

/// One distinct thing React drew: a class, its axes, its text.
class Probe {
  const Probe(this.className, this.data, this.text);
  final String className;
  final Map<String, String> data;
  final String text;

  bool get isField =>
      className == 'dz-text-field' ||
      className == 'dz-search-field' ||
      className == 'dz-select' ||
      className == 'dz-form-field';

  /// Where the React root is a row and the drawn box is a part inside it.
  bool get decorationIsAPart =>
      className == 'dz-checkbox' || className == 'dz-radio';

  bool get disabled =>
      data.containsKey('disabled') || data['aria-disabled'] == 'true';

  WidgetSize size(WidgetSize fallback) => switch (data['size']) {
    'tiny' => WidgetSize.tiny,
    'small' => WidgetSize.small,
    'medium' => WidgetSize.medium,
    'large' => WidgetSize.large,
    _ => fallback,
  };
}

/// The widget a probe names, or null where this port has no way to build it.
///
/// One clause per React class: the `data-*` axes become the widget's
/// arguments, by the names the design system gives them. A class this port
/// has no widget for, or an axis value its enum does not carry, returns
/// null and is counted rather than compared — the registry test already
/// says what is missing; this one measures what is there.
///
/// Only *leaf* components are built: the ones whose look is a function of
/// their own axes. A container — an action bar, a form field, a sidebar —
/// takes its size and its type from what it holds, so a probe of one says
/// more about the story than about the component; those are the
/// screenshot layer's to compare.
Widget? build(Probe probe) {
  final Map<String, String> data = probe.data;
  final Widget label = Text(probe.text);
  void noop() {}

  switch (probe.className) {
    case 'dz-button':
      return Button(
        variant: pick(ButtonVariant.values, data['variant']),
        tint: pick(ButtonTint.values, data['tint']),
        size: probe.size(WidgetSize.small),
        expand: data['full-width'] == 'true',
        onPressed: probe.disabled ? null : noop,
        child: label,
      );
    case 'dz-icon-button':
      return IconButton(
        icon: material.Icons.add,
        variant: pick(IconButtonVariant.values, data['variant']),
        tint: pick(IconButtonTint.values, data['tint']),
        size: probe.size(WidgetSize.tiny),
        onPressed: probe.disabled ? null : noop,
      );
    case 'dz-badge':
      return Badge(
        variant: pick(BadgeVariant.values, data['variant']),
        tint: pick(BadgeTint.values, data['tint']),
        size: probe.size(WidgetSize.medium),
        child: label,
      );
    case 'dz-checkbox':
      return Checkbox(
        value: data.containsKey('checked'),
        onChanged: probe.disabled ? null : (_) {},
        tint: pick(CheckboxTint.values, data['tint']) ?? CheckboxTint.primary,
        size: probe.size(WidgetSize.medium),
        label: probe.text.isEmpty ? null : label,
      );
    case 'dz-radio':
      return Radio<int>(
        value: 1,
        groupValue: data.containsKey('checked') ? 1 : 0,
        onChanged: probe.disabled ? null : (_) {},
        tint: pick(RadioTint.values, data['tint']) ?? RadioTint.primary,
        size: probe.size(WidgetSize.medium),
        label: probe.text.isEmpty ? null : label,
      );
    case 'dz-switch':
      return Switch(
        value: data.containsKey('checked'),
        onChanged: probe.disabled ? null : (_) {},
        size: probe.size(WidgetSize.medium),
      );
    case 'dz-key-cap':
      return KeyCap(
        probe.text,
        variant:
            pick(KeyCapVariant.values, data['variant']) ?? KeyCapVariant.hint,
        size: probe.size(WidgetSize.medium),
      );
    case 'dz-card':
      final CardVariant? variant = pick(CardVariant.values, data['variant']);
      if (variant == null) return null;
      return Card(
        variant: variant,
        tint: pick(CardTint.values, data['tint']) ?? CardTint.primary,
        size: probe.size(WidgetSize.medium),
        child: label,
      );
    case 'dz-callout':
      return Callout(
        tint: pick(CalloutTint.values, data['tint']) ?? CalloutTint.neutral,
        size: probe.size(WidgetSize.medium),
        message: label,
      );
    case 'dz-progress':
      return Progress(
        value: data.containsKey('indeterminate') ? null : 0.35,
        size: probe.size(WidgetSize.small),
        tint: pick(ProgressTint.values, data['tint']) ?? ProgressTint.primary,
      );
    case 'dz-spinner':
      return Loader(
        size: probe.size(WidgetSize.medium),
        tint: pick(LoaderTint.values, data['tint']) ?? LoaderTint.primary,
      );
    case 'dz-section-label':
      return SectionLabel(
        probe.text,
        tint: pick(SectionLabelTint.values, data['tint']),
      );
    case 'dz-toast':
      return Toast(
        message: probe.text,
        tint: pick(ToastTint.values, data['tint']) ?? ToastTint.neutral,
      );
    case 'dz-nav-item':
      return NavItem(
        label: probe.text,
        size: probe.size(WidgetSize.medium),
        onPressed: probe.disabled ? null : noop,
      );
    case 'dz-option-card':
      return OptionCard(
        title: probe.text,
        selected: false,
        onPressed: probe.disabled ? null : noop,
      );
    case 'dz-search-field':
      return SearchField(
        placeholder: 'Search',
        size: probe.size(WidgetSize.medium),
      );
    case 'dz-text-field':
      return TextField(
        size: probe.size(WidgetSize.medium),
        placeholder: probe.text.isEmpty ? 'Placeholder' : null,
        enabled: !probe.disabled,
        maxLines: data.containsKey('multiline') ? 3 : 1,
      );
    case 'dz-divider':
      return data['orientation'] == 'vertical'
          ? const SizedBox(height: 24, child: VerticalDivider())
          : const Divider();
    case 'dz-segmented-control':
      return SegmentedControl<int>(
        items: const [
          SegmentedItem(value: 0, label: 'Design'),
          SegmentedItem(value: 1, label: 'Code'),
        ],
        value: 0,
        onChanged: (_) {},
        size: probe.size(WidgetSize.small),
        activeStyle:
            pick(SegmentedActiveStyle.values, data['active-style']) ??
            SegmentedActiveStyle.raised,
        stretch: data['stretch'] == 'true',
      );
    case 'dz-pill-tabs':
      return PillTabs<int>(
        items: const [
          PillTabItem(value: 0, label: 'All'),
          PillTabItem(value: 1, label: 'Open'),
        ],
        value: 0,
        onChanged: (_) {},
      );
    default:
      return null;
  }
}

/// The enum value spelled the way the axis spells it, or null.
T? pick<T extends Enum>(List<T> values, String? name) {
  if (name == null) return null;
  for (final T value in values) {
    if (value.name == name) return value;
  }
  return null;
}
