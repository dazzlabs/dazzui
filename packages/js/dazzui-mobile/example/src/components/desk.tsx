import type { Platform } from '@dazzlabs/dazzui-mobile'
import { type CSSProperties, useEffect, useRef, useState } from 'react'

import type { Story } from '../lib/story'
import type { Platforms, Theme } from '../lib/themes'
import { Phone } from './phone'

/* The taller of the two devices with its frame, plus its label. */
const DEVICE_HEIGHT = 935 + 40

/**
 * The document: the two devices side by side, running the same story
 * under the family the panel chose. They stand at their logical size and
 * scale down together when the window is shorter than a phone.
 */
export function Desk({
  story,
  theme,
  platforms,
}: {
  story: Story
  theme: Theme
  platforms: Platforms
}) {
  const fit = useFit()
  const shown: Platform[] =
    platforms === 'both' ? ['ios', 'android'] : [platforms]
  return (
    <div className="gk-desk" ref={fit.ref}>
      <div
        className="gk-desk__row"
        style={{ '--phone-scale': fit.scale } as CSSProperties}
      >
        {shown.map(platform => (
          <div key={platform} className="gk-device">
            <span className="gk-device__label">
              {platform === 'ios' ? 'iOS 27 · iPhone' : 'Android · Pixel'}
            </span>
            <Phone
              platform={platform}
              family={theme.family}
              appearance={theme.mode}
            >
              <story.Demo key={`${story.id}-${platform}`} />
            </Phone>
          </div>
        ))}
      </div>
    </div>
  )
}

function useFit() {
  const ref = useRef<HTMLDivElement>(null)
  const [scale, setScale] = useState(1)
  useEffect(() => {
    const element = ref.current
    if (!element) return
    const measure = () =>
      setScale(Math.min(1, (element.clientHeight - 64) / DEVICE_HEIGHT))
    measure()
    const observer = new ResizeObserver(measure)
    observer.observe(element)
    return () => observer.disconnect()
  }, [])
  return { ref, scale }
}
