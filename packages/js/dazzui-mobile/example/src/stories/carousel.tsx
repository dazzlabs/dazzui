import {
  Carousel,
  CarouselItem,
  NavigationBar,
  Screen,
  Text,
} from '@dazzlabs/dazzui-mobile'
import { type ReactNode, useState } from 'react'

import type { Story } from '../lib/story'

/* Placeholders for photographs: a wash between two of the theme's accents. */
const SLIDES = [
  { label: 'Coastline', from: 'info', to: 'success' },
  { label: 'Old town', from: 'warning', to: 'danger' },
  { label: 'Night market', from: 'accent', to: 'info' },
  { label: 'Harbour', from: 'success', to: 'accent' },
  { label: 'Hills', from: 'neutral', to: 'warning' },
] as const

const color = (name: string) =>
  name === 'accent' ? 'var(--color-accent)' : `var(--color-accent-${name})`

function Placeholder({ from, to }: { from: string; to: string }) {
  return (
    <div
      style={{
        height: 200,
        background: `linear-gradient(135deg, ${color(from)}, ${color(to)})`,
      }}
    />
  )
}

/* A carousel runs edge to edge, so it stands outside the stage's padding
   and only its caption is inset. */
function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
      <div style={{ padding: '0 20px' }}>
        <Text
          role="footnote"
          tone="secondary"
          as="h2"
          className="stage__caption"
        >
          {title}
        </Text>
      </div>
      {children}
    </section>
  )
}

function Demo() {
  const [index, setIndex] = useState(0)
  return (
    <Screen bar={<NavigationBar title="Carousel" />}>
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          gap: 28,
          padding: '8px 0 40px',
        }}
      >
        <Section title="Labelled">
          <Carousel aria-label="Places" value={index} onValueChange={setIndex}>
            {SLIDES.map(slide => (
              <CarouselItem key={slide.label} label={slide.label}>
                <Placeholder from={slide.from} to={slide.to} />
              </CarouselItem>
            ))}
          </Carousel>
          <div style={{ padding: '0 20px' }}>
            <Text role="footnote" tone="secondary">
              Showing {SLIDES[index]?.label}. Swipe, or focus the strip and use
              the arrow keys.
            </Text>
          </div>
        </Section>
        <Section title="With a page control on both">
          <Carousel aria-label="Colours" showPageControl>
            {SLIDES.slice(0, 3).map(slide => (
              <CarouselItem key={slide.label}>
                <Placeholder from={slide.to} to={slide.from} />
              </CarouselItem>
            ))}
          </Carousel>
        </Section>
      </div>
    </Screen>
  )
}

const story: Story = {
  id: 'carousel',
  name: 'Carousel',
  summary:
    'A paging scroll view with its page control; Material’s carousel with the next item peeking.',
  Demo,
}

export default story
