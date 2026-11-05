import { gaps, CHILDREN as C } from './support.mjs'

// The GTK interpreter must implement every listed prop. Unsupported scenes
// remain visible as explicit gaps rather than rendering a misleading subset.
export const SUPPORT = {
  Row: [C, 'gap', 'align', 'justify', 'width'],
  Col: [C, 'gap', 'align', 'width'],
  Frame: [C, 'width', 'height'],
  Splitter: [C, 'orientation', 'onSizesChange'],
  SplitterPanel: [C, 'defaultSize', 'minSize', 'maxSize'],
  Text: [C, 'strong', 'tone', 'tint'],
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
  Icon: ['icon', 'size'],
  IconButton: [C, 'label', 'size', 'variant', 'tint', 'disabled'],
  OtpField: [
    'length',
    'groupAfter',
    'size',
    'state',
    'tint',
    'value',
    'defaultValue',
    'onValueChange',
    'onValueInvalid',
    'onValueComplete',
    'validationType',
    'mask',
    'readOnly',
    'disabled',
    'required',
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
  KeyCap: [C, 'variant', 'size'],
  Avatar: ['name', 'size', 'shape', 'tint', 'fallback'],
  AvatarGroup: [C, 'max', 'size'],
  Tooltip: [C, 'label', 'side', 'align', 'delay', 'closeDelay', 'disabled'],
  PreviewCard: [C, 'trigger', 'side', 'align', 'width', 'delay', 'closeDelay'],
  Popover: [C, 'trigger', 'title', 'description', 'side', 'align', 'width'],
  PopoverClose: ['render'],
  Menu: ['trigger', 'items', 'align', 'side', 'tint'],
  ContextMenu: [C, 'items', 'tint'],
  Menubar: ['menus', 'tint', 'disabled'],
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
  Table: [C, 'tint', 'aria-label'],
  TableHead: [C],
  TableRow: [C, 'active', 'interactive', 'onClick'],
  TableCell: [C, 'head', 'align'],
  Dialog: [C, 'trigger', 'width', 'tone'],
  DialogHeader: ['title', 'subtitle'],
  DialogBody: [C],
  DialogFooter: [C],
  DialogClose: ['render'],
  Drawer: [C, 'trigger', 'side', 'size', 'title', 'description', 'footer'],
  DrawerClose: ['render'],
  ToastProvider: [C, 'limit'],
  ToastViewport: ['placement'],
  Accordion: [
    C,
    'defaultValue',
    'value',
    'onValueChange',
    'multiple',
    'divided',
    'disabled',
  ],
  AccordionItem: [C, 'value', 'title', 'icon', 'meta', 'disabled'],
  ToggleGroup: [
    'items',
    'value',
    'onValueChange',
    'multiple',
    'size',
    'variant',
    'tint',
    'aria-label',
  ],
  StepList: [C],
  Step: ['status', 'label', 'meta', 'tint'],
  Combobox: [
    'options',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'empty',
    'clearable',
    'size',
    'state',
    'tint',
    'disabled',
    'required',
    'readOnly',
  ],
  Autocomplete: [
    'items',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'empty',
    'size',
    'state',
    'tint',
    'mono',
    'disabled',
    'required',
    'readOnly',
  ],
  SearchField: [
    'value',
    'onValueChange',
    'placeholder',
    'aria-label',
    'size',
    'disabled',
  ],
  SegmentedControl: [
    'items',
    'value',
    'onValueChange',
    'size',
    'tint',
    'activeStyle',
    'stretch',
    'iconOnly',
    'aria-label',
  ],
  Tag: [
    C,
    'selected',
    'onSelectedChange',
    'onRemove',
    'removeLabel',
    'variant',
    'tint',
    'size',
    'disabled',
  ],
  PillTabs: [
    'items',
    'value',
    'defaultValue',
    'onValueChange',
    'tint',
    'aria-label',
  ],
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
  Badge: [C, 'variant', 'tint', 'size'],
  Card: [C, 'variant', 'size'],
  OptionCard: [
    'title',
    'description',
    'selected',
    'onClick',
    'tint',
    'disabled',
  ],
  Callout: [C, 'title', 'tint', 'size', 'icon', 'action'],
  ActionBar: [C, 'align'],
  Fieldset: [C, 'legend', 'disabled'],
  Checkbox: [
    C,
    'defaultChecked',
    'checked',
    'onCheckedChange',
    'indeterminate',
    'disabled',
    'tint',
    'note',
    'size',
  ],
  Switch: [
    'defaultChecked',
    'checked',
    'onCheckedChange',
    'disabled',
    'size',
    'tint',
    'aria-label',
  ],
  TextField: [
    'defaultValue',
    'placeholder',
    'aria-label',
    'type',
    'required',
    'multiline',
    'rows',
    'state',
    'readOnly',
    'mono',
    'size',
    'disabled',
  ],
  Select: [
    'options',
    'defaultValue',
    'value',
    'onValueChange',
    'placeholder',
    'state',
    'mono',
    'readOnly',
    'required',
    'tint',
    'size',
    'aria-label',
    'disabled',
  ],
  Slider: [
    'value',
    'defaultValue',
    'aria-label',
    'size',
    'tint',
    'showValue',
    'disabled',
    'min',
    'max',
    'step',
    'largeStep',
    'minStepsBetweenValues',
    'thumbCollisionBehavior',
    'thumbAlignment',
    'onValueChange',
    'onValueCommitted',
  ],
  Progress: ['value', 'aria-label', 'size', 'tint', 'gradient'],
  Spinner: ['size', 'tint', 'onAccent', 'label'],
  Skeleton: ['shape', 'width', 'height'],
  SkeletonText: ['lines'],
  NumberField: [
    'defaultValue',
    'value',
    'onValueChange',
    'min',
    'max',
    'step',
    'unit',
    'size',
    'tint',
    'state',
    'aria-label',
    'disabled',
  ],
  Divider: ['orientation', 'tone'],
  Code: [C],
  CodeBlock: [
    'code',
    'title',
    'language',
    'lineNumbers',
    'copyable',
    'wrap',
    'actions',
  ],
  SectionLabel: [C, 'faint', 'tint'],
  Link: [C, 'href', 'external', 'tint', 'underline'],
  Collapsible: [
    C,
    'title',
    'icon',
    'meta',
    'defaultOpen',
    'open',
    'onOpenChange',
  ],
  EmptyState: [C, 'title', 'action'],
  Form: [C, 'onSubmit', 'onFormSubmit', 'errors', 'validationMode'],
  FormField: [
    C,
    'label',
    'hint',
    'state',
    'tint',
    'name',
    'disabled',
    'validationMode',
  ],
  SidebarProvider: [C, 'defaultOpen', 'open', 'onOpenChange'],
  Sidebar: [C, 'side', 'variant', 'collapsible', 'collapsed', 'width'],
  SidebarInset: [C],
  SidebarHeader: [C],
  SidebarFooter: [C],
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
    'tooltip',
    'onClick',
    'variant',
    'tint',
    'size',
    'disabled',
    'aria-label',
  ],
  SidebarMenuBadge: [C, 'tint'],
  SidebarTrigger: [C, 'label', 'disabled'],
  Calendar: [
    'mode',
    'value',
    'defaultValue',
    'onValueChange',
    'size',
    'months',
    'month',
    'defaultMonth',
    'onMonthChange',
    'min',
    'max',
    'weekStartsOn',
    'locale',
    'today',
    'navigation',
    'tint',
    'aria-label',
  ],
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
  Preferences: [C],
  PreferenceGroup: [C, 'title', 'description'],
  PreferenceSection: [C, 'label', 'footer'],
  PreferenceRow: [C, 'title', 'subtitle', 'onOpen', 'tint'],
  RadioGroup: ['options', 'defaultValue', 'aria-label', 'size'],
  Toggle: [
    C,
    'label',
    'size',
    'defaultPressed',
    'pressed',
    'onPressedChange',
    'variant',
    'tint',
    'disabled',
  ],
}

export function sceneGaps(id, scene) {
  const problems = gaps([{ id, preview: scene, examples: [] }], SUPPORT, 'GTK4')
  const visit = (value, toastScope = false) => {
    if (!value || typeof value !== 'object') return
    const inToast = toastScope || value.$el === 'ToastProvider'
    if (value.$el === 'ToastProvider') {
      const p = value.props ?? {}
      if (p.limit !== undefined && (!Number.isInteger(p.limit) || p.limit < 0))
        problems.push(`${id}: invalid or dynamic toast limit`)
      if (
        !Array.isArray(value.children) ||
        value.children.filter(n => n.$el === 'ToastViewport').length !== 1
      )
        problems.push(`${id}: a toast provider needs one direct viewport`)
      if (
        Array.isArray(value.children) &&
        value.children.filter(n => n.$el !== 'ToastViewport').length !== 1
      )
        problems.push(
          `${id}: toast provider siblings need an authored Row or Col`
        )
    }
    if (value.$el === 'ToastViewport') {
      if (!inToast)
        problems.push(`${id}: ToastViewport requires a ToastProvider`)
      if (
        value.props?.placement !== undefined &&
        !['top', 'bottom'].includes(value.props.placement)
      )
        problems.push(`${id}: invalid or dynamic toast placement`)
    }
    for (const step of Array.isArray(value.$do) ? value.$do : []) {
      if (step.toast !== undefined) {
        const p = step.toast
        if (!inToast)
          problems.push(`${id}: toast action requires a ToastProvider`)
        const text = v =>
          typeof v === 'string' ||
          typeof v === 'number' ||
          typeof v?.$state === 'string' ||
          Array.isArray(v?.$concat)
        if (
          !p ||
          typeof p !== 'object' ||
          !text(p.title) ||
          Object.keys(p).some(
            k =>
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
              ].includes(k)
          ) ||
          (p.id !== undefined && !text(p.id)) ||
          (p.tint !== undefined &&
            ![
              'primary',
              'neutral',
              'info',
              'success',
              'warning',
              'danger',
            ].includes(p.tint)) ||
          (p.icon !== undefined && typeof p.icon?.$icon !== 'string') ||
          (p.timeout !== undefined &&
            !(
              typeof p.timeout === 'number' &&
              Number.isFinite(p.timeout) &&
              p.timeout >= 0
            )) ||
          (p.dismissable !== undefined && typeof p.dismissable !== 'boolean') ||
          (p.dismissLabel !== undefined && !text(p.dismissLabel)) ||
          (p.priority !== undefined && !['low', 'high'].includes(p.priority)) ||
          (p.onClose !== undefined && !Array.isArray(p.onClose?.$do)) ||
          (p.action !== undefined &&
            (!text(p.action?.label) ||
              !Array.isArray(p.action?.onClick?.$do) ||
              Object.keys(p.action).some(
                k => !['label', 'onClick'].includes(k)
              )))
        )
          problems.push(
            `${id}: rich or invalid toast options are not yet mapped`
          )
      }
    }
    if (
      ['Table', 'TableHead', 'TableRow', 'Splitter'].includes(value.$el) &&
      value.children !== undefined &&
      !Array.isArray(value.children)
    )
      problems.push(`${id}: dynamic ${value.$el} children are not yet mapped`)
    if (
      value.$el === 'Table' &&
      Array.isArray(value.children) &&
      value.children.some(
        node => !['TableHead', 'TableRow'].includes(node?.$el)
      )
    )
      problems.push(`${id}: table children must be headers or rows`)
    if (
      ['TableHead', 'TableRow'].includes(value.$el) &&
      Array.isArray(value.children) &&
      value.children.some(node => node?.$el !== 'TableCell')
    )
      problems.push(`${id}: table rows need TableCell children`)
    if (
      value.$el === 'TableCell' &&
      ['head', 'align'].some(key => typeof value.props?.[key] === 'object')
    )
      problems.push(
        `${id}: dynamic table cell drawing props are not yet mapped`
      )
    if (['Combobox', 'Autocomplete'].includes(value.$el)) {
      const props = value.props ?? {}
      if (props.empty !== undefined && typeof props.empty !== 'string')
        problems.push(`${id}: rich combobox empty content is not yet mapped`)
      if (value.$el === 'Combobox' && Array.isArray(props.options)) {
        for (const option of props.options.flatMap(
          item => item.options ?? [item]
        )) {
          if (
            typeof option.value !== 'string' ||
            typeof option.label !== 'string'
          )
            problems.push(
              `${id}: combobox options need string values and labels`
            )
        }
      }
      if (
        value.$el === 'Autocomplete' &&
        Array.isArray(props.items) &&
        props.items.some(item => typeof item !== 'string')
      )
        problems.push(`${id}: autocomplete suggestions must be strings`)
    }
    if (
      value.$el === 'Button' &&
      value.props?.type !== undefined &&
      !['button', 'submit', 'reset'].includes(value.props.type)
    )
      problems.push(`${id}: dynamic button form actions are not yet mapped`)
    if (['Form', 'FormField'].includes(value.$el)) {
      if (
        value.props?.validationMode !== undefined &&
        !['onSubmit', 'onBlur', 'onChange'].includes(value.props.validationMode)
      )
        problems.push(`${id}: dynamic form validation modes are not yet mapped`)
      if (
        value.$el === 'FormField' &&
        ['label', 'hint'].some(
          key =>
            value.props?.[key] !== undefined &&
            typeof value.props[key] !== 'string'
        )
      )
        problems.push(`${id}: rich form labels and hints are not yet mapped`)
      if (value.$el === 'Form') {
        const visitForm = node => {
          if (!node || typeof node !== 'object') return
          if (node !== value && node.$el === 'Form')
            problems.push(`${id}: nested forms are not supported`)
          if (
            node !== value &&
            [
              'Select',
              'Combobox',
              'Autocomplete',
              'OtpField',
              'NumberField',
              'DateRangePicker',
              'Checkbox',
              'Switch',
              'RadioGroup',
              'Slider',
              'Dialog',
              'Popover',
              'PreviewCard',
            ].includes(node.$el)
          )
            problems.push(
              `${id}: form validation for ${node.$el} is not yet mapped`
            )
          if (node.children !== undefined && !Array.isArray(node.children))
            problems.push(`${id}: dynamic form children are not yet mapped`)
          for (const child of Array.isArray(node.children) ? node.children : [])
            visitForm(child)
        }
        visitForm(value)
      }
    }
    if (value.$el === 'Drawer') {
      const p = value.props ?? {}
      if (
        p.side !== undefined &&
        !['left', 'right', 'top', 'bottom'].includes(p.side)
      )
        problems.push(`${id}: dynamic drawer side is not yet mapped`)
      if (
        p.size !== undefined &&
        !(
          typeof p.size === 'number' &&
          Number.isFinite(p.size) &&
          p.size > 0
        ) &&
        !(
          typeof p.size === 'string' &&
          /^\d+(?:\.\d+)?(?:px|rem)$/.test(p.size) &&
          parseFloat(p.size) > 0
        )
      )
        problems.push(`${id}: drawer size needs a positive fixed length`)
      for (const key of ['title', 'description'])
        if (p[key] !== undefined && typeof p[key] !== 'string')
          problems.push(
            `${id}: rich or dynamic drawer ${key} is not yet mapped`
          )
      if (!p.trigger?.$el)
        problems.push(`${id}: a drawer needs an authored trigger`)
      if (value.children !== undefined && !Array.isArray(value.children))
        problems.push(`${id}: dynamic drawer children are not yet mapped`)
    }
    if (value.$el?.startsWith('Sidebar')) {
      const p = value.props ?? {}
      if (value.children !== undefined && !Array.isArray(value.children))
        problems.push(`${id}: dynamic sidebar children are not yet mapped`)
      if (value.$el === 'Sidebar') {
        for (const [key, options] of Object.entries({
          side: ['left', 'right'],
          variant: ['sidebar', 'floating', 'inset'],
          collapsible: ['offcanvas', 'icon', 'none'],
        }))
          if (p[key] !== undefined && !options.includes(p[key]))
            problems.push(`${id}: dynamic sidebar ${key} is not yet mapped`)
        if (
          p.width !== undefined &&
          (typeof p.width !== 'string' ||
            !/^(?:\d+(?:\.\d*)?|\.\d+)(?:rem|px)$/.test(p.width))
        )
          problems.push(`${id}: sidebar width needs a fixed length`)
      }
      if (
        ['SidebarMenuButton', 'SidebarMenuBadge', 'SidebarGroupLabel'].includes(
          value.$el
        ) &&
        (Array.isArray(value.children) ? value.children : []).some(
          child => typeof child !== 'string' && typeof child !== 'number'
        )
      )
        problems.push(`${id}: rich sidebar captions are not yet mapped`)
      if (
        value.$el === 'SidebarMenuButton' &&
        p.tooltip !== undefined &&
        typeof p.tooltip !== 'string'
      )
        problems.push(`${id}: rich sidebar tooltips are not yet mapped`)
      if (
        value.$el === 'SidebarTrigger' &&
        p.label !== undefined &&
        typeof p.label !== 'string'
      )
        problems.push(
          `${id}: dynamic sidebar trigger labels are not yet mapped`
        )
      if (value.$el === 'SidebarTrigger' && (value.children ?? []).length)
        problems.push(`${id}: custom sidebar trigger glyphs are not yet mapped`)
    }
    if (value.$el === 'DateRangePicker') {
      const p = value.props ?? {}
      const dynamic = v =>
        Array.isArray(v)
          ? v.some(dynamic)
          : v &&
            typeof v === 'object' &&
            Object.entries(v).some(
              ([k, value]) => k.startsWith('$') || dynamic(value)
            )
      for (const key of [
        'months',
        'weekStartsOn',
        'timeStep',
        'locale',
        'labels',
        'presets',
        'durations',
        'today',
      ])
        if (dynamic(p[key]))
          problems.push(`${id}: dynamic date-range ${key} is not yet mapped`)
      if (p.months !== undefined && ![1, 2].includes(p.months))
        problems.push(`${id}: date-range picker shows one or two months`)
      if (
        p.weekStartsOn !== undefined &&
        (!Number.isInteger(p.weekStartsOn) ||
          p.weekStartsOn < 0 ||
          p.weekStartsOn > 6)
      )
        problems.push(`${id}: invalid date-range week start`)
      if (
        p.timeStep !== undefined &&
        (!Number.isInteger(p.timeStep) || p.timeStep <= 0 || p.timeStep > 1440)
      )
        problems.push(`${id}: date-range time step needs whole minutes`)
      if (
        p.presets !== undefined &&
        (!Array.isArray(p.presets) ||
          p.presets.some(
            p =>
              !Array.isArray(p.days) ||
              p.days.length !== 2 ||
              p.days.some(n => !Number.isInteger(n))
          ))
      )
        problems.push(`${id}: date-range presets need day offsets`)
      if (
        p.durations !== undefined &&
        (!Array.isArray(p.durations) ||
          p.durations.some(d => !Number.isInteger(d.minutes)))
      )
        problems.push(`${id}: date-range durations need whole minutes`)
    }
    if (value.$el === 'Calendar') {
      const p = value.props ?? {}
      for (const [key, choices] of Object.entries({
        mode: ['single', 'range'],
        months: [1, 2],
        weekStartsOn: [0, 1, 2, 3, 4, 5, 6],
        navigation: [true, false],
      }))
        if (p[key] !== undefined && !choices.includes(p[key]))
          problems.push(
            `${id}: dynamic or invalid calendar ${key} is not yet mapped`
          )
      if (p.locale !== undefined && typeof p.locale !== 'string')
        problems.push(`${id}: dynamic calendar locales are not yet mapped`)
    }
    if (value.$el === 'Frame') {
      const fixed = v =>
        typeof v === 'string' && /^(?:\d+(?:\.\d*)?|\.\d+)(?:rem|px)$/.test(v)
      const props = value.props ?? {}
      if (
        props.width !== undefined &&
        props.width !== '100%' &&
        !fixed(props.width)
      )
        problems.push(`${id}: frame widths need a fixed length or 100%`)
      if (props.height !== undefined && !fixed(props.height))
        problems.push(`${id}: frame heights need a fixed length`)
    }
    if (value.$el === 'Splitter') {
      if (
        value.props?.orientation &&
        !['horizontal', 'vertical'].includes(value.props.orientation)
      )
        problems.push(`${id}: dynamic splitter orientation is not yet mapped`)
      if (
        Array.isArray(value.children) &&
        value.children.some(node => node?.$el !== 'SplitterPanel')
      )
        problems.push(`${id}: splitter children must be panels`)
    }
    if (value.$el === 'SplitterPanel') {
      for (const key of ['minSize', 'maxSize'])
        if (
          value.props?.[key] !== undefined &&
          typeof value.props[key] !== 'number'
        )
          problems.push(
            `${id}: dynamic splitter constraints are not yet mapped`
          )
    }
    if (value.$el === 'OtpField') {
      const props = value.props ?? {}
      if (!Number.isInteger(props.length) || props.length <= 0)
        problems.push(`${id}: OTP length must be a positive committed integer`)
      if (
        props.groupAfter !== undefined &&
        (!Number.isInteger(props.groupAfter) || props.groupAfter < 0)
      )
        problems.push(
          `${id}: OTP grouping needs a committed nonnegative integer`
        )
      for (const key of [
        'groupAfter',
        'size',
        'state',
        'tint',
        'validationType',
        'required',
      ])
        if (typeof props[key] === 'object')
          problems.push(`${id}: dynamic OTP ${key} is not yet mapped`)
      if (
        props.validationType !== undefined &&
        !['numeric', 'alpha', 'alphanumeric', 'none'].includes(
          props.validationType
        )
      )
        problems.push(`${id}: unsupported OTP validation type`)
    }
    if (value.$el === 'Tree') {
      const ids = new Set()
      const walk = nodes => {
        if (!Array.isArray(nodes)) {
          problems.push(`${id}: dynamic tree arrays are not yet mapped`)
          return
        }
        for (const node of nodes) {
          if (!node || typeof node.id !== 'string' || ids.has(node.id))
            problems.push(`${id}: tree ids must be unique strings`)
          else ids.add(node.id)
          for (const key of Object.keys(node ?? {}))
            if (
              ![
                'id',
                'label',
                'icon',
                'meta',
                'disabled',
                'children',
                'text',
              ].includes(key)
            )
              problems.push(`${id}: tree item ${key} is not yet mapped`)
          if (node?.children !== undefined) walk(node.children)
        }
      }
      walk(value.props?.items)
      for (const key of ['size', 'tint'])
        if (typeof value.props?.[key] === 'object')
          problems.push(`${id}: dynamic tree drawing props are not yet mapped`)
    }
    if (['Menu', 'ContextMenu', 'Menubar'].includes(value.$el)) {
      const props = value.props ?? {}
      const sets =
        value.$el === 'Menubar' ? props.menus : [{ items: props.items }]
      if (!Array.isArray(sets))
        problems.push(`${id}: dynamic menu arrays are not yet mapped`)
      else
        for (const menu of sets) {
          if (value.$el === 'Menubar') {
            if (typeof menu.label !== 'string')
              problems.push(`${id}: dynamic menubar labels are not yet mapped`)
            for (const key of Object.keys(menu))
              if (!['label', 'items', 'disabled'].includes(key))
                problems.push(`${id}: menubar entry ${key} is not yet mapped`)
          }
          if (!Array.isArray(menu.items))
            problems.push(`${id}: dynamic menu items are not yet mapped`)
          else
            for (const item of menu.items)
              for (const key of Object.keys(item))
                if (
                  ![
                    'label',
                    'icon',
                    'shortcut',
                    'checked',
                    'disabled',
                    'onSelect',
                  ].includes(key)
                )
                  problems.push(`${id}: menu item ${key} is not yet mapped`)
        }
    }
    if (value.$el === 'Slider') {
      const props = value.props ?? {}
      for (const key of [
        'min',
        'max',
        'step',
        'largeStep',
        'minStepsBetweenValues',
        'thumbAlignment',
        'thumbCollisionBehavior',
      ])
        if (typeof props[key] === 'object')
          problems.push(`${id}: dynamic slider ${key} is not yet mapped`)
      if (
        props.thumbAlignment !== undefined &&
        props.thumbAlignment !== 'center'
      )
        problems.push(`${id}: inset slider thumbs are not yet mapped`)
      if (
        props.thumbCollisionBehavior !== undefined &&
        !['push', 'none'].includes(props.thumbCollisionBehavior)
      )
        problems.push(`${id}: slider swap collision is not yet mapped`)
      for (const key of ['value', 'defaultValue'])
        if (Array.isArray(props[key]) && !props[key].length)
          problems.push(`${id}: empty slider arrays are not yet mapped`)
    }
    if (value.$el === 'Select') {
      const options = value.props?.options
      if (!Array.isArray(options))
        problems.push(`${id}: dynamic select options are not yet mapped`)
      else if (
        options.some(
          o =>
            typeof o.label !== 'string' ||
            o.options?.some(o => typeof o.label !== 'string')
        )
      )
        problems.push(`${id}: rich select option labels are not yet mapped`)
      if (
        value.props?.placeholder &&
        typeof value.props.placeholder !== 'string'
      )
        problems.push(`${id}: rich select placeholders are not yet mapped`)
    }
    Object.values(value).forEach(v =>
      Array.isArray(v)
        ? v.forEach(item => visit(item, inToast))
        : visit(v, inToast)
    )
  }
  visit(scene.content)
  return [...new Set(problems)]
}

/** A story keeps React's preview/examples split and its shared scene addresses. */
export function storyJSON(story) {
  const scene = (id, title, value, note) => ({
    id,
    title,
    ...(note ? { note } : {}),
    ...value,
    gaps: sceneGaps(id, value),
  })
  return JSON.stringify(
    {
      id: story.id,
      name: story.name,
      summary: story.summary,
      preview: scene(`${story.id}/preview`, 'Preview', story.preview),
      examples: story.examples.map((e, i) =>
        scene(`${story.id}/${i}`, e.title, e.scene, e.note)
      ),
    },
    null,
    2
  )
}

const header =
  '// Generated by tools/codegen from the React example. Do not edit.\n'
const moduleName = id => id.replaceAll('-', '_')
function raw(text) {
  let hashes = '#'
  while (text.includes(`"${hashes}`)) hashes += '#'
  return `r${hashes}"${text}"${hashes}`
}
export const storyFile = id => `${moduleName(id)}.rs`
export function storyRust(story) {
  return `${header}pub const STORY: &str = ${raw(storyJSON(story))};\n`
}
export function storiesIndexRust(stories) {
  return `${header}
${stories.map(s => `mod ${moduleName(s.id)};`).join('\n')}

pub const SOURCES: &[&str] = &[
${stories.map(s => `    ${moduleName(s.id)}::STORY,`).join('\n')}
];

pub fn all() -> &'static [crate::support::story::Story] {
    static STORIES: std::sync::OnceLock<Vec<crate::support::story::Story>> = std::sync::OnceLock::new();
    STORIES.get_or_init(|| SOURCES.iter().map(|source| serde_json::from_str(source).expect("Generated story")).collect())
}

pub fn story_of(id: &str) -> Option<&'static crate::support::story::Story> {
    all().iter().find(|story| story.id == id)
}

pub fn matching(query: &str) -> Vec<&'static crate::support::story::Story> {
    let needle = query.trim().to_lowercase();
    all().iter().filter(|story| story.name.to_lowercase().contains(&needle) || story.summary.to_lowercase().contains(&needle)).collect()
}
`
}
export function themesRust(groups) {
  return `${header}
#[derive(Clone, Copy, Debug)]
pub struct Family { pub value: &'static str, pub title: &'static str }
#[derive(Clone, Copy, Debug)]
pub struct FamilyGroup { pub title: &'static str, pub families: &'static [Family] }

pub const FAMILY_GROUPS: &[FamilyGroup] = &[
${groups.map(group => `    FamilyGroup { title: ${JSON.stringify(group.title)}, families: &[\n${group.families.map(f => `        Family { value: ${JSON.stringify(f.value)}, title: ${JSON.stringify(f.title)} },`).join('\n')}\n    ] },`).join('\n')}
];
`
}
