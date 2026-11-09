// Copyright 2018 Google LLC
// Adapted from Skia's ganesh/ops/FillRRectOp.cpp; see ../../LICENSE-SKIA.
// The raster mesh is deliberately separate from the logical clip and hitbox.

use super::Composite;
use crate::{Pixels, point};

/// A triangle-list vertex shared by the DirectX and Blade solid clip pipelines.
#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct SolidRRectVertex {
    pub position: [f32; 2],
    pub arc: [f32; 2],
    pub gradient: [f32; 2],
    pub color: [f32; 4],
    pub content_mask: [f32; 4],
    pub hardware_derivatives: u32,
    pub pad: u32,
}

impl Composite {
    /// Ganesh folds a uniform rectangle covering a rounded clip into a filled
    /// rrect. Its complex/elliptical rrect mesh uses L1 coverage and constrained
    /// raster radii. Gradients, images and multiple draws retain the clip path;
    /// macOS Chromium's Graphite renderer retains analytic ellipse coverage.
    pub(crate) fn solid_rrect_mesh(&self) -> Option<Vec<SolidRRectVertex>> {
        if cfg!(target_os = "macos") {
            return None;
        }
        let scene = &self.scene;
        if scene.quads.len() != 1
            || !scene.shadows.is_empty()
            || !scene.paths.is_empty()
            || !scene.underlines.is_empty()
            || !scene.monochrome_sprites.is_empty()
            || !scene.polychrome_sprites.is_empty()
            || !scene.surfaces.is_empty()
            || !scene.composites.is_empty()
        {
            return None;
        }
        let quad = &scene.quads[0];
        let mut color = quad.background.solid_color()?;
        if quad.corner_radii != Default::default() || quad.border_widths != Default::default() {
            return None;
        }
        let source_bounds = self.bounds - self.source_origin;
        if !source_bounds.is_contained_within(&quad.bounds.intersect(&quad.content_mask.bounds)) {
            return None;
        }
        let matrix = self.transformation.rotation_scale;
        if matrix[0][1] != 0. || matrix[1][0] != 0. || matrix[0][0] <= 0. || matrix[1][1] <= 0. {
            return None;
        }
        let width = self.bounds.size.width.0;
        let height = self.bounds.size.height.0;
        let rx = self.corner_radii;
        let ry = self.corner_radii_y;
        let radii = [
            [rx.top_left.0, ry.top_left.0],
            [rx.top_right.0, ry.top_right.0],
            [rx.bottom_right.0, ry.bottom_right.0],
            [rx.bottom_left.0, ry.bottom_left.0],
        ];
        let first = radii[0][0];
        let simple_circle = radii.iter().all(|r| *r == [first; 2]);
        // Ganesh's simple circular rrect fast path uses analytic L2 coverage.
        // An actual oval goes through FillRRectOp even when it is circular.
        let oval = radii.iter().all(|r| *r == [width * 0.5, height * 0.5]);
        if width <= 0. || height <= 0. || (simple_circle && !oval) {
            return None;
        }
        color.a *= self.opacity;
        let mask = self.content_mask.bounds;
        let template = SolidRRectVertex {
            position: [0.; 2],
            arc: [0.; 2],
            gradient: [0.; 2],
            color: [color.h, color.s, color.l, color.a],
            content_mask: [
                mask.origin.x.0,
                mask.origin.y.0,
                mask.size.width.0,
                mask.size.height.0,
            ],
            hardware_derivatives: 0,
            pad: 0,
        };
        let pixel_size = [width * matrix[0][0], height * matrix[1][1]];
        let (vertices, indices) = rrect_mesh(radii, [width, height], pixel_size);
        Some(
            indices
                .into_iter()
                .map(|index| {
                    let vertex = vertices[index];
                    let position = self.transformation.apply(point(
                        Pixels(self.bounds.origin.x.0 + (vertex.position[0] + 1.) * width * 0.5),
                        Pixels(self.bounds.origin.y.0 + (vertex.position[1] + 1.) * height * 0.5),
                    ));
                    SolidRRectVertex {
                        position: [position.x.0, position.y.0],
                        arc: vertex.arc,
                        gradient: vertex.gradient,
                        hardware_derivatives: vertex.hardware_derivatives,
                        ..template
                    }
                })
                .collect(),
        )
    }
}

fn rotate(mut v: [f32; 2], count: usize) -> [f32; 2] {
    for _ in 0..count {
        v = [-v[1], v[0]];
    }
    v
}

fn rrect_mesh(
    radii: [[f32; 2]; 4],
    size: [f32; 2],
    device_size: [f32; 2],
) -> (Vec<SolidRRectVertex>, Vec<usize>) {
    let hardware_derivatives = radii.iter().all(|r| {
        let x = r[0] * device_size[0] / size[0];
        let y = r[1] * device_size[1] / size[1];
        x.min(y).max(1.).powi(2) * 5. > x.max(y)
    }) as u32;
    let radii = radii.map(|r| [r[0] * 2. / size[0], r[1] * 2. / size[1]]);
    let pixel = [2. / device_size[0], 2. / device_size[1]];
    let aa = pixel.map(|p| p * 0.5);
    let octagon = 1. / (1. + 0.5_f32.sqrt());
    let definitions = [
        ([0., 1.], [-1., 0.], 0.),
        ([0., 1.], [1., 0.], 1.),
        ([1., 0.], [0., 1.], 1.),
        ([1., 0.], [0., -1.], 0.),
        ([octagon, 0.], [-1., -1.], 0.),
        ([0., octagon], [-1., -1.], 0.),
    ];
    let make_vertex = |corner_index: usize, definition: usize, mut linear: bool| {
        let (outset, direction, mut coverage) = definitions[definition];
        let outset = rotate(outset, corner_index);
        let mut direction = rotate(direction, corner_index);
        let mut corner: [f32; 2] = [[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]][corner_index];
        let mut radius = radii[corner_index];
        let mut multiplier = 1.;
        if aa[0].max(aa[1]) > 1. {
            for axis in 0..2 {
                corner[axis] = corner[axis].signum() * aa[axis].max(1.);
            }
            multiplier = 1. / (aa[0].max(1.) * aa[1].max(1.));
            radius = [0.; 2];
        }
        if radius[0] < aa[0] * 1.5 || radius[1] < aa[1] * 1.5 {
            radius = [0.; 2];
            direction = corner.map(|v| v.signum() * if coverage > 0.5 { -1. } else { 1. });
            linear = true;
        } else {
            let neighbors = [
                radii[[1, 0, 3, 2][corner_index]][0],
                radii[[3, 2, 1, 0][corner_index]][1],
            ];
            for axis in 0..2 {
                let minimum = pixel[axis] * 1.5;
                let maximum = 2. - minimum;
                radius[axis] = radius[axis].max(minimum).min(maximum);
                let neighbor = neighbors[axis].max(minimum).min(maximum);
                let spacing = 2. - radius[axis] - neighbor;
                radius[axis] -= 0.5 * (pixel[axis] * 0.0625 - spacing).max(0.);
            }
        }
        let aa_outset = [direction[0] * aa[0], direction[1] * aa[1]];
        let mut position = [0.; 2];
        for axis in 0..2 {
            position[axis] = corner[axis] + outset[axis] * radius[axis] + aa_outset[axis];
        }
        if coverage > 0.5 {
            for axis in 0..2 {
                if direction[axis] != 0. && position[axis] * corner[axis] < 0. {
                    let backset = position[axis].abs();
                    position[axis] = 0.;
                    let other = 1 - axis;
                    position[other] +=
                        backset * corner[other].signum() * pixel[other] / pixel[axis];
                    coverage = (coverage - 0.5) * corner[axis].abs()
                        / (corner[axis].abs() + backset)
                        + 0.5;
                }
            }
        }
        let mut arc = [0., coverage * multiplier];
        let mut gradient = [0.; 2];
        if !linear {
            for axis in 0..2 {
                let a = 1. - outset[axis].abs() + aa_outset[axis] / radius[axis] * corner[axis];
                arc[axis] = a + if axis == 0 { 1. } else { 0. };
                gradient[axis] = a / radius[axis] * 4. / device_size[axis];
            }
        }
        SolidRRectVertex {
            position,
            arc,
            gradient,
            color: [0.; 4],
            content_mask: [0.; 4],
            hardware_derivatives,
            pad: 0,
        }
    };
    let mut vertices = Vec::with_capacity(40);
    for (corner, definition) in [
        (3, 2),
        (0, 1),
        (0, 2),
        (1, 1),
        (1, 2),
        (2, 1),
        (2, 2),
        (3, 1),
        (3, 3),
        (0, 0),
        (0, 3),
        (1, 0),
        (1, 3),
        (2, 0),
        (2, 3),
        (3, 0),
    ] {
        vertices.push(make_vertex(corner, definition, true));
    }
    for corner in 0..4 {
        for definition in 0..6 {
            vertices.push(make_vertex(corner, definition, false));
        }
    }
    let mut indices = Vec::with_capacity(90);
    for n in 1..7 {
        indices.extend([0, n, n + 1]);
    }
    for n in 0..4 {
        let a = n * 2;
        indices.extend([a, a + 1, 8 + a, a + 1, 9 + a, 8 + a]);
        indices.extend([0, 1, 5, 1, 5, 2, 5, 2, 4, 2, 4, 3].map(|i| 16 + n * 6 + i));
    }
    (vertices, indices)
}
