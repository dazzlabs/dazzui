import fs from 'node:fs/promises'
import path from 'node:path'
import { execSync } from 'node:child_process'
import YAML from 'yaml'

/**
 * Downloads theme.css (with caching), extracts the @theme default block,
 * and parses all CSS variables into a Map.
 * Returns a Map of variable names (without --) to their values.
 */
async function getCssVariables() {
  const themeCssUrl =
    'https://raw.githubusercontent.com/tailwindlabs/tailwindcss/' +
    'refs/heads/main/packages/tailwindcss/theme.css'

  // Download fresh copy if not loaded from cache
  const res = await fetch(themeCssUrl)

  if (!res.ok) {
    throw new Error(
      `Failed to download theme.css from Tailwind (status ${res.status})`
    )
  }

  let css = await res.text()

  // Extract @theme default block
  const marker = '@theme default'
  const start = css.indexOf(marker)

  if (start === -1) {
    throw new Error(
      'Could not find "@theme default { ... }" block in theme.css'
    )
  }

  const firstBrace = css.indexOf('{', start)
  if (firstBrace === -1) {
    throw new Error('Malformed "@theme default" block')
  }

  let depth = 1
  let i = firstBrace + 1

  for (; i < css.length; i++) {
    const ch = css[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) break
    }
  }

  if (depth !== 0) {
    throw new Error('Unbalanced braces in "@theme default" block')
  }

  // Extract inner content, without the outer braces
  const block = css.slice(firstBrace + 1, i)

  // Parse all CSS custom properties from the block
  const vars = new Map()
  // Match CSS custom properties, including multi-line values
  // The regex uses [\s\S] to match any character including newlines
  const re = /--([a-z0-9-]+)\s*:\s*([\s\S]*?);/gi
  let match

  while ((match = re.exec(block)) !== null) {
    const name = match[1] // without leading `--`
    // Clean up the value: remove extra whitespace but preserve structure
    const rawValue = match[2]
      .split('\n')
      .map(line => line.trim())
      .filter(Boolean)
      .join(' ')
      .trim()
    vars.set(name, rawValue)
  }

  return vars
}

function buildBordersTokens(vars) {
  const borderRadius = {
    $type: 'dimension',
  }
  const borderWidth = {
    $type: 'dimension',
  }

  for (const [name, raw] of vars) {
    if (name.startsWith('radius-')) {
      const key = name.slice('radius-'.length)
      borderRadius[key] = { $value: raw }
      continue
    }

    if (name.startsWith('border-')) {
      const key = name.slice('border-'.length)
      borderWidth[key] = { $value: raw }
      continue
    }
  }

  const borders = {
    radius: borderRadius,
  }

  // Only include border-width if there are any values
  if (Object.keys(borderWidth).length > 0) {
    borders['border-width'] = borderWidth
  }

  return borders
}

/**
 * Handle hex color - return as-is without conversion.
 */
function handleHexColor(hex) {
  // Return hex color directly without conversion
  return hex
}

/**
 * Parse OKLCH color string to DTCG color object format.
 * Input: "oklch(97.1% 0.013 17.38)" or "oklch(97.1% 0.013 17.38 / 0.5)" or "#000"
 * Output: { colorSpace: "oklch", components: [0.971, 0.013, 17.38], alpha: 1 } or "#000"
 */
function parseOklchToObject(colorStr) {
  // Handle hex colors - return as-is
  if (colorStr.startsWith('#')) {
    return handleHexColor(colorStr)
  }

  // Match oklch(...) format with optional alpha
  const match = colorStr.match(
    /oklch\(([\d.]+%?)\s+([\d.]+)\s+([\d.]+)(?:\s*\/\s*([\d.]+))?\)/
  )

  if (!match) {
    console.warn(`Failed to parse OKLCH color: ${colorStr}`)
    return colorStr // Return original if parsing fails
  }

  let [, lightness, chroma, hue, alpha] = match

  // Convert lightness from percentage to decimal (0-1)
  if (lightness.endsWith('%')) {
    lightness = parseFloat(lightness) / 100
  } else {
    lightness = parseFloat(lightness)
  }

  // Chroma and hue are already in correct format
  chroma = parseFloat(chroma)
  hue = parseFloat(hue)

  // Alpha defaults to 1 if not specified
  alpha = alpha ? parseFloat(alpha) : 1

  return {
    colorSpace: 'oklch',
    components: [lightness, chroma, hue],
    alpha: alpha,
  }
}

function buildColorTokens(vars) {
  const color = {
    $type: 'color',
  }

  for (const [name, value] of vars) {
    if (!name.startsWith('color-')) continue

    const rest = name.slice('color-'.length) // e.g. "red-500" or "white"
    const parts = rest.split('-')

    // Convert OKLCH string to object format
    const colorValue = parseOklchToObject(value)

    if (parts.length === 1) {
      // e.g. color-white, color-black, color-transparent
      color[parts[0]] = { $value: colorValue }
    } else {
      const hue = parts[0] // "red"
      const shade = parts.slice(1).join('-') // "500"
      if (!color[hue]) {
        color[hue] = {}
      }
      color[hue][shade] = { $value: colorValue }
    }
  }

  return { color }
}

/**
 * Parse a single CSS shadow string into an object.
 * Handles formats like:
 * - "0 4px 6px -1px rgb(0 0 0 / 0.1)"
 * - "0 1px rgb(0 0 0 / 0.05)"
 * - "inset 0 1px rgb(0 0 0 / 0.05)"
 */
function parseShadowValue(shadowStr) {
  const trimmed = shadowStr.trim()
  if (!trimmed) return null

  let isInset = false
  let remaining = trimmed

  // Check for inset keyword
  if (remaining.startsWith('inset ')) {
    isInset = true
    remaining = remaining.slice(6).trim()
  }

  // Find color value - match rgb/rgba/hsl/hsla functions or hex/named colors
  // Color is typically at the end
  const colorMatch = remaining.match(
    /(rgb|rgba|hsl|hsla)\([^)]+\)|#[0-9a-fA-F]{3,8}|[a-zA-Z]+$/
  )

  let color = ''
  let numericPart = remaining

  if (colorMatch) {
    color = colorMatch[0]
    // Remove color from the string to get numeric parts
    const colorIndex = remaining.lastIndexOf(color)
    numericPart = remaining.slice(0, colorIndex).trim()
  } else {
    // Default color if not found
    color = 'rgb(0, 0, 0, 0.15)'
  }

  // Normalize color format: rgb(0 0 0 / 0.05) -> rgb(0, 0, 0, 0.05)
  color = color.replace(
    /(rgb|rgba|hsl|hsla)\(([^)]+)\)/,
    (match, func, content) => {
      // Convert space-separated with / to comma-separated
      // rgb(0 0 0 / 0.05) -> rgb(0, 0, 0, 0.05)
      // Split by spaces, then join with commas, handling the / separator specially
      const parts = content.trim().split(/\s+/)
      const slashIndex = parts.findIndex(p => p === '/')

      if (slashIndex >= 0) {
        // Format: "0 0 0 / 0.05" -> "0, 0, 0, 0.05"
        const beforeSlash = parts.slice(0, slashIndex).join(', ')
        const afterSlash = parts.slice(slashIndex + 1).join(' ')
        return `${func}(${beforeSlash}, ${afterSlash})`
      } else {
        // No slash, just join with commas
        return `${func}(${parts.join(', ')})`
      }
    }
  )

  // Parse numeric values: offsetX offsetY blur spread
  const parts = numericPart.split(/\s+/).filter(Boolean)

  // Helper to ensure value has units (default to px if it's a plain number)
  const ensureUnit = val => {
    if (!val) return '0px'
    // If it's just a number (possibly negative), add px
    if (/^-?\d+$/.test(val)) {
      return val + 'px'
    }
    return val
  }

  const offsetX = ensureUnit(parts[0])
  const offsetY = ensureUnit(parts[1])
  const blur = parts[2] ? ensureUnit(parts[2]) : '0px'
  const spread = parts[3] ? ensureUnit(parts[3]) : '0px'

  const result = {
    offsetX,
    offsetY,
    blur,
    spread,
    color,
  }

  // Only add inset if true
  if (isInset) {
    result.inset = true
  }

  return result
}

/**
 * Parse CSS shadow value(s) - handles single or multiple shadows.
 * For multiple shadows, takes the first one.
 */
function parseShadow(shadowStr) {
  if (!shadowStr) return null

  // Split by comma to handle multiple shadows
  const shadows = shadowStr
    .split(',')
    .map(s => s.trim())
    .filter(Boolean)

  // For now, parse the first shadow
  // If multiple shadows are needed, this could return an array
  return parseShadowValue(shadows[0])
}

function buildEffectsTokens(vars) {
  const shadow = {
    $type: 'shadow',
  }
  const insetShadow = {
    $type: 'shadow',
  }
  const dropShadow = {
    $type: 'shadow',
  }
  const textShadow = {
    $type: 'shadow',
  }

  for (const [name, raw] of vars) {
    if (name.startsWith('shadow-')) {
      const key = name.slice('shadow-'.length)
      const parsed = parseShadow(raw)
      if (parsed) {
        shadow[key] = { $value: parsed }
      } else {
        console.warn(
          `Failed to parse shadow for ${name}: "${raw}", using raw value`
        )
        shadow[key] = { $value: raw }
      }
      continue
    }

    if (name.startsWith('inset-shadow-')) {
      const key = name.slice('inset-shadow-'.length)
      const parsed = parseShadow(raw)
      if (parsed) {
        insetShadow[key] = { $value: parsed }
      } else {
        console.warn(
          `Failed to parse inset-shadow for ${name}, using raw value`
        )
        insetShadow[key] = { $value: raw }
      }
      continue
    }

    if (name.startsWith('drop-shadow-')) {
      const key = name.slice('drop-shadow-'.length)
      const parsed = parseShadow(raw)
      if (parsed) {
        dropShadow[key] = { $value: parsed }
      } else {
        console.warn(`Failed to parse drop-shadow for ${name}, using raw value`)
        dropShadow[key] = { $value: raw }
      }
      continue
    }

    if (name.startsWith('text-shadow-')) {
      const key = name.slice('text-shadow-'.length)
      const parsed = parseShadow(raw)
      if (parsed) {
        textShadow[key] = { $value: parsed }
      } else {
        console.warn(`Failed to parse text-shadow for ${name}, using raw value`)
        textShadow[key] = { $value: raw }
      }
      continue
    }
  }

  return {
    shadow,
    'inset-shadow': insetShadow,
    'drop-shadow': dropShadow,
    'text-shadow': textShadow,
  }
}

function buildFiltersTokens(vars) {
  const blur = {
    $type: 'dimension',
  }

  for (const [name, raw] of vars) {
    if (name.startsWith('blur-')) {
      const key = name.slice('blur-'.length)
      blur[key] = { $value: raw }
      continue
    }

    if (name === 'blur') {
      blur.base = { $value: raw }
      continue
    }
  }

  return {
    blur,
  }
}

function buildSizingTokens(vars) {
  const size = {
    $type: 'dimension',
  }
  const container = {
    $type: 'dimension',
  }

  for (const [name, raw] of vars) {
    if (name.startsWith('size-')) {
      const key = name.slice('size-'.length)
      size[key] = { $value: raw }
      continue
    }

    if (name.startsWith('container-')) {
      const key = name.slice('container-'.length)
      container[key] = { $value: raw }
      continue
    }
  }

  return {
    size,
    container,
  }
}

function buildSpacingTokens(vars) {
  const spacing = {
    $type: 'dimension',
  }

  // First, collect any explicit spacing variables from theme.css
  for (const [name, raw] of vars) {
    if (name.startsWith('spacing-')) {
      const key = name.slice('spacing-'.length)
      spacing[key] = { $value: raw }
      continue
    }

    if (name === 'spacing') {
      spacing.base = { $value: raw }
      continue
    }
  }

  // If we have a base spacing unit, generate the full Tailwind spacing scale
  // This makes the tokens compatible with DTCG format while supporting Tailwind's
  // multiplier-based system (p-1, p-2, etc.)
  if (spacing.base) {
    const baseValue = spacing.base.$value
    // Parse the base value (e.g., "0.25rem" -> 0.25)
    const match = baseValue.match(/^([\d.]+)(.*)$/)
    if (match) {
      const baseNum = parseFloat(match[1])
      const unit = match[2] || 'rem'

      // Generate Tailwind's default spacing scale
      // 0, 0.5 (px), 1, 1.5, 2, 2.5, 3, 3.5, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 72, 80, 96
      const scale = [
        { key: '0', multiplier: 0 },
        { key: 'px', multiplier: 1, value: '1px' }, // Special case: px is always 1px
        { key: '0.5', multiplier: 0.5 },
        { key: '1', multiplier: 1 },
        { key: '1.5', multiplier: 1.5 },
        { key: '2', multiplier: 2 },
        { key: '2.5', multiplier: 2.5 },
        { key: '3', multiplier: 3 },
        { key: '3.5', multiplier: 3.5 },
        { key: '4', multiplier: 4 },
        { key: '5', multiplier: 5 },
        { key: '6', multiplier: 6 },
        { key: '7', multiplier: 7 },
        { key: '8', multiplier: 8 },
        { key: '9', multiplier: 9 },
        { key: '10', multiplier: 10 },
        { key: '11', multiplier: 11 },
        { key: '12', multiplier: 12 },
        { key: '14', multiplier: 14 },
        { key: '16', multiplier: 16 },
        { key: '20', multiplier: 20 },
        { key: '24', multiplier: 24 },
        { key: '28', multiplier: 28 },
        { key: '32', multiplier: 32 },
        { key: '36', multiplier: 36 },
        { key: '40', multiplier: 40 },
        { key: '44', multiplier: 44 },
        { key: '48', multiplier: 48 },
        { key: '52', multiplier: 52 },
        { key: '56', multiplier: 56 },
        { key: '60', multiplier: 60 },
        { key: '64', multiplier: 64 },
        { key: '72', multiplier: 72 },
        { key: '80', multiplier: 80 },
        { key: '96', multiplier: 96 },
      ]

      for (const { key, multiplier, value } of scale) {
        // Don't overwrite if already exists
        if (!spacing[key]) {
          if (value) {
            // Special case (like px)
            spacing[key] = { $value: value }
          } else {
            // Calculate: multiplier * baseNum + unit
            const calculatedValue = multiplier * baseNum
            spacing[key] = { $value: `${calculatedValue}${unit}` }
          }
        }
      }
    }
  }

  return {
    spacing,
  }
}

function parseFontList(raw) {
  return raw
    .split(',')
    .map(s => s.trim())
    .filter(Boolean)
    .map(s => s.replace(/^['"]|['"]$/g, ''))
}

/**
 * Extract line-height from calc expression or raw value.
 * For "calc(1.5 / 1)" the numerator (1.5) is the line-height in rem.
 * For plain numbers like "1", multiply by fontSize to get rem value.
 * @param {string} raw - The raw CSS value
 * @param {string} fontSize - The associated font size (e.g., "0.75rem")
 * @returns {string} Line-height with rem unit
 */
function extractLineHeight(raw, fontSize) {
  // Match calc(X / Y) pattern where X is the line-height in rem
  const calcMatch = raw.match(/^calc\(\s*([\d.]+)\s*\/\s*([\d.]+)\s*\)$/)
  if (calcMatch) {
    const lineHeightRem = parseFloat(calcMatch[1])
    return `${lineHeightRem}rem`
  }

  // For plain numbers (unitless multipliers), calculate actual rem value
  const num = Number.parseFloat(raw)
  if (!Number.isNaN(num)) {
    // Extract font size number from "0.75rem" -> 0.75
    const fontSizeMatch = fontSize.match(/^([\d.]+)(.*)$/)
    if (fontSizeMatch) {
      const fontSizeNum = parseFloat(fontSizeMatch[1])
      const unit = fontSizeMatch[2] || 'rem'
      const lineHeightValue = num * fontSizeNum
      // Round to reasonable precision
      const rounded = Math.round(lineHeightValue * 10000) / 10000
      return `${rounded}${unit}`
    }
    // Fallback: return as rem
    return `${num}rem`
  }

  return raw
}

function buildTypographyTokens(vars) {
  const typography = {
    font: {
      $type: 'fontFamily',
    },
    text: {
      $type: 'dimension',
    },
    'font-weight': {
      $type: 'fontWeight',
    },
    leading: {
      $type: 'number',
    },
    tracking: {
      $type: 'dimension',
    },
  }

  // Collect font sizes and line heights separately first
  const textSizes = new Map() // key -> { size, lineHeightRaw }

  for (const [name, raw] of vars) {
    // Font families
    if (name === 'font-sans' || name === 'font-serif' || name === 'font-mono') {
      const key = name.split('-')[1] // "sans" | "serif" | "mono"
      typography['font'][key] = { $value: parseFontList(raw) }
      continue
    }

    // Font weights, e.g. --font-weight-thin: 100;
    if (name.startsWith('font-weight-')) {
      const key = name.slice('font-weight-'.length) // "thin", "normal", "bold", etc.
      const num = Number.parseInt(raw, 10)
      typography['font-weight'][key] = {
        $value: Number.isNaN(num) ? raw : num,
      }
      continue
    }

    // Text sizes with line height, e.g. --text-xs--line-height: calc(1 / 0.75);
    if (name.startsWith('text-') && name.includes('--line-height')) {
      // Extract size key: "text-xs--line-height" -> "xs"
      const key = name.slice('text-'.length, name.indexOf('--line-height'))
      // Skip text-shadow line heights (shouldn't exist, but just in case)
      if (key.startsWith('shadow-')) continue

      if (!textSizes.has(key)) {
        textSizes.set(key, {})
      }
      textSizes.get(key).lineHeightRaw = raw
      continue
    }

    // Text sizes (font sizes), e.g. --text-xs: 0.75rem;
    // Must check after --line-height to avoid matching those
    if (name.startsWith('text-') && !name.includes('--')) {
      const key = name.slice('text-'.length) // "xs", "sm", "base", etc.
      // Skip text-shadow entries (handled in effects)
      if (key.startsWith('shadow-')) continue

      if (!textSizes.has(key)) {
        textSizes.set(key, {})
      }
      textSizes.get(key).size = raw
      continue
    }

    // Line heights, e.g. --leading-tight: 1.25;
    if (name.startsWith('leading-')) {
      const key = name.slice('leading-'.length)
      const num = Number.parseFloat(raw)
      typography['leading'][key] = {
        $value: Number.isNaN(num) ? raw : num,
      }
      continue
    }

    // Letter spacing, e.g. --tracking-tight: -0.025em;
    if (name.startsWith('tracking-')) {
      const key = name.slice('tracking-'.length)
      typography['tracking'][key] = { $value: raw }
      continue
    }
  }

  // Now build font-size tokens with proper line-height units
  for (const [key, data] of textSizes) {
    typography['text'][key] = {}
    if (data.size) {
      typography['text'][key]['size'] = { $value: data.size }
    }
    if (data.lineHeightRaw) {
      const lineHeight = extractLineHeight(
        data.lineHeightRaw,
        data.size || '1rem'
      )
      typography['text'][key]['line-height'] = { $value: lineHeight }
    }
  }

  return typography
}

async function writeTokenFile(fileName, data) {
  const filePath = path.join(
    process.cwd(),
    'tokens',
    'base',
    'primitives',
    fileName
  )
  await fs.mkdir(path.dirname(filePath), { recursive: true })
  const json = JSON.stringify(data, null, 2)
  await fs.writeFile(filePath, json + '\n', 'utf8')
}

async function syncBasePrimitives() {
  const cssVars = await getCssVariables()

  const data = {
    base: {
      ...buildBordersTokens(cssVars),
      ...buildColorTokens(cssVars),
      ...buildEffectsTokens(cssVars),
      ...buildFiltersTokens(cssVars),
      ...buildSizingTokens(cssVars),
      ...buildSpacingTokens(cssVars),
      ...buildTypographyTokens(cssVars),
    },
  }

  const filePath = path.join(process.cwd(), 'tokens', 'global', '_base.yaml')
  await fs.mkdir(path.dirname(filePath), { recursive: true })
  await fs.writeFile(filePath, YAML.stringify(data), 'utf8')
}

/**
 * Where each platform package's generated sources land.
 *
 * This package is a generator, not a dependency: a platform package holds the
 * output rather than importing it, so it builds and ships without the design
 * system being installed alongside it. `dist/` is only the staging area the
 * plugins write to on the way here.
 *
 * The distributed files are committed, for the same reason
 * `tokens/global/_base.yaml` is: what a token change does to a platform is
 * then visible in the diff rather than only after a rebuild.
 */
const TARGETS = [
  {
    package: '@dazzlabs/dazzui-react',
    dir: path.join('..', 'ui_react', 'src', 'generated'),
    files: ['style.css', 'control.css'],
  },
  // The Flutter sources still land in dist/flutter and go nowhere: they are
  // waiting on a `packages/ui_flutter` to be their target here.
]

async function distribute() {
  for (const target of TARGETS) {
    const targetDir = path.join(process.cwd(), target.dir)
    await fs.mkdir(targetDir, { recursive: true })

    for (const file of target.files) {
      await fs.copyFile(
        path.join(process.cwd(), 'dist', file),
        path.join(targetDir, file)
      )
    }
    console.log(
      `→ ${target.files.length} files to ${target.package} (${target.dir})`
    )
  }
}

async function main() {
  const shouldSync = process.argv.includes('--sync')

  if (shouldSync) {
    await syncBasePrimitives()
  }

  execSync('tz build', { stdio: 'inherit' })
  await distribute()
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
