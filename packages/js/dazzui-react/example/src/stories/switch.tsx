import { Switch, TINTS } from '@dazzlabs/dazzui-react'
import { Col, Row, Text } from '../components/stage'
import type { Story } from '../lib/story'

/** A switch draws no label of its own, so the row that names it is here. */
function Labelled({ label, on = false }: { label: string; on?: boolean }) {
  return (
    <Row justify="between" gap="4">
      <Text>{label}</Text>
      <Switch defaultChecked={on} aria-label={label} />
    </Row>
  )
}

const story: Story = {
  id: 'switch',
  name: 'Switch',
  summary: 'A setting that takes effect the moment it is flipped.',
  Preview: () => (
    <Col width="14rem">
      <Labelled label="Notifications" on />
      <Labelled label="Sync over cellular" />
    </Col>
  ),
  examples: [
    {
      title: 'Sizes',
      Demo: () => (
        <Row gap="5">
          <Switch size="small" defaultChecked aria-label="Small" />
          <Switch size="medium" defaultChecked aria-label="Medium" />
          <Switch size="large" defaultChecked aria-label="Large" />
        </Row>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Row gap="5">
          {TINTS.map(tint => (
            <Switch key={tint} tint={tint} defaultChecked aria-label={tint} />
          ))}
        </Row>
      ),
    },
    {
      title: 'Disabled',
      Demo: () => (
        <Row gap="5">
          <Switch disabled aria-label="Off" />
          <Switch disabled defaultChecked aria-label="On" />
        </Row>
      ),
    },
  ],
}

export default story
