export interface DazzFormFieldRegistration {
  validate: (focus: boolean) => boolean
  reset: () => void
}

export function fieldError(value: string, required: boolean, email: boolean): string {
  if (required && !value.length) return 'Please fill out this field.'
  if (email && value.length && !/^[^\s@]+@[^\s@]+$/.test(value)) return 'Please enter an email address.'
  return ''
}

/** Validation and reset stay with the controls, which own native editing. */
export class DazzFormContext {
  active: boolean = false
  attempted: boolean = false
  private fields: Map<string, DazzFormFieldRegistration> = new Map()
  onSubmit: () => void = () => {}
  register(id: string, field: DazzFormFieldRegistration): void { this.fields.set(id, field) }
  unregister(id: string): void { this.fields.delete(id) }
  submit(): boolean {
    if (!this.active) return false
    this.attempted = true
    let valid = true
    this.fields.forEach((field: DazzFormFieldRegistration) => {
      const result = field.validate(valid)
      if (!result) valid = false
    })
    if (valid) this.onSubmit()
    return valid
  }
  reset(): void {
    this.attempted = false
    this.fields.forEach((field: DazzFormFieldRegistration) => field.reset())
  }
}
