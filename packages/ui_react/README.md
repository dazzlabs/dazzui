# @dazzlabs/dazzui-react

The React implementation of DazzUI. Every value it draws with is decided
in `packages/ds`, which generates `src/generated/` into this package; those
files are committed, so this package builds and installs on its own with no
dependency on the design system.

## Using it

Import both stylesheets — the token sheet is separate on purpose, because it is
the contract, not an implementation detail:

```ts
import '@dazzlabs/dazzui-react/style.css'
import '@dazzlabs/dazzui-react/components.css'
```

```tsx
import { Button, FormField, Input } from '@dazzlabs/dazzui-react'

<FormField label="Endpoint">
  <Input mono defaultValue="https://api.example.com" />
</FormField>
<Button variant="filled">Save</Button>
```

There is no Tailwind here, and no icon set: `Icon` takes any component that
renders an `<svg>`.

## Building

```bash
pnpm build
```

After editing a token, regenerate first — from the workspace root, `pnpm
packages` does both in order.

## Conventions

- One `<name>.tsx` / `<name>.css` pair per component, both listed in their
  registries (`src/components/index.ts`, `src/styles/components.css`).
- Variants are `data-*` attributes, so a selector names the axis it varies.
- A value with no token behind it is a missing token: add it to
  `packages/ds` rather than writing a literal into the CSS.
- Never hand-edit `src/generated/`; the next generator run overwrites it.

See AGENTS.md at the repository root for the rules on tinting and the shared
control state machine.
