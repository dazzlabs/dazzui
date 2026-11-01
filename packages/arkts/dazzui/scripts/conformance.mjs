import { spawnSync } from 'node:child_process'

const [scene = 'button/preview', theme = 'bright-light'] = process.argv.slice(2)
const result = spawnSync(
  process.env.HDC ?? 'hdc',
  [
    'shell',
    'aa',
    'start',
    '-b',
    'dev.dazzlabs.dazzui.playground',
    '-a',
    'EntryAbility',
    '--pb',
    'conformance',
    'true',
    '--ps',
    'sceneId',
    scene,
    '--ps',
    'themeName',
    theme,
  ],
  { stdio: 'inherit' }
)
if (result.error)
  throw new Error(
    `A connected HarmonyOS PC with the signed gallery installed is required: ${result.error.message}`
  )
if (result.status !== 0) process.exit(result.status ?? 1)
console.log(
  'Use Capture geometry and image in the native capture page. Export its JSON and PNG from the app files directory through DevEco Studio. These are native captures; they have not joined the shared conformance baselines.'
)
