import { gaps, CHILDREN as C } from './support.mjs'
import { dateFormulaType } from './harmony-dates.mjs'

// Keep this list paired with example/entry/.../lib/SceneNode.ets. A scene
// with any gap is shown as a gap in full; it is never partly rendered.
export const SUPPORT = {
  WindowFrame: [
    C,
    'title',
    'subtitle',
    'platform',
    'titleHidden',
    'titlebar',
    'fullSizeContent',
    'controls',
    'inactive',
    'width',
    'height',
    'minWidth',
    'maxWidth',
    'minHeight',
    'maxHeight',
    'onClose',
    'onMinimize',
    'onMaximize',
  ],
  WindowTitlebar: [
    'title',
    'subtitle',
    'titleHidden',
    'controls',
    'leading',
    'toolbar',
    'trailing',
    'accessory',
    'onClose',
    'onMinimize',
    'onMaximize',
  ],
  WindowBody: [C],
  WindowMain: [C],
  WindowContent: [C],
  WindowFooter: [C],
  WindowPane: [C],
  DateRangePicker: [
    'value',
    'defaultValue',
    'onValueChange',
    'presets',
    'durations',
    'timeStep',
    'months',
    'weekStartsOn',
    'locale',
    'labels',
    'today',
    'size',
    'state',
    'tint',
    'disabled',
    'aria-label',
  ],
  Calendar: [
    'mode',
    'value',
    'defaultValue',
    'onValueChange',
    'month',
    'defaultMonth',
    'onMonthChange',
    'today',
    'min',
    'max',
    'months',
    'weekStartsOn',
    'locale',
    'navigation',
    'size',
    'tint',
    'aria-label',
  ],
  ChromeBar: [
    C,
    'width',
    'height',
    'padding',
    'background',
    'borderWidth',
    'borderColor',
  ],
  Tabs: [
    'items',
    'value',
    'onValueChange',
    'onClose',
    'trailing',
    'aria-label',
  ],
  ToastProvider: [C, 'limit'],
  ToastViewport: ['placement'],
  SidebarProvider: [C, 'open', 'defaultOpen', 'onOpenChange'],
  Sidebar: [C, 'side', 'variant', 'collapsible'],
  SidebarInset: [C],
  SidebarHeader: [C],
  SidebarContent: [C],
  SidebarGroup: [C],
  SidebarGroupLabel: [C],
  SidebarGroupContent: [C],
  SidebarMenu: [C],
  SidebarMenuItem: [C],
  SidebarMenuButton: [
    C,
    'active',
    'icon',
    'size',
    'variant',
    'tint',
    'tooltip',
    'disabled',
    'onClick',
  ],
  SidebarMenuBadge: [C, 'tint'],
  SidebarTrigger: ['label'],
  Tree: [
    'items',
    'expanded',
    'defaultExpanded',
    'onExpandedChange',
    'selected',
    'onSelectedChange',
    'size',
    'tint',
    'aria-label',
  ],
  ShortcutRecorder: [
    'value',
    'onValueChange',
    'onClear',
    'placeholder',
    'recordingLabel',
    'clearLabel',
    'state',
    'disabled',
    'tint',
    'aria-label',
  ],
  Combobox: [
    'options',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'empty',
    'clearable',
    'size',
    'disabled',
    'aria-label',
  ],
  Autocomplete: [
    'items',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'empty',
    'size',
    'disabled',
    'aria-label',
  ],
  Accordion: [C, 'defaultValue', 'multiple', 'divided'],
  AccordionItem: [C, 'value', 'title'],
  AvatarGroup: [C, 'max', 'size'],
  Link: [C, 'href', 'external', 'underline', 'tint'],
  PillTabs: ['items', 'defaultValue', 'aria-label'],
  ToggleGroup: [
    'items',
    'value',
    'onValueChange',
    'multiple',
    'size',
    'aria-label',
  ],
  Row: [C, 'gap', 'align', 'justify', 'width'],
  Col: [C, 'gap', 'align', 'width'],
  Frame: [C, 'width', 'height'],
  Text: [C, 'strong', 'tone', 'tint'],
  Icon: ['icon', 'size'],
  Button: [
    C,
    'variant',
    'tint',
    'size',
    'disabled',
    'fullWidth',
    'shortcut',
    'onClick',
    'type',
  ],
  IconButton: [C, 'label', 'size', 'variant', 'tint', 'disabled', 'onClick'],
  Toggle: [
    C,
    'label',
    'size',
    'variant',
    'tint',
    'disabled',
    'pressed',
    'defaultPressed',
    'onPressedChange',
  ],
  Badge: [C, 'variant', 'tint', 'size'],
  Avatar: ['name', 'size', 'shape', 'tint', 'fallback'],
  KeyCap: [C, 'variant', 'size'],
  Card: [C, 'variant', 'size'],
  Checkbox: [
    C,
    'checked',
    'defaultChecked',
    'indeterminate',
    'onCheckedChange',
    'disabled',
    'tint',
    'size',
    'note',
  ],
  Switch: [
    'checked',
    'defaultChecked',
    'onCheckedChange',
    'size',
    'tint',
    'disabled',
    'aria-label',
  ],
  TextField: [
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'aria-label',
    'type',
    'multiline',
    'rows',
    'state',
    'readOnly',
    'required',
    'mono',
    'size',
    'disabled',
  ],
  OtpField: [
    'length',
    'groupAfter',
    'value',
    'defaultValue',
    'onValueChange',
    'onValueComplete',
    'onValueInvalid',
    'size',
    'state',
    'tint',
    'disabled',
    'readOnly',
    'required',
    'autoSubmit',
    'aria-label',
  ],
  SearchField: [
    'value',
    'onValueChange',
    'placeholder',
    'aria-label',
    'size',
    'disabled',
  ],
  Select: [
    'options',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'size',
    'disabled',
    'aria-label',
  ],
  RadioGroup: [
    'options',
    'value',
    'defaultValue',
    'onValueChange',
    'size',
    'tint',
    'disabled',
    'aria-label',
  ],
  Slider: [
    'value',
    'defaultValue',
    'onValueChange',
    'min',
    'max',
    'step',
    'size',
    'tint',
    'disabled',
    'showValue',
    'aria-label',
  ],
  Progress: ['value', 'size', 'tint', 'gradient', 'aria-label'],
  Meter: [
    'value',
    'min',
    'max',
    'label',
    'showValue',
    'size',
    'tint',
    'aria-label',
  ],
  Spinner: ['size', 'tint', 'onAccent'],
  SegmentedControl: [
    'items',
    'value',
    'defaultValue',
    'onValueChange',
    'size',
    'tint',
    'activeStyle',
    'iconOnly',
    'aria-label',
  ],
  NumberField: [
    'value',
    'defaultValue',
    'onValueChange',
    'min',
    'max',
    'step',
    'unit',
    'size',
    'disabled',
    'aria-label',
  ],
  OptionCard: [
    'title',
    'description',
    'selected',
    'onClick',
    'disabled',
    'tint',
  ],
  Form: [C, 'onSubmit'],
  FormField: [C, 'label', 'hint', 'state'],
  Fieldset: [C, 'legend', 'disabled'],
  Callout: [C, 'title', 'tint', 'action'],
  EmptyState: [C, 'title', 'action'],
  Preferences: [C],
  PreferenceGroup: [C, 'title', 'description'],
  PreferenceSection: [C, 'label', 'footer'],
  PreferenceRow: [C, 'title', 'subtitle', 'onOpen'],
  Divider: ['tone', 'orientation'],
  Code: [C],
  CodeBlock: ['code', 'title', 'language', 'lineNumbers', 'copyable'],
  SectionLabel: [C, 'faint', 'tint'],
  Collapsible: [C, 'title', 'defaultOpen', 'meta'],
  Skeleton: ['width', 'height', 'shape'],
  SkeletonText: ['lines'],
  Table: [C],
  TableHead: [C],
  TableRow: [C],
  TableCell: [C, 'head', 'align'],
  StepList: [C],
  Step: [C, 'status', 'label', 'meta'],
  Tag: [C, 'selected', 'onSelectedChange', 'onRemove', 'tint'],
  Splitter: [C, 'orientation'],
  SplitterPanel: [C, 'defaultSize', 'minSize', 'maxSize'],
  Dialog: [C, 'trigger', 'width', 'tone'],
  DialogHeader: ['title', 'subtitle'],
  DialogBody: [C],
  DialogFooter: [C],
  DialogClose: [C, 'render'],
  Drawer: [C, 'trigger', 'title', 'description', 'footer', 'side', 'size'],
  DrawerClose: [C, 'render'],
  Popover: [C, 'trigger', 'title', 'description', 'width', 'side'],
  PreviewCard: [
    C,
    'trigger',
    'width',
    'side',
    'align',
    'delay',
    'closeDelay',
    'open',
    'defaultOpen',
    'onOpenChange',
  ],
  Menu: ['trigger', 'items'],
  Menubar: ['menus'],
  PopoverClose: ['render'],
  ContextMenu: [C, 'items'],
  Tooltip: [C, 'label', 'side', 'delay', 'closeDelay', 'disabled'],
  ActionBar: [C, 'align'],
}

export function sceneGaps(id, scene) {
  const problems = gaps(
    [{ id, preview: scene, examples: [] }],
    SUPPORT,
    'HarmonyOS'
  )
  const validToast = options => {
    if (!options || typeof options !== 'object' || Array.isArray(options))
      return false
    const scalar = item =>
      ['string', 'number'].includes(typeof item) ||
      typeof item?.$state === 'string'
    const string = item =>
      typeof item === 'string' || typeof item?.$state === 'string'
    if (!scalar(options.title)) return false
    if (
      Object.keys(options).some(
        key =>
          ![
            'id',
            'title',
            'tint',
            'icon',
            'action',
            'dismissable',
            'dismissLabel',
            'timeout',
            'priority',
            'onClose',
          ].includes(key)
      )
    )
      return false
    if (options.id !== undefined && !string(options.id)) return false
    if (
      options.tint !== undefined &&
      !['primary', 'neutral', 'info', 'success', 'warning', 'danger'].includes(
        options.tint
      )
    )
      return false
    if (options.icon !== undefined && typeof options.icon?.$icon !== 'string')
      return false
    if (
      options.dismissable !== undefined &&
      typeof options.dismissable !== 'boolean'
    )
      return false
    if (options.dismissLabel !== undefined && !string(options.dismissLabel))
      return false
    if (
      options.timeout !== undefined &&
      (typeof options.timeout !== 'number' ||
        !Number.isFinite(options.timeout) ||
        options.timeout < 0)
    )
      return false
    if (
      options.priority !== undefined &&
      !['low', 'high'].includes(options.priority)
    )
      return false
    if (options.onClose !== undefined && !Array.isArray(options.onClose?.$do))
      return false
    if (
      options.action !== undefined &&
      (!scalar(options.action?.label) ||
        !Array.isArray(options.action?.onClick?.$do) ||
        Object.keys(options.action).some(
          key => !['label', 'onClick'].includes(key)
        ))
    )
      return false
    return true
  }
  const visit = (value, toastScope = false) => {
    if (!value || typeof value !== 'object') return
    const props = value.props ?? {}
    const inToast = toastScope || value.$el === 'ToastProvider'
    if (['WindowFrame', 'WindowTitlebar'].includes(value.$el)) {
      if (
        props.platform !== undefined &&
        ![
          'macos',
          'macos15',
          'windows',
          'gnome',
          'ubuntu',
          'kde',
          'omarchy',
        ].includes(props.platform)
      )
        problems.push(`${id}: invalid or dynamic window platform`)
      if (
        props.controls !== undefined &&
        props.controls !== false &&
        (!props.controls ||
          typeof props.controls !== 'object' ||
          Array.isArray(props.controls) ||
          Object.entries(props.controls).some(
            ([key, state]) =>
              !['close', 'minimize', 'maximize'].includes(key) ||
              ![true, false, 'enabled', 'disabled', 'hidden'].includes(state)
          ))
      )
        problems.push(`${id}: unsupported window controls`)
      if (
        props.titlebar !== undefined &&
        props.titlebar !== false &&
        (!props.titlebar ||
          typeof props.titlebar !== 'object' ||
          Array.isArray(props.titlebar) ||
          Object.keys(props.titlebar).some(
            key =>
              !['leading', 'toolbar', 'trailing', 'accessory'].includes(key)
          ))
      )
        problems.push(`${id}: unsupported window titlebar`)
      for (const key of [
        'width',
        'height',
        'minWidth',
        'maxWidth',
        'minHeight',
        'maxHeight',
      ])
        if (
          props[key] !== undefined &&
          !(
            (typeof props[key] === 'number' &&
              Number.isFinite(props[key]) &&
              props[key] >= 0) ||
            (typeof props[key] === 'string' &&
              /^\d+(?:\.\d+)?(?:px|rem|%)$/.test(props[key]))
          )
        )
          problems.push(`${id}: unsupported window ${key}`)
    }
    const validRangeFormula = range =>
      range &&
      Object.keys(range).every(key => ['start', 'end'].includes(key)) &&
      dateFormulaType(range.start) === 'date' &&
      dateFormulaType(range.end) === 'date'
    if (
      Object.hasOwn(value, '$dateRange') &&
      !validRangeFormula(value.$dateRange)
    )
      problems.push(`${id}: invalid date range formula`)
    if (value.$el === 'DateRangePicker') {
      const date = input =>
        Array.isArray(input?.$date) &&
        input.$date.length >= 3 &&
        input.$date.length <= 7 &&
        input.$date.every(Number.isInteger)
      for (const key of ['value', 'defaultValue'])
        if (props[key] !== undefined) {
          const input = props[key]?.$state
            ? scene.state?.[props[key].$state]
            : props[key]
          if (
            input !== null &&
            (!input ||
              Object.keys(input).some(
                key => !['start', 'end', 'allDay'].includes(key)
              ) ||
              !date(input.start) ||
              !date(input.end) ||
              typeof input.allDay !== 'boolean')
          )
            problems.push(`${id}: invalid picker ${key}`)
        }
      if (props.today !== undefined && !date(props.today))
        problems.push(`${id}: invalid picker today`)
      for (const [key, choices] of Object.entries({
        months: [1, 2],
        size: ['small', 'medium', 'large'],
        state: ['default', 'error'],
      }))
        if (props[key] !== undefined && !choices.includes(props[key]))
          problems.push(`${id}: invalid or dynamic picker ${key}`)
      if (
        props.weekStartsOn !== undefined &&
        (!Number.isInteger(props.weekStartsOn) ||
          props.weekStartsOn < 0 ||
          props.weekStartsOn > 6)
      )
        problems.push(`${id}: invalid picker week start`)
      if (
        props.timeStep !== undefined &&
        (!Number.isFinite(props.timeStep) || props.timeStep <= 0)
      )
        problems.push(`${id}: invalid picker time step`)
      if (props.locale !== undefined && typeof props.locale !== 'string')
        problems.push(`${id}: dynamic picker locale`)
      if (
        props.labels !== undefined &&
        (!props.labels ||
          Object.entries(props.labels).some(
            ([key, label]) =>
              ![
                'placeholder',
                'presets',
                'start',
                'end',
                'duration',
                'allDay',
                'cancel',
                'done',
              ].includes(key) || typeof label !== 'string'
          ))
      )
        problems.push(`${id}: invalid picker labels`)
      if (
        props.presets !== undefined &&
        (!Array.isArray(props.presets) ||
          props.presets.some(
            preset =>
              !preset ||
              Object.keys(preset).some(
                key => !['label', 'range'].includes(key)
              ) ||
              typeof preset.label !== 'string' ||
              !validRangeFormula(preset.range?.$dateRange)
          ))
      )
        problems.push(`${id}: unsupported picker presets`)
      if (
        props.durations !== undefined &&
        (!Array.isArray(props.durations) ||
          props.durations.some(
            duration =>
              !duration ||
              Object.keys(duration).some(
                key => !['label', 'minutes'].includes(key)
              ) ||
              typeof duration.label !== 'string' ||
              !Number.isFinite(duration.minutes)
          ))
      )
        problems.push(`${id}: invalid picker durations`)
    }
    const validDate = input =>
      Array.isArray(input) &&
      input.length >= 3 &&
      input.length <= 7 &&
      input.every(Number.isInteger) &&
      Number.isFinite(
        new Date(
          input[0],
          input[1],
          input[2],
          input[3] ?? 0,
          input[4] ?? 0,
          input[5] ?? 0,
          input[6] ?? 0
        ).getTime()
      )
    for (const [key, valid] of Object.entries({
      $date: validDate,
      $get: input =>
        Array.isArray(input) &&
        input.length === 2 &&
        ['start', 'end', 'allDay'].includes(input[1]),
      $dateText: input =>
        Array.isArray(input) &&
        input.length === 2 &&
        ['date', 'datetime'].includes(input[1]),
      $if: input => Array.isArray(input) && input.length === 3,
      $concat: Array.isArray,
    }))
      if (Object.hasOwn(value, key) && !valid(value[key]))
        problems.push(`${id}: invalid expression ${key}`)
    if (value.$el === 'Calendar') {
      const resolve = input =>
        input?.$state ? scene.state?.[input.$state] : input
      const date = input => input == null || validDate(input.$date)
      const range = input =>
        input != null &&
        typeof input === 'object' &&
        Object.hasOwn(input, 'start') &&
        Object.hasOwn(input, 'end') &&
        Object.keys(input).every(key => ['start', 'end'].includes(key)) &&
        date(input.start) &&
        date(input.end)
      for (const [key, choices] of Object.entries({
        mode: ['single', 'range'],
        size: ['small', 'medium'],
        months: [1, 2],
      }))
        if (props[key] !== undefined && !choices.includes(props[key]))
          problems.push(`${id}: invalid or dynamic calendar ${key}`)
      if (
        props.weekStartsOn !== undefined &&
        (!Number.isInteger(props.weekStartsOn) ||
          props.weekStartsOn < 0 ||
          props.weekStartsOn > 6)
      )
        problems.push(`${id}: invalid or dynamic calendar week start`)
      for (const key of ['today', 'min', 'max', 'month', 'defaultMonth'])
        if (props[key] !== undefined && !date(resolve(props[key])))
          problems.push(`${id}: invalid calendar date ${key}`)
      for (const key of ['value', 'defaultValue'])
        if (
          props[key] !== undefined &&
          !(props.mode === 'range'
            ? range(resolve(props[key]))
            : date(resolve(props[key])))
        )
          problems.push(`${id}: invalid calendar ${key}`)
      if (props.locale !== undefined && typeof props.locale !== 'string')
        problems.push(`${id}: dynamic calendar locale`)
      if (
        props.navigation !== undefined &&
        typeof props.navigation !== 'boolean'
      )
        problems.push(`${id}: dynamic calendar navigation`)
    }
    if (value.$el === 'Tabs') {
      const items = props.items?.$state
        ? scene.state?.[props.items.$state]
        : props.items
      const valid =
        Array.isArray(items) &&
        items.every(
          item =>
            item &&
            typeof item.value === 'string' &&
            typeof item.label === 'string' &&
            Object.keys(item).every(key =>
              [
                'value',
                'label',
                'icon',
                'count',
                'status',
                'busy',
                'detail',
                'closable',
                'title',
                'disabled',
              ].includes(key)
            ) &&
            (item.icon === undefined || typeof item.icon?.$icon === 'string') &&
            (item.count === undefined ||
              (typeof item.count === 'number' &&
                Number.isFinite(item.count))) &&
            ['busy', 'closable', 'disabled'].every(
              key => item[key] === undefined || typeof item[key] === 'boolean'
            ) &&
            ['detail', 'title'].every(
              key => item[key] === undefined || typeof item[key] === 'string'
            ) &&
            (item.status === undefined ||
              ['neutral', 'primary', 'success', 'warning', 'danger'].includes(
                item.status
              ))
        ) &&
        new Set(items.map(item => item.value)).size === items.length
      if (!valid) problems.push(`${id}: rich or invalid navigation tab items`)
    }
    if (
      value.$el === 'ToastProvider' &&
      props.limit !== undefined &&
      (!Number.isInteger(props.limit) || props.limit < 0)
    )
      problems.push(`${id}: invalid or dynamic toast limit`)
    if (value.$el === 'ToastViewport') {
      if (!inToast)
        problems.push(`${id}: ToastViewport requires a ToastProvider`)
      if (
        props.placement !== undefined &&
        !['top', 'bottom'].includes(props.placement)
      )
        problems.push(`${id}: invalid or dynamic toast placement`)
    }
    if (value.$el === 'PreviewCard') {
      for (const [key, choices] of Object.entries({
        side: ['top', 'bottom', 'left', 'right'],
        align: ['start', 'center', 'end'],
      }))
        if (props[key] !== undefined && !choices.includes(props[key]))
          problems.push(`${id}: dynamic or unsupported preview ${key}`)
      for (const key of ['delay', 'closeDelay'])
        if (
          props[key] !== undefined &&
          (typeof props[key] !== 'number' ||
            !Number.isFinite(props[key]) ||
            props[key] < 0)
        )
          problems.push(`${id}: dynamic or invalid preview ${key}`)
      if (
        props.width !== undefined &&
        !(
          typeof props.width === 'number' &&
          Number.isFinite(props.width) &&
          props.width > 0
        ) &&
        !(
          typeof props.width === 'string' &&
          /^\d+(?:\.\d+)?(?:px|rem)$/.test(props.width) &&
          parseFloat(props.width) > 0
        )
      )
        problems.push(`${id}: unsupported preview width`)
      if (!['Link', 'Button', 'IconButton'].includes(props.trigger?.$el))
        problems.push(`${id}: unsupported preview trigger`)
    }
    if (value.$el === 'Text') {
      for (const child of value.children ?? []) {
        if (child?.$el !== 'PreviewCard') continue
        const trigger = child.props?.trigger
        if (
          trigger?.$el !== 'Link' ||
          trigger.props?.external ||
          (trigger.children ?? []).some(
            run => run && typeof run === 'object' && run.$el
          )
        )
          problems.push(`${id}: inline previews require a plain link trigger`)
      }
    }
    if (value.$el === 'Sidebar') {
      for (const [key, choices] of Object.entries({
        side: ['left', 'right'],
        variant: ['sidebar', 'floating', 'inset'],
        collapsible: ['offcanvas', 'icon', 'none'],
      }))
        if (props[key] !== undefined && !choices.includes(props[key]))
          problems.push(`${id}: dynamic or unsupported sidebar ${key}`)
    }
    if (value.$el === 'SidebarMenuButton') {
      for (const [key, choices] of Object.entries({
        size: ['small', 'medium', 'large'],
        variant: [
          'filled',
          'tinted',
          'outlined',
          'recessed',
          'normal',
          'plain',
        ],
      }))
        if (props[key] !== undefined && !choices.includes(props[key]))
          problems.push(`${id}: dynamic or unsupported sidebar button ${key}`)
      if (
        props.icon !== undefined &&
        !(
          props.icon?.$el === 'Icon' &&
          typeof props.icon.props?.icon?.$icon === 'string' &&
          (props.icon.props.size === undefined ||
            (typeof props.icon.props.size === 'number' &&
              Number.isFinite(props.icon.props.size)))
        )
      )
        problems.push(
          `${id}: rich or dynamic sidebar glyphs are not mapped on HarmonyOS`
        )
      if (props.tooltip !== undefined && typeof props.tooltip !== 'string')
        problems.push(
          `${id}: rich or dynamic sidebar tooltips are not mapped on HarmonyOS`
        )
    }
    if (
      value.$el === 'SidebarMenuItem' &&
      ((value.children ?? []).filter(
        child => child?.$el === 'SidebarMenuButton'
      ).length !== 1 ||
        (value.children ?? []).some(
          child =>
            !['SidebarMenuButton', 'SidebarMenuBadge'].includes(child?.$el)
        ))
    )
      problems.push(`${id}: unsupported sidebar menu composition`)
    if (value.$el === 'Tree') {
      const icon = node =>
        node?.$el === 'Icon' &&
        typeof node.props?.icon?.$icon === 'string' &&
        (node.props.size === undefined ||
          (typeof node.props.size === 'number' &&
            Number.isFinite(node.props.size)))
      const badge = node =>
        node?.$el === 'Badge' &&
        Object.values(node.props ?? {}).every(
          prop => typeof prop === 'string'
        ) &&
        (node.children ?? []).every(child =>
          ['string', 'number'].includes(typeof child)
        )
      const valid = items =>
        Array.isArray(items) &&
        items.every(
          item =>
            item &&
            typeof item.id === 'string' &&
            typeof item.label === 'string' &&
            Object.keys(item).every(key =>
              [
                'id',
                'label',
                'text',
                'icon',
                'meta',
                'disabled',
                'children',
              ].includes(key)
            ) &&
            (item.text === undefined || typeof item.text === 'string') &&
            (item.disabled === undefined ||
              typeof item.disabled === 'boolean') &&
            (item.icon === undefined || icon(item.icon)) &&
            (item.meta === undefined ||
              typeof item.meta === 'string' ||
              typeof item.meta === 'number' ||
              badge(item.meta)) &&
            (item.children === undefined || valid(item.children))
        )
      if (!valid(props.items))
        problems.push(
          `${id}: rich or dynamic tree items are not mapped on HarmonyOS`
        )
    }
    if (value.$el === 'Drawer') {
      if (props.footer?.$state || props.footer?.$eq)
        problems.push(
          `${id}: dynamic drawer footer presence is not mapped on HarmonyOS`
        )
      if (
        props.trigger !== undefined &&
        !['Button', 'IconButton', 'Toggle', 'Link'].includes(props.trigger?.$el)
      )
        problems.push(`${id}: unsupported drawer trigger control`)
      if (
        props.side !== undefined &&
        !['left', 'right', 'top', 'bottom'].includes(props.side)
      )
        problems.push(`${id}: unsupported drawer edge`)
      for (const key of ['title', 'description'])
        if (props[key] !== undefined && typeof props[key] !== 'string')
          problems.push(
            `${id}: rich or dynamic drawer ${key} is not mapped on HarmonyOS`
          )
      if (
        props.size !== undefined &&
        !(
          typeof props.size === 'number' &&
          Number.isFinite(props.size) &&
          props.size >= 0
        ) &&
        !(
          typeof props.size === 'string' &&
          /^\d+(\.\d+)?(rem|px|%)$/.test(props.size)
        )
      )
        problems.push(`${id}: unsupported drawer size`)
    }
    if (
      value.$el === 'DrawerClose' &&
      props.render !== undefined &&
      !['Button', 'IconButton', 'Toggle', 'Link'].includes(props.render?.$el)
    )
      problems.push(`${id}: unsupported drawer close control`)
    if (
      value.$el === 'DrawerClose' &&
      props.render === undefined &&
      value.children?.some(child => child?.$el || child?.$icon)
    )
      problems.push(
        `${id}: rich drawer close labels require a mapped render control`
      )
    if (value.$el === 'Slider') {
      if (
        props.size !== undefined &&
        !['small', 'medium', 'large'].includes(props.size)
      )
        problems.push(`${id}: dynamic or unsupported slider size`)
      for (const key of ['value', 'defaultValue']) {
        const entry = props[key]
        if (
          entry !== undefined &&
          !entry?.$state &&
          !(
            (typeof entry === 'number' && Number.isFinite(entry)) ||
            (Array.isArray(entry) &&
              entry.length >= 2 &&
              entry.every(
                item => typeof item === 'number' && Number.isFinite(item)
              ))
          )
        )
          problems.push(
            `${id}: slider ${key} requires a number or at least two numeric endpoints`
          )
      }
      const min = props.min ?? 0
      const max = props.max ?? 100
      const step = props.step ?? 1
      if (
        ![min, max, step].every(
          item => typeof item === 'number' && Number.isFinite(item)
        ) ||
        min >= max ||
        step <= 0 ||
        step > max - min
      )
        problems.push(`${id}: dynamic or invalid slider bounds or step`)
    }
    if (
      ['Select', 'Combobox', 'RadioGroup', 'SegmentedControl'].includes(
        value.$el
      )
    ) {
      const valid = (options, groups = true) =>
        Array.isArray(options) &&
        options.every(
          o =>
            o &&
            typeof o.label === 'string' &&
            (o.options
              ? groups &&
                ['Select', 'Combobox'].includes(value.$el) &&
                valid(o.options, false)
              : typeof o.value === 'string' &&
                (!o.icon ||
                  (value.$el === 'SegmentedControl' &&
                    typeof o.icon.$icon === 'string')))
        )
      if (!valid(props.options ?? props.items))
        problems.push(
          `${id}: rich or dynamic options are not yet mapped on HarmonyOS`
        )
    }
    if (['Combobox', 'Autocomplete'].includes(value.$el)) {
      for (const key of ['empty', 'placeholder'])
        if (props[key] !== undefined && typeof props[key] !== 'string')
          problems.push(
            `${id}: rich or dynamic ${key} content is not mapped on HarmonyOS`
          )
    }
    if (
      value.$el === 'Autocomplete' &&
      (!Array.isArray(props.items) ||
        props.items.some(item => typeof item !== 'string'))
    )
      problems.push(
        `${id}: rich or dynamic suggestions are not mapped on HarmonyOS`
      )
    if (
      value.$el === 'Select' &&
      props.placeholder &&
      typeof props.placeholder !== 'string'
    )
      problems.push(
        `${id}: rich select placeholders are not yet mapped on HarmonyOS`
      )
    if (
      value.$el === 'TextField' &&
      props.type &&
      !['text', 'password', 'email', 'number', 'tel', 'url'].includes(
        props.type
      )
    )
      problems.push(
        `${id}: input type ${props.type} is not mapped on HarmonyOS`
      )
    if (
      ['Button', 'IconButton', 'Toggle'].includes(value.$el) &&
      value.children?.some(
        child =>
          typeof child === 'object' &&
          child.$el &&
          !['Icon', 'Spinner'].includes(child.$el)
      )
    )
      problems.push(
        `${id}: composite button labels are not yet mapped on HarmonyOS`
      )
    if (
      ['Menu', 'ContextMenu'].includes(value.$el) &&
      (!Array.isArray(props.items) ||
        props.items.some(
          item =>
            Object.keys(item).some(
              key =>
                ![
                  'label',
                  'disabled',
                  'onSelect',
                  'icon',
                  'shortcut',
                  'checked',
                ].includes(key)
            ) || typeof item.label !== 'string'
        ))
    )
      problems.push(`${id}: rich menu items are not yet mapped on HarmonyOS`)
    for (const action of value.$do ?? []) {
      if (
        typeof action.remove === 'string' &&
        typeof action.key === 'string' &&
        Object.hasOwn(action, 'match') &&
        Object.keys(action).every(key =>
          ['remove', 'key', 'match'].includes(key)
        )
      )
        continue
      if (
        Object.hasOwn(action, 'set') &&
        Object.keys(action).every(key => ['set', 'to'].includes(key))
      )
        continue
      if (Object.keys(action).length !== 1 || !Object.hasOwn(action, 'toast'))
        problems.push(`${id}: unsupported scene action`)
      else if (!inToast)
        problems.push(`${id}: toast action requires a ToastProvider`)
      else if (!validToast(action.toast))
        problems.push(
          `${id}: rich or invalid toast options are not mapped on HarmonyOS`
        )
    }
    if (
      [
        'Badge',
        'Checkbox',
        'KeyCap',
        'Code',
        'Text',
        'SectionLabel',
        'DialogClose',
        'SidebarGroupLabel',
        'SidebarMenuBadge',
        'SidebarMenuButton',
      ].includes(value.$el) &&
      value.children?.some(
        child =>
          typeof child === 'object' &&
          child.$el &&
          !(
            value.$el === 'Text' &&
            (['Code', 'Link', 'PreviewCard'].includes(child.$el) ||
              (child.$el === 'Icon' &&
                value.children.every(
                  run =>
                    run?.$el === 'Icon' ||
                    (typeof run === 'string' && !run.trim())
                )))
          )
      )
    )
      problems.push(`${id}: rich labels are not yet mapped on HarmonyOS`)
    for (const [key, child] of Object.entries(value)) {
      if (
        key.startsWith('$') &&
        ![
          '$el',
          '$at',
          '$icon',
          '$do',
          '$state',
          '$eq',
          '$arg',
          '$date',
          '$get',
          '$dateText',
          '$if',
          '$not',
          '$concat',
          '$dateRange',
        ].includes(key)
      )
        problems.push(`${id}: expression ${key} is not mapped on HarmonyOS`)
      if (Array.isArray(child)) child.forEach(item => visit(item, inToast))
      else visit(child, inToast)
    }
  }
  visit(scene)
  return [...new Set(problems)]
}

/** A typed wire format: the ArkTS interpreter needs neither any nor reflection. */
export function value(input) {
  if (input == null) return { kind: 'null' }
  if (typeof input === 'string') return { kind: 'string', text: input }
  if (typeof input === 'number') return { kind: 'number', number: input }
  if (typeof input === 'boolean') return { kind: 'boolean', boolean: input }
  if (Array.isArray(input)) return { kind: 'array', items: input.map(value) }
  if (input.$el)
    return {
      kind: 'element',
      name: input.$el,
      address: input.$at ?? -1,
      fields: Object.fromEntries(
        Object.entries(input.props ?? {}).map(([key, v]) => [key, value(v)])
      ),
      items: (input.children ?? []).map(value),
    }
  if (input.$icon) return { kind: 'icon', text: input.$icon }
  if (input.$state) return { kind: 'state', text: input.$state }
  if (input.$arg !== undefined) return { kind: 'argument', number: input.$arg }
  if (input.$eq) return { kind: 'eq', items: input.$eq.map(value) }
  if (input.$date) return { kind: 'date', items: input.$date.map(value) }
  if (input.$dateRange)
    return {
      kind: 'dateRange',
      fields: {
        start: value(input.$dateRange.start),
        end: value(input.$dateRange.end),
      },
    }
  if (input.$get) return { kind: 'get', items: input.$get.map(value) }
  if (input.$dateText)
    return { kind: 'dateText', items: input.$dateText.map(value) }
  if (input.$if) return { kind: 'if', items: input.$if.map(value) }
  if (input.$not) return { kind: 'not', items: [value(input.$not)] }
  if (input.$concat) return { kind: 'concat', items: input.$concat.map(value) }
  if (input.$do) return { kind: 'actions', items: input.$do.map(value) }
  return {
    kind: 'object',
    fields: Object.fromEntries(
      Object.entries(input).map(([key, v]) => [key, value(v)])
    ),
  }
}

export function storiesData(stories) {
  const scene = (id, title, scene, note = '') => ({
    id,
    title,
    note,
    content: value(scene.content),
    initial: Object.fromEntries(
      Object.entries(scene.state ?? {}).map(([key, v]) => [key, value(v)])
    ),
    gaps: sceneGaps(id, scene),
  })
  return stories.map(story => ({
    id: story.id,
    name: story.name,
    summary: story.summary,
    preview: scene(`${story.id}/preview`, 'Preview', story.preview),
    examples: story.examples.map((example, i) =>
      scene(`${story.id}/${i}`, example.title, example.scene, example.note)
    ),
  }))
}

const header =
  '// Generated by tools/codegen from the React example. Do not edit.\n'
export function storiesETS(stories) {
  return `${header}import { Story } from '../lib/SceneTypes'\n\nexport const STORIES: Story[] = JSON.parse(${JSON.stringify(JSON.stringify(storiesData(stories)))}) as Story[]\n`
}
export function themesETS(groups) {
  return `${header}import { ThemeGroup } from '../lib/SceneTypes'\n\nexport const THEME_GROUPS: ThemeGroup[] = ${JSON.stringify(groups, null, 2)}\n`
}

export function coverageJSON(stories) {
  const scenes = storiesData(stories).flatMap(story => [
    story.preview,
    ...story.examples,
  ])
  return (
    JSON.stringify(
      {
        platform: 'harmony',
        total: scenes.length,
        mapped: scenes.filter(scene => scene.gaps.length === 0).length,
        gaps: Object.fromEntries(
          scenes
            .filter(scene => scene.gaps.length)
            .map(scene => [scene.id, scene.gaps])
        ),
      },
      null,
      2
    ) + '\n'
  )
}
