use crate::{ControlSize, ThemeVariables, Tint, TypeStyle};
use inset_painting::{Border, BorderRadiusGeometry, BorderStyle, BoxDecoration};
#[derive(Debug, Clone, Copy)]
pub struct FieldMetrics {
    pub height: f64,
    pub padding_block: f64,
    pub face: TypeStyle,
}
impl FieldMetrics {
    pub fn of(vars: &ThemeVariables, size: ControlSize) -> Self {
        let m = vars.metrics(size);
        Self {
            height: m.size,
            padding_block: m.padding_block,
            face: if size == ControlSize::Tiny {
                vars.label_small
            } else {
                vars.label_medium
            },
        }
    }
}
/// The common field surface, used by text entry and compound input controls.
pub fn field_box_decoration(
    vars: &ThemeVariables,
    focused: bool,
    invalid: bool,
    enabled: bool,
    grows: bool,
) -> BoxDecoration {
    let accent = vars
        .ramp(if invalid { Tint::Danger } else { Tint::Primary })
        .shade(vars.focus_ring_shade);
    BoxDecoration::new()
        .color(if !enabled {
            vars.color_surface_sunken
        } else if focused {
            vars.color_surface
        } else {
            vars.color_surface_muted
        })
        .border(Border::all(
            if enabled && (focused || invalid) {
                accent
            } else {
                vars.color_border_strong
            },
            vars.stroke_hairline,
            BorderStyle::Solid,
            -1.0,
        ))
        .border_radius(BorderRadiusGeometry::circular(if grows {
            vars.radius_large
        } else {
            vars.radius_medium
        }))
}
