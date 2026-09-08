#![cfg_attr(rustfmt, rustfmt_skip)]
//! The complete token set for one theme.
//!
//! DO NOT EDIT - This file is auto-generated

use gpui::{FontWeight, Hsla, Pixels, px};

use super::colors;
use crate::foundation::{
    BezierCurve, ColorDescriptor, ColorRecipe, ColorSwatch, TokenShadow,
    TypeStyle,
};

/// The half of the token set a theme may re-point.
///
/// Rust has no default arguments, so this is how a theme names only what
/// moves: `..STUDIO_LIGHT_SEED` fills in the rest. Everything written in
/// terms of these — the control profiles, the component parts, the
/// interactive recipes — is derived by [`ThemeVariables::resolve`] and so
/// follows a re-pointed role on its own.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeSeed {
    pub color_primary: ColorSwatch,
    pub color_neutral: ColorSwatch,
    pub color_info: ColorSwatch,
    pub color_success: ColorSwatch,
    pub color_warning: ColorSwatch,
    pub color_danger: ColorSwatch,
    pub color_canvas: Hsla,
    pub color_surface: Hsla,
    pub color_surface_muted: Hsla,
    pub color_surface_sunken: Hsla,
    pub color_surface_subtle: Hsla,
    pub color_surface_inset: Hsla,
    pub color_surface_raised: Hsla,
    pub color_surface_overlay: Hsla,
    pub color_surface_chrome: Hsla,
    pub color_surface_column: Hsla,
    pub color_content: Hsla,
    pub color_content_secondary: Hsla,
    pub color_content_nav: Hsla,
    pub color_content_muted: Hsla,
    pub color_content_subtle: Hsla,
    pub color_content_faint: Hsla,
    pub color_border: Hsla,
    pub color_border_strong: Hsla,
    pub color_border_muted: Hsla,
    pub color_on_accent: Hsla,
    pub control_pressed_alpha: f32,
    pub control_tiny_size: Pixels,
    pub control_small_size: Pixels,
    pub control_medium_size: Pixels,
    pub control_large_size: Pixels,
    pub shadow_2xs: &'static [TokenShadow],
    pub shadow_xs: &'static [TokenShadow],
    pub shadow_sm: &'static [TokenShadow],
    pub shadow_md: &'static [TokenShadow],
    pub shadow_lg: &'static [TokenShadow],
    pub shadow_xl: &'static [TokenShadow],
    pub shadow_2xl: &'static [TokenShadow],
    pub focus_width: Pixels,
    pub focus_offset: Pixels,
    pub focus_glow_shade: u16,
    pub focus_glow_alpha: f32,
    pub focus_ring_shade: u16,
    pub focus_ring_alpha: f32,
    pub frame_window_radius: Pixels,
    pub frame_popover_radius: Pixels,
    pub frame_titlebar_size: Pixels,
    pub frame_sidebar_width: Pixels,
    pub frame_sidebar_icon_width: Pixels,
    pub frame_rail_width: Pixels,
    pub frame_aside_width: Pixels,
    pub frame_nav_gap: Pixels,
    pub motion_duration: f32,
    pub motion_easing: BezierCurve,
    pub radius_none: Pixels,
    pub radius_tiny: Pixels,
    pub radius_small: Pixels,
    pub radius_medium: Pixels,
    pub radius_large: Pixels,
    pub radius_big: Pixels,
    pub radius_full: Pixels,
    pub spacing_px: Pixels,
    pub spacing_0: Pixels,
    pub spacing_0_5: Pixels,
    pub spacing_1: Pixels,
    pub spacing_1_5: Pixels,
    pub spacing_2: Pixels,
    pub spacing_2_5: Pixels,
    pub spacing_3: Pixels,
    pub spacing_3_5: Pixels,
    pub spacing_4: Pixels,
    pub spacing_5: Pixels,
    pub spacing_6: Pixels,
    pub spacing_7: Pixels,
    pub spacing_8: Pixels,
    pub spacing_9: Pixels,
    pub spacing_10: Pixels,
    pub spacing_11: Pixels,
    pub spacing_12: Pixels,
    pub spacing_14: Pixels,
    pub spacing_16: Pixels,
    pub spacing_20: Pixels,
    pub stroke_hairline: Pixels,
    pub stroke_control: Pixels,
    pub headline_small: TypeStyle,
    pub headline_medium: TypeStyle,
    pub headline_large: TypeStyle,
    pub title_small: TypeStyle,
    pub title_medium: TypeStyle,
    pub title_large: TypeStyle,
    pub body_small: TypeStyle,
    pub body_medium: TypeStyle,
    pub body_large: TypeStyle,
    pub label_quiet: TypeStyle,
    pub label_strong: TypeStyle,
    pub label_small: TypeStyle,
    pub label_medium: TypeStyle,
    pub label_large: TypeStyle,
    pub caption_small: TypeStyle,
    pub caption_medium: TypeStyle,
    pub caption_large: TypeStyle,
    pub wash_surface: f32,
    pub wash_edge: f32,
    pub checkbox_radius: Pixels,
    pub dialog_width: Pixels,
    pub dialog_scrim_alpha: f32,
    pub menu_min_width: Pixels,
    pub menu_item_padding: Pixels,
    pub preferences_width: Pixels,
    pub progress_gradient_from: Hsla,
    pub progress_gradient_to: Hsla,
    pub segmented_control_inset: Pixels,
    pub shortcut_recorder_width: Pixels,
    pub switch_medium_width: Pixels,
    pub switch_medium_height: Pixels,
    pub switch_medium_thumb: Pixels,
    pub toast_max_width: Pixels,
    pub control_color_recessed_border: ColorDescriptor,
    /// The ramp step `control_color_filled_surface` picks when normal; a theme re-points it.
    pub control_color_filled_surface_normal_shade: u16,
    /// The ramp step `control_color_filled_surface` picks when hovered; a theme re-points it.
    pub control_color_filled_surface_hovered_shade: u16,
    /// The ramp step `control_color_filled_surface` picks when pressed; a theme re-points it.
    pub control_color_filled_surface_pressed_shade: u16,
    pub control_color_filled_border: ColorDescriptor,
    /// The ramp step `control_color_tinted_content` picks when normal; a theme re-points it.
    pub control_color_tinted_content_normal_shade: u16,
    /// The ramp step `control_color_tinted_content` picks when hovered; a theme re-points it.
    pub control_color_tinted_content_hovered_shade: u16,
    /// The ramp step `control_color_tinted_content` picks when pressed; a theme re-points it.
    pub control_color_tinted_content_pressed_shade: u16,
    pub control_color_tinted_border: ColorDescriptor,
    /// The ramp step `control_color_outlined_content` picks when normal; a theme re-points it.
    pub control_color_outlined_content_normal_shade: u16,
    /// The ramp step `control_color_outlined_content` picks when hovered; a theme re-points it.
    pub control_color_outlined_content_hovered_shade: u16,
    /// The ramp step `control_color_outlined_content` picks when pressed; a theme re-points it.
    pub control_color_outlined_content_pressed_shade: u16,
    pub control_color_outlined_border: ColorDescriptor,
    pub control_color_plain_surface: ColorDescriptor,
    /// The ramp step `control_color_plain_content` picks when normal; a theme re-points it.
    pub control_color_plain_content_normal_shade: u16,
    /// The ramp step `control_color_plain_content` picks when hovered; a theme re-points it.
    pub control_color_plain_content_hovered_shade: u16,
    /// The ramp step `control_color_plain_content` picks when pressed; a theme re-points it.
    pub control_color_plain_content_pressed_shade: u16,
    pub control_color_plain_border: ColorDescriptor,
}

/// Studio Light — the baseline every other theme is written against.
pub const STUDIO_LIGHT_SEED: ThemeSeed = ThemeSeed {
    color_primary: colors::BRAND,
    color_neutral: colors::NEUTRAL,
    color_info: colors::SKY,
    color_success: colors::GREEN,
    color_warning: colors::AMBER,
    color_danger: colors::RED,
    color_canvas: Hsla { h: 0.7, s: 0.357143, l: 0.945098, a: 1.0 },
    color_surface: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_muted: Hsla { h: 0.666667, s: 0.230769, l: 0.97451, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.666667, s: 0.191489, l: 0.907843, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.651515, s: 0.354839, l: 0.121569, a: 0.058824 },
    color_surface_inset: Hsla { h: 0.666667, s: 0.2, l: 0.95098, a: 1.0 },
    color_surface_raised: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.666667, s: 0.230769, l: 0.97451, a: 1.0 },
    color_surface_column: Hsla { h: 0.666667, s: 0.25, l: 0.984314, a: 1.0 },
    color_content: Hsla { h: 0.652778, s: 0.4, l: 0.117647, a: 1.0 },
    color_content_secondary: Hsla { h: 0.645833, s: 0.210526, l: 0.298039, a: 1.0 },
    color_content_nav: Hsla { h: 0.641414, s: 0.18232, l: 0.354902, a: 1.0 },
    color_content_muted: Hsla { h: 0.642157, s: 0.165049, l: 0.403922, a: 1.0 },
    color_content_subtle: Hsla { h: 0.633333, s: 0.15, l: 0.607843, a: 1.0 },
    color_content_faint: Hsla { h: 0.635802, s: 0.169811, l: 0.688235, a: 1.0 },
    color_border: Hsla { h: 0.651515, s: 0.354839, l: 0.121569, a: 0.070588 },
    color_border_strong: Hsla { h: 0.651515, s: 0.354839, l: 0.121569, a: 0.090196 },
    color_border_muted: Hsla { h: 0.633333, s: 0.116279, l: 0.421569, a: 1.0 },
    color_on_accent: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    control_pressed_alpha: 0.9,
    control_tiny_size: px(24.0),
    control_small_size: px(26.0),
    control_medium_size: px(28.0),
    control_large_size: px(32.0),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.101961 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.121569 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(12.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.141176 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(8.0), blur: px(24.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.2 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(30.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.219608 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(12.0), blur: px(32.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.239216 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(56.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.278431 } }],
    focus_width: px(3.0),
    focus_offset: px(0.0),
    focus_glow_shade: 600,
    focus_glow_alpha: 0.14,
    focus_ring_shade: 600,
    focus_ring_alpha: 0.45,
    frame_window_radius: px(18.0),
    frame_popover_radius: px(16.0),
    frame_titlebar_size: px(52.0),
    frame_sidebar_width: px(172.0),
    frame_sidebar_icon_width: px(48.0),
    frame_rail_width: px(150.0),
    frame_aside_width: px(214.0),
    frame_nav_gap: px(3.0),
    motion_duration: 0.15,
    motion_easing: BezierCurve { x1: 0.4, y1: 0.0, x2: 0.2, y2: 1.0 },
    radius_none: px(0.0),
    radius_tiny: px(7.0),
    radius_small: px(8.0),
    radius_medium: px(10.0),
    radius_large: px(12.0),
    radius_big: px(16.0),
    radius_full: px(9999.0),
    spacing_px: px(1.0),
    spacing_0: px(0.0),
    spacing_0_5: px(2.0),
    spacing_1: px(4.0),
    spacing_1_5: px(6.0),
    spacing_2: px(8.0),
    spacing_2_5: px(10.0),
    spacing_3: px(12.0),
    spacing_3_5: px(14.0),
    spacing_4: px(16.0),
    spacing_5: px(20.0),
    spacing_6: px(24.0),
    spacing_7: px(28.0),
    spacing_8: px(32.0),
    spacing_9: px(36.0),
    spacing_10: px(40.0),
    spacing_11: px(44.0),
    spacing_12: px(48.0),
    spacing_14: px(56.0),
    spacing_16: px(64.0),
    spacing_20: px(80.0),
    stroke_hairline: px(1.0),
    stroke_control: px(1.5),
    headline_small: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(20.0), weight: FontWeight::SEMIBOLD, line_height: px(26.0) },
    headline_medium: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(24.0), weight: FontWeight::SEMIBOLD, line_height: px(30.0) },
    headline_large: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(28.0), weight: FontWeight::SEMIBOLD, line_height: px(34.0) },
    title_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::SEMIBOLD, line_height: px(20.0) },
    title_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(15.0), weight: FontWeight::SEMIBOLD, line_height: px(22.0) },
    title_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(17.0), weight: FontWeight::SEMIBOLD, line_height: px(22.0) },
    body_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::NORMAL, line_height: px(18.0) },
    body_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::NORMAL, line_height: px(20.0) },
    body_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(15.0), weight: FontWeight::NORMAL, line_height: px(22.0) },
    label_quiet: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::MEDIUM, line_height: px(12.0) },
    label_strong: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::BOLD, line_height: px(12.0) },
    label_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(11.0), weight: FontWeight::SEMIBOLD, line_height: px(11.0) },
    label_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::SEMIBOLD, line_height: px(12.0) },
    label_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::SEMIBOLD, line_height: px(13.0) },
    caption_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(11.0), weight: FontWeight::NORMAL, line_height: px(14.0) },
    caption_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::NORMAL, line_height: px(18.0) },
    caption_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::NORMAL, line_height: px(20.0) },
    wash_surface: 0.06,
    wash_edge: 0.2,
    checkbox_radius: px(5.0),
    dialog_width: px(440.0),
    dialog_scrim_alpha: 0.25,
    menu_min_width: px(176.0),
    menu_item_padding: px(7.0),
    preferences_width: px(480.0),
    progress_gradient_from: colors::BRAND.shade(600),
    progress_gradient_to: Hsla { h: 0.695652, s: 1.0, l: 0.77451, a: 1.0 },
    segmented_control_inset: px(3.0),
    shortcut_recorder_width: px(132.0),
    switch_medium_width: px(32.0),
    switch_medium_height: px(18.0),
    switch_medium_thumb: px(14.0),
    toast_max_width: px(420.0),
    control_color_recessed_border: ColorDescriptor {
        normal: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        hovered: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        pressed: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
    },
    control_color_filled_surface_normal_shade: 600,
    control_color_filled_surface_hovered_shade: 700,
    control_color_filled_surface_pressed_shade: 700,
    control_color_filled_border: ColorDescriptor {
        normal: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        hovered: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        pressed: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
    },
    control_color_tinted_content_normal_shade: 700,
    control_color_tinted_content_hovered_shade: 800,
    control_color_tinted_content_pressed_shade: 800,
    control_color_tinted_border: ColorDescriptor {
        normal: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        hovered: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        pressed: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
    },
    control_color_outlined_content_normal_shade: 700,
    control_color_outlined_content_hovered_shade: 800,
    control_color_outlined_content_pressed_shade: 800,
    control_color_outlined_border: ColorDescriptor {
        normal: ColorRecipe { color: None, shade: Some(600), alpha: Some(1.0) },
        hovered: ColorRecipe { color: None, shade: Some(700), alpha: Some(1.0) },
        pressed: ColorRecipe { color: None, shade: Some(700), alpha: Some(0.9) },
        disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
    },
    control_color_plain_surface: ColorDescriptor {
        normal: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        hovered: ColorRecipe { color: None, shade: Some(600), alpha: Some(0.08) },
        pressed: ColorRecipe { color: None, shade: Some(600), alpha: Some(0.12) },
        disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
    },
    control_color_plain_content_normal_shade: 700,
    control_color_plain_content_hovered_shade: 800,
    control_color_plain_content_pressed_shade: 800,
    control_color_plain_border: ColorDescriptor {
        normal: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        hovered: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        pressed: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
    },
};

/// Every token, for one theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeVariables {
    pub color_primary: ColorSwatch,
    pub color_neutral: ColorSwatch,
    pub color_info: ColorSwatch,
    pub color_success: ColorSwatch,
    pub color_warning: ColorSwatch,
    pub color_danger: ColorSwatch,
    pub color_canvas: Hsla,
    pub color_surface: Hsla,
    pub color_surface_muted: Hsla,
    pub color_surface_sunken: Hsla,
    pub color_surface_subtle: Hsla,
    pub color_surface_inset: Hsla,
    pub color_surface_raised: Hsla,
    pub color_surface_overlay: Hsla,
    pub color_surface_chrome: Hsla,
    pub color_surface_column: Hsla,
    pub color_content: Hsla,
    pub color_content_secondary: Hsla,
    pub color_content_nav: Hsla,
    pub color_content_muted: Hsla,
    pub color_content_subtle: Hsla,
    pub color_content_faint: Hsla,
    pub color_border: Hsla,
    pub color_border_strong: Hsla,
    pub color_border_muted: Hsla,
    pub color_on_accent: Hsla,
    pub control_pressed_alpha: f32,
    pub control_tiny_size: Pixels,
    pub control_small_size: Pixels,
    pub control_medium_size: Pixels,
    pub control_large_size: Pixels,
    pub shadow_2xs: &'static [TokenShadow],
    pub shadow_xs: &'static [TokenShadow],
    pub shadow_sm: &'static [TokenShadow],
    pub shadow_md: &'static [TokenShadow],
    pub shadow_lg: &'static [TokenShadow],
    pub shadow_xl: &'static [TokenShadow],
    pub shadow_2xl: &'static [TokenShadow],
    pub focus_width: Pixels,
    pub focus_offset: Pixels,
    pub focus_glow_shade: u16,
    pub focus_glow_alpha: f32,
    pub focus_ring_shade: u16,
    pub focus_ring_alpha: f32,
    pub frame_window_radius: Pixels,
    pub frame_popover_radius: Pixels,
    pub frame_titlebar_size: Pixels,
    pub frame_sidebar_width: Pixels,
    pub frame_sidebar_icon_width: Pixels,
    pub frame_rail_width: Pixels,
    pub frame_aside_width: Pixels,
    pub frame_nav_gap: Pixels,
    pub motion_duration: f32,
    pub motion_easing: BezierCurve,
    pub radius_none: Pixels,
    pub radius_tiny: Pixels,
    pub radius_small: Pixels,
    pub radius_medium: Pixels,
    pub radius_large: Pixels,
    pub radius_big: Pixels,
    pub radius_full: Pixels,
    pub spacing_px: Pixels,
    pub spacing_0: Pixels,
    pub spacing_0_5: Pixels,
    pub spacing_1: Pixels,
    pub spacing_1_5: Pixels,
    pub spacing_2: Pixels,
    pub spacing_2_5: Pixels,
    pub spacing_3: Pixels,
    pub spacing_3_5: Pixels,
    pub spacing_4: Pixels,
    pub spacing_5: Pixels,
    pub spacing_6: Pixels,
    pub spacing_7: Pixels,
    pub spacing_8: Pixels,
    pub spacing_9: Pixels,
    pub spacing_10: Pixels,
    pub spacing_11: Pixels,
    pub spacing_12: Pixels,
    pub spacing_14: Pixels,
    pub spacing_16: Pixels,
    pub spacing_20: Pixels,
    pub stroke_hairline: Pixels,
    pub stroke_control: Pixels,
    pub headline_small: TypeStyle,
    pub headline_medium: TypeStyle,
    pub headline_large: TypeStyle,
    pub title_small: TypeStyle,
    pub title_medium: TypeStyle,
    pub title_large: TypeStyle,
    pub body_small: TypeStyle,
    pub body_medium: TypeStyle,
    pub body_large: TypeStyle,
    pub label_quiet: TypeStyle,
    pub label_strong: TypeStyle,
    pub label_small: TypeStyle,
    pub label_medium: TypeStyle,
    pub label_large: TypeStyle,
    pub caption_small: TypeStyle,
    pub caption_medium: TypeStyle,
    pub caption_large: TypeStyle,
    pub wash_surface: f32,
    pub wash_edge: f32,
    pub checkbox_radius: Pixels,
    pub dialog_width: Pixels,
    pub dialog_scrim_alpha: f32,
    pub menu_min_width: Pixels,
    pub menu_item_padding: Pixels,
    pub preferences_width: Pixels,
    pub progress_gradient_from: Hsla,
    pub progress_gradient_to: Hsla,
    pub segmented_control_inset: Pixels,
    pub shortcut_recorder_width: Pixels,
    pub switch_medium_width: Pixels,
    pub switch_medium_height: Pixels,
    pub switch_medium_thumb: Pixels,
    pub toast_max_width: Pixels,
    pub control_color_recessed_border: ColorDescriptor,
    /// The ramp step `control_color_filled_surface` picks when normal; a theme re-points it.
    pub control_color_filled_surface_normal_shade: u16,
    /// The ramp step `control_color_filled_surface` picks when hovered; a theme re-points it.
    pub control_color_filled_surface_hovered_shade: u16,
    /// The ramp step `control_color_filled_surface` picks when pressed; a theme re-points it.
    pub control_color_filled_surface_pressed_shade: u16,
    pub control_color_filled_border: ColorDescriptor,
    /// The ramp step `control_color_tinted_content` picks when normal; a theme re-points it.
    pub control_color_tinted_content_normal_shade: u16,
    /// The ramp step `control_color_tinted_content` picks when hovered; a theme re-points it.
    pub control_color_tinted_content_hovered_shade: u16,
    /// The ramp step `control_color_tinted_content` picks when pressed; a theme re-points it.
    pub control_color_tinted_content_pressed_shade: u16,
    pub control_color_tinted_border: ColorDescriptor,
    /// The ramp step `control_color_outlined_content` picks when normal; a theme re-points it.
    pub control_color_outlined_content_normal_shade: u16,
    /// The ramp step `control_color_outlined_content` picks when hovered; a theme re-points it.
    pub control_color_outlined_content_hovered_shade: u16,
    /// The ramp step `control_color_outlined_content` picks when pressed; a theme re-points it.
    pub control_color_outlined_content_pressed_shade: u16,
    pub control_color_outlined_border: ColorDescriptor,
    pub control_color_plain_surface: ColorDescriptor,
    /// The ramp step `control_color_plain_content` picks when normal; a theme re-points it.
    pub control_color_plain_content_normal_shade: u16,
    /// The ramp step `control_color_plain_content` picks when hovered; a theme re-points it.
    pub control_color_plain_content_hovered_shade: u16,
    /// The ramp step `control_color_plain_content` picks when pressed; a theme re-points it.
    pub control_color_plain_content_pressed_shade: u16,
    pub control_color_plain_border: ColorDescriptor,
    pub control_tiny_gap: Pixels,
    pub control_tiny_padding_block: Pixels,
    pub control_tiny_padding_inline: Pixels,
    pub control_tiny_radius: Pixels,
    pub control_tiny_content: TypeStyle,
    pub control_small_gap: Pixels,
    pub control_small_padding_block: Pixels,
    pub control_small_padding_inline: Pixels,
    pub control_small_radius: Pixels,
    pub control_small_content: TypeStyle,
    pub control_medium_gap: Pixels,
    pub control_medium_padding_block: Pixels,
    pub control_medium_padding_inline: Pixels,
    pub control_medium_radius: Pixels,
    pub control_medium_content: TypeStyle,
    pub control_large_gap: Pixels,
    pub control_large_padding_block: Pixels,
    pub control_large_padding_inline: Pixels,
    pub control_large_radius: Pixels,
    pub control_large_content: TypeStyle,
    pub checkbox_thickness: Pixels,
    pub checkbox_small_box: Pixels,
    pub checkbox_medium_box: Pixels,
    pub checkbox_large_box: Pixels,
    pub dialog_scrim_color: Hsla,
    pub radio_thickness: Pixels,
    pub radio_small_box: Pixels,
    pub radio_small_dot: Pixels,
    pub radio_medium_box: Pixels,
    pub radio_medium_dot: Pixels,
    pub radio_large_box: Pixels,
    pub radio_large_dot: Pixels,
    pub switch_small_width: Pixels,
    pub switch_small_height: Pixels,
    pub switch_small_thumb: Pixels,
    pub switch_large_width: Pixels,
    pub switch_large_height: Pixels,
    pub switch_large_thumb: Pixels,
    pub control_color_normal_surface: ColorDescriptor,
    pub control_color_normal_content: ColorDescriptor,
    pub control_color_normal_border: ColorDescriptor,
    pub control_color_recessed_surface: ColorDescriptor,
    pub control_color_recessed_content: ColorDescriptor,
    pub control_color_filled_surface: ColorDescriptor,
    pub control_color_filled_content: ColorDescriptor,
    pub control_color_tinted_surface: ColorDescriptor,
    pub control_color_tinted_content: ColorDescriptor,
    pub control_color_outlined_surface: ColorDescriptor,
    pub control_color_outlined_content: ColorDescriptor,
    pub control_color_plain_content: ColorDescriptor,
}

impl ThemeVariables {
    /// Derive the whole token set from the half a theme re-points.
    ///
    /// This is the Rust half of what a cascade does for CSS and a computed
    /// property does for Swift: a role moves once in the seed and everything
    /// written in terms of it follows here.
    pub const fn resolve(seed: ThemeSeed) -> Self {
        let color_primary = seed.color_primary;
        let color_neutral = seed.color_neutral;
        let color_info = seed.color_info;
        let color_success = seed.color_success;
        let color_warning = seed.color_warning;
        let color_danger = seed.color_danger;
        let color_canvas = seed.color_canvas;
        let color_surface = seed.color_surface;
        let color_surface_muted = seed.color_surface_muted;
        let color_surface_sunken = seed.color_surface_sunken;
        let color_surface_subtle = seed.color_surface_subtle;
        let color_surface_inset = seed.color_surface_inset;
        let color_surface_raised = seed.color_surface_raised;
        let color_surface_overlay = seed.color_surface_overlay;
        let color_surface_chrome = seed.color_surface_chrome;
        let color_surface_column = seed.color_surface_column;
        let color_content = seed.color_content;
        let color_content_secondary = seed.color_content_secondary;
        let color_content_nav = seed.color_content_nav;
        let color_content_muted = seed.color_content_muted;
        let color_content_subtle = seed.color_content_subtle;
        let color_content_faint = seed.color_content_faint;
        let color_border = seed.color_border;
        let color_border_strong = seed.color_border_strong;
        let color_border_muted = seed.color_border_muted;
        let color_on_accent = seed.color_on_accent;
        let control_pressed_alpha = seed.control_pressed_alpha;
        let control_tiny_size = seed.control_tiny_size;
        let control_small_size = seed.control_small_size;
        let control_medium_size = seed.control_medium_size;
        let control_large_size = seed.control_large_size;
        let shadow_2xs = seed.shadow_2xs;
        let shadow_xs = seed.shadow_xs;
        let shadow_sm = seed.shadow_sm;
        let shadow_md = seed.shadow_md;
        let shadow_lg = seed.shadow_lg;
        let shadow_xl = seed.shadow_xl;
        let shadow_2xl = seed.shadow_2xl;
        let focus_width = seed.focus_width;
        let focus_offset = seed.focus_offset;
        let focus_glow_shade = seed.focus_glow_shade;
        let focus_glow_alpha = seed.focus_glow_alpha;
        let focus_ring_shade = seed.focus_ring_shade;
        let focus_ring_alpha = seed.focus_ring_alpha;
        let frame_window_radius = seed.frame_window_radius;
        let frame_popover_radius = seed.frame_popover_radius;
        let frame_titlebar_size = seed.frame_titlebar_size;
        let frame_sidebar_width = seed.frame_sidebar_width;
        let frame_sidebar_icon_width = seed.frame_sidebar_icon_width;
        let frame_rail_width = seed.frame_rail_width;
        let frame_aside_width = seed.frame_aside_width;
        let frame_nav_gap = seed.frame_nav_gap;
        let motion_duration = seed.motion_duration;
        let motion_easing = seed.motion_easing;
        let radius_none = seed.radius_none;
        let radius_tiny = seed.radius_tiny;
        let radius_small = seed.radius_small;
        let radius_medium = seed.radius_medium;
        let radius_large = seed.radius_large;
        let radius_big = seed.radius_big;
        let radius_full = seed.radius_full;
        let spacing_px = seed.spacing_px;
        let spacing_0 = seed.spacing_0;
        let spacing_0_5 = seed.spacing_0_5;
        let spacing_1 = seed.spacing_1;
        let spacing_1_5 = seed.spacing_1_5;
        let spacing_2 = seed.spacing_2;
        let spacing_2_5 = seed.spacing_2_5;
        let spacing_3 = seed.spacing_3;
        let spacing_3_5 = seed.spacing_3_5;
        let spacing_4 = seed.spacing_4;
        let spacing_5 = seed.spacing_5;
        let spacing_6 = seed.spacing_6;
        let spacing_7 = seed.spacing_7;
        let spacing_8 = seed.spacing_8;
        let spacing_9 = seed.spacing_9;
        let spacing_10 = seed.spacing_10;
        let spacing_11 = seed.spacing_11;
        let spacing_12 = seed.spacing_12;
        let spacing_14 = seed.spacing_14;
        let spacing_16 = seed.spacing_16;
        let spacing_20 = seed.spacing_20;
        let stroke_hairline = seed.stroke_hairline;
        let stroke_control = seed.stroke_control;
        let headline_small = seed.headline_small;
        let headline_medium = seed.headline_medium;
        let headline_large = seed.headline_large;
        let title_small = seed.title_small;
        let title_medium = seed.title_medium;
        let title_large = seed.title_large;
        let body_small = seed.body_small;
        let body_medium = seed.body_medium;
        let body_large = seed.body_large;
        let label_quiet = seed.label_quiet;
        let label_strong = seed.label_strong;
        let label_small = seed.label_small;
        let label_medium = seed.label_medium;
        let label_large = seed.label_large;
        let caption_small = seed.caption_small;
        let caption_medium = seed.caption_medium;
        let caption_large = seed.caption_large;
        let wash_surface = seed.wash_surface;
        let wash_edge = seed.wash_edge;
        let checkbox_radius = seed.checkbox_radius;
        let dialog_width = seed.dialog_width;
        let dialog_scrim_alpha = seed.dialog_scrim_alpha;
        let menu_min_width = seed.menu_min_width;
        let menu_item_padding = seed.menu_item_padding;
        let preferences_width = seed.preferences_width;
        let progress_gradient_from = seed.progress_gradient_from;
        let progress_gradient_to = seed.progress_gradient_to;
        let segmented_control_inset = seed.segmented_control_inset;
        let shortcut_recorder_width = seed.shortcut_recorder_width;
        let switch_medium_width = seed.switch_medium_width;
        let switch_medium_height = seed.switch_medium_height;
        let switch_medium_thumb = seed.switch_medium_thumb;
        let toast_max_width = seed.toast_max_width;
        let control_color_recessed_border = seed.control_color_recessed_border;
        let control_color_filled_surface_normal_shade = seed.control_color_filled_surface_normal_shade;
        let control_color_filled_surface_hovered_shade = seed.control_color_filled_surface_hovered_shade;
        let control_color_filled_surface_pressed_shade = seed.control_color_filled_surface_pressed_shade;
        let control_color_filled_border = seed.control_color_filled_border;
        let control_color_tinted_content_normal_shade = seed.control_color_tinted_content_normal_shade;
        let control_color_tinted_content_hovered_shade = seed.control_color_tinted_content_hovered_shade;
        let control_color_tinted_content_pressed_shade = seed.control_color_tinted_content_pressed_shade;
        let control_color_tinted_border = seed.control_color_tinted_border;
        let control_color_outlined_content_normal_shade = seed.control_color_outlined_content_normal_shade;
        let control_color_outlined_content_hovered_shade = seed.control_color_outlined_content_hovered_shade;
        let control_color_outlined_content_pressed_shade = seed.control_color_outlined_content_pressed_shade;
        let control_color_outlined_border = seed.control_color_outlined_border;
        let control_color_plain_surface = seed.control_color_plain_surface;
        let control_color_plain_content_normal_shade = seed.control_color_plain_content_normal_shade;
        let control_color_plain_content_hovered_shade = seed.control_color_plain_content_hovered_shade;
        let control_color_plain_content_pressed_shade = seed.control_color_plain_content_pressed_shade;
        let control_color_plain_border = seed.control_color_plain_border;

        let control_tiny_gap = spacing_1;
        let control_tiny_padding_block = spacing_1;
        let control_tiny_padding_inline = spacing_2_5;
        let control_tiny_radius = radius_small;
        let control_tiny_content = label_small;
        let control_small_gap = spacing_1_5;
        let control_small_padding_block = spacing_1;
        let control_small_padding_inline = spacing_3;
        let control_small_radius = radius_small;
        let control_small_content = label_medium;
        let control_medium_gap = spacing_2;
        let control_medium_padding_block = spacing_1_5;
        let control_medium_padding_inline = spacing_4;
        let control_medium_radius = radius_small;
        let control_medium_content = label_medium;
        let control_large_gap = spacing_2;
        let control_large_padding_block = spacing_1_5;
        let control_large_padding_inline = spacing_4;
        let control_large_radius = radius_medium;
        let control_large_content = label_medium;
        let checkbox_thickness = stroke_control;
        let checkbox_small_box = spacing_3_5;
        let checkbox_medium_box = spacing_4;
        let checkbox_large_box = spacing_5;
        let dialog_scrim_color = color_content;
        let radio_thickness = stroke_control;
        let radio_small_box = spacing_3_5;
        let radio_small_dot = spacing_1_5;
        let radio_medium_box = spacing_4;
        let radio_medium_dot = spacing_2;
        let radio_large_box = spacing_5;
        let radio_large_dot = spacing_2_5;
        let switch_small_width = spacing_7;
        let switch_small_height = spacing_4;
        let switch_small_thumb = spacing_3;
        let switch_large_width = spacing_11;
        let switch_large_height = spacing_6;
        let switch_large_thumb = spacing_5;
        let control_color_normal_surface = ColorDescriptor {
            normal: ColorRecipe { color: Some(color_surface), shade: None, alpha: None },
            hovered: ColorRecipe { color: Some(color_surface_subtle), shade: None, alpha: None },
            pressed: ColorRecipe { color: Some(color_surface_subtle), shade: None, alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_surface_sunken), shade: None, alpha: None },
        };
        let control_color_normal_content = ColorDescriptor {
            normal: ColorRecipe { color: Some(color_content), shade: None, alpha: None },
            hovered: ColorRecipe { color: Some(color_content), shade: None, alpha: None },
            pressed: ColorRecipe { color: Some(color_content), shade: None, alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_content_faint), shade: None, alpha: None },
        };
        let control_color_normal_border = ColorDescriptor {
            normal: ColorRecipe { color: Some(color_border_strong), shade: None, alpha: None },
            hovered: ColorRecipe { color: Some(color_border_strong), shade: None, alpha: None },
            pressed: ColorRecipe { color: Some(color_border_strong), shade: None, alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        };
        let control_color_recessed_surface = ColorDescriptor {
            normal: ColorRecipe { color: Some(color_surface_inset), shade: None, alpha: None },
            hovered: ColorRecipe { color: Some(color_surface_sunken), shade: None, alpha: None },
            pressed: ColorRecipe { color: Some(color_surface_sunken), shade: None, alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_surface_sunken), shade: None, alpha: None },
        };
        let control_color_recessed_content = ColorDescriptor {
            normal: ColorRecipe { color: Some(color_content), shade: None, alpha: None },
            hovered: ColorRecipe { color: Some(color_content), shade: None, alpha: None },
            pressed: ColorRecipe { color: Some(color_content), shade: None, alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_content_faint), shade: None, alpha: None },
        };
        let control_color_filled_surface = ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(control_color_filled_surface_normal_shade), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(control_color_filled_surface_hovered_shade), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(control_color_filled_surface_pressed_shade), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_surface_sunken), shade: None, alpha: None },
        };
        let control_color_filled_content = ColorDescriptor {
            normal: ColorRecipe { color: Some(color_on_accent), shade: None, alpha: None },
            hovered: ColorRecipe { color: Some(color_on_accent), shade: None, alpha: None },
            pressed: ColorRecipe { color: Some(color_on_accent), shade: None, alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_content_faint), shade: None, alpha: None },
        };
        let control_color_tinted_surface = ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(600), alpha: Some(0.12) },
            hovered: ColorRecipe { color: None, shade: Some(600), alpha: Some(0.2) },
            pressed: ColorRecipe { color: None, shade: Some(600), alpha: Some(0.24) },
            disabled: ColorRecipe { color: Some(color_surface_sunken), shade: None, alpha: None },
        };
        let control_color_tinted_content = ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(control_color_tinted_content_normal_shade), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(control_color_tinted_content_hovered_shade), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(control_color_tinted_content_pressed_shade), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_content_faint), shade: None, alpha: None },
        };
        let control_color_outlined_surface = ColorDescriptor {
            normal: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
            hovered: ColorRecipe { color: None, shade: Some(600), alpha: Some(0.08) },
            pressed: ColorRecipe { color: None, shade: Some(600), alpha: Some(0.12) },
            disabled: ColorRecipe { color: Some(color_surface_sunken), shade: None, alpha: None },
        };
        let control_color_outlined_content = ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(control_color_outlined_content_normal_shade), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(control_color_outlined_content_hovered_shade), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(control_color_outlined_content_pressed_shade), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_content_faint), shade: None, alpha: None },
        };
        let control_color_plain_content = ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(control_color_plain_content_normal_shade), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(control_color_plain_content_hovered_shade), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(control_color_plain_content_pressed_shade), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(color_content_faint), shade: None, alpha: None },
        };

        Self {
            color_primary,
            color_neutral,
            color_info,
            color_success,
            color_warning,
            color_danger,
            color_canvas,
            color_surface,
            color_surface_muted,
            color_surface_sunken,
            color_surface_subtle,
            color_surface_inset,
            color_surface_raised,
            color_surface_overlay,
            color_surface_chrome,
            color_surface_column,
            color_content,
            color_content_secondary,
            color_content_nav,
            color_content_muted,
            color_content_subtle,
            color_content_faint,
            color_border,
            color_border_strong,
            color_border_muted,
            color_on_accent,
            control_pressed_alpha,
            control_tiny_size,
            control_small_size,
            control_medium_size,
            control_large_size,
            shadow_2xs,
            shadow_xs,
            shadow_sm,
            shadow_md,
            shadow_lg,
            shadow_xl,
            shadow_2xl,
            focus_width,
            focus_offset,
            focus_glow_shade,
            focus_glow_alpha,
            focus_ring_shade,
            focus_ring_alpha,
            frame_window_radius,
            frame_popover_radius,
            frame_titlebar_size,
            frame_sidebar_width,
            frame_sidebar_icon_width,
            frame_rail_width,
            frame_aside_width,
            frame_nav_gap,
            motion_duration,
            motion_easing,
            radius_none,
            radius_tiny,
            radius_small,
            radius_medium,
            radius_large,
            radius_big,
            radius_full,
            spacing_px,
            spacing_0,
            spacing_0_5,
            spacing_1,
            spacing_1_5,
            spacing_2,
            spacing_2_5,
            spacing_3,
            spacing_3_5,
            spacing_4,
            spacing_5,
            spacing_6,
            spacing_7,
            spacing_8,
            spacing_9,
            spacing_10,
            spacing_11,
            spacing_12,
            spacing_14,
            spacing_16,
            spacing_20,
            stroke_hairline,
            stroke_control,
            headline_small,
            headline_medium,
            headline_large,
            title_small,
            title_medium,
            title_large,
            body_small,
            body_medium,
            body_large,
            label_quiet,
            label_strong,
            label_small,
            label_medium,
            label_large,
            caption_small,
            caption_medium,
            caption_large,
            wash_surface,
            wash_edge,
            checkbox_radius,
            dialog_width,
            dialog_scrim_alpha,
            menu_min_width,
            menu_item_padding,
            preferences_width,
            progress_gradient_from,
            progress_gradient_to,
            segmented_control_inset,
            shortcut_recorder_width,
            switch_medium_width,
            switch_medium_height,
            switch_medium_thumb,
            toast_max_width,
            control_color_recessed_border,
            control_color_filled_surface_normal_shade,
            control_color_filled_surface_hovered_shade,
            control_color_filled_surface_pressed_shade,
            control_color_filled_border,
            control_color_tinted_content_normal_shade,
            control_color_tinted_content_hovered_shade,
            control_color_tinted_content_pressed_shade,
            control_color_tinted_border,
            control_color_outlined_content_normal_shade,
            control_color_outlined_content_hovered_shade,
            control_color_outlined_content_pressed_shade,
            control_color_outlined_border,
            control_color_plain_surface,
            control_color_plain_content_normal_shade,
            control_color_plain_content_hovered_shade,
            control_color_plain_content_pressed_shade,
            control_color_plain_border,
            control_tiny_gap,
            control_tiny_padding_block,
            control_tiny_padding_inline,
            control_tiny_radius,
            control_tiny_content,
            control_small_gap,
            control_small_padding_block,
            control_small_padding_inline,
            control_small_radius,
            control_small_content,
            control_medium_gap,
            control_medium_padding_block,
            control_medium_padding_inline,
            control_medium_radius,
            control_medium_content,
            control_large_gap,
            control_large_padding_block,
            control_large_padding_inline,
            control_large_radius,
            control_large_content,
            checkbox_thickness,
            checkbox_small_box,
            checkbox_medium_box,
            checkbox_large_box,
            dialog_scrim_color,
            radio_thickness,
            radio_small_box,
            radio_small_dot,
            radio_medium_box,
            radio_medium_dot,
            radio_large_box,
            radio_large_dot,
            switch_small_width,
            switch_small_height,
            switch_small_thumb,
            switch_large_width,
            switch_large_height,
            switch_large_thumb,
            control_color_normal_surface,
            control_color_normal_content,
            control_color_normal_border,
            control_color_recessed_surface,
            control_color_recessed_content,
            control_color_filled_surface,
            control_color_filled_content,
            control_color_tinted_surface,
            control_color_tinted_content,
            control_color_outlined_surface,
            control_color_outlined_content,
            control_color_plain_content,
        }
    }
}

/// Studio Light — white card, light grey canvas, violet accent.
pub static STUDIO_LIGHT: ThemeVariables = ThemeVariables::resolve(STUDIO_LIGHT_SEED);

/// The `bright-light` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static BRIGHT_LIGHT: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.222222, s: 0.222222, l: 0.894118, a: 1.0 },
    color_surface: Hsla { h: 0.125, s: 0.333333, l: 0.976471, a: 1.0 },
    color_surface_muted: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.133333, s: 0.16129, l: 0.878431, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.058824 },
    color_surface_inset: Hsla { h: 0.142857, s: 0.189189, l: 0.927451, a: 1.0 },
    color_surface_raised: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.125, s: 0.333333, l: 0.976471, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_column: Hsla { h: 0.138889, s: 0.214286, l: 0.945098, a: 1.0 },
    color_content: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 1.0 },
    color_content_secondary: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.780392 },
    color_content_nav: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.701961 },
    color_content_muted: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.658824 },
    color_content_subtle: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.45098 },
    color_content_faint: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.380392 },
    color_border: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.078431 },
    color_border_strong: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.090196 },
    color_border_muted: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.25098 },
    color_on_accent: Hsla { h: 0.202257, s: 1.0, l: 0.623529, a: 1.0 },
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.121569 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.141176 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(12.0), spread: px(0.0), color: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.141176 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(8.0), blur: px(24.0), spread: px(0.0), color: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.180392 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(30.0), spread: px(0.0), color: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.180392 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(12.0), blur: px(32.0), spread: px(0.0), color: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.2 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(56.0), spread: px(0.0), color: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 0.239216 } }],
    frame_window_radius: px(16.0),
    frame_popover_radius: px(14.0),
    radius_small: px(9999.0),
    radius_medium: px(9999.0),
    radius_big: px(14.0),
    progress_gradient_from: colors::INK.shade(600),
    progress_gradient_to: colors::INK.shade(300),
    color_primary: colors::INK,
    ..STUDIO_LIGHT_SEED
});

/// The `bright-dark` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static BRIGHT_DARK: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.597222, s: 0.5, l: 0.047059, a: 1.0 },
    color_surface: Hsla { h: 0.592593, s: 0.428571, l: 0.082353, a: 1.0 },
    color_surface_muted: Hsla { h: 0.595238, s: 0.344262, l: 0.119608, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.590278, s: 0.3, l: 0.156863, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.070588 },
    color_surface_inset: Hsla { h: 0.590909, s: 0.333333, l: 0.129412, a: 1.0 },
    color_surface_raised: Hsla { h: 0.589744, s: 0.236364, l: 0.215686, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.595238, s: 0.381818, l: 0.107843, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.595238, s: 0.381818, l: 0.107843, a: 1.0 },
    color_surface_column: Hsla { h: 0.59375, s: 0.444444, l: 0.070588, a: 1.0 },
    color_content: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 1.0 },
    color_content_secondary: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.780392 },
    color_content_nav: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.701961 },
    color_content_muted: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.658824 },
    color_content_subtle: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.45098 },
    color_content_faint: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.380392 },
    color_border: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.078431 },
    color_border_strong: Hsla { h: 0.233333, s: 0.185185, l: 0.947059, a: 0.090196 },
    color_border_muted: Hsla { h: 0.589744, s: 0.19697, l: 0.258824, a: 1.0 },
    color_on_accent: Hsla { h: 0.603448, s: 0.460317, l: 0.123529, a: 1.0 },
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.4 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.54902 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.580392 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.6 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.701961 } }],
    focus_ring_alpha: 0.55,
    frame_window_radius: px(16.0),
    frame_popover_radius: px(14.0),
    radius_small: px(9999.0),
    radius_medium: px(9999.0),
    radius_big: px(14.0),
    progress_gradient_from: colors::ACID.shade(500),
    progress_gradient_to: colors::ACID.shade(500),
    color_danger: colors::RED_DARK,
    color_primary: colors::ACID,
    color_success: colors::GREEN_DARK,
    color_warning: colors::AMBER_DARK,
    control_color_filled_surface_normal_shade: 500,
    control_color_filled_surface_hovered_shade: 600,
    control_color_filled_surface_pressed_shade: 600,
    control_color_outlined_border: ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(400), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(300), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(300), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        },
    control_color_outlined_content_normal_shade: 300,
    control_color_outlined_content_hovered_shade: 200,
    control_color_outlined_content_pressed_shade: 200,
    control_color_plain_content_normal_shade: 300,
    control_color_plain_content_hovered_shade: 200,
    control_color_plain_content_pressed_shade: 200,
    control_color_tinted_content_normal_shade: 300,
    control_color_tinted_content_hovered_shade: 200,
    control_color_tinted_content_pressed_shade: 200,
    ..STUDIO_LIGHT_SEED
});

/// The `frost-light` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static FROST_LIGHT: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.545455, s: 0.268293, l: 0.919608, a: 1.0 },
    color_surface: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_muted: Hsla { h: 0.555556, s: 0.3, l: 0.960784, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.544444, s: 0.263158, l: 0.888235, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.548611, s: 0.461538, l: 0.101961, a: 0.058824 },
    color_surface_inset: Hsla { h: 0.541667, s: 0.266667, l: 0.941176, a: 1.0 },
    color_surface_raised: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.555556, s: 0.3, l: 0.960784, a: 1.0 },
    color_surface_column: Hsla { h: 0.555556, s: 0.3, l: 0.960784, a: 1.0 },
    color_content: Hsla { h: 0.548611, s: 0.461538, l: 0.101961, a: 1.0 },
    color_content_secondary: Hsla { h: 0.54878, s: 0.303704, l: 0.264706, a: 1.0 },
    color_content_nav: Hsla { h: 0.547101, s: 0.283951, l: 0.317647, a: 1.0 },
    color_content_muted: Hsla { h: 0.550725, s: 0.25, l: 0.360784, a: 1.0 },
    color_content_subtle: Hsla { h: 0.54878, s: 0.177489, l: 0.547059, a: 1.0 },
    color_content_faint: Hsla { h: 0.546875, s: 0.188235, l: 0.666667, a: 1.0 },
    color_border: Hsla { h: 0.548611, s: 0.461538, l: 0.101961, a: 0.129412 },
    color_border_strong: Hsla { h: 0.548611, s: 0.461538, l: 0.101961, a: 0.168627 },
    color_border_muted: Hsla { h: 0.550725, s: 0.25, l: 0.360784, a: 1.0 },
    color_on_accent: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    progress_gradient_from: colors::FROST.shade(600),
    progress_gradient_to: colors::FROST.shade(300),
    color_primary: colors::FROST,
    ..STUDIO_LIGHT_SEED
});

/// The `frost-dark` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static FROST_DARK: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.566667, s: 0.416667, l: 0.047059, a: 1.0 },
    color_surface: Hsla { h: 0.564103, s: 0.333333, l: 0.076471, a: 1.0 },
    color_surface_muted: Hsla { h: 0.566667, s: 0.306122, l: 0.096078, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.564103, s: 0.270833, l: 0.188235, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.070588 },
    color_surface_inset: Hsla { h: 0.560606, s: 0.314286, l: 0.137255, a: 1.0 },
    color_surface_raised: Hsla { h: 0.5625, s: 0.225352, l: 0.278431, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.566667, s: 0.306122, l: 0.096078, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.566667, s: 0.306122, l: 0.096078, a: 1.0 },
    color_surface_column: Hsla { h: 0.566667, s: 0.306122, l: 0.096078, a: 1.0 },
    color_content: Hsla { h: 0.551282, s: 0.448276, l: 0.943137, a: 1.0 },
    color_content_secondary: Hsla { h: 0.551724, s: 0.27619, l: 0.794118, a: 1.0 },
    color_content_nav: Hsla { h: 0.553922, s: 0.186813, l: 0.643137, a: 1.0 },
    color_content_muted: Hsla { h: 0.553922, s: 0.186813, l: 0.643137, a: 1.0 },
    color_content_subtle: Hsla { h: 0.54955, s: 0.16895, l: 0.570588, a: 1.0 },
    color_content_faint: Hsla { h: 0.550388, s: 0.179916, l: 0.468627, a: 1.0 },
    color_border: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.160784 },
    color_border_strong: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.2 },
    color_border_muted: Hsla { h: 0.557471, s: 0.224806, l: 0.252941, a: 1.0 },
    color_on_accent: Hsla { h: 0.54, s: 0.757576, l: 0.064706, a: 1.0 },
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.4 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.54902 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.580392 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.6 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.701961 } }],
    focus_ring_alpha: 0.5,
    progress_gradient_from: colors::FROST.shade(500),
    progress_gradient_to: colors::FROST.shade(300),
    color_danger: colors::RED_DARK,
    color_primary: colors::FROST,
    color_success: colors::GREEN_DARK,
    color_warning: colors::AMBER_DARK,
    control_color_filled_surface_normal_shade: 500,
    control_color_filled_surface_hovered_shade: 600,
    control_color_filled_surface_pressed_shade: 600,
    control_color_outlined_border: ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(400), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(300), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(300), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        },
    control_color_outlined_content_normal_shade: 300,
    control_color_outlined_content_hovered_shade: 200,
    control_color_outlined_content_pressed_shade: 200,
    control_color_plain_content_normal_shade: 300,
    control_color_plain_content_hovered_shade: 200,
    control_color_plain_content_pressed_shade: 200,
    control_color_tinted_content_normal_shade: 300,
    control_color_tinted_content_hovered_shade: 200,
    control_color_tinted_content_pressed_shade: 200,
    ..STUDIO_LIGHT_SEED
});

/// The `graphite-light` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static GRAPHITE_LIGHT: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.666667, s: 0.095238, l: 0.917647, a: 1.0 },
    color_surface: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_muted: Hsla { h: 0.666667, s: 0.125, l: 0.968627, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.666667, s: 0.09434, l: 0.896078, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.666667, s: 0.08, l: 0.098039, a: 0.058824 },
    color_surface_inset: Hsla { h: 0.666667, s: 0.103448, l: 0.943137, a: 1.0 },
    color_surface_raised: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.666667, s: 0.125, l: 0.968627, a: 1.0 },
    color_surface_column: Hsla { h: 0.666667, s: 0.125, l: 0.968627, a: 1.0 },
    color_content: Hsla { h: 0.666667, s: 0.08, l: 0.098039, a: 1.0 },
    color_content_secondary: Hsla { h: 0.666667, s: 0.061538, l: 0.254902, a: 1.0 },
    color_content_nav: Hsla { h: 0.666667, s: 0.061728, l: 0.317647, a: 1.0 },
    color_content_muted: Hsla { h: 0.666667, s: 0.043956, l: 0.356863, a: 1.0 },
    color_content_subtle: Hsla { h: 0.666667, s: 0.04, l: 0.558824, a: 1.0 },
    color_content_faint: Hsla { h: 0.666667, s: 0.045455, l: 0.654902, a: 1.0 },
    color_border: Hsla { h: 0.666667, s: 0.08, l: 0.098039, a: 0.129412 },
    color_border_strong: Hsla { h: 0.666667, s: 0.08, l: 0.098039, a: 0.168627 },
    color_border_muted: Hsla { h: 0.666667, s: 0.043956, l: 0.356863, a: 1.0 },
    color_on_accent: Hsla { h: 0.0, s: 0.0, l: 0.980392, a: 1.0 },
    frame_window_radius: px(16.0),
    frame_popover_radius: px(12.0),
    radius_small: px(7.0),
    radius_medium: px(8.0),
    radius_large: px(10.0),
    radius_big: px(12.0),
    progress_gradient_from: colors::GRAPHITE.shade(600),
    progress_gradient_to: colors::GRAPHITE.shade(400),
    color_primary: colors::GRAPHITE,
    ..STUDIO_LIGHT_SEED
});

/// The `graphite-dark` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static GRAPHITE_DARK: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.666667, s: 0.111111, l: 0.035294, a: 1.0 },
    color_surface: Hsla { h: 0.666667, s: 0.090909, l: 0.064706, a: 1.0 },
    color_surface_muted: Hsla { h: 0.666667, s: 0.069767, l: 0.084314, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.666667, s: 0.060241, l: 0.162745, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.070588 },
    color_surface_inset: Hsla { h: 0.666667, s: 0.081967, l: 0.119608, a: 1.0 },
    color_surface_raised: Hsla { h: 0.666667, s: 0.063492, l: 0.247059, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.666667, s: 0.069767, l: 0.084314, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.666667, s: 0.069767, l: 0.084314, a: 1.0 },
    color_surface_column: Hsla { h: 0.666667, s: 0.069767, l: 0.084314, a: 1.0 },
    color_content: Hsla { h: 0.666667, s: 0.1, l: 0.960784, a: 1.0 },
    color_content_secondary: Hsla { h: 0.666667, s: 0.069307, l: 0.801961, a: 1.0 },
    color_content_nav: Hsla { h: 0.666667, s: 0.048128, l: 0.633333, a: 1.0 },
    color_content_muted: Hsla { h: 0.666667, s: 0.048128, l: 0.633333, a: 1.0 },
    color_content_subtle: Hsla { h: 0.666667, s: 0.04, l: 0.558824, a: 1.0 },
    color_content_faint: Hsla { h: 0.666667, s: 0.040724, l: 0.433333, a: 1.0 },
    color_border: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.160784 },
    color_border_strong: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.2 },
    color_border_muted: Hsla { h: 0.666667, s: 0.058824, l: 0.233333, a: 1.0 },
    color_on_accent: Hsla { h: 0.666667, s: 0.073171, l: 0.080392, a: 1.0 },
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.4 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.54902 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.580392 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.6 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.701961 } }],
    focus_ring_alpha: 0.5,
    frame_window_radius: px(16.0),
    frame_popover_radius: px(12.0),
    radius_small: px(7.0),
    radius_medium: px(8.0),
    radius_large: px(10.0),
    radius_big: px(12.0),
    progress_gradient_from: colors::GRAPHITE_DARK.shade(500),
    progress_gradient_to: colors::GRAPHITE_DARK.shade(400),
    color_danger: colors::RED_DARK,
    color_primary: colors::GRAPHITE_DARK,
    color_success: colors::GREEN_DARK,
    color_warning: colors::AMBER_DARK,
    control_color_filled_surface_normal_shade: 500,
    control_color_filled_surface_hovered_shade: 600,
    control_color_filled_surface_pressed_shade: 600,
    control_color_outlined_border: ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(400), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(300), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(300), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        },
    control_color_outlined_content_normal_shade: 300,
    control_color_outlined_content_hovered_shade: 200,
    control_color_outlined_content_pressed_shade: 200,
    control_color_plain_content_normal_shade: 300,
    control_color_plain_content_hovered_shade: 200,
    control_color_plain_content_pressed_shade: 200,
    control_color_tinted_content_normal_shade: 300,
    control_color_tinted_content_hovered_shade: 200,
    control_color_tinted_content_pressed_shade: 200,
    ..STUDIO_LIGHT_SEED
});

/// The `ember-light` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static EMBER_LIGHT: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.106061, s: 0.34375, l: 0.87451, a: 1.0 },
    color_surface: Hsla { h: 0.111111, s: 1.0, l: 0.988235, a: 1.0 },
    color_surface_muted: Hsla { h: 0.107143, s: 0.4375, l: 0.937255, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.106667, s: 0.333333, l: 0.852941, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.078947, s: 0.358491, l: 0.103922, a: 0.058824 },
    color_surface_inset: Hsla { h: 0.105263, s: 0.422222, l: 0.911765, a: 1.0 },
    color_surface_raised: Hsla { h: 0.111111, s: 1.0, l: 0.988235, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.111111, s: 1.0, l: 0.988235, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.107143, s: 0.4375, l: 0.937255, a: 1.0 },
    color_surface_column: Hsla { h: 0.107143, s: 0.4375, l: 0.937255, a: 1.0 },
    color_content: Hsla { h: 0.078947, s: 0.358491, l: 0.103922, a: 1.0 },
    color_content_secondary: Hsla { h: 0.078125, s: 0.271186, l: 0.231373, a: 1.0 },
    color_content_nav: Hsla { h: 0.073529, s: 0.22973, l: 0.290196, a: 1.0 },
    color_content_muted: Hsla { h: 0.07619, s: 0.19774, l: 0.347059, a: 1.0 },
    color_content_subtle: Hsla { h: 0.071429, s: 0.117647, l: 0.533333, a: 1.0 },
    color_content_faint: Hsla { h: 0.083333, s: 0.150538, l: 0.635294, a: 1.0 },
    color_border: Hsla { h: 0.078947, s: 0.358491, l: 0.103922, a: 0.129412 },
    color_border_strong: Hsla { h: 0.078947, s: 0.358491, l: 0.103922, a: 0.168627 },
    color_border_muted: Hsla { h: 0.07619, s: 0.19774, l: 0.347059, a: 1.0 },
    color_on_accent: Hsla { h: 0.097222, s: 1.0, l: 0.976471, a: 1.0 },
    progress_gradient_from: colors::EMBER.shade(600),
    progress_gradient_to: colors::EMBER.shade(300),
    color_primary: colors::EMBER,
    ..STUDIO_LIGHT_SEED
});

/// The `ember-dark` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static EMBER_DARK: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.083333, s: 0.333333, l: 0.035294, a: 1.0 },
    color_surface: Hsla { h: 0.092593, s: 0.290323, l: 0.060784, a: 1.0 },
    color_surface_muted: Hsla { h: 0.074074, s: 0.219512, l: 0.080392, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.088235, s: 0.220779, l: 0.15098, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.070588 },
    color_surface_inset: Hsla { h: 0.089744, s: 0.22807, l: 0.111765, a: 1.0 },
    color_surface_raised: Hsla { h: 0.090278, s: 0.210526, l: 0.223529, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.074074, s: 0.219512, l: 0.080392, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.074074, s: 0.219512, l: 0.080392, a: 1.0 },
    color_surface_column: Hsla { h: 0.074074, s: 0.219512, l: 0.080392, a: 1.0 },
    color_content: Hsla { h: 0.101852, s: 0.5, l: 0.929412, a: 1.0 },
    color_content_secondary: Hsla { h: 0.090909, s: 0.282051, l: 0.770588, a: 1.0 },
    color_content_nav: Hsla { h: 0.081081, s: 0.180488, l: 0.598039, a: 1.0 },
    color_content_muted: Hsla { h: 0.081081, s: 0.180488, l: 0.598039, a: 1.0 },
    color_content_subtle: Hsla { h: 0.081197, s: 0.15415, l: 0.503922, a: 1.0 },
    color_content_faint: Hsla { h: 0.083333, s: 0.177083, l: 0.376471, a: 1.0 },
    color_border: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.160784 },
    color_border_strong: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.2 },
    color_border_muted: Hsla { h: 0.094203, s: 0.20354, l: 0.221569, a: 1.0 },
    color_on_accent: Hsla { h: 0.066667, s: 0.714286, l: 0.082353, a: 1.0 },
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.4 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.54902 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.580392 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.6 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.701961 } }],
    focus_ring_alpha: 0.5,
    progress_gradient_from: colors::EMBER_DARK.shade(500),
    progress_gradient_to: colors::EMBER_DARK.shade(300),
    color_danger: colors::RED_DARK,
    color_primary: colors::EMBER_DARK,
    color_success: colors::GREEN_DARK,
    color_warning: colors::AMBER_DARK,
    control_color_filled_surface_normal_shade: 500,
    control_color_filled_surface_hovered_shade: 600,
    control_color_filled_surface_pressed_shade: 600,
    control_color_outlined_border: ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(400), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(300), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(300), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        },
    control_color_outlined_content_normal_shade: 300,
    control_color_outlined_content_hovered_shade: 200,
    control_color_outlined_content_pressed_shade: 200,
    control_color_plain_content_normal_shade: 300,
    control_color_plain_content_hovered_shade: 200,
    control_color_plain_content_pressed_shade: 200,
    control_color_tinted_content_normal_shade: 300,
    control_color_tinted_content_hovered_shade: 200,
    control_color_tinted_content_pressed_shade: 200,
    ..STUDIO_LIGHT_SEED
});

/// The `nocturne-light` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static NOCTURNE_LIGHT: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.666667, s: 0.2, l: 0.921569, a: 1.0 },
    color_surface: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_muted: Hsla { h: 0.666667, s: 0.285714, l: 0.972549, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.666667, s: 0.208333, l: 0.905882, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.651515, s: 0.135802, l: 0.158824, a: 0.058824 },
    color_surface_inset: Hsla { h: 0.666667, s: 0.259259, l: 0.947059, a: 1.0 },
    color_surface_raised: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.666667, s: 0.285714, l: 0.972549, a: 1.0 },
    color_surface_column: Hsla { h: 0.666667, s: 0.285714, l: 0.972549, a: 1.0 },
    color_content: Hsla { h: 0.651515, s: 0.135802, l: 0.158824, a: 1.0 },
    color_content_secondary: Hsla { h: 0.647059, s: 0.131783, l: 0.252941, a: 1.0 },
    color_content_nav: Hsla { h: 0.650794, s: 0.121387, l: 0.339216, a: 1.0 },
    color_content_muted: Hsla { h: 0.647436, s: 0.116071, l: 0.439216, a: 1.0 },
    color_content_subtle: Hsla { h: 0.646667, s: 0.114155, l: 0.570588, a: 1.0 },
    color_content_faint: Hsla { h: 0.65, s: 0.138889, l: 0.717647, a: 1.0 },
    color_border: Hsla { h: 0.651515, s: 0.135802, l: 0.158824, a: 0.129412 },
    color_border_strong: Hsla { h: 0.651515, s: 0.135802, l: 0.158824, a: 0.168627 },
    color_border_muted: Hsla { h: 0.652778, s: 0.181818, l: 0.870588, a: 1.0 },
    color_on_accent: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 },
    frame_window_radius: px(16.0),
    frame_popover_radius: px(12.0),
    radius_small: px(7.0),
    radius_medium: px(8.0),
    radius_large: px(10.0),
    radius_big: px(12.0),
    progress_gradient_from: colors::NOCTURNE.shade(600),
    progress_gradient_to: colors::NOCTURNE.shade(300),
    color_primary: colors::NOCTURNE,
    ..STUDIO_LIGHT_SEED
});

/// The `nocturne-dark` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static NOCTURNE_DARK: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.65625, s: 0.333333, l: 0.094118, a: 1.0 },
    color_surface: Hsla { h: 0.645833, s: 0.266667, l: 0.117647, a: 1.0 },
    color_surface_muted: Hsla { h: 0.644444, s: 0.176471, l: 0.166667, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.641026, s: 0.121495, l: 0.209804, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.666667, s: 0.1, l: 0.921569, a: 0.070588 },
    color_surface_inset: Hsla { h: 0.625, s: 0.088889, l: 0.176471, a: 1.0 },
    color_surface_raised: Hsla { h: 0.630952, s: 0.1, l: 0.27451, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.644444, s: 0.176471, l: 0.166667, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.645833, s: 0.222222, l: 0.141176, a: 1.0 },
    color_surface_column: Hsla { h: 0.644444, s: 0.176471, l: 0.166667, a: 1.0 },
    color_content: Hsla { h: 0.666667, s: 0.1, l: 0.921569, a: 1.0 },
    color_content_secondary: Hsla { h: 0.636364, s: 0.297297, l: 0.854902, a: 1.0 },
    color_content_nav: Hsla { h: 0.638889, s: 0.184615, l: 0.745098, a: 1.0 },
    color_content_muted: Hsla { h: 0.638889, s: 0.125, l: 0.623529, a: 1.0 },
    color_content_subtle: Hsla { h: 0.637681, s: 0.090909, l: 0.503922, a: 1.0 },
    color_content_faint: Hsla { h: 0.631579, s: 0.096447, l: 0.386275, a: 1.0 },
    color_border: Hsla { h: 0.666667, s: 0.1, l: 0.921569, a: 0.160784 },
    color_border_strong: Hsla { h: 0.666667, s: 0.1, l: 0.921569, a: 0.2 },
    color_border_muted: Hsla { h: 0.630952, s: 0.1, l: 0.27451, a: 1.0 },
    color_on_accent: Hsla { h: 0.645833, s: 0.266667, l: 0.117647, a: 1.0 },
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.4 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.54902 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.580392 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.6 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.701961 } }],
    focus_ring_alpha: 0.5,
    frame_window_radius: px(16.0),
    frame_popover_radius: px(12.0),
    radius_small: px(7.0),
    radius_medium: px(8.0),
    radius_large: px(10.0),
    radius_big: px(12.0),
    progress_gradient_from: colors::NOCTURNE.shade(500),
    progress_gradient_to: colors::NOCTURNE.shade(300),
    color_danger: colors::RED_DARK,
    color_primary: colors::NOCTURNE,
    color_success: colors::GREEN_DARK,
    color_warning: colors::AMBER_DARK,
    control_color_filled_surface_normal_shade: 500,
    control_color_filled_surface_hovered_shade: 600,
    control_color_filled_surface_pressed_shade: 600,
    control_color_outlined_border: ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(400), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(300), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(300), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        },
    control_color_outlined_content_normal_shade: 300,
    control_color_outlined_content_hovered_shade: 200,
    control_color_outlined_content_pressed_shade: 200,
    control_color_plain_content_normal_shade: 300,
    control_color_plain_content_hovered_shade: 200,
    control_color_plain_content_pressed_shade: 200,
    control_color_tinted_content_normal_shade: 300,
    control_color_tinted_content_hovered_shade: 200,
    control_color_tinted_content_pressed_shade: 200,
    ..STUDIO_LIGHT_SEED
});

/// The `studio-dark` counterpart of [`STUDIO_LIGHT`].
///
/// Every token that moves under this theme is named here; the rest fall
/// through to [`STUDIO_LIGHT_SEED`], and everything derived from them
/// follows on its own.
pub static STUDIO_DARK: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { h: 0.638889, s: 0.230769, l: 0.05098, a: 1.0 },
    color_surface: Hsla { h: 0.619048, s: 0.212121, l: 0.064706, a: 1.0 },
    color_surface_muted: Hsla { h: 0.625, s: 0.181818, l: 0.086275, a: 1.0 },
    color_surface_sunken: Hsla { h: 0.619048, s: 0.155556, l: 0.176471, a: 1.0 },
    color_surface_subtle: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.070588 },
    color_surface_inset: Hsla { h: 0.621212, s: 0.169231, l: 0.127451, a: 1.0 },
    color_surface_raised: Hsla { h: 0.622807, s: 0.140741, l: 0.264706, a: 1.0 },
    color_surface_overlay: Hsla { h: 0.625, s: 0.181818, l: 0.086275, a: 1.0 },
    color_surface_chrome: Hsla { h: 0.625, s: 0.181818, l: 0.086275, a: 1.0 },
    color_surface_column: Hsla { h: 0.619048, s: 0.259259, l: 0.052941, a: 1.0 },
    color_content: Hsla { h: 0.645833, s: 0.444444, l: 0.964706, a: 1.0 },
    color_content_secondary: Hsla { h: 0.633333, s: 0.263158, l: 0.813725, a: 1.0 },
    color_content_nav: Hsla { h: 0.631313, s: 0.195266, l: 0.668627, a: 1.0 },
    color_content_muted: Hsla { h: 0.631313, s: 0.195266, l: 0.668627, a: 1.0 },
    color_content_subtle: Hsla { h: 0.630631, s: 0.189744, l: 0.617647, a: 1.0 },
    color_content_faint: Hsla { h: 0.640351, s: 0.152, l: 0.490196, a: 1.0 },
    color_border: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.058824 },
    color_border_strong: Hsla { h: 0.0, s: 0.0, l: 1.0, a: 0.090196 },
    color_border_muted: Hsla { h: 0.623188, s: 0.175573, l: 0.256863, a: 1.0 },
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.4 } }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.45098 } }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.54902 } }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.580392 } }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.6 } }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.701961 } }],
    focus_ring_alpha: 0.5,
    progress_gradient_from: colors::BRAND.shade(500),
    progress_gradient_to: colors::BRAND.shade(300),
    color_danger: colors::RED_DARK,
    color_success: colors::GREEN_DARK,
    color_warning: colors::AMBER_DARK,
    control_color_filled_surface_normal_shade: 500,
    control_color_filled_surface_hovered_shade: 600,
    control_color_filled_surface_pressed_shade: 600,
    control_color_outlined_border: ColorDescriptor {
            normal: ColorRecipe { color: None, shade: Some(400), alpha: Some(1.0) },
            hovered: ColorRecipe { color: None, shade: Some(300), alpha: Some(1.0) },
            pressed: ColorRecipe { color: None, shade: Some(300), alpha: Some(0.9) },
            disabled: ColorRecipe { color: Some(colors::TRANSPARENT), shade: None, alpha: None },
        },
    control_color_outlined_content_normal_shade: 300,
    control_color_outlined_content_hovered_shade: 200,
    control_color_outlined_content_pressed_shade: 200,
    control_color_plain_content_normal_shade: 300,
    control_color_plain_content_hovered_shade: 200,
    control_color_plain_content_pressed_shade: 200,
    control_color_tinted_content_normal_shade: 300,
    control_color_tinted_content_hovered_shade: 200,
    control_color_tinted_content_pressed_shade: 200,
    ..STUDIO_LIGHT_SEED
});
