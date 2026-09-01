import {
  ActionBar,
  Button,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
  DialogScrim,
} from '@dazzlabs/dazzui-react'
import { useEffect } from 'react'

import type { Answer, PendingAsk } from '../lib/player'
import { DiffView } from './diff-view'

/**
 * Wider than `--dialog-width`, which is sized for a sentence and two buttons.
 * This sheet has to show a hunk, and a diff that wraps is not a diff.
 */
const REVIEW_WIDTH = '34rem'

export interface PermissionDialogProps {
  ask: PendingAsk
  onAnswer: (answer: Answer) => void
}

/**
 * The gate in front of the first write.
 *
 * It shows the change rather than describing it, because the only question
 * worth asking here is one the reader can answer from what is on screen.
 */
export function PermissionDialog({ ask, onAnswer }: PermissionDialogProps) {
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      // Escape is a decision, not a dismissal: closing the sheet without an
      // answer would leave the run blocked on a question nobody can see.
      if (event.key === 'Escape') onAnswer('deny')
    }
    document.addEventListener('keydown', onKeyDown)
    return () => document.removeEventListener('keydown', onKeyDown)
  }, [onAnswer])

  return (
    <DialogScrim>
      <Dialog width={REVIEW_WIDTH}>
        <DialogHeader
          subtitle={`${ask.tool} · ${ask.target}`}
          title={ask.question}
        />
        <DialogBody>
          <p className="ca-permission__detail">{ask.detail}</p>
          <div className="ca-permission__preview">
            <DiffView lines={ask.lines} />
          </div>
        </DialogBody>
        <DialogFooter>
          <ActionBar align="end">
            <Button variant="plain" onClick={() => onAnswer('deny')}>
              Deny
            </Button>
            <Button onClick={() => onAnswer('always')}>
              Allow {ask.tool} always
            </Button>
            {/* The safe answer takes the focus, so Return is the safe answer. */}
            <Button autoFocus variant="filled" onClick={() => onAnswer('once')}>
              Allow once
            </Button>
          </ActionBar>
        </DialogFooter>
      </Dialog>
    </DialogScrim>
  )
}
