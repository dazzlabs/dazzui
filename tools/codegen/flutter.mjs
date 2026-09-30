import { CHILDREN as children, gaps } from './support.mjs'

/*
 * The Flutter example's generated sources: the stories as data, the glyphs
 * they name, and the themes the window offers.
 *
 * `SUPPORT` is the contract with the Flutter scene interpreter
 * (`example/lib/lib/mapping.dart`): every component a story uses and
 * every prop it sets must be listed here, and each listing is something the
 * interpreter draws. A story that reaches past it fails generation, naming
 * the component and the prop — a React change never lands on Flutter as a
 * quiet gap.
 */

const tint = 'tint'
const size = 'size'
const label = 'aria-label'

export const SUPPORT = {
  // The stage.
  Row: ['gap', 'width', 'align', 'justify', children],
  Col: ['gap', 'width', 'align', children],
  Frame: ['width', 'height', children],
  Text: ['tone', 'strong', tint, children],

  Accordion: ['defaultValue', 'multiple', 'divided', children],
  AccordionItem: ['value', 'title', children],
  ActionBar: ['align', children],
  Autocomplete: ['items', 'placeholder', size],
  Avatar: ['name', size, 'shape'],
  AvatarGroup: ['max', size, children],
  Badge: ['variant', tint, size, children],
  Button: [
    'variant',
    tint,
    size,
    'shortcut',
    'disabled',
    'type',
    'fullWidth',
    'onClick',
    children,
  ],
  Callout: [tint, size, 'title', 'action', children],
  Card: ['variant', size, children],
  Checkbox: [
    'defaultChecked',
    'indeterminate',
    'disabled',
    'note',
    tint,
    size,
    children,
  ],
  Code: [children],
  CodeBlock: ['code', 'title', 'language', 'lineNumbers', 'copyable'],
  Collapsible: ['title', 'defaultOpen', 'meta', children],
  Combobox: ['options', 'defaultValue', 'placeholder', 'empty', 'clearable'],
  ContextMenu: ['items', children],
  Dialog: ['width', 'tone', 'trigger', children],
  DialogBody: [children],
  DialogClose: ['render'],
  DialogFooter: [children],
  DialogHeader: ['title', 'subtitle'],
  Divider: ['tone', 'orientation'],
  Drawer: ['side', 'title', 'description', 'trigger', 'footer', children],
  DrawerClose: ['render'],
  EmptyState: ['title', 'action'],
  Fieldset: ['legend', children],
  Form: ['onSubmit', children],
  FormField: ['label', 'hint', 'state', children],
  Icon: ['icon', size],
  IconButton: ['label', 'variant', tint, size, children],
  KeyCap: ['variant', size, children],
  Link: ['href', 'external', 'underline', tint, children],
  Menu: ['trigger', 'items'],
  Menubar: ['menus'],
  Meter: ['value', 'label', 'showValue', size, tint, label],
  NumberField: ['defaultValue', 'unit', 'min', 'max', 'step', label],
  OptionCard: ['title', 'description', 'selected', 'onClick', 'disabled'],
  OtpField: ['length', 'groupAfter', size, 'state', label],
  PillTabs: ['items', 'defaultValue', label],
  Popover: ['trigger', 'title', 'description', 'width', 'side', children],
  PopoverClose: ['render'],
  PreferenceGroup: ['title', 'description', children],
  PreferenceRow: ['title', 'subtitle', 'onOpen', children],
  PreferenceSection: ['label', 'footer', children],
  Preferences: [children],
  PreviewCard: ['trigger', 'width', children],
  Progress: ['value', size, tint, 'gradient', label],
  RadioGroup: ['options', 'defaultValue', size, label],
  SearchField: ['value', 'onValueChange', 'placeholder', label],
  SectionLabel: ['faint', tint, children],
  SegmentedControl: ['items', 'value', 'onValueChange', size, label],
  Select: ['options', 'defaultValue', 'placeholder', size, 'disabled', label],
  ShortcutRecorder: [
    'value',
    'onValueChange',
    'onClear',
    'state',
    'disabled',
    label,
  ],
  Sidebar: ['variant', 'collapsible', children],
  SidebarContent: [children],
  SidebarGroup: [children],
  SidebarGroupContent: [children],
  SidebarGroupLabel: [children],
  SidebarHeader: [children],
  SidebarInset: [children],
  SidebarMenu: [children],
  SidebarMenuBadge: [children],
  SidebarMenuButton: ['icon', 'tooltip', 'active', 'onClick', children],
  SidebarMenuItem: [children],
  SidebarProvider: [children],
  SidebarTrigger: [],
  Skeleton: ['shape', 'width', 'height'],
  SkeletonText: ['lines'],
  Slider: ['defaultValue', size, 'showValue', label],
  Spinner: [size, 'onAccent'],
  Splitter: ['orientation', children],
  SplitterPanel: ['defaultSize', 'minSize', 'maxSize', children],
  Step: ['status', 'label', 'meta'],
  StepList: [children],
  Switch: ['defaultChecked', size, tint, 'disabled', label],
  Table: [children],
  TableCell: ['head', 'align', children],
  TableHead: [children],
  TableRow: [children],
  Tag: ['selected', 'onSelectedChange', 'onRemove', tint, children],
  TextField: [
    'defaultValue',
    'placeholder',
    'type',
    'required',
    'state',
    'readOnly',
    'mono',
    size,
    'disabled',
    'multiline',
    'rows',
    label,
  ],
  ToastProvider: [children],
  ToastViewport: ['placement'],
  Toggle: ['label', size, 'defaultPressed', 'variant', children],
  ToggleGroup: ['items', 'value', 'onValueChange', 'multiple', size, label],
  Tooltip: ['label', 'side', children],
  Tree: [
    'items',
    size,
    'defaultExpanded',
    'selected',
    'onSelectedChange',
    label,
  ],
}

/** Every component and prop a story sets that Flutter does not draw. */
export const unsupported = stories => gaps(stories, SUPPORT, 'Flutter')

/* Dart ------------------------------------------------------------------ */

const header =
  '// Generated by tools/codegen from the React example. Do not edit.\n'

/** A JSON value as a Dart literal. `$` is escaped: Dart would interpolate it. */
function dart(value, indent = '') {
  if (value === null) return 'null'
  if (typeof value === 'string')
    return JSON.stringify(value).replace(/\$/g, '\\$')
  if (typeof value !== 'object') return String(value)
  const inner = indent + '  '
  if (Array.isArray(value))
    return value.length
      ? `[\n${value.map(item => inner + dart(item, inner)).join(',\n')},\n${indent}]`
      : '[]'
  const entries = Object.entries(value)
  return entries.length
    ? `{\n${entries.map(([key, item]) => `${inner}${dart(key)}: ${dart(item, inner)}`).join(',\n')},\n${indent}}`
    : '{}'
}

function scene({ state, content }) {
  return `Scene(${state ? `state: ${dart(state)}, ` : ''}content: ${dart(content)})`
}

const DART_KEYWORDS = new Set(
  'assert break case catch class const continue default do else enum extends false final finally for if in is new null rethrow return super switch this throw true try var void while with'.split(
    ' '
  )
)

/** A story's file name on Flutter: the React file's, in Dart's snake case. */
export const storyFile = id => `${id.replace(/-/g, '_')}.dart`

/** One story, in a file of its own — as the React example keeps it. */
export function storyDart(story) {
  const examples = story.examples.map(
    example => `    Example(
      title: ${dart(example.title)},${example.note ? `\n      note: ${dart(example.note)},` : ''}
      demo: ${scene(example.scene)},
    ),`
  )
  return `// Generated by tools/codegen from
// packages/js/dazzui/example/src/stories/${story.id}.tsx. Do not edit.
import 'package:dazzui_example/lib/story.dart';

const story = Story(
  id: ${dart(story.id)},
  name: ${dart(story.name)},
  summary: ${dart(story.summary)},
  preview: ${scene(story.preview)},
  examples: [
${examples.join('\n')}
  ],
);
`
}

/** Every story, by name — the React example's \`stories/index.ts\`. */
export function storiesIndexDart(stories) {
  // A Dart keyword cannot be a prefix; `switch` becomes `switch_`.
  const prefix = story => {
    const name = story.id.replace(/-/g, '_')
    return DART_KEYWORDS.has(name) ? `${name}_` : name
  }
  return `${header}import 'package:dazzui_example/lib/story.dart';
${[...stories]
  .sort((a, b) => a.id.localeCompare(b.id))
  .map(story => `import '${storyFile(story.id)}' as ${prefix(story)};`)
  .join('\n')}

/// Every component the example shows, by name.
const stories = <Story>[
${stories.map(story => `  ${prefix(story)}.story,`).join('\n')}
];

Story? storyOf(String? id) =>
    stories.where((story) => story.id == id).firstOrNull;

/// The components a query leaves.
List<Story> matching([String query = '']) {
  final String needle = query.trim().toLowerCase();
  return [
    for (final story in stories)
      if (story.name.toLowerCase().contains(needle) ||
          story.summary.toLowerCase().contains(needle))
        story,
  ];
}
`
}

/** \`ArrowDownload20Regular\` → \`arrow_download_20_regular\`. */
const snake = name =>
  name
    .replace(/([a-z])([A-Z0-9])/g, '$1_$2')
    .replace(/([0-9])([A-Z])/g, '$1_$2')
    .toLowerCase()

export function glyphsDart(stories) {
  const names = new Set()
  const visit = value => {
    if (Array.isArray(value)) return value.forEach(visit)
    if (!value || typeof value !== 'object') return
    if (typeof value.$icon === 'string') names.add(value.$icon)
    Object.values(value).forEach(visit)
  }
  visit(stories)
  return `${header}import 'package:fluentui_system_icons/fluentui_system_icons.dart';
import 'package:flutter/widgets.dart';

/// The Fluent glyphs the stories name, by their React name.
const glyphs = <String, IconData>{
${[...names]
  .sort()
  .map(name => `  ${dart(name)}: FluentIcons.${snake(name)},`)
  .join('\n')}
};
`
}

const camel = id =>
  id.replace(/-([a-z0-9])/g, (_, letter) => letter.toUpperCase())

/**
 * The themes, as the React window groups them, and the generated variables
 * each one is. Studio Light is the package's own `themeVariables`; every
 * other theme is `themeVariables` and its name.
 */
export function themesDart(groups, variables) {
  const constant = id =>
    id === 'studio-light'
      ? 'themeVariables'
      : `themeVariables${camel(id).replace(/^./, c => c.toUpperCase())}`
  const ids = groups.flatMap(group =>
    group.families.flatMap(family => [
      `${family.value}-light`,
      `${family.value}-dark`,
    ])
  )
  for (const id of ids)
    if (!new RegExp(`\\bconst ${constant(id)} =`).test(variables))
      throw new Error(`The Flutter package has no variables for ${id}`)
  return `${header}import 'package:dazzui/dazzui.dart';

/// A theme: a family, in one of its two halves.
class ThemeChoice {
  const ThemeChoice(this.family, {this.dark = false});

  final String family;
  final bool dark;

  /// The theme's name, as the web's \`data-theme\` writes it.
  String get id => '$family-\${dark ? 'dark' : 'light'}';

  ThemeData get data => ThemeData(
    vars: themeVariablesOf(id),
    brightness: dark ? Brightness.dark : Brightness.light,
  );

  ThemeChoice copyWith({String? family, bool? dark}) =>
      ThemeChoice(family ?? this.family, dark: dark ?? this.dark);

  @override
  bool operator ==(Object other) =>
      other is ThemeChoice && other.family == family && other.dark == dark;

  @override
  int get hashCode => Object.hash(family, dark);
}

const defaultTheme = ThemeChoice('studio');

/// A theme family: one name, in light and in dark.
class ThemeFamily {
  const ThemeFamily(this.value, this.title);

  final String value;
  final String title;
}

class ThemeFamilyGroup {
  const ThemeFamilyGroup(this.title, this.families);

  final String title;
  final List<ThemeFamily> families;
}

const themeFamilyGroups = <ThemeFamilyGroup>[
${groups
  .map(
    group => `  ThemeFamilyGroup(${dart(group.title)}, [
${group.families.map(family => `    ThemeFamily(${dart(family.value)}, ${dart(family.title)}),`).join('\n')}
  ]),`
  )
  .join('\n')}
];

/// The variables a theme is drawn with, by its React name.
ThemeVariables themeVariablesOf(String id) => switch (id) {
${ids.map(id => `  ${dart(id)} => ${constant(id)},`).join('\n')}
  _ => throw ArgumentError.value(id, 'id', 'No such theme'),
};
`
}
