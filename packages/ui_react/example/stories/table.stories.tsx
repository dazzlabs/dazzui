import { defineStory } from '../src/lib/scene'
import scenes from './table.scene.json'
import { Delete20Regular as Trash } from '@fluentui/react-icons'
import {
  ActionBar,
  Badge,
  Button,
  EmptyState,
  Icon,
  IconButton,
  SearchField,
  Table,
  TableCell,
  TableHead,
  TableRow,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof Table> = {
  title: 'Components/Table',
  component: Table,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const WIDTH = '38rem'

const TERMS = [
  { term: 'segment', translation: '段', hits: 412, scope: 'Global' },
  { term: 'glossary', translation: '术语库', hits: 168, scope: 'Global' },
  { term: 'provider', translation: '服务', hits: 96, scope: 'Project' },
  { term: 'placeholder', translation: '占位符', hits: 24, scope: 'Project' },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Table"
      description="A flex grid rather than a `<table>`: rows are often clickable and column widths are set by the design, both of which a real table fights. The role attributes keep it announced as a table anyway."
    >
      <Section
        title="Selectable rows"
        description="An interactive row is reachable with Tab and operable with Enter or Space. Selection is the tinted recipe — the row is picked out, not pressed — and it sits at a light step of the ramp, which is why the hover wash is neutral rather than tinted: a tinted hover would outweigh the selection it is previewing. Leave `interactive` off for a table that only reports; the rows then take no focus and grow no hover."
      >
        <SelectableTable />
      </Section>

      <Section
        title="Alignment"
        description="Cells split the row evenly and align to the start. Numbers go to the end so their digits line up column-wise — the one case where a reader is comparing down rather than across."
      >
        <Table>
          <TableHead>
            <TableCell head>Provider</TableCell>
            <TableCell head>Endpoint</TableCell>
            <TableCell head align="end">
              Characters
            </TableCell>
            <TableCell head align="end">
              Latency
            </TableCell>
          </TableHead>
          <TableRow>
            <TableCell>DeepL</TableCell>
            <TableCell>api.deepl.com</TableCell>
            <TableCell align="end">1,204,880</TableCell>
            <TableCell align="end">204 ms</TableCell>
          </TableRow>
          <TableRow>
            <TableCell>Google</TableCell>
            <TableCell>translation.googleapis.com</TableCell>
            <TableCell align="end">96,412</TableCell>
            <TableCell align="end">88 ms</TableCell>
          </TableRow>
        </Table>
      </Section>

      <Section
        title="Tints"
        description="The tint picks out the selected row. A table normally keeps one throughout, the way a navigation column does."
      >
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <div style={{ flex: 1 }}>
                <Table tint={tint}>
                  <TableRow interactive active>
                    <TableCell>Selected row</TableCell>
                    <TableCell align="end">412</TableCell>
                  </TableRow>
                  <TableRow interactive>
                    <TableCell>Hover me</TableCell>
                    <TableCell align="end">168</TableCell>
                  </TableRow>
                </Table>
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

function SelectableTable() {
  const [selected, setSelected] = useState('glossary')
  return (
    <Table>
      <TableHead>
        <TableCell head>Term</TableCell>
        <TableCell head>Translation</TableCell>
        <TableCell head align="end">
          Hits
        </TableCell>
        <TableCell head align="end">
          Scope
        </TableCell>
      </TableHead>
      {TERMS.map(row => (
        <TableRow
          key={row.term}
          interactive
          active={selected === row.term}
          onClick={() => setSelected(row.term)}
        >
          <TableCell>{row.term}</TableCell>
          <TableCell>{row.translation}</TableCell>
          <TableCell align="end">{row.hits}</TableCell>
          <TableCell align="end">
            <Badge size="small" tint="neutral">
              {row.scope}
            </Badge>
          </TableCell>
        </TableRow>
      ))}
    </Table>
  )
}

export const Selectable: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <SelectableTable />
    </div>
  ),
}

export const Inert: Story = defineStory(scenes.Inert)

export const Alignment: Story = defineStory(scenes.Alignment)

export const WithControls: Story = {
  render: () => {
    const [selected, setSelected] = useState('glossary')
    return (
      <div style={{ width: WIDTH }}>
        <Table>
          <TableHead>
            <TableCell head>Term</TableCell>
            <TableCell head>Translation</TableCell>
            <TableCell head align="end">
              Scope
            </TableCell>
            <TableCell head align="end" />
          </TableHead>
          {TERMS.map(row => (
            <TableRow
              key={row.term}
              interactive
              active={selected === row.term}
              onClick={() => setSelected(row.term)}
            >
              <TableCell>{row.term}</TableCell>
              <TableCell>{row.translation}</TableCell>
              <TableCell align="end">
                <Badge
                  size="small"
                  tint={row.scope === 'Global' ? 'primary' : 'neutral'}
                >
                  {row.scope}
                </Badge>
              </TableCell>
              <TableCell align="end">
                <IconButton
                  label={`Remove ${row.term}`}
                  size="small"
                  tint="danger"
                >
                  <Icon icon={Trash} size={14} />
                </IconButton>
              </TableCell>
            </TableRow>
          ))}
        </Table>
      </div>
    )
  },
}

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <Table key={tint} tint={tint}>
          <TableRow interactive active>
            <TableCell>Selected row</TableCell>
            <TableCell align="end">412</TableCell>
          </TableRow>
          <TableRow interactive>
            <TableCell>Hover me</TableCell>
            <TableCell align="end">168</TableCell>
          </TableRow>
        </Table>
      ))}
    </Col>
  ),
}

export const Empty: Story = {
  render: () => {
    const [query, setQuery] = useState('placeholder')
    const matched = TERMS.filter(row =>
      row.term.toLowerCase().includes(query.toLowerCase())
    )

    return (
      <Col width={WIDTH}>
        <ActionBar align="between">
          <div style={{ width: '16rem' }}>
            <SearchField
              placeholder="Search terms"
              value={query}
              onValueChange={setQuery}
            />
          </div>
          <Button onClick={() => setQuery('')}>Clear</Button>
        </ActionBar>
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
            <EmptyState title={`No terms match “${query}”`} />
          ) : null}
        </Table>
      </Col>
    )
  },
}
