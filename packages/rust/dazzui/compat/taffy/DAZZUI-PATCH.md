# Taffy flex gap patch

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
