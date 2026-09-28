import { SectionLabel, Splitter, SplitterPanel } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import type { ReactNode } from 'react'

import { Page, Section } from '../src/components/gallery'
import { Frame } from '../src/components/stage'

const meta: Meta<typeof Splitter> = {
  title: 'Components/Splitter',
  component: Splitter,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** A region with a name in it, so a seam can be seen to move something. */
function Region({ label, children }: { label: string; children?: ReactNode }) {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        gap: 'var(--spacing-2)',
        height: '100%',
        padding: 'var(--spacing-3)',
      }}
    >
      <SectionLabel>{label}</SectionLabel>
      {children}
    </div>
  )
}

function Window() {
  return (
    <Splitter>
      <SplitterPanel defaultSize={24} minSize={15} maxSize={40}>
        <Region label="Files" />
      </SplitterPanel>
      <SplitterPanel defaultSize={52} minSize={30}>
        <Region label="Document" />
      </SplitterPanel>
      <SplitterPanel defaultSize={24} minSize={15} maxSize={40}>
        <Region label="Inspector" />
      </SplitterPanel>
    </Splitter>
  )
}

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Splitter"
      description="Two or more regions the reader can re-proportion. A desktop window is mostly this — a column beside a document beside an inspector — and which of the three deserves the room is the reader's decision rather than the designer's."
    >
      <Section
        title="Beside a Sidebar"
        description="Where a column has one right width and the reader only opens or shuts it, that is Sidebar instead: it carries its own width token and its own rail, and drawing it with a splitter would offer a choice the design has already made. Reach for this where there is no right answer — where the reader's screen, and what they are doing on it, decide."
      >
        <Frame height="16rem" width="38rem">
          <Window />
        </Frame>
      </Section>

      <Section
        title="The seam"
        description="The handle is drawn from the space between two panels rather than added to it: the hit target is a whole step of the scale, so grabbing a seam does not demand pixel accuracy, while the rule itself stays the hairline every other separator in the system is. Under the pointer it takes the accent and thickens — by a scale rather than a width, so nothing reflows on the way to a drag."
      >
        <Frame height="10rem" width="38rem">
          <Splitter>
            <SplitterPanel defaultSize={50}>
              <Region label="Before" />
            </SplitterPanel>
            <SplitterPanel defaultSize={50}>
              <Region label="After" />
            </SplitterPanel>
          </Splitter>
        </Frame>
      </Section>

      <Section
        title="From the keyboard"
        description="Each seam is a `separator` in the tab order with a value it reports: the arrow keys move it one percent, shift ten, and Home and End push it to the panel's own floor and ceiling. Double-clicking a seam splits that pair evenly, which is what a reader means by the gesture."
      >
        <Frame height="10rem" width="38rem">
          <Splitter orientation="vertical">
            <SplitterPanel defaultSize={40} minSize={20}>
              <Region label="Editor" />
            </SplitterPanel>
            <SplitterPanel defaultSize={60} minSize={20}>
              <Region label="Output" />
            </SplitterPanel>
          </Splitter>
        </Frame>
      </Section>
    </Page>
  ),
}

export const Horizontal: Story = {
  render: () => (
    <Frame height="12rem" width="34rem">
      <Splitter>
        <SplitterPanel defaultSize={50}>
          <Region label="Before" />
        </SplitterPanel>
        <SplitterPanel defaultSize={50}>
          <Region label="After" />
        </SplitterPanel>
      </Splitter>
    </Frame>
  ),
}

export const Vertical: Story = {
  render: () => (
    <Frame height="14rem" width="30rem">
      <Splitter orientation="vertical">
        <SplitterPanel defaultSize={40} minSize={20}>
          <Region label="Editor" />
        </SplitterPanel>
        <SplitterPanel defaultSize={60} minSize={20}>
          <Region label="Output" />
        </SplitterPanel>
      </Splitter>
    </Frame>
  ),
}

export const ThreePanels: Story = {
  render: () => (
    <Frame height="14rem" width="38rem">
      <Window />
    </Frame>
  ),
}

export const Constrained: Story = {
  render: () => (
    <Frame height="12rem" width="34rem">
      <Splitter>
        <SplitterPanel defaultSize={30} minSize={25} maxSize={45}>
          <Region label="25–45%" />
        </SplitterPanel>
        <SplitterPanel defaultSize={70}>
          <Region label="the rest" />
        </SplitterPanel>
      </Splitter>
    </Frame>
  ),
}

export const Nested: Story = {
  render: () => (
    <Frame height="16rem" width="38rem">
      <Splitter>
        <SplitterPanel defaultSize={30} minSize={20}>
          <Region label="Files" />
        </SplitterPanel>
        <SplitterPanel defaultSize={70}>
          <Splitter orientation="vertical">
            <SplitterPanel defaultSize={65}>
              <Region label="Editor" />
            </SplitterPanel>
            <SplitterPanel defaultSize={35}>
              <Region label="Terminal" />
            </SplitterPanel>
          </Splitter>
        </SplitterPanel>
      </Splitter>
    </Frame>
  ),
}
