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
  DazzSearchField,
  DazzCheckbox,
  DazzSwitch,
  DazzSelect,
  DazzRadioGroup,
  DazzSlider,
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
import { SceneValue } from './SceneTypes'
import {
  SceneState,
  EMPTY_VALUE,
  text,
  stringValue,
  booleanValue,
  numberValue,
} from './SceneState'

@Component
export struct SceneNode {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.dialog') dialog: DazzDialogContext = new DazzDialogContext()
  @Prop node: SceneValue
  @Prop sceneId: string = ''
  @Prop splitterAxis: string = 'horizontal'
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
    this.state.run(this.node.fields?.[key] ?? EMPTY_VALUE, argument)
    this.revision++
  }
  items(key: string): DazzItem[] {
    return (this.p(key).items ?? []).map((item: SceneValue): DazzItem => {
      const fields = item.fields ?? {}
      return {
        value: text(fields['value'] ?? EMPTY_VALUE),
        label: text(fields['label'] ?? EMPTY_VALUE),
        disabled: fields['disabled']?.boolean ?? false,
      }
    })
  }
  menuItems(): DazzMenuItem[] {
    return (this.p('items').items ?? []).map(
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
              this.state.run(fields['onSelect'])
              this.revision++
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
        SceneNode({
          node: item,
          state: this.state,
          revision: $revision,
          sceneId: this.sceneId,
          splitterAxis:
            this.node.name === 'Splitter'
              ? this.s('orientation', 'horizontal')
              : this.splitterAxis,
        })
      },
      (item: SceneValue, index: number) => String(index)
    )
  }
  @LocalBuilder icons() {
    ForEach(
      (this.node.items ?? []).filter(
        (item: SceneValue) => item.kind === 'element' && item.name === 'Icon'
      ),
      (item: SceneValue, index: number) => {
        DazzIcon({
          name: item.fields?.['icon']?.text ?? '',
          iconSize:
            this.tokens.size(this.controlSize(), 'icon-size') -
            this.tokens.metric('control.label-glyph.shrink'),
          color: this.tokens.paint(
            this.variant(),
            'content',
            this.b('disabled') ? 'disabled' : 'normal',
            this.tint()
          ),
        }).id(this.specimenID())
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
        .width(this.length('width'))
        .constraintSize({ maxWidth: '100%' })
    } else if (this.node.name === 'Col') {
      Flex({
        direction: FlexDirection.Column,
        alignItems: this.stageAlignment(),
        space: { main: LengthMetrics.vp(this.gap()) },
      }) {
        this.children()
      }
        .width(this.length('width'))
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
    } else if (this.node.name === 'Text') {
      Text(this.label())
        .fontSize(this.tokens.type('bodySmall').size)
        .fontWeight(
          this.tokens.type(this.b('strong') ? 'labelStrong' : 'bodySmall')
            .weight
        )
        .fontColor(
          this.has('tint')
            ? this.tokens.color(this.tint() + '.500')
            : this.tokens.color(
                this.s('tone') === 'secondary' ? 'content-secondary' : 'content'
              )
        )
    } else if (this.node.name === 'Icon') {
      DazzIcon({ name: this.s('icon'), iconSize: this.optionalNumber('size') }).id(
        this.specimenID()
      )
    } else if (this.node.name === 'Button') {
      DazzButton({
        label: this.label(),
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
        label: this.label(),
        note: this.s('note'),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        disabled: this.b('disabled'),
        onCheckedChange: (value: boolean) =>
          this.run('onCheckedChange', booleanValue(value)),
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
        multiline: this.b('multiline'),
        mono: this.b('mono'),
        rows: this.n('rows', 4),
        state: this.s('state', 'normal'),
        inputType: this.inputType(),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'SearchField') {
      DazzSearchField({
        value: this.optionalString('value'),
        placeholder: this.s('placeholder', 'Search'),
        accessibilityLabel: this.s('aria-label', 'Search'),
        controlSize: this.controlSize(),
        disabled: this.b('disabled'),
        onValueChange: (value: string) =>
          this.run('onValueChange', stringValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'Select') {
      DazzSelect({
        options: this.items('options'),
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
        value: this.optionalNumber('value'),
        defaultValue: this.n('defaultValue'),
        min: this.n('min'),
        max: this.n('max', 100),
        step: this.n('step', 1),
        controlSize: this.s('size', 'medium'),
        tint: this.tint(),
        showValue: this.b('showValue'),
        disabled: this.b('disabled'),
        accessibilityLabel: this.s('aria-label'),
        onValueChange: (value: number) =>
          this.run('onValueChange', numberValue(value)),
      }).id(this.specimenID())
    } else if (this.node.name === 'Progress') {
      DazzProgress({
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
        activeStyle: this.s('activeStyle', 'normal'),
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
      DazzCode({ code: this.s('code'), block: true }).id(this.specimenID())
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
    } else if (this.node.name === 'Dialog') {
      DazzDialog({
        contentWidth: this.has('width') ? (this.length('width') as number) : undefined,
        trigger: this.forwardTrigger,
        content: this.children,
      }).id(this.specimenID())
    } else if (this.node.name === 'DialogHeader') {
      DazzFormField({ label: this.s('title'), hint: this.s('subtitle') }).id(
        this.specimenID()
      )
    } else if (this.node.name === 'DialogBody') {
      Column({ space: this.tokens.metric('spacing.4') }) {
        this.children()
      }
        .alignItems(HorizontalAlign.Start)
        .width('100%')
    } else if (this.node.name === 'DialogFooter') {
      Row({ space: this.tokens.metric('spacing.2') }) {
        this.children()
      }
        .justifyContent(FlexAlign.End)
        .width('100%')
    } else if (this.node.name === 'DialogClose') {
      if (this.has('render')) {
        Row() {
          SceneNode({
            node: this.p('render'),
            state: this.state,
            revision: $revision,
            sceneId: this.sceneId,
          })
        }
          .onClick(() => this.dialog.close())
      } else {
        DazzDialogClose({ label: this.label() })
      }
    } else if (this.node.name === 'Popover') {
      DazzPopover({
        title: this.s('title'),
        description: this.s('description'),
        contentWidth: this.has('width') ? (this.length('width') as number) : undefined,
        side: this.placement(),
        trigger: this.forwardTrigger,
        content: this.children,
      }).id(this.specimenID())
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
        .fontSize(this.tokens.type('bodySmall').size)
        .fontColor(this.tokens.color('content'))
    } else if (this.node.kind === 'element') {
      Text('Unmapped component: ' + this.node.name).fontColor(
        this.tokens.color('danger.700')
      )
    }
  }

  @LocalBuilder forwardTrigger() {
    this.trigger()
  }
}
