// Copyright 2022 Google LLC
// Adapted from Skia's AnalyticRRectRenderStep and sksl_graphite_vert.sksl.
// See ../../LICENSE-SKIA. Only filled, axis-aligned rrects are needed here.

use super::Composite;
use crate::{Pixels, point};

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct GraphiteRRectVertex {
    pub position: [f32; 2],
    pub local_position: [f32; 2],
    /// Solid-interior flag and device-space AA outset correction.
    pub control: [f32; 2],
}

impl Composite {
    /// Corner ellipses that cannot share an analytic clip transform need
    /// per-draw processing: geometric intersections or sample coverage.
    #[cfg(target_os = "macos")]
    pub(crate) fn requires_graphite_clip_pass(&self) -> bool {
        if self.graphite_rrect_mesh().is_some()
            || self.transformation.rotation_scale != [[1., 0.], [0., 1.]]
            || self
                .transformation
                .translation
                .iter()
                .any(|v| v.fract() != 0.)
        {
            return false;
        }
        let x = self.corner_radii;
        let y = self.corner_radii_y;
        let mut ratio: Option<f32> = None;
        for (rx, ry) in [
            (x.top_left, y.top_left),
            (x.top_right, y.top_right),
            (x.bottom_right, y.bottom_right),
            (x.bottom_left, y.bottom_left),
        ] {
            if rx.0 < 0.25 || ry.0 < 0.25 {
                continue;
            }
            let value = if (rx.0 - ry.0).abs() <= 0.25 {
                1.
            } else {
                rx.0 / ry.0
            };
            if ratio.is_some_and(|previous| (previous - value).abs() > 0.25) {
                return true;
            }
            ratio = Some(value);
        }
        false
    }

    /// Filled conic wedges use Skia's quarter-pixel tessellation precision.
    /// Preserve those chords rather than testing the exact ellipse at samples.
    #[cfg(target_os = "macos")]
    pub(crate) fn graphite_clip_mesh(&self) -> Vec<crate::Point<crate::ScaledPixels>> {
        let b = self.bounds - self.source_origin;
        let l = b.left().0;
        let t = b.top().0;
        let r = b.right().0;
        let bottom = b.bottom().0;
        let rx = self.corner_radii;
        let ry = self.corner_radii_y;
        let corners = [
            ([l, t + ry.top_left.0], [l, t], [l + rx.top_left.0, t]),
            ([r - rx.top_right.0, t], [r, t], [r, t + ry.top_right.0]),
            (
                [r, bottom - ry.bottom_right.0],
                [r, bottom],
                [r - rx.bottom_right.0, bottom],
            ),
            (
                [l + rx.bottom_left.0, bottom],
                [l, bottom],
                [l, bottom - ry.bottom_left.0],
            ),
        ];
        let mut boundary = Vec::new();
        for (p0, p1, p2) in corners {
            if p0 == p1 || p1 == p2 {
                boundary.push(point(
                    crate::ScaledPixels(p1[0]),
                    crate::ScaledPixels(p1[1]),
                ));
                continue;
            }
            append_conic(&mut boundary, p0, p1, p2, std::f32::consts::FRAC_1_SQRT_2);
        }
        let mut triangles = Vec::with_capacity(boundary.len() * 3);
        for i in 0..boundary.len() {
            triangles.extend([b.center(), boundary[i], boundary[(i + 1) % boundary.len()]]);
        }
        triangles
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn fold_clip_into(
        &self,
        bounds: crate::Bounds<crate::ScaledPixels>,
        primitive: crate::Primitive,
    ) -> Option<Composite> {
        use crate::{ContentMask, Corners, ScaledPixels, Scene, Size, TransformationMatrix};
        let clip = self.bounds - self.source_origin;
        let visible = self
            .transformation
            .inverse()?
            .apply_bounds(self.content_mask.bounds.map(|p| Pixels(p.0)))
            .map(|p| ScaledPixels(p.0))
            - self.source_origin;
        let relevant = clip.intersect(&visible);
        let intersection = clip.intersect(&bounds);
        if intersection.size.width.0 <= 1. || intersection.size.height.0 <= 1. {
            return None;
        }
        let (bounds, x, y) = if relevant.is_contained_within(&bounds) {
            (clip, self.corner_radii, self.corner_radii_y)
        } else {
            let radii = [
                (
                    self.corner_radii.top_left,
                    self.corner_radii_y.top_left,
                    true,
                    true,
                ),
                (
                    self.corner_radii.top_right,
                    self.corner_radii_y.top_right,
                    false,
                    true,
                ),
                (
                    self.corner_radii.bottom_right,
                    self.corner_radii_y.bottom_right,
                    false,
                    false,
                ),
                (
                    self.corner_radii.bottom_left,
                    self.corner_radii_y.bottom_left,
                    true,
                    false,
                ),
            ];
            let mut surviving = [Size::<ScaledPixels>::default(); 4];
            for (i, (rx, ry, left, top)) in radii.into_iter().enumerate() {
                if rx.0 == 0. || ry.0 == 0. {
                    continue;
                }
                let corner = crate::Bounds::new(
                    point(
                        if left { clip.left() } else { clip.right() - rx },
                        if top { clip.top() } else { clip.bottom() - ry },
                    ),
                    crate::size(rx, ry),
                );
                if corner.intersect(&intersection).is_empty() {
                    continue;
                }
                if !corner.is_contained_within(&intersection) {
                    return None;
                }
                if (if left {
                    intersection.left() != clip.left()
                } else {
                    intersection.right() != clip.right()
                }) || (if top {
                    intersection.top() != clip.top()
                } else {
                    intersection.bottom() != clip.bottom()
                }) {
                    return None;
                }
                surviving[i] = crate::size(rx, ry);
            }
            let [tl, tr, br, bl] = surviving;
            (
                intersection,
                Corners {
                    top_left: tl.width,
                    top_right: tr.width,
                    bottom_right: br.width,
                    bottom_left: bl.width,
                },
                Corners {
                    top_left: tl.height,
                    top_right: tr.height,
                    bottom_right: br.height,
                    bottom_left: bl.height,
                },
            )
        };
        let mut scene = Scene::default();
        scene.push_elliptical_composite(
            bounds,
            ContentMask { bounds: visible },
            x,
            y,
            1.,
            TransformationMatrix::unit(),
        );
        scene.insert_primitive(primitive);
        scene.pop_composite();
        scene.finish();
        scene.composites.into_iter().next()
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn can_fold_all_clip_draws(&self) -> bool {
        let scene = &self.scene;
        scene.shadows.is_empty()
            && scene.paths.is_empty()
            && scene.underlines.is_empty()
            && scene.monochrome_sprites.is_empty()
            && scene.surfaces.is_empty()
            && scene.quads.iter().all(|q| {
                q.corner_radii == Default::default()
                    && q.border_widths == Default::default()
                    && self
                        .fold_clip_into(
                            q.bounds.intersect(&q.content_mask.bounds),
                            q.clone().into(),
                        )
                        .is_some()
            })
            && scene.polychrome_sprites.iter().all(|s| {
                s.corner_radii == Default::default()
                    && self
                        .fold_clip_into(
                            s.bounds.intersect(&s.content_mask.bounds),
                            s.clone().into(),
                        )
                        .is_some()
            })
            && scene.composites.iter().all(|c| {
                c.corner_radii == Default::default()
                    && c.corner_radii_y == Default::default()
                    && c.transformation.rotation_scale[0][1] == 0.
                    && c.transformation.rotation_scale[1][0] == 0.
                    && self
                        .fold_clip_into(
                            c.painted_bounds.intersect(&c.content_mask.bounds),
                            c.clone().into(),
                        )
                        .is_some()
            })
    }

    /// Graphite can fold a clip into a single rectangle/image that covers it.
    /// Its outer vertices move by one device pixel while local coordinates
    /// stay on the original shape; the fragment shader corrects that outset.
    /// Multiple draws are considered independently by the clip pass.
    pub(crate) fn graphite_rrect_mesh(&self) -> Option<Vec<GraphiteRRectVertex>> {
        if !cfg!(target_os = "macos") {
            return None;
        }
        let scene = &self.scene;
        if !scene.shadows.is_empty()
            || !scene.paths.is_empty()
            || !scene.underlines.is_empty()
            || !scene.monochrome_sprites.is_empty()
            || !scene.surfaces.is_empty()
        {
            return None;
        }
        let (bounds, mask) = match (
            scene.quads.as_slice(),
            scene.polychrome_sprites.as_slice(),
            scene.composites.as_slice(),
        ) {
            ([quad], [], [])
                if quad.corner_radii == Default::default()
                    && quad.border_widths == Default::default() =>
            {
                (quad.bounds, quad.content_mask.bounds)
            }
            ([], [sprite], []) if sprite.corner_radii == Default::default() => {
                (sprite.bounds, sprite.content_mask.bounds)
            }
            ([], [], [child])
                if child.corner_radii == Default::default()
                    && child.corner_radii_y == Default::default()
                    && child.transformation.rotation_scale[0][1] == 0.
                    && child.transformation.rotation_scale[1][0] == 0. =>
            {
                (child.painted_bounds, child.content_mask.bounds)
            }
            _ => return None,
        };
        let visible = self
            .transformation
            .inverse()?
            .apply_bounds(self.content_mask.bounds.map(|p| Pixels(p.0)))
            .map(|p| crate::ScaledPixels(p.0));
        let relevant = self.bounds.intersect(&visible) - self.source_origin;
        if !relevant.is_contained_within(&bounds.intersect(&mask)) {
            return None;
        }
        let matrix = self.transformation.rotation_scale;
        if matrix[0][1] != 0.
            || matrix[1][0] != 0.
            || matrix[0][0] <= 0.
            || matrix[0][0] != matrix[1][1]
        {
            return None;
        }
        let width = self.bounds.size.width.0;
        let height = self.bounds.size.height.0;
        if width <= 0. || height <= 0. {
            return None;
        }
        let x = self.corner_radii;
        let y = self.corner_radii_y;
        let radii = [
            [x.top_left.0, y.top_left.0],
            [x.top_right.0, y.top_right.0],
            [x.bottom_right.0, y.bottom_right.0],
            [x.bottom_left.0, y.bottom_left.0],
        ];
        if radii.iter().all(|r| r[0] == 0. || r[1] == 0.) {
            return None;
        }
        let scale = matrix[0][0];
        let aa_radius = 1. / scale;
        let complex_inset = radii
            .iter()
            .any(|r| 2. * aa_radius >= width - r[0] || 2. * aa_radius >= height - r[1]);
        let center = [self.bounds.center().x.0, self.bounds.center().y.0];
        let l = self.bounds.origin.x.0;
        let t = self.bounds.origin.y.0;
        let corners = [
            [l, t],
            [l + width, t],
            [l + width, t + height],
            [l, t + height],
        ];
        let q = std::f32::consts::FRAC_1_SQRT_2;
        let definitions = [
            ([1., 0.], [1., 0.], 1., false),
            ([1., 0.], [q, q], 1., false),
            ([0., 1.], [q, q], 1., false),
            ([0., 1.], [0., 1.], 1., false),
            ([1., 0.], [q, q], 0., false),
            ([0., 1.], [q, q], 0., false),
            ([1., 0.], [1., 0.], -1., false),
            ([0., 1.], [0., 1.], -1., false),
            ([1., 0.], [1., 0.], -1., true),
        ];
        let mut vertices = Vec::with_capacity(36);
        for corner in 0..4 {
            let radius = if corner % 2 == 0 {
                radii[corner]
            } else {
                [radii[corner][1], radii[corner][0]]
            };
            let rounded = radius[0] > 0. && radius[1] > 0.;
            let join = if rounded {
                std::f32::consts::SQRT_2 - 1.
            } else {
                1.
            };
            for (position, normal, normal_scale, solid) in definitions {
                let local = if normal_scale < 0. && (complex_inset || solid) {
                    center
                } else {
                    let relative = if normal_scale < 0. {
                        if !rounded || radius[0].min(radius[1]) <= aa_radius {
                            radius.map(|r| r - aa_radius)
                        } else {
                            [
                                radius[0] * position[0] - aa_radius * normal[0],
                                radius[1] * position[1] - aa_radius * normal[1],
                            ]
                        }
                    } else {
                        [
                            radius[0] * (position[0] + join * position[1]),
                            radius[1] * (position[1] + join * position[0]),
                        ]
                    };
                    let offset = rotate([radius[0] - relative[0], radius[1] - relative[1]], corner);
                    [
                        corners[corner][0] + offset[0],
                        corners[corner][1] + offset[1],
                    ]
                };
                let transformed = self
                    .transformation
                    .apply(point(Pixels(local[0]), Pixels(local[1])));
                let mut device_position = [transformed.x.0, transformed.y.0];
                if normal_scale > 0. {
                    let aspect = if rounded {
                        [radius[1], radius[0]]
                    } else {
                        [1.; 2]
                    };
                    let normal = [-normal[0] * aspect[0], -normal[1] * aspect[1]];
                    let length = normal[0].hypot(normal[1]);
                    let offset = rotate(normal.map(|v| v / length), corner);
                    device_position[0] += offset[0];
                    device_position[1] += offset[1];
                }
                vertices.push(GraphiteRRectVertex {
                    position: device_position,
                    local_position: local,
                    control: [
                        if solid { 1. } else { 0. },
                        if normal_scale > 0. { -1. } else { 0. },
                    ],
                });
            }
        }
        // One strip with degenerate joins, matching Graphite's three bands.
        let mut strip = Vec::with_capacity(73);
        for c in 0..4 {
            strip.extend([0, 4, 1, 5, 2, 3, 3, 5].map(|i| c * 9 + i));
        }
        strip.extend([0, 4]);
        for c in 0..4 {
            strip.extend([4, 6, 5, 7].map(|i| c * 9 + i));
        }
        strip.extend([4, 6]);
        for c in 0..4 {
            strip.extend([
                c * 9 + 6,
                c * 9 + 8,
                c * 9 + 7,
                c * 9 + 7,
                ((c + 1) % 4) * 9 + 8,
            ]);
        }
        strip.push(6);
        Some(strip.into_iter().map(|i| vertices[i]).collect())
    }
}

fn rotate(mut v: [f32; 2], count: usize) -> [f32; 2] {
    for _ in 0..count {
        v = [-v[1], v[0]];
    }
    v
}

// Copyright 2020 Google LLC. Adapted from Skia's WangsFormula::conic_p2
// and PatchWriter::chopAndWriteConics; see ../../LICENSE-SKIA.
#[cfg(target_os = "macos")]
fn conic_segments_squared(
    mut p0: [f32; 2],
    mut p1: [f32; 2],
    mut p2: [f32; 2],
    weight: f32,
) -> f32 {
    let center = [0, 1].map(|axis| {
        0.5 * (p0[axis].min(p1[axis]).min(p2[axis]) + p0[axis].max(p1[axis]).max(p2[axis]))
    });
    for i in 0..2 {
        p0[i] -= center[i];
        p1[i] -= center[i];
        p2[i] -= center[i];
    }
    let max_length = p0[0]
        .hypot(p0[1])
        .max(p1[0].hypot(p1[1]))
        .max(p2[0].hypot(p2[1]));
    let dp = [0, 1].map(|axis| p0[axis] - 2. * weight * p1[axis] + p2[axis]);
    // Four means at most 1/4 device pixel curve error.
    let numerator =
        dp[0].hypot(dp[1]) * 4. + (max_length * 4. - 1.).max(0.) * (2. - 2. * weight).abs();
    numerator / (4. * weight.min(1.))
}

#[cfg(target_os = "macos")]
fn append_conic(
    boundary: &mut Vec<crate::Point<crate::ScaledPixels>>,
    p0: [f32; 2],
    p1: [f32; 2],
    p2: [f32; 2],
    weight: f32,
) {
    // Fixed-count wedges support at most 32 parametric segments. Larger
    // conics are chopped in homogeneous space and each patch is normalized.
    let n2 = conic_segments_squared(p0, p1, p2, weight);
    let patches = (n2.min(1024. * 1024.).sqrt() / 32.).ceil().max(1.) as usize;
    let project = |p: [f32; 3]| [p[0] / p[2], p[1] / p[2]];
    let mut h0 = [p0[0], p0[1], 1.];
    let mut h1 = [p1[0] * weight, p1[1] * weight, weight];
    let h2 = [p2[0], p2[1], 1.];
    let mix = |a: [f32; 3], b: [f32; 3], t: f32| [0, 1, 2].map(|i| (b[i] - a[i]) * t + a[i]);
    for remaining in (2..=patches).rev() {
        let t = 1. / remaining as f32;
        let ab = mix(h0, h1, t);
        let bc = mix(h1, h2, t);
        let abc = mix(ab, bc, t);
        append_conic_patch(
            boundary,
            project(h0),
            project(ab),
            project(abc),
            ab[2] / (h0[2] * abc[2]).sqrt(),
        );
        h0 = abc;
        h1 = bc;
    }
    append_conic_patch(boundary, project(h0), project(h1), p2, h1[2] / h0[2].sqrt());
}

#[cfg(target_os = "macos")]
fn append_conic_patch(
    boundary: &mut Vec<crate::Point<crate::ScaledPixels>>,
    p0: [f32; 2],
    p1: [f32; 2],
    p2: [f32; 2],
    weight: f32,
) {
    let n2 = conic_segments_squared(p0, p1, p2, weight);
    let segments = (n2.sqrt().ceil() as usize).clamp(1, 32).next_power_of_two();
    for i in 0..=segments {
        // Keep patch endpoints exact, as the fixed-count vertex shader does.
        let p = if i == 0 {
            p0
        } else if i == segments {
            p2
        } else {
            let t = i as f32 / segments as f32;
            let v = 1. - t;
            let weights = [v * v, 2. * weight * t * v, t * t];
            let denominator: f32 = weights.iter().sum();
            [0, 1].map(|axis| {
                (weights[0] * p0[axis] + weights[1] * p1[axis] + weights[2] * p2[axis])
                    / denominator
            })
        };
        boundary.push(point(crate::ScaledPixels(p[0]), crate::ScaledPixels(p[1])));
    }
}
