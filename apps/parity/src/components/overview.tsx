import {
  Callout,
  Card,
  SectionLabel,
  Table,
  TableCell,
  TableHead,
  TableRow,
} from '@dazzlabs/dazzui-react'
import { useMemo } from 'react'

import {
  driftingFields,
  FIELDS,
  type Field,
  type ParityData,
  type Platform,
} from '../lib/model'

export interface OverviewProps {
  data: ParityData
  platforms: readonly Platform[]
  onOpen: (className: string) => void
}

/**
 * Where the drift is: a card per platform with its totals, then the same
 * checks counted by field and by component, so the biggest bar is where
 * the next commit should go.
 */
export function Overview({ data, platforms, onOpen }: OverviewProps) {
  const stats = useMemo(() => {
    const byField = new Map<Field, Map<string, number>>()
    const byClass = new Map<string, Map<string, number>>()
    const totals = new Map<string, { built: number; drifting: number }>()

    for (const platform of platforms) {
      let built = 0
      let drifting = 0
      for (const specimen of data.specimens) {
        if (platform.measured?.[specimen.key]) built += 1
        const fields = driftingFields(specimen, platform)
        drifting += fields.size
        for (const field of fields) {
          const row = byField.get(field) ?? new Map<string, number>()
          row.set(platform.id, (row.get(platform.id) ?? 0) + 1)
          byField.set(field, row)
        }
        if (fields.size > 0) {
          const row =
            byClass.get(specimen.className) ?? new Map<string, number>()
          row.set(platform.id, (row.get(platform.id) ?? 0) + fields.size)
          byClass.set(specimen.className, row)
        }
      }
      totals.set(platform.id, { built, drifting })
    }
    return { byField, byClass, totals }
  }, [data, platforms])

  const max = (rows: Map<string, Map<string, number>>) =>
    Math.max(1, ...[...rows.values()].flatMap(row => [...row.values()]))
  const fieldMax = max(stats.byField as Map<string, Map<string, number>>)
  const classMax = max(stats.byClass)

  const unmeasured = platforms.filter(platform => !platform.measured)

  return (
    <div className="pv-overview">
      {unmeasured.length > 0 && (
        <Callout tint="warning" title="Some platforms have not been measured">
          {unmeasured.map(p => p.label).join(', ')}: run the platform's
          reference test to write <code>measured.json</code>, then reload. Until
          then only its drift list is known, not the values on each side.
        </Callout>
      )}

      <div className="pv-overview__platforms">
        {platforms.map(platform => {
          const total = stats.totals.get(platform.id) ?? {
            built: 0,
            drifting: 0,
          }
          return (
            <Card key={platform.id} variant="raised" size="large">
              <SectionLabel>{platform.label}</SectionLabel>
              <div className="pv-overview__stats">
                <Stat label="Specimens built" value={total.built} />
                <Stat
                  label="Checks drifting"
                  value={
                    platform.measured ? total.drifting : platform.drift.length
                  }
                  tone={total.drifting > 0 ? 'drift' : 'clear'}
                  tint={total.drifting > 0 ? 'danger' : 'success'}
                />
                <Stat
                  label="Listed in drift.txt"
                  value={platform.drift.length}
                />
              </div>
            </Card>
          )
        })}
      </div>

      <div className="pv-overview__tables">
        <Table>
          <TableHead>
            <TableCell head>By field</TableCell>
            {platforms.map(p => (
              <TableCell key={p.id} head align="end">
                {p.label}
              </TableCell>
            ))}
          </TableHead>
          {FIELDS.map(field => (
            <TableRow key={field}>
              <TableCell>{field}</TableCell>
              {platforms.map(p => (
                <TableCell key={p.id} align="end">
                  <Bar
                    value={stats.byField.get(field)?.get(p.id) ?? 0}
                    max={fieldMax}
                  />
                </TableCell>
              ))}
            </TableRow>
          ))}
        </Table>

        <Table>
          <TableHead>
            <TableCell head>By component</TableCell>
            {platforms.map(p => (
              <TableCell key={p.id} head align="end">
                {p.label}
              </TableCell>
            ))}
          </TableHead>
          {[...stats.byClass.entries()]
            .sort(([a], [b]) => a.localeCompare(b))
            .map(([className, row]) => (
              <TableRow
                key={className}
                interactive
                onClick={() => onOpen(className)}
              >
                <TableCell>{className.replace(/^ff-/, '')}</TableCell>
                {platforms.map(p => (
                  <TableCell key={p.id} align="end">
                    <Bar value={row.get(p.id) ?? 0} max={classMax} />
                  </TableCell>
                ))}
              </TableRow>
            ))}
        </Table>
      </div>
    </div>
  )
}

function Stat({
  label,
  value,
  tone,
  tint,
}: {
  label: string
  value: number
  tone?: 'drift' | 'clear'
  tint?: 'danger' | 'success'
}) {
  return (
    <div className="pv-stat">
      <span className="pv-stat__label">{label}</span>
      <span className="pv-stat__value" data-tone={tone} data-tint={tint}>
        {value}
      </span>
    </div>
  )
}

/** A count on a bar scaled to the largest in its table. */
function Bar({ value, max }: { value: number; max: number }) {
  if (value === 0) return <span className="pv-mono">0</span>
  return (
    <span className="pv-bar" data-tint="danger">
      <span
        className="pv-bar__fill"
        style={{ width: `${(value / max) * 100}%` }}
      />
      <span className="pv-bar__value">{value}</span>
    </span>
  )
}
