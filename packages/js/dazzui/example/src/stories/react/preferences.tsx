import {
  PreferenceRow,
  Preferences,
  PreferenceSection,
  PreferenceThemeSwatches,
  Switch,
} from '@dazzlabs/dazzui'

import { Col } from '../../components/stage'
import type { StoryExtension } from '../../lib/story'

// The native ports resolve each swatch's own palette from their token tables.

const THEMES = [
  { value: 'bright-light', label: 'Bright', theme: '' },
  { value: 'studio-light', label: 'Studio' },
  { value: 'frost-light', label: 'Frost' },
  { value: 'graphite-light', label: 'Graphite' },
  { value: 'ember-light', label: 'Ember' },
  { value: 'nocturne-light', label: 'Nocturne' },
]

const extension: StoryExtension = {
  id: 'preferences',
  platforms: ['flutter', 'swiftui', 'winui', 'gpui'],
  examples: [
    {
      title: 'Choosing a theme',
      note: 'Each swatch is drawn in its own theme, whatever the page is in.',
      Demo: () => (
        <Col width="32rem">
          <Preferences>
            <PreferenceSection label="Appearance">
              <PreferenceRow title="Theme">
                <PreferenceThemeSwatches
                  aria-label="Theme"
                  options={THEMES}
                  defaultValue="bright-light"
                />
              </PreferenceRow>
              <PreferenceRow title="Follow the system">
                <Switch defaultChecked aria-label="Follow the system" />
              </PreferenceRow>
            </PreferenceSection>
          </Preferences>
        </Col>
      ),
    },
  ],
}

export default extension
