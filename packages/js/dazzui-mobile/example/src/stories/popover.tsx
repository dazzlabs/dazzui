import { Filter24Regular, Info24Regular } from '@fluentui/react-icons'
import {
  Button,
  IconButton,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Popover,
  PopoverClose,
  Screen,
} from '@dazzlabs/dazzui-mobile'

import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen
      bar={
        <NavigationBar
          title="Popover"
          trailing={
            <Popover
              align="end"
              trigger={
                <IconButton label="Filters">
                  <Filter24Regular />
                </IconButton>
              }
              title="Filter your library"
              actions={
                <>
                  <PopoverClose
                    render={<Button variant="plain" size="small" />}
                  >
                    Got it
                  </PopoverClose>
                  <Button variant="plain" size="small">
                    Learn more
                  </Button>
                </>
              }
            >
              Show only favourites, or narrow the list to what was added this
              week.
            </Popover>
          }
        />
      }
    >
      <List>
        <ListSection
          header="Storage"
          footer="The info buttons open a popover beside the row."
        >
          <ListItem
            title="Optimise Storage"
            trailing={
              <Popover
                align="end"
                trigger={
                  <IconButton label="About Optimise Storage" variant="plain">
                    <Info24Regular />
                  </IconButton>
                }
              >
                Full-size photos are kept in the cloud and smaller versions on
                this device.
              </Popover>
            }
          />
          <ListItem
            title="Keep Originals"
            trailing={
              <Popover
                align="end"
                title="Keep Originals"
                trigger={
                  <IconButton label="About Keep Originals" variant="plain">
                    <Info24Regular />
                  </IconButton>
                }
              >
                Every photo stays on this device at full size.
              </Popover>
            }
          />
        </ListSection>
      </List>
    </Screen>
  )
}

const story: Story = {
  id: 'popover',
  name: 'Popover',
  summary:
    'The arrowless glass popover iOS 26 draws on iPhone; Material’s rich tooltip.',
  Demo,
}

export default story
