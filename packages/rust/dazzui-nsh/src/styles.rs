//! The shared profiles and color recipes; no platform palette lives here.
use crate::{
    ThemeTokens,
    axes::{ControlColors, Size, State, Tint, Variant},
    tokens::{TypeStyle, shadows},
};
use nsh_core::{element::StyleRefinement, prelude::*};

pub fn typography(s: StyleRefinement, face: TypeStyle) -> StyleRefinement {
    s.font_family(face.face.family())
        .text_size(px(face.size))
        .line_height(px(face.line_height))
        .font_weight(face.font_weight())
}

pub fn colors(
    s: StyleRefinement,
    t: &ThemeTokens,
    variant: Variant,
    tint: Tint,
    state: State,
) -> StyleRefinement {
    let c = ControlColors::resolve(t, variant, tint, state);
    s.bg(c.surface).text_color(c.content).border_color(c.border)
}

pub fn focus(s: StyleRefinement, t: &ThemeTokens) -> StyleRefinement {
    s.ring(px(t.focus_width))
        .ring_color(t.focus_ring.resolve(&t.color_primary))
        .ring_offset(px(t.focus_offset))
}

pub fn control(
    s: StyleRefinement,
    t: &ThemeTokens,
    size: Size,
    variant: Variant,
    tint: Tint,
    disabled: bool,
) -> StyleRefinement {
    let p = size.profile(t);
    let mut face = p.content;
    if variant == Variant::Filled && !disabled {
        face.weight = t.label_strong.weight;
    }
    let s = typography(s, face)
        .h(px(p.size))
        .gap(px(p.gap))
        .px(px(p.padding_inline))
        .py(px(p.padding_block))
        .rounded(px(p.radius))
        .border(px(t.stroke_hairline));
    let s = colors(
        s,
        t,
        variant,
        tint,
        if disabled {
            State::Disabled
        } else {
            State::Normal
        },
    );
    if variant == Variant::Filled && !disabled {
        s.shadow(shadows(t.shadow_2xs))
    } else {
        s.shadow_none()
    }
}

pub fn popup(s: StyleRefinement, t: &ThemeTokens) -> StyleRefinement {
    typography(s, t.label_medium)
        .min_w(px(t.menu_min_width))
        .p(px(t.spacing_1))
        .rounded(px(t.radius_large))
        .border(px(t.stroke_hairline))
        .border_color(t.color_border_strong.hsla())
        .bg(t.color_surface_overlay.hsla())
        .text_color(t.color_content.hsla())
        .shadow(shadows(t.shadow_lg))
}

/// The shared TextField / Select box, distinct from a recessed button.
pub fn field(
    s: StyleRefinement,
    t: &ThemeTokens,
    size: Size,
    tint: Tint,
    disabled: bool,
    error: bool,
) -> StyleRefinement {
    let mut face = size.profile(t).content;
    face.weight = t.body_small.weight;
    let ramp = tint.ramp(t);
    let s = typography(s, face)
        .h(px(size.profile(t).size))
        .w_full()
        .px(px(t.spacing_3))
        .py(px(size.profile(t).padding_block))
        .rounded(px(t.control_field_radius))
        .border(px(t.stroke_hairline))
        .border_color(if error {
            ramp.shade(600).hsla()
        } else {
            t.color_border_strong.hsla()
        })
        .bg(if error {
            ramp.shade(50).hsla()
        } else {
            t.color_surface_muted.hsla()
        })
        .text_color(if error {
            ramp.shade(800).hsla()
        } else {
            t.color_content.hsla()
        });
    if disabled {
        let c = ControlColors::resolve(t, Variant::Normal, tint, State::Disabled);
        s.bg(c.surface).text_color(c.content)
    } else {
        s
    }
}

pub fn field_focus(
    s: StyleRefinement,
    t: &ThemeTokens,
    tint: Tint,
    error: bool,
) -> StyleRefinement {
    let s = s
        .border_color(tint.ramp(t).shade(600).hsla())
        .ring(px(t.focus_width))
        .ring_color(t.focus_glow.resolve(tint.ramp(t)));
    if error {
        s
    } else {
        s.bg(t.color_surface.hsla())
    }
}
pub fn popup_item(s: StyleRefinement, t: &ThemeTokens) -> StyleRefinement {
    typography(s, t.label_quiet)
        .flex_row()
        .items_center()
        .gap(px(t.spacing_2))
        .py(px(t.menu_item_padding))
        .px(px(t.spacing_2))
        .rounded(px(t.radius_large - t.spacing_1))
        .text_color(t.color_content.hsla())
}

/// Intrinsic advance for a single-line control label. The painted text retains
/// the engine's pixel-ceiled box; layout uses the actual platform font advance
/// so a row of labels does not accumulate one rounding error per child.
pub fn label(
    content: impl Fn() -> SharedString + 'static,
    face: impl Fn() -> TypeStyle + 'static,
) -> nsh_core::element::View {
    let content = computed(content);
    let face = computed(face);
    let width = computed(move || {
        let face = face.get();
        measure_text(
            &format!("{} {}px {}", face.weight, face.size, face.face.family()),
            &content.get(),
        )
    });
    view()
        .w(move || width.get())
        .h(move || px(face.get().line_height))
        .flex_shrink_0()
        .style_with(move |s| typography(s, face.get()))
        .child(
            text(content)
                .absolute()
                .left(px(0.0))
                .top(px(0.0))
                .w(move || {
                    let scale = nsh_core::element::pixel_ratio();
                    px((f32::from(width.get()) * scale).ceil() / scale)
                }),
        )
}

/// A wrapping paragraph with the design's line boxes. CoreText rounds the
/// suggested frame's height up, even with fractional leading; keep that paint
/// box but give flex layout the actual number of lines times the requested leading.
pub fn paragraph(
    content: impl Fn() -> SharedString + 'static,
    face: impl Fn() -> TypeStyle + 'static,
) -> nsh_core::element::View {
    let content = computed(content);
    let face = computed(face);
    let lines = signal(1.0_f32);
    view()
        .w_full()
        .min_w(px(0.0))
        .h(move || px(lines.get() * face.get().line_height))
        .child(
            text(content)
                .absolute()
                .left(px(0.0))
                .top(px(0.0))
                .w_full()
                .style_with(move |s| typography(s, face.get()))
                .on_layout(move |bounds: &nsh_core::Bounds<nsh_core::Points>| {
                    lines.set(
                        (bounds.size.height.0 / face.get_untracked().line_height)
                            .round()
                            .max(1.0),
                    );
                }),
        )
}

/// CSS's baseline from the top of a line box. Browsers round each font's ascent
/// and descent before distributing the leading, which matters when a row mixes
/// UI and monospace faces. Taffy's synthesized bottom baseline cannot do that.
pub fn baseline(face: TypeStyle) -> f32 {
    let metrics = nsh_core::element::text_metrics(
        &format!("{} {}px {}", face.weight, face.size, face.face.family()),
        "x",
    );
    let ascent = metrics.ascent.0.round();
    let descent = metrics.descent.0.round();
    (face.line_height - ascent - descent) / 2.0 + ascent
}
