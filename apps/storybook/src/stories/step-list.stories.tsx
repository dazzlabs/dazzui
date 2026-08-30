import { Step, StepList } from '@dazzlabs/dazzui-react'
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Page, Section, TokenTable } from '../components/gallery'
import { componentTokens } from '../lib/tokens'

const meta: Meta<typeof StepList> = {
  title: 'Components/StepList',
  component: StepList,
}

export default meta
type Story = StoryObj<typeof meta>

export const Overview: Story = {
  render: () => (
    <Page
      title="StepList"
      description="A pipeline, one line per stage. The marker carries the status — a check when a stage is finished, the spinner while it runs, an empty ring before it starts — so the label never has to say it twice."
    >
      <Section
        title="In flight"
        description="A finished step is success wherever the list is tinted: it is the one fixed meaning in the column, not a variation on the accent."
      >
        <StepList>
          <Step status="done" label="Parsed the document" meta="15 pages" />
          <Step status="done" label="Extracted segments" meta="392" />
          <Step status="active" label="Translating" meta="168 / 392" />
          <Step status="idle" label="Applying the glossary" />
          <Step status="idle" label="Rebuilding the layout" />
        </StepList>
      </Section>

      <Section
        title="Finished"
        description="Every stage done, so nothing in the column is asking for attention."
      >
        <StepList>
          <Step status="done" label="Parsed the document" meta="15 pages" />
          <Step status="done" label="Extracted segments" meta="392" />
          <Step status="done" label="Translating" meta="392 / 392" />
        </StepList>
      </Section>

      <Section title="Tokens">
        <TokenTable
          tokens={componentTokens('step-list')}
          labelOf={token => token.name.slice('step-list-'.length)}
          nameHeader="Part"
        />
      </Section>
    </Page>
  ),
}
