import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

import { parityData } from './parity-data'

export default defineConfig({
  plugins: [react(), parityData()],
})
