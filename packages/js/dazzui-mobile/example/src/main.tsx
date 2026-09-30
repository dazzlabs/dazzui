import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
/* The window is the desktop kit's; the screens inside it are this one's. */
import '@dazzlabs/dazzui/style.css'
import '@dazzlabs/dazzui/components.css'
import '@dazzlabs/dazzui-mobile/mobile.css'
import '@dazzlabs/dazzui-mobile/components.css'

import { App } from './app'
import './styles/base.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>
)
