import { OtpField } from '@dazzlabs/dazzui-react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'otp-field',
  name: 'OtpField',
  summary: 'A code, a character to a box.',
  Preview: () => <OtpField length={6} aria-label="Code" />,
  examples: [
    {
      title: 'Six digits',
      Demo: () => <OtpField length={6} aria-label="Code" />,
    },
    {
      title: 'In two groups',
      Demo: () => <OtpField length={6} groupAfter={3} aria-label="Code" />,
    },
    {
      title: 'Sizes and error',
      Demo: () => (
        <Col align="start" gap="4">
          <OtpField length={4} size="small" aria-label="Small" />
          <OtpField length={4} size="large" aria-label="Large" />
          <OtpField length={4} state="error" aria-label="Error" />
        </Col>
      ),
    },
  ],
}

export default story
