import {
  WeatherMoon20Regular as Moon,
  WeatherSunny20Regular as Sun,
} from '@fluentui/react-icons'
import {
  Checkbox,
  Icon,
  Meter,
  RadioGroup,
  Select,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  Toggle,
} from '@dazzlabs/dazzui-react'
import { type CSSProperties, type ReactNode } from 'react'

import { percent } from '../lib/format'
import type { Index } from '../lib/index'
import type { Lens, Show } from '../lib/lens'
import type { Loaded, StoryScenes } from '../lib/model'
import { FAMILY_GROUPS } from '../lib/themes'
import { score } from '../lib/verdict'
import { tintFor } from './overview'

interface Props {
  loaded: Loaded
  index: Index
  lens: Lens
  /** The story in view, whose scores the panel shows; the whole run otherwise. */
  story: StoryScenes | undefined
  width: number
  resizer: ReactNode
}

const SHOW: { value: Show; label: string }[] = [
  { value: 'all', label: 'Everything' },
  { value: 'off', label: 'Only what differs' },
  { value: 'unread', label: 'Only what could not be read' },
]

/**
 * What the document is looked at through: which implementations, which
 * theme and which half of it, and how much to show. The run's date and
 * commit sit at the bottom — a measurement without them is an impression.
 */
export function LensPanel({
  loaded,
  index,
  lens,
  story,
  width,
  resizer,
}: Props) {
  const dark = lens.theme.mode === 'dark'
  const families = FAMILY_GROUPS.map(group => ({
    label: group.title,
    options: group.families.map(f => ({ value: f.value, label: f.title })),
  }))

  return (
    <Sidebar
      side="right"
      collapsible="none"
      className="gk-lens"
      style={{ '--sidebar-width': `${width}px` } as CSSProperties}
    >
      {resizer}
      <SidebarHeader className="gk-lens__head">
        <strong>Lens</strong>
        <Toggle
          label={dark ? 'Switch to light' : 'Switch to dark'}
          pressed={dark}
          onPressedChange={pressed =>
            lens.setTheme({ ...lens.theme, mode: pressed ? 'dark' : 'light' })
          }
        >
          <Icon icon={dark ? Moon : Sun} />
        </Toggle>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Implementations</SidebarGroupLabel>
          <SidebarGroupContent className="g-lens__list">
            {loaded.implementations.map(platform => {
              const readings = loaded.readings[platform]!
              const shown = lens.platforms.includes(platform)
              const value = score(
                story ? index.story(platform, story) : index.total(platform)
              )
              return (
                <div key={platform} className="g-lens__platform">
                  <Checkbox
                    size="small"
                    checked={shown}
                    onCheckedChange={checked =>
                      lens.setPlatforms(
                        loaded.implementations.filter(p =>
                          p === platform ? checked : lens.platforms.includes(p)
                        )
                      )
                    }
                    note={readings.font ? undefined : 'no shared font'}
                  >
                    {readings.title}
                  </Checkbox>
                  <Meter
                    size="small"
                    value={Math.round((value ?? 0) * 100)}
                    tint={tintFor(value)}
                    aria-label={`${readings.title} score`}
                  />
                  <span className="g-lens__score g-quiet">
                    {percent(value)}
                  </span>
                </div>
              )
            })}
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Theme</SidebarGroupLabel>
          <SidebarGroupContent>
            <Select
              size="small"
              options={families}
              value={lens.theme.family}
              onValueChange={family => lens.setTheme({ ...lens.theme, family })}
              aria-label="Theme family"
            />
            {!index.collected && (
              <p className="g-quiet g-lens__note">Not collected in this run.</p>
            )}
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Show</SidebarGroupLabel>
          <SidebarGroupContent>
            <RadioGroup
              size="small"
              options={SHOW}
              value={lens.show}
              onValueChange={lens.setShow}
              aria-label="What to show"
            />
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter className="g-lens__run">
        <span className="g-quiet">
          Collected {new Date(loaded.run.collected).toLocaleString()} at{' '}
          <code>{loaded.run.commit}</code>
        </span>
      </SidebarFooter>
    </Sidebar>
  )
}
