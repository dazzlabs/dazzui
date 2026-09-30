import { Badge } from '@dazzlabs/dazzui'

/**
 * The name, and which of its implementations this is. Every port has a
 * example of its own, and a window of one looks like a window of the next.
 */
export function Brand({ onClick }: { onClick: () => void }) {
  return (
    <button type="button" className="g-brand" onClick={onClick}>
      <span className="g-brand__mark" />
      DazzUI
      <Badge size="small" variant="tinted">
        React
      </Badge>
    </button>
  )
}
