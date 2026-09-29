import { Link, TINTS } from '@dazzlabs/dazzui-react'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'link',
  name: 'Link',
  summary: 'Somewhere to go, set in the line it is read in.',
  Preview: () => (
    <Row>
      <Link href="#/link">Read the guide</Link>
      <Link href="#/link" external>
        Open the repository
      </Link>
    </Row>
  ),
  examples: [
    {
      title: 'Underline',
      Demo: () => (
        <Row gap="5">
          <Link href="#/link" underline="hover">
            On hover
          </Link>
          <Link href="#/link" underline="always">
            Always
          </Link>
          <Link href="#/link" underline="none">
            Never
          </Link>
        </Row>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Row gap="5">
          {TINTS.map(tint => (
            <Link key={tint} href="#/link" tint={tint}>
              {tint}
            </Link>
          ))}
        </Row>
      ),
    },
  ],
}

export default story
