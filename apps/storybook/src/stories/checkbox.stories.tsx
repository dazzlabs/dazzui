import {
  Button,
  Card,
  Checkbox,
  Divider,
  SectionLabel,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { Col, Row } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Checkbox> = {
  title: 'Components/Checkbox',
  component: Checkbox,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

function Checklist() {
  const [parts, setParts] = useState({ text: true, notes: true, images: false })

  return (
    <Col gap="var(--spacing-2)">
      <Checkbox
        checked={parts.text}
        note="392 segments"
        onCheckedChange={text => setParts({ ...parts, text })}
      >
        Translated text
      </Checkbox>
      <Checkbox
        checked={parts.notes}
        note="24 comments"
        onCheckedChange={notes => setParts({ ...parts, notes })}
      >
        Reviewer notes
      </Checkbox>
      <Checkbox
        checked={parts.images}
        note="+2.1 MB"
        onCheckedChange={images => setParts({ ...parts, images })}
      >
        Embedded images
      </Checkbox>
      <Checkbox checked={false} disabled>
        Revision history
      </Checkbox>
    </Col>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Checkbox"
      description="A box and a label on one row. The box defines the row's height, so a stacked checklist sits at the density the boxes set — and the label carries the selection twice over, resting one ink step back and taking the full weight only when its box is checked. A checkbox states an intention a form will act on later, which is what separates it from a Switch."
    >
      <Section
        title="A checklist"
        description="A filled square when set, a neutral outline when not: an empty box is an outline, not a quiet accent. A note is a de-emphasised aside inside the label — a count, a size — so it wraps with the text and stays part of the hit target."
      >
        <Checklist />
      </Section>

      <Section
        title="Sizes"
        description="The square and the row grow together, so a list of them keeps one rhythm whatever size it is set at. The box is the one part that is its own token: a control height would swallow a square that small."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Checkbox checked size={size}>
                Set
              </Checkbox>
              <Checkbox checked={false} size={size}>
                Unset
              </Checkbox>
              <Checkbox checked disabled size={size}>
                Disabled
              </Checkbox>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The set state is the filled recipe, so it moves with the ramp. The unset box stays neutral on every tint — an outline is the absence of a choice, and tinting it would imply one."
      >
        <Stack>
          <Specimen label="checked">
            {TINTS.map(tint => (
              <Checkbox key={tint} checked tint={tint}>
                {tint}
              </Checkbox>
            ))}
          </Specimen>
          <Specimen label="unchecked">
            {TINTS.map(tint => (
              <Checkbox key={tint} checked={false} tint={tint}>
                {tint}
              </Checkbox>
            ))}
          </Specimen>
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('checkbox')}
          labelOf={token => token.name.slice('checkbox-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const Checked: Story = {
  render: () => <Checkbox checked>Translated text</Checkbox>,
}

export const Unchecked: Story = {
  render: () => <Checkbox checked={false}>Translated text</Checkbox>,
}

export const Disabled: Story = {
  render: () => (
    <Col gap="var(--spacing-2)">
      <Checkbox checked disabled>
        Revision history
      </Checkbox>
      <Checkbox checked={false} disabled>
        Revision history
      </Checkbox>
    </Col>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col gap="var(--spacing-2)">
      {SIZES.map(size => (
        <Row key={size}>
          <Checkbox checked size={size}>
            Set
          </Checkbox>
          <Checkbox checked={false} size={size}>
            Unset
          </Checkbox>
        </Row>
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col gap="var(--spacing-2)">
      <Row>
        {TINTS.map(tint => (
          <Checkbox key={tint} checked tint={tint}>
            {tint}
          </Checkbox>
        ))}
      </Row>
      <Row>
        {TINTS.map(tint => (
          <Checkbox key={tint} checked={false} tint={tint}>
            {tint}
          </Checkbox>
        ))}
      </Row>
    </Col>
  ),
}

export const WithANote: Story = {
  render: () => (
    <Col gap="var(--spacing-2)">
      <Checkbox checked note="392 segments">
        Translated text
      </Checkbox>
      <Checkbox checked note="+2.1 MB">
        Embedded images
      </Checkbox>
      <Checkbox checked={false} note="needs a paid plan">
        Revision history
      </Checkbox>
    </Col>
  ),
}

export const AChecklist: Story = {
  render: () => <Checklist />,
}

export const InAForm: Story = {
  render: () => {
    const [scope, setScope] = useState({
      project: true,
      shared: false,
      archive: false,
    })
    const chosen = Object.values(scope).filter(Boolean).length

    return (
      <Card style={{ width: '22rem' }}>
        <SectionLabel>Search in</SectionLabel>
        <Col gap="var(--spacing-2)">
          <Checkbox
            checked={scope.project}
            onCheckedChange={project => setScope({ ...scope, project })}
          >
            This project
          </Checkbox>
          <Checkbox
            checked={scope.shared}
            onCheckedChange={shared => setScope({ ...scope, shared })}
          >
            Shared glossaries
          </Checkbox>
          <Checkbox
            checked={scope.archive}
            note="slower"
            onCheckedChange={archive => setScope({ ...scope, archive })}
          >
            The archive
          </Checkbox>
        </Col>
        <Divider />
        <Button disabled={chosen === 0} fullWidth variant="filled">
          {chosen === 0 ? 'Pick a scope' : `Search ${chosen} sources`}
        </Button>
      </Card>
    )
  },
}
