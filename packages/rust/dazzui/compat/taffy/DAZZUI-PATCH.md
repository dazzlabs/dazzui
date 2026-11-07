# Taffy layout patches

Source: the published `taffy` 0.9.0 crate, MIT, with its source provenance in
`.cargo_vcs_info.json`. GPUI pins this version.

The main-axis auto-margin branch subtracts gaps from available free space
but omits their positional offsets. A trailing control consequently lands
one gap short of the edge, and intermediate children lose their gaps.
Keep the existing free-space distribution and insert the gap offsets in
visual order for rows, columns and their reverse directions. This is layout
compatibility; DazzUI dimensions remain in `ds`.

The DazzUI workspace and standalone GPUI compatibility crate both patch
Taffy to this committed source. `tests/layout.rs` verifies native child
positions with auto margins in all four flex directions.

Measured text nodes can supply a vertical baseline through
`set_vertical_baseline`. Changing or clearing it invalidates ancestor layout
caches. Flex containers propagate the first child's baseline with each line's
alignment offset exactly once; upstream omitted the line offset and counted
the child's cross-axis offset twice. Native layout tests cover mixed text,
inline controls, centered children and wrapped lines.
