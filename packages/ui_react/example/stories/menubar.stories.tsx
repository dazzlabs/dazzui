import { KeyCap, Menubar, type MenubarMenu } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'
import { useState } from 'react'

import { Page, Section } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof Menubar> = {
  title: 'Components/Menubar',
  component: Menubar,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const MENUS: MenubarMenu[] = [
  {
    label: 'File',
    items: [
      { label: 'New project', shortcut: <KeyCap>⌘N</KeyCap> },
      { label: 'Open…', shortcut: <KeyCap>⌘O</KeyCap> },
      { label: 'Save', shortcut: <KeyCap>⌘S</KeyCap> },
      { label: 'Close window', shortcut: <KeyCap>⌘W</KeyCap> },
    ],
  },
  {
    label: 'Edit',
    items: [
      { label: 'Undo', shortcut: <KeyCap>⌘Z</KeyCap> },
      { label: 'Redo', shortcut: <KeyCap>⇧⌘Z</KeyCap>, disabled: true },
      { label: 'Find…', shortcut: <KeyCap>⌘F</KeyCap> },
    ],
  },
  {
    label: 'View',
    items: [
      { label: 'Sidebar', shortcut: <KeyCap>⌘1</KeyCap> },
      { label: 'Dock', shortcut: <KeyCap>⌘2</KeyCap> },
      { label: 'Full screen', shortcut: <KeyCap>⌃⌘F</KeyCap> },
    ],
  },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Menubar"
      description="The application menu bar: several menus that behave as one. That is the whole reason it exists rather than a row of Menus — once one of them is open, moving the pointer along the bar opens the next without a second click, and the arrow keys walk the bar as well as the rows. It is the behaviour every desktop menu bar has and none of them documents."
    >
      <Section
        title="The bar"
        description="Each menu is a Menu, so the panel, the rows and the checkmarks are the same drawing as everywhere else; the bar adds only its triggers. They are chrome rather than controls — no box at rest, and a wash only under the pointer or while their own menu is open."
      >
        <Menubar menus={MENUS} />
      </Section>

      <Section
        title="Checked rows"
        description="A menu that reports a choice reveals the leading column on every row of the group. Set `checked` on all of them, including the unchecked ones, so the labels stay on one edge."
      >
        <Menubar
          menus={[
            {
              label: 'View',
              items: [
                { label: 'Compact', checked: true },
                { label: 'Comfortable', checked: false },
              ],
            },
          ]}
        />
      </Section>
    </Page>
  ),
}

export const Bare: Story = {
  render: () => <Menubar menus={MENUS} />,
}

export const WithChecks: Story = {
  render: () => {
    const [density, setDensity] = useState('Compact')
    return (
      <Col>
        <Menubar
          menus={[
            {
              label: 'View',
              items: ['Compact', 'Comfortable'].map(item => ({
                label: item,
                checked: density === item,
                onSelect: () => setDensity(item),
              })),
            },
          ]}
        />
      </Col>
    )
  },
}

export const Disabled: Story = {
  render: () => (
    <Menubar
      menus={[
        ...MENUS,
        { label: 'Window', items: [{ label: 'Minimise' }], disabled: true },
      ]}
    />
  ),
}
