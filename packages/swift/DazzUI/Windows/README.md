# Windows assets

FluentIcons contains the 47 glyphs referenced by the shared DazzUI library and
gallery. `tools/codegen/swiftui-windows.mjs` extracts them from the installed
`@fluentui/react-icons` package (the repository lockfile pins the version).
Run `pnpm swiftui:windows:icons` after adding a glyph; `--check` verifies the
checked-in Swift enum and SVG files without writing.

Artwork: Microsoft Fluent UI System Icons, copyright Microsoft Corporation,
[MIT licensed](https://github.com/microsoft/fluentui-system-icons/blob/main/LICENSE).
The license is included in the resource bundle at `FluentIcons/Resources/LICENSE.txt`.
These application-owned assets are independent of swiftui-port.
