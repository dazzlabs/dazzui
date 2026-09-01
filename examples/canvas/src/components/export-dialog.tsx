import {
  ActionBar,
  Button,
  Checkbox,
  Dialog,
  DialogBody,
  DialogFooter,
  DialogHeader,
  DialogScrim,
  FieldValue,
  FormField,
  PillTabs,
  SegmentedControl,
} from '@dazzlabs/dazzui-react'
import { useEffect, useState } from 'react'

import type { Rect } from '../lib/geometry'

const FORMATS = [
  { value: 'png', label: 'PNG' },
  { value: 'svg', label: 'SVG' },
  { value: 'pdf', label: 'PDF' },
]

const SCALES = [
  { value: '1', label: '1×' },
  { value: '2', label: '2×' },
  { value: '3', label: '3×' },
]

export interface ExportDialogProps {
  /** What would be written: the selection's box, or the whole page's. */
  box: Rect | null
  /** How many shapes are in it, for the sheet to name what it is exporting. */
  count: number
  whole: boolean
  onClose: () => void
  onExport: (summary: string) => void
}

/**
 * The sheet that would write a file, if this were an app rather than a
 * prototype.
 *
 * It is here because an export is where a canvas's units stop being its own
 * and become somebody else's pixels, and that conversion is worth showing:
 * pick a scale and the readout underneath says what actually comes out.
 */
export function ExportDialog({
  box,
  count,
  whole,
  onClose,
  onExport,
}: ExportDialogProps) {
  const [format, setFormat] = useState('png')
  const [scale, setScale] = useState('2')
  const [background, setBackground] = useState(true)

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') onClose()
    }
    document.addEventListener('keydown', onKeyDown)
    return () => document.removeEventListener('keydown', onKeyDown)
  }, [onClose])

  const factor = Number(scale)
  const width = Math.round((box?.width ?? 0) * factor)
  const height = Math.round((box?.height ?? 0) * factor)

  return (
    <DialogScrim>
      <Dialog>
        <DialogHeader
          subtitle={
            whole
              ? `The whole page — ${count} ${count === 1 ? 'shape' : 'shapes'}`
              : `${count} selected ${count === 1 ? 'shape' : 'shapes'}`
          }
          title="Export"
        />

        <DialogBody className="cv-export">
          <FormField label="Format">
            <SegmentedControl
              items={FORMATS}
              stretch
              value={format}
              onValueChange={setFormat}
            />
          </FormField>

          <FormField label="Scale">
            <PillTabs items={SCALES} value={scale} onValueChange={setScale} />
          </FormField>

          {/* The note slot is an aside on the label — a count, a size — so
              what happens when this is off goes in the label itself. */}
          <Checkbox checked={background} onCheckedChange={setBackground}>
            Include the background, or export it transparent
          </Checkbox>

          <FormField
            hint="Nothing is written. This is a prototype of the interface, not of the encoder."
            label="Output"
          >
            <FieldValue mono>
              {box ? `${width} × ${height} px` : 'Nothing to export'}
            </FieldValue>
          </FormField>
        </DialogBody>

        <DialogFooter>
          <ActionBar align="end">
            <Button variant="plain" onClick={onClose}>
              Cancel
            </Button>
            <Button
              autoFocus
              disabled={!box}
              variant="filled"
              onClick={() =>
                onExport(
                  `${count} ${count === 1 ? 'shape' : 'shapes'} at ${width} × ${height} — ${format.toUpperCase()}`
                )
              }
            >
              Export
            </Button>
          </ActionBar>
        </DialogFooter>
      </Dialog>
    </DialogScrim>
  )
}
