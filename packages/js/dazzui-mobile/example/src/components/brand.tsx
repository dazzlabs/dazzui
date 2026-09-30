import { Badge } from '@dazzlabs/dazzui'

/** The name, and which of its implementations this is. */
export function Brand({ onClick }: { onClick: () => void }) {
  return (
    <button type="button" className="g-brand" onClick={onClick}>
      <span className="g-brand__mark" />
      DazzUI
      <Badge size="small" variant="tinted">
        Mobile
      </Badge>
    </button>
  )
}
