import { DazzCombobox, DazzAutocomplete } from '@dazzlabs/dazzui'
import { DazzWindowFrame, DazzWindowTitlebar, DazzWindowBody, DazzWindowMain, DazzWindowContent, DazzWindowFooter, DazzWindowPlatform, DazzWindowControls, DazzWindowControl, DazzWindowTitleStyle } from '@dazzlabs/dazzui'
import { DazzCalendar, DazzCalendarValue, DazzCalendarMode, DazzCalendarSize } from '@dazzlabs/dazzui'
import { DazzDateRangePicker, DazzDateRangeValue, DazzDateRangePreset, DazzDateRangeDuration, DazzDateRangePickerLabels, DATE_RANGE_PRESETS, DATE_RANGE_DURATIONS, evaluateDateRangeFormula } from '@dazzlabs/dazzui'
import { sceneDateRange, sceneDateRangeValue, sceneDateRangePreset } from './SceneDateRangeData'
import { DazzSelectOption } from '@dazzlabs/dazzui'
import {
  DazzSidebarProvider, DazzSidebar, DazzSidebarInset, DazzSidebarHeader,
  DazzSidebarContent, DazzSidebarGroup, DazzSidebarGroupLabel,
  DazzSidebarGroupContent, DazzSidebarMenu, DazzSidebarMenuItem,
  DazzSidebarMenuButton, DazzSidebarMenuBadge, DazzSidebarTrigger,
} from '@dazzlabs/dazzui'
import { LengthMetrics } from '@kit.ArkUI'
import { DazzDialogContext } from '@dazzlabs/dazzui'
import {
  DazzTag,
  DazzStepList,
  DazzStep,
  DazzTable,
  DazzTableRow,
  DazzTableCell,
  DazzSkeletonText,
  DazzSplitter,
  DazzSplitterPanel,
} from '@dazzlabs/dazzui'
import {
  DazzTint,
  DazzVariant,
  DazzSize,
  DazzItem,
  DazzTokens,
  DazzButton,
  DazzIconButton,
  DazzToggle,
  DazzTextField,
  DazzOtpField,
  DazzTree,
  DazzTreeNode,
  DazzSearchField,
  DazzCheckbox,
  DazzSwitch,
  DazzSelect,
  DazzRadioGroup,
  DazzSlider,
  DazzSliderValue,
  DazzSliderSize,
  DazzSegmentedControl,
  DazzProgress,
  DazzSpinner,
  DazzMeter,
  DazzFormField,
  DazzFieldset,
  DazzPreferences,
  DazzPreferenceGroup,
  DazzPreferenceSection,
  DazzPreferenceRow,
  DazzDialog,
  DazzDialogClose,
  DazzPopover,
  DazzPreviewCard,
  DazzTooltip,
  DazzMenu,
  DazzMenuItem,
  DazzNumberField,
  DazzAvatar,
  DazzBadge,
  DazzCallout,
  DazzCard,
  DazzCode,
  DazzCollapsible,
  DazzDivider,
  DazzEmptyState,
  DazzIcon,
  DazzKeyCap,
  DazzOptionCard,
  DazzSectionLabel,
  DazzSkeleton,
} from '@dazzlabs/dazzui'
import { SceneInline } from './SceneInline'
import { SceneChrome, SceneHeaderContext } from './SceneChrome'
import { DazzTabs, DazzTabItem } from '@dazzlabs/dazzui'
import { DazzToastProvider, DazzToastViewport, DazzToastContext } from '@dazzlabs/dazzui'
import { sceneToastOptions } from './SceneToastData'
import { DazzShortcutRecorder } from '@dazzlabs/dazzui'
import { DazzDrawer, DazzDrawerClose, DazzDrawerSide } from '@dazzlabs/dazzui'
import { DazzAccordion, DazzAccordionItem, DazzAvatarGroup, DazzAvatarPerson, DazzCodeBlock, DazzLink, DazzPillTabs, DazzToggleGroup, DazzMenubar, DazzMenubarEntry, DazzPopoverClose, DazzForm, DazzActivationBoundary, DazzDialogHeader, DazzDialogBody, DazzDialogFooter } from '@dazzlabs/dazzui'
import { SceneValue, SceneNavigation } from './SceneTypes'
import {
  SceneState,
  EMPTY_VALUE,
  text,
  stringValue,
  booleanValue,
  numberValue,
  dateValue,
  dateFromValue,
} from './SceneState'

@Component
export struct SceneNode {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.dialog') dialog: DazzDialogContext = new DazzDialogContext()
  @Consume('dazzui.toast') toast: DazzToastContext = new DazzToastContext()
  @Consume('dazzui.example.header') header: SceneHeaderContext = new SceneHeaderContext()
  @Consume('dazzui.navigation') navigation: SceneNavigation = new SceneNavigation()
  @Prop node: SceneValue
  @Prop sceneId: string = ''
  @Prop splitterAxis: string = 'horizontal'
  @Prop stageStretch: boolean = false
  @Consume('dazzui.window-title') titleStyle: DazzWindowTitleStyle = new DazzWindowTitleStyle()
  @Prop inWindowTitle: boolean = false
  state: SceneState = new SceneState()
  @Link revision: number

  specimenID(): string {
    return this.sceneId + '/' + String(this.node.address)
  }
  p(key: string): SceneValue {
    // All descendants read this link; changing a slot updates the whole scene.
    if (this.revision < 0) return EMPTY_VALUE
    return this.state.evaluate(this.node.fields?.[key] ?? EMPTY_VALUE)
  }
  has(key: string): boolean {
    return this.node.fields?.[key] !== undefined
  }
  s(key: string, fallback: string = ''): string {
    return this.has(key) ? text(this.p(key)) : fallback
  }
  n(key: string, fallback: number = 0): number {
    return this.p(key).number ?? fallback
  }
  b(key: string, fallback: boolean = false): boolean {
    return this.p(key).boolean ?? fallback
  }
  optionalString(key: string): string | undefined {
    return this.has(key) ? this.s(key) : undefined
  }
  optionalNumber(key: string): number | undefined {
    return this.has(key) ? this.n(key) : undefined
  }
  sliderValue(key: string): DazzSliderValue | undefined {
    if (!this.has(key)) return undefined
    const value = this.p(key)
    return value.kind === 'array' ? (value.items ?? []).map((item: SceneValue): number => this.state.evaluate(item).number ?? 0) : value.number
  }
  date(key: string): Date | undefined { return dateFromValue(this.p(key)) ?? undefined }
  pickerValue(key: string): DazzDateRangeValue | null | undefined { return this.has(key) ? sceneDateRange(this.p(key)) : undefined }
  pickerPresets(): DazzDateRangePreset[] {
    return this.has('presets') ? (this.p('presets').items ?? []).map((item: SceneValue): DazzDateRangePreset => sceneDateRangePreset(item, evaluateDateRangeFormula)) : DATE_RANGE_PRESETS
  }
  pickerDurations(): DazzDateRangeDuration[] {
    return this.has('durations') ? (this.p('durations').items ?? []).map((item: SceneValue): DazzDateRangeDuration => ({ label: text(item.fields?.['label'] ?? EMPTY_VALUE), minutes: item.fields?.['minutes']?.number ?? 0 })) : DATE_RANGE_DURATIONS
  }
  pickerLabels(): DazzDateRangePickerLabels {
    const labels = this.p('labels').fields ?? {}
    const label = (key: string): string | undefined => labels[key] ? text(labels[key]) : undefined
    return { placeholder: label('placeholder'), presets: label('presets'), start: label('start'), end: label('end'), duration: label('duration'), allDay: label('allDay'), cancel: label('cancel'), done: label('done') }
  }
  calendarValue(key: string): DazzCalendarValue | undefined {
    if (!this.has(key)) return undefined
    const value = this.p(key)
    if (this.s('mode', 'single') !== 'range') return dateFromValue(value)
    return { start: dateFromValue(this.state.evaluate(value.fields?.['start'] ?? EMPTY_VALUE)), end: dateFromValue(this.state.evaluate(value.fields?.['end'] ?? EMPTY_VALUE)) }
  }
  calendarChanged(value: DazzCalendarValue): void {
    if (value instanceof Date || value === null) this.run('onValueChange', dateValue(value as Date | null))
    else this.run('onValueChange', { kind: 'object', fields: { 'start': dateValue(value.start), 'end': dateValue(value.end) } })
  }
  optionalBoolean(key: string): boolean | undefined {
    return this.has(key) ? this.b(key) : undefined
  }
  controlSize(fallback: string = 'medium'): DazzSize {
    return this.s('size', fallback) as DazzSize
  }
  tint(): DazzTint {
    return this.s('tint', 'primary') as DazzTint
  }
  variant(fallback: string = 'normal'): DazzVariant {
    return this.s('variant', fallback) as DazzVariant
  }
  label(): string {
    return (this.node.items ?? [])
      .filter(
        (item: SceneValue) => item.kind !== 'element' && item.kind !== 'icon'
      )
      .map((item: SceneValue) => text(this.state.evaluate(item)))
      .join('')
  }
  run(key: string, argument: SceneValue = EMPTY_VALUE): void {
    if (!this.has(key)) return
    this.perform(this.node.fields?.[key] ?? EMPTY_VALUE, argument)
  }
  perform(action: SceneValue, argument: SceneValue = EMPTY_VALUE): void {
    this.state.run(action, argument, (options: SceneValue) => {
      this.toast.add(sceneToastOptions(options, this.state, (callback: SceneValue) => this.perform(callback)))
    })
    this.revision++
  }
  strings(key: string): string[] { return (this.p(key).items ?? []).map((item: SceneValue): string => text(this.state.evaluate(item))) }
  stringsChanged(key: string, values: string[]): void { this.run(key, { kind: 'array', items: values.map((value: string): SceneValue => stringValue(value)) }) }
  avatarPeople(): DazzAvatarPerson[] {
    return (this.node.items ?? []).map((item: SceneValue): DazzAvatarPerson => ({ name: text(item.fields?.['name'] ?? EMPTY_VALUE), fallback: text(item.fields?.['fallback'] ?? EMPTY_VALUE), tint: (text(item.fields?.['tint'] ?? EMPTY_VALUE) || 'primary') as DazzTint }))
  }
  tabItems(): DazzTabItem[] {
    return (this.p('items').items ?? []).map((item: SceneValue): DazzTabItem => {
      const fields = item.fields ?? {}
      const read = (key: string): SceneValue => this.state.evaluate(fields[key] ?? EMPTY_VALUE)
      return { value: text(read('value')), label: text(read('label')),
        icon: fields['icon'] ? text(read('icon')) : undefined,
        count: read('count').number,
        status: fields['status'] ? text(read('status')) as 'neutral' | 'primary' | 'success' | 'warning' | 'danger' : undefined,
        busy: read('busy').boolean, detail: fields['detail'] ? text(read('detail')) : undefined,
        closable: read('closable').boolean, title: fields['title'] ? text(read('title')) : undefined,
        disabled: read('disabled').boolean }
    })
  }
  @LocalBuilder tabsTrailing() {
    SceneNode({ node: this.p('trailing'), state: this.state, sceneId: this.sceneId, revision: $revision })
  }
  items(key: string): DazzItem[] {
    return (this.p(key).items ?? []).map((item: SceneValue): DazzItem => {
      const fields = item.fields ?? {}
      return {
        value: text(fields['value'] ?? EMPTY_VALUE),
        label: text(fields['label'] ?? EMPTY_VALUE),
        icon: fields['icon']?.kind === 'icon' ? fields['icon'].text : fields['icon']?.fields?.['icon']?.text ?? fields['label']?.fields?.['icon']?.text,
        hint: text(fields['hint'] ?? EMPTY_VALUE),
        count: fields['count']?.number,
        disabled: fields['disabled']?.boolean ?? false,
      }
    })
  }
  options(value: SceneValue = this.p('options')): DazzSelectOption[] {
    return (value.items ?? []).map((item: SceneValue): DazzSelectOption => {
      const fields = item.fields ?? {}
      return { value: fields['value'] ? text(this.state.evaluate(fields['value'])) : undefined,
        label: text(this.state.evaluate(fields['label'] ?? EMPTY_VALUE)),
        disabled: fields['disabled'] ? this.state.evaluate(fields['disabled']).boolean ?? false : false,
        options: fields['options'] ? this.options(fields['options']) : undefined }
    })
  }
  treeItems(value: SceneValue = this.p('items')): DazzTreeNode[] {
    return (value.items ?? []).map((item: SceneValue): DazzTreeNode => {
      const fields = item.fields ?? {}
      const meta = this.state.evaluate(fields['meta'] ?? EMPTY_VALUE)
      const icon = fields['icon']
      return {
        id: text(this.state.evaluate(fields['id'] ?? EMPTY_VALUE)),
        label: text(this.state.evaluate(fields['label'] ?? EMPTY_VALUE)),
        icon: icon?.kind === 'icon' ? icon.text : icon?.fields?.['icon']?.text,
        iconSize: icon?.fields?.['size']?.number,
        text: fields['text'] ? text(this.state.evaluate(fields['text'])) : undefined,
        disabled: this.state.evaluate(fields['disabled'] ?? EMPTY_VALUE).boolean,
        children: fields['children'] ? this.treeItems(fields['children']) : undefined,
        meta: meta.kind !== 'element' && !(meta.kind === 'number' && meta.number === 0)
          ? text(meta) : undefined,
        badge: meta.name === 'Badge' ? {
          label: (meta.items ?? []).map((part: SceneValue): string => text(this.state.evaluate(part))).join(''),
          tint: (text(meta.fields?.['tint'] ?? EMPTY_VALUE) || 'primary') as DazzTint,
          variant: text(meta.fields?.['variant'] ?? EMPTY_VALUE) || 'tinted',
          controlSize: (text(meta.fields?.['size'] ?? EMPTY_VALUE) || 'medium') as DazzSize,
        } : undefined,
      }
    })
  }
  iconText(): boolean {
    return (this.node.items ?? []).some((item: SceneValue) => item.name === 'Icon') &&
      (this.node.items ?? []).every((item: SceneValue) => item.name === 'Icon' || (item.kind === 'string' && !text(item).trim()))
  }
  sidebarButton(): SceneValue {
    if (this.revision < 0) return EMPTY_VALUE
    return (this.node.items ?? []).find((item: SceneValue) => item.name === 'SidebarMenuButton') ?? EMPTY_VALUE
  }
  sidebarItemValue(key: string): SceneValue {
    return this.state.evaluate(this.sidebarButton().fields?.[key] ?? EMPTY_VALUE)
  }
  textColor(): string {
    return this.s('tint') ? this.tokens.color(this.s('tint') + '.500') :
      this.s('tone') === 'secondary' ? this.tokens.color('content-secondary') : this.title()?.color ?? this.tokens.color('content')
  }
  title(): DazzWindowTitleStyle | undefined { return this.inWindowTitle && this.titleStyle.active ? this.titleStyle : undefined }
  menuItems(value: SceneValue = this.p('items')): DazzMenuItem[] {
    return (value.items ?? []).map(
      (item: SceneValue): DazzMenuItem => {
        const fields = item.fields ?? {}
        return {
          label: text(fields['label'] ?? EMPTY_VALUE),
          disabled: fields['disabled']?.boolean ?? false,
          icon: fields['icon']?.fields?.['icon']?.text,
          shortcut: fields['shortcut'] ? text(fields['shortcut']) : '',
          checked: fields['checked']?.boolean,
          onSelect: () => {
            if (fields['onSelect']) {
              this.perform(fields['onSelect'])
            }
          },
        }
      }
    )
  }
  length(key: string, fallback: Length = 'auto'): Length {
    const value = this.p(key)
    if (value.kind === 'number') return value.number ?? fallback
    if (value.kind !== 'string') return fallback
    const length = value.text ?? ''
    if (length.endsWith('rem')) return parseFloat(length) * 16
    if (length.endsWith('px')) return parseFloat(length)
    return length
  }
  windowLength(key: string): number | string | undefined {
    const value = this.p(key)
    if (value.kind === 'number') return value.number
    if (value.kind !== 'string') return undefined
    const length = value.text ?? ''
    return length.endsWith('rem') ? parseFloat(length) * 16 : length.endsWith('px') ? parseFloat(length) : length
  }
  windowControls(): DazzWindowControls | false | undefined {
    if (!this.has('controls')) return undefined
    const value = this.p('controls')
    if (value.kind === 'boolean') return false
    const read = (key: string): DazzWindowControl | undefined => {
      const item = value.fields?.[key]
      return item?.kind === 'boolean' ? item.boolean : item?.text as DazzWindowControl | undefined
    }
    return { close: read('close'), minimize: read('minimize'), maximize: read('maximize') }
  }
  windowPart(key: string): SceneValue {
    return this.node.name === 'WindowFrame' ? this.p('titlebar').fields?.[key] ?? EMPTY_VALUE : this.p(key)
  }
  hasWindowPart(key: string): boolean { const value = this.windowPart(key); return value.kind !== 'null' && (value.kind !== 'array' || !!value.items?.length) }
  @LocalBuilder windowSlot(value: SceneValue, inTitle: boolean = false) {
    if (value.kind === 'array') {
      ForEach(value.items ?? [], (item: SceneValue) => {
        SceneNode({ node: item, state: this.state, sceneId: this.sceneId, revision: $revision, inWindowTitle: inTitle })
      }, (item: SceneValue, index: number) => String(index))
    } else {
      SceneNode({ node: value, state: this.state, sceneId: this.sceneId, revision: $revision, inWindowTitle: inTitle })
    }
  }
  @LocalBuilder windowTitle() { this.windowSlot(this.p('title'), true) }
  @LocalBuilder windowLeading() { this.windowSlot(this.windowPart('leading')) }
  @LocalBuilder windowToolbar() { this.windowSlot(this.windowPart('toolbar')) }
  @LocalBuilder windowTrailing() { this.windowSlot(this.windowPart('trailing')) }
  @LocalBuilder windowAccessory() { this.windowSlot(this.windowPart('accessory')) }
  gap(): number {
    return this.tokens.metric('spacing.' + this.s('gap', '3').replace('-', '.'))
  }
  stageAlignment(): ItemAlign {
    const value = this.s(
      'align',
      this.node.name === 'Row' ? 'center' : 'stretch'
    )
    return value === 'start'
      ? ItemAlign.Start
      : value === 'end'
        ? ItemAlign.End
        : value === 'stretch'
          ? ItemAlign.Stretch
          : ItemAlign.Center
  }
  stretchChildren(): boolean {
    return ['WindowFrame', 'WindowBody', 'WindowMain', 'WindowContent', 'WindowFooter', 'WindowPane'].includes(this.node.name ?? '') || this.node.name === 'Drawer' || this.node.name === 'DialogBody' ||
      ((this.node.name === 'PreviewCard' || this.node.name === 'Popover') && this.has('width')) ||
      (this.node.name === 'Col' && (this.stageStretch || this.has('width')) && this.s('align', 'stretch') === 'stretch')
  }
  drawerFooterVisible(): boolean {
    const footer = this.p('footer')
    return this.has('footer') && footer.kind !== 'null' &&
      (footer.kind !== 'boolean' || footer.boolean === true) &&
      (footer.kind !== 'number' || footer.number !== 0) &&
      (footer.kind !== 'string' || footer.text !== '')
  }
  placement(): Placement {
    const side = this.s('side', this.node.name === 'Tooltip' ? 'top' : 'bottom')
    return side === 'left'
      ? Placement.Left
      : side === 'right'
        ? Placement.Right
        : side === 'top'
          ? Placement.Top
          : Placement.Bottom
  }
  inputType(): InputType {
    const type = this.s('type', 'text')
    return type === 'password'
      ? InputType.Password
      : type === 'email'
        ? InputType.Email
        : type === 'number'
          ? InputType.Number
          : type === 'tel'
            ? InputType.PhoneNumber
            : InputType.Normal
  }
  @LocalBuilder children() {
    ForEach(
      this.node.items ?? [],
      (item: SceneValue, index: number) => {
        if ((this.node.name === 'SidebarProvider' && (item.name === 'SidebarInset' || item.name === 'WindowMain')) ||
          (this.node.name === 'WindowMain' && item.name === 'WindowContent') ||
          (this.node.name === 'Sidebar' && item.name === 'SidebarContent')) {
          SceneNode({ node: item, state: this.state, revision: $revision,
            sceneId: this.sceneId, stageStretch: this.stretchChildren(),
            splitterAxis: this.splitterAxis }).layoutWeight(1)
        } else if (this.node.name === 'SidebarProvider' && item.name === 'Sidebar') {
          SceneNode({ node: item, state: this.state, revision: $revision,
            sceneId: this.sceneId, stageStretch: this.stretchChildren(),
            splitterAxis: this.splitterAxis }).zIndex(text(this.state.evaluate(item.fields?.['collapsible'] ?? stringValue('offcanvas'))) === 'none' ? 0 : 1)
        } else if (this.node.name === 'Dialog') {
          // The addressable scene wrapper participates in the dialog's Flex.
          // Keep its fixed chrome from inheriting Flex's default shrink.
          SceneNode({ node: item, state: this.state, revision: $revision,
            sceneId: this.sceneId, stageStretch: this.stretchChildren(),
            splitterAxis: this.splitterAxis }).flexShrink(item.name === 'DialogBody' ? 1 : 0)
        } else {
        SceneNode({
          node: item,
          inWindowTitle: this.inWindowTitle && (this.node.name === 'Row' || this.node.name === 'Col'),
          state: this.state,
          revision: $revision,
          sceneId: this.sceneId,
          stageStretch: this.stretchChildren(),
          splitterAxis:
            this.node.name === 'Splitter'
              ? this.s('orientation', 'horizontal')
              : this.splitterAxis,
        })
        }
      },
      (item: SceneValue, index: number) => String(index)
    )
  }
  @LocalBuilder icons() {
    ForEach(
      (this.node.items ?? []).filter(
        (item: SceneValue) => item.kind === 'element' && (item.name === 'Icon' || item.name === 'Spinner')
      ),
      (item: SceneValue, index: number) => {
        if (item.name === 'Spinner') {
          DazzSpinner({ controlSize: text(item.fields?.['size'] ?? stringValue('medium')), onAccent: item.fields?.['onAccent']?.boolean ?? false })
        } else {
        DazzIcon({
          name: item.fields?.['icon']?.text ?? '',
          iconSize:
            this.state.evaluate(item.fields?.['size'] ?? EMPTY_VALUE).number ??
            this.tokens.size(this.controlSize(), 'icon-size') -
            this.tokens.metric('control.label-glyph.shrink'),
          color: this.tokens.paint(
            this.variant(),
            'content',
            this.b('disabled') ? 'disabled' : 'normal',
            this.tint()
          ),
        }).id(this.specimenID())
        }
      },
      (item: SceneValue, index: number) => String(index)
    )
  }
  @LocalBuilder trigger() {
    SceneNode({
      node: this.node.fields?.['trigger'] ?? EMPTY_VALUE,
      state: this.state,
      revision: $revision,
      sceneId: this.sceneId,
    })
  }
  @LocalBuilder previewContent(node: SceneValue) {
    ForEach(node.items ?? [], (item: SceneValue) => {
      SceneNode({ node: item, state: this.state, revision: $revision,
        sceneId: this.sceneId, stageStretch: node.fields?.['width'] !== undefined })
    }, (item: SceneValue) => String(item.address))
  }
  @LocalBuilder action() {
    if (this.has('action')) {
      SceneNode({
        node: this.node.fields?.['action'] ?? EMPTY_VALUE,
        state: this.state,
        revision: $revision,
        sceneId: this.sceneId,
      })
    }
  }

  build() {
    if (this.node.name === 'Row') {
      Flex({
        direction: FlexDirection.Row,
        wrap: this.s('justify') === 'between' ? FlexWrap.NoWrap : FlexWrap.Wrap,
        alignItems: this.stageAlignment(),
        justifyContent:
          this.s('justify') === 'between'
            ? FlexAlign.SpaceBetween
            : FlexAlign.Start,
        space: { main: LengthMetrics.vp(this.gap()), cross: LengthMetrics.vp(this.gap()) },
      }) {
        this.children()
      }
        .width(this.has('width') ? this.length('width') : this.stageStretch ? '100%' : LayoutPolicy.wrapContent)
        .height(LayoutPolicy.wrapContent)
        .constraintSize({ maxWidth: '100%' })
    } else if (this.node.name === 'Col') {
      Flex({
        direction: FlexDirection.Column,
        alignItems: this.stageAlignment(),
        space: { main: LengthMetrics.vp(this.gap()) },
      }) {
        this.children()
      }
        .width(this.has('width') ? this.length('width') : this.stageStretch ? '100%' : LayoutPolicy.wrapContent)
        .height(LayoutPolicy.wrapContent)
        .constraintSize({ maxWidth: '100%' })
    } else if (this.node.name === 'Frame') {
      Row() {
        this.children()
      }
        .width(this.length('width', 576))
        .height(this.length('height', 256))
        .constraintSize({ maxWidth: '100%' })
        .clip(true)
        .border({
          width: this.tokens.metric('stroke.hairline'),
          color: this.tokens.color('border'),
          radius: this.tokens.metric('radius.large'),
        })
    } else if (this.node.name === 'ChromeBar') {
      SceneChrome({ contentWidth: this.n('width'), contentHeight: this.n('height'), paddingToken: this.s('padding'),
        backgroundRole: this.s('background'), borderWidthToken: this.s('borderWidth'), borderColorRole: this.s('borderColor'), content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'WindowFrame') {
      DazzWindowFrame({ platform: this.s('platform', 'macos') as DazzWindowPlatform,
        title: this.p('title').kind === 'element' ? '' : this.s('title'), subtitle: this.s('subtitle'), titleHidden: this.b('titleHidden'),
        showTitlebar: !this.has('titlebar') || this.p('titlebar').kind !== 'boolean' || this.b('titlebar'),
        fullSizeContent: this.b('fullSizeContent'), controls: this.windowControls(), inactive: this.b('inactive'),
        applicationBody: (this.node.items ?? []).some((item: SceneValue) => item.name === 'WindowBody'),
        contentWidth: this.windowLength('width'), contentHeight: this.windowLength('height'), minimumWidth: this.windowLength('minWidth'),
        maximumWidth: this.windowLength('maxWidth') ?? '100%', minimumHeight: this.windowLength('minHeight'), maximumHeight: this.windowLength('maxHeight'),
        titleContent: this.p('title').kind === 'element' ? this.windowTitle : undefined,
        leading: this.hasWindowPart('leading') ? this.windowLeading : undefined, toolbarContent: this.hasWindowPart('toolbar') ? this.windowToolbar : undefined,
        trailing: this.hasWindowPart('trailing') ? this.windowTrailing : undefined, accessory: this.hasWindowPart('accessory') ? this.windowAccessory : undefined,
        content: this.children, onClose: () => this.run('onClose'), onMinimize: () => this.run('onMinimize'), onMaximize: () => this.run('onMaximize') }).id(this.specimenID())
    } else if (this.node.name === 'WindowTitlebar') {
      DazzWindowTitlebar({ inPane: true, title: this.p('title').kind === 'element' ? '' : this.s('title'), subtitle: this.s('subtitle'), titleHidden: this.b('titleHidden'), controls: this.windowControls(),
        titleContent: this.p('title').kind === 'element' ? this.windowTitle : undefined,
        leading: this.hasWindowPart('leading') ? this.windowLeading : undefined, toolbarContent: this.hasWindowPart('toolbar') ? this.windowToolbar : undefined,
        trailing: this.hasWindowPart('trailing') ? this.windowTrailing : undefined, accessory: this.hasWindowPart('accessory') ? this.windowAccessory : undefined,
        onClose: this.has('onClose') ? () => this.run('onClose') : undefined, onMinimize: this.has('onMinimize') ? () => this.run('onMinimize') : undefined, onMaximize: this.has('onMaximize') ? () => this.run('onMaximize') : undefined }).id(this.specimenID())
    } else if (this.node.name === 'WindowBody') {
      DazzWindowBody({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'WindowMain') {
      DazzWindowMain({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'WindowContent') {
      DazzWindowContent({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'WindowFooter') {
      DazzWindowFooter({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'WindowPane') {
      Column() { this.children() }.width('100%').alignItems(HorizontalAlign.Start)
        .padding({ left: this.tokens.metric('spacing.5'), right: this.tokens.metric('spacing.5'), bottom: this.tokens.metric('spacing.5') }).id(this.specimenID())
    } else if (this.node.name === 'DateRangePicker') {
      DazzDateRangePicker({ value: this.pickerValue('value'), defaultValue: this.pickerValue('defaultValue') ?? null,
        presets: this.pickerPresets(), durations: this.pickerDurations(), labels: this.pickerLabels(), timeStep: this.n('timeStep', 15),
        months: this.n('months', 1) as 1 | 2, weekStartsOn: this.n('weekStartsOn', 1) as 0 | 1 | 2 | 3 | 4 | 5 | 6,
        locale: this.optionalString('locale'), today: this.date('today') ?? new Date(), tint: this.tint(), state: this.s('state', 'default') as 'default' | 'error', disabled: this.b('disabled'),
        controlSize: this.s('size', 'medium') as 'small' | 'medium' | 'large', fullWidth: this.stageStretch, accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: DazzDateRangeValue) => this.run('onValueChange', sceneDateRangeValue(value)) }).id(this.specimenID())
    } else if (this.node.name === 'Calendar') {
      DazzCalendar({ mode: this.s('mode', 'single') as DazzCalendarMode, value: this.calendarValue('value'), defaultValue: this.calendarValue('defaultValue') ?? null,
        month: this.date('month'), defaultMonth: this.date('defaultMonth'), today: this.date('today') ?? new Date(), min: this.date('min'), max: this.date('max'),
        months: this.n('months', 1) as 1 | 2, weekStartsOn: this.n('weekStartsOn', 1) as 0 | 1 | 2 | 3 | 4 | 5 | 6, locale: this.optionalString('locale'), navigation: this.b('navigation', true),
        controlSize: this.s('size', 'medium') as DazzCalendarSize, tint: this.tint(), accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: DazzCalendarValue) => this.calendarChanged(value), onMonthChange: (month: Date) => this.run('onMonthChange', dateValue(month)) }).id(this.specimenID())
    } else if (this.node.name === 'Tabs') {
      DazzTabs({ items: this.tabItems(), value: this.p('value').kind === 'null' ? null : this.s('value'),
        contentHeight: this.header.height, accessibilityLabel: this.s('aria-label'),
        trailing: this.has('trailing') ? this.tabsTrailing : undefined,
        onValueChange: (value: string) => this.run('onValueChange', stringValue(value)),
        onClose: this.has('onClose') ? (value: string) => this.run('onClose', stringValue(value)) : undefined }).id(this.specimenID())
    } else if (this.node.name === 'ToastProvider') {
      DazzToastProvider({ limit: this.n('limit', 3), content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'ToastViewport') {
      DazzToastViewport({ placement: this.s('placement', 'bottom'), viewportId: this.specimenID() })
    } else if (this.node.name === 'Text') {
      if (this.iconText()) {
        Row() {
          ForEach((this.node.items ?? []).filter((item: SceneValue) => item.name === 'Icon'), (item: SceneValue) => {
            DazzIcon({ name: text(this.state.evaluate(item.fields?.['icon'] ?? EMPTY_VALUE)),
              iconSize: item.fields?.['size']?.number, color: this.textColor() })
              .id(this.sceneId + '/' + String(item.address))
          }, (item: SceneValue) => String(item.address))
        }.id(this.specimenID())
      } else {
        SceneInline({ node: this.node, state: this.state, revision: $revision,
          inWindowTitle: this.inWindowTitle,
          sceneId: this.sceneId, previewContent: this.previewContent,
          onNavigate: (href: string) => this.navigation.open(href) })
      }
    } else if (this.node.name === 'Icon') {
      DazzIcon({ name: this.s('icon'), iconSize: this.optionalNumber('size'), color: this.title()?.color ?? '' }).id(
        this.specimenID()
      )
    } else if (this.node.name === 'Button') {
      DazzButton({
        label: this.label(),
        buttonType: this.s('type', 'button'),
        variant: this.variant('recessed'),
        tint: this.tint(),
        controlSize: this.controlSize('small'),
        disabled: this.b('disabled'),
        fullWidth: this.b('fullWidth'),
        shortcut: this.s('shortcut'),
        content: this.icons,
        onAction: () => this.run('onClick'),
      }).id(this.specimenID())
    } else if (this.node.name === 'IconButton') {
      DazzIconButton({
        label: this.s('label'),
        variant: this.variant('plain'),
        tint: this.s('tint', 'neutral') as DazzTint,
        controlSize: this.controlSize('tiny'),
        disabled: this.b('disabled'),
        content: this.icons,
        onAction: () => this.run('onClick'),
      }).id(this.specimenID())
    } else if (this.node.name === 'Toggle') {
      DazzToggle({
        label: this.s('label', this.label()),
        text: this.label(),
        controlSize: this.controlSize(),
        variant: this.variant(),
        tint: this.tint(),
        disabled: this.b('disabled'),
        pressed: this.optionalBoolean('pressed'),
        defaultPressed: this.b('defaultPressed'),
        content: this.icons,
        onPressedChange: (value: boolean) =>
          this.run('onPressedChange', booleanValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'SidebarProvider') {
      DazzSidebarProvider({ open: this.optionalBoolean('open'), defaultOpen: this.b('defaultOpen', true),
        content: this.children, onOpenChange: (open: boolean) => this.run('onOpenChange', booleanValue(open)) }).id(this.specimenID())
    } else if (this.node.name === 'Sidebar') {
      DazzSidebar({ side: this.s('side', 'left'), variant: this.s('variant', 'sidebar'),
        collapsible: this.s('collapsible', 'offcanvas'), content: this.children }).id(this.specimenID()).zIndex(this.s('collapsible', 'offcanvas') === 'none' ? 0 : 1)
    } else if (this.node.name === 'SidebarInset') {
      DazzSidebarInset({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'SidebarHeader') {
      DazzSidebarHeader({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'SidebarContent') {
      DazzSidebarContent({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'SidebarGroup') {
      DazzSidebarGroup({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'SidebarGroupLabel') {
      DazzSidebarGroupLabel({ label: this.label() }).id(this.specimenID())
    } else if (this.node.name === 'SidebarGroupContent') {
      DazzSidebarGroupContent({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'SidebarMenu') {
      DazzSidebarMenu({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'SidebarMenuItem') {
      DazzSidebarMenuItem({ active: this.sidebarItemValue('active').boolean ?? false,
        disabled: this.sidebarItemValue('disabled').boolean ?? false,
        variant: (text(this.sidebarItemValue('variant')) || 'filled') as DazzVariant,
        tint: (text(this.sidebarItemValue('tint')) || 'primary') as DazzTint,
        hasBadge: (this.node.items ?? []).some((item: SceneValue) => item.name === 'SidebarMenuBadge'),
        content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'SidebarMenuButton') {
      DazzSidebarMenuButton({ label: this.label(), icon: this.p('icon').fields?.['icon']?.text ?? '',
        iconSize: this.p('icon').fields?.['size']?.number,
        iconID: this.p('icon').address === undefined ? '' : this.sceneId + '/' + String(this.p('icon').address),
        active: this.optionalBoolean('active'), disabled: this.optionalBoolean('disabled'),
        variant: this.optionalString('variant') as DazzVariant | undefined,
        tint: this.optionalString('tint') as DazzTint | undefined,
        controlSize: this.s('size', 'medium'), tooltip: this.s('tooltip'),
        onAction: () => this.run('onClick') }).id(this.specimenID())
    } else if (this.node.name === 'SidebarMenuBadge') {
      DazzSidebarMenuBadge({ label: this.label(), tint: this.tint(), badgeID: this.specimenID() })
    } else if (this.node.name === 'SidebarTrigger') {
      DazzSidebarTrigger({ label: this.s('label', 'Toggle sidebar') }).id(this.specimenID())
    } else if (this.node.name === 'Accordion') {
      DazzAccordion({ defaultValue: this.strings('defaultValue'), multiple: this.b('multiple'), divided: this.b('divided'),
        content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'AccordionItem') {
      DazzAccordionItem({ value: this.s('value'), title: this.s('title'), content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'AvatarGroup') {
      DazzAvatarGroup({ people: this.avatarPeople(), max: this.n('max', Infinity), controlSize: this.s('size', 'medium') }).id(this.specimenID())
    } else if (this.node.name === 'Link') {
      DazzLink({ label: this.label(), href: this.s('href'), external: this.b('external'), underline: this.s('underline', 'hover'), tint: this.tint(),
        onNavigate: this.s('href').startsWith('#/') ? (href: string) => this.navigation.open(href) : undefined }).id(this.specimenID())
    } else if (this.node.name === 'PillTabs') {
      DazzPillTabs({ items: this.items('items'), defaultValue: this.s('defaultValue'), accessibilityLabel: this.s('aria-label') }).id(this.specimenID())
    } else if (this.node.name === 'ToggleGroup') {
      DazzToggleGroup({ items: this.items('items'), value: this.has('value') ? this.strings('value') : undefined,
        multiple: this.b('multiple'), controlSize: this.controlSize(), accessibilityLabel: this.s('aria-label'),
        onValueChange: (values: string[]) => this.stringsChanged('onValueChange', values) }).id(this.specimenID())
    } else if (this.node.name === 'Badge') {
      DazzBadge({
        label: this.label(),
        variant: this.variant('tinted'),
        tint: this.tint(),
        controlSize: this.controlSize(),
      }).id(this.specimenID())
    } else if (this.node.name === 'Avatar') {
      DazzAvatar({
        name: this.s('name'),
        fallback: this.s('fallback'),
        shape: this.s('shape', 'circle'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
      }).id(this.specimenID())
    } else if (this.node.name === 'KeyCap') {
      DazzKeyCap({
        label: this.label(),
        variant: this.s('variant', 'hint'),
        controlSize: this.s('size', 'medium'),
      }).id(this.specimenID())
    } else if (this.node.name === 'Card') {
      DazzCard({
        variant: this.s('variant', 'raised'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Checkbox') {
      DazzCheckbox({
        checked: this.optionalBoolean('checked'),
        defaultChecked: this.b('defaultChecked'),
        indeterminate: this.b('indeterminate'),
        label: this.label(),
        note: this.s('note'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        disabled: this.b('disabled'),
        onCheckedChange: (value: boolean) =>
          this.run('onCheckedChange', booleanValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'ShortcutRecorder') {
      DazzShortcutRecorder({ value: this.s('value'), placeholder: this.s('placeholder', 'Record shortcut'),
        recordingLabel: this.s('recordingLabel', 'Press keys…'), clearLabel: this.s('clearLabel', 'Clear shortcut'),
        state: this.s('state', 'default'), disabled: this.b('disabled'), tint: this.tint(),
        accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: string) => this.run('onValueChange', stringValue(value)),
        onClear: () => { if (this.has('onClear')) this.run('onClear'); else this.run('onValueChange', stringValue('')) }
      }).id(this.specimenID())
    } else if (this.node.name === 'Switch') {
      DazzSwitch({
        checked: this.optionalBoolean('checked'),
        defaultChecked: this.b('defaultChecked'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        disabled: this.b('disabled'),
        accessibilityLabel: this.s('aria-label'),
        onCheckedChange: (value: boolean) =>
          this.run('onCheckedChange', booleanValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'TextField') {
      DazzTextField({
        value: this.optionalString('value'),
        defaultValue: this.s('defaultValue'),
        placeholder: this.s('placeholder'),
        accessibilityLabel: this.s('aria-label'),
        controlSize: this.controlSize(),
        disabled: this.b('disabled'),
        readOnly: this.b('readOnly'),
        required: this.b('required'),
        multiline: this.b('multiline'),
        mono: this.b('mono'),
        rows: this.n('rows', 4),
        state: this.s('state', 'normal'),
        inputType: this.inputType(),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'OtpField') {
      DazzOtpField({
        length: this.n('length', 6),
        groupAfter: this.n('groupAfter'),
        value: this.optionalString('value'),
        defaultValue: this.s('defaultValue'),
        controlSize: this.controlSize(),
        state: this.s('state', 'default'),
        tint: this.tint(),
        disabled: this.b('disabled'),
        readOnly: this.b('readOnly'),
        required: this.b('required'),
        autoSubmit: this.b('autoSubmit'),
        accessibilityLabel: this.s('aria-label', 'Code'),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
        onValueComplete: (value: string) =>
          this.run('onValueComplete', stringValue(value)),
        onValueInvalid: (value: string) =>
          this.run('onValueInvalid', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'Tree') {
      DazzTree({
        items: this.treeItems(),
        expanded: this.has('expanded') ? this.strings('expanded') : undefined,
        defaultExpanded: this.strings('defaultExpanded'),
        selected: this.p('selected').kind === 'null' ? undefined : this.optionalString('selected'),
        controlSize: this.controlSize(),
        tint: this.tint(),
        accessibilityLabel: this.s('aria-label'),
        onSelectedChange: (id: string) => this.run('onSelectedChange', stringValue(id)),
        onExpandedChange: (ids: string[]) => this.stringsChanged('onExpandedChange', ids),
      }).id(this.specimenID())
    } else if (this.node.name === 'SearchField') {
      DazzSearchField({
        stretch: this.stageStretch,
        value: this.optionalString('value'),
        placeholder: this.s('placeholder', 'Search'),
        accessibilityLabel: this.s('aria-label', 'Search'),
        controlSize: this.controlSize(),
        disabled: this.b('disabled'),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'Combobox') {
      DazzCombobox({ options: this.options(), value: this.optionalString('value'),
        defaultValue: this.s('defaultValue'), placeholder: this.s('placeholder', 'Search…'),
        empty: this.s('empty', 'No matches'), clearable: this.b('clearable', true),
        controlSize: this.controlSize(), disabled: this.b('disabled'), accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: string | null) => this.run('onValueChange', value === null ? EMPTY_VALUE : stringValue(value))
      }).id(this.specimenID())
    } else if (this.node.name === 'Autocomplete') {
      DazzAutocomplete({ items: this.strings('items'), value: this.optionalString('value'),
        defaultValue: this.s('defaultValue'), placeholder: this.s('placeholder'), empty: this.s('empty', 'No matches'),
        controlSize: this.controlSize(), disabled: this.b('disabled'), accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: string) => this.run('onValueChange', stringValue(value))
      }).id(this.specimenID())
    } else if (this.node.name === 'Select') {
      DazzSelect({
        options: this.options(),
        value: this.optionalString('value'),
        defaultValue: this.s('defaultValue'),
        placeholder: this.s('placeholder', 'Choose'),
        controlSize: this.controlSize(),
        disabled: this.b('disabled'),
        accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'RadioGroup') {
      DazzRadioGroup({
        options: this.items('options'),
        value: this.optionalString('value'),
        defaultValue: this.s('defaultValue'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        disabled: this.b('disabled'),
        accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'Slider') {
      DazzSlider({
        value: this.sliderValue('value'),
        defaultValue: this.sliderValue('defaultValue'),
        min: this.n('min'),
        max: this.n('max', 100),
        step: this.n('step', 1),
        controlSize: this.s('size', 'medium') as DazzSliderSize,
        tint: this.tint(),
        showValue: this.b('showValue'),
        disabled: this.b('disabled'),
        accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: DazzSliderValue) =>
          this.run('onValueChange', Array.isArray(value) ? { kind: 'array', items: value.map((item: number): SceneValue => numberValue(item)) } : numberValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'Progress') {
      DazzProgress({
        gradient: this.b('gradient'),
        value: this.n('value'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        accessibilityLabel: this.s('aria-label'),
      }).id(this.specimenID())
    } else if (this.node.name === 'Spinner') {
      DazzSpinner({
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        onAccent: this.b('onAccent'),
      }).id(this.specimenID())
    } else if (this.node.name === 'Meter') {
      DazzMeter({
        value: this.n('value'),
        min: this.n('min'),
        max: this.n('max', 100),
        label: this.s('label'),
        showValue: this.b('showValue'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
      }).id(this.specimenID())
    } else if (this.node.name === 'SegmentedControl') {
      DazzSegmentedControl({
        items: this.items('items'),
        value: this.optionalString('value'),
        defaultValue: this.s('defaultValue'),
        controlSize: this.controlSize(),
        tint: this.tint(),
        activeStyle: this.s('activeStyle', 'raised'),
        iconOnly: this.b('iconOnly'),
        accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'NumberField') {
      DazzNumberField({
        value: this.optionalNumber('value'),
        defaultValue: this.n('defaultValue'),
        min: this.n('min', -Infinity),
        max: this.n('max', Infinity),
        step: this.n('step', 1),
        unit: this.s('unit'),
        controlSize: this.controlSize(),
        disabled: this.b('disabled'),
        accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: number) =>
          this.run('onValueChange', numberValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'OptionCard') {
      DazzOptionCard({
        title: this.s('title'),
        description: this.s('description'),
        selected: this.b('selected'),
        disabled: this.b('disabled'),
        tint: this.tint(),
        onAction: () => this.run('onClick'),
      }).id(this.specimenID())
    } else if (this.node.name === 'Form') {
      DazzForm({ content: this.children, onSubmit: () => this.run('onSubmit') }).id(this.specimenID())
    } else if (this.node.name === 'FormField') {
      DazzFormField({
        label: this.s('label'),
        hint: this.s('hint'),
        state: this.s('state', 'normal'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Fieldset') {
      DazzFieldset({
        legend: this.s('legend'),
        disabled: this.b('disabled'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Callout') {
      DazzCallout({
        title: this.s('title'),
        tint: this.s('tint', 'info') as DazzTint,
        content: this.children,
        action: this.action,
      }).id(this.specimenID())
    } else if (this.node.name === 'EmptyState') {
      DazzEmptyState({
        title: this.s('title'),
        content: this.children,
        action: this.action,
      }).id(this.specimenID())
    } else if (this.node.name === 'Preferences') {
      DazzPreferences({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'PreferenceGroup') {
      DazzPreferenceGroup({
        title: this.s('title'),
        description: this.s('description'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'PreferenceSection') {
      DazzPreferenceSection({
        label: this.s('label'),
        footer: this.s('footer'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'PreferenceRow') {
      DazzPreferenceRow({
        onOpen: this.has('onOpen') ? () => this.run('onOpen') : undefined,
        title: this.s('title'),
        subtitle: this.s('subtitle'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Divider') {
      DazzDivider({
        tone: this.s('tone', 'normal'),
        orientation: this.s('orientation', 'horizontal'),
      }).id(this.specimenID())
    } else if (this.node.name === 'Code') {
      DazzCode({ code: this.label() }).id(this.specimenID())
    } else if (this.node.name === 'CodeBlock') {
      DazzCodeBlock({ code: this.s('code'), title: this.s('title'), language: this.s('language'), lineNumbers: this.b('lineNumbers'), copyable: this.b('copyable') }).id(this.specimenID())
    } else if (this.node.name === 'SectionLabel') {
      DazzSectionLabel({
        label: this.label(),
        faint: this.b('faint'),
        tint: this.s('tint', 'neutral') as DazzTint,
      }).id(this.specimenID())
    } else if (this.node.name === 'Collapsible') {
      DazzCollapsible({
        title: this.s('title'),
        meta: this.s('meta'),
        defaultOpen: this.b('defaultOpen'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Skeleton') {
      DazzSkeleton({
        contentWidth: this.length('width', '100%'),
        contentHeight: this.length('height', this.tokens.metric('spacing.4')),
        shape: this.s('shape', 'rect'),
      }).id(this.specimenID())
    } else if (this.node.name === 'Drawer') {
      if (this.drawerFooterVisible()) {
        DazzDrawer({
          side: this.s('side', 'right') as DazzDrawerSide,
          title: this.s('title'), description: this.s('description'),
          extent: this.has('size') ? this.length('size') : undefined,
          trigger: this.forwardTrigger, content: this.children, footer: this.drawerFooter,
        }).id(this.specimenID())
      } else {
        DazzDrawer({
          side: this.s('side', 'right') as DazzDrawerSide,
          title: this.s('title'), description: this.s('description'),
          extent: this.has('size') ? this.length('size') : undefined,
          trigger: this.forwardTrigger, content: this.children,
        }).id(this.specimenID())
      }
    } else if (this.node.name === 'DrawerClose') {
      if (this.has('render')) {
        DazzDrawerClose({ content: this.renderSlot }).id(this.specimenID())
      } else {
        DazzDrawerClose({ label: this.label() }).id(this.specimenID())
      }
    } else if (this.node.name === 'Dialog') {
      DazzDialog({
        tone: this.s('tone', 'default'),
        accessibilityLabel: text((this.node.items ?? []).find((item: SceneValue) => item.name === 'DialogHeader')?.fields?.['title'] ?? EMPTY_VALUE),
        contentWidth: this.has('width') ? (this.length('width') as number) : undefined,
        trigger: this.forwardTrigger,
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'DialogHeader') {
      DazzDialogHeader({ title: this.s('title'), subtitle: this.s('subtitle') }).id(this.specimenID()).flexShrink(0)
    } else if (this.node.name === 'DialogBody') {
      DazzDialogBody({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'DialogFooter') {
      DazzDialogFooter({ content: this.children }).id(this.specimenID()).flexShrink(0)
    } else if (this.node.name === 'DialogClose') {
      if (this.has('render')) {
        DazzActivationBoundary({ content: this.renderSlot, onAction: () => this.dialog.close() })
      } else {
        DazzDialogClose({ label: this.label() })
      }
    } else if (this.node.name === 'PreviewCard') {
      DazzPreviewCard({ trigger: this.trigger, content: this.children,
        open: this.optionalBoolean('open'), defaultOpen: this.b('defaultOpen'),
        contentWidth: this.has('width') ? Number(this.length('width')) : undefined,
        side: this.s('side', 'bottom'), contentAlign: this.s('align', 'center'),
        delay: this.optionalNumber('delay'), closeDelay: this.optionalNumber('closeDelay'),
        onOpenChange: (open: boolean) => this.run('onOpenChange', booleanValue(open)),
      }).id(this.specimenID())
    } else if (this.node.name === 'Popover') {
      DazzPopover({
        title: this.s('title'),
        description: this.s('description'),
        contentWidth: this.has('width') ? (this.length('width') as number) : undefined,
        side: this.placement(),
        trigger: this.forwardTrigger,
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'PopoverClose') {
      DazzPopoverClose({ content: this.renderSlot }).id(this.specimenID())
    } else if (this.node.name === 'Menubar') {
      DazzMenubar({ menus: (this.p('menus').items ?? []).map((entry: SceneValue): DazzMenubarEntry => ({
        label: text(entry.fields?.['label'] ?? EMPTY_VALUE), items: this.menuItems(entry.fields?.['items'] ?? EMPTY_VALUE),
        disabled: entry.fields?.['disabled']?.boolean ?? false
      })), disabled: this.b('disabled'), orientation: this.s('orientation', 'horizontal'), modal: this.b('modal', true) }).id(this.specimenID())
    } else if (this.node.name === 'ContextMenu') {
      DazzMenu({
        items: this.menuItems(),
        contextMenu: true,
        trigger: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Menu') {
      DazzMenu({ items: this.menuItems(), trigger: this.forwardTrigger }).id(
        this.specimenID()
      )
    } else if (this.node.name === 'Tooltip') {
      DazzTooltip({
        label: this.s('label'),
        side: this.placement(),
        delay: this.optionalNumber('delay'), closeDelay: this.optionalNumber('closeDelay'), disabled: this.b('disabled'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Table') {
      DazzTable({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'TableHead') {
      DazzTableRow({ head: true, content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'TableRow') {
      DazzTableRow({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'TableCell') {
      DazzTableCell({
        head: this.b('head'),
        contentAlign: this.s('align', 'start'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'StepList') {
      DazzStepList({ content: this.children }).id(this.specimenID())
    } else if (this.node.name === 'Step') {
      DazzStep({
        status: this.s('status', 'idle'),
        label: this.s('label'),
        meta: this.s('meta'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'Tag') {
      DazzTag({
        label: this.label(),
        selected: this.b('selected'),
        tint: this.tint(),
        selectable: this.has('onSelectedChange'),
        removable: this.has('onRemove'),
        onSelectedChange: (value: boolean) =>
          this.run('onSelectedChange', booleanValue(value)),
        onRemove: () => this.run('onRemove'),
      }).id(this.specimenID())
    } else if (this.node.name === 'SkeletonText') {
      DazzSkeletonText({ lines: this.n('lines', 3) }).id(this.specimenID())
    } else if (this.node.name === 'Splitter') {
      DazzSplitter({
        orientation: this.s('orientation', 'horizontal'),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'SplitterPanel') {
      DazzSplitterPanel({
        orientation: this.splitterAxis,
        defaultSize: this.n('defaultSize', 50),
        minSize: this.n('minSize'),
        maxSize: this.n('maxSize', 100),
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'ActionBar') {
      Row({ space: this.tokens.metric('spacing.2') }) {
        this.children()
      }
        .justifyContent(
          this.s('align') === 'start'
            ? FlexAlign.Start
            : this.s('align') === 'between'
              ? FlexAlign.SpaceBetween
              : FlexAlign.End
        )
        .width('100%')
    } else if (
      this.node.kind === 'string' ||
      this.node.kind === 'number' ||
      this.node.kind === 'state' ||
      this.node.kind === 'eq'
    ) {
      Text(text(this.state.evaluate(this.node)))
        .fontSize(this.tokens.fontSize(this.title()?.size ?? this.tokens.type('bodySmall').size))
        .fontWeight(this.title()?.weight)
        .lineHeight(this.title() ? this.tokens.fontSize(this.titleStyle.leading) : undefined)
        .fontColor(this.title()?.color ?? this.tokens.color('content'))
    } else if (this.node.kind === 'element') {
      Text('Unmapped component: ' + this.node.name).fontColor(
        this.tokens.color('danger.700')
      )
    }
  }

  @LocalBuilder renderSlot() {
    SceneNode({ node: this.p('render'), state: this.state, revision: $revision, sceneId: this.sceneId })
  }
  @LocalBuilder drawerFooter() {
    SceneNode({ node: this.p('footer'), state: this.state, revision: $revision, sceneId: this.sceneId })
  }
  @LocalBuilder forwardTrigger() {
    this.trigger()
  }
}
