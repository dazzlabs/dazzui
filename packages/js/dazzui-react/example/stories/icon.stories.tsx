import { defineStory } from '../src/lib/scene'
import scenes from './icon.scene.json'
import {
  Flash20Regular as Bolt,
  Checkmark20Regular as Check,
  Document20Regular as Doc,
  Globe20Regular as Globe,
} from '@fluentui/react-icons'
import {
  ActionBar,
  Badge,
  Button,
  Icon,
  IconButton,
  SidebarMenuButton,
  TINTS,
} from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof Icon> = {
  title: 'Components/Icon',
  component: Icon,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

/** Stand-ins, so the gallery does not pull in an icon set of its own. */
/** A 24-viewBox glyph, to show that the box is scaled rather than assumed. */
/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Icon"
      description="Draws a glyph at a given size. The system draws with Fluent, which `dazzui-react` depends on, but `Icon` is not bound to it: the prop takes any component that renders an svg, so a product can pass its own set without forking the components that use one."
    >
      <Section
        title="Sizes"
        description="`size` sets the rendered edge length, in whatever unit CSS takes. There is no size scale here: an icon is sized by the thing it sits in — a small button's label, a rail row, a badge — so the number belongs at the call site. A fixed-size glyph is scaled to fit, so a 24-unit set and a 20-unit set line up without either being redrawn."
      >
        <Stack>
          <Specimen label="12 → 32">
            <Icon icon={Bolt} size={12} />
            <Icon icon={Bolt} size={14} />
            <Icon icon={Bolt} size={16} />
            <Icon icon={Bolt} size={20} />
            <Icon icon={Bolt} size={24} />
            <Icon icon={Bolt} size={32} />
          </Specimen>
          <Specimen label="20 vs 24">
            <Icon icon={Doc} size={24} />
            <Icon icon={Globe} size={24} />
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Colour"
        description="Fill rides on `currentColor`, so an icon takes the tone of whatever it sits in and no component has to pass a colour down to it. That is what lets one glyph go inside a filled button, a tinted badge, and a muted caption without three variants of it."
      >
        <Stack>
          <Specimen label="in badges">
            {TINTS.map(tint => (
              <Badge key={tint} tint={tint}>
                <Icon icon={Check} size={12} /> {tint}
              </Badge>
            ))}
          </Specimen>
          <Specimen label="content roles">
            <span>
              <Icon icon={Doc} size={16} />
            </span>
            <span style={{ color: 'var(--color-content-muted)' }}>
              <Icon icon={Doc} size={16} />
            </span>
            <span style={{ color: 'var(--color-content-subtle)' }}>
              <Icon icon={Doc} size={16} />
            </span>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Where it lands"
        description="In each case the glyph is a child, not a prop on the component — which is why the system needs no icon registry and a product can drop its own set in."
      >
        <Stack>
          <Specimen label="button">
            <Button variant="filled">
              <Icon icon={Bolt} size={14} />
              Translate
            </Button>
            <IconButton label="Open">
              <Icon icon={Doc} size={16} />
            </IconButton>
          </Specimen>
          <Specimen label="nav row">
            <div style={{ width: '11rem' }}>
              <SidebarMenuButton active icon={<Icon icon={Doc} size={14} />}>
                <span>handbook.docx</span>
              </SidebarMenuButton>
            </div>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const Default: Story = defineStory(scenes.Default)

export const Sizes: Story = defineStory(scenes.Sizes)

export const SizedByAToken: Story = defineStory(scenes.SizedByAToken)

export const AnyViewBox: Story = defineStory(scenes.AnyViewBox)

export const InheritedColour: Story = defineStory(scenes.InheritedColour)

export const InAButton: Story = defineStory(scenes.InAButton)

export const InAToolbar: Story = {
  render: () => (
    <ActionBar>
      <IconButton label="Translate">
        <Icon icon={Bolt} size={16} />
      </IconButton>
      <IconButton label="Open">
        <Icon icon={Doc} size={16} />
      </IconButton>
      <IconButton label="Done">
        <Icon icon={Check} size={16} />
      </IconButton>
    </ActionBar>
  ),
}

export const InANavRow: Story = {
  render: () => (
    <Col gap="var(--spacing-1)" width="13rem">
      <SidebarMenuButton active icon={<Icon icon={Doc} size={14} />}>
        <span>Documents</span>
      </SidebarMenuButton>
      <SidebarMenuButton icon={<Icon icon={Globe} size={14} />}>
        <span>Glossaries</span>
      </SidebarMenuButton>
      <SidebarMenuButton icon={<Icon icon={Bolt} size={14} />}>
        <span>Shortcuts</span>
      </SidebarMenuButton>
    </Col>
  ),
}

export const InACallout: Story = defineStory(scenes.InACallout)
