# @dazzlabs/dazzui

Design tokens for DazzUI, built with [Terrazzo](https://terrazzo.app/).

## Overview

This package contains design tokens that can be used across multiple platforms:

- **CSS** - for web applications
- **Flutter** - Dart sources for the Flutter package

## Building Tokens

From this directory:

```bash
pnpm build
```

Or from the workspace root:

```bash
pnpm tokens
```

This will generate:

- `dist/style.css` - CSS custom properties
- `dist/flutter/*.dart` - Dart sources for the Flutter package

## Token Categories

### Base Tokens

#### Primitives

- **Colors** (`colors.json`): Full color palettes with shades from 50-950
  - Chromatic: red, orange, amber, yellow, lime, green, emerald, teal, cyan, sky, blue, indigo, violet, purple, fuchsia, pink, rose
  - Neutral: slate, gray, zinc, neutral, stone
  - Base: black, white
- **Borders** (`borders.json`): Border-related tokens
  - Border Radius: xs, sm, md, lg, xl, 2xl, 3xl, 4xl
  - Border Width: Configurable border widths
- **Effects** (`effects.json`): Shadow and visual effects
  - Shadow: Box shadows for elevation (2xs, xs, sm, md, lg, xl, 2xl)
  - Drop Shadow: Drop shadow effects for images and icons (xs, sm, md, lg, xl, 2xl)
  - Text Shadow: Subtle text shadow effects (2xs, xs, sm, md, lg)
  - Inset Shadow: Inner shadows for depth (2xs, xs, sm)
- **Sizing** (`sizing.json`): Size and dimension tokens
  - Container: Max-width values for responsive containers (3xs to 7xl)
  - Size: General sizing tokens
- **Spacing** (`spacing.json`): Consistent spacing scale
  - Numeric scale: 0-96 (0rem to 24rem)
  - Fractional values: 0.5, 1.5, 2.5, 3.5
  - Special: px (1px), base (0.25rem)
- **Typography** (`typography.json`): Font-related tokens
  - Font Families: sans, serif, mono
  - Font Sizes: xs to 9xl with corresponding line heights
  - Font Weights: thin (100) to black (900)
  - Line Heights (leading): tight, snug, normal, relaxed, loose
  - Letter Spacing: tighter, tight, normal, wide, wider, widest

#### Semantics

- **Colors** (`colors.json`): Semantic color tokens for interactive elements
  - Interactive states: filled, tinted, outlined, plain
  - State variations: default, hover, active, focus, disabled
- **Spacing** (`spacing.json`): Semantic spacing for interactive elements
  - Padding: block (tight, normal, loose) and inline (tight, normal, loose)
- **Typography** (`typography.json`): Pre-composed typography styles
  - Display: displaySmall, displayMedium, displayLarge
  - Headline: headlineSmall, headlineMedium, headlineLarge
  - Title: titleSmall, titleMedium, titleLarge
  - Body: bodySmall, bodyMedium, bodyLarge
  - Label: labelSmall, labelMedium, labelLarge
  - Caption: captionSmall, captionMedium, captionLarge

### Component Tokens

- **Button** (`button.json`): Button component tokens
  - Primary: background, text, border-radius
  - Secondary: background, text
- **Card** (`card.json`): Card component tokens (currently empty)

## Token Structure

Tokens are organized in the following structure:

```
tokens/
├── base/
│   ├── primitives/     # Raw values (colors, spacing, etc.)
│   └── semantics/      # Semantic references to primitives
└── components/         # Component-specific tokens
```

## Development

### Browse Tokens

Every token is documented in the Storybook gallery. From the workspace root:

```bash
pnpm storybook
```

### Lint and Format

Linting and formatting are configured once for the whole workspace. Run them
from the repository root:

```bash
pnpm -w lint
```

```bash
pnpm -w format
```

## License

MIT
