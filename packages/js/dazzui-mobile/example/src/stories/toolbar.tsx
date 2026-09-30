import {
  Archive24Regular,
  ArrowReply24Regular,
  Camera24Regular,
  Compose24Regular,
  Delete24Regular,
  Edit24Regular,
  Flag24Regular,
  Folder24Regular,
  Share24Regular,
  TaskListLtr24Regular,
} from '@fluentui/react-icons'
import {
  Fab,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Text,
  Toolbar,
  ToolbarButton,
  ToolbarGroup,
  ToolbarSpacer,
  usePlatform,
} from '@dazzlabs/dazzui-mobile'

import { Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

const NOTES = [
  ['Groceries', 'Oat milk, lemons, basil'],
  ['Trip to Lisbon', 'Book the tram tour for Saturday'],
  ['Standup', 'Ship the carousel, review the toolbar'],
  ['Books', 'The Left Hand of Darkness'],
  ['Ideas', 'A page control that stretches'],
  ['Recipes', 'Shakshuka for four'],
  ['Garden', 'Repot the fig before spring'],
  ['Gifts', 'Something for Mei’s birthday'],
] as const

function Demo() {
  // iOS keeps the new-note action in the toolbar; Android hands it to the
  // FAB beside it, which iOS does not draw.
  const platform = usePlatform()
  return (
    <Screen
      bar={<NavigationBar title="Notes" large />}
      tabBar={
        <Toolbar
          aria-label="Note actions"
          fab={
            <Fab
              icon={<Compose24Regular />}
              label="New note"
              tone="secondary"
            />
          }
        >
          <ToolbarGroup>
            <ToolbarButton label="Checklist">
              <TaskListLtr24Regular />
            </ToolbarButton>
            <ToolbarButton label="Camera">
              <Camera24Regular />
            </ToolbarButton>
          </ToolbarGroup>
          {platform === 'ios' ? (
            <>
              <ToolbarSpacer />
              <ToolbarGroup>
                <ToolbarButton label="New note">
                  <Compose24Regular />
                </ToolbarButton>
              </ToolbarGroup>
            </>
          ) : null}
        </Toolbar>
      }
    >
      <List>
        <ListSection header="Today">
          {NOTES.map(([title, subtitle]) => (
            <ListItem
              key={title}
              title={title}
              subtitle={subtitle}
              onClick={() => {}}
            />
          ))}
        </ListSection>
      </List>
      <Stage>
        <Group title="In the flow">
          <Toolbar aria-label="Message actions" position="static">
            <ToolbarGroup>
              <ToolbarButton label="Archive">
                <Archive24Regular />
              </ToolbarButton>
              <ToolbarButton label="Move">
                <Folder24Regular />
              </ToolbarButton>
              <ToolbarButton label="Flag">
                <Flag24Regular />
              </ToolbarButton>
            </ToolbarGroup>
            <ToolbarSpacer />
            <ToolbarGroup>
              <ToolbarButton label="Delete" tone="destructive">
                <Delete24Regular />
              </ToolbarButton>
            </ToolbarGroup>
          </Toolbar>
        </Group>
        <Group title="Vibrant (Material)">
          <Toolbar
            aria-label="Editing"
            position="static"
            variant="vibrant"
            fab={<Fab icon={<Edit24Regular />} label="Edit" tone="tertiary" />}
          >
            <ToolbarGroup>
              <ToolbarButton label="Reply">
                <ArrowReply24Regular />
              </ToolbarButton>
              <ToolbarButton label="Share">
                <Share24Regular />
              </ToolbarButton>
              <ToolbarButton label="Flag">
                <Flag24Regular />
              </ToolbarButton>
            </ToolbarGroup>
          </Toolbar>
        </Group>
        <Text role="footnote" tone="secondary">
          iOS gathers the items into glass capsules and keeps the primary action
          in the bar; Android floats one capsule with a FAB beside it.
        </Text>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'toolbar',
  name: 'Toolbar',
  summary:
    'UIToolbar’s glass capsules at the foot of the screen; Material’s floating toolbar with its FAB.',
  Demo,
}

export default story
