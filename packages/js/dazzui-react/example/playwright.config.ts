import { defineConfig } from '@playwright/test'
import scenes from './src/generated/scenes.json' with { type: 'json' }

export default defineConfig({
  testDir: './tests',
  timeout: 60_000,
  workers: 1,
  retries: 0,
  updateSnapshots: 'none',
  reporter: [['list'], ['html', { open: 'never' }]],
  outputDir: './test-results',
  snapshotPathTemplate: `{testDir}/goldens/${process.platform}-${process.arch}/{arg}{ext}`,
  use: {
    baseURL: 'http://127.0.0.1:6007',
    viewport: { width: 960, height: 640 },
    deviceScaleFactor: 2,
    reducedMotion: 'reduce',
    colorScheme: 'light',
    locale: 'en-US',
    timezoneId: 'UTC',
    trace: 'retain-on-failure',
  },
  webServer: {
    command: 'pnpm exec vite preview --host 127.0.0.1 --port 6007 --strictPort',
    url: 'http://127.0.0.1:6007',
    reuseExistingServer: false,
  },
  projects: [
    { name: 'smoke', testMatch: 'playground.spec.ts' },
    { name: 'golden', testMatch: 'golden.spec.ts' },
    {
      name: 'golden-pilot',
      testMatch: 'golden.spec.ts',
      grep: new RegExp(
        `(${Object.keys(scenes).join('|')}) / initial / studio-light`
      ),
    },
  ],
})
