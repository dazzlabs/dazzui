import {
  Button,
  KeyCap,
  type KeyCapVariant,
  Menu,
  PreferenceRow,
  PreferenceSection,
  SearchField,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof KeyCap> = {
  title: 'Components/KeyCap',
  component: KeyCap,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** Quietest first: the order the design reaches for them. */
const VARIANTS: KeyCapVariant[] = ['hint', 'strong', 'key']

const SIZES = ['small', 'medium', 'large'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="KeyCap"
      description="One key, drawn three ways. It reads the theme's ink and surface roles directly and takes no tint — a key looks like a key whatever is around it. A cap says what the key *is*; where the key can be changed, that is a ShortcutRecorder instead."
    >
      <Section
        title="Variants"
        description="`hint` — the default — is bare faint text in the display cut: the `⌘F` beside an empty search field, the `⌥2` in a card's corner, an aside the eye may skip. `strong` is the same a step darker, for a hint that has to survive on a busy row. `key` alone draws the cap — a recessed, chip-cornered fill with no border, since a key is pressed *into* the surface rather than raised off it — and is what a settings page lists shortcuts in."
      >
        <Stack>
          <Specimen label="hint">
            <KeyCap variant="hint">⌥2</KeyCap>
          </Specimen>
          <Specimen label="strong">
            <KeyCap variant="strong">⌘K</KeyCap>
          </Specimen>
          <Specimen label="key">
            <KeyCap variant="key">⌘⇧T</KeyCap>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="Colours are shared across the three; only the type steps, and only at the top — `small` and `medium` set the same face, `large` goes one up. Pick the size from what the key sits beside: `small` next to body text, `medium` in a row of controls."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <KeyCap size={size}>⌘K</KeyCap>
              <KeyCap size={size} variant="strong">
                ⌘K
              </KeyCap>
              <KeyCap size={size} variant="key">
                ⌘
              </KeyCap>
              <KeyCap size={size} variant="key">
                ⇧
              </KeyCap>
              <KeyCap size={size} variant="key">
                K
              </KeyCap>
              <KeyCap size={size} variant="key">
                Esc
              </KeyCap>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Chords"
        description="A chord is several caps composed by the caller, which keeps the separator the caller's decision: macOS sets its modifiers flush, a cross-platform app usually wants a `+` between them. A hint sets the whole chord in one cap — `⌘⇧K` is one glyph run, not three — and the cap sits on the text baseline it is set in, so a sentence naming a shortcut does not gain a line of leading."
      >
        <Stack>
          <Specimen label="flush">
            <KeyCap size="small" variant="key">
              ⌘
            </KeyCap>
            <KeyCap size="small" variant="key">
              ⇧
            </KeyCap>
            <KeyCap size="small" variant="key">
              K
            </KeyCap>
          </Specimen>
          <Specimen label="joined">
            <span
              style={{
                display: 'inline-flex',
                gap: 'var(--spacing-1)',
                alignItems: 'center',
                color: 'var(--color-content-muted)',
              }}
            >
              <KeyCap size="small" variant="key">
                Ctrl
              </KeyCap>
              +
              <KeyCap size="small" variant="key">
                Shift
              </KeyCap>
              +
              <KeyCap size="small" variant="key">
                P
              </KeyCap>
            </span>
          </Specimen>
          <Specimen label="as a hint">
            <KeyCap size="small">⌘⇧K</KeyCap>
          </Specimen>
          <Specimen label="in prose">
            <span>
              Open the command palette with <KeyCap size="small">⌘K</KeyCap>,
              then type a document name.
            </span>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Hint: Story = {
  render: () => <KeyCap>⌥2</KeyCap>,
}

export const Strong: Story = {
  render: () => <KeyCap variant="strong">⌘K</KeyCap>,
}

export const Key: Story = {
  render: () => <KeyCap variant="key">⌘⇧T</KeyCap>,
}

export const AllVariants: Story = {
  render: () => (
    <Row gap="var(--spacing-5)">
      {VARIANTS.map(variant => (
        <KeyCap key={variant} variant={variant}>
          ⌘K
        </KeyCap>
      ))}
    </Row>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <Row key={size} gap="var(--spacing-1)">
          <KeyCap size={size} variant="key">
            ⌘
          </KeyCap>
          <KeyCap size={size} variant="key">
            ⇧
          </KeyCap>
          <KeyCap size={size} variant="key">
            K
          </KeyCap>
          <KeyCap size={size} variant="key">
            Esc
          </KeyCap>
        </Row>
      ))}
    </Col>
  ),
}

export const AChord: Story = {
  render: () => (
    <Row gap="var(--spacing-1)">
      <KeyCap size="small" variant="key">
        ⌘
      </KeyCap>
      <KeyCap size="small" variant="key">
        ⇧
      </KeyCap>
      <KeyCap size="small" variant="key">
        K
      </KeyCap>
    </Row>
  ),
}

export const AJoinedChord: Story = {
  render: () => (
    <span
      style={{
        display: 'inline-flex',
        gap: 'var(--spacing-1)',
        alignItems: 'center',
        color: 'var(--color-content-muted)',
      }}
    >
      <KeyCap size="small" variant="key">
        Ctrl
      </KeyCap>
      +
      <KeyCap size="small" variant="key">
        Shift
      </KeyCap>
      +
      <KeyCap size="small" variant="key">
        P
      </KeyCap>
    </span>
  ),
}

export const WordKeys: Story = {
  render: () => (
    <Row gap="var(--spacing-1)">
      <KeyCap size="small" variant="key">
        Esc
      </KeyCap>
      <KeyCap size="small" variant="key">
        Tab
      </KeyCap>
      <KeyCap size="small" variant="key">
        Space
      </KeyCap>
      <KeyCap size="small" variant="key">
        ↩
      </KeyCap>
      <KeyCap size="small" variant="key">
        ⌫
      </KeyCap>
    </Row>
  ),
}

export const InProse: Story = {
  render: () => (
    <div style={{ maxWidth: '26rem' }}>
      <p style={{ margin: 0 }}>
        Open the command palette with <KeyCap size="small">⌘K</KeyCap>, then
        type a document name. Press <KeyCap size="small">Esc</KeyCap> to close.
      </p>
    </div>
  ),
}

export const AsASearchHint: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    return (
      <div style={{ width: '18rem' }}>
        <SearchField
          hint={<KeyCap size="small">⌘F</KeyCap>}
          placeholder="Search terms"
          value={query}
          onValueChange={setQuery}
        />
      </div>
    )
  },
}

export const InAMenu: Story = {
  render: () => (
    <Menu
      align="start"
      trigger={({ open }) => (
        <Button tint="neutral" variant={open ? 'filled' : 'normal'}>
          Options
        </Button>
      )}
      items={[
        { label: 'Duplicate', shortcut: <KeyCap size="small">⌘D</KeyCap> },
        { label: 'Rename', shortcut: <KeyCap size="small">↩</KeyCap> },
        { label: 'Export…', shortcut: <KeyCap size="small">⌘E</KeyCap> },
      ]}
    />
  ),
}

/* The settings page lists a shortcut as the keys themselves, not as a hint. */
export const InASettingsRow: Story = {
  render: () => (
    <div style={{ width: '28rem' }}>
      <PreferenceSection label="Shortcuts">
        <PreferenceRow title="Command palette" subtitle="Cannot be changed">
          <span style={{ display: 'inline-flex', gap: 'var(--spacing-1)' }}>
            <KeyCap size="small" variant="key">
              ⌘
            </KeyCap>
            <KeyCap size="small" variant="key">
              K
            </KeyCap>
          </span>
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}
