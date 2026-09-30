import { Building24Regular } from '@fluentui/react-icons'
import {
  Avatar,
  AvatarGroup,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
} from '@dazzlabs/dazzui-mobile'

import { Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

/**
 * A portrait to stand in for a photo: a head and shoulders on a wash, as
 * an SVG the story can carry without a network.
 */
function portrait(sky: string, skin: string, shirt: string) {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 80"><rect width="80" height="80" fill="${sky}"/><circle cx="40" cy="32" r="14" fill="${skin}"/><path d="M12 84c0-17 12.5-28 28-28s28 11 28 28z" fill="${shirt}"/></svg>`
  return `data:image/svg+xml,${encodeURIComponent(svg)}`
}

const MAYA = portrait('#9cc9e8', '#e6b89c', '#3d5a80')
const THEO = portrait('#f2c572', '#8d5a3b', '#2a9d8f')

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Avatar" />}>
      <List>
        <ListSection header="In a list">
          <ListItem
            title="Trip to Lisbon"
            subtitle="Shared album"
            trailing={
              <AvatarGroup size="small" max={3}>
                <Avatar src={MAYA} name="Maya Chen" />
                <Avatar name="Jonas Weber" />
                <Avatar src={THEO} name="Theo Okafor" />
                <Avatar name="Priya Nair" />
                <Avatar name="Lena Park" />
              </AvatarGroup>
            }
          />
        </ListSection>
      </List>
      <Stage>
        <Group title="Sizes">
          <Row>
            <Avatar size="small" name="Ada Lovelace" />
            <Avatar size="medium" name="Ada Lovelace" />
            <Avatar size="large" name="Ada Lovelace" />
          </Row>
        </Group>
        <Group title="Pictures">
          <Row>
            <Avatar size="small" src={MAYA} name="Maya Chen" />
            <Avatar size="medium" src={MAYA} name="Maya Chen" />
            <Avatar size="large" src={THEO} name="Theo Okafor" />
          </Row>
        </Group>
        <Group title="Fallbacks">
          <Row>
            <Avatar name="Grace Hopper" />
            <Avatar name="Cher" />
            <Avatar icon={<Building24Regular />} alt="Northwind Ltd." />
            <Avatar alt="Unknown caller" />
          </Row>
        </Group>
        <Group title="Group">
          <Row>
            <AvatarGroup>
              <Avatar src={MAYA} name="Maya Chen" />
              <Avatar name="Jonas Weber" />
              <Avatar src={THEO} name="Theo Okafor" />
            </AvatarGroup>
          </Row>
          <Row>
            <AvatarGroup max={3}>
              <Avatar name="Priya Nair" />
              <Avatar src={MAYA} name="Maya Chen" />
              <Avatar name="Lena Park" />
              <Avatar name="Omar Haddad" />
              <Avatar name="Sofia Rossi" />
              <Avatar name="Kenji Sato" />
            </AvatarGroup>
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'avatar',
  name: 'Avatar',
  summary:
    'A picture or a monogram: Contacts’ grey gradient; Material’s primary container.',
  Demo,
}

export default story
