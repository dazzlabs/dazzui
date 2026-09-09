import { Badge, Collapsible } from '@dazzlabs/dazzui-react'
import { Folder16Regular, Wrench16Regular } from '@fluentui/react-icons'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { useState } from 'react'

import { Page, Section, Specimen, Stack } from '../components/gallery'
import { Col } from '../components/stage'

const meta: Meta<typeof Collapsible> = {
  title: 'Components/Collapsible',
  component: Collapsible,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const LOG = `$ pnpm test --filter core
  ✓ resolves a relative import  (4ms)
  ✓ resolves a bare specifier   (2ms)
  ✓ falls back to the index     (3ms)`

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Collapsible"
      description="One thing that opens. The header row is the affordance and the panel is what it discloses — a tool call's output, a stack trace, the rest of a long description. It is the smallest of the three ways this system hides something: a collapsible keeps its subject in place and only folds it, where a Popover moves it off the page and a Dialog stops the page."
    >
      <Section
        title="Shut and open"
        description="The chevron is the only moving part: one glyph turned rather than two swapped, so the turn is the animation. The panel animates its own height, which is the one thing CSS cannot do alone — `auto` is not a length to interpolate from, so Base UI measures the content and publishes it."
      >
        <Stack>
          <Specimen label="shut">
            <div style={{ width: '22rem' }}>
              <Collapsible title="Environment">
                Node 24.20, pnpm 10.4, macOS 15.5
              </Collapsible>
            </div>
          </Specimen>
          <Specimen label="open">
            <div style={{ width: '22rem' }}>
              <Collapsible defaultOpen title="Environment">
                Node 24.20, pnpm 10.4, macOS 15.5
              </Collapsible>
            </div>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="The header row"
        description="A glyph before the title and an aside after it. The aside is what makes a shut row worth reading — a count, a duration, a status — so the reader can decide whether to open it at all. It steps back to the faint ink and never takes the row's weight."
      >
        <div style={{ width: '26rem' }}>
          <Collapsible
            title="Ran 3 tests"
            icon={<Wrench16Regular />}
            meta={<Badge tint="success">passed</Badge>}
          >
            <pre style={{ margin: 0, fontFamily: 'var(--base-font-code)' }}>
              {LOG}
            </pre>
          </Collapsible>
        </div>
      </Section>

      <Section
        title="Against an Accordion"
        description="Several of these that know about each other are an Accordion — the arrow keys move between its headers, and by default opening one shuts the last. A run of separate Collapsibles does neither, which is right when the sections are unrelated and wrong when they are one list."
      >
        <div style={{ width: '26rem' }}>
          <Collapsible title="Advanced" icon={<Folder16Regular />}>
            Nothing here depends on anything else being shut.
          </Collapsible>
        </div>
      </Section>
    </Page>
  ),
}

export const Shut: Story = {
  render: () => (
    <div style={{ width: '22rem' }}>
      <Collapsible title="Environment">
        Node 24.20, pnpm 10.4, macOS 15.5
      </Collapsible>
    </div>
  ),
}

export const Open: Story = {
  render: () => (
    <div style={{ width: '22rem' }}>
      <Collapsible defaultOpen title="Environment">
        Node 24.20, pnpm 10.4, macOS 15.5
      </Collapsible>
    </div>
  ),
}

export const WithMeta: Story = {
  render: () => (
    <div style={{ width: '26rem' }}>
      <Collapsible
        defaultOpen
        title="Ran 3 tests"
        icon={<Wrench16Regular />}
        meta={<Badge tint="success">passed</Badge>}
      >
        <pre style={{ margin: 0, fontFamily: 'var(--base-font-code)' }}>
          {LOG}
        </pre>
      </Collapsible>
    </div>
  ),
}

export const Disabled: Story = {
  render: () => (
    <div style={{ width: '22rem' }}>
      <Collapsible disabled title="Environment">
        Node 24.20, pnpm 10.4, macOS 15.5
      </Collapsible>
    </div>
  ),
}

export const Interactive: Story = {
  render: () => {
    const [open, setOpen] = useState(false)
    return (
      <Col width="24rem">
        <Collapsible
          open={open}
          onOpenChange={setOpen}
          title="Request headers"
          meta={open ? 'hide' : 'show'}
        >
          <pre style={{ margin: 0, fontFamily: 'var(--base-font-code)' }}>
            {'accept: application/json\nauthorization: Bearer ****'}
          </pre>
        </Collapsible>
      </Col>
    )
  },
}
