import {
  Avatar,
  Badge,
  Button,
  PreviewCard,
  Tooltip,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col, Row } from '../src/components/stage'

const meta: Meta<typeof PreviewCard> = {
  title: 'Components/PreviewCard',
  component: PreviewCard,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const LINK = {
  color: 'var(--control-color-plain-content-normal)',
  textDecoration: 'underline',
  textUnderlineOffset: '2px',
  cursor: 'pointer',
}

function Person() {
  return (
    <Col gap="var(--spacing-3)">
      <Row>
        <Avatar name="Ada Lovelace" size="large" />
        <Col gap="var(--spacing-0-5)">
          <strong>Ada Lovelace</strong>
          <span style={{ color: 'var(--color-content-subtle)' }}>@ada</span>
        </Col>
      </Row>
      <span style={{ color: 'var(--color-content-secondary)' }}>
        Writes the notes that turn a machine into a program. 42 commits this
        month.
      </span>
    </Col>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="PreviewCard"
      description="What is behind a link, shown without following it — a repository, a person, an issue. It is a Tooltip that holds a card rather than a label, and the difference is what may go in it: a tooltip must carry nothing a reader needs, because a pointer never reaches it; a preview card stays open while the pointer travels, so it can hold a link of its own. It is never the only way to reach what it shows."
    >
      <Section
        title="A link, previewed"
        description="It opens on hover and on focus, after a rest — never on a click, which belongs to the link. The card wears the same sheet a Popover does, with a ceiling on its width: a popover is as wide as what a caller put in it, a preview card is filled from whatever it previews."
      >
        <span>
          Reviewed by{' '}
          <PreviewCard trigger={<a style={LINK}>@ada</a>}>
            <Person />
          </PreviewCard>{' '}
          three days ago.
        </span>
      </Section>

      <Section
        title="Against a Tooltip"
        description="If the panel has anything in it a reader might want to reach — a link, a button, a value to select — it is a preview card. If it is a phrase naming what an icon does, it is a tooltip, and it must stay one."
      >
        <Stack>
          <Specimen label="tooltip">
            <Tooltip label="Open in a new window">
              <Button size="small">Open</Button>
            </Tooltip>
          </Specimen>
          <Specimen label="preview card">
            <PreviewCard trigger={<Button size="small">dazzlabs/dazzui</Button>}>
              <Col gap="var(--spacing-2)">
                <Row>
                  <Avatar name="DazzUI" shape="square" size="small" />
                  <strong>dazzlabs/dazzui</strong>
                  <Badge size="small" tint="neutral">
                    public
                  </Badge>
                </Row>
                <span style={{ color: 'var(--color-content-secondary)' }}>
                  The design system, its tokens, and four platform
                  implementations of it.
                </span>
              </Col>
            </PreviewCard>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Sides"
        description="Positioned and flipped by Base UI, against the same collision rules every popup here follows."
      >
        <Stack>
          {(['top', 'bottom', 'left', 'right'] as const).map(side => (
            <Specimen key={side} label={side}>
              <PreviewCard
                side={side}
                trigger={<Button size="small">{side}</Button>}
              >
                <Person />
              </PreviewCard>
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const OnALink: Story = {
  render: () => (
    <span>
      Reviewed by{' '}
      <PreviewCard trigger={<a style={LINK}>@ada</a>}>
        <Person />
      </PreviewCard>{' '}
      three days ago.
    </span>
  ),
}

export const ARepository: Story = {
  render: () => (
    <PreviewCard trigger={<a style={LINK}>dazzlabs/dazzui</a>}>
      <Col gap="var(--spacing-2)">
        <Row>
          <Avatar name="DazzUI" shape="square" size="small" />
          <strong>dazzlabs/dazzui</strong>
          <Badge size="small" tint="neutral">
            public
          </Badge>
        </Row>
        <span style={{ color: 'var(--color-content-secondary)' }}>
          The design system, its tokens, and four platform implementations of
          it.
        </span>
      </Col>
    </PreviewCard>
  ),
}

export const Sides: Story = {
  render: () => (
    <Row>
      {(['top', 'bottom', 'left', 'right'] as const).map(side => (
        <PreviewCard
          key={side}
          side={side}
          trigger={<Button size="small">{side}</Button>}
        >
          <Person />
        </PreviewCard>
      ))}
    </Row>
  ),
}

export const Instant: Story = {
  render: () => (
    <PreviewCard delay={0} trigger={<a style={LINK}>@ada</a>}>
      <Person />
    </PreviewCard>
  ),
}
