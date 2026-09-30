import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import '@dazzlabs/dazzui-react/style.css'
import '@dazzlabs/dazzui-react/components.css'

import { App } from './app'
import './styles/base.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>
)
