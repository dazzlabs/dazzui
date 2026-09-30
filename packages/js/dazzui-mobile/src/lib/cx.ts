type ClassValue = string | false | null | undefined

/**
 * Join the truthy class names. A component puts its own class first and the
 * consumer's last, so a consumer's rule of equal specificity wins on source
 * order alone.
 */
export function cx(...values: ClassValue[]): string {
  return values.filter(Boolean).join(' ')
}
