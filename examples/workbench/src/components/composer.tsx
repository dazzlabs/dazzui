import {
  Badge,
  Button,
  Icon,
  IconButton,
  KeyCap,
  Menu,
  SegmentedControl,
  TextField,
} from '@dazzlabs/dazzui-react'
import type { FormEvent, KeyboardEvent } from 'react'

import {
  AtGlyph,
  ChevronGlyph,
  ClipGlyph,
  SendGlyph,
  StopGlyph,
} from '../data/icons'
import { type Mode, MODELS, MODES } from '../lib/settings'

/** What the field asks for, in each mode. */
const PLACEHOLDER: Record<Mode, string> = {
  ask: 'Ask about the code… nothing will be written',
  plan: 'Say what should change… the agent drafts the plan and waits',
  agent: 'Tell the agent what to do… @ references a file, / runs a command',
  auto: 'Describe the outcome… the agent runs to a ready pull request',
}

export interface ComposerProps {
  running: boolean
  model: string
  onModelChange: (model: string) => void
  /** How far this message lets the agent go. */
  mode: Mode
  onModeChange: (mode: Mode) => void
  /** The one thing the message carries with it, where there is one. */
  reference: string | null
  onSend: (prompt: string) => void
  onStop: () => void
  /** Filled in by the empty state when a suggestion is picked. */
  draft: string
  onDraftChange: (draft: string) => void
}

/**
 * The composer is one control made of several: the reference the message
 * goes out attached to, above the field; the field; and under it the row of
 * things it will be sent *with* — how far it lets the agent go, and which
 * model. The box owns the border and the focus, so the field inside it gives
 * up its own.
 *
 * The mode is here rather than in settings because it is decided per
 * message: the same session asks a question, then wants a plan, then says
 * go. The context meter is not here — it is on the status bar, where it can
 * be read from every page rather than only from this one.
 */
export function Composer({
  running,
  model,
  onModelChange,
  mode,
  onModeChange,
  reference,
  onSend,
  onStop,
  draft,
  onDraftChange,
}: ComposerProps) {
  const ready = draft.trim().length > 0 && !running
  const current = MODELS.find(entry => entry.value === model)
  const chosen = MODES.find(entry => entry.value === mode)

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
    <form className="wb-composer" onSubmit={submit}>
      <div data-tint="primary" className="wb-composer__box">
        {reference ? (
          <span className="wb-composer__refs">
            <Badge className="wb-composer__ref" size="small" tint="neutral">
              <Icon icon={AtGlyph} size={11} />
              <span className="wb-truncate wb-mono">{reference}</span>
            </Badge>
          </span>
        ) : null}

        <TextField
          aria-label="Message"
          className="wb-composer__input"
          multiline
          placeholder={PLACEHOLDER[mode]}
          value={draft}
          onChange={event => onDraftChange(event.target.value)}
          onKeyDown={onKeyDown}
        />

        <div className="wb-composer__row">
          <span className="wb-composer__with">
            <SegmentedControl
              aria-label="Mode"
              className="wb-composer__mode"
              items={MODES.map(entry => ({
                value: entry.value,
                label: entry.label,
              }))}
              size="small"
              title={chosen?.hint}
              value={mode}
              onValueChange={onModeChange}
            />

            <Menu
              align="start"
              items={MODELS.map(entry => ({
                label: entry.label,
                checked: entry.value === model,
                onSelect: () => onModelChange(entry.value),
              }))}
              trigger={
                <Button size="tiny" tint="neutral" variant="outlined">
                  {current?.label ?? 'Model'}
                  <Icon
                    icon={ChevronGlyph}
                    size={11}
                    className="wb-composer__caret"
                  />
                </Button>
              }
            />
          </span>

          <IconButton
            label="Attach a file"
            size="small"
            tint="neutral"
            variant="plain"
          >
            <Icon icon={ClipGlyph} size={15} />
          </IconButton>

          {running ? (
            <IconButton
              label="Stop the run"
              size="medium"
              tint="danger"
              variant="tinted"
              onClick={onStop}
            >
              <Icon icon={StopGlyph} size={15} />
            </IconButton>
          ) : (
            <IconButton
              disabled={!ready}
              label="Send"
              size="medium"
              type="submit"
              variant="filled"
            >
              <Icon icon={SendGlyph} size={15} />
            </IconButton>
          )}
        </div>
      </div>

      <p className="wb-composer__hint">
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
        <span className="wb-truncate">{chosen?.hint}</span>
      </p>
    </form>
  )
}
