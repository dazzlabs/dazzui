import {
  Component,
  createElement,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react'
import { createRoot } from 'react-dom/client'
import { Button, SearchField } from '@dazzlabs/dazzui-react'
import '@dazzlabs/dazzui-react/style.css'
import '@dazzlabs/dazzui-react/components.css'
import './styles/preview.css'
import './styles/playground.css'
import catalog from './generated/catalog.json'
import { applyTheme, DEFAULT_THEME, THEMES } from './lib/themes'
import type { StoryModule, StoryObj } from './lib/story'

const modules = import.meta.glob<StoryModule>('../stories/*.stories.tsx')
const stories = catalog.stories
type Entry = (typeof stories)[number]
type Selection = { story: string; theme: string; revision: number }
type Preview = {
  ready: boolean
  story: string
  theme: string
  layout: string
  error: string | null
  show: (story: string, theme: string) => void
}
declare global {
  interface Window {
    __PLAYGROUND__?: Preview
  }
}

const query = () => new URLSearchParams(location.search)
const themeValue = (value: string | null) =>
  THEMES.some(t => t.value === value) ? value! : DEFAULT_THEME
const initial = (): Selection => ({
  story:
    query().get('story') ?? query().get('id') ?? 'components-button--overview',
  theme: themeValue(query().get('theme')),
  revision: 0,
})

class Boundary extends Component<
  { children: ReactNode },
  { error: string | null }
> {
  state = { error: null as string | null }
  static getDerivedStateFromError(error: Error) {
    return { error: error.message }
  }
  componentDidCatch(error: Error) {
    if (window.__PLAYGROUND__) window.__PLAYGROUND__.error = error.message
  }
  render() {
    return this.state.error ? (
      <p role="alert">{this.state.error}</p>
    ) : (
      this.props.children
    )
  }
}

function Ready({ selection }: { selection: Selection }) {
  useEffect(() => {
    let active = true
    void document.fonts.ready.then(() =>
      requestAnimationFrame(() =>
        requestAnimationFrame(() => {
          if (active && window.__PLAYGROUND__ && !window.__PLAYGROUND__.error)
            window.__PLAYGROUND__.ready = true
        })
      )
    )
    return () => {
      active = false
    }
  }, [selection])
  return null
}

function Canvas() {
  const [selection, setSelection] = useState(initial)
  const [loaded, setLoaded] = useState<{
    module: StoryModule
    selection: Selection
  } | null>(null)
  const [error, setError] = useState<string | null>(null)
  const entry = stories.find(s => s.id === selection.story)
  useEffect(() => {
    let active = true
    setLoaded(null)
    setError(null)
    applyTheme(document.documentElement, selection.theme)
    document.getElementById('root')!.dataset.layout =
      entry?.layout ?? 'fullscreen'
    window.__PLAYGROUND__ = {
      ready: false,
      story: selection.story,
      theme: selection.theme,
      layout: entry?.layout ?? 'fullscreen',
      error: null,
      show: (story, theme) => {
        if (window.__PLAYGROUND__) window.__PLAYGROUND__.ready = false
        setSelection(s => ({
          story,
          theme: themeValue(theme),
          revision: s.revision + 1,
        }))
      },
    }
    const load = entry && modules[`../${entry.source}`]
    if (!load) {
      const message = `Unknown story: ${selection.story}`
      window.__PLAYGROUND__.error = message
      setError(message)
      return
    }
    void load()
      .then(module => {
        if (active) setLoaded({ module, selection })
      })
      .catch(error => {
        if (!active) return
        const message = String(error)
        if (window.__PLAYGROUND__) window.__PLAYGROUND__.error = message
        setError(message)
      })
    return () => {
      active = false
    }
  }, [selection, entry])
  if (error) return <p role="alert">{error}</p>
  if (!loaded || loaded.selection !== selection || !entry) return null
  const story = loaded.module[entry.exportName] as StoryObj
  let Render = story.render
  for (const decorate of [
    ...(loaded.module.default.decorators ?? []),
    ...(story.decorators ?? []),
  ]) {
    const Inner = Render
    Render = () => decorate(Inner)
  }
  return (
    <Boundary key={`${selection.story}:${selection.revision}`}>
      <main id="playground-root" data-layout={entry.layout}>
        {createElement(Render)}
        <Ready selection={selection} />
      </main>
    </Boundary>
  )
}

function Playground() {
  const [selection, setSelection] = useState(initial)
  const [search, setSearch] = useState('')
  const [width, setWidth] = useState('fill')
  const current = stories.find(s => s.id === selection.story)
  const pages = useMemo(() => {
    const result = new Map<string, Entry[]>()
    for (const story of stories) {
      if (
        !`${story.title} ${story.name}`
          .toLowerCase()
          .includes(search.toLowerCase())
      )
        continue
      result.set(story.title, [...(result.get(story.title) ?? []), story])
    }
    return result
  }, [search])
  useEffect(() => {
    applyTheme(document.documentElement, selection.theme)
    document.title = `${current?.title ?? 'Unknown story'} · ${current?.name ?? ''} — DazzUI`
  }, [selection, current])
  useEffect(() => {
    const restore = () => setSelection(initial())
    window.addEventListener('popstate', restore)
    return () => window.removeEventListener('popstate', restore)
  }, [])
  const navigate = (story: string, theme = selection.theme) => {
    const url = new URL(location.href)
    url.search = new URLSearchParams({ story, theme }).toString()
    history.pushState(null, '', url)
    setSelection(s => ({ story, theme, revision: s.revision + 1 }))
  }
  const canvas = `./index.html?${new URLSearchParams({ canvas: '1', story: selection.story, theme: selection.theme })}`
  return (
    <div className="pg-shell">
      <aside className="pg-sidebar">
        <div className="pg-brand">
          DazzUI <span>React</span>
        </div>
        <SearchField
          value={search}
          onValueChange={setSearch}
          placeholder="Find a story"
        />
        <nav aria-label="Stories">
          {pages.size === 0 && <p>No matching stories.</p>}
          {[...pages].map(([title, entries]) => (
            <details key={title} open={!!search || title === current?.title}>
              <summary>{title}</summary>
              {entries.map(story => (
                <a
                  key={story.id}
                  href={`?story=${story.id}&theme=${selection.theme}`}
                  aria-current={
                    story.id === selection.story ? 'page' : undefined
                  }
                  onClick={event => {
                    if (
                      event.metaKey ||
                      event.ctrlKey ||
                      event.shiftKey ||
                      event.altKey
                    )
                      return
                    event.preventDefault()
                    navigate(story.id)
                  }}
                >
                  {story.name}
                </a>
              ))}
            </details>
          ))}
        </nav>
      </aside>
      <section className="pg-workspace">
        <header className="pg-toolbar">
          <span className="pg-title">
            {current ? `${current.title} / ${current.name}` : 'Unknown story'}
          </span>
          <label>
            Theme{' '}
            <select
              aria-label="Theme"
              value={selection.theme}
              onChange={event => navigate(selection.story, event.target.value)}
            >
              {THEMES.map(theme => (
                <option key={theme.value} value={theme.value}>
                  {theme.title}
                </option>
              ))}
            </select>
          </label>
          <label>
            Canvas{' '}
            <select
              aria-label="Canvas"
              value={width}
              onChange={event => setWidth(event.target.value)}
            >
              <option value="fill">Fill</option>
              <option value="960">960 × 640</option>
              <option value="390">390 × 844</option>
            </select>
          </label>
          <Button
            onClick={() =>
              setSelection(s => ({ ...s, revision: s.revision + 1 }))
            }
          >
            Reset
          </Button>
          <a href={canvas} target="_blank" rel="noreferrer">
            Open canvas
          </a>
        </header>
        <div className="pg-stage" data-viewport={width}>
          <iframe
            key={`${selection.story}:${selection.theme}:${selection.revision}`}
            title="Story canvas"
            src={canvas}
          />
        </div>
      </section>
    </div>
  )
}

createRoot(document.getElementById('root')!).render(
  query().has('canvas') ? <Canvas /> : <Playground />
)
