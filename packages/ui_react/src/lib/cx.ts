type ClassValue = string | false | null | undefined

/**
 * Join the truthy class names.
 *
 * Components put their own class first and the consumer's last, so a
 * consumer's rule of equal specificity wins on source order alone. There is
 * no utility-class merging to do: a component's design values come from its
 * tokens, not from classes a consumer might have to override.
 */
export function cx(...values: ClassValue[]): string {
  return values.filter(Boolean).join(' ')
}
