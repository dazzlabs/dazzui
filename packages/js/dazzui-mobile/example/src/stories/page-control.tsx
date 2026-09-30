import {
  NavigationBar,
  PageControl,
  Screen,
  Text,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  const [page, setPage] = useState(1)
  return (
    <Screen bar={<NavigationBar title="Page control" />}>
      <Stage>
        <Group title="Plain">
          <PageControl
            aria-label="Onboarding"
            count={5}
            value={page}
            onValueChange={setPage}
          />
          <Text role="footnote" tone="secondary">
            Page {page + 1} of 5 — press a dot, or focus one and use the arrow
            keys.
          </Text>
        </Group>
        <Group title="Over content">
          <div
            style={{
              display: 'flex',
              alignItems: 'flex-end',
              justifyContent: 'center',
              height: 160,
              paddingBottom: 12,
              borderRadius: 26,
              background:
                'linear-gradient(135deg, var(--color-accent-info), var(--color-accent-success))',
            }}
          >
            <PageControl
              aria-label="Photos"
              count={4}
              defaultValue={2}
              background
            />
          </div>
          <Text role="footnote" tone="secondary">
            iOS sets the dots in a glass capsule of their own; Android draws
            them bare.
          </Text>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'page-control',
  name: 'Page control',
  summary:
    'UIPageControl’s dots, in glass over content; the pill-and-dots row Google’s apps draw.',
  Demo,
}

export default story
