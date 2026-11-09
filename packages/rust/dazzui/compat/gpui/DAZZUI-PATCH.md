# GPUI compositor patch

Source: the published `gpui` 0.2.2 crate, Apache-2.0, from Zed revision
`69e2130295c2649963eb639fc70b4f2ee8ea1624` (the published archive records a
dirty source tree). The upstream license and source are retained here.

The Metal, Blade and DirectX renderers add isolated subtree compositing.
GPUI's original `opacity` multiplies every primitive separately and its overflow mask is a
rectangle. CSS applies opacity after composing the subtree and clips children
to the parent's rounded padding edge. These differences are visible in
DazzUI's Dialog even when its layout and colors agree with React.

The patch adds nested scene groups, offscreen passes and premultiplied
group blending, and uses them for opacity, rounded overflow and uniform
subtree scale. Scaling preserves layout and maps hitboxes, pointer events,
IME rectangles and character probes between the same coordinate systems.
The input transform participates in cached views' keys so animated parents
cannot reuse stale hitboxes. A parent's destination clip is inverse-mapped
into the layer's source coordinates before prepaint and paint, so scaling
does not discard children that move inside the clip. Blade reuses a cleared
texture per nesting depth and uses premultiplied primitive pipelines inside
each group. DirectX uses cleared BGRA layers with source-over alpha and samples
each layer through a premultiplied blend pipeline. This is a renderer
compatibility patch; design decisions and dimensions remain in `ds`.

Layer source bounds follow the inverse-mapped clip, including negative and
off-window coordinates that a scale brings into view. Nested opacity retains
that source clip. Materialized scenes use pixel-aligned local texture origins;
the cached operation log remains in window coordinates. All three renderers size
their intermediate textures for every source layer in the frame, independently
of the destination window. Path rasterization and sampling use the actual
intermediate extent while root drawing retains the window's projection.
The hardware suite checks all four off-window edges under scale and nested
opacity, and mask/image/path rendering as the intermediate extent grows and
shrinks. The same off-window scene is compared with CSS in Chromium.

Styled backgrounds, borders and shadows snap their box to logical CSS pixels
before device scaling. Each drawable retains Taffy's unrounded absolute bounds
through prepaint and paint, so a fractional origin is not rounded once for the
display and again for CSS. The layout API retains its display-pixel rounding;
low-level primitive drawing still accepts its supplied geometry. Cached views
include precise bounds in their reuse key and measure their subtree against
those bounds, preserving paint when a precise edge crosses a logical pixel
while the display-rounded box stays the same. Equal precise geometry reuses
cached paint.
The background minimum-one-pixel rule exempts sizes at or below 4/64px that
round to zero. Rounded padding clips use the separate allowing-zero rule,
including when an extremely narrow background would otherwise paint one pixel.
Rectangular overflow clips keep the normal snapping rule. Paint clipping and
its ellipse constraints derive from the same rounded inner rectangle. The
rounding rules follow Blink's LayoutUnit and rounded-border geometry; the
retained BSD terms are in `LICENSE-CHROMIUM`.

Overflow clips use the padding edge even with transparent or unpainted borders.
Each inner corner keeps separate horizontal and vertical radii after subtracting
its adjoining border widths. If an opposite border leaves too little room,
both axes are constrained together to fit the padding rectangle, matching
Chromium's rounded overflow clip. All three renderers retain both ellipse axes through
scene materialization, cached paint replay and the fragment clip. General subtree
clips use circle signed-distance coverage and normalize elliptical implicit
equations by their gradients; the single-fill paths below use backend-specific
meshes, retaining remaining differences in browser diagnostics.
Windows and Linux additionally match Ganesh's solid-fill optimization: one
unrounded solid quad covering the entire clip is drawn as a filled rrect.
Simple circular rrects retain analytic distance coverage; complex corners and
ovals use the shared `scene/solid_rrect.rs` mesh adapted from Skia's
`FillRRectOp` (the retained BSD terms are in `LICENSE-SKIA`). Its constrained
raster radii, linear edge strips and L1 arc derivatives do not change logical
clip radii or hitboxes. DirectX and Blade draw the mesh directly with a
premultiplied solid pipeline. Gradients, images and multiple draws retain the
existing composited clip.
On macOS, a single covering rectangle, image or rectangular opacity layer uses the filled Graphite
`AnalyticRRectRenderStep` mesh. The shared `scene/graphite_rrect.rs` helper
retains the original local positions while outsetting outer vertices by one
device pixel. Interpolated outset correction and implicit L2 distances produce
coverage for both circular and elliptical corners; exact circle distance alone
misses small radii. Metal and macOS Blade use the same mesh and retain ancestor
clipping, source texture coordinates and group opacity. Uniform positive scales
apply to vertex placement and distance conversion; subpixel dimensions use
Graphite's coverage scale and bias.

Native Metal and macOS Blade also process complex clips per draw, following the pinned
Chrome 157 [Graphite clip stack](https://skia.googlesource.com/skia/+/ca84ce0a6f1659748188469a89361ccea71e3a64/src/gpu/graphite/ClipStack.cpp).
A rectangle, image or axis-aligned opacity layer can fold its intersection into
a filled rrect when every surviving corner remains whole. Cuts through a curve
retain the clip instead. If every draw folds, the layer stays single-sampled;
otherwise a four-sample stencil clip retains coverage through child blending.
Analytic draws bypass that stencil, while other draws test it. This distinction
also affects mesh interpolation in a mixed MSAA pass.
Clip boundaries use rational conic wedges at quarter-device-pixel precision,
with Wang's segment bound and homogeneous chopping above the fixed 32-segment
patch limit. The stencil and multisample color attachments are retained across
nested composites and path raster passes, cleared for each new source layer,
and reused by depth and extent. Ordinary batches stay batched. Scene encoding
restores its pipeline mode on success and buffer-growth errors.
This clip pass currently handles unscaled axis-aligned layers with integral
translations; other complex transforms retain the general composited clip.
Blade retains its multisample attachments across pass breaks and resolves only
after the complete clipped subtree; its resolve operation does not guarantee
preservation of the original samples. Its Metal backend uses a combined
depth/stencil attachment, with depth writes disabled. Source licensing for the
adapted Graphite mesh and tessellation is in `LICENSE-SKIA`.
The rounded layer retains its ancestor rectangle separately from its own
clip. This preserves the mesh's half-device-pixel AA fringe, while the
ancestor still clips it. Overlap ordering conservatively includes that fringe;
a later adjacent primitive cannot be reordered beneath it. The GPU suite
checks the fringe against an ancestor clip, a later fill and group opacity.

Visible/clip axis pairs preserve the parent's extent on the visible axis and do
not round the clip. Hidden or scroll paired with visible clips both axes, as CSS
computes the visible axis to auto. This patch changes the draw/prepaint clip;
GPUI's explicit scroll-handle and layout APIs remain in place.
The geometry follows [CSS corner shaping](https://www.w3.org/TR/css-backgrounds-3/#corner-shaping)
and the inner-clip constraint observed in Chromium's `FloatRoundedRect`/
`SkRRect` conversion. The wide-border fixture checks that constraint against
the rendered browser, including partially covered edge pixels.

Input keeps layout rectangles at 1/64px precision while rounded border and
padding-edge curves use their pixel-snapped geometry. A Div inserts
its own hitbox before applying its padding-edge clip to descendants, so a
transparent border remains interactive. Rounded hit clips form a shared,
persistent chain, with each ancestor's inverse transform captured when the clip
is established. Nested and scaled children test every ancestor in that chain;
cached views include its geometry in their reuse key. Equal geometry still
reuses cached views, while changing only a radius updates their hitboxes.
Rounded-curve queries retain inclusive edge intersection even if a snapped
inner clip collapses to a line; the independent layout rectangle still requires
a nonempty intersection. Mouse hit queries round their origin to 1/64px and use
the browser's 1x1 layout-pixel aperture. Wheel targeting uses a separate integer-position query
without changing the event coordinates or subpixel hover state. The override
is scoped to event dispatch and is restored afterward.
Frame queries always use window coordinates, including redispatch from a
scaled listener. Listener event positions and `mouse_position()` still use
that listener's layout space; the transform is applied once at the callback
boundary rather than again during hit testing.
`rounded-hitbox.cases.json` drives native component and headless browser checks
for hover, click and wheel targeting across borders, elliptical inner corners,
visible/hidden/single-axis overflow, nesting and parent/child scaling. Fractional
origins straddle logical-pixel boundaries, and narrow boxes retain the different
background, rounded-clip and layout extents. DOM point queries have separate
expectations where their floored precision differs from actual mouse events.
Both the browser and native tests check 1x and 2x; native tests also exercise
cached and uncached child targets. Test windows can change pixel density through
the production resize callback while retaining the same logical viewport.
These are input/layout observations; native OS
event delivery still requires real-window validation.

`Window::capture_scene` re-renders the last submitted scene to an app-owned
texture with the native presentation pipeline, including nested
composites, path antialiasing and glyph atlases. It waits for GPU completion before
reading a shared buffer, removes row alignment and returns straight-alpha
sRGB RGBA pixels. Metal and Blade declare that same display color space. This
diagnostic capture does not include OS overlays or compositor effects and
does not require Screen Recording. Linux's X11 and Wayland windows delegate
to Blade's readback. Blade accepts the RGBA/BGRA 8-bit unorm formats used by
its sRGB surface; other formats explicitly fail capture. DirectX copies its
BGRA render target into a staging texture and maps it for a blocking CPU read,
removing row pitch and converting premultiplied BGRA to straight RGBA.

All three renderers clamp fragment texture coordinates to each atlas tile's texel
centers, retaining the interior pixel mapping while preventing enlarged masks and images
from filtering neighboring allocations. Their polychrome atlases store
premultiplied BGRA, converted once from the platform's straight-alpha bytes;
the fragment shaders preserve premultiplied interpolation and convert only
for a straight-alpha blend pipeline. Metal's primitive pipeline receives
straight colors after filtering; its composite pipeline receives premultiplied
layers. Blade readback uses the surface's actual root blend mode, while isolated
groups always use premultiplied pipelines. Shared integer alpha conversion
rounds to the nearest 8-bit value and preserves saturated ink at every nonzero
alpha; floating-point truncation previously exported some 255s as 254s.
Swash's layered color outlines are normalized to straight BGRA at the Linux
text-system boundary, matching decoded images and macOS color glyphs. DirectWrite
advances its color-run enumerator before reading each layer, unmaps the completed
staging read and normalizes straight BGRA with the shared integer conversion.
The macOS family loader accepts faces without a Latin `m`, matching Linux;
missing-character metric requests already return an error. A valid symbol or
non-Latin face is not rejected before its requested characters can be shaped.

Blade's shared underline/strikethrough fragment path applies ink alpha once,
then multiplies geometric wave coverage and group opacity independently.
The GPU regression compares translucent decorations with opaque decorations
at the same geometry across the three surface alpha modes, five ink alphas,
straight and wavy lines, clipping and isolated groups. It verifies alpha
composition rather than claiming the wave shape matches CSS text decoration.

Window-anchored elements snap their final position to display pixels before
subtracting their parent's origin. Rounding the local translation to whole
logical pixels shifted window edges when the parent started on a half pixel.

Interaction styles merge individual text properties with the base style.
Upstream replaces the entire text refinement when hover or active supplies
only a color, dropping a control's font size, weight and leading. The patch
keeps those properties while applying the state's ink, so pressing a button
cannot change its text width.

Font selection matches on the weight each face draws at, read from the
`wght` variation axis, rather than on the weight its file declares. A
variable instance keeps its face's own trait — SF Pro's Medium is 530 on
macOS 15 — and CSS face matching handed a 500 it cannot find among the
traits prefers a 400 face over the nearer one, so medium text drew regular
there. The candidates' weights come from `font_kit::css_properties`.

`Window::observe_element_paint` records an element's computed surface style
and actual submitted text/SVG paint inputs. Nested observation scopes own
their own surface, text and glyphs. It records both the layout root and the
first decorated surface for collectors that need to look through an unstyled
control wrapper. Custom path submission is flagged separately: tessellated
meshes do not preserve CSS radii or stroke widths. Text size and leading
come from the painter; font weights remain unknown when a font ID aliases different requested
weights. The observer does not reconstruct values from design tokens or
substitute a desired style for the text that was drawn.
An opt-in `observe_element_paint_with_surfaces` records every own surface in
submission order for compound-control checks, including a Switch's track and
thumb. Ordinary collection retains the smaller first-surface observation.

`WrappedLine::set_color` updates a cached plain label's foreground without
reshaping it. This lets hover ink reach the glyph painter, rather than only
changing the parent element's style.

Line truncation measures complete shaped strings, including the suffix,
instead of summing isolated character advances. Kerning and ligatures can
otherwise cause a label to truncate at its own intrinsic width. Comparisons
use 1/64px layout precision and retain extended grapheme clusters, including
combining accents and joined emoji. This shared path covers both kit labels
and ordinary GPUI text.
Ordinary text's layout cache includes truncation width as well as wrapping
width. Each measurement starts with the original style runs; otherwise an
intrinsic measurement can suppress a later ellipsis, or a previously shortened
run can leak into a wider measurement. A flex-layout regression checks the
painted text while narrowing and widening the same view.

`InteractiveElement::map_paint_style` lets the kit interpolate the resolved
interaction colors immediately before surface and child paint. It does not
replace hover/active state or change layout. Paint observations receive this
final style, so they report the actual intermediate colors.

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

Default scroll handling honors `Window::prevent_default` without suppressing
ordinary ancestor wheel listeners. A focused NumberField can consume wheel
input for stepping while the gallery's document stays still; default fields,
unfocused fields and Ctrl-wheel retain the enclosing page's scrolling path.

Keep local edits limited to this rendering seam. Do not edit an installed
Cargo registry cache: the committed copy is the build input.

Single-line shaping coalesces adjacent runs only when their resolved fonts
and decorations agree. Otherwise a same-color fallback run silently keeps the
preceding font. On macOS, fallback families match the requested weight and
style independently of the primary face's variable axes. Logical font IDs and
attributed-string boundaries preserve requests that share a physical face.
AAT tracking replaces Core Text's tracking with browser-precision advances.
Continuation marks retain their glyph order after shaping, including Indic
pre-base marks, instead of using logical Unicode graphemes as painted groups.
Tracked variable outlines round their hmtx variation deltas in design units
before scaling, matching the browser's HarfBuzz advances; bitmap fonts and
fonts without tracking retain Core Text's fractional metrics. Fonts without
STAT receive no browser tracking. Emoji uses the
public fallback face, matching browser metrics rather than the scaled system
UI face. Its bitmap strikes and ink bounds are resolved at device size, with
fractional glyph positioning, rather than enlarged from a logical-size strike.
macOS glyph rasterization keeps grayscale antialiasing and subpixel positioning
but disables Core Graphics font smoothing, matching the example's browser
grayscale text. That separate smoothing switch emboldens even alpha-only
masks; native/browser glyph captures verify the difference independently of
the font's weight and shaping advances.
The font-kit compatibility layer reads static CSS weights from OS/2
and variable instance weights from Core Text's resolved variation values,
including the axis default when a named face sits at that default. Descriptor
attributes alone can omit the selected instance's weight.

Bidirectional lines retain native leading and trailing insertion edges from
Core Text, DirectWrite hit testing or cosmic-text's cluster boxes. A line uses
the default LTR paragraph direction, including when its first word is RTL.

On macOS, RTL runs whose native glyph and source-cluster identities agree
with HarfBuzz use HarfRust positioning over the selected face's public Core
Text tables. Fonts and variation axes stay native; table and layout data are
cached without private font paths. GPOS runs at browser 16.16 precision, with
native unpositioned advances or the existing tracked-font design-unit rounding
supplied before positioning. This fixes reversed Latin pair kerning without
moving errors into a compensating selection width. Native insertion coordinates
are remapped through the same positioned glyphs. Invisible format glyphs are
filtered after shaping so their removal cannot merge neighbouring source
clusters. Runs with different substitutions or source partitions keep the
native path; this seam does not replace that source map.

On macOS, soft rows are shaped again with their full paragraph's pre/post
context rather than slicing already substituted glyphs. An overwide ligature
breaks at its first grapheme and the remaining cluster is reshaped before the
next fit decision. This preserves Arabic joining while keeping glyphs on both
sides of a broken ligature. Core Text resolves fallback faces; their native
size ratio and variation axes remain part of the font ID and follow metrics,
rasterization and row shaping. Fresh rows use HarfRust's own AAT tracking
partition at the actual point size; native-run repositioning omits the tracking
table and keeps the existing native tracking map. Runs are ordered at their
resolved bidi levels, including separate fallback fonts inside an RTL phrase.
Trailing hanging spaces allocate their own inline item on the 1/64px grid.
Other backends retain the existing row path when contextual shaping is absent.

`WindowTextSystem::new` exposes an independent native layout cache for probes;
it does not require a platform window or substitute synthetic fonts.

The weaker adjacent embedding level selects the ordinary caret; mouse clicks
choose the nearest of these ordinary positions, keeping logical selection
consistent with browser inputs at direction changes. Single-row selection
and IME range rectangles use character edges, preserving disjoint visual
selection fragments. Wrapped bidi paragraphs break in logical source order at
Unicode line opportunities, retaining native grapheme advances. Each row applies
L1/L2 with the paragraph's resolved levels, so continuing RTL runs, embeddings
and trailing whitespace reorder within that row. Glyphs, caret, hit and selection
geometry use the same logical row range.
`TextLayout::wrapped_text` reads those source boundaries too.
Selection fragments retain their row origins. LTR soft breaks and hard separators
select one extra space advance; continuing RTL runs select their trailing space
once. Absolute highlight edges snap to CSS pixels before device scaling, and
hanging spaces can paint into the textarea's inline padding.
On macOS, paragraph-final L1 levels match native visual layout. Selection edges
are bounded to the shaped advance, formatting controls have no selection ink,
and clusters before those controls recover their edge from native advancing
glyphs, excluding combining marks. Invisible bidi controls share the ordinary
caret on the weaker surrounding bidi level. Combining graphemes retain their
whole selection extent, independently of native scalar insertion positions.
Inline fragment allocation ignores zero-ink control boxes attached to a distant
visual edge. Carets and highlights share that allocation on the 1/64px inline
grid, before whole-CSS-pixel selection snapping. Otherwise a native edge just
below a half pixel can lose a full highlight column.
Wrapped rows retain their already allocated origins instead of reallocating
fragments from rounded character edges.
When Core Text omits an internal ligature stop beside a control, an invalid
outer-edge offset is replaced by the advancing cluster's grapheme subdivision;
valid native internal stops remain unchanged.
Visible characters retain native insertion coordinates separately, so correcting
a highlight does not replace their ordinary caret positions. Wrapped endpoints are
rebased to their logical row.
`example/tool/field-selection.mjs` compares actual native and React selection
pixels on macOS, with submitted quads available as diagnostics. Both editors
use transparent foreground and opaque magenta selection in this probe; fonts
and layout remain unchanged. Paired partial pixels on antialiased curves are
counted separately, while opaque interiors and flat edges retain strict pixel
comparison. The probe reads the real browser editor's layout through DevTools.
Its fixtures include pure RTL, explicit embeddings, hard and soft separators
and emergency Arabic word wrapping. Per-character control-boundary cases also
cover the last letter, combining marks and ligatures before a closing delimiter.
`font-metrics` accepts an optional `wrap` width to expose native row source ranges,
glyphs and insertion edges, embedding levels and glyph IDs for further
inspection. These probes cover their samples rather than all
Unicode shaping or text rasterization.
A break that bisects a native ligature still needs contextual reshaping; the
native row probe exposes a glyph-empty fragment at widths below that ligature's
advance. This remains an explicit rendering gap on macOS and Linux.
The selection probe retains measured macOS differences in reversed Latin pair
kerning. These are failures, not accepted tolerances. Font range rectangles are
diagnostics: actual selection paint also includes the row's native leading.
Native stops retain the following character's embedding level so DazzUI's
editors can allocate bidi fragments on the browser's 1/64px inline grid while
keeping native shaping advances available unchanged.
DirectWrite source conversion supports backward seeks between visual runs,
and its glyph placement preserves native run origins, direction and offsets.
All backends expose glyph Y offsets in downward-positive coordinates; paint
and observation apply those offsets to positioned marks. The text system also
exposes native line gap so editors can include normal inline leading without
changing caret height.
On macOS, combining marks are canonically reordered inside each font run
before shaping. This prevents Core Text from placing a reordered accent an
entire em below its base. A source map restores original UTF-8 glyph indices
and UTF-16 insertion queries; the editor's value is preserved verbatim.
Core Text insertion positions are retained for left-to-right lines as well.
Glyph order can put an Indic pre-base mark after the queried source index
while painting it at the cluster's beginning; searching those glyph indices
places the caret on the wrong side. Native insertion offsets preserve the
correct cluster edge and the line's tracking adjustments.
Ordinary LTR grapheme edges use painted glyph origins, retaining native
interpolation for boundaries inside a ligature without their own glyph.
Interior character indices share the downstream grapheme edge and are
excluded from mouse insertion stops.

DirectWrite treats glyph zero as missing coverage, enabling the authored
fallback stack instead of accepting .notdef. Text layout preserves requested
weights and styles and enables pair kerning and the default `kern` feature;
an explicit `kern=0` remains disabled. Physical face identity includes
simulations and variation axes. Fallback glyph runs retain the exact face
DirectWrite shaped, so rasterization cannot silently select another face
from the same family.

The Linux loader accepts PostScript names as well as family names and no
longer requires a Latin `m` glyph in otherwise valid Arabic or emoji faces.
Logical synthetic bold and italic faces retain the selected physical face's
shaping advances; Swash applies synthesis when drawing the outline. Both
raster bounds and pixels come from the same synthesized image. Linux outline
shape and raster sizes use the browser's 1/64px font-size precision. Bitmap-only
fonts use embedded strike advances at the requested size, updating following
glyph positions and bidi edges together. Their raster size retains that same
requested size rather than the outline grid. Native fixture
tests exercise non-Latin coverage and synthetic bold's ink and advances;
they are explicitly ignored unless invoked on a host with Noto Color Emoji
and Noto Naskh Arabic installed:

```sh
cargo test --offline --manifest-path compat/gpui/Cargo.toml --lib \
  native_font_tests --features test-support --target-dir target -- --ignored
```

`Window::focused_input_snapshot` reads the installed platform input handler's
actual text and UTF-16 selection for opt-in native interaction checks. It does
not infer selection from the painted caret.

`Window::unrounded_layout_size` exposes Taffy's intrinsic size before display
pixel snapping. Tabs rounds that value once to logical CSS pixels for its
hidden ruler; rounding an already snapped rectangle changes overflow choices
at exact-fit boundaries. Painting continues to use the ordinary snapped boxes.

The compositor's scene tests run from the repository root with:

```sh
cargo test --manifest-path packages/rust/dazzui/compat/gpui/Cargo.toml \
  --lib compositor_ --features runtime_shaders,test-support \
  --target-dir packages/rust/dazzui/target
```

Metal, Blade and DirectX share `src/platform/renderer_tests.rs`. Their GPU readback
tests run without creating a platform window. Metal owns an unattached layer
and renders to app-owned textures; it never acquires or presents a drawable.
Its autorelease pool outlives the renderer and the GPU readback completes before
resources are released. DirectX uses a plain BGRA render target and staging
readback, with no HWND or swap chain. This also works from SSH session 0,
where DXGI presentation swap chains are unavailable.
The Windows fixture uses the production BGRA premultiplied format and requires
a hardware adapter. The font probe uses each platform's actual glyph rasterizer.
From `packages/rust/dazzui`, with the host's native graphics libraries set:

```sh
mkdir -p target/renderer-gpu-probe
DAZZUI_GPU_PROBE_OUT="$PWD/target/renderer-gpu-probe" \
  CARGO_INCREMENTAL=0 cargo test --offline \
  --manifest-path compat/gpui/Cargo.toml --target-dir target \
  --config 'patch.crates-io.zed-font-kit.path="compat/font-kit"' \
  --config 'patch.crates-io.taffy.path="compat/taffy"' \
  --features runtime_shaders,test-support --lib gpu_tests \
  -- --nocapture --test-threads=1
```

They exercise the production scene encoder, GPU textures and readback for
opaque and translucent overlapping children under group opacity, nested rounded
overflow, scaled parent clipping, texture reuse across siblings/frames/resizing, atlas masks,
color images, paths, BGRA channel/alpha conversion and enlarged transparent
images. Blade exercises all three surface alpha modes; Metal exercises its
production BGRA premultiplied surface, as does DirectX. The logged device
identifies whether the driver is hardware or software. These probes establish
compositing behavior; synthetic atlas masks do not establish font rasterization
or full-gallery pixel parity.

`example/tool/blade-compositor.mjs --gpu --native target/renderer-gpu-probe --out <dir>`
accepts images from all three renderers and compares them with equivalent CSS in
headless Chromium. It uses the React example's installed Playwright and accepts
`DAZZUI_CHROMIUM_EXECUTABLE`. Selected interior and edge probes permit one
8-bit rounding unit; the report retains full-image differences, including
rounded-edge AA, as diagnostics rather than treating those probes as
full-scene conformance.
The current fixtures still expose rounded-edge coverage and one-unit image
interpolation rounding differences; passing probes do not waive them.

`--gpu` explicitly requests hardware compositing and rasterization, selects
ANGLE Vulkan on headless Linux, and rejects SwiftShader/software fallback.
`browser.json` records the browser version, devices, enabled features, active
GL renderer and Skia backend before capture; the final report retains it too.
Hardware validation checks the active renderer, since the adapter inventory can
also include inactive software adapters on Windows. Omitting the flag
preserves Chromium's default, including its software renderer on headless Linux.
That path's different antialiasing can fail the same strict edge probes; its
results are separate evidence, not a reason to widen the pixel tolerance.
On the verified Intel UHD 600/Mesa GaneshGL host and macOS Graphite browser,
all 64 selected probes pass with the filled-rrect meshes. The expanded alpha
matrix below retains the remaining clip-mask and paint-geometry differences;
selected probes do not establish full cross-backend pixel parity.

`fixtures/colr-alpha.ttf` is an original square glyph with a translucent COLR
layer. Regenerate it with `python3 fixtures/colr-alpha.py`. Its ordinary Linux
font test checks actual Swash output and the public glyph-raster boundary,
without requiring a system color font or a platform window.
The GPU font probe feeds a glyph rasterized by Swash or Core Text through
the atlas and group compositor; the browser loads the same font through
`FontFace`. This verifies one translucent glyph, rather than substituting a mask
for font rendering.

Windows SSH session 0 can execute the native texture tests but cannot create
DXGI presentation swap chains, and hardware Chromium screenshot capture fails
there. The browser comparison was run with headless Chrome in the signed-in
user's session through a hidden one-shot scheduled task, removed afterward.
On the verified Iris Xe/Chrome 157 GaneshGL path, the solid-rrect mesh closes
the two wide-border corner failures and all 64 selected probes pass. Full-mask
checks below continue to expose image and split-fill clip differences.

`example/tool/overflow-raster.mjs --gpu --native <capture-dir> --out <results>`
compares every alpha pixel at 1x and 2x in the 32 shared
`fixtures/overflow-raster.json` cases. It records the browser's actual GPU,
Skia backend, resolved clip commands and differences, and exits unsuccessfully
for any pixel exceeding one UNORM unit. The matrix includes tiny, eccentric,
nonuniform and wide-border corners, gradient/image/split fills, uniform scales,
fractional origins and subpixel widths. Marked cases additionally compare
native GPU pixels from real Div layout and paint at 2x, including child content
masks; missing styled captures fail rather than being skipped.
The original 18 cases pass 36/36 alpha masks on both native Metal and macOS
Blade, and 32/36 on Windows/Linux Ganesh. The first 25 cases pass 50/50 on
both macOS backends and 46/50 on Windows/Linux. All five actual
Div cases retain zero alpha difference on every backend.
The complete 32-case matrix adds translucent overlap, an opacity layer,
interleaved draws across a nested layer, an image plus another fill, translated
clips, mixed corner radii and a large cross-viewport clip. It passes 62/64 alpha
masks on both macOS backends and 46/64 on Windows/Linux. The macOS
remaining alpha failures are both large cross-viewport captures; their draw/clip
classification and raster viewport still need investigation.
Four marked color cases additionally gate every premultiplied RGB channel at
the same one-UNORM tolerance, together with alpha. Both macOS backends pass 5/8
of these RGBA masks; nested opacity and image-overlay color rounding still fail.
Windows/Linux retain all eight as failures. The latest Blade comparison reuses
the saved hardware-Chromium reference on this host; it is not a fresh browser run. Missing captures fail rather
than being skipped, and these failures remain in the default report.
The paint-snapping cases exercise both minimum visible backgrounds and
collapsed rounded clips. These paint checks remain separate from the shared browser/native input
fixtures above; passing either does not establish every fractional layout or
native OS event-delivery behavior.
Other premultiplied RGB differences remain diagnostics, including gradient
texture/interpolation rounding. A passing alpha mask or selected probe does
not establish full color or general rounded-clip parity.
