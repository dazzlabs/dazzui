import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'

/**
 * The trailing information column. It carries what the current selection *is*,
 * where the sidebar and the rail carry what you can navigate to — which is why
 * it hangs off the opposite edge and takes no rows.
 */
export function Aside({
  className,
  ...props
}: ComponentPropsWithoutRef<'aside'>) {
  return <aside className={cx('dz-aside', className)} {...props} />
}
