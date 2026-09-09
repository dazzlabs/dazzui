import {
  Dialog,
  Icon,
  type IconGlyph,
  KeyCap,
  SearchField,
} from '@dazzlabs/dazzui-react'
import { useMemo, useState } from 'react'

export interface Command {
  id: string
  label: string
  /** What kind of thing it is: `Go to`, `Session`, `Project`, `Run`. */
  group: string
  glyph: IconGlyph
  /** The binding, where the command has one. */
  keys?: string
  run: () => void
}

export interface CommandPaletteProps {
  commands: readonly Command[]
  onClose: () => void
}

/**
 * ⌘K: everything the window can do, in one list.
 *
 * It is a `Dialog` rather than a popover because it takes the window over —
 * which is what makes it worth reaching for — and Base UI's focus trap is
 * what keeps the arrow keys inside it. The field filters; Return runs the
 * first match, so the common case is three keystrokes and never a pointer.
 */
export function CommandPalette({ commands, onClose }: CommandPaletteProps) {
  const [query, setQuery] = useState('')

  const found = useMemo(() => {
    const needle = query.trim().toLowerCase()
    if (!needle) return commands
    return commands.filter(command =>
      `${command.group} ${command.label}`.toLowerCase().includes(needle)
    )
  }, [commands, query])

  const run = (command: Command) => {
    command.run()
    onClose()
  }

  return (
    <Dialog
      open
      className="wb-palette-sheet"
      width="34rem"
      onOpenChange={open => {
        if (!open) onClose()
      }}
    >
      <form
        className="wb-palette-form"
        onSubmit={event => {
          event.preventDefault()
          if (found[0]) run(found[0])
        }}
      >
        <SearchField
          autoFocus
          className="wb-palette-field"
          placeholder="Go to a page, a session, a project…"
          value={query}
          onValueChange={setQuery}
        />
      </form>

      <div className="wb-palette-list">
        {found.length === 0 ? (
          <span className="wb-palette-empty">
            Nothing matches “{query.trim()}”.
          </span>
        ) : null}
        {found.map((command, index) => (
          <button
            key={command.id}
            aria-current={index === 0 || undefined}
            className="wb-palette-row"
            type="button"
            onClick={() => run(command)}
          >
            <Icon icon={command.glyph} size={15} />
            <span className="wb-palette-group">{command.group}</span>
            <span className="wb-truncate">{command.label}</span>
            {command.keys ? (
              <KeyCap size="small" variant="hint">
                {command.keys}
              </KeyCap>
            ) : null}
          </button>
        ))}
      </div>
    </Dialog>
  )
}
