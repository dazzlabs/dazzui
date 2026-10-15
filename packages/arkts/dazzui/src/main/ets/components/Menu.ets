import { DazzTokens } from '../foundation/Tokens'

import { DazzButton } from './Button'

export interface DazzMenuItem {
  label: string
  disabled?: boolean
  icon?: string
  shortcut?: string
  checked?: boolean
  onSelect?: () => void
}

@Component
export struct DazzMenu {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop items: DazzMenuItem[] = []
  @Prop contextMenu: boolean = false
  @BuilderParam trigger: () => void = this.defaultTrigger
  @LocalBuilder defaultTrigger() {
    DazzButton({ label: 'Menu' })
  }
  @LocalBuilder menu() {
    Menu() {
      ForEach(
        this.items,
        (item: DazzMenuItem, index: number) => {
          MenuItem({
            content: item.label,
            labelInfo: item.shortcut,
            startIcon: item.icon
              ? $rawfile('dazzui/icons/' + item.icon + '.svg')
              : undefined,
          })
            .selected(item.checked ?? false)
            .contentFont({ size: this.tokens.type('labelSmall').size })
            .contentFontColor(
              this.tokens.color(item.disabled ? 'content-faint' : 'content')
            )
            .enabled(!item.disabled)
            .onClick(() => {
              item.onSelect?.()
            })
        },
        (item: DazzMenuItem, index: number) => String(index)
      )
    }
      .backgroundColor(this.tokens.color('surface-overlay'))
      .borderRadius(this.tokens.metric('frame.popover.radius'))
      .constraintSize({ minWidth: this.tokens.metric('menu.min-width') })
  }
  build() {
    if (this.contextMenu) {
      Row() {
        this.trigger()
      }
        .bindContextMenu(this.menu, ResponseType.RightClick)
    } else {
      Row() {
        this.trigger()
      }
        .bindMenu(this.menu)
    }
  }
}
