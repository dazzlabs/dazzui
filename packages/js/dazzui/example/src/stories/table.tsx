import { Badge, Table, TableCell, TableHead, TableRow } from '@dazzlabs/dazzui'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const DEPLOYS = [
  { name: 'web', region: 'Frankfurt', status: 'Live', tint: 'success' },
  { name: 'api', region: 'Virginia', status: 'Building', tint: 'info' },
  { name: 'docs', region: 'Tokyo', status: 'Failed', tint: 'danger' },
] as const

function Deploys() {
  return (
    <Table>
      <TableHead>
        <TableCell head>Service</TableCell>
        <TableCell head>Region</TableCell>
        <TableCell head align="end">
          Status
        </TableCell>
      </TableHead>
      {DEPLOYS.map(row => (
        <TableRow key={row.name}>
          <TableCell>{row.name}</TableCell>
          <TableCell>{row.region}</TableCell>
          <TableCell align="end">
            <Badge tint={row.tint}>{row.status}</Badge>
          </TableCell>
        </TableRow>
      ))}
    </Table>
  )
}

const story: Story = {
  id: 'table',
  name: 'Table',
  summary: 'Rows of the same kind of thing, lined up to be compared.',
  Preview: () => (
    <Col width="20rem">
      <Deploys />
    </Col>
  ),
  examples: [
    {
      title: 'Default',
      Demo: () => (
        <Col width="32rem">
          <Deploys />
        </Col>
      ),
    },
  ],
}

export default story
