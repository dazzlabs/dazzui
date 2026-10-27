import {
  PreferenceRow,
  Preferences,
  PreferenceSection,
  PreferenceThemeSwatches,
  Switch,
} from '@dazzlabs/dazzui'

import { Col } from '../../components/stage'
import type { StoryExtension } from '../../lib/story'

/*
 * React only, for now: a swatch takes its theme by nesting a `data-theme`,
 * which no port has a way to do inside one subtree yet. These examples join
 * the compiled Preferences page on React and nowhere else.
 */

const THEMES = [
  { value: 'studio-light', label: 'Studio', theme: '' },
  { value: 'bright-light', label: 'Bright' },
  { value: 'frost-light', label: 'Frost' },
  { value: 'graphite-light', label: 'Graphite' },
  { value: 'ember-light', label: 'Ember' },
  { value: 'nocturne-light', label: 'Nocturne' },
]

const extension: StoryExtension = {
  id: 'preferences',
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
                  defaultValue="studio-light"
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
