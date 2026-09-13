import { Link, TINTS } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '../src/lib/story'

import { Page, Section, Specimen, Stack } from '../src/components/gallery'
import { Col } from '../src/components/stage'

const meta: Meta<typeof Link> = {
  title: 'Components/Link',
  component: Link,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const UNDERLINES = ['hover', 'always', 'none'] as const

export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Link"
      description="Text that goes somewhere. It is the one interactive thing in this package that is genuinely inline, which is why it exists beside Button variant='plain' rather than inside it: a button keeps its box at every variant, and a box is what a link in a sentence must not have. It takes the shared state machine for its colour — the plain recipe, a grade darker on hover — and none of its geometry."
    >
      <Section
        title="Underline"
        description="`hover` is the default, because a paragraph with a rule under every link reads as struck through. `always` is for a link that has to be findable without a pointer — a list of references, a footer. `none` is for one whose surroundings already say it is a link: a breadcrumb, a row that is entirely a target."
      >
        <Stack>
          {UNDERLINES.map(underline => (
            <Specimen key={underline} label={underline}>
              <Link href="#" underline={underline}>
                Open the changelog
              </Link>
            </Specimen>
          ))}
        </Stack>
      </Section>

      <Section
        title="External"
        description="Sets the target and the rel that keeps the opener out of the new page's reach, and draws the arrow that says so before the reader commits. The arrow is inline, so it wraps with the last word rather than stranding itself on a line of its own."
      >
        <Stack>
          <Specimen label="same window">
            <Link href="#">Release notes</Link>
          </Specimen>
          <Specimen label="external">
            <Link external href="https://base-ui.com">
              Base UI
            </Link>
          </Specimen>
        </Stack>
      </Section>

      <Section
        title="Tints"
        description="A link is accent text, so it reads the plain recipe's content grade off whichever ramp it is tinted with. Danger is the one worth reaching for — a link that deletes something."
      >
        <Stack>
          {TINTS.map(tint => (
            <Specimen key={tint} label={tint}>
              <Link href="#" tint={tint}>
                Continue
              </Link>
            </Specimen>
          ))}
        </Stack>
      </Section>
    </Page>
  ),
}

export const Underlines: Story = {
  render: () => (
    <Col>
      {UNDERLINES.map(underline => (
        <Link key={underline} href="#" underline={underline}>
          Open the changelog
        </Link>
      ))}
    </Col>
  ),
}

export const External: Story = {
  render: () => (
    <Link external href="https://base-ui.com">
      Base UI
    </Link>
  ),
}

export const Tints: Story = {
  render: () => (
    <Col>
      {TINTS.map(tint => (
        <Link key={tint} href="#" tint={tint}>
          Continue
        </Link>
      ))}
    </Col>
  ),
}

export const Disabled: Story = {
  render: () => (
    <Link aria-disabled href="#">
      Not available yet
    </Link>
  ),
}

export const InProse: Story = {
  render: () => (
    <Col width="26rem">
      <p
        style={{
          margin: 0,
          color: 'var(--color-content)',
          fontFamily: 'var(--body-small-font-family)',
          fontSize: 'var(--body-small-font-size)',
          lineHeight: 'var(--body-small-line-height)',
        }}
      >
        Behaviour comes from <Link href="#">Base UI</Link>, and the drawing is
        ours. Where a control has to reach a form, read{' '}
        <Link href="#">the field conventions</Link> first — or open the{' '}
        <Link external href="https://base-ui.com">
          upstream docs
        </Link>{' '}
        for the parts this package does not wrap.
      </p>
    </Col>
  ),
}
