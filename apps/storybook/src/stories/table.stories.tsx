import {
  Badge,
  Table,
  TableCell,
  TableHead,
  TableRow,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Table> = {
  title: 'Components/Table',
  component: Table,
}

export default meta
type Story = StoryObj<typeof meta>

const TERMS = [
  { term: 'segment', translation: '段', hits: 412, scope: 'Global' },
  { term: 'glossary', translation: '术语库', hits: 168, scope: 'Global' },
  { term: 'provider', translation: '服务', hits: 96, scope: 'Project' },
  { term: 'placeholder', translation: '占位符', hits: 24, scope: 'Project' },
]

function Glossary() {
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

export const Overview: Story = {
  render: () => (
    <Page
      title="Table"
      description="A flex grid rather than a `<table>`: rows are often clickable and column widths are set by the design, both of which a real table fights. The role attributes keep it announced as a table anyway."
    >
      <Section
        title="Selectable rows"
        description="An interactive row is reachable with Tab and operable with Enter or Space. Selection is the tinted recipe — the row is picked out, not pressed."
      >
        <Glossary />
      </Section>

      <Section
        title="Inert"
        description="Leave `interactive` off for a table that only reports; the rows then take no focus and grow no hover."
      >
        <Table>
          <TableHead>
            <TableCell head>Stage</TableCell>
            <TableCell head align="end">
              Segments
            </TableCell>
          </TableHead>
          <TableRow>
            <TableCell>Parsed</TableCell>
            <TableCell align="end">392</TableCell>
          </TableRow>
          <TableRow>
            <TableCell>Translated</TableCell>
            <TableCell align="end">168</TableCell>
          </TableRow>
        </Table>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('table')}
          labelOf={token => token.name.slice('table-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
