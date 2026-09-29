import { Avatar, AvatarGroup } from '@dazzlabs/dazzui-react'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

const PEOPLE = [
  'Ada Lovelace',
  'Grace Hopper',
  'Alan Turing',
  'Edsger Dijkstra',
]

const story: Story = {
  id: 'avatar',
  name: 'Avatar',
  summary: 'A person, by their picture or by their initials.',
  Preview: () => (
    <AvatarGroup max={3}>
      {PEOPLE.map(name => (
        <Avatar key={name} name={name} />
      ))}
    </AvatarGroup>
  ),
  examples: [
    {
      title: 'Sizes and shapes',
      Demo: () => (
        <Row>
          <Avatar size="small" name="Ada Lovelace" />
          <Avatar size="medium" name="Ada Lovelace" />
          <Avatar size="large" name="Ada Lovelace" />
          <Avatar size="large" shape="square" name="Ada Lovelace" />
        </Row>
      ),
    },
    {
      title: 'A group',
      Demo: () => (
        <AvatarGroup max={3}>
          {PEOPLE.map(name => (
            <Avatar key={name} name={name} />
          ))}
        </AvatarGroup>
      ),
    },
  ],
}

export default story
