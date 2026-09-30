import {
  ArrowReply24Regular,
  Copy24Regular,
  Delete24Regular,
  Heart24Regular,
  Share24Regular,
} from '@fluentui/react-icons'
import {
  ContextMenu,
  type MenuItem,
  NavigationBar,
  Screen,
  Text,
} from '@dazzlabs/dazzui-mobile'
import type { CSSProperties } from 'react'

import { Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

/*
 * The specimens draw themselves with inline styles, so the copy iOS lifts
 * over the backdrop is drawn exactly as the original is.
 */

const ACCENTS = [
  'var(--color-accent)',
  'var(--color-accent-info)',
  'var(--color-accent-success)',
  'var(--color-accent-warning)',
  'var(--color-accent-danger)',
  'var(--color-accent-neutral)',
]

const PHOTOS = ACCENTS.flatMap((accent, index) => [
  accent,
  `color-mix(in srgb, ${ACCENTS[(index + 2) % ACCENTS.length]} 60%, ${accent})`,
])

const MESSAGES: { text: string; mine: boolean }[] = [
  { text: 'Are we still on for lunch?', mine: false },
  { text: 'Yes — 12:30 at the usual place.', mine: true },
  { text: 'Perfect. I’ll book a table by the window.', mine: false },
  { text: 'See you there!', mine: true },
]

const ITEMS: readonly (MenuItem | null)[] = [
  { label: 'Copy', icon: <Copy24Regular /> },
  { label: 'Share', icon: <Share24Regular /> },
  null,
  { label: 'Delete', icon: <Delete24Regular />, role: 'destructive' },
]

const MESSAGE_ITEMS: readonly (MenuItem | null)[] = [
  { label: 'Reply', icon: <ArrowReply24Regular /> },
  { label: 'Like', icon: <Heart24Regular /> },
  ...ITEMS,
]

const grid: CSSProperties = {
  display: 'grid',
  gridTemplateColumns: 'repeat(4, 1fr)',
  gap: 2,
  margin: '0 -20px',
}

const tile = (fill: string): CSSProperties => ({
  aspectRatio: '1',
  background: fill,
  cursor: 'default',
})

const bubble = (mine: boolean): CSSProperties => ({
  alignSelf: mine ? 'flex-end' : 'flex-start',
  maxWidth: '75%',
  padding: '8px 14px',
  borderRadius: 18,
  background: mine ? 'var(--color-accent)' : 'var(--color-surface-muted)',
  color: mine ? 'var(--color-on-accent)' : 'var(--color-content)',
  font: '400 16px/21px var(--dzm-font)',
  cursor: 'default',
})

function Photo({ fill, index }: { fill: string; index: number }) {
  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'flex-end',
        width: 300,
        height: 220,
        padding: 16,
        background: fill,
        color: 'var(--color-on-accent)',
        font: '600 17px/22px var(--dzm-font)',
      }}
    >
      Photo {index + 1}
    </div>
  )
}

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Context Menu" />}>
      <Stage>
        <Group title="Photos — a larger preview lifts">
          <div style={grid}>
            {PHOTOS.map((fill, index) => (
              <ContextMenu
                key={index}
                items={ITEMS}
                preview={<Photo fill={fill} index={index} />}
              >
                <div
                  role="img"
                  aria-label={`Photo ${index + 1}`}
                  style={tile(fill)}
                />
              </ContextMenu>
            ))}
          </div>
        </Group>
        <Group title="Messages — the bubble itself lifts">
          <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
            {MESSAGES.map((message, index) => (
              <ContextMenu key={index} items={MESSAGE_ITEMS}>
                <div style={bubble(message.mine)}>{message.text}</div>
              </ContextMenu>
            ))}
          </div>
        </Group>
        <Text role="footnote" tone="secondary">
          Press and hold; on a desktop, right-click.
        </Text>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'context-menu',
  name: 'Context Menu',
  summary:
    'A long press lifts the element over a blur with the glass menu beside it; Material’s menu at the finger.',
  Demo,
}

export default story
