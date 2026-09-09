import { Avatar, AvatarGroup, Card, TINTS } from '@dazzlabs/dazzui-react'
import { Person16Regular } from '@fluentui/react-icons'
import type { Meta, StoryObj } from '@storybook/react-vite'
import type { CSSProperties } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof Avatar> = {
  title: 'Components/Avatar',
  component: Avatar,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const SIZES = ['small', 'medium', 'large'] as const

/* An inline picture rather than a URL: the gallery has to draw the same thing
   on a machine with no network, and what this story is showing is the image
   path rather than the image. */
const PORTRAIT =
  'data:image/svg+xml;utf8,' +
  encodeURIComponent(
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48">
       <rect width="48" height="48" fill="#5b5bd6"/>
       <circle cx="24" cy="19" r="8" fill="#dcd9ff"/>
       <path d="M6 48c0-10 8-16 18-16s18 6 18 16z" fill="#dcd9ff"/>
     </svg>`
  )

const PEOPLE = [
  { name: 'Ada Lovelace', tint: 'primary' },
  { name: 'Grace Hopper', tint: 'info' },
  { name: 'Alan Turing', tint: 'success' },
  { name: 'Katherine Johnson', tint: 'warning' },
  { name: 'Barbara Liskov', tint: 'danger' },
] as const

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Avatar"
      description="The picture that stands for a person or a project. What it is really for is the case where there is no picture — Base UI tracks the image's loading status, so the initials are what is drawn until the picture has actually decoded rather than flashing empty behind a broken one. It is not interactive and stays off the shared control state machine: an avatar that lit up under the pointer would read as something you can press."
    >
      <Section
        title="Sizes"
        description="Three steps off the 4px grid rather than off the control profile — an avatar is not a control, and these are the sizes a row, a header and a card actually ask for. The type is a step behind the box on every size, so two letters clear the corner on a circle."
      >
        <Stack>
          {SIZES.map(size => (
            <Specimen key={size} label={size}>
              <Avatar name="Ada Lovelace" size={size} />
              <Avatar name="DazzUI" shape="square" size={size} />
              <Avatar
                name="Ada Lovelace"
                size={size}
                fallback={<Person16Regular />}
              />
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="Circle and square"
        description="A person is a circle and a thing is a square — a project, a repository, an organisation. The square takes the container corner rather than the control corner: under Bright, where controls are pills, a control-cornered square would go round and stop being the shape that says “project”."
      >
        <Stack>
          <Specimen label="circle">
            {PEOPLE.slice(0, 3).map(person => (
              <Avatar key={person.name} name={person.name} tint={person.tint} />
            ))}
          </Specimen>
          <Specimen label="square">
            <Avatar name="DazzUI" shape="square" tint="primary" />
            <Avatar name="Acme Web" shape="square" tint="info" />
            <Avatar name="Design Tokens" shape="square" tint="neutral" />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="The fallback is the tinted recipe's resting pair — a wash of the ramp under a grade of it — read directly rather than through the state machine, since nothing here responds to a pointer."
      >
        <Specimen label="tints">
          {TINTS.map(tint => (
            <Avatar key={tint} name={tint} tint={tint} />
          ))}
        </Specimen>
      </Section>

      <Section
        title="Groups"
        description="Overlapped by a third of the box and cut out of each other by a ring in the ground colour. The ring is a variable, so a group sitting on a card re-points `--avatar-ring` to the card's own surface — the same move a second navigation column makes with `--sidebar-width`."
      >
        <Stack>
          <Specimen label="on paper">
            <AvatarGroup max={3}>
              {PEOPLE.map(person => (
                <Avatar
                  key={person.name}
                  name={person.name}
                  tint={person.tint}
                />
              ))}
            </AvatarGroup>
          </Specimen>
          <Specimen label="on a card">
            <Card style={{ padding: 'var(--spacing-4)' }}>
              <AvatarGroup
                max={3}
                style={
                  {
                    '--avatar-ring': 'var(--color-surface-muted)',
                  } as CSSProperties
                }
              >
                {PEOPLE.map(person => (
                  <Avatar
                    key={person.name}
                    name={person.name}
                    tint={person.tint}
                  />
                ))}
              </AvatarGroup>
            </Card>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Initials: Story = {
  render: () => (
    <Row>
      {PEOPLE.slice(0, 4).map(person => (
        <Avatar key={person.name} name={person.name} tint={person.tint} />
      ))}
    </Row>
  ),
}

export const Image: Story = {
  render: () => (
    <Row>
      {SIZES.map(size => (
        <Avatar key={size} size={size} name="Ada Lovelace" src={PORTRAIT} />
      ))}
    </Row>
  ),
}

export const Square: Story = {
  render: () => (
    <Row>
      <Avatar name="DazzUI" shape="square" />
      <Avatar name="Acme Web" shape="square" tint="info" />
      <Avatar name="Design Tokens" shape="square" tint="neutral" />
    </Row>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <Row key={size}>
          <Avatar name="Ada Lovelace" size={size} />
          <Avatar name="DazzUI" shape="square" size={size} />
        </Row>
      ))}
    </Col>
  ),
}

export const Tints: Story = {
  render: () => (
    <Row>
      {TINTS.map(tint => (
        <Avatar key={tint} name={tint} tint={tint} />
      ))}
    </Row>
  ),
}

export const Groups: Story = {
  render: () => (
    <Col>
      {SIZES.map(size => (
        <AvatarGroup key={size} size={size} max={3}>
          {PEOPLE.map(person => (
            <Avatar
              key={person.name}
              size={size}
              name={person.name}
              tint={person.tint}
            />
          ))}
        </AvatarGroup>
      ))}
    </Col>
  ),
}
