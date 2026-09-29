import { Accordion, AccordionItem } from '@dazzlabs/dazzui-react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const FAQ = [
  {
    value: 'where',
    title: 'Where do the values come from?',
    body: 'Every colour, length and face is a token. A component reads the variable; it never writes a value of its own.',
  },
  {
    value: 'themes',
    title: 'How many themes are there?',
    body: 'Six drawn families in light and dark, and the imported ones beside them.',
  },
  {
    value: 'ports',
    title: 'What keeps the platforms in line?',
    body: 'One design, measured in the browser, that every port is held to.',
  },
]

const items = FAQ.map(item => (
  <AccordionItem key={item.value} value={item.value} title={item.title}>
    {item.body}
  </AccordionItem>
))

const story: Story = {
  id: 'accordion',
  name: 'Accordion',
  summary: 'Several disclosures that know about each other.',
  Preview: () => (
    <Col width="18rem">
      <Accordion defaultValue={['where']}>{items.slice(0, 2)}</Accordion>
    </Col>
  ),
  examples: [
    {
      title: 'One at a time',
      note: 'Opening one shuts the last, so a reader is always looking at one section.',
      Demo: () => (
        <Col width="28rem">
          <Accordion defaultValue={['where']}>{items}</Accordion>
        </Col>
      ),
    },
    {
      title: 'Several at once',
      Demo: () => (
        <Col width="28rem">
          <Accordion multiple defaultValue={['where', 'themes']}>
            {items}
          </Accordion>
        </Col>
      ),
    },
    {
      title: 'Divided',
      Demo: () => (
        <Col width="28rem">
          <Accordion divided defaultValue={['where']}>
            {items}
          </Accordion>
        </Col>
      ),
    },
  ],
}

export default story
