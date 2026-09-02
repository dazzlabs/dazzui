import {
  ActionBar,
  Button,
  Card,
  EmptyState,
  KeyCap,
  SearchField,
  Table,
  TableCell,
  TableHead,
  TableRow,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof SearchField> = {
  title: 'Components/SearchField',
  component: SearchField,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const TERMS = [
  { term: 'segment', translation: '段', hits: 412 },
  { term: 'glossary', translation: '术语库', hits: 168 },
  { term: 'provider', translation: '服务', hits: 96 },
  { term: 'placeholder', translation: '占位符', hits: 24 },
]

function Search({
  size,
  hint,
  initial = '',
  tint,
}: {
  size?: (typeof SIZES)[number]
  hint?: boolean
  initial?: string
  tint?: (typeof TINTS)[number]
}) {
  const [query, setQuery] = useState(initial)
  return (
    <div style={{ width: '18rem' }}>
      <SearchField
        hint={hint ? <KeyCap size="small">⌘F</KeyCap> : undefined}
        placeholder="Search terms"
        size={size}
        tint={tint}
        value={query}
        onValueChange={setQuery}
      />
    </div>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="SearchField"
      description="Inline search for a list or a table. The box belongs to the wrapper rather than the input, because the glyph and the trailing slot sit inside it — which is also why the focus ring comes from `:focus-within`. It holds no results of its own, so the same component narrows a table, a rail, or a menu without knowing which."
    >
      <Section
        title="The trailing slot"
        description="A hint sits there while the field is empty and gives way to the clear button once there is something to clear, so the two never compete for the same corner. Type something and press Escape: it clears first, and only dismisses once the field is already empty, so a stray press never closes a search mid-way."
      >
        <Stack>
          <Specimen label="empty">
            <Search hint />
          </Specimen>
          <Specimen label="with a query">
            <Search hint initial="placeholder" />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="The glyph and the trailing slot hold their distance from the edges at every size, so the text starts and ends on the same inset a plain TextField's does."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Search size={size} />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The tint reaches the focus ring and the focused border. Focus each of these — nothing else about the field changes, because a search box is chrome and should not be carrying a colour of its own at rest."
      >
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <Search tint={tint} />
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const Empty: Story = {
  render: () => <Search />,
}

export const WithAQuery: Story = {
  render: () => <Search initial="placeholder" />,
}

export const WithAHint: Story = {
  render: () => <Search hint />,
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <Search key={size} size={size} />
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {TINTS.map(tint => (
        <Search key={tint} tint={tint} />
      ))}
    </Col>
  ),
}

export const Dismissable: Story = {
  render: () => {
    const [query, setQuery] = useState('placeholder')
    const [dismissed, setDismissed] = useState(false)

    if (dismissed) {
      return (
        <ActionBar>
          <span style={{ color: 'var(--color-content-muted)' }}>
            Dismissed.
          </span>
          <Button size="small" onClick={() => setDismissed(false)}>
            Search again
          </Button>
        </ActionBar>
      )
    }

    return (
      <div style={{ width: '18rem' }}>
        <SearchField
          placeholder="Search terms"
          value={query}
          onValueChange={setQuery}
          onDismiss={() => setDismissed(true)}
        />
      </div>
    )
  },
}

export const FilteringAList: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    const matched = TERMS.filter(row =>
      row.term.toLowerCase().includes(query.toLowerCase())
    )

    return (
      <Col width="32rem">
        <SearchField
          hint={<KeyCap size="small">⌘F</KeyCap>}
          placeholder="Search terms"
          value={query}
          onValueChange={setQuery}
        />
        <Table>
          <TableHead>
            <TableCell head>Term</TableCell>
            <TableCell head>Translation</TableCell>
            <TableCell head align="end">
              Hits
            </TableCell>
          </TableHead>
          {matched.map(row => (
            <TableRow key={row.term}>
              <TableCell>{row.term}</TableCell>
              <TableCell>{row.translation}</TableCell>
              <TableCell align="end">{row.hits}</TableCell>
            </TableRow>
          ))}
          {matched.length === 0 ? (
            <EmptyState
              title={`No terms match “${query}”`}
              action={
                <Button onClick={() => setQuery('')}>Clear search</Button>
              }
            />
          ) : null}
        </Table>
      </Col>
    )
  },
}

export const InAPaneHeader: Story = {
  render: () => {
    const [query, setQuery] = useState('')
    return (
      <Card style={{ width: '32rem' }}>
        <ActionBar align="between">
          <div style={{ flex: 1, maxWidth: '16rem' }}>
            <SearchField
              placeholder="Search terms"
              value={query}
              onValueChange={setQuery}
            />
          </div>
          <ActionBar>
            <Button>Import</Button>
            <Button variant="filled">New term</Button>
          </ActionBar>
        </ActionBar>
      </Card>
    )
  },
}
