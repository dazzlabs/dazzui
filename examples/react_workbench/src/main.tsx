import '@dazzlabs/dazzui-react/style.css'
import '@dazzlabs/dazzui-react/components.css'
import './styles/app.css'

import { ToastProvider } from '@dazzlabs/dazzui-react'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import { App } from './app'

const root = document.getElementById('root')
if (!root) throw new Error('index.html is missing its #root element')

/* The toast queue wraps the app rather than living inside it: the app is
   what pushes into it, and a provider cannot be its own consumer. */
createRoot(root).render(
  <StrictMode>
    <ToastProvider>
      <App />
    </ToastProvider>
  </StrictMode>
)
