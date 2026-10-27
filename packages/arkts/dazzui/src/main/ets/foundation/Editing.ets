/** Editors report native pre-edit text so popup shortcuts respect the IME. */
export class DazzEditingContext {
  private editors: Set<string> = new Set()
  preview(id: string, value: string): void {
    if (value) this.editors.add(id)
    else this.editors.delete(id)
  }
  clear(id: string): void { this.editors.delete(id) }
  composing(): boolean { return this.editors.size > 0 }
}
