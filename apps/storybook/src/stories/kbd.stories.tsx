import {
  Button,
  Kbd,
  Menu,
  PreferenceRow,
  PreferenceSection,
  SearchField,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Kbd> = {
  title: 'Components/Kbd',
  component: Kbd,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Kbd"
      description="One key cap. Unlike a badge it carries its own surface, content, and border colours in the token graph, so it takes no tint — a key looks like a key whatever is around it. A cap says what the key *is*; where the key can be changed, that is a ShortcutRecorder instead."
    >
      <Section
        title="Sizes"
        description="Colours are shared across the three; only the type and the metrics step. Pick the size from what the cap sits beside — `small` next to body text, `medium` in a row of controls."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Kbd size={size}>⌘</Kbd>
              <Kbd size={size}>⇧</Kbd>
              <Kbd size={size}>K</Kbd>
              <Kbd size={size}>Esc</Kbd>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Chords"
        description="A chord is several caps composed by the caller, which keeps the separator the caller's decision: macOS sets its modifiers flush, a cross-platform app usually wants a `+` between them. The cap sits on the text baseline it is set in, so a sentence naming a shortcut does not gain a line of leading."
      >
        <Stack>
          <Specimen label="flush">
            <Kbd size="small">⌘</Kbd>
            <Kbd size="small">⇧</Kbd>
            <Kbd size="small">K</Kbd>
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
              <Kbd size="small">Ctrl</Kbd>+<Kbd size="small">Shift</Kbd>+
              <Kbd size="small">P</Kbd>
            </span>
          </Specimen>
          <Specimen label="in prose">
            <span>
              Open the command palette with <Kbd size="small">⌘</Kbd>{' '}
              <Kbd size="small">K</Kbd>, then type a document name.
            </span>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => <Kbd>⌘</Kbd>,
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <Row key={size} gap="var(--spacing-1)">
          <Kbd size={size}>⌘</Kbd>
          <Kbd size={size}>⇧</Kbd>
          <Kbd size={size}>K</Kbd>
          <Kbd size={size}>Esc</Kbd>
        </Row>
      ))}
    </Col>
  ),
}

export const AChord: Story = {
  render: () => (
    <Row gap="var(--spacing-1)">
      <Kbd size="small">⌘</Kbd>
      <Kbd size="small">⇧</Kbd>
      <Kbd size="small">K</Kbd>
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
      <Kbd size="small">Ctrl</Kbd>+<Kbd size="small">Shift</Kbd>+
      <Kbd size="small">P</Kbd>
    </span>
  ),
}

export const WordKeys: Story = {
  render: () => (
    <Row gap="var(--spacing-1)">
      <Kbd size="small">Esc</Kbd>
      <Kbd size="small">Tab</Kbd>
      <Kbd size="small">Space</Kbd>
      <Kbd size="small">↩</Kbd>
      <Kbd size="small">⌫</Kbd>
    </Row>
  ),
}

export const InProse: Story = {
  render: () => (
    <div style={{ maxWidth: '26rem' }}>
      <p style={{ margin: 0 }}>
        Open the command palette with <Kbd size="small">⌘</Kbd>{' '}
        <Kbd size="small">K</Kbd>, then type a document name. Press{' '}
        <Kbd size="small">Esc</Kbd> to close.
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
          hint={<Kbd size="small">⌘F</Kbd>}
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
      trigger={({ open, onClick }) => (
        <Button
          aria-expanded={open}
          aria-haspopup="menu"
          tint="neutral"
          variant={open ? 'filled' : 'normal'}
          onClick={onClick}
        >
          Options
        </Button>
      )}
      items={[
        { label: 'Duplicate', shortcut: <Kbd size="small">⌘D</Kbd> },
        { label: 'Rename', shortcut: <Kbd size="small">↩</Kbd> },
        { label: 'Export…', shortcut: <Kbd size="small">⌘E</Kbd> },
      ]}
    />
  ),
}

export const InASettingsRow: Story = {
  render: () => (
    <div style={{ width: '28rem' }}>
      <PreferenceSection label="Shortcuts">
        <PreferenceRow title="Command palette" subtitle="Cannot be changed">
          <span style={{ display: 'inline-flex', gap: 'var(--spacing-1)' }}>
            <Kbd size="small">⌘</Kbd>
            <Kbd size="small">K</Kbd>
          </span>
        </PreferenceRow>
      </PreferenceSection>
    </div>
  ),
}
