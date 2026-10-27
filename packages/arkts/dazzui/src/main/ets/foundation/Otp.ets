/** Numeric OTP edits stay contiguous, as in Base UI's default validation mode. */
export function normalizeOtp(value: string, length: number): string {
  const count = Number.isFinite(length) ? Math.max(0, Math.floor(length)) : 0
  return value.replace(/\s/g, '').replace(/[^\d]/g, '').slice(0, count)
}

export function replaceOtp(
  current: string, index: number, inserted: string, length: number
): string {
  const digits = normalizeOtp(inserted, length)
  return normalizeOtp(
    current.slice(0, index) + digits + current.slice(index + digits.length),
    length
  )
}

export function removeOtp(current: string, index: number): string {
  if (index < 0 || index >= current.length) return current
  return current.slice(0, index) + current.slice(index + 1)
}
