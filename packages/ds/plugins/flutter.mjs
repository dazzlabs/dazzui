/**
 * Terrazzo plugin to generate Flutter design tokens
 * Generates Dart code with proper type conversions for Flutter
 */
class FlutterPlugin {
  get name() {
    return '@dazzlabs/dazzui-ds-plugin-flutter'
  }

  async build(context) {
    const { tokens, sources, outputFile } = context

    // Generate colors.dart file
    const colorsCode = generateColorsFile(tokens, sources)
    outputFile('flutter/colors.dart', colorsCode)

    // Generate theme_variables.dart file
    const themeVariablesCode = generateThemeVariablesFile(tokens, sources)
    outputFile('flutter/theme_variables.dart', themeVariablesCode)
  }
}

/**
 * Create a Flutter plugin instance
 * Generates colors.dart and theme_variables.dart files
 */
export default function flutterPlugin() {
  return new FlutterPlugin()
}

/**
 * Generate colors.dart file from tokens/global/_base.json (base.color.*)
 */
function generateColorsFile(tokens, sources) {
  const colorTokens = []

  // Filter tokens from tokens/global/_base.json (base.color.<name>.<shade>)
  for (const source of sources || []) {
    const filename = String(source.filename || '')
    const normalizedPath = filename.replace(/\\/g, '/').replace('file://', '')

    if (normalizedPath.includes('/tokens/global/_base.json')) {
      try {
        const sourceTokens = JSON.parse(source.src)
        const baseColors = sourceTokens?.base?.color
        if (baseColors && typeof baseColors === 'object') {
          collectColorTokens(baseColors, 'base.color', colorTokens, tokens)
        }
      } catch (e) {
        console.warn('Failed to parse _base.json for colors', e.message)
      }
      break
    }
  }

  // Generate Dart code
  let code = `import 'package:flutter/painting.dart';\n\n`
  code += `extension ExtendedColorSwatch on ColorSwatch<int> {\n`
  code += `  Color get shade50 => this[50]!;\n`
  code += `  Color get shade100 => this[100]!;\n`
  code += `  Color get shade200 => this[200]!;\n`
  code += `  Color get shade300 => this[300]!;\n`
  code += `  Color get shade400 => this[400]!;\n`
  code += `  Color get shade500 => this[500]!;\n`
  code += `  Color get shade600 => this[600]!;\n`
  code += `  Color get shade700 => this[700]!;\n`
  code += `  Color get shade800 => this[800]!;\n`
  code += `  Color get shade900 => this[900]!;\n`
  code += `  Color get shade950 => this[950]!;\n`
  code += `}\n\n`
  code += `/// Primitive color tokens\n`
  code += `/// DO NOT EDIT - This file is auto-generated\n`
  code += `abstract final class Colors {\n`
  code += `  /// Completely invisible.\n`
  code += `  static const Color transparent = Color(0x00000000);\n\n`
  code += `  /// Completely opaque black.\n`
  code += `  static const Color black = Color(0xFF000000);\n\n`
  code += `  /// Completely opaque white.\n`
  code += `  static const Color white = Color(0xFFFFFFFF);\n\n`

  // Sort color groups
  const colorGroups = {}
  for (const colorToken of colorTokens) {
    const parts = colorToken.id.split('.')
    if (parts.length >= 4 && parts[0] === 'base' && parts[1] === 'color') {
      const colorName = parts[2]
      const shade = parts[3]
      if (!colorGroups[colorName]) {
        colorGroups[colorName] = {}
      }
      colorGroups[colorName][shade] = colorToken
    }
  }

  // Generate ColorSwatch<int> constants
  for (const [colorName, shades] of Object.entries(colorGroups)) {
    // Skip black and white as they're already defined
    if (colorName === 'black' || colorName === 'white') {
      continue
    }

    // Get the 600 shade as the primary value (or 500 if 600 doesn't exist)
    const primaryShade = shades['600'] || shades['500']
    if (!primaryShade) continue

    const primaryValue = convertColorValue(primaryShade.$value)
    const primaryHex =
      primaryValue.match(/0x([0-9A-F]+)/)?.[1]?.toLowerCase() || 'ff000000'

    code += `  static const ColorSwatch<int> ${colorName} = ColorSwatch<int>(\n`
    code += `    0x${primaryHex},\n`
    code += `    <int, Color>{\n`

    // Sort shades numerically
    const sortedShades = Object.keys(shades).sort(
      (a, b) => parseInt(a) - parseInt(b)
    )

    for (const shade of sortedShades) {
      const token = shades[shade]
      const value = convertColorValue(token.$value)
      code += `      ${shade}: ${value},\n`
    }

    code += `    },\n`
    code += `  );\n\n`
  }

  code += `}\n`
  return code
}

/**
 * Collect color tokens from the source
 */
function collectColorTokens(obj, prefix, result, allTokens) {
  for (const [key, value] of Object.entries(obj)) {
    if (key.startsWith('$')) continue

    const tokenId = prefix ? `${prefix}.${key}` : key

    if (value && typeof value === 'object') {
      if (value.$value !== undefined) {
        // This is a token, get the resolved value from allTokens
        const resolvedToken = allTokens[tokenId]
        if (resolvedToken && resolvedToken.$type === 'color') {
          result.push({
            id: tokenId,
            ...resolvedToken,
          })
        }
      } else {
        // This is a group, recurse
        collectColorTokens(value, tokenId, result, allTokens)
      }
    }
  }
}

/**
 * Capitalize first letter
 */
function capitalize(str) {
  return str.charAt(0).toUpperCase() + str.slice(1)
}

/**
 * Generate theme_variables.dart file
 */
function generateThemeVariablesFile(tokens, sources) {
  const className = 'ThemeVariables'
  const imports = `import 'package:flutter/widgets.dart';
import '../foundation/color_descriptor.dart';
import 'colors.dart';`

  const categorizedTokens = categorizeTokensBySource(tokens, sources)

  const { fields, initializers } = generateClassContent(
    categorizedTokens,
    tokens
  )

  return `${imports}

/// Design tokens generated from Terrazzo
/// DO NOT EDIT - This file is auto-generated
class ${className} {
  const ${className}({
${initializers}
  });

${fields}
}

/// Global instance of ${className}
const ${className.charAt(0).toLowerCase() + className.slice(1)} = ${className}();
`
}

/**
 * Categorize tokens by their source file structure
 */
function categorizeTokensBySource(tokens, sources) {
  const categories = {}

  // Build a map of token ID to source file info and original value
  const tokenSourceMap = {}
  const tokenOriginalValueMap = {}

  if (sources && Array.isArray(sources)) {
    for (const source of sources) {
      const filename = String(source.filename || '')
      const normalizedPath = filename.replace(/\\/g, '/').replace('file://', '')

      // Extract category and subcategory from path
      // Example: /path/to/tokens/global/_base.json
      const tokensIndex = normalizedPath.lastIndexOf('/tokens/')
      if (tokensIndex !== -1) {
        const pathAfterTokens = normalizedPath.substring(
          tokensIndex + '/tokens/'.length
        )
        const parts = pathAfterTokens.split('/')

        // Last part is filename.json
        const filename = parts[parts.length - 1].replace('.json', '')
        const categoryParts = parts.slice(0, -1) // Everything except the filename
        const categoryPath = categoryParts.join('/')
        const subcategory = filename

        if (categoryPath && subcategory) {
          try {
            // Parse the JSON source
            const sourceTokens = JSON.parse(source.src)

            // Recursively collect all token IDs from this source
            const collectTokenIds = (obj, prefix = '') => {
              for (const [key, value] of Object.entries(obj)) {
                if (key.startsWith('$')) continue // Skip metadata keys

                const tokenId = prefix ? `${prefix}.${key}` : key

                if (value && typeof value === 'object') {
                  if (value.$value !== undefined) {
                    // This is a token
                    tokenSourceMap[tokenId] = {
                      categoryPath,
                      subcategory,
                      fullPath: normalizedPath,
                    }
                    // Store original value with references
                    tokenOriginalValueMap[tokenId] = value.$value
                  } else {
                    // This is a group, recurse
                    collectTokenIds(value, tokenId)
                  }
                }
              }
            }

            collectTokenIds(sourceTokens)
          } catch (e) {
            console.warn(`Failed to parse source: ${filename}`, e.message)
          }
        }
      }
    }
  }

  // Organize tokens by category path and subcategory
  for (const [id, token] of Object.entries(tokens)) {
    const sourceInfo = tokenSourceMap[id]

    if (sourceInfo) {
      const { categoryPath, subcategory } = sourceInfo

      // Initialize category if it doesn't exist
      if (!categories[categoryPath]) {
        categories[categoryPath] = {
          title: formatCategoryTitle(categoryPath),
          subcategories: {},
        }
      }

      // Initialize subcategory if it doesn't exist
      if (!categories[categoryPath].subcategories[subcategory]) {
        categories[categoryPath].subcategories[subcategory] = {
          title: formatSubcategoryTitle(subcategory),
          tokens: [],
        }
      }

      // Add token to subcategory with original value
      categories[categoryPath].subcategories[subcategory].tokens.push({
        id,
        ...token,
        $originalValue: tokenOriginalValueMap[id],
      })
    } else {
      // Fallback for tokens without source info
      if (!categories['other']) {
        categories['other'] = {
          title: 'Other',
          subcategories: {
            uncategorized: {
              title: 'Uncategorized',
              tokens: [],
            },
          },
        }
      }

      categories['other'].subcategories['uncategorized'].tokens.push({
        id,
        ...token,
        $originalValue: tokenOriginalValueMap[id],
      })
    }
  }

  return categories
}

/**
 * Format category path as title
 */
function formatCategoryTitle(categoryPath) {
  return categoryPath
    .split('/')
    .map(part => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' / ')
}

/**
 * Format subcategory name as title
 */
function formatSubcategoryTitle(subcategory) {
  return subcategory
    .split(/[-_]/)
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

/**
 * Generate class content with all token constants organized by source
 */
function generateClassContent(categories, allTokens) {
  const fieldSections = []
  const initializerSections = []

  const isBaseTokenId = id => typeof id === 'string' && id.startsWith('base.')

  // We generate human-readable section headers in the emitted Dart.
  // Avoid emitting a noisy "Base" header (it doesn't add value and shows up as `//  Base`).
  const emitSubcategoryHeader = title => {
    if (typeof title !== 'string') return
    const trimmed = title.trim()
    if (!trimmed) return
    if (trimmed.toLowerCase() === 'base') return
    fieldSections.push(`  // ${trimmed}`)
  }

  const extractRefId = value => {
    if (typeof value !== 'string') return null
    const match = value.match(/^\{(.+)\}$/)
    return match ? match[1] : null
  }

  // Define the order of categories
  const categoryOrder = ['global', 'component', 'theme/default']

  // Get all category keys, prioritizing the predefined order
  const allCategoryKeys = [
    ...categoryOrder.filter(key => categories[key]),
    ...Object.keys(categories).filter(
      key => !categoryOrder.includes(key) && key !== 'other'
    ),
  ]

  // Add 'other' at the end if it exists
  if (categories['other']) {
    allCategoryKeys.push('other')
  }

  // Generate sections for each category
  for (const categoryKey of allCategoryKeys) {
    const category = categories[categoryKey]

    fieldSections.push(`  // #region ${category.title}`)
    fieldSections.push('')

    // Sort subcategories alphabetically
    const sortedSubcategories = Object.keys(category.subcategories).sort()

    // For Global category, collect control tokens separately
    const isGlobalCategory = categoryKey === 'global'
    const controlTokensBuffer = {
      fields: [],
      initializers: [],
    }

    for (const subcategoryKey of sortedSubcategories) {
      const subcategory = category.subcategories[subcategoryKey]
      const subcategoryStartLen = fieldSections.length

      // Handle colors subcategory specially
      if (subcategoryKey === 'color' && categoryKey === 'base/primitives') {
        emitSubcategoryHeader(subcategory.title)

        // Collect color groups
        const colorGroups = {}
        for (const token of subcategory.tokens) {
          const parts = token.id.split('.')
          if (parts.length >= 3) {
            const colorName = parts[1]
            if (!colorGroups[colorName]) {
              colorGroups[colorName] = true
            }
          }
        }

        // Generate ColorSwatch<int> field declarations
        for (const colorName of Object.keys(colorGroups).sort()) {
          if (colorName === 'black' || colorName === 'white') {
            continue
          }
          const varName = `color${capitalize(colorName)}`
          fieldSections.push(`  final ColorSwatch<int> ${varName};`)
          initializerSections.push(`    this.${varName} = Colors.${colorName},`)
        }

        fieldSections.push('')
        continue
      }

      emitSubcategoryHeader(subcategory.title)

      // Special handling for colors subcategory: group by color name and reference base ColorSwatch<int>
      if (subcategoryKey === 'color' && categoryKey === 'global') {
        // NOTE:
        // tokens/global/color.json contains both:
        // - Palette tokens:   color.<group>.<shade> (e.g. color.neutral.100)
        // - Semantic tokens:  color.surface, color.content, ...
        // - Other namespaces: interactive.*
        //
        // We generate ColorSwatch<int> fields for palette groups, and then fall back
        // to normal generation for everything else in this subcategory.

        const paletteTokens = []
        const otherTokens = []

        for (const token of subcategory.tokens) {
          if (isBaseTokenId(token.id)) continue

          const parts = token.id.split('.')
          const isPaletteShade =
            parts.length === 3 &&
            parts[0] === 'color' &&
            /^\d+$/.test(parts[2] || '')

          if (isPaletteShade) paletteTokens.push(token)
          else otherTokens.push(token)
        }

        // Collect palette color groups (e.g., primary, neutral, info, etc.)
        const colorGroups = {}

        for (const token of paletteTokens) {
          const parts = token.id.split('.')
          const colorGroupName = parts[1] // e.g., "primary", "neutral", "info"
          const shade = parts[2] // e.g., "50", "100", etc.

          if (!colorGroups[colorGroupName]) {
            colorGroups[colorGroupName] = {
              tokens: [],
              baseColorMap: new Map(), // Map shade -> base color name
            }
          }

          // Extract base color name from token reference
          const originalValue = token.$originalValue || token.$value
          if (token.$type === 'color' && typeof originalValue === 'string') {
            const refId = extractRefId(originalValue)
            if (refId && refId.startsWith('base.color.')) {
              const refParts = refId.split('.')
              if (
                refParts.length === 4 &&
                refParts[0] === 'base' &&
                refParts[1] === 'color'
              ) {
                const baseColorName = refParts[2] // e.g., "blue", "neutral", "sky"
                colorGroups[colorGroupName].baseColorMap.set(
                  shade,
                  baseColorName
                )
              }
            }
          }

          colorGroups[colorGroupName].tokens.push(token)
        }

        // Generate ColorSwatch<int> fields for color groups that all reference the same base color
        for (const [colorGroupName, group] of Object.entries(colorGroups)) {
          // Check if all shades reference the same base color
          const baseColorNames = Array.from(
            new Set(group.baseColorMap.values())
          )

          // Check if all tokens have base color references and they all reference the same base color
          const allTokensHaveBaseColorRefs = group.tokens.every(token => {
            const originalValue = token.$originalValue || token.$value
            if (token.$type === 'color' && typeof originalValue === 'string') {
              const refId = extractRefId(originalValue)
              return refId && refId.startsWith('base.color.')
            }
            return false
          })

          if (
            allTokensHaveBaseColorRefs &&
            baseColorNames.length === 1 &&
            group.baseColorMap.size > 0 &&
            group.baseColorMap.size === group.tokens.length
          ) {
            // All shades reference the same base color, generate ColorSwatch<int> field
            const baseColorName = baseColorNames[0]
            const varName = `color${capitalize(colorGroupName)}`
            const comment = generateComment(group.tokens[0])

            if (comment) {
              fieldSections.push(`  ${comment}`)
            }

            fieldSections.push(`  final ColorSwatch<int> ${varName};`)
            initializerSections.push(
              `    this.${varName} = Colors.${baseColorName},`
            )
          } else {
            // Group control color tokens by their base ID (without state and .shade/.alpha/.color suffix)
            const controlColorGroupsPalette = new Map()

            // Not all shades reference the same base color, use original logic
            for (const token of group.tokens) {
              const varName = tokenIdToVarName(token.id)

              // Check if this is a control token in Global category
              const isControlToken =
                isGlobalCategory && varName.startsWith('control')

              // Group control.color tokens that end with .shade, .alpha, or .color
              if (
                isControlToken &&
                varName.startsWith('controlColor') &&
                (token.id.endsWith('.shade') ||
                  token.id.endsWith('.alpha') ||
                  token.id.endsWith('.color'))
              ) {
                // Extract base ID without state (normal/hovered/pressed/disabled) and .shade/.alpha/.color
                const withoutSuffix = token.id.replace(
                  /\.(shade|alpha|color)$/,
                  ''
                )
                const match = withoutSuffix.match(
                  /^(.+)\.(normal|hovered|pressed|disabled)$/
                )

                if (match) {
                  const baseId = match[1]
                  const state = match[2]

                  if (!controlColorGroupsPalette.has(baseId)) {
                    controlColorGroupsPalette.set(baseId, {
                      normal: { shade: null, alpha: null, color: null },
                      hovered: { shade: null, alpha: null, color: null },
                      pressed: { shade: null, alpha: null, color: null },
                      disabled: { shade: null, alpha: null, color: null },
                      comment: null,
                    })
                  }

                  const group = controlColorGroupsPalette.get(baseId)
                  if (token.id.endsWith('.shade')) {
                    group[state].shade = token.$value
                    if (!group.comment) group.comment = generateComment(token)
                  } else if (token.id.endsWith('.alpha')) {
                    group[state].alpha = token.$value
                  } else if (token.id.endsWith('.color')) {
                    group[state].color = token
                    if (!group.comment) group.comment = generateComment(token)
                  }
                  continue
                }
              }

              const comment = generateComment(token)
              const targetFieldSections = isControlToken
                ? controlTokensBuffer.fields
                : fieldSections
              const targetInitializerSections = isControlToken
                ? controlTokensBuffer.initializers
                : initializerSections

              if (comment) {
                targetFieldSections.push(`  ${comment}`)
              }

              const originalValue = token.$originalValue || token.$value
              let colorReference = null

              if (
                token.$type === 'color' &&
                typeof originalValue === 'string'
              ) {
                const refId = extractRefId(originalValue)
                if (refId && refId.startsWith('base.color.')) {
                  const parts = refId.split('.')
                  if (
                    parts.length === 4 &&
                    parts[0] === 'base' &&
                    parts[1] === 'color'
                  ) {
                    const baseColorName = parts[2]
                    const shade = parts[3]
                    colorReference = `Colors.${baseColorName}[${shade}]!`
                  } else if (
                    parts.length === 3 &&
                    parts[0] === 'base' &&
                    parts[1] === 'color'
                  ) {
                    // e.g. {base.color.white}
                    const baseColorName = parts[2]
                    colorReference = `Colors.${baseColorName}`
                  }
                }
              }

              if (colorReference) {
                targetFieldSections.push(`  final Color ${varName};`)
                targetInitializerSections.push(
                  `    this.${varName} = ${colorReference},`
                )
              } else {
                const value = convertTokenValue(token, allTokens)
                targetFieldSections.push(`  final ${value.type} ${varName};`)
                const needsConst = ![
                  'int',
                  'double',
                  'num',
                  'String',
                  'bool',
                  'FontWeight',
                ].includes(value.type)
                const constKeyword = needsConst ? 'const ' : ''
                targetInitializerSections.push(
                  `    this.${varName} = ${constKeyword}${value.value},`
                )
              }
            }

            // Process grouped control color tokens from palette section
            for (const [baseId, group] of controlColorGroupsPalette) {
              // Check if each state has either color OR (shade AND alpha)
              const hasValidNormal =
                group.normal.color !== null ||
                (group.normal.shade !== null && group.normal.alpha !== null)
              const hasValidHovered =
                group.hovered.color !== null ||
                (group.hovered.shade !== null && group.hovered.alpha !== null)
              const hasValidPressed =
                group.pressed.color !== null ||
                (group.pressed.shade !== null && group.pressed.alpha !== null)
              const hasValidDisabled =
                group.disabled.color !== null ||
                (group.disabled.shade !== null && group.disabled.alpha !== null)

              const hasAllStates =
                hasValidNormal &&
                hasValidHovered &&
                hasValidPressed &&
                hasValidDisabled

              if (hasAllStates) {
                const varName = tokenIdToVarName(baseId)
                const targetFieldSections = controlTokensBuffer.fields
                const targetInitializerSections =
                  controlTokensBuffer.initializers

                if (group.comment) {
                  targetFieldSections.push(`  ${group.comment}`)
                }

                targetFieldSections.push(`  final ColorDescriptor ${varName};`)

                // Format opacity values to always show decimal point
                const formatOpacity = val => {
                  const num = Number(val)
                  return num === Math.floor(num) ? `${num}.0` : String(num)
                }

                // Helper to convert color token to Dart Color value
                const getColorValue = colorToken => {
                  if (!colorToken) return null
                  const value = convertTokenValue(colorToken, allTokens)
                  return value.value
                }

                const params = []
                if (group.normal.color) {
                  params.push(
                    `      normalColor: ${getColorValue(group.normal.color)}`
                  )
                }
                if (group.normal.shade !== null) {
                  params.push(`      normalShade: ${group.normal.shade}`)
                }
                if (group.normal.alpha !== null) {
                  params.push(
                    `      normalOpacity: ${formatOpacity(group.normal.alpha)}`
                  )
                }
                if (group.hovered.color) {
                  params.push(
                    `      hoveredColor: ${getColorValue(group.hovered.color)}`
                  )
                }
                if (group.hovered.shade !== null) {
                  params.push(`      hoveredShade: ${group.hovered.shade}`)
                }
                if (group.hovered.alpha !== null) {
                  params.push(
                    `      hoveredOpacity: ${formatOpacity(group.hovered.alpha)}`
                  )
                }
                if (group.pressed.color) {
                  params.push(
                    `      pressedColor: ${getColorValue(group.pressed.color)}`
                  )
                }
                if (group.pressed.shade !== null) {
                  params.push(`      pressedShade: ${group.pressed.shade}`)
                }
                if (group.pressed.alpha !== null) {
                  params.push(
                    `      pressedOpacity: ${formatOpacity(group.pressed.alpha)}`
                  )
                }
                if (group.disabled.color) {
                  params.push(
                    `      disabledColor: ${getColorValue(group.disabled.color)}`
                  )
                }
                if (group.disabled.shade !== null) {
                  params.push(`      disabledShade: ${group.disabled.shade}`)
                }
                if (group.disabled.alpha !== null) {
                  params.push(
                    `      disabledOpacity: ${formatOpacity(group.disabled.alpha)}`
                  )
                }

                targetInitializerSections.push(
                  `    this.${varName} = const ColorDescriptor(\n` +
                    params.join(',\n') +
                    ',\n' +
                    `    ),`
                )
              }
            }
          }
        }

        // Group control color tokens by their base ID (without state and .shade/.alpha/.color suffix)
        const controlColorGroupsOther = new Map()

        // Generate the remaining tokens in this subcategory (semantic colors + interactive.*)
        for (const token of otherTokens) {
          const varName = tokenIdToVarName(token.id)

          // Check if this is a control token in Global category
          const isControlToken =
            isGlobalCategory && varName.startsWith('control')

          // Group control.color tokens that end with .shade, .alpha, or .color
          if (
            isControlToken &&
            varName.startsWith('controlColor') &&
            (token.id.endsWith('.shade') ||
              token.id.endsWith('.alpha') ||
              token.id.endsWith('.color'))
          ) {
            // Extract base ID without state (normal/hovered/pressed/disabled) and .shade/.alpha/.color
            const withoutSuffix = token.id.replace(/\.(shade|alpha|color)$/, '')
            const match = withoutSuffix.match(
              /^(.+)\.(normal|hovered|pressed|disabled)$/
            )

            if (match) {
              const baseId = match[1]
              const state = match[2]

              if (!controlColorGroupsOther.has(baseId)) {
                controlColorGroupsOther.set(baseId, {
                  normal: { shade: null, alpha: null, color: null },
                  hovered: { shade: null, alpha: null, color: null },
                  pressed: { shade: null, alpha: null, color: null },
                  disabled: { shade: null, alpha: null, color: null },
                  comment: null,
                })
              }

              const group = controlColorGroupsOther.get(baseId)
              if (token.id.endsWith('.color')) {
                group[state].color = token
                if (!group.comment) group.comment = generateComment(token)
              } else if (token.id.endsWith('.shade')) {
                group[state].shade = token.$value
                if (!group.comment) group.comment = generateComment(token)
              } else if (token.id.endsWith('.alpha')) {
                group[state].alpha = token.$value
                if (!group.comment) group.comment = generateComment(token)
              }
              continue
            }
          }

          const comment = generateComment(token)
          const targetFieldSections = isControlToken
            ? controlTokensBuffer.fields
            : fieldSections
          const targetInitializerSections = isControlToken
            ? controlTokensBuffer.initializers
            : initializerSections

          if (comment) {
            targetFieldSections.push(`  ${comment}`)
          }

          const originalValue = token.$originalValue || token.$value
          const value = convertTokenValue(token, allTokens)

          // Check if the value contains token references
          // First check the original value for token references like {token.id}
          const hasTokenReference = (() => {
            // Check original value for token reference pattern
            if (typeof originalValue === 'string') {
              const refId = extractRefId(originalValue)
              if (refId && !isBaseTokenId(refId)) return true
            }

            // For complex types (objects), recursively check if any property value is a token reference string
            if (typeof originalValue === 'object' && originalValue !== null) {
              const hasTokenRefInObject = obj => {
                for (const value of Object.values(obj)) {
                  if (typeof value === 'string') {
                    const refId = extractRefId(value)
                    if (refId && !isBaseTokenId(refId)) return true
                  }
                  if (
                    typeof value === 'object' &&
                    value !== null &&
                    !Array.isArray(value)
                  ) {
                    if (hasTokenRefInObject(value)) {
                      return true
                    }
                  }
                }
                return false
              }

              if (hasTokenRefInObject(originalValue)) {
                return true
              }
            }

            // Also check the converted value for token references
            // Skip if it's just a primitive value (number, string literal, or enum)
            if (
              value.value.match(
                /^(const\s+)?(Color\(0x|FontWeight\.w\d+|BorderStyle\.|'\w+'|\d+\.?\d*|const\s+\[?\]?)$/
              )
            ) {
              return false
            }

            // Check if value contains identifiers that look like token references
            const tokenRefPattern =
              /\b([a-z][a-zA-Z0-9_]*(?:_[a-zA-Z0-9]+)*)\b(?!\s*[:(])/g
            const matches = value.value.match(tokenRefPattern)
            if (!matches) return false

            const dartKeywords = [
              'const',
              'final',
              'var',
              'true',
              'false',
              'null',
            ]
            const typeNames = [
              'Color',
              'TextStyle',
              'BorderSide',
              'BoxShadow',
              'Offset',
              'FontWeight',
              'BorderStyle',
              'ColorDescriptor',
            ]

            const actualReferences = matches.filter(
              m => !dartKeywords.includes(m) && !typeNames.includes(m)
            )
            return actualReferences.length > 0
          })()

          // If has token reference, use getter; otherwise use constructor parameter
          if (hasTokenReference) {
            if (value.type === 'TextStyle' && value.value.includes(',')) {
              const formattedValue = value.value
                .replace('const TextStyle(', 'TextStyle(')
                .replace('TextStyle(', 'TextStyle(\n    ')
                .replace(/, /g, ',\n    ')
                .replace(/\)$/, ',\n  )')
              targetFieldSections.push(
                `  ${value.type} get ${varName} => ${formattedValue};`
              )
            } else {
              targetFieldSections.push(
                `  ${value.type} get ${varName} => ${value.value};`
              )
            }
          } else {
            if (value.type === 'TextStyle' && value.value.includes(',')) {
              const formattedValue = value.value
                .replace('TextStyle(', 'const TextStyle(\n    ')
                .replace(/, /g, ',\n    ')
                .replace(/\)$/, ',\n  )')
              targetFieldSections.push(`  final ${value.type} ${varName};`)
              targetInitializerSections.push(
                `    this.${varName} = ${formattedValue},`
              )
            } else {
              targetFieldSections.push(`  final ${value.type} ${varName};`)
              const primitiveTypes = [
                'int',
                'double',
                'num',
                'String',
                'bool',
                'FontWeight',
              ]
              const needsConst = !primitiveTypes.includes(value.type)
              const constKeyword = needsConst ? 'const ' : ''
              targetInitializerSections.push(
                `    this.${varName} = ${constKeyword}${value.value},`
              )
            }
          }
        }

        // Process grouped control color tokens
        for (const [baseId, group] of controlColorGroupsOther) {
          // Check if each state has either color OR (shade AND alpha)
          const hasValidNormal =
            group.normal.color !== null ||
            (group.normal.shade !== null && group.normal.alpha !== null)
          const hasValidHovered =
            group.hovered.color !== null ||
            (group.hovered.shade !== null && group.hovered.alpha !== null)
          const hasValidPressed =
            group.pressed.color !== null ||
            (group.pressed.shade !== null && group.pressed.alpha !== null)
          const hasValidDisabled =
            group.disabled.color !== null ||
            (group.disabled.shade !== null && group.disabled.alpha !== null)

          const hasAllStates =
            hasValidNormal &&
            hasValidHovered &&
            hasValidPressed &&
            hasValidDisabled

          if (hasAllStates) {
            const varName = tokenIdToVarName(baseId)
            const targetFieldSections = controlTokensBuffer.fields
            const targetInitializerSections = controlTokensBuffer.initializers

            if (group.comment) {
              targetFieldSections.push(`  ${group.comment}`)
            }

            // NOTE:
            // `control.color.normal.{surface,content,border}` depend on semantic color tokens
            // (e.g. `color.surface`, `color.content`, `color.border`). If we emit them as
            // constructor-initialized `final` fields, we end up needing a `const
            // ColorDescriptor(...)` initializer, which can't reference other (non-const)
            // instance fields. Emit these as getters instead.
            const shouldEmitAsGetter =
              baseId === 'control.color.normal.surface' ||
              baseId === 'control.color.normal.content' ||
              baseId === 'control.color.normal.border'

            if (!shouldEmitAsGetter) {
              targetFieldSections.push(`  final ColorDescriptor ${varName};`)
            }

            // Format opacity values to always show decimal point
            const formatOpacity = val => {
              const num = Number(val)
              return num === Math.floor(num) ? `${num}.0` : String(num)
            }

            // Helper to convert color token to Dart Color value
            const getColorValue = colorToken => {
              if (!colorToken) return null
              const value = convertTokenValue(colorToken, allTokens)
              return value.value
            }

            const params = []
            if (group.normal.color) {
              params.push(
                `      normalColor: ${getColorValue(group.normal.color)}`
              )
            }
            if (group.normal.shade !== null) {
              params.push(`      normalShade: ${group.normal.shade}`)
            }
            if (group.normal.alpha !== null) {
              params.push(
                `      normalOpacity: ${formatOpacity(group.normal.alpha)}`
              )
            }
            if (group.hovered.color) {
              params.push(
                `      hoveredColor: ${getColorValue(group.hovered.color)}`
              )
            }
            if (group.hovered.shade !== null) {
              params.push(`      hoveredShade: ${group.hovered.shade}`)
            }
            if (group.hovered.alpha !== null) {
              params.push(
                `      hoveredOpacity: ${formatOpacity(group.hovered.alpha)}`
              )
            }
            if (group.pressed.color) {
              params.push(
                `      pressedColor: ${getColorValue(group.pressed.color)}`
              )
            }
            if (group.pressed.shade !== null) {
              params.push(`      pressedShade: ${group.pressed.shade}`)
            }
            if (group.pressed.alpha !== null) {
              params.push(
                `      pressedOpacity: ${formatOpacity(group.pressed.alpha)}`
              )
            }
            if (group.disabled.color) {
              params.push(
                `      disabledColor: ${getColorValue(group.disabled.color)}`
              )
            }
            if (group.disabled.shade !== null) {
              params.push(`      disabledShade: ${group.disabled.shade}`)
            }
            if (group.disabled.alpha !== null) {
              params.push(
                `      disabledOpacity: ${formatOpacity(group.disabled.alpha)}`
              )
            }

            if (shouldEmitAsGetter) {
              // Adjust indentation for a multi-line getter body.
              const getterParams = params.map(p => p.replace(/^      /, '    '))
              targetFieldSections.push(
                `  ColorDescriptor get ${varName} => ColorDescriptor(\n` +
                  getterParams.join(',\n') +
                  ',\n' +
                  `  );`
              )
            } else {
              targetInitializerSections.push(
                `    this.${varName} = const ColorDescriptor(\n` +
                  params.join(',\n') +
                  ',\n' +
                  `    ),`
              )
            }
          }
        }

        fieldSections.push('')
        continue
      }

      // Group control color tokens by their base ID (without state and .shade/.alpha/.color suffix)
      const controlColorGroups = new Map()

      // Generate tokens for this subcategory
      for (const token of subcategory.tokens) {
        // Do not generate base.* tokens into ThemeVariables
        if (isBaseTokenId(token.id)) {
          continue
        }

        const varName = tokenIdToVarName(token.id)

        // Check if this is a control token in Global category
        const isControlToken = isGlobalCategory && varName.startsWith('control')

        // Group control.color tokens that end with .shade, .alpha, or .color
        if (
          isControlToken &&
          varName.startsWith('controlColor') &&
          (token.id.endsWith('.shade') ||
            token.id.endsWith('.alpha') ||
            token.id.endsWith('.color'))
        ) {
          // Extract base ID without state (normal/hovered/pressed/disabled) and .shade/.alpha/.color
          const withoutSuffix = token.id.replace(/\.(shade|alpha|color)$/, '')
          const match = withoutSuffix.match(
            /^(.+)\.(normal|hovered|pressed|disabled)$/
          )

          if (match) {
            const baseId = match[1]
            const state = match[2]

            if (!controlColorGroups.has(baseId)) {
              controlColorGroups.set(baseId, {
                normal: { shade: null, alpha: null, color: null },
                hovered: { shade: null, alpha: null, color: null },
                pressed: { shade: null, alpha: null, color: null },
                disabled: { shade: null, alpha: null, color: null },
                comment: null,
              })
            }

            const group = controlColorGroups.get(baseId)
            if (token.id.endsWith('.shade')) {
              group[state].shade = token.$value
              if (!group.comment) group.comment = generateComment(token)
            } else if (token.id.endsWith('.alpha')) {
              group[state].alpha = token.$value
            } else if (token.id.endsWith('.color')) {
              group[state].color = token
              if (!group.comment) group.comment = generateComment(token)
            }
            continue
          }
        }

        const comment = generateComment(token)
        const targetFieldSections = isControlToken
          ? controlTokensBuffer.fields
          : fieldSections
        const targetInitializerSections = isControlToken
          ? controlTokensBuffer.initializers
          : initializerSections

        if (comment) {
          targetFieldSections.push(`  ${comment}`)
        }

        const originalValue = token.$originalValue || token.$value
        const value = convertTokenValue(token, allTokens)

        // Check if the value contains token references
        // First check the original value for token references like {token.id}
        const hasTokenReference = (() => {
          // Check original value for token reference pattern
          if (typeof originalValue === 'string') {
            // Match token reference pattern: {token.id}
            const refId = extractRefId(originalValue)
            if (refId && !isBaseTokenId(refId)) return true
          }

          // For complex types (objects), recursively check if any property value is a token reference string
          if (typeof originalValue === 'object' && originalValue !== null) {
            const hasTokenRefInObject = obj => {
              for (const value of Object.values(obj)) {
                if (typeof value === 'string') {
                  const refId = extractRefId(value)
                  if (refId && !isBaseTokenId(refId)) return true
                }
                if (
                  typeof value === 'object' &&
                  value !== null &&
                  !Array.isArray(value)
                ) {
                  if (hasTokenRefInObject(value)) {
                    return true
                  }
                }
              }
              return false
            }

            if (hasTokenRefInObject(originalValue)) {
              return true
            }
          }

          // Also check the converted value for token references
          // Skip if it's just a primitive value (number, string literal, or enum)
          if (
            value.value.match(
              /^(const\s+)?(Color\(0x|FontWeight\.w\d+|BorderStyle\.|'\w+'|\d+\.?\d*|const\s+\[?\]?)$/
            )
          ) {
            return false
          }

          // Check if value contains identifiers that look like token references
          // (lowercase_with_underscores or camelCase, not followed by parentheses)
          const tokenRefPattern =
            /\b([a-z][a-zA-Z0-9_]*(?:_[a-zA-Z0-9]+)*)\b(?!\s*[:(])/g
          const matches = value.value.match(tokenRefPattern)

          if (!matches) return false

          // Filter out Dart keywords and type names
          const dartKeywords = [
            'const',
            'final',
            'var',
            'true',
            'false',
            'null',
          ]
          const typeNames = [
            'Color',
            'TextStyle',
            'BorderSide',
            'BoxShadow',
            'Offset',
            'FontWeight',
            'BorderStyle',
          ]

          const actualReferences = matches.filter(
            m => !dartKeywords.includes(m) && !typeNames.includes(m)
          )

          return actualReferences.length > 0
        })()

        // If has token reference, use getter; otherwise use constructor parameter
        if (hasTokenReference) {
          // Format TextStyle on multiple lines for better readability
          if (value.type === 'TextStyle' && value.value.includes(',')) {
            const formattedValue = value.value
              .replace('const TextStyle(', 'TextStyle(')
              .replace('TextStyle(', 'TextStyle(\n    ')
              .replace(/, /g, ',\n    ')
              .replace(/\)$/, ',\n  )')
            targetFieldSections.push(
              `  ${value.type} get ${varName} => ${formattedValue};`
            )
          } else {
            targetFieldSections.push(
              `  ${value.type} get ${varName} => ${value.value};`
            )
          }
        } else {
          // Format TextStyle on multiple lines for better readability
          if (value.type === 'TextStyle' && value.value.includes(',')) {
            const formattedValue = value.value
              .replace('TextStyle(', 'const TextStyle(\n    ')
              .replace(/, /g, ',\n    ')
              .replace(/\)$/, ',\n  )')
            targetFieldSections.push(`  final ${value.type} ${varName};`)
            targetInitializerSections.push(
              `    this.${varName} = ${formattedValue},`
            )
          } else {
            targetFieldSections.push(`  final ${value.type} ${varName};`)
            // Add const only for complex types (not primitives like int, double, num, String)
            const primitiveTypes = [
              'int',
              'double',
              'num',
              'String',
              'bool',
              'FontWeight',
            ]
            const needsConst = !primitiveTypes.includes(value.type)
            const constKeyword = needsConst ? 'const ' : ''
            targetInitializerSections.push(
              `    this.${varName} = ${constKeyword}${value.value},`
            )
          }
        }
      }

      // Process grouped control color tokens
      for (const [baseId, group] of controlColorGroups) {
        // Check if each state has either color OR (shade AND alpha)
        const hasValidNormal =
          group.normal.color !== null ||
          (group.normal.shade !== null && group.normal.alpha !== null)
        const hasValidHovered =
          group.hovered.color !== null ||
          (group.hovered.shade !== null && group.hovered.alpha !== null)
        const hasValidPressed =
          group.pressed.color !== null ||
          (group.pressed.shade !== null && group.pressed.alpha !== null)
        const hasValidDisabled =
          group.disabled.color !== null ||
          (group.disabled.shade !== null && group.disabled.alpha !== null)

        const hasAllStates =
          hasValidNormal &&
          hasValidHovered &&
          hasValidPressed &&
          hasValidDisabled

        if (hasAllStates) {
          const varName = tokenIdToVarName(baseId)
          const targetFieldSections = controlTokensBuffer.fields
          const targetInitializerSections = controlTokensBuffer.initializers

          if (group.comment) {
            targetFieldSections.push(`  ${group.comment}`)
          }

          targetFieldSections.push(`  final ColorDescriptor ${varName};`)

          // Format opacity values to always show decimal point
          const formatOpacity = val => {
            const num = Number(val)
            return num === Math.floor(num) ? `${num}.0` : String(num)
          }

          // Helper to convert color token to Dart Color value
          const getColorValue = colorToken => {
            if (!colorToken) return null
            const value = convertTokenValue(colorToken, allTokens)
            return value.value
          }

          const params = []
          if (group.normal.color) {
            params.push(
              `      normalColor: ${getColorValue(group.normal.color)}`
            )
          }
          if (group.normal.shade !== null) {
            params.push(`      normalShade: ${group.normal.shade}`)
          }
          if (group.normal.alpha !== null) {
            params.push(
              `      normalOpacity: ${formatOpacity(group.normal.alpha)}`
            )
          }
          if (group.hovered.color) {
            params.push(
              `      hoveredColor: ${getColorValue(group.hovered.color)}`
            )
          }
          if (group.hovered.shade !== null) {
            params.push(`      hoveredShade: ${group.hovered.shade}`)
          }
          if (group.hovered.alpha !== null) {
            params.push(
              `      hoveredOpacity: ${formatOpacity(group.hovered.alpha)}`
            )
          }
          if (group.pressed.color) {
            params.push(
              `      pressedColor: ${getColorValue(group.pressed.color)}`
            )
          }
          if (group.pressed.shade !== null) {
            params.push(`      pressedShade: ${group.pressed.shade}`)
          }
          if (group.pressed.alpha !== null) {
            params.push(
              `      pressedOpacity: ${formatOpacity(group.pressed.alpha)}`
            )
          }
          if (group.disabled.color) {
            params.push(
              `      disabledColor: ${getColorValue(group.disabled.color)}`
            )
          }
          if (group.disabled.shade !== null) {
            params.push(`      disabledShade: ${group.disabled.shade}`)
          }
          if (group.disabled.alpha !== null) {
            params.push(
              `      disabledOpacity: ${formatOpacity(group.disabled.alpha)}`
            )
          }

          targetInitializerSections.push(
            `    this.${varName} = const ColorDescriptor(\n` +
              params.join(',\n') +
              ',\n' +
              `    ),`
          )
        }
      }

      // Only add a separator line if this subcategory actually emitted output.
      // (When we skip headers like "Base" and also skip base.* tokens, we'd otherwise
      // create extra blank lines in the generated Dart.)
      if (fieldSections.length > subcategoryStartLen) {
        if (fieldSections[fieldSections.length - 1] !== '') {
          fieldSections.push('')
        }
      }
    }

    // For Global category, append control tokens at the end
    if (isGlobalCategory && controlTokensBuffer.fields.length > 0) {
      fieldSections.push('  // Control')
      fieldSections.push(...controlTokensBuffer.fields)
      fieldSections.push('')
      initializerSections.push(...controlTokensBuffer.initializers)
    }

    fieldSections.push(`  // #endregion`)
    fieldSections.push('')
  }

  return {
    fields: fieldSections.join('\n'),
    initializers: initializerSections.join('\n'),
  }
}

/**
 * Convert token to Dart value with appropriate type
 */
function convertTokenValue(token, allTokens) {
  const type = token.$type

  const extractRefIdFromString = value => {
    if (typeof value !== 'string') return null
    const match = value.match(/^\{(.+)\}$/)
    return match ? match[1] : null
  }

  switch (type) {
    case 'color':
      // Handle token references like "{color.neutral.100}" or "{base.color.blue.500}"
      // so that generated Dart stays type-safe and doesn't emit placeholder Color(...) values.
      {
        const originalValue = token.$originalValue || token.$value
        const refId = extractRefIdFromString(originalValue)

        if (refId) {
          // Base color references are emitted as Colors.<name> or Colors.<name>[<shade>]!
          if (refId.startsWith('base.color.')) {
            const parts = refId.split('.')
            if (parts.length === 4) {
              const baseColorName = parts[2]
              const shade = parts[3]
              return {
                type: 'Color',
                value: `Colors.${baseColorName}[${shade}]!`,
              }
            }
            if (parts.length === 3) {
              const baseColorName = parts[2]
              return { type: 'Color', value: `Colors.${baseColorName}` }
            }
          }

          // Global palette references are emitted against generated ColorSwatch<int> fields
          // e.g. {color.neutral.100} -> colorNeutral[100]!
          const paletteRefMatch = refId.match(/^color\.([^.]+)\.(\d+)$/)
          if (paletteRefMatch) {
            const groupName = paletteRefMatch[1]
            const shade = paletteRefMatch[2]
            return {
              type: 'Color',
              value: `color${capitalize(groupName)}[${shade}]!`,
            }
          }

          // Other token references become variable references
          return { type: 'Color', value: tokenIdToVarName(refId) }
        }
      }

      return {
        type: 'Color',
        value: convertColorValue(token.$value),
      }

    case 'dimension':
    case 'spacing':
      return {
        type: 'double',
        value: convertDimensionValue(
          token.$originalValue || token.$value,
          allTokens
        ),
      }

    case 'fontSize':
    case 'lineHeight':
      return {
        type: 'double',
        value: convertDimensionValue(token.$value, allTokens),
      }

    case 'fontWeight':
      return {
        type: 'FontWeight',
        value: convertFontWeight(token.$value),
      }

    case 'fontFamily':
      return {
        type: 'String',
        value: convertFontFamily(token.$value),
      }

    case 'border':
      return {
        type: 'BorderSide',
        value: convertBorderValue(token.$value, allTokens),
      }

    case 'shadow':
      return {
        type: 'List<BoxShadow>',
        value: convertShadowValue(token.$value, allTokens),
      }

    case 'typography':
      return {
        type: 'TextStyle',
        value: convertTypographyValue(
          token.$originalValue || token.$value,
          allTokens
        ),
      }

    case 'number':
      // Check if token ID contains 'alpha' - if so, always use double
      const hasAlphaInName =
        token.id && token.id.toLowerCase().includes('alpha')
      const isInt = Number.isInteger(token.$value) && !hasAlphaInName
      return {
        type: isInt ? 'int' : 'double',
        value: token.$value.toString(),
      }

    default:
      return {
        type: 'dynamic',
        value: JSON.stringify(token.$value),
      }
  }
}

/**
 * Convert token ID to valid Dart variable name
 */
function tokenIdToVarName(id) {
  // Split by dots and convert to camelCase
  const parts = id.split('.')
  let varName = ''

  for (let i = 0; i < parts.length; i++) {
    const part = parts[i]
    // Split by special characters (colons, hyphens, underscores, etc.)
    const subParts = part.split(/[:\-_\s]+/).filter(p => p.length > 0)

    for (let j = 0; j < subParts.length; j++) {
      const subPart = subParts[j]
      // Remove any remaining invalid characters
      const cleanPart = subPart.replace(/[^a-zA-Z0-9]/g, '')

      if (cleanPart.length === 0) continue

      // First part of the entire variable name should be lowercase
      if (varName === '') {
        varName = cleanPart.charAt(0).toLowerCase() + cleanPart.slice(1)
      } else {
        // Subsequent parts should have uppercase first letter
        varName += cleanPart.charAt(0).toUpperCase() + cleanPart.slice(1)
      }
    }
  }

  // Ensure it starts with a letter (prefix with 'n' if it starts with a number)
  if (/^\d/.test(varName)) {
    varName = 'n' + varName
  }

  // Fallback to a default name if empty
  if (varName === '') {
    varName = 'token'
  }

  return varName
}

/**
 * Convert color value to Flutter Color
 */
function convertColorValue(value) {
  if (typeof value === 'string') {
    // Handle hex colors
    if (value.startsWith('#')) {
      const hex = value.replace('#', '')
      if (hex.length === 3) {
        // Convert short hex to full hex
        const r = hex[0] + hex[0]
        const g = hex[1] + hex[1]
        const b = hex[2] + hex[2]
        return `Color(0xFF${r}${g}${b})`
      } else if (hex.length === 6) {
        return `Color(0xFF${hex})`
      } else if (hex.length === 8) {
        // RGBA format
        return `Color(0x${hex})`
      }
    }
    return `Color(0xFF000000) // TODO: Parse ${value}`
  }

  if (typeof value === 'object' && value.colorSpace) {
    // Convert OKLCH to RGB
    const rgb = oklchToRgb(value)
    const alpha = Math.round((value.alpha ?? 1) * 255)
    const r = Math.round(rgb.r * 255)
    const g = Math.round(rgb.g * 255)
    const b = Math.round(rgb.b * 255)

    const hex = [alpha, r, g, b]
      .map(v => v.toString(16).padStart(2, '0'))
      .join('')
      .toUpperCase()

    return `Color(0x${hex})`
  }

  return 'Color(0xFF000000) // Unknown color format'
}

/**
 * Convert OKLCH color to RGB
 * Based on the OKLCH color space conversion
 */
function oklchToRgb(oklch) {
  const { components, alpha = 1 } = oklch
  const [L, C, H] = components

  // Convert OKLCH to OKLAB
  const hRad = (H * Math.PI) / 180
  const a = C * Math.cos(hRad)
  const b = C * Math.sin(hRad)

  // Convert OKLAB to linear RGB
  const l_ = L + 0.3963377774 * a + 0.2158037573 * b
  const m_ = L - 0.1055613458 * a - 0.0638541728 * b
  const s_ = L - 0.0894841775 * a - 1.291485548 * b

  const l = l_ * l_ * l_
  const m = m_ * m_ * m_
  const s = s_ * s_ * s_

  let r = +4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s
  let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s
  let b_ = -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s

  // Apply gamma correction (sRGB)
  r = srgbGamma(r)
  g = srgbGamma(g)
  b_ = srgbGamma(b_)

  // Clamp values
  r = Math.max(0, Math.min(1, r))
  g = Math.max(0, Math.min(1, g))
  b_ = Math.max(0, Math.min(1, b_))

  return { r, g, b: b_, alpha }
}

/**
 * Apply sRGB gamma correction
 */
function srgbGamma(value) {
  if (value <= 0.0031308) {
    return 12.92 * value
  }
  return 1.055 * Math.pow(value, 1 / 2.4) - 0.055
}

/**
 * Convert dimension value to Flutter double
 */
function convertDimensionValue(value, tokens) {
  if (typeof value === 'number') {
    return value.toString()
  }

  if (typeof value === 'string') {
    // Check for token reference pattern: {token.id}
    const tokenRefMatch = value.match(/^\{(.+)\}$/)
    if (tokenRefMatch) {
      const tokenId = tokenRefMatch[1]
      // If referencing base.* tokens, inline the resolved literal value
      if (tokenId.startsWith('base.')) {
        const baseToken = tokens ? tokens[tokenId] : null
        if (baseToken) return convertDimensionValue(baseToken.$value, tokens)
      }
      return tokenIdToVarName(tokenId)
    }

    // Parse rem, px, etc.
    const match = value.match(/^([\d.]+)(rem|px|em)?$/)
    if (match) {
      const num = parseFloat(match[1])
      const unit = match[2]

      if (unit === 'rem' || unit === 'em') {
        // Convert rem to pixels (assuming 16px base)
        return (num * 16).toString()
      } else if (unit === 'px') {
        return num.toString()
      }
      return num.toString()
    }
  }

  if (typeof value === 'object' && value.value !== undefined) {
    const num = value.value
    const unit = value.unit

    if (unit === 'rem' || unit === 'em') {
      return (num * 16).toString()
    }
    return num.toString()
  }

  return '0.0'
}

/**
 * Convert font weight to Flutter FontWeight
 */
function convertFontWeight(value) {
  if (typeof value === 'number') {
    const weightMap = {
      100: 'FontWeight.w100',
      200: 'FontWeight.w200',
      300: 'FontWeight.w300',
      400: 'FontWeight.w400',
      500: 'FontWeight.w500',
      600: 'FontWeight.w600',
      700: 'FontWeight.w700',
      800: 'FontWeight.w800',
      900: 'FontWeight.w900',
    }
    return weightMap[value] || `FontWeight.w${value}`
  }

  if (typeof value === 'string') {
    const weightNameMap = {
      thin: 'FontWeight.w100',
      extralight: 'FontWeight.w200',
      light: 'FontWeight.w300',
      normal: 'FontWeight.w400',
      regular: 'FontWeight.w400',
      medium: 'FontWeight.w500',
      semibold: 'FontWeight.w600',
      bold: 'FontWeight.w700',
      extrabold: 'FontWeight.w800',
      black: 'FontWeight.w900',
    }
    return weightNameMap[value.toLowerCase()] || 'FontWeight.w400'
  }

  return 'FontWeight.w400'
}

/**
 * Convert font family to Flutter String
 */
function convertFontFamily(value) {
  if (Array.isArray(value)) {
    // Use the first font family
    return `'${value[0]}'`
  }
  if (typeof value === 'string') {
    return `'${value}'`
  }
  return "'sans-serif'"
}

/**
 * Convert border value to Flutter BorderSide
 */
function convertBorderValue(value, tokens) {
  if (typeof value === 'object') {
    const width = value.width
      ? convertDimensionValue(value.width, tokens)
      : '1.0'
    const color = value.color
      ? convertColorValue(value.color)
      : 'Color(0xFF000000)'
    const style =
      value.style === 'dashed' ? 'BorderStyle.none' : 'BorderStyle.solid'

    return `BorderSide(width: ${width}, color: ${color}, style: ${style})`
  }

  return 'BorderSide()'
}

/**
 * Convert shadow value to Flutter BoxShadow list
 */
function convertShadowValue(value, tokens) {
  if (Array.isArray(value)) {
    const shadows = value.map(shadow => {
      const offsetX = shadow.offsetX
        ? convertDimensionValue(shadow.offsetX, tokens)
        : '0.0'
      const offsetY = shadow.offsetY
        ? convertDimensionValue(shadow.offsetY, tokens)
        : '0.0'
      const blur = shadow.blur
        ? convertDimensionValue(shadow.blur, tokens)
        : '0.0'
      const spread = shadow.spread
        ? convertDimensionValue(shadow.spread, tokens)
        : '0.0'
      const color = shadow.color
        ? convertColorValue(shadow.color)
        : 'Color(0x33000000)'

      return `BoxShadow(offset: Offset(${offsetX}, ${offsetY}), blurRadius: ${blur}, spreadRadius: ${spread}, color: ${color})`
    })

    return `[${shadows.join(', ')}]`
  }

  if (typeof value === 'object') {
    const offsetX = value.offsetX
      ? convertDimensionValue(value.offsetX, tokens)
      : '0.0'
    const offsetY = value.offsetY
      ? convertDimensionValue(value.offsetY, tokens)
      : '0.0'
    const blur = value.blur ? convertDimensionValue(value.blur, tokens) : '0.0'
    const spread = value.spread
      ? convertDimensionValue(value.spread, tokens)
      : '0.0'
    const color = value.color
      ? convertColorValue(value.color)
      : 'Color(0x33000000)'

    return `[BoxShadow(offset: Offset(${offsetX}, ${offsetY}), blurRadius: ${blur}, spreadRadius: ${spread}, color: ${color})]`
  }

  return '[]'
}

/**
 * Convert typography value to Flutter TextStyle
 */
function convertTypographyValue(value, tokens) {
  // Handle token reference strings (e.g., "{labelSmall}")
  if (typeof value === 'string') {
    const tokenRef = extractTokenReference(value, tokens)
    if (tokenRef) {
      // Return the variable name directly, as it already points to a TextStyle
      return tokenRef
    }
    return 'TextStyle()'
  }

  if (typeof value !== 'object') {
    return 'TextStyle()'
  }

  const parts = []

  // Font family
  if (value.fontFamily) {
    const fontFamily = extractTokenReference(value.fontFamily, tokens)
    if (fontFamily) {
      parts.push(`fontFamily: ${fontFamily}`)
    } else if (Array.isArray(value.fontFamily)) {
      parts.push(`fontFamily: '${value.fontFamily[0]}'`)
    } else if (typeof value.fontFamily === 'string') {
      parts.push(`fontFamily: '${value.fontFamily}'`)
    }
  }

  // Font size
  if (value.fontSize !== undefined) {
    const fontSize = extractTokenReference(value.fontSize, tokens)
    if (fontSize) {
      parts.push(`fontSize: ${fontSize}`)
    } else {
      const sizeValue = convertDimensionValue(value.fontSize, tokens)
      parts.push(`fontSize: ${sizeValue}`)
    }
  }

  // Font weight
  if (value.fontWeight !== undefined) {
    const fontWeight = extractTokenReference(value.fontWeight, tokens)
    if (fontWeight) {
      parts.push(`fontWeight: ${fontWeight}`)
    } else {
      const weightValue = convertFontWeight(value.fontWeight)
      parts.push(`fontWeight: ${weightValue}`)
    }
  }

  // Line height
  if (value.lineHeight !== undefined) {
    const lineHeightRef = extractTokenReference(value.lineHeight, tokens)

    if (lineHeightRef) {
      // Token reference - check if it's a relative value (number type) or absolute value (dimension type)
      const lineHeightTokenId = extractTokenId(value.lineHeight)
      const lineHeightToken = tokens ? tokens[lineHeightTokenId] : null

      // If it's a number type token (like leading.tight = 1.25), use it directly
      if (lineHeightToken && lineHeightToken.$type === 'number') {
        parts.push(`height: ${lineHeightRef}`)
      } else {
        // It's a dimension type token (like text.2xl.line-height = 2rem), calculate relative value
        if (value.fontSize !== undefined) {
          const fontSize = extractTokenReference(value.fontSize, tokens)
          if (fontSize) {
            parts.push(`height: ${lineHeightRef} / ${fontSize}`)
          } else {
            const sizeValue = convertDimensionValue(value.fontSize, tokens)
            parts.push(`height: ${lineHeightRef} / ${sizeValue}`)
          }
        } else {
          parts.push(`height: ${lineHeightRef}`)
        }
      }
    } else {
      // Direct value - check if it's already a relative number
      if (typeof value.lineHeight === 'number') {
        // Already a relative value
        parts.push(`height: ${value.lineHeight}`)
      } else {
        // Calculate relative line height (lineHeight / fontSize)
        if (value.fontSize !== undefined) {
          const lineHeightValue = parseFloat(
            convertDimensionValue(value.lineHeight, tokens)
          )
          const fontSizeValue = parseFloat(
            convertDimensionValue(value.fontSize, tokens)
          )
          if (
            !isNaN(lineHeightValue) &&
            !isNaN(fontSizeValue) &&
            fontSizeValue > 0
          ) {
            const relativeHeight = lineHeightValue / fontSizeValue
            parts.push(`height: ${relativeHeight}`)
          } else {
            const heightValue = convertDimensionValue(value.lineHeight, tokens)
            parts.push(`height: ${heightValue}`)
          }
        } else {
          const heightValue = convertDimensionValue(value.lineHeight, tokens)
          parts.push(`height: ${heightValue}`)
        }
      }
    }
  }

  // Letter spacing
  if (value.letterSpacing !== undefined) {
    const letterSpacing = extractTokenReference(value.letterSpacing, tokens)
    if (letterSpacing) {
      parts.push(`letterSpacing: ${letterSpacing}`)
    } else {
      const spacingValue = convertDimensionValue(value.letterSpacing, tokens)
      parts.push(`letterSpacing: ${spacingValue}`)
    }
  }

  if (parts.length === 0) {
    return 'TextStyle()'
  }

  // Leading distribution
  parts.push('leadingDistribution: TextLeadingDistribution.even')

  return `TextStyle(${parts.join(', ')})`
}

/**
 * Extract token ID from a value string
 * Returns the token ID like "font-weight.bold" from "{font-weight.bold}"
 */
function extractTokenId(value) {
  if (typeof value === 'string') {
    const match = value.match(/^\{(.+)\}$/)
    if (match) {
      return match[1]
    }
  }
  return null
}

/**
 * Extract token reference from a value string
 * Converts token references like "{font-weight.bold}" to "font_weightBold"
 */
function extractTokenReference(value, tokens) {
  if (typeof value === 'string') {
    // Match token reference pattern: {token.id}
    const match = value.match(/^\{(.+)\}$/)
    if (match) {
      const tokenId = match[1]
      // If referencing base.* tokens, inline the resolved literal value
      if (tokenId.startsWith('base.') && tokens && tokens[tokenId]) {
        const referenced = { ...tokens[tokenId], $originalValue: undefined }
        return convertTokenValue(referenced, tokens).value
      }
      return tokenIdToVarName(tokenId)
    }
  }

  return null
}

/**
 * Generate comment for token
 */
function generateComment(token) {
  if (token.$description) {
    return `/// ${token.$description}`
  }
  return null
}
