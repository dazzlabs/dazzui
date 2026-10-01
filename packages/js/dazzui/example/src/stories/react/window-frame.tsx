import { Button, WindowFrame, type WindowFramePlatform } from '@dazzlabs/dazzui'
import type { ReactNode } from 'react'

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
