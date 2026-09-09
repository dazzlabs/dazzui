import { Accordion, AccordionItem, Badge } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, Specimen, Stack } from '../components/gallery'

const meta: Meta<typeof Accordion> = {
  title: 'Components/Accordion',
  component: Accordion,
  parameters: { layout: 'centered' },
}

export default meta
type Story = StoryObj<typeof meta>

const FAQ = [
  {
    value: 'where',
    title: 'Where do the values come from?',
    body: 'Every colour, length and face is a token in packages/ds. A component reads the variable; it never writes a value of its own.',
  },
  {
    value: 'themes',
    title: 'How many themes are there?',
    body: 'Twelve, as six families over one default. The modes compose, so a value shared along the light/dark axis is written once.',
  },
  {
    value: 'ports',
    title: 'What keeps the platforms in line?',
    body: 'The parity pipeline measures the React build in a browser and holds every port to those numbers.',
  },
]

/** The documented page. Every story after this one is the component alone. */
export const Overview: Story = {
  parameters: { layout: 'fullscreen' },
  render: () => (
    <Page
      title="Accordion"
      description="Several disclosures that know about each other. That knowledge is the whole difference from a run of Collapsibles: the arrow keys walk the headers, Home and End reach the ends, and by default opening one shuts the last — so a reader is always looking at one section rather than at a page that grew while they were reading it."
    >
      <Section
        title="One at a time"
        description="The default, and the one to reach for when the sections are alternatives — a FAQ, a set of steps, anything where two open at once would push the third off the screen."
      >
        <div style={{ width: '28rem' }}>
          <Accordion defaultValue={['where']}>
            {FAQ.map(item => (
              <AccordionItem
                key={item.value}
                value={item.value}
                title={item.title}
              >
                {item.body}
              </AccordionItem>
            ))}
          </Accordion>
        </div>
      </Section>

      <Section
        title="Several at once"
        description="`multiple` is for the case where the reader is comparing rather than choosing — two log sections side by side, two diffs. It is a deliberate opt-in, because a list that can be opened all the way is a list that can be scrolled past."
      >
        <div style={{ width: '28rem' }}>
          <Accordion multiple defaultValue={['where', 'themes']}>
            {FAQ.map(item => (
              <AccordionItem
                key={item.value}
                value={item.value}
                title={item.title}
              >
                {item.body}
              </AccordionItem>
            ))}
          </Accordion>
        </div>
      </Section>

      <Section
        title="Divided"
        description="Undivided, the sections sit on one run and the rows are separated by their own hover wash alone. `divided` draws the hairline, for the case where the sections are genuinely a list and the reader is counting them."
      >
        <Stack>
          <Specimen label="divided">
            <div style={{ width: '28rem' }}>
              <Accordion divided defaultValue={['where']}>
                {FAQ.map(item => (
                  <AccordionItem
                    key={item.value}
                    value={item.value}
                    title={item.title}
                    meta={<Badge size="small">{item.value}</Badge>}
                  >
                    {item.body}
                  </AccordionItem>
                ))}
              </Accordion>
            </div>
          </Specimen>
        </Stack>
      </Section>
    </Page>
  ),
}

export const OneAtATime: Story = {
  render: () => (
    <div style={{ width: '28rem' }}>
      <Accordion defaultValue={['where']}>
        {FAQ.map(item => (
          <AccordionItem key={item.value} value={item.value} title={item.title}>
            {item.body}
          </AccordionItem>
        ))}
      </Accordion>
    </div>
  ),
}

export const Multiple: Story = {
  render: () => (
    <div style={{ width: '28rem' }}>
      <Accordion multiple defaultValue={['where', 'themes']}>
        {FAQ.map(item => (
          <AccordionItem key={item.value} value={item.value} title={item.title}>
            {item.body}
          </AccordionItem>
        ))}
      </Accordion>
    </div>
  ),
}

export const Divided: Story = {
  render: () => (
    <div style={{ width: '28rem' }}>
      <Accordion divided defaultValue={['where']}>
        {FAQ.map(item => (
          <AccordionItem
            key={item.value}
            value={item.value}
            title={item.title}
            meta={<Badge size="small">{item.value}</Badge>}
          >
            {item.body}
          </AccordionItem>
        ))}
      </Accordion>
    </div>
  ),
}

export const Disabled: Story = {
  render: () => (
    <div style={{ width: '28rem' }}>
      <Accordion divided defaultValue={['where']}>
        <AccordionItem value="where" title="Where do the values come from?">
          Every colour, length and face is a token in packages/ds.
        </AccordionItem>
        <AccordionItem disabled value="locked" title="Managed by your admin">
          Nothing here can be changed on this device.
        </AccordionItem>
      </Accordion>
    </div>
  ),
}
