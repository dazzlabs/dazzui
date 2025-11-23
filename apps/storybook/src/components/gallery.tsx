import {
  type CSSProperties,
  type ReactNode,
  useLayoutEffect,
  useState,
} from 'react'

import { resolve, type Token } from '../lib/tokens'
import './gallery.css'

/**
 * Resolved values come from `getComputedStyle`, which only answers once the
 * token stylesheet is attached to the document. Re-read after mount so the
 * first paint is not stuck with an empty value.
 */
export function useResolvedValue(property: string): string {
  const [value, setValue] = useState(() => resolve(property))
  useLayoutEffect(() => setValue(resolve(property)), [property])
  return value
}

type PageProps = {
  title: string
  description?: ReactNode
  children: ReactNode
}

export function Page({ title, description, children }: PageProps) {
  return (
    <div className="dt-page">
      <header>
        <h1 className="dt-page__title">{title}</h1>
        {description ? (
          <p className="dt-page__description">{description}</p>
        ) : null}
      </header>
      {children}
    </div>
  )
}

type SectionProps = {
  title: string
  description?: ReactNode
  children: ReactNode
}

export function Section({ title, description, children }: SectionProps) {
  return (
    <section className="dt-section">
      <header className="dt-section__header">
        <h2 className="dt-section__title">{title}</h2>
        {description ? (
          <p className="dt-section__description">{description}</p>
        ) : null}
      </header>
      {children}
    </section>
  )
}

export function Stack({ children }: { children: ReactNode }) {
  return <div className="dt-stack">{children}</div>
}

/** Inline styles that also set custom properties. */
type CssVars = CSSProperties & Record<string, string>

type GridProps = {
  /** Minimum column width; the grid fills the remaining space. */
  min?: string
  children: ReactNode
}

export function Grid({ min = '16rem', children }: GridProps) {
  return (
    <div className="dt-grid" style={{ '--dt-grid-min': min } as CssVars}>
      {children}
    </div>
  )
}

/** Property name, authored value, and the value the browser resolves it to. */
export function TokenMeta({ token, label }: { token: Token; label?: string }) {
  const resolved = useResolvedValue(token.property)
  return (
    <div>
      <div className="dt-name">{label ?? token.property}</div>
      <div className="dt-value">{token.value}</div>
      {resolved && resolved !== token.value ? (
        <div className="dt-value">{resolved}</div>
      ) : null}
    </div>
  )
}

export function Swatch({ token }: { token: Token }) {
  return (
    <div className="dt-swatch">
      <div
        className="dt-swatch__preview"
        style={{ background: `var(${token.property})` }}
      />
      <TokenMeta token={token} />
    </div>
  )
}

/** One step of a color ramp: the swatch plus its shade number. */
export function RampStep({ token, step }: { token: Token; step: string }) {
  const resolved = useResolvedValue(token.property)
  return (
    <div className="dt-swatch dt-swatch--compact">
      <div
        className="dt-swatch__preview"
        style={{ background: `var(${token.property})` }}
      />
      <div>
        <div className="dt-name">{step}</div>
        <div className="dt-value">{resolved || token.value}</div>
      </div>
    </div>
  )
}

type RampProps = {
  label: string
  steps: { step: string; token: Token }[]
}

export function Ramp({ label, steps }: RampProps) {
  return (
    <div>
      <div className="dt-ramp__label">{label}</div>
      <div className="dt-ramp">
        {steps.map(({ step, token }) => (
          <RampStep key={token.name} step={step} token={token} />
        ))}
      </div>
    </div>
  )
}

/** A boxed preview (radius, shadow, …) above the token it demonstrates. */
export function Tile({
  token,
  label,
  children,
}: {
  token: Token
  label?: string
  children: ReactNode
}) {
  return (
    <div className="dt-tile">
      <div className="dt-tile__preview">{children}</div>
      <TokenMeta token={token} label={label} />
    </div>
  )
}

/** A dimension token drawn to scale. */
export function Bar({ token, label }: { token: Token; label: string }) {
  const resolved = useResolvedValue(token.property)
  return (
    <div className="dt-bar">
      <div className="dt-name">{label}</div>
      <div
        className="dt-bar__fill"
        style={{ width: `var(${token.property})`, minWidth: '1px' }}
      />
      <div className="dt-value">{resolved || token.value}</div>
    </div>
  )
}

type TokenTableProps = {
  tokens: Token[]
  /** Shorten the name shown in the first column. */
  labelOf?: (token: Token) => string
  /** Header for the first column. */
  nameHeader?: string
}

export function TokenTable({
  tokens,
  labelOf,
  nameHeader = 'Token',
}: TokenTableProps) {
  if (tokens.length === 0) {
    return <p className="dt-empty">No tokens defined yet.</p>
  }

  return (
    <div className="dt-table__scroll">
      <table className="dt-table">
        <thead>
          <tr>
            <th>{nameHeader}</th>
            <th>Value</th>
            <th>Resolved</th>
          </tr>
        </thead>
        <tbody>
          {tokens.map(token => (
            <TokenRow
              key={token.name}
              token={token}
              label={labelOf?.(token) ?? token.property}
            />
          ))}
        </tbody>
      </table>
    </div>
  )
}

function TokenRow({ token, label }: { token: Token; label: string }) {
  const resolved = useResolvedValue(token.property)
  return (
    <tr>
      <td className="dt-name">{label}</td>
      <td className="dt-value">{token.value}</td>
      <td className="dt-value">{resolved}</td>
    </tr>
  )
}
