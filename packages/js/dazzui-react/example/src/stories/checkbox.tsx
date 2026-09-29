import { Checkbox, TINTS } from '@dazzlabs/dazzui-react'
import { Col, Row } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'checkbox',
  name: 'Checkbox',
  summary:
    'A choice that stands on its own, or one of several that may all hold.',
  Preview: () => (
    <Col>
      <Checkbox defaultChecked>Send me the digest</Checkbox>
      <Checkbox>Include archived projects</Checkbox>
    </Col>
  ),
  examples: [
    {
      title: 'States',
      Demo: () => (
        <Col>
          <Checkbox>Unchecked</Checkbox>
          <Checkbox defaultChecked>Checked</Checkbox>
          <Checkbox indeterminate>Indeterminate</Checkbox>
          <Checkbox disabled defaultChecked>
            Disabled
          </Checkbox>
        </Col>
      ),
    },
    {
      title: 'With a note',
      Demo: () => (
        <Checkbox defaultChecked note="Once a week, on Monday morning.">
          Send me the digest
        </Checkbox>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Row gap="5">
          {TINTS.map(tint => (
            <Checkbox key={tint} tint={tint} defaultChecked>
              {tint}
            </Checkbox>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
