import { Table, TableCell, TableHead, TableRow } from '@dazzlabs/dazzui'
import { useState } from 'react'

import { axesOf, formatValue } from '../lib/format'
import type { Index } from '../lib/index'
import type { Show } from '../lib/lens'
import type { Loaded, Scene, Specimen } from '../lib/model'
import { type Check, type Tally, add, EMPTY, tally } from '../lib/verdict'
import { Chip, Mark } from './chip'

interface Props {
  loaded: Loaded
  index: Index
  scene: Scene
  platforms: readonly string[]
  show: Show
}

/** Whether the lens lets a specimen through. */
function passes(show: Show, tallies: Tally[]) {
  if (show === 'all') return true
  const sum = tallies.reduce(add, { ...EMPTY })
  return show === 'off' ? sum.off + sum.missing > 0 : sum.unread > 0
}

/**
 * Every specimen in the scene as a row, a column per implementation, a chip
 * in each cell. Opening a row lays its properties out, the reference beside
 * each implementation.
 */
export function Specimens({ loaded, index, scene, platforms, show }: Props) {
  const [open, setOpen] = useState<string | undefined>()
  const rows = scene.specimens
    .map(specimen => ({
      specimen,
      checks: platforms.map(p => index.specimen(p, specimen.address)),
    }))
    .map(row => ({ ...row, tallies: row.checks.map(list => tally(list)) }))
    .filter(row => passes(show, row.tallies))

  if (rows.length === 0) {
    return (
      <p className="g-quiet g-none">
        Nothing in this scene the lens lets through.
      </p>
    )
  }

  return (
    <Table className="g-specimens">
      <TableHead>
        <TableCell head className="g-specimens__name">
          Specimen
        </TableCell>
        {platforms.map(p => (
          <TableCell key={p} head align="end">
            {loaded.readings[p]!.title}
          </TableCell>
        ))}
      </TableHead>
      {rows.map(({ specimen, checks, tallies }) => {
        const opened = open === specimen.address
        return (
          <div key={specimen.address} className="g-specimens__group">
            <TableRow
              interactive
              active={opened}
              onClick={() => setOpen(opened ? undefined : specimen.address)}
              aria-expanded={opened}
            >
              <TableCell className="g-specimens__name">
                <span className="g-specimen">
                  <span className="g-specimen__component">
                    {specimen.component}
                  </span>
                  <span className="g-quiet">{axesOf(specimen.axes)}</span>
                  {specimen.text && (
                    <span className="g-specimen__text">“{specimen.text}”</span>
                  )}
                </span>
                <span className="g-address">{specimen.address}</span>
              </TableCell>
              {tallies.map((t, i) => (
                <TableCell key={platforms[i]} align="end">
                  <Chip tally={t} />
                </TableCell>
              ))}
            </TableRow>
            {opened && (
              <Properties
                loaded={loaded}
                specimen={specimen}
                platforms={platforms}
                checks={checks}
              />
            )}
          </div>
        )
      })}
    </Table>
  )
}

/** The reference's value, then each implementation's, marked. */
function Properties({
  loaded,
  platforms,
  checks,
}: {
  loaded: Loaded
  specimen: Specimen
  platforms: readonly string[]
  checks: Check[][]
}) {
  /* Every platform is held to the same list; the first column names it. */
  const properties = checks.find(list => list.length > 0) ?? []
  return (
    <div className="g-properties">
      <Table className="g-properties__table">
        <TableHead>
          <TableCell head>Property</TableCell>
          <TableCell head align="end">
            {loaded.readings[loaded.reference]!.title}
          </TableCell>
          {platforms.map(p => (
            <TableCell key={p} head align="end">
              {loaded.readings[p]!.title}
            </TableCell>
          ))}
        </TableHead>
        {properties.map((check, row) => (
          <TableRow key={check.property}>
            <TableCell>
              <code className="g-property">{check.property}</code>
            </TableCell>
            <TableCell align="end">
              <span className="g-value">{formatValue(check.expected)}</span>
            </TableCell>
            {platforms.map((p, i) => {
              const cell = checks[i]?.[row]
              return (
                <TableCell key={p} align="end">
                  {cell ? (
                    <span className="g-value" data-verdict={cell.verdict}>
                      <Mark verdict={cell.verdict} />
                      {formatValue(cell.actual)}
                    </span>
                  ) : (
                    <span className="g-quiet">—</span>
                  )}
                </TableCell>
              )
            })}
          </TableRow>
        ))}
      </Table>
    </div>
  )
}
