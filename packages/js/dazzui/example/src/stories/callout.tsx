import { Button, Callout, TINTS } from '@dazzlabs/dazzui'
import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'callout',
  name: 'Callout',
  summary: 'Something the reader should know before they go on.',
  Preview: () => (
    <Col width="18rem">
      <Callout tint="info" title="A new version is ready">
        Restart to finish updating.
      </Callout>
    </Col>
  ),
  examples: [
    {
      title: 'Tints',
      Demo: () => (
        <Col width="28rem">
          {TINTS.map(tint => (
            <Callout key={tint} tint={tint} title={`A ${tint} callout`}>
              What happened, and what to do about it.
            </Callout>
          ))}
        </Col>
      ),
    },
    {
      title: 'With an action',
      Demo: () => (
        <Col width="28rem">
          <Callout
            tint="warning"
            title="Your trial ends in three days"
            action={
              <Button size="small" variant="tinted" tint="warning">
                Upgrade
              </Button>
            }
          >
            Projects stay readable after it does.
          </Callout>
        </Col>
      ),
    },
  ],
}

export default story
