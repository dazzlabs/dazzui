import { Badge, Tag, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Tag> = {
  title: 'Components/Tag',
  component: Tag,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const
const VARIANTS = ['filled', 'tinted', 'outlined', 'raised'] as const

const LABELS = ['bug', 'good first issue', 'needs design', 'blocked'] as const

function Filters() {
  const [on, setOn] = useState<string[]>(['bug'])
  return (
    <Row gap="var(--spacing-1-5)">
      {LABELS.map(label => (
        <Tag
          key={label}
          selected={on.includes(label)}
          variant="outlined"
          onSelectedChange={next =>
            setOn(current =>
              next ? [...current, label] : current.filter(one => one !== label)
            )
          }
        >
          {label}
        </Tag>
      ))}
    </Row>
  )
}

function RemovableTags() {
  const [labels, setLabels] = useState<string[]>([...LABELS])
  return (
    <Row gap="var(--spacing-1-5)">
      {labels.map(label => (
        <Tag
          key={label}
          removeLabel={`Remove ${label}`}
          onRemove={() =>
            setLabels(current => current.filter(one => one !== label))
          }
        >
          {label}
        </Tag>
      ))}
    </Row>
  )
}

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Tag"
      description="A Badge you can act on. The two are one drawing and two components rather than one component with an `interactive` prop, because the difference is not cosmetic: a badge reads its recipes at rest and stays out of the shared state machine, since a label that moves under the pointer reads as something you can press. A tag is something you can press — and taking the state machine is the whole reason it exists."
    >
      <Section
        title="Beside a Badge"
        description="They share the capsule — the same box at the same three densities, written once in badge.css — and nothing else. A tag that is neither selectable nor removable should be a Badge."
      >
        <Stack>
          <Specimen label="Badge">
            <Badge>v1.7.0</Badge>
          </Specimen>
          <Specimen label="Tag, selectable">
            <Tag selected variant="outlined">
              bug
            </Tag>
          </Specimen>
          <Specimen label="Tag, removable">
            <Tag onRemove={() => {}}>needs design</Tag>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Selected"
        description="Drawn twice over: the tinted fill the component swaps the variant to, and the tint's edge. One alone is not enough — a tinted tag that was already tinted would look untouched, and an edge alone is a pixel to spot in a row of forty."
      >
        <Filters />
      </Section>

      <Section
        title="Dismissing"
        description="The dismiss button is its own control rather than a second job for the label: a chip that removes itself when clicked has no way to be selected, and one that does both from the same target removes things by accident. It is faint at rest and only resolves under the pointer — an ✕ at full strength on every chip turns a list of tags into a list of buttons."
      >
        <RemovableTags />
      </Section>

      <Section
        title="Sizes and variants"
        description="Both are the Badge's, because the capsule is the Badge's. What is added is the resting, hover, pressed and disabled states the recipes already carry."
      >
        <Stack>
          {VARIANTS.map(variant => (
            <Specimen key={variant} label={variant}>
              <Tag
                selected={false}
                variant={variant}
                onSelectedChange={() => {}}
              >
                label
              </Tag>
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const Selectable: Story = {
  render: () => <Filters />,
}

export const Removable: Story = {
  render: () => <RemovableTags />,
}

export const Sizes: Story = {
  render: () => (
    <Row>
      {SIZES.map(size => (
        <Tag key={size} size={size} onRemove={() => {}}>
          {size}
        </Tag>
      ))}
    </Row>
  ),
}

export const Variants: Story = {
  render: () => (
    <Row>
      {VARIANTS.map(variant => (
        <Tag key={variant} variant={variant}>
          {variant}
        </Tag>
      ))}
    </Row>
  ),
}

export const Tints: Story = {
  render: () => (
    <Row>
      {TINTS.map(tint => (
        <Tag key={tint} tint={tint} onRemove={() => {}}>
          {tint}
        </Tag>
      ))}
    </Row>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Row>
      <Tag disabled selected={false} onSelectedChange={() => {}}>
        unavailable
      </Tag>
      <Tag disabled onRemove={() => {}}>
        pinned
      </Tag>
    </Row>
  ),
}

export const AFilterBar: Story = {
  render: () => (
    <Col width="26rem">
      <Filters />
      <RemovableTags />
    </Col>
  ),
}
