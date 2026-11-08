import { NumberField } from '@dazzlabs/dazzui'
import { useState } from 'react'
import { Col, Row } from '../../components/stage'
import type { StoryExtension } from '../../lib/story'
function Controlled() {
  const [value, setValue] = useState<number | null>(12)
  return (
    <Col width="10rem">
      <NumberField
        value={value}
        onValueChange={setValue}
        step={5}
        smallStep={0.25}
        largeStep={20}
        min={0}
        max={100}
        aria-label="Controlled number"
      />
    </Col>
  )
}
function Committed() {
  const [value, setValue] = useState<number | null>(12)
  const [committed, setCommitted] = useState<number | null>(12)
  return (
    <Row>
      <Col width="10rem">
        <NumberField
          value={value}
          onValueChange={setValue}
          onValueCommitted={setCommitted}
          min={0}
          max={100}
          allowOutOfRange
          aria-label="Editable value"
        />
      </Col>
      <Col width="10rem">
        <NumberField value={committed} readOnly aria-label="Committed value" />
      </Col>
    </Row>
  )
}
const extension: StoryExtension = {
  id: 'number-field',
  platforms: ['gpui'],
  examples: [
    {
      title: 'Stepping',
      note: 'Arrow keys add the step; Shift and Alt use the large and small steps. Grid snapping is explicit.',
      Demo: () => (
        <Row>
          <Col width="10rem">
            <NumberField
              defaultValue={12}
              step={5}
              aria-label="Relative steps"
            />
          </Col>
          <Col width="10rem">
            <NumberField
              defaultValue={12}
              step={5}
              min={0}
              snapOnStep
              aria-label="Snapped steps"
            />
          </Col>
        </Row>
      ),
    },
    {
      title: 'Controlled',
      note: 'The parent holds the number, including an empty value. Home and End reach the bounds.',
      Demo: Controlled,
    },
    {
      title: 'States and precision',
      note: 'Formatting the display preserves the number used for the next step.',
      Demo: () => (
        <Row>
          <Col width="10rem">
            <NumberField
              defaultValue={0.123456789}
              step={0.00000001}
              size="small"
              aria-label="Precision"
            />
          </Col>
          <Col width="10rem">
            <NumberField defaultValue={12} readOnly aria-label="Read only" />
          </Col>
          <Col width="10rem">
            <NumberField defaultValue={12} disabled aria-label="Disabled" />
          </Col>
          <Col width="10rem">
            <NumberField defaultValue={12} state="error" aria-label="Error" />
          </Col>
        </Row>
      ),
    },
    {
      title: 'Wheel input',
      note: 'Wheel stepping is opt-in and requires focus. Ctrl-wheel stays available for zoom; the other field keeps normal page scrolling.',
      Demo: () => (
        <Row>
          <Col width="10rem">
            <NumberField
              defaultValue={12}
              step={5}
              allowWheelScrub
              aria-label="Wheel stepping"
            />
          </Col>
          <Col width="10rem">
            <NumberField
              defaultValue={12}
              step={5}
              aria-label="Page scrolling"
            />
          </Col>
        </Row>
      ),
    },
    {
      title: 'Changes and commits',
      note: 'The read-only field follows committed edits. Text entry may exceed 0–100; stepping still clamps to those bounds. A held stepper commits on release.',
      Demo: Committed,
    },
  ],
}
export default extension
