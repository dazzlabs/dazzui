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

Window-anchored elements snap their final position to display pixels before
subtracting their parent's origin. Rounding the local translation to whole
logical pixels shifted window edges when the parent started on a half pixel.

Interaction styles merge individual text properties with the base style.
Upstream replaces the entire text refinement when hover or active supplies
only a color, dropping a control's font size, weight and leading. The patch
keeps those properties while applying the state's ink, so pressing a button
cannot change its text width.

`Window::observe_element_paint` records an element's computed surface style
and actual submitted text/SVG paint inputs. Nested observation scopes own
their own surface, text and glyphs. It records both the layout root and the
first decorated surface for collectors that need to look through an unstyled
control wrapper. Custom path submission is flagged separately: tessellated
meshes do not preserve CSS radii or stroke widths. Text size and leading
come from the painter; font weights remain unknown when a font ID aliases different requested
weights. The observer does not reconstruct values from design tokens or
substitute a desired style for the text that was drawn.

`WrappedLine::set_color` updates a cached plain label's foreground without
reshaping it. This lets hover ink reach the glyph painter, rather than only
changing the parent element's style.

`Styled::baseline_offset` gives measured elements an explicit baseline relative
to their border box. Text supplies its font baseline to layout and paints at
that same baseline. `Window::text_baseline` uses the requested, unrounded line
height so fractional leading does not move glyphs by a logical pixel.
`WrappedLine::baseline_offset` exposes the shaped line's original baseline for
that translation. Paint observations include the actual submitted baseline in
transformed window coordinates.

Pointer down and release handlers are registered together in every frame.
A click that completes before the next paint therefore clears active styling
immediately, rather than leaving the control pressed until another release.

Keep local edits limited to this rendering seam. Do not edit an installed
Cargo registry cache: the committed copy is the build input.

The compositor's scene tests run from the repository root with:

```sh
cargo test --manifest-path packages/rust/dazzui/compat/gpui/Cargo.toml \
  --lib compositor_ --features runtime_shaders,test-support \
  --target-dir packages/rust/dazzui/target
```
