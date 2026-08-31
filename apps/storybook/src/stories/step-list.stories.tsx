import {
  Card,
  Progress,
  SectionLabel,
  Step,
  StepList,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useEffect, useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof StepList> = {
  title: 'Components/StepList',
  component: StepList,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const WIDTH = '26rem'

const STAGES = [
  { label: 'Parsed the document', meta: '15 pages' },
  { label: 'Extracted segments', meta: '392' },
  { label: 'Translating', meta: '168 / 392' },
  { label: 'Applying the glossary', meta: undefined },
  { label: 'Rebuilding the layout', meta: undefined },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="StepList"
      description="A pipeline, one line per stage. The marker carries the status — a check when a stage is finished, the spinner while it runs, an empty ring before it starts — so the label never has to say it twice. Each is a different marker rather than a different colour of the same one, which is what lets the column be read at a glance and still work in monochrome. The list holds no state: the host passes each stage its status."
    >
      <Section
        title="In flight"
        description="A finished step is success wherever the list is tinted: it is the one fixed meaning in the column, not a variation on the accent. The trailing detail — a count, a duration, a size — is pushed to the far edge and set back, so a column of them lines up without competing with the labels."
      >
        <div style={{ maxWidth: WIDTH }}>
          <StepList>
            <Step status="done" label="Parsed the document" meta="15 pages" />
            <Step status="done" label="Extracted segments" meta="392" />
            <Step status="active" label="Translating" meta="168 / 392" />
            <Step status="idle" label="Applying the glossary" />
            <Step status="idle" label="Rebuilding the layout" />
          </StepList>
        </div>
      </Section>

      <Section
        title="Finished"
        description="Every stage done, so nothing in the column is asking for attention. This is the state a list should be able to end in without changing shape — no summary row appearing, no markers moving."
      >
        <div style={{ maxWidth: WIDTH }}>
          <StepList>
            <Step status="done" label="Parsed the document" meta="15 pages" />
            <Step status="done" label="Extracted segments" meta="392" />
            <Step status="done" label="Translating" meta="392 / 392" />
          </StepList>
        </div>
      </Section>

      <Section
        title="Tints"
        description="The tint reaches the active step — its spinner and its label. A finished step stays on the success ramp whatever the list is tinted with, because “done” is a fixed meaning rather than an accent."
      >
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <div style={{ flex: 1 }}>
                <StepList>
                  <Step status="done" label="Extracted segments" tint={tint} />
                  <Step
                    status="active"
                    label="Translating"
                    meta="168 / 392"
                    tint={tint}
                  />
                  <Step status="idle" label="Rebuilding" tint={tint} />
                </StepList>
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const InFlight: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <StepList>
        <Step status="done" label="Parsed the document" meta="15 pages" />
        <Step status="done" label="Extracted segments" meta="392" />
        <Step status="active" label="Translating" meta="168 / 392" />
        <Step status="idle" label="Applying the glossary" />
        <Step status="idle" label="Rebuilding the layout" />
      </StepList>
    </div>
  ),
}

export const Statuses: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <StepList>
        <Step status="done" label="Done" />
        <Step status="active" label="Active" />
        <Step status="idle" label="Idle" />
      </StepList>
    </div>
  ),
}

export const Finished: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <StepList>
        <Step status="done" label="Parsed the document" meta="15 pages" />
        <Step status="done" label="Extracted segments" meta="392" />
        <Step status="done" label="Translating" meta="392 / 392" />
        <Step status="done" label="Applying the glossary" meta="96 hits" />
        <Step status="done" label="Rebuilding the layout" meta="15 pages" />
      </StepList>
    </div>
  ),
}

export const WithoutDetail: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <StepList>
        <Step status="done" label="Connected" />
        <Step status="active" label="Processing" />
        <Step status="idle" label="Finishing" />
      </StepList>
    </div>
  ),
}

export const TimedDetail: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <StepList>
        <Step status="done" label="Connected" meta="0.4s" />
        <Step status="done" label="Uploaded" meta="2.1s" />
        <Step status="active" label="Processing" meta="12s elapsed" />
      </StepList>
    </div>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <StepList key={tint}>
          <Step status="done" label="Extracted segments" tint={tint} />
          <Step
            status="active"
            label="Translating"
            meta="168 / 392"
            tint={tint}
          />
          <Step status="idle" label="Rebuilding" tint={tint} />
        </StepList>
      ))}
    </Col>
  ),
}

export const Advancing: Story = {
  render: () => {
    const [stage, setStage] = useState(0)

    useEffect(() => {
      const timer = window.setInterval(() => {
        setStage(current => (current >= STAGES.length ? 0 : current + 1))
      }, 1400)
      return () => window.clearInterval(timer)
    }, [])

    return (
      <Card style={{ width: WIDTH }}>
        <SectionLabel>handbook.docx</SectionLabel>
        <Progress value={(stage / STAGES.length) * 100} />
        <StepList>
          {STAGES.map((entry, index) => (
            <Step
              key={entry.label}
              label={entry.label}
              meta={index < stage ? entry.meta : undefined}
              status={
                index < stage ? 'done' : index === stage ? 'active' : 'idle'
              }
            />
          ))}
        </StepList>
      </Card>
    )
  },
}
