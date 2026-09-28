import {
  Badge,
  Card,
  EmptyState,
  SectionLabel,
  Table,
  TableCell,
  TableHead,
  TableRow,
  type Tint,
} from '@dazzlabs/dazzui-react'
import { useMemo } from 'react'

import { Renderings } from './renderings'

import {
  checkName,
  driftingFields,
  FIELDS,
  format,
  isColourField,
  verdict,
  type Field,
  type ParityData,
  type Platform,
  type Specimen,
  type ThemeId,
  type Value,
} from '../lib/model'

export interface SpecimenListProps {
  data: ParityData
  specimens: readonly Specimen[]
  platforms: readonly Platform[]
  theme: ThemeId
  query: string
}

/**
 * One component's specimens, drifting ones first. Each card shows what
 * React drew — cut out of the story's screenshot — and a table of every
 * field with React's value beside each platform's, under the theme the
 * toolbar has selected.
 */
export function SpecimenList({
  data,
  specimens,
  platforms,
  theme,
  query,
}: SpecimenListProps) {
  const needle = query.trim().toLowerCase()
  const rows = useMemo(() => {
    const scored = specimens
      .filter(s => needle === '' || s.key.toLowerCase().includes(needle))
      .map(specimen => ({
        specimen,
        drift: platforms.reduce(
          (n, p) => n + driftingFields(specimen, p).size,
          0
        ),
      }))
    scored.sort(
      (a, b) =>
        b.drift - a.drift || a.specimen.key.localeCompare(b.specimen.key)
    )
    return scored
  }, [specimens, platforms, needle])

  if (rows.length === 0) {
    return <EmptyState title="No specimens match" />
  }

  const className = rows[0].specimen.className
  const drifting = rows.filter(row => row.drift > 0).length

  return (
    <div className="pv-specimens">
      <div className="pv-content__header">
        <h1 className="pv-content__heading">{className.replace(/^ff-/, '')}</h1>
        <span className="pv-content__note">
          {rows.length} specimens · {drifting} drifting
        </span>
      </div>
      {rows.map(({ specimen }) => (
        <SpecimenCard
          key={specimen.key}
          data={data}
          specimen={specimen}
          platforms={platforms}
          theme={theme}
        />
      ))}
    </div>
  )
}

const AXIS_TINT: Record<string, Tint> = {
  danger: 'danger',
  warning: 'warning',
  success: 'success',
  info: 'info',
  neutral: 'neutral',
  primary: 'primary',
}

function SpecimenCard({
  data,
  specimen,
  platforms,
  theme,
}: {
  data: ParityData
  specimen: Specimen
  platforms: readonly Platform[]
  theme: ThemeId
}) {
  const expected = specimen.expected[theme] ?? {}
  const rect = specimen.sample.rects[theme]
  const axes = Object.entries(specimen.data)

  return (
    <Card variant="outlined" size="medium">
      <div className="pv-specimen__header">
        {axes.map(([axis, value]) => (
          <Badge
            key={axis}
            size="small"
            variant={
              axis === 'tint'
                ? 'filled'
                : axis === 'variant'
                  ? 'tinted'
                  : 'outlined'
            }
            tint={axis === 'tint' ? (AXIS_TINT[value] ?? 'neutral') : 'neutral'}
          >
            {value === '' || value === 'true' ? axis : `${axis} ${value}`}
          </Badge>
        ))}
        {specimen.text !== '' && (
          <span className="pv-content__note pv-truncate">
            “{specimen.text}”
          </span>
        )}
        <span className="pv-specimen__key pv-mono">
          {specimen.sample.story}
        </span>
      </div>

      <Renderings
        specimen={specimen}
        platforms={platforms}
        theme={theme}
        react={data.renders}
      />

      <div className="pv-specimen__body">
        {data.screenshots && rect && (
          <div className="pv-crop">
            <SectionLabel faint>React</SectionLabel>
            <div className="pv-crop__well">
              <div
                className="pv-crop__image"
                style={{
                  width: rect.width,
                  height: rect.height,
                  backgroundImage: `url(/screenshots/${theme}/${specimen.sample.story}.png)`,
                  backgroundSize: `${data.viewport.width}px auto`,
                  backgroundPosition: `-${rect.x}px -${rect.y}px`,
                }}
              />
            </div>
            <span className="pv-crop__caption">
              {format('width', rect.width)} × {format('height', rect.height)}
            </span>
          </div>
        )}

        <Table className="pv-specimen__table">
          <TableHead>
            <TableCell head>Field</TableCell>
            <TableCell head>React</TableCell>
            {platforms.map(p => (
              <TableCell key={p.id} head>
                {p.label}
              </TableCell>
            ))}
          </TableHead>
          {FIELDS.map(field => {
            const want = expected[field]
            if (want == null) return null
            return (
              <TableRow key={field}>
                <TableCell>{field}</TableCell>
                <TableCell>
                  <ValueCell field={field} value={want} />
                </TableCell>
                {platforms.map(platform => {
                  const got =
                    platform.measured?.[specimen.key]?.[theme]?.[field]
                  const result = verdict(specimen, field, want, got)
                  const listed = platform.drift.includes(
                    checkName(specimen, field)
                  )
                  return (
                    <TableCell key={platform.id}>
                      <ValueCell
                        field={field}
                        value={got}
                        verdict={result}
                        listed={listed}
                        missing={platform.measured === null}
                      />
                    </TableCell>
                  )
                })}
              </TableRow>
            )
          })}
        </Table>
      </div>
    </Card>
  )
}

function ValueCell({
  field,
  value,
  verdict: result,
  listed,
  missing,
}: {
  field: Field
  value: Value | undefined
  verdict?: 'match' | 'drift' | null
  listed?: boolean
  missing?: boolean
}) {
  if (missing) return <span className="pv-content__note">not run</span>
  const swatch =
    isColourField(field) && typeof value === 'string' && value.startsWith('#')
  return (
    <span
      className="pv-cell pv-mono"
      data-tint="danger"
      data-verdict={result ?? undefined}
      data-listed={result === 'drift' ? String(listed) : undefined}
      title={
        result === 'drift' && !listed
          ? 'Drifts, and is not in drift.txt'
          : undefined
      }
    >
      {swatch && (
        <span className="pv-swatch" style={{ backgroundColor: value }} />
      )}
      {format(field, value)}
    </span>
  )
}
