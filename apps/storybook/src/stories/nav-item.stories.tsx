import {
  Book20Regular as Book,
  Document20Regular as Doc,
  Settings20Regular as Gear,
} from '@fluentui/react-icons'
import { Badge, Icon, NavAction, NavItem, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { type ReactNode, useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col, Row } from '../components/stage'

const meta: Meta<typeof NavItem> = {
  title: 'Components/NavItem',
  component: NavItem,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** Rows are full-width, so they need a column to sit in to be read fairly. */
function Column({
  children,
  width = '13rem',
}: {
  children: ReactNode
  width?: string
}) {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: 'var(--spacing-1)',
        width,
        padding: 'var(--spacing-2)',
        /* The container corner, not the control corner: Bright makes the
           control steps pills, and a column of rows is not a control. */
        borderRadius: 'var(--radius-large)',
        background: 'var(--color-surface-muted)',
      }}
    >
      {children}
    </div>
  )
}

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="NavItem"
      description="One row of a navigation column. The same component fills a sidebar and a rail — they differ by a size, not by a component, because a rail row is a sidebar row at a tighter scale. There are only two sizes for that reason: a row larger than the sidebar's would stop being a list."
    >
      <Section
        title="States"
        description="Hover an idle row: the wash comes off the tinted ramp, which is the opposite of what a menu item or a table row gets. The difference is what selection looks like — a nav row selects with the filled recipe, so a tinted hover previews it from well below. A table row selects at a much lighter shade, and a tinted hover would outweigh it."
      >
        <Stack>
          <Specimen label="medium">
            <Column width="11rem">
              <NavItem active icon={<Icon icon={Doc} size={14} />}>
                Current page
              </NavItem>
              <NavItem icon={<Icon icon={Doc} size={14} />}>Idle</NavItem>
              <NavItem disabled icon={<Icon icon={Doc} size={14} />}>
                Disabled
              </NavItem>
            </Column>
          </Specimen>
          <Specimen label="small">
            <Column width="11rem">
              <NavItem active size="small">
                Current page
              </NavItem>
              <NavItem size="small">Idle</NavItem>
              <NavItem disabled size="small">
                Disabled
              </NavItem>
            </Column>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Content"
        description="The leading glyph follows the row's colour, so it tints with selection rather than being coloured a second time. Icons on some rows and not others leaves the labels on two edges — give a column all of them or none. The row is a flex line, so a trailing count needs no prop."
      >
        <Stack>
          <Specimen label="icons">
            <Column>
              <NavItem active icon={<Icon icon={Doc} size={14} />}>
                Documents
              </NavItem>
              <NavItem icon={<Icon icon={Book} size={14} />}>
                Glossaries
              </NavItem>
              <NavItem icon={<Icon icon={Gear} size={14} />}>Settings</NavItem>
            </Column>
          </Specimen>
          <Specimen label="counts">
            <Column>
              <NavItem active>
                Needs review
                <Badge
                  size="small"
                  tint="neutral"
                  style={{ marginInlineStart: 'auto' }}
                >
                  24
                </Badge>
              </NavItem>
              <NavItem>
                Done
                <Badge
                  size="small"
                  tint="neutral"
                  style={{ marginInlineStart: 'auto' }}
                >
                  168
                </Badge>
              </NavItem>
            </Column>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="NavAction"
        description="The action pinned to the foot of a column — “New”, “Add a provider”. Shaped like a row so the column still reads as one list, but printed in the ramp rather than filled with it, because it adds rather than selects."
      >
        <Column>
          <NavItem active size="small">
            Product terms
          </NavItem>
          <NavItem size="small">Legal</NavItem>
          <NavAction size="small">＋ New glossary</NavAction>
        </Column>
      </Section>

      <Section
        title="Tints"
        description="Selection is the filled recipe, so the ramp is what a selected row is painted with. A column normally keeps one tint throughout: a list where each row selects in a different colour is a legend, not a navigation."
      >
        <Stack>
          <Specimen label="selected">
            {TINTS.map(tint => (
              <NavItem key={tint} active tint={tint} style={{ width: 'auto' }}>
                {tint}
              </NavItem>
            ))}
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Idle: Story = {
  render: () => (
    <Column>
      <NavItem>Documents</NavItem>
    </Column>
  ),
}

export const Active: Story = {
  render: () => (
    <Column>
      <NavItem active>Documents</NavItem>
    </Column>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Column>
      <NavItem disabled>Documents</NavItem>
    </Column>
  ),
}

export const States: Story = {
  render: () => (
    <Column>
      <NavItem active>Current page</NavItem>
      <NavItem>Idle</NavItem>
      <NavItem disabled>Disabled</NavItem>
    </Column>
  ),
}

export const Sizes: Story = {
  render: () => (
    <Row>
      <Column width="11rem">
        <NavItem active icon={<Icon icon={Doc} size={14} />}>
          handbook.docx
        </NavItem>
        <NavItem icon={<Icon icon={Doc} size={14} />}>contract.pdf</NavItem>
      </Column>
      <Column width="11rem">
        <NavItem active icon={<Icon icon={Doc} size={12} />} size="small">
          handbook.docx
        </NavItem>
        <NavItem icon={<Icon icon={Doc} size={12} />} size="small">
          contract.pdf
        </NavItem>
      </Column>
    </Row>
  ),
}

export const WithAnIcon: Story = {
  render: () => (
    <Column>
      <NavItem active icon={<Icon icon={Doc} size={14} />}>
        Documents
      </NavItem>
      <NavItem icon={<Icon icon={Book} size={14} />}>Glossaries</NavItem>
      <NavItem icon={<Icon icon={Gear} size={14} />}>Settings</NavItem>
    </Column>
  ),
}

export const WithACount: Story = {
  render: () => (
    <Column>
      <NavItem active>
        Needs review
        <Badge
          size="small"
          tint="neutral"
          style={{ marginInlineStart: 'auto' }}
        >
          24
        </Badge>
      </NavItem>
      <NavItem>
        In progress
        <Badge
          size="small"
          tint="neutral"
          style={{ marginInlineStart: 'auto' }}
        >
          6
        </Badge>
      </NavItem>
      <NavItem>
        Done
        <Badge
          size="small"
          tint="neutral"
          style={{ marginInlineStart: 'auto' }}
        >
          168
        </Badge>
      </NavItem>
    </Column>
  ),
}

export const Action: Story = {
  render: () => (
    <Column>
      <NavItem active size="small">
        Product terms
      </NavItem>
      <NavItem size="small">Legal</NavItem>
      <NavAction size="small">＋ New glossary</NavAction>
    </Column>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      <Row>
        {TINTS.map(tint => (
          <NavItem key={tint} active tint={tint} style={{ width: 'auto' }}>
            {tint}
          </NavItem>
        ))}
      </Row>
      <Row>
        {TINTS.map(tint => (
          <NavItem key={tint} tint={tint} style={{ width: 'auto' }}>
            {tint}
          </NavItem>
        ))}
      </Row>
    </Col>
  ),
}

export const Selecting: Story = {
  render: () => {
    const [pane, setPane] = useState('Documents')
    const panes = [
      { label: 'Documents', icon: Doc },
      { label: 'Glossaries', icon: Book },
      { label: 'Settings', icon: Gear },
    ]

    return (
      <Column>
        {panes.map(entry => (
          <NavItem
            key={entry.label}
            active={pane === entry.label}
            icon={<Icon icon={entry.icon} size={14} />}
            onClick={() => setPane(entry.label)}
          >
            {entry.label}
          </NavItem>
        ))}
      </Column>
    )
  },
}
