import {
  Badge,
  Card,
  Progress,
  SectionLabel,
  Step,
  StepList,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useEffect, useState } from 'react'

import {
  Page,
  Section,
  Specimen,
  Stack,
  TokenTable,
} from '../components/gallery'
import { Col } from '../components/stage'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof Progress> = {
  title: 'Components/Progress',
  component: Progress,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

const WIDTH = '26rem'

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Progress"
      description="A neutral track with a tinted fill. The track is a token because it stays neutral whatever the fill is tinted with; the fill's width is data, so it is the one thing set inline. Width is also the only thing that animates, and it animates because the value changed — the component holds no timer of its own."
    >
      <Section
        title="Values"
        description="0 to 100, clamped. A value outside the range is a bug in the caller rather than a state to design for, so it is pulled back to the end of the track instead of overflowing it. Omit `value` when the work has no measurable end: the bar sweeps instead of filling, and reports no value to assistive tech."
      >
        <Stack>
          {[0, 12, 48, 76, 100].map(value => (
            <Specimen key={value} label={`${value}%`}>
              <div style={{ flex: 1 }}>
                <Progress value={value} />
              </div>
            </Specimen>
          ))}
          <Specimen label="no value">
            <div style={{ flex: 1 }}>
              <Progress />
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Sizes"
        description="Only the track's thickness and its radius change. `small` is the default, because a progress bar is usually reporting on something else on the page rather than being the page."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <div style={{ flex: 1 }}>
                <Progress size={size} value={64} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The fill moves with the ramp; the track does not, because it is the same empty channel whatever is filling it. `danger` on a bar reads as a job in trouble rather than as a job about to end."
      >
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <div style={{ flex: 1 }}>
                <Progress tint={tint} value={64} />
              </div>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('progress')}
          labelOf={token => token.name.slice('progress-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}

export const Default: Story = {
  render: () => (
    <div style={{ width: WIDTH }}>
      <Progress value={43} />
    </div>
  ),
}

export const Values: Story = {
  render: () => (
    <Col width={WIDTH}>
      {[0, 12, 48, 76, 100].map(value => (
        <Progress key={value} value={value} />
      ))}
    </Col>
  ),
}

export const Indeterminate: Story = {
  render: () => (
    <Col width={WIDTH}>
      <Progress />
      <Progress tint="info" />
    </Col>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col width={WIDTH}>
      {SIZES.map(size => (
        <Progress key={size} size={size} value={64} />
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col width={WIDTH}>
      {TINTS.map(tint => (
        <Progress key={tint} tint={tint} value={64} />
      ))}
    </Col>
  ),
}

export const Live: Story = {
  render: () => {
    const [value, setValue] = useState(8)

    useEffect(() => {
      const timer = window.setInterval(() => {
        setValue(current => (current >= 100 ? 0 : current + 4))
      }, 400)
      return () => window.clearInterval(timer)
    }, [])

    return (
      <div style={{ width: WIDTH }}>
        <Progress value={value} />
      </div>
    )
  },
}

export const WithACount: Story = {
  render: () => (
    <Card style={{ width: '24rem' }}>
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}
      >
        <SectionLabel>handbook.docx</SectionLabel>
        <Badge size="small" tint="primary">
          43%
        </Badge>
      </div>
      <Progress value={43} />
      <span style={{ color: 'var(--color-content-muted)' }}>
        168 of 392 segments
      </span>
    </Card>
  ),
}

export const BesideStages: Story = {
  render: () => (
    <Card style={{ width: '24rem' }}>
      <Progress value={43} />
      <StepList>
        <Step status="done" label="Parsed the document" meta="15 pages" />
        <Step status="active" label="Translating" meta="168 / 392" />
        <Step status="idle" label="Rebuilding the layout" />
      </StepList>
    </Card>
  ),
}
