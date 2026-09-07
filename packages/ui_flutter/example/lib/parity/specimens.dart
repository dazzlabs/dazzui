import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' as material;

/// A specimen's key as a file name — the same derivation on every platform,
/// so the viewer can find one port's picture beside another's.
String renderName(String key) {
  return key
      .toLowerCase()
      .replaceAll(RegExp('[^a-z0-9]+'), '-')
      .replaceAll(RegExp(r'^-+|-+$'), '');
}

/// One distinct thing React drew: a class, its axes, its text.
class Probe {
  const Probe(this.className, this.data, this.text, this.declares);
  final String className;
  final Map<String, String> data;
  final String text;

  /// The fields the React component's own stylesheet decides.
  ///
  /// A computed style always answers — a callout root reports the page's
  /// 13px because it inherits it, having no type of its own — so holding a
  /// port to one is holding it to a value the component never chose. Only
  /// what the stylesheet declares is the component's own decision.
  final Set<String> declares;

  /// Components whose height is what they hold rather than what they are.
  ///
  /// A button is its size profile's height whatever its label; a card is as
  /// tall as its children, and those came from the story rather than from
  /// any axis this specimen carries. Comparing one is comparing two
  /// different contents, so the height is left to the screenshot layer.
  bool get heightIsContentDriven => const {
    'dz-card',
    'dz-callout',
    'dz-option-card',
    'dz-toast',
    'dz-step',
    'dz-aside',
    'dz-dialog',
    'dz-dialog-scrim',
    'dz-preference-row',
    'dz-rail',
    'dz-sidebar',
    'dz-sidebar-card',
  }.contains(className);

  /// An overlay whose box is the viewport's rather than its own: a scrim
  /// dims the window it is fixed to, and what it measures says how big the
  /// browser was.
  bool get fillsTheViewport => const {
    'dz-dialog-scrim',
    'dz-toast-viewport',
  }.contains(className);

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
        icon: Icon(material.Icons.add),
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
      // `has-description` means a title *and* a body: React sets it only
      // when both are given, so a callout carrying it is two lines.
      return Callout(
        tint: pick(CalloutTint.values, data['tint']) ?? CalloutTint.neutral,
        size: probe.size(WidgetSize.medium),
        title: data.containsKey('has-description') ? const Text('Title') : null,
        message: label,
      );
    case 'dz-progress':
      return Progress(
        value: data.containsKey('indeterminate') ? null : 0.35,
        size: probe.size(WidgetSize.small),
        tint: pick(ProgressTint.values, data['tint']) ?? ProgressTint.primary,
      );
    case 'dz-spinner':
      return Spinner(
        size: probe.size(WidgetSize.medium),
        tint: pick(SpinnerTint.values, data['tint']) ?? SpinnerTint.primary,
      );
    case 'dz-section-label':
      return SectionLabel(
        probe.text,
        tone: data['tone'] == 'faint'
            ? SectionLabelTone.faint
            : SectionLabelTone.normal,
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
        selected: data['aria-checked'] == 'true',
        tint:
            pick(OptionCardTint.values, data['tint']) ?? OptionCardTint.primary,
        onPressed: probe.disabled ? null : noop,
      );
    case 'dz-search-field':
      // A value rather than a placeholder: React's root declares the type
      // and the ink a field's *content* is set in, and a placeholder is
      // drawn in neither.
      return SearchField(
        controller: TextEditingController(
          text: probe.text.isEmpty ? 'aurora' : probe.text,
        ),
        placeholder: 'Search',
        size: probe.size(WidgetSize.medium),
      );
    case 'dz-text-field':
      return TextField(
        controller: TextEditingController(
          text: probe.text.isEmpty ? 'aurora' : probe.text,
        ),
        size: probe.size(WidgetSize.medium),
        state: data['aria-invalid'] == 'true'
            ? TextFieldState.error
            : TextFieldState.normal,
        tint: pick(TextFieldTint.values, data['tint']) ?? TextFieldTint.primary,
        // A focused specimen is a focused field: the box lifts to the paper
        // behind an accent edge, and no other state of it says so.
        autofocus: data.containsKey('focused'),
        enabled: !probe.disabled,
        maxLines: data.containsKey('multiline') ? 3 : 1,
      );
    // The containers, which are here for their edges rather than their
    // boxes: what a panel decides for itself is its ground, its outline and
    // its corner, and what it holds came from the story.
    case 'dz-aside':
      return Aside(children: [label]);
    case 'dz-rail':
      return Rail(children: [label]);
    case 'dz-sidebar':
      return Sidebar(
        children: [
          SidebarGroup(label: 'Components', children: [label]),
        ],
      );
    case 'dz-sidebar-card':
      return SidebarCard(label: 'Queue', children: [label]);
    case 'dz-dialog':
      return Dialog(
        tone: data['tone'] == 'danger' ? DialogTone.danger : DialogTone.normal,
        children: [
          DialogBody(children: [label]),
        ],
      );
    case 'dz-dialog-scrim':
      return DialogScrim(
        child: Dialog(
          children: [
            DialogBody(children: [label]),
          ],
        ),
      );
    case 'dz-preference-row':
      return PreferenceRow(
        title: probe.text.isEmpty ? 'Launch at login' : probe.text,
        // An interactive row is one that opens something, which is what
        // gives it a corner and a wash of its own.
        onPressed: data['interactive'] == 'true' ? noop : null,
      );
    case 'dz-divider':
      final DividerTone tone = data['tone'] == 'muted'
          ? DividerTone.muted
          : DividerTone.normal;
      return data['orientation'] == 'vertical'
          ? SizedBox(height: 24, child: VerticalDivider(tone: tone))
          : Divider(tone: tone);
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
