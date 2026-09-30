import { NumberField } from '@dazzlabs/dazzui'
import { Col, Row } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'number-field',
  name: 'NumberField',
  summary: 'A number, stepped or typed, with the unit it is counted in.',
  Preview: () => (
    <Col width="10rem">
      <NumberField defaultValue={12} unit="px" aria-label="Size" />
    </Col>
  ),
  examples: [
    {
      title: 'With a unit and bounds',
      Demo: () => (
        <Row>
          <Col width="10rem">
            <NumberField defaultValue={12} unit="px" aria-label="Size" />
          </Col>
          <Col width="10rem">
            <NumberField
              defaultValue={80}
              min={0}
              max={100}
              step={5}
              unit="%"
              aria-label="Opacity"
            />
          </Col>
        </Row>
      ),
    },
  ],
}

export default story
