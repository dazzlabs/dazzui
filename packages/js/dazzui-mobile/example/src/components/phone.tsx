import {
  type Appearance,
  type Family,
  MobileTheme,
  type Platform,
  ToastProvider,
} from '@dazzlabs/dazzui-mobile'
import type { ReactNode } from 'react'

import './phone.css'

export interface PhoneProps {
  platform: Platform
  family: Family
  appearance: Appearance
  children: ReactNode
}

/**
 * A device: an iPhone for iOS, a Pixel for Android, each at its own
 * logical size — 393 by 852 and 412 by 915 points — with the status bar
 * the OS draws and the theme installed on the screen inside. The frame is
 * presentation; nothing the kit draws depends on it.
 */
export function Phone({ platform, family, appearance, children }: PhoneProps) {
  return (
    <div className="phone" data-platform={platform}>
      <div className="phone__screen">
        <MobileTheme
          family={family}
          appearance={appearance}
          platform={platform}
          contained
          className="phone__theme"
        >
          <StatusBar platform={platform} />
          <div className="phone__content">
            <ToastProvider>{children}</ToastProvider>
          </div>
          <HomeIndicator platform={platform} />
        </MobileTheme>
      </div>
    </div>
  )
}

function StatusBar({ platform }: { platform: Platform }) {
  return (
    <div className="phone__status" aria-hidden>
      <span className="phone__time">9:41</span>
      {platform === 'ios' ? (
        <span className="phone__island" />
      ) : (
        <span className="phone__hole" />
      )}
      <span className="phone__signals">
        <svg viewBox="0 0 20 12" width="18" height="11">
          <rect x="0" y="8" width="3" height="4" rx="0.8" fill="currentColor" />
          <rect
            x="5"
            y="5.5"
            width="3"
            height="6.5"
            rx="0.8"
            fill="currentColor"
          />
          <rect
            x="10"
            y="3"
            width="3"
            height="9"
            rx="0.8"
            fill="currentColor"
          />
          <rect
            x="15"
            y="0"
            width="3"
            height="12"
            rx="0.8"
            fill="currentColor"
          />
        </svg>
        <svg viewBox="0 0 16 12" width="16" height="12">
          <path
            d="M8 11.2 5.6 8.7a3.3 3.3 0 0 1 4.8 0L8 11.2zM3.4 6.5A6.4 6.4 0 0 1 12.6 6.5l-1.5 1.5a4.4 4.4 0 0 0-6.2 0L3.4 6.5zM1 4A9.8 9.8 0 0 1 15 4l-1.4 1.5a7.8 7.8 0 0 0-11.2 0L1 4z"
            fill="currentColor"
          />
        </svg>
        <svg viewBox="0 0 28 13" width="27" height="13">
          <rect
            x="0.5"
            y="0.5"
            width="24"
            height="12"
            rx="3.5"
            fill="none"
            stroke="currentColor"
            strokeOpacity="0.4"
          />
          <rect x="2" y="2" width="21" height="9" rx="2" fill="currentColor" />
          <path
            d="M26 4.5v4a2 2 0 0 0 0-4z"
            fill="currentColor"
            fillOpacity="0.4"
          />
        </svg>
      </span>
    </div>
  )
}

function HomeIndicator({ platform }: { platform: Platform }) {
  return <div className="phone__home" data-platform={platform} aria-hidden />
}
