import {
  ActionBar,
  Aside,
  Badge,
  Divider,
  FormField,
  Icon,
  IconButton,
  TextField,
  KeyCap,
  SectionLabel,
  Select,
  Textarea,
  type Tint,
  TINTS,
} from '@dazzlabs/dazzui-react'
import {
  AlignBottom20Regular,
  AlignCenterHorizontal20Regular,
  AlignCenterVertical20Regular,
  AlignLeft20Regular,
  AlignRight20Regular,
  AlignTop20Regular,
  Copy20Regular,
  Delete20Regular,
  PositionBackward20Regular,
  PositionForward20Regular,
} from '@fluentui/react-icons'
import { type ReactNode, useState } from 'react'

import { SHORTCUTS } from '../data/shortcuts'
import { union } from '../lib/geometry'
import type { Edge, Page, Shape, ShapeVariant } from '../lib/scene'

const ALIGNMENTS: {
  edge: Edge
  label: string
  icon: typeof AlignLeft20Regular
}[] = [
  { edge: 'left', label: 'Align left', icon: AlignLeft20Regular },
  {
    edge: 'center',
    label: 'Align centres',
    icon: AlignCenterHorizontal20Regular,
  },
  { edge: 'right', label: 'Align right', icon: AlignRight20Regular },
  { edge: 'top', label: 'Align top', icon: AlignTop20Regular },
  {
    edge: 'middle',
    label: 'Align middles',
    icon: AlignCenterVertical20Regular,
  },
  { edge: 'bottom', label: 'Align bottom', icon: AlignBottom20Regular },
]

const VARIANTS: { value: ShapeVariant; label: string }[] = [
  { value: 'filled', label: 'Filled' },
  { value: 'tinted', label: 'Tinted' },
  { value: 'outlined', label: 'Outlined' },
]

export interface InspectAsideProps {
  page: Page
  selected: readonly Shape[]
  /** The tint a shape drawn next will take. */
  tint: Tint
  onTintChange: (tint: Tint) => void
  /** One change to every selected shape, as one step in the history. */
  onPatch: (change: Partial<Shape>) => void
  /**
   * The same change, drafted while a field is being typed into. A name typed
   * a letter at a time is one thing that happened, not eleven, so the field
   * opens a step on focus and records it when the caret leaves.
   */
  typing: {
    start: () => void
    draft: (change: Partial<Shape>) => void
    end: () => void
  }
  onAlign: (edge: Edge) => void
  onOrder: (direction: 1 | -1) => void
  onDuplicate: () => void
  onDelete: () => void
}

/**
 * The trailing column: what the selection *is*.
 *
 * It carries no navigation, which is the whole distinction the design system
 * draws between an aside and the two columns on the other edge — those say
 * where you can go, this says what you are looking at. With nothing selected
 * it describes the page instead of emptying, because a column that comes and
 * goes moves the canvas under the pointer every time you deselect.
 */
export function InspectAside({
  page,
  selected,
  tint,
  onTintChange,
  onPatch,
  typing,
  onAlign,
  onOrder,
  onDuplicate,
  onDelete,
}: InspectAsideProps) {
  const one = selected.length === 1 ? selected[0] : null
  const box = union(selected)

  if (!box) {
    return (
      <Aside className="cv-aside">
        <Group label="Page">
          <div className="cv-rows">
            <Row label="Name" value={page.name} />
            <Row label="Layers" value={String(page.shapes.length)} />
          </div>
        </Group>

        <Divider />

        <Group label="New shapes">
          <Swatches value={tint} onChange={onTintChange} />
          <p className="cv-aside__note">
            The tint a shape is drawn in. It is the design system's own axis —
            the same one a button takes — so the drawing follows the theme.
          </p>
        </Group>

        <Divider />

        <Group label="Keys">
          <dl className="cv-keys">
            {SHORTCUTS.map(shortcut => (
              <div key={shortcut.label} className="cv-keys__row">
                <dt>{shortcut.label}</dt>
                <dd>
                  <KeyCap size="small" variant="key">
                    {shortcut.keys}
                  </KeyCap>
                </dd>
              </div>
            ))}
          </dl>
        </Group>
      </Aside>
    )
  }

  const textual = one && one.kind !== 'frame'
  const tintable = one && one.kind !== 'frame'
  const roundable = one && one.kind !== 'ellipse' && one.kind !== 'text'

  return (
    <Aside className="cv-aside">
      <Group label={one ? 'Layer' : 'Selection'}>
        {one ? (
          <>
            <TextField
              aria-label="Name"
              size="small"
              value={one.name}
              onBlur={typing.end}
              onChange={event => typing.draft({ name: event.target.value })}
              onFocus={typing.start}
            />
            <span className="cv-aside__kind">
              <Badge size="small" tint="neutral" variant="outlined">
                {one.kind}
              </Badge>
            </span>
          </>
        ) : (
          <p className="cv-aside__note">
            {selected.length} shapes. Their box is below; the tools act on all
            of them.
          </p>
        )}
      </Group>

      <Divider />

      <Group label={one ? 'Position' : 'Bounding box'}>
        <div className="cv-pairs">
          <NumberField
            label="X"
            value={box.x}
            onCommit={x => onPatch({ x })}
            readOnly={!one}
          />
          <NumberField
            label="Y"
            value={box.y}
            onCommit={y => onPatch({ y })}
            readOnly={!one}
          />
          <NumberField
            label="W"
            value={box.width}
            onCommit={width => onPatch({ width })}
            readOnly={!one}
          />
          <NumberField
            label="H"
            value={box.height}
            onCommit={height => onPatch({ height })}
            readOnly={!one}
          />
        </div>
      </Group>

      <Divider />

      <Group label="Arrange">
        {/* Aligning is a thing you do to a group; with one shape selected
            there is nothing to line it up with but itself. */}
        <ActionBar>
          {ALIGNMENTS.map(entry => (
            <IconButton
              key={entry.edge}
              disabled={selected.length < 2}
              label={entry.label}
              size="small"
              variant="plain"
              onClick={() => onAlign(entry.edge)}
            >
              <Icon icon={entry.icon} size={15} />
            </IconButton>
          ))}
        </ActionBar>

        <ActionBar>
          <IconButton
            label="Bring forward"
            size="small"
            variant="plain"
            onClick={() => onOrder(1)}
          >
            <Icon icon={PositionForward20Regular} size={15} />
          </IconButton>
          <IconButton
            label="Send backward"
            size="small"
            variant="plain"
            onClick={() => onOrder(-1)}
          >
            <Icon icon={PositionBackward20Regular} size={15} />
          </IconButton>
          <IconButton
            label="Duplicate"
            size="small"
            variant="plain"
            onClick={onDuplicate}
          >
            <Icon icon={Copy20Regular} size={15} />
          </IconButton>
          <IconButton
            label="Delete"
            size="small"
            tint="danger"
            variant="plain"
            onClick={onDelete}
          >
            <Icon icon={Delete20Regular} size={15} />
          </IconButton>
        </ActionBar>
      </Group>

      {one ? (
        <>
          <Divider />

          <Group label="Appearance">
            {tintable ? (
              <Swatches value={one.tint} onChange={t => onPatch({ tint: t })} />
            ) : null}

            {/* A frame is paper and a text is ink; neither has a fill to pick,
                so the row is not there rather than being there and disabled. */}
            {one.kind !== 'frame' && one.kind !== 'text' ? (
              <FormField label="Fill">
                <Select
                  size="small"
                  value={one.variant}
                  onChange={event =>
                    onPatch({ variant: event.target.value as ShapeVariant })
                  }
                >
                  {VARIANTS.map(entry => (
                    <option key={entry.value} value={entry.value}>
                      {entry.label}
                    </option>
                  ))}
                </Select>
              </FormField>
            ) : null}

            <div className="cv-pairs">
              {roundable ? (
                <NumberField
                  label="Corner"
                  value={one.radius}
                  onCommit={radius => onPatch({ radius })}
                />
              ) : null}
              {textual ? (
                <NumberField
                  label="Type"
                  value={one.size}
                  onCommit={size => onPatch({ size })}
                />
              ) : null}
              <NumberField
                label="Opacity"
                value={one.opacity}
                onCommit={opacity =>
                  onPatch({ opacity: Math.min(100, Math.max(0, opacity)) })
                }
              />
            </div>
          </Group>

          {textual ? (
            <>
              <Divider />

              <Group label="Text">
                <Textarea
                  aria-label="Text"
                  className="cv-aside__text"
                  size="small"
                  value={one.text}
                  onBlur={typing.end}
                  onChange={event => typing.draft({ text: event.target.value })}
                  onFocus={typing.start}
                />
                <p className="cv-aside__note">
                  Or double-click the shape and type on the canvas.
                </p>
              </Group>
            </>
          ) : null}
        </>
      ) : null}
    </Aside>
  )
}

function Group({ label, children }: { label: string; children: ReactNode }) {
  return (
    <section className="cv-group">
      <SectionLabel>{label}</SectionLabel>
      {children}
    </section>
  )
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="cv-row">
      <span className="cv-row__label">{label}</span>
      <span className="cv-row__value">{value}</span>
    </div>
  )
}

interface NumberFieldProps {
  label: string
  value: number
  onCommit: (value: number) => void
  readOnly?: boolean
}

/**
 * A number the canvas owns and the keyboard may borrow.
 *
 * It holds a draft string while it is being typed into, because the value it
 * shows is rounded off a float the canvas is free to change under it: without
 * the draft, clearing the field to type a new number would put the old one
 * straight back, and a minus sign would never survive being typed.
 */
function NumberField({
  label,
  value,
  onCommit,
  readOnly = false,
}: NumberFieldProps) {
  const [draft, setDraft] = useState<string | null>(null)

  return (
    <FormField label={label}>
      <TextField
        mono
        readOnly={readOnly}
        size="small"
        value={draft ?? String(Math.round(value))}
        onBlur={() => setDraft(null)}
        onChange={event => {
          setDraft(event.target.value)
          const next = Number(event.target.value)
          if (Number.isFinite(next)) onCommit(next)
        }}
        onKeyDown={event => {
          if (event.key === 'Enter') event.currentTarget.blur()
        }}
      />
    </FormField>
  )
}

/** The six ramps, as the six colours this canvas can be drawn in. */
function Swatches({
  value,
  onChange,
}: {
  value: Tint
  onChange: (tint: Tint) => void
}) {
  return (
    <div aria-label="Tint" className="cv-swatches" role="radiogroup">
      {TINTS.map(entry => (
        <button
          key={entry}
          aria-checked={entry === value}
          aria-label={entry}
          className="cv-swatch"
          data-tint={entry}
          role="radio"
          title={entry}
          type="button"
          onClick={() => onChange(entry)}
        />
      ))}
    </div>
  )
}
