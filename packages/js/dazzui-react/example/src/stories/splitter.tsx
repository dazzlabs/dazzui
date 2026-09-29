import { Splitter, SplitterPanel } from '@dazzlabs/dazzui-react'
import type { ReactNode } from 'react'

import { Col, Frame, Text } from '../components/stage'
import type { Story } from '../lib/story'

/** A pane's name, in the middle of it. */
function Pane({ children }: { children: ReactNode }) {
  return (
    <Col width="100%" align="center">
      <Text tone="secondary">{children}</Text>
    </Col>
  )
}

const story: Story = {
  id: 'splitter',
  name: 'Splitter',
  summary: 'Panes that share a window, and the seam between them.',
  Preview: () => (
    <Frame width="100%" height="9rem">
      <Splitter>
        <SplitterPanel defaultSize={35}>
          <Pane>List</Pane>
        </SplitterPanel>
        <SplitterPanel defaultSize={65}>
          <Pane>Detail</Pane>
        </SplitterPanel>
      </Splitter>
    </Frame>
  ),
  examples: [
    {
      title: 'Three panes',
      note: 'Drag a seam, or focus it and use the arrow keys. Double-click evens the pair.',
      Demo: () => (
        <Frame>
          <Splitter>
            <SplitterPanel defaultSize={24} minSize={15} maxSize={40}>
              <Pane>Navigator</Pane>
            </SplitterPanel>
            <SplitterPanel defaultSize={52} minSize={30}>
              <Pane>Editor</Pane>
            </SplitterPanel>
            <SplitterPanel defaultSize={24} minSize={15} maxSize={40}>
              <Pane>Inspector</Pane>
            </SplitterPanel>
          </Splitter>
        </Frame>
      ),
    },
    {
      title: 'Stacked',
      Demo: () => (
        <Frame>
          <Splitter orientation="vertical">
            <SplitterPanel defaultSize={65}>
              <Pane>Editor</Pane>
            </SplitterPanel>
            <SplitterPanel defaultSize={35}>
              <Pane>Terminal</Pane>
            </SplitterPanel>
          </Splitter>
        </Frame>
      ),
    },
  ],
}

export default story
