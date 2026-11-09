// todo("windows"): remove
#![cfg_attr(windows, allow(dead_code))]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(not(target_os = "windows"))]
mod graphite_rrect;
#[cfg(all(
    not(target_os = "windows"),
    any(not(target_os = "macos"), feature = "macos-blade")
))]
pub(crate) use graphite_rrect::GraphiteRRectVertex;

#[cfg(any(not(target_os = "macos"), feature = "macos-blade"))]
mod solid_rrect;
#[cfg(any(not(target_os = "macos"), feature = "macos-blade"))]
pub(crate) use solid_rrect::SolidRRectVertex;

use crate::{
    AtlasTextureId, AtlasTile, Background, Bounds, ContentMask, Corners, Edges, Hsla, Pixels,
    Point, Radians, ScaledPixels, Size, bounds_tree::BoundsTree, point,
};
use std::{
    fmt::Debug,
    iter::Peekable,
    ops::{Add, Range, Sub},
    slice,
};

#[allow(non_camel_case_types, unused)]
pub(crate) type PathVertex_ScaledPixels = PathVertex<ScaledPixels>;

pub(crate) type DrawOrder = u32;

#[derive(Default)]
pub(crate) struct Scene {
    pub(crate) paint_operations: Vec<PaintOperation>,
    primitive_bounds: BoundsTree<ScaledPixels>,
    layer_stack: Vec<DrawOrder>,
    pub(crate) shadows: Vec<Shadow>,
    pub(crate) quads: Vec<Quad>,
    pub(crate) paths: Vec<Path<ScaledPixels>>,
    pub(crate) underlines: Vec<Underline>,
    pub(crate) monochrome_sprites: Vec<MonochromeSprite>,
    pub(crate) polychrome_sprites: Vec<PolychromeSprite>,
    pub(crate) surfaces: Vec<PaintSurface>,
    pub(crate) composites: Vec<Composite>,
    has_composites: bool,
}

impl Scene {
    pub fn clear(&mut self) {
        self.paint_operations.clear();
        self.primitive_bounds.clear();
        self.layer_stack.clear();
        self.paths.clear();
        self.shadows.clear();
        self.quads.clear();
        self.underlines.clear();
        self.monochrome_sprites.clear();
        self.polychrome_sprites.clear();
        self.surfaces.clear();
        self.composites.clear();
        self.has_composites = false;
    }

    pub fn len(&self) -> usize {
        self.paint_operations.len()
    }

    pub fn push_layer(&mut self, bounds: Bounds<ScaledPixels>) {
        let order = self.primitive_bounds.insert(bounds);
        self.layer_stack.push(order);
        self.paint_operations
            .push(PaintOperation::StartLayer(bounds));
    }

    pub fn pop_layer(&mut self) {
        self.layer_stack.pop();
        self.paint_operations.push(PaintOperation::EndLayer);
    }

    pub fn insert_primitive(&mut self, primitive: impl Into<Primitive>) {
        let mut primitive = primitive.into();
        let clipped_bounds = primitive
            .bounds()
            .intersect(&primitive.content_mask().bounds);

        if clipped_bounds.is_empty() {
            return;
        }

        let order = self
            .layer_stack
            .last()
            .copied()
            .unwrap_or_else(|| self.primitive_bounds.insert(clipped_bounds));
        match &mut primitive {
            Primitive::Shadow(shadow) => {
                shadow.order = order;
                self.shadows.push(shadow.clone());
            }
            Primitive::Quad(quad) => {
                quad.order = order;
                self.quads.push(quad.clone());
            }
            Primitive::Path(path) => {
                path.order = order;
                path.id = PathId(self.paths.len());
                self.paths.push(path.clone());
            }
            Primitive::Underline(underline) => {
                underline.order = order;
                self.underlines.push(underline.clone());
            }
            Primitive::MonochromeSprite(sprite) => {
                sprite.order = order;
                self.monochrome_sprites.push(sprite.clone());
            }
            Primitive::PolychromeSprite(sprite) => {
                sprite.order = order;
                self.polychrome_sprites.push(sprite.clone());
            }
            Primitive::Composite(composite) => {
                composite.order = order;
                self.composites.push(composite.clone());
            }
            Primitive::Surface(surface) => {
                surface.order = order;
                self.surfaces.push(surface.clone());
            }
        }
        self.paint_operations
            .push(PaintOperation::Primitive(primitive));
    }

    pub fn replay(&mut self, range: Range<usize>, prev_scene: &Scene) {
        for operation in &prev_scene.paint_operations[range] {
            match operation {
                PaintOperation::Primitive(primitive) => self.insert_primitive(primitive.clone()),
                PaintOperation::StartLayer(bounds) => self.push_layer(*bounds),
                PaintOperation::EndLayer => self.pop_layer(),
                PaintOperation::StartComposite(
                    bounds,
                    mask,
                    radii,
                    radii_y,
                    opacity,
                    transform,
                ) => self.push_elliptical_composite(
                    *bounds,
                    mask.clone(),
                    *radii,
                    *radii_y,
                    *opacity,
                    *transform,
                ),
                PaintOperation::EndComposite => self.pop_composite(),
            }
        }
    }

    #[cfg(test)]
    pub fn push_composite(
        &mut self,
        bounds: Bounds<ScaledPixels>,
        mask: ContentMask<ScaledPixels>,
        radii: Corners<ScaledPixels>,
        opacity: f32,
        transform: TransformationMatrix,
    ) {
        self.push_elliptical_composite(bounds, mask, radii, radii, opacity, transform);
    }

    pub(crate) fn push_elliptical_composite(
        &mut self,
        bounds: Bounds<ScaledPixels>,
        mask: ContentMask<ScaledPixels>,
        radii: Corners<ScaledPixels>,
        radii_y: Corners<ScaledPixels>,
        opacity: f32,
        transform: TransformationMatrix,
    ) {
        self.has_composites = true;
        self.paint_operations.push(PaintOperation::StartComposite(
            bounds, mask, radii, radii_y, opacity, transform,
        ));
    }

    pub fn pop_composite(&mut self) {
        self.paint_operations.push(PaintOperation::EndComposite);
    }

    pub fn finish(&mut self) {
        if self.has_composites {
            // Keep one flat operation log for cached views' global paint ranges.
            // Only the renderer's batches become a hierarchy of isolated scenes.
            let mut operations = self.paint_operations.iter();
            let rendered = Self::materialize(&mut operations);
            self.primitive_bounds = rendered.primitive_bounds;
            self.layer_stack = rendered.layer_stack;
            self.shadows = rendered.shadows;
            self.quads = rendered.quads;
            self.paths = rendered.paths;
            self.underlines = rendered.underlines;
            self.monochrome_sprites = rendered.monochrome_sprites;
            self.polychrome_sprites = rendered.polychrome_sprites;
            self.surfaces = rendered.surfaces;
            self.composites = rendered.composites;
            self.has_composites = false;
        }
        self.shadows.sort_by_key(|shadow| shadow.order);
        self.quads.sort_by_key(|quad| quad.order);
        self.paths.sort_by_key(|path| path.order);
        self.underlines.sort_by_key(|underline| underline.order);
        self.monochrome_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        self.polychrome_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        self.surfaces.sort_by_key(|surface| surface.order);
        self.composites.sort_by_key(|composite| composite.order);
    }

    fn materialize<'a>(operations: &mut impl Iterator<Item = &'a PaintOperation>) -> Scene {
        let mut scene = Scene::default();
        while let Some(operation) = operations.next() {
            match operation {
                PaintOperation::Primitive(primitive) => scene.insert_primitive(primitive.clone()),
                PaintOperation::StartLayer(bounds) => scene.push_layer(*bounds),
                PaintOperation::EndLayer => scene.pop_layer(),
                PaintOperation::StartComposite(
                    bounds,
                    mask,
                    radii,
                    radii_y,
                    opacity,
                    transform,
                ) => {
                    let mut child = Self::materialize(operations);
                    if child.len() > 0 {
                        // Rasterize in a local, pixel-aligned texture space. A
                        // scale can bring negative or off-window source pixels
                        // into view, so the window is not the source extent.
                        let source_origin = bounds.origin.map(|v| ScaledPixels(v.0.floor()));
                        child.translate_rendered(source_origin.map(|v| ScaledPixels(-v.0)));
                        let painted_bounds = transform
                            .apply_bounds(bounds.map(|p| Pixels(p.0)))
                            .map(|p| ScaledPixels(p.0));
                        // Filled rrect meshes have a half-device-pixel AA
                        // fringe. Include it in overlap ordering and culling.
                        let painted_bounds =
                            if *radii != Corners::default() && *radii_y != Corners::default() {
                                painted_bounds.dilate(ScaledPixels(0.5))
                            } else {
                                painted_bounds
                            };
                        scene.insert_primitive(Composite {
                            order: 0,
                            bounds: *bounds,
                            painted_bounds,
                            transformation: *transform,
                            source_origin,
                            content_mask: mask.clone(),
                            corner_radii: *radii,
                            corner_radii_y: *radii_y,
                            opacity: *opacity,
                            scene: std::rc::Rc::new(child),
                        });
                    }
                }
                PaintOperation::EndComposite => break,
            }
        }
        scene.finish();
        scene
    }

    fn translate_rendered(&mut self, offset: Point<ScaledPixels>) {
        // Only the materialized renderer tree moves. The original operation
        // log stays in window coordinates for cached paint ranges.
        macro_rules! translate {
            ($items:expr) => {
                for item in $items {
                    item.bounds.origin += offset;
                    item.content_mask.bounds.origin += offset;
                }
            };
        }
        translate!(&mut self.quads);
        translate!(&mut self.shadows);
        translate!(&mut self.underlines);
        translate!(&mut self.polychrome_sprites);
        translate!(&mut self.surfaces);
        for path in &mut self.paths {
            path.bounds.origin += offset;
            path.content_mask.bounds.origin += offset;
            for vertex in &mut path.vertices {
                vertex.xy_position += offset;
                vertex.content_mask.bounds.origin += offset;
            }
        }
        for sprite in &mut self.monochrome_sprites {
            sprite.transformation.translation[0] += offset.x.0;
            sprite.transformation.translation[1] += offset.y.0;
            sprite.content_mask.bounds.origin += offset;
        }
        for composite in &mut self.composites {
            // Its child already has its own texture coordinates. Move only
            // this layer's destination and parent clip.
            composite.transformation.translation[0] += offset.x.0;
            composite.transformation.translation[1] += offset.y.0;
            composite.painted_bounds.origin += offset;
            composite.content_mask.bounds.origin += offset;
        }
    }

    /// Shared intermediate extent for this frame, including every source layer.
    pub(crate) fn intermediate_size(
        &self,
        mut minimum: Size<crate::DevicePixels>,
    ) -> Size<crate::DevicePixels> {
        for composite in &self.composites {
            minimum.width.0 = minimum
                .width
                .0
                .max((composite.bounds.right().0.ceil() - composite.source_origin.x.0) as i32);
            minimum.height.0 = minimum
                .height
                .0
                .max((composite.bounds.bottom().0.ceil() - composite.source_origin.y.0) as i32);
            minimum = composite.scene.intermediate_size(minimum);
        }
        minimum
    }

    #[cfg_attr(
        all(
            any(target_os = "linux", target_os = "freebsd"),
            not(any(feature = "x11", feature = "wayland"))
        ),
        allow(dead_code)
    )]
    pub(crate) fn batches(&self) -> impl Iterator<Item = PrimitiveBatch<'_>> {
        BatchIterator {
            shadows: &self.shadows,
            shadows_start: 0,
            shadows_iter: self.shadows.iter().peekable(),
            quads: &self.quads,
            quads_start: 0,
            quads_iter: self.quads.iter().peekable(),
            paths: &self.paths,
            paths_start: 0,
            paths_iter: self.paths.iter().peekable(),
            underlines: &self.underlines,
            underlines_start: 0,
            underlines_iter: self.underlines.iter().peekable(),
            monochrome_sprites: &self.monochrome_sprites,
            monochrome_sprites_start: 0,
            monochrome_sprites_iter: self.monochrome_sprites.iter().peekable(),
            polychrome_sprites: &self.polychrome_sprites,
            polychrome_sprites_start: 0,
            polychrome_sprites_iter: self.polychrome_sprites.iter().peekable(),
            surfaces: &self.surfaces,
            surfaces_start: 0,
            surfaces_iter: self.surfaces.iter().peekable(),
            composites_iter: self.composites.iter().peekable(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
pub(crate) enum PrimitiveKind {
    Shadow,
    #[default]
    Quad,
    Path,
    Underline,
    MonochromeSprite,
    PolychromeSprite,
    Surface,
    Composite,
}

pub(crate) enum PaintOperation {
    Primitive(Primitive),
    StartLayer(Bounds<ScaledPixels>),
    EndLayer,
    StartComposite(
        Bounds<ScaledPixels>,
        ContentMask<ScaledPixels>,
        Corners<ScaledPixels>,
        Corners<ScaledPixels>,
        f32,
        TransformationMatrix,
    ),
    EndComposite,
}

#[derive(Clone)]
pub(crate) enum Primitive {
    Shadow(Shadow),
    Quad(Quad),
    Path(Path<ScaledPixels>),
    Underline(Underline),
    MonochromeSprite(MonochromeSprite),
    PolychromeSprite(PolychromeSprite),
    Surface(PaintSurface),
    Composite(Composite),
}

impl Primitive {
    pub fn bounds(&self) -> &Bounds<ScaledPixels> {
        match self {
            Primitive::Shadow(shadow) => &shadow.bounds,
            Primitive::Quad(quad) => &quad.bounds,
            Primitive::Path(path) => &path.bounds,
            Primitive::Underline(underline) => &underline.bounds,
            Primitive::MonochromeSprite(sprite) => &sprite.bounds,
            Primitive::PolychromeSprite(sprite) => &sprite.bounds,
            Primitive::Surface(surface) => &surface.bounds,
            Primitive::Composite(composite) => &composite.painted_bounds,
        }
    }

    pub fn content_mask(&self) -> &ContentMask<ScaledPixels> {
        match self {
            Primitive::Shadow(shadow) => &shadow.content_mask,
            Primitive::Quad(quad) => &quad.content_mask,
            Primitive::Path(path) => &path.content_mask,
            Primitive::Underline(underline) => &underline.content_mask,
            Primitive::MonochromeSprite(sprite) => &sprite.content_mask,
            Primitive::PolychromeSprite(sprite) => &sprite.content_mask,
            Primitive::Surface(surface) => &surface.content_mask,
            Primitive::Composite(composite) => &composite.content_mask,
        }
    }
}

#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
struct BatchIterator<'a> {
    shadows: &'a [Shadow],
    shadows_start: usize,
    shadows_iter: Peekable<slice::Iter<'a, Shadow>>,
    quads: &'a [Quad],
    quads_start: usize,
    quads_iter: Peekable<slice::Iter<'a, Quad>>,
    paths: &'a [Path<ScaledPixels>],
    paths_start: usize,
    paths_iter: Peekable<slice::Iter<'a, Path<ScaledPixels>>>,
    underlines: &'a [Underline],
    underlines_start: usize,
    underlines_iter: Peekable<slice::Iter<'a, Underline>>,
    monochrome_sprites: &'a [MonochromeSprite],
    monochrome_sprites_start: usize,
    monochrome_sprites_iter: Peekable<slice::Iter<'a, MonochromeSprite>>,
    polychrome_sprites: &'a [PolychromeSprite],
    polychrome_sprites_start: usize,
    polychrome_sprites_iter: Peekable<slice::Iter<'a, PolychromeSprite>>,
    surfaces: &'a [PaintSurface],
    surfaces_start: usize,
    surfaces_iter: Peekable<slice::Iter<'a, PaintSurface>>,
    composites_iter: Peekable<slice::Iter<'a, Composite>>,
}

impl<'a> Iterator for BatchIterator<'a> {
    type Item = PrimitiveBatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut orders_and_kinds = [
            (
                self.shadows_iter.peek().map(|s| s.order),
                PrimitiveKind::Shadow,
            ),
            (self.quads_iter.peek().map(|q| q.order), PrimitiveKind::Quad),
            (self.paths_iter.peek().map(|q| q.order), PrimitiveKind::Path),
            (
                self.underlines_iter.peek().map(|u| u.order),
                PrimitiveKind::Underline,
            ),
            (
                self.monochrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::MonochromeSprite,
            ),
            (
                self.polychrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::PolychromeSprite,
            ),
            (
                self.surfaces_iter.peek().map(|s| s.order),
                PrimitiveKind::Surface,
            ),
        ];
        let mut orders_and_kinds = {
            let mut orders = orders_and_kinds.to_vec();
            orders.push((
                self.composites_iter.peek().map(|c| c.order),
                PrimitiveKind::Composite,
            ));
            orders
        };
        orders_and_kinds.sort_by_key(|(order, kind)| (order.unwrap_or(u32::MAX), *kind));

        let first = orders_and_kinds[0];
        let second = orders_and_kinds[1];
        let (batch_kind, max_order_and_kind) = if first.0.is_some() {
            (first.1, (second.0.unwrap_or(u32::MAX), second.1))
        } else {
            return None;
        };

        match batch_kind {
            PrimitiveKind::Shadow => {
                let shadows_start = self.shadows_start;
                let mut shadows_end = shadows_start + 1;
                self.shadows_iter.next();
                while self
                    .shadows_iter
                    .next_if(|shadow| (shadow.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    shadows_end += 1;
                }
                self.shadows_start = shadows_end;
                Some(PrimitiveBatch::Shadows(
                    &self.shadows[shadows_start..shadows_end],
                ))
            }
            PrimitiveKind::Quad => {
                let quads_start = self.quads_start;
                let mut quads_end = quads_start + 1;
                self.quads_iter.next();
                while self
                    .quads_iter
                    .next_if(|quad| (quad.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    quads_end += 1;
                }
                self.quads_start = quads_end;
                Some(PrimitiveBatch::Quads(&self.quads[quads_start..quads_end]))
            }
            PrimitiveKind::Path => {
                let paths_start = self.paths_start;
                let mut paths_end = paths_start + 1;
                self.paths_iter.next();
                while self
                    .paths_iter
                    .next_if(|path| (path.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    paths_end += 1;
                }
                self.paths_start = paths_end;
                Some(PrimitiveBatch::Paths(&self.paths[paths_start..paths_end]))
            }
            PrimitiveKind::Underline => {
                let underlines_start = self.underlines_start;
                let mut underlines_end = underlines_start + 1;
                self.underlines_iter.next();
                while self
                    .underlines_iter
                    .next_if(|underline| (underline.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    underlines_end += 1;
                }
                self.underlines_start = underlines_end;
                Some(PrimitiveBatch::Underlines(
                    &self.underlines[underlines_start..underlines_end],
                ))
            }
            PrimitiveKind::MonochromeSprite => {
                let texture_id = self.monochrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.monochrome_sprites_start;
                let mut sprites_end = sprites_start + 1;
                self.monochrome_sprites_iter.next();
                while self
                    .monochrome_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.monochrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    sprites: &self.monochrome_sprites[sprites_start..sprites_end],
                })
            }
            PrimitiveKind::PolychromeSprite => {
                let texture_id = self.polychrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.polychrome_sprites_start;
                let mut sprites_end = self.polychrome_sprites_start + 1;
                self.polychrome_sprites_iter.next();
                while self
                    .polychrome_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.polychrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites: &self.polychrome_sprites[sprites_start..sprites_end],
                })
            }
            PrimitiveKind::Composite => Some(PrimitiveBatch::Composite(
                self.composites_iter.next().unwrap(),
            )),
            PrimitiveKind::Surface => {
                let surfaces_start = self.surfaces_start;
                let mut surfaces_end = surfaces_start + 1;
                self.surfaces_iter.next();
                while self
                    .surfaces_iter
                    .next_if(|surface| (surface.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    surfaces_end += 1;
                }
                self.surfaces_start = surfaces_end;
                Some(PrimitiveBatch::Surfaces(
                    &self.surfaces[surfaces_start..surfaces_end],
                ))
            }
        }
    }
}

#[derive(Debug)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
pub(crate) enum PrimitiveBatch<'a> {
    Shadows(&'a [Shadow]),
    Quads(&'a [Quad]),
    Paths(&'a [Path<ScaledPixels>]),
    Underlines(&'a [Underline]),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [MonochromeSprite],
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [PolychromeSprite],
    },
    Surfaces(&'a [PaintSurface]),
    Composite(&'a Composite),
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub(crate) struct Quad {
    pub order: DrawOrder,
    pub border_style: BorderStyle,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub background: Background,
    pub border_color: Hsla,
    pub corner_radii: Corners<ScaledPixels>,
    pub border_widths: Edges<ScaledPixels>,
}

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Primitive::Quad(quad)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct Underline {
    pub order: DrawOrder,
    pub pad: u32, // align to 8 bytes
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub thickness: ScaledPixels,
    pub wavy: u32,
}

impl From<Underline> for Primitive {
    fn from(underline: Underline) -> Self {
        Primitive::Underline(underline)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct Shadow {
    pub order: DrawOrder,
    pub blur_radius: ScaledPixels,
    pub bounds: Bounds<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
}

impl From<Shadow> for Primitive {
    fn from(shadow: Shadow) -> Self {
        Primitive::Shadow(shadow)
    }
}

/// The style of a border.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[repr(C)]
pub enum BorderStyle {
    /// A solid border.
    #[default]
    Solid = 0,
    /// A dashed border.
    Dashed = 1,
}

/// A data type representing a 2 dimensional transformation that can be applied to an element.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TransformationMatrix {
    /// 2x2 matrix containing rotation and scale,
    /// stored row-major
    pub rotation_scale: [[f32; 2]; 2],
    /// translation vector
    pub translation: [f32; 2],
}

impl Eq for TransformationMatrix {}

impl TransformationMatrix {
    /// The unit matrix, has no effect.
    pub fn unit() -> Self {
        Self {
            rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
            translation: [0.0, 0.0],
        }
    }

    /// Move the origin by a given point
    pub fn translate(mut self, point: Point<ScaledPixels>) -> Self {
        self.compose(Self {
            rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
            translation: [point.x.0, point.y.0],
        })
    }

    /// Clockwise rotation in radians around the origin
    pub fn rotate(self, angle: Radians) -> Self {
        self.compose(Self {
            rotation_scale: [
                [angle.0.cos(), -angle.0.sin()],
                [angle.0.sin(), angle.0.cos()],
            ],
            translation: [0.0, 0.0],
        })
    }

    /// Scale around the origin
    pub fn scale(self, size: Size<f32>) -> Self {
        self.compose(Self {
            rotation_scale: [[size.width, 0.0], [0.0, size.height]],
            translation: [0.0, 0.0],
        })
    }

    /// Perform matrix multiplication with another transformation
    /// to produce a new transformation that is the result of
    /// applying both transformations: first, `other`, then `self`.
    #[inline]
    pub fn compose(self, other: TransformationMatrix) -> TransformationMatrix {
        if other == Self::unit() {
            return self;
        }
        // Perform matrix multiplication
        TransformationMatrix {
            rotation_scale: [
                [
                    self.rotation_scale[0][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][0],
                    self.rotation_scale[0][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][1],
                ],
                [
                    self.rotation_scale[1][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][0],
                    self.rotation_scale[1][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][1],
                ],
            ],
            translation: [
                self.translation[0]
                    + self.rotation_scale[0][0] * other.translation[0]
                    + self.rotation_scale[0][1] * other.translation[1],
                self.translation[1]
                    + self.rotation_scale[1][0] * other.translation[0]
                    + self.rotation_scale[1][1] * other.translation[1],
            ],
        }
    }

    /// Invert a nonsingular affine transformation.
    pub fn inverse(self) -> Option<Self> {
        let [[a, b], [c, d]] = self.rotation_scale;
        let determinant = a * d - b * c;
        if !determinant.is_finite() || determinant.abs() <= f32::EPSILON {
            return None;
        }
        let inverse = Self {
            rotation_scale: [
                [d / determinant, -b / determinant],
                [-c / determinant, a / determinant],
            ],
            translation: [0., 0.],
        };
        let translated = inverse.apply(point(
            Pixels(-self.translation[0]),
            Pixels(-self.translation[1]),
        ));
        Some(Self {
            translation: [translated.x.0, translated.y.0],
            ..inverse
        })
    }

    /// The axis-aligned bounds containing all four transformed corners.
    pub fn apply_bounds(self, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        let points = [
            bounds.origin,
            bounds.top_right(),
            bounds.bottom_left(),
            bounds.bottom_right(),
        ]
        .map(|p| self.apply(p));
        let mut min = points[0];
        let mut max = points[0];
        for p in points.into_iter().skip(1) {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        Bounds::from_corners(min, max)
    }

    /// Apply transformation to a point, mainly useful for debugging
    pub fn apply(&self, point: Point<Pixels>) -> Point<Pixels> {
        let input = [point.x.0, point.y.0];
        let mut output = self.translation;
        for (i, output_cell) in output.iter_mut().enumerate() {
            for (k, input_cell) in input.iter().enumerate() {
                *output_cell += self.rotation_scale[i][k] * *input_cell;
            }
        }
        Point::new(output[0].into(), output[1].into())
    }
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        Self::unit()
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct MonochromeSprite {
    pub order: DrawOrder,
    pub pad: u32, // align to 8 bytes
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub tile: AtlasTile,
    pub transformation: TransformationMatrix,
}

impl From<MonochromeSprite> for Primitive {
    fn from(sprite: MonochromeSprite) -> Self {
        Primitive::MonochromeSprite(sprite)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct PolychromeSprite {
    pub order: DrawOrder,
    pub pad: u32, // align to 8 bytes
    pub grayscale: bool,
    pub opacity: f32,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub tile: AtlasTile,
}

impl From<PolychromeSprite> for Primitive {
    fn from(sprite: PolychromeSprite) -> Self {
        Primitive::PolychromeSprite(sprite)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PaintSurface {
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    #[cfg(target_os = "macos")]
    pub image_buffer: core_video::pixel_buffer::CVPixelBuffer,
}

impl From<PaintSurface> for Primitive {
    fn from(surface: PaintSurface) -> Self {
        Primitive::Surface(surface)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathId(pub(crate) usize);

/// A line made up of a series of vertices and control points.
#[derive(Clone, Debug)]
pub struct Path<P: Clone + Debug + Default + PartialEq> {
    pub(crate) id: PathId,
    pub(crate) order: DrawOrder,
    pub(crate) bounds: Bounds<P>,
    pub(crate) content_mask: ContentMask<P>,
    pub(crate) vertices: Vec<PathVertex<P>>,
    pub(crate) color: Background,
    start: Point<P>,
    current: Point<P>,
    contour_count: usize,
}

impl Path<Pixels> {
    /// Create a new path with the given starting point.
    pub fn new(start: Point<Pixels>) -> Self {
        Self {
            id: PathId(0),
            order: DrawOrder::default(),
            vertices: Vec::new(),
            start,
            current: start,
            bounds: Bounds {
                origin: start,
                size: Default::default(),
            },
            content_mask: Default::default(),
            color: Default::default(),
            contour_count: 0,
        }
    }

    /// Scale this path by the given factor.
    pub fn scale(&self, factor: f32) -> Path<ScaledPixels> {
        Path {
            id: self.id,
            order: self.order,
            bounds: self.bounds.scale(factor),
            content_mask: self.content_mask.scale(factor),
            vertices: self
                .vertices
                .iter()
                .map(|vertex| vertex.scale(factor))
                .collect(),
            start: self.start.map(|start| start.scale(factor)),
            current: self.current.scale(factor),
            contour_count: self.contour_count,
            color: self.color,
        }
    }

    /// Move the start, current point to the given point.
    pub fn move_to(&mut self, to: Point<Pixels>) {
        self.contour_count += 1;
        self.start = to;
        self.current = to;
    }

    /// Draw a straight line from the current point to the given point.
    pub fn line_to(&mut self, to: Point<Pixels>) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(
                (self.start, self.current, to),
                (point(0., 1.), point(0., 1.), point(0., 1.)),
            );
        }
        self.current = to;
    }

    /// Draw a curve from the current point to the given point, using the given control point.
    pub fn curve_to(&mut self, to: Point<Pixels>, ctrl: Point<Pixels>) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(
                (self.start, self.current, to),
                (point(0., 1.), point(0., 1.), point(0., 1.)),
            );
        }

        self.push_triangle(
            (self.current, ctrl, to),
            (point(0., 0.), point(0.5, 0.), point(1., 1.)),
        );
        self.current = to;
    }

    /// Push a triangle to the Path.
    pub fn push_triangle(
        &mut self,
        xy: (Point<Pixels>, Point<Pixels>, Point<Pixels>),
        st: (Point<f32>, Point<f32>, Point<f32>),
    ) {
        self.bounds = self
            .bounds
            .union(&Bounds {
                origin: xy.0,
                size: Default::default(),
            })
            .union(&Bounds {
                origin: xy.1,
                size: Default::default(),
            })
            .union(&Bounds {
                origin: xy.2,
                size: Default::default(),
            });

        self.vertices.push(PathVertex {
            xy_position: xy.0,
            st_position: st.0,
            content_mask: Default::default(),
        });
        self.vertices.push(PathVertex {
            xy_position: xy.1,
            st_position: st.1,
            content_mask: Default::default(),
        });
        self.vertices.push(PathVertex {
            xy_position: xy.2,
            st_position: st.2,
            content_mask: Default::default(),
        });
    }
}

impl<T> Path<T>
where
    T: Clone + Debug + Default + PartialEq + PartialOrd + Add<T, Output = T> + Sub<Output = T>,
{
    #[allow(unused)]
    pub(crate) fn clipped_bounds(&self) -> Bounds<T> {
        self.bounds.intersect(&self.content_mask.bounds)
    }
}

impl From<Path<ScaledPixels>> for Primitive {
    fn from(path: Path<ScaledPixels>) -> Self {
        Primitive::Path(path)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct PathVertex<P: Clone + Debug + Default + PartialEq> {
    pub(crate) xy_position: Point<P>,
    pub(crate) st_position: Point<f32>,
    pub(crate) content_mask: ContentMask<P>,
}

impl PathVertex<Pixels> {
    pub fn scale(&self, factor: f32) -> PathVertex<ScaledPixels> {
        PathVertex {
            xy_position: self.xy_position.scale(factor),
            st_position: self.st_position,
            content_mask: self.content_mask.scale(factor),
        }
    }
}

/// An isolated scene, sampled once after all of its primitives are composed.
#[derive(Clone)]
pub(crate) struct Composite {
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub painted_bounds: Bounds<ScaledPixels>,
    pub transformation: TransformationMatrix,
    pub source_origin: Point<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub corner_radii_y: Corners<ScaledPixels>,
    pub opacity: f32,
    pub scene: std::rc::Rc<Scene>,
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Composite")
            .field("bounds", &self.bounds)
            .field("opacity", &self.opacity)
            .finish_non_exhaustive()
    }
}

impl From<Composite> for Primitive {
    fn from(composite: Composite) -> Self {
        Self::Composite(composite)
    }
}

#[cfg(test)]
mod compositor_tests {
    use super::*;

    #[gpui::test]
    fn styled_div_paints_its_inner_ellipses_without_a_visible_border(
        cx: &mut crate::TestAppContext,
    ) {
        use crate::{
            AvailableSpace, IntoElement, ParentElement, Styled, canvas, div, px, rgb, size,
        };
        let cx = cx.add_empty_window();
        cx.simulate_resize(size(px(64.), px(64.)));
        cx.draw(Point::default(), size(px(64.), px(64.)), |_, _| {
            canvas(
                |_, window, cx| {
                    let mut element = div()
                        .size(px(48.))
                        .overflow_hidden()
                        .rounded(px(16.))
                        .border_t(px(4.))
                        .border_r(px(10.))
                        .border_b(px(8.))
                        .border_l(px(2.))
                        .border_color(Hsla::transparent_black())
                        .child(div().size(px(64.)).flex_shrink_0().bg(rgb(0x00ff00)))
                        .into_any_element();
                    element.layout_as_root(
                        size(
                            AvailableSpace::Definite(px(64.)),
                            AvailableSpace::Definite(px(64.)),
                        ),
                        window,
                        cx,
                    );
                    element.prepaint_at(point(px(8.), px(8.)), window, cx);
                    element
                },
                |_, mut element, window, cx| {
                    element.paint(window, cx);
                    let scale = window.scale_factor();
                    let scene = &mut window.next_frame.scene;
                    scene.finish();
                    let clip = scene.composites.last().expect("rounded overflow layer");
                    assert_eq!(
                        clip.bounds,
                        Bounds::new(point(px(10.), px(12.)), size(px(36.), px(36.))).scale(scale)
                    );
                    assert_eq!(
                        clip.content_mask.bounds,
                        Bounds::new(Point::default(), size(px(64.), px(64.))).scale(scale),
                        "the ancestor clip must not truncate the rrect's AA fringe"
                    );
                    assert_eq!(clip.corner_radii.top_left, ScaledPixels(14. * scale));
                    assert_eq!(clip.corner_radii_y.top_left, ScaledPixels(12. * scale));
                    assert_eq!(clip.corner_radii.bottom_right, ScaledPixels(6. * scale));
                    assert_eq!(clip.corner_radii_y.bottom_right, ScaledPixels(8. * scale));
                },
            )
            .size_full()
        });
    }

    #[gpui::test]
    fn styled_div_snaps_paint_from_unrounded_layout_once(cx: &mut crate::TestAppContext) {
        use crate::{
            AvailableSpace, IntoElement, ParentElement, Styled, canvas, div, px, rgb, size,
        };
        let cx = cx.add_empty_window();
        cx.simulate_resize(size(px(64.), px(64.)));
        for width in [0.75, 0.1] {
            cx.draw(Point::default(), size(px(64.), px(64.)), move |_, _| {
                canvas(
                    move |_, window, cx| {
                        let mut element = div()
                            .size_full()
                            .child(
                                div()
                                    .absolute()
                                    .left(px(8.25))
                                    .top(px(8.5))
                                    .w(px(width))
                                    .h(px(20.))
                                    .bg(rgb(0x00ff00)),
                            )
                            .into_any_element();
                        element.layout_as_root(
                            size(
                                AvailableSpace::Definite(px(64.)),
                                AvailableSpace::Definite(px(64.)),
                            ),
                            window,
                            cx,
                        );
                        element.prepaint_at(Point::default(), window, cx);
                        element
                    },
                    |_, mut element, window, cx| {
                        element.paint(window, cx);
                        let scale = window.scale_factor();
                        let scene = &window.next_frame.scene;
                        let green = scene
                            .quads
                            .iter()
                            .find(|q| q.background.solid_color() == Some(rgb(0x00ff00).into()))
                            .expect("a subpixel CSS box still paints one logical pixel");
                        assert_eq!(
                            green.bounds,
                            Bounds::new(point(px(8.), px(9.)), size(px(1.), px(20.))).scale(scale)
                        );
                    },
                )
                .size_full()
            });
        }
    }

    #[gpui::test]
    fn styled_div_snaps_rounded_overflow_from_unrounded_layout(cx: &mut crate::TestAppContext) {
        use crate::{
            AvailableSpace, IntoElement, ParentElement, Styled, canvas, div, px, rgb, size,
        };
        let cx = cx.add_empty_window();
        cx.simulate_resize(size(px(64.), px(64.)));
        cx.draw(Point::default(), size(px(64.), px(64.)), |_, _| {
            canvas(
                |_, window, cx| {
                    let mut element = div()
                        .size_full()
                        .child(
                            div()
                                .absolute()
                                .left(px(8.25))
                                .top(px(8.5))
                                .size(px(48.))
                                .overflow_hidden()
                                .rounded(px(16.))
                                .border_t(px(4.))
                                .border_x(px(2.))
                                .border_b(px(4.))
                                .child(
                                    div()
                                        .absolute()
                                        .left(px(-2.))
                                        .top(px(-4.))
                                        .size(px(48.))
                                        .bg(rgb(0x00ff00)),
                                ),
                        )
                        .into_any_element();
                    element.layout_as_root(
                        size(
                            AvailableSpace::Definite(px(64.)),
                            AvailableSpace::Definite(px(64.)),
                        ),
                        window,
                        cx,
                    );
                    element.prepaint_at(Point::default(), window, cx);
                    element
                },
                |_, mut element, window, cx| {
                    element.paint(window, cx);
                    let scale = window.scale_factor();
                    let scene = &mut window.next_frame.scene;
                    scene.finish();
                    let clip = scene.composites.last().expect("rounded overflow");
                    assert_eq!(
                        clip.bounds,
                        Bounds::new(point(px(10.), px(13.)), size(px(44.), px(40.))).scale(scale)
                    );
                },
            )
            .size_full()
        });
    }

    #[gpui::test]
    fn styled_div_snaps_cached_views_when_precise_origin_crosses_a_pixel(
        cx: &mut crate::TestAppContext,
    ) {
        use crate::{
            AnyView, AppContext, Context, Entity, IntoElement, ParentElement, Render,
            StyleRefinement, Styled, Window, div, px, rgb, size,
        };
        use std::{cell::Cell, rc::Rc};
        struct Leaf(Rc<Cell<u32>>);
        impl Render for Leaf {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                self.0.set(self.0.get() + 1);
                div().size_full().bg(rgb(0x00ff00))
            }
        }
        struct Parent {
            x: f32,
            leaf: Entity<Leaf>,
        }
        impl Render for Parent {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let mut style = StyleRefinement::default();
                style.size.width = Some(px(0.75).into());
                style.size.height = Some(px(20.).into());
                div().size_full().child(
                    div()
                        .absolute()
                        .left(px(self.x))
                        .top(px(8.5))
                        .w(px(0.75))
                        .h(px(20.))
                        .child(AnyView::from(self.leaf.clone()).cached(style)),
                )
            }
        }
        let renders = Rc::new(Cell::new(0));
        let leaf = cx.new(|_| Leaf(renders.clone()));
        let (view, cx) = cx.add_window_view(|_, _| Parent { x: 8.49, leaf });
        cx.simulate_resize(size(px(64.), px(64.)));
        cx.run_until_parked();
        for (x, expected_x) in [(8.49, 8.), (8.51, 9.)] {
            let before = renders.get();
            view.update(cx, |view, cx| {
                view.x = x;
                cx.notify();
            });
            cx.run_until_parked();
            if x > 8.5 {
                assert!(
                    renders.get() > before,
                    "the same display bounds must not reuse stale CSS paint"
                );
            }
            cx.update(|window, _| {
                let q = window
                    .rendered_frame
                    .scene
                    .quads
                    .iter()
                    .find(|q| q.background.solid_color() == Some(rgb(0x00ff00).into()))
                    .unwrap();
                assert_eq!(
                    q.bounds.origin.x,
                    px(expected_x).scale(window.scale_factor())
                );
                assert_eq!(q.bounds.size.width, px(1.).scale(window.scale_factor()));
            });
            let count = renders.get();
            view.update(cx, |_, cx| cx.notify());
            cx.run_until_parked();
            assert_eq!(
                renders.get(),
                count,
                "identical precise geometry still reuses the cache"
            );
        }
    }

    #[gpui::test]
    fn window_scale_and_nested_opacity_retain_the_inverse_viewport(cx: &mut crate::TestAppContext) {
        use crate::{Styled, canvas, px, rgb, size};
        let cx = cx.add_empty_window();
        cx.simulate_resize(size(px(64.), px(64.)));
        cx.draw(Point::default(), size(px(64.), px(64.)), |_, _| {
            canvas(
                |_, _, _| (),
                |_, _, window, _| {
                    window.with_element_scale(point(px(32.), px(32.)), 0.5, |window| {
                        window.with_element_opacity(Some(0.5), |window| {
                            window.paint_quad(crate::fill(
                                Bounds::new(point(px(-16.), px(24.)), size(px(16.), px(16.))),
                                rgb(0xff0000),
                            ));
                        });
                    });
                    let scale = window.scale_factor();
                    let scene = &mut window.next_frame.scene;
                    scene.finish();
                    let outer = scene.composites.last().unwrap();
                    assert_eq!(outer.bounds.origin.x, ScaledPixels(-32. * scale));
                    assert_eq!(outer.bounds.size.width, ScaledPixels(128. * scale));
                    let opacity = &outer.scene.composites[0];
                    assert_eq!(opacity.bounds, outer.bounds);
                    assert_eq!(opacity.opacity, 0.5);
                    assert_eq!(opacity.scene.quads.len(), 1);
                    assert_eq!(
                        opacity.scene.quads[0].bounds.origin.x,
                        ScaledPixels(16. * scale)
                    );
                },
            )
            .size_full()
        });
    }

    fn bounds() -> Bounds<ScaledPixels> {
        Bounds::new(
            point(ScaledPixels(0.), ScaledPixels(0.)),
            Size::new(ScaledPixels(100.), ScaledPixels(100.)),
        )
    }

    fn quad() -> Quad {
        Quad {
            bounds: bounds(),
            content_mask: ContentMask { bounds: bounds() },
            ..Quad::default()
        }
    }

    fn start(scene: &mut Scene, opacity: f32) {
        scene.push_composite(
            bounds(),
            ContentMask { bounds: bounds() },
            Corners::default(),
            opacity,
            TransformationMatrix::unit(),
        );
    }

    #[test]
    fn nested_groups_isolate_their_primitives_and_keep_global_paint_indices() {
        let mut scene = Scene::default();
        scene.insert_primitive(quad());
        start(&mut scene, 0.5);
        scene.insert_primitive(quad());
        start(&mut scene, 0.25);
        scene.insert_primitive(quad());
        scene.pop_composite();
        scene.pop_composite();
        let count = scene.len();
        scene.finish();
        assert_eq!(scene.len(), count);
        assert_eq!(scene.quads.len(), 1);
        assert_eq!(scene.composites.len(), 1);
        let outer = &scene.composites[0];
        assert_eq!(outer.opacity, 0.5);
        assert_eq!(outer.scene.quads.len(), 1);
        assert_eq!(outer.scene.composites[0].opacity, 0.25);
        assert_eq!(outer.scene.composites[0].scene.quads.len(), 1);
    }

    #[test]
    fn cached_child_ranges_replay_inside_the_current_group() {
        let mut previous = Scene::default();
        start(&mut previous, 0.25);
        let begin = previous.len();
        previous.insert_primitive(quad());
        let end = previous.len();
        previous.pop_composite();
        previous.finish();
        let mut next = Scene::default();
        start(&mut next, 0.75);
        next.replay(begin..end, &previous);
        next.pop_composite();
        next.finish();
        assert!(next.quads.is_empty());
        assert_eq!(next.composites[0].opacity, 0.75);
        assert_eq!(next.composites[0].scene.quads.len(), 1);
    }

    #[test]
    fn cached_parent_ranges_replay_nested_groups_without_flattening_them() {
        let mut previous = Scene::default();
        start(&mut previous, 0.5);
        previous.insert_primitive(quad());
        previous.pop_composite();
        previous.finish();
        let mut next = Scene::default();
        next.replay(0..previous.len(), &previous);
        next.finish();
        assert!(next.quads.is_empty());
        assert_eq!(next.composites.len(), 1);
        assert_eq!(next.composites[0].scene.quads.len(), 1);
    }

    #[test]
    fn compositing_does_not_move_a_rounded_edge_when_the_viewport_clips_it() {
        let mut scene = Scene::default();
        let clip = Bounds::new(
            point(ScaledPixels(20.), ScaledPixels(0.)),
            Size::new(ScaledPixels(80.), ScaledPixels(100.)),
        );
        scene.push_composite(
            bounds(),
            ContentMask { bounds: clip },
            Corners::all(ScaledPixels(12.)),
            1.,
            TransformationMatrix::unit(),
        );
        scene.insert_primitive(quad());
        scene.pop_composite();
        scene.finish();
        assert_eq!(scene.composites[0].bounds, bounds());
        assert_eq!(scene.composites[0].content_mask.bounds, clip);
    }

    #[test]
    fn cached_scale_groups_retain_their_source_and_painted_bounds() {
        let transform = TransformationMatrix {
            rotation_scale: [[0.5, 0.], [0., 0.5]],
            translation: [10., 20.],
        };
        let mut previous = Scene::default();
        previous.push_composite(
            bounds(),
            ContentMask { bounds: bounds() },
            Corners::default(),
            1.,
            transform,
        );
        previous.insert_primitive(quad());
        previous.pop_composite();
        previous.finish();
        let mut next = Scene::default();
        next.replay(0..previous.len(), &previous);
        next.finish();
        let group = &next.composites[0];
        assert_eq!(group.bounds, bounds());
        assert_eq!(
            group.painted_bounds.origin,
            point(ScaledPixels(10.), ScaledPixels(20.))
        );
        assert_eq!(group.painted_bounds.size.width, ScaledPixels(50.));
        assert_eq!(group.transformation, transform);
        let local = point(Pixels(30.), Pixels(40.));
        assert_eq!(
            transform.inverse().unwrap().apply(transform.apply(local)),
            local
        );
    }
}
