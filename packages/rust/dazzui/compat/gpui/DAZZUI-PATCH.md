# GPUI compositor patch

Source: the published `gpui` 0.2.2 crate, Apache-2.0, from Zed revision
`69e2130295c2649963eb639fc70b4f2ee8ea1624` (the published archive records a
dirty source tree). The upstream license and source are retained here.

The Metal renderer adds isolated subtree compositing. GPUI's original
`opacity` multiplies every primitive separately and its overflow mask is a
rectangle. CSS applies opacity after composing the subtree and clips children
to the parent's rounded padding edge. These differences are visible in
DazzUI's Dialog even when its layout and colors agree with React.

The patch adds nested scene groups, offscreen Metal passes and premultiplied
group blending, and uses them for opacity, rounded overflow and uniform
subtree scale. Scaling preserves layout and maps hitboxes, pointer events,
IME rectangles and character probes between the same coordinate systems.
The input transform participates in cached views' keys so animated parents
cannot reuse stale hitboxes. Other
renderers retain their upstream behavior. This is a renderer compatibility
patch; design decisions and dimensions remain in `ds`.

Interaction styles merge individual text properties with the base style.
Upstream replaces the entire text refinement when hover or active supplies
only a color, dropping a control's font size, weight and leading. The patch
keeps those properties while applying the state's ink, so pressing a button
cannot change its text width.

Keep local edits limited to this rendering seam. Do not edit an installed
Cargo registry cache: the committed copy is the build input.

The compositor's scene tests run from the repository root with:

```sh
cargo test --manifest-path packages/rust/dazzui/compat/gpui/Cargo.toml \
  --lib compositor_ --features runtime_shaders,test-support \
  --target-dir packages/rust/dazzui/target
```
