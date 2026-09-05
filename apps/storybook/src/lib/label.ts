/**
 * A story that prints an axis value as the specimen's own text — a button
 * that says which variant it is — shows it the way a real label would be
 * set, capital first. The prop value stays as written; only the copy moves.
 */
export function label(value: string): string {
  return value.charAt(0).toUpperCase() + value.slice(1)
}
