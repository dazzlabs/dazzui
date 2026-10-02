import {
  Book20Regular,
  Document20Regular,
  PanelLeftExpand20Regular,
  Pin20Regular,
  Settings20Regular,
  Share20Regular,
} from '@fluentui/react-icons'
import {
  Button,
  Icon,
  IconButton,
  SearchField,
  SegmentedControl,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
  Toggle,
  WindowBody,
  WindowContent,
  WindowFooter,
  WindowFrame,
  WindowMain,
  WindowTitlebar,
  type WindowFrameControls,
  type WindowFramePlatform,
} from '@dazzlabs/dazzui'
import { type ReactNode, useState } from 'react'

import { Col, Row, Text } from '../../components/stage'
import type { Story } from '../../lib/story'

/*
 * React only: no other platform draws a window inside a window, so this
 * story sits outside the directory codegen compiles, and may use what the
 * portable subset forbids.
 */

const PLATFORMS: { platform: WindowFramePlatform; title: string }[] = [
  { platform: 'macos', title: 'macOS 27' },
  { platform: 'macos15', title: 'macOS 15' },
  { platform: 'windows', title: 'Windows 11' },
  { platform: 'gnome', title: 'GNOME' },
  { platform: 'ubuntu', title: 'Ubuntu' },
  { platform: 'kde', title: 'KDE Plasma' },
  { platform: 'omarchy', title: 'Omarchy' },
]

/** The window's content, at the inset a dialog body has. */
function Pane({ children }: { children: ReactNode }) {
  return (
    <div style={{ padding: 'var(--spacing-5)', paddingBlockStart: 0 }}>
      {children}
    </div>
  )
}

function Update() {
  return (
    <Pane>
      <Col gap="3">
        <Col gap="1">
          <Text strong>A new version is available</Text>
          <Text tone="secondary">Version 2.4 is ready to install.</Text>
        </Col>
        <Row justify="between">
          <Button size="small" variant="normal">
            Later
          </Button>
          <Button size="small" variant="filled">
            Install
          </Button>
        </Row>
      </Col>
    </Pane>
  )
}

/** The three states a control can be in, side by side on two platforms. */
const STATES: { title: string; controls: WindowFrameControls }[] = [
  {
    title: 'Enabled',
    controls: { close: 'enabled', minimize: 'enabled', maximize: 'enabled' },
  },
  {
    title: 'Disabled',
    controls: { close: 'disabled', minimize: 'disabled', maximize: 'disabled' },
  },
  {
    title: 'Hidden',
    controls: { close: 'hidden', minimize: 'hidden', maximize: 'hidden' },
  },
]

function ControlStates({ platform }: { platform: WindowFramePlatform }) {
  return (
    <Row gap="5" align="start">
      {STATES.map(state => (
        <WindowFrame
          key={state.title}
          platform={platform}
          title={state.title}
          width={240}
          controls={state.controls}
        >
          <Pane>
            <Text tone="secondary">{state.title}</Text>
          </Pane>
        </WindowFrame>
      ))}
    </Row>
  )
}

/** A band with something in every slot. */
function Slots({
  platform,
  title,
}: {
  platform: WindowFramePlatform
  title: ReactNode
}) {
  const [pinned, setPinned] = useState(false)
  return (
    <WindowFrame
      platform={platform}
      title={title}
      subtitle="Draft"
      width="100%"
      titlebar={{
        leading: (
          <IconButton label="Show sidebar" size="small">
            <Icon icon={PanelLeftExpand20Regular} />
          </IconButton>
        ),
        toolbar: (
          <>
            <Toggle
              size="small"
              label="Pin"
              pressed={pinned}
              onPressedChange={setPinned}
            >
              <Icon icon={Pin20Regular} />
            </Toggle>
            <Button size="small" variant="plain">
              Export
            </Button>
          </>
        ),
        trailing: (
          <IconButton label="Share" size="small">
            <Icon icon={Share20Regular} />
          </IconButton>
        ),
      }}
    >
      <Pane>
        <Text tone="secondary">
          A leading control, a title, a subtitle, a toolbar and a trailing one.
        </Text>
      </Pane>
    </WindowFrame>
  )
}

const VIEWS = [
  { value: 'list', label: 'List' },
  { value: 'board', label: 'Board' },
]

/** The band's own centre: no title drawn, a segmented control where it was. */
function Accessory({ platform }: { platform: WindowFramePlatform }) {
  const [view, setView] = useState('list')
  return (
    <WindowFrame
      platform={platform}
      title="Views"
      titleHidden
      width={420}
      titlebar={{
        accessory: (
          <SegmentedControl
            size="small"
            aria-label="View"
            items={VIEWS}
            value={view}
            onValueChange={setView}
          />
        ),
      }}
    >
      <Pane>
        <Text tone="secondary">The accessory takes the band's centre.</Text>
      </Pane>
    </WindowFrame>
  )
}

const PANES = [
  { label: 'Documents', icon: Document20Regular, count: 12 },
  { label: 'Glossaries', icon: Book20Regular },
  { label: 'Settings', icon: Settings20Regular },
]

/**
 * An application window: the sidebar runs from the window's top edge to its
 * foot, and the band over the pane beside it is the toolbar. The frame draws
 * no band of its own — only the controls, which `fullSizeContent` puts over
 * the sidebar's head — so nothing is above the sidebar but the window.
 */
function Workspace({ platform }: { platform: WindowFramePlatform }) {
  const [pane, setPane] = useState('Documents')
  const [query, setQuery] = useState('')
  return (
    <WindowFrame
      platform={platform}
      title="Workspace"
      fullSizeContent
      titlebar={false}
      width="100%"
      height="24rem"
    >
      <WindowBody>
        <SidebarProvider>
          <Sidebar collapsible="icon">
            {/* The controls sit over this strip. */}
            <SidebarHeader />
            <SidebarContent>
              <SidebarGroup>
                <SidebarGroupLabel>Library</SidebarGroupLabel>
                <SidebarGroupContent>
                  <SidebarMenu>
                    {PANES.map(({ label, icon, count }) => (
                      <SidebarMenuItem key={label}>
                        <SidebarMenuButton
                          icon={<Icon icon={icon} />}
                          tooltip={label}
                          active={pane === label}
                          onClick={() => setPane(label)}
                        >
                          {label}
                        </SidebarMenuButton>
                        {count && <SidebarMenuBadge>{count}</SidebarMenuBadge>}
                      </SidebarMenuItem>
                    ))}
                  </SidebarMenu>
                </SidebarGroupContent>
              </SidebarGroup>
            </SidebarContent>
          </Sidebar>
          <WindowMain>
            <WindowTitlebar
              leading={<SidebarTrigger />}
              title={pane}
              toolbar={
                <SearchField
                  size="small"
                  value={query}
                  onValueChange={setQuery}
                  placeholder="Search"
                />
              }
            />
            <WindowContent>
              <Pane>
                <Text tone="secondary">
                  The band is this pane's, so the sidebar keeps the window's
                  whole height.
                </Text>
              </Pane>
            </WindowContent>
            <WindowFooter>
              <Row gap="2" justify="between" width="100%">
                <Text tone="secondary">12 documents</Text>
                <Button size="small" variant="filled">
                  New document
                </Button>
              </Row>
            </WindowFooter>
          </WindowMain>
        </SidebarProvider>
      </WindowBody>
    </WindowFrame>
  )
}

const story: Story = {
  id: 'window-frame',
  name: 'WindowFrame',
  summary: 'A top-level window, in the chrome of the desktop it names.',
  Preview: () => (
    <WindowFrame title="Software Update" width={280}>
      <Update />
    </WindowFrame>
  ),
  examples: [
    {
      title: 'Platforms',
      note: 'The colours are the theme’s; the chrome is the platform’s own.',
      Demo: () => (
        <Row gap="6" align="start">
          {PLATFORMS.map(({ platform, title }) => (
            <WindowFrame
              key={platform}
              platform={platform}
              title={title}
              width={300}
            >
              <Update />
            </WindowFrame>
          ))}
        </Row>
      ),
    },
    {
      title: 'Inactive',
      note: 'Not the key window: the controls and the title grey out.',
      Demo: () => (
        <Row gap="6" align="start">
          {PLATFORMS.map(({ platform, title }) => (
            <WindowFrame
              key={platform}
              platform={platform}
              title={title}
              width={300}
              inactive
            >
              <Update />
            </WindowFrame>
          ))}
        </Row>
      ),
    },
    {
      title: 'Fixed size',
      note: 'A window that cannot be resized: Close alone on Windows, the other lights disabled on macOS.',
      Demo: () => (
        <Row gap="6" align="start">
          {(['macos', 'windows', 'ubuntu'] as const).map(platform => (
            <WindowFrame
              key={platform}
              platform={platform}
              title="About"
              width={300}
              controls={{ close: true }}
            >
              <Pane>
                <Text tone="secondary">Version 2.4 (204)</Text>
              </Pane>
            </WindowFrame>
          ))}
        </Row>
      ),
    },
    {
      title: 'Controls',
      note: 'A control the window cannot do is greyed on macOS and left out on Windows and GNOME; one it does not carry is left out everywhere.',
      Demo: () => (
        <Col gap="5">
          <ControlStates platform="macos" />
          <ControlStates platform="windows" />
        </Col>
      ),
    },
    {
      title: 'Titlebar slots',
      note: 'Leading, toolbar and trailing, and a title that may be a node — here a glyph and a name on Windows.',
      Demo: () => (
        <Col gap="6" align="start">
          <Slots platform="macos" title="Report" />
          <Slots
            platform="windows"
            title={
              <Row gap="2">
                <Icon icon={Document20Regular} />
                <Text strong>Report</Text>
              </Row>
            }
          />
          <Slots platform="ubuntu" title="Report" />
        </Col>
      ),
    },
    {
      title: 'Accessory',
      note: 'The band’s own centre, where a macOS titlebar accessory view sits: the title yields to it.',
      Demo: () => <Accessory platform="macos" />,
    },
    {
      title: 'Application window',
      note: 'A full-size content view: the sidebar reaches the top edge, the controls sit over its head, and a band with a toolbar in it takes the 52px a SidebarHeader is.',
      Demo: () => <Workspace platform="macos" />,
    },
    {
      title: 'Content only',
      note: 'No titlebar and no controls, with the platform’s own corner, edge and shadow kept — what Omarchy used to be borrowed for.',
      Demo: () => (
        <Row gap="6" align="start">
          {(['macos', 'windows', 'ubuntu'] as const).map(platform => (
            <WindowFrame
              key={platform}
              platform={platform}
              title={`${platform} content`}
              titlebar={false}
              controls={false}
              width={280}
            >
              <Pane>
                <Text tone="secondary">Nothing above the content.</Text>
              </Pane>
            </WindowFrame>
          ))}
        </Row>
      ),
    },
    {
      title: 'Sizing',
      note: 'The height is the window’s own, chrome included; no width, and it opens at the frame’s token width. Min and max win over both.',
      Demo: () => (
        <Col gap="4" align="start">
          <WindowFrame title="No size given">
            <Pane>
              <Text tone="secondary">
                A window that names no size is the size a window is.
              </Text>
            </Pane>
          </WindowFrame>
          <Row gap="6" align="start">
            <WindowFrame title="Min width" width={200} minWidth={320}>
              <Pane>
                <Text tone="secondary">200 asked for, 320 drawn.</Text>
              </Pane>
            </WindowFrame>
            <WindowFrame title="Max height" width={320} maxHeight={100}>
              <Pane>
                <Col gap="1">
                  <Text tone="secondary">Cut off at 100.</Text>
                  <Text tone="secondary">The content here is taller</Text>
                  <Text tone="secondary">than the window it is in,</Text>
                  <Text tone="secondary">so it stops at the edge.</Text>
                </Col>
              </Pane>
            </WindowFrame>
          </Row>
        </Col>
      ),
    },
    {
      title: 'Hidden title',
      note: 'macOS only: a transparent titlebar, the title kept as the accessible name.',
      Demo: () => (
        <WindowFrame title="Preferences" width={320} height={120} titleHidden>
          <Pane>
            <Text tone="secondary">The content runs up to the lights.</Text>
          </Pane>
        </WindowFrame>
      ),
    },
  ],
}

export default story
