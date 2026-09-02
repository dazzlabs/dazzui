import {
  Button,
  Icon,
  IconButton,
  KeyCap,
  SegmentedControl,
  Select,
  Textarea,
} from '@dazzlabs/dazzui-react'
import { type FormEvent, type KeyboardEvent, useState } from 'react'

import { ClipGlyph, SendGlyph, StopGlyph } from '../data/icons'

export type Mode = 'ask' | 'build'

const MODES = [
  { value: 'ask' as const, label: 'Ask' },
  { value: 'build' as const, label: 'Build' },
]

const MODELS = ['forge-1-pro', 'forge-1', 'forge-1-mini']

export interface ComposerProps {
  running: boolean
  /** Whether a write still stops for a permission sheet. */
  asks: boolean
  model: string
  onModelChange: (model: string) => void
  mode: Mode
  onModeChange: (mode: Mode) => void
  onSend: (prompt: string) => void
  onStop: () => void
  /** Filled in by the empty state when a suggestion is picked. */
  draft: string
  onDraftChange: (draft: string) => void
}

export function Composer({
  running,
  asks,
  model,
  onModelChange,
  mode,
  onModeChange,
  onSend,
  onStop,
  draft,
  onDraftChange,
}: ComposerProps) {
  const [attached, setAttached] = useState(0)
  const ready = draft.trim().length > 0 && !running

  const submit = (event?: FormEvent) => {
    event?.preventDefault()
    if (!ready) return
    onSend(draft.trim())
    onDraftChange('')
  }

  const onKeyDown = (event: KeyboardEvent<HTMLTextAreaElement>) => {
    // Enter sends and Shift+Enter breaks the line — the other way round costs
    // a keystroke on every message to save one on the rare paragraph.
    if (event.key !== 'Enter' || event.shiftKey) return
    event.preventDefault()
    submit()
  }

  return (
    <form className="ca-composer" onSubmit={submit}>
      <div data-tint="primary" className="ca-composer__box">
        <Textarea
          aria-label="Message"
          className="ca-composer__input"
          placeholder="Ask for a change, or describe what is wrong…"
          value={draft}
          onChange={event => onDraftChange(event.target.value)}
          onKeyDown={onKeyDown}
        />

        <div className="ca-composer__row">
          <Select
            aria-label="Model"
            className="ca-composer__model"
            size="small"
            value={model}
            onChange={event => onModelChange(event.target.value)}
          >
            {MODELS.map(name => (
              <option key={name} value={name}>
                {name}
              </option>
            ))}
          </Select>

          <SegmentedControl
            aria-label="Mode"
            items={MODES}
            size="small"
            value={mode}
            onValueChange={onModeChange}
          />

          <span className="ca-composer__gap" />

          {attached ? (
            <span className="ca-composer__attached">{attached} attached</span>
          ) : null}
          <IconButton
            label="Attach a file"
            onClick={() => setAttached(count => count + 1)}
          >
            <Icon icon={ClipGlyph} size={16} />
          </IconButton>

          {running ? (
            <Button
              size="small"
              tint="danger"
              variant="tinted"
              onClick={onStop}
            >
              <Icon icon={StopGlyph} size={14} />
              Stop
            </Button>
          ) : (
            <Button
              disabled={!ready}
              size="small"
              type="submit"
              variant="filled"
            >
              <Icon icon={SendGlyph} size={14} />
              Send
            </Button>
          )}
        </div>
      </div>

      <p className="ca-composer__hint">
        <KeyCap size="small" variant="key">
          ⏎
        </KeyCap>
        to send
        <span aria-hidden>·</span>
        <KeyCap size="small" variant="key">
          ⇧⏎
        </KeyCap>
        for a new line
        <span aria-hidden>·</span>
        {mode === 'ask'
          ? 'Ask mode reads and explains; it never writes.'
          : asks
            ? 'Build mode writes files, and asks before the first one.'
            : 'Build mode writes files, and permissions are turned off.'}
      </p>
    </form>
  )
}
