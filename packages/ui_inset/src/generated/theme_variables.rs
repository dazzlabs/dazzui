#![cfg_attr(rustfmt, rustfmt_skip)]
//! The complete token set for one theme.
//!
//! DO NOT EDIT - This file is auto-generated

use inset_embedder::{FontWeight, Color};
use crate::foundation::px;

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
    pub color_canvas: Color,
    pub color_surface: Color,
    pub color_surface_muted: Color,
    pub color_surface_sunken: Color,
    pub color_surface_subtle: Color,
    pub color_surface_inset: Color,
    pub color_surface_raised: Color,
    pub color_surface_overlay: Color,
    pub color_surface_chrome: Color,
    pub color_surface_column: Color,
    pub color_content: Color,
    pub color_content_secondary: Color,
    pub color_content_nav: Color,
    pub color_content_muted: Color,
    pub color_content_subtle: Color,
    pub color_content_faint: Color,
    pub color_border: Color,
    pub color_border_strong: Color,
    pub color_border_muted: Color,
    pub color_on_accent: Color,
    pub control_pressed_alpha: f64,
    pub control_container_radius: f64,
    pub control_tiny_size: f64,
    pub control_small_size: f64,
    pub control_medium_size: f64,
    pub control_large_size: f64,
    pub shadow_2xs: &'static [TokenShadow],
    pub shadow_xs: &'static [TokenShadow],
    pub shadow_sm: &'static [TokenShadow],
    pub shadow_md: &'static [TokenShadow],
    pub shadow_lg: &'static [TokenShadow],
    pub shadow_xl: &'static [TokenShadow],
    pub shadow_2xl: &'static [TokenShadow],
    pub focus_width: f64,
    pub focus_offset: f64,
    pub focus_glow_shade: u16,
    pub focus_glow_alpha: f64,
    pub focus_ring_shade: u16,
    pub focus_ring_alpha: f64,
    pub frame_window_radius: f64,
    pub frame_popover_radius: f64,
    pub frame_titlebar_size: f64,
    pub frame_sidebar_width: f64,
    pub frame_sidebar_icon_width: f64,
    pub frame_rail_width: f64,
    pub frame_aside_width: f64,
    pub frame_nav_gap: f64,
    pub motion_duration: f64,
    pub motion_easing: BezierCurve,
    pub radius_none: f64,
    pub radius_tiny: f64,
    pub radius_small: f64,
    pub radius_medium: f64,
    pub radius_large: f64,
    pub radius_big: f64,
    pub radius_full: f64,
    pub spacing_px: f64,
    pub spacing_0: f64,
    pub spacing_0_5: f64,
    pub spacing_1: f64,
    pub spacing_1_5: f64,
    pub spacing_2: f64,
    pub spacing_2_5: f64,
    pub spacing_3: f64,
    pub spacing_3_5: f64,
    pub spacing_4: f64,
    pub spacing_5: f64,
    pub spacing_6: f64,
    pub spacing_7: f64,
    pub spacing_8: f64,
    pub spacing_9: f64,
    pub spacing_10: f64,
    pub spacing_11: f64,
    pub spacing_12: f64,
    pub spacing_14: f64,
    pub spacing_16: f64,
    pub spacing_20: f64,
    pub stroke_hairline: f64,
    pub stroke_control: f64,
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
    pub wash_surface: f64,
    pub wash_edge: f64,
    pub checkbox_radius: f64,
    pub dialog_width: f64,
    pub dialog_scrim_alpha: f64,
    pub drawer_size: f64,
    pub menu_min_width: f64,
    pub menu_item_padding: f64,
    pub preferences_width: f64,
    pub preview_card_width: f64,
    pub progress_gradient_from: Color,
    pub progress_gradient_to: Color,
    pub segmented_control_inset: f64,
    pub shortcut_recorder_width: f64,
    pub switch_medium_width: f64,
    pub switch_medium_height: f64,
    pub switch_medium_thumb: f64,
    pub toast_max_width: f64,
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
    color_canvas: Color::from_argb(255, 238, 236, 246),
    color_surface: Color::from_argb(255, 255, 255, 255),
    color_surface_muted: Color::from_argb(255, 247, 247, 250),
    color_surface_sunken: Color::from_argb(255, 227, 227, 236),
    color_surface_subtle: Color::from_argb(15, 20, 22, 42),
    color_surface_inset: Color::from_argb(255, 240, 240, 245),
    color_surface_raised: Color::from_argb(255, 255, 255, 255),
    color_surface_overlay: Color::from_argb(255, 255, 255, 255),
    color_surface_chrome: Color::from_argb(255, 247, 247, 250),
    color_surface_column: Color::from_argb(255, 250, 250, 252),
    color_content: Color::from_argb(255, 18, 20, 42),
    color_content_secondary: Color::from_argb(255, 60, 64, 92),
    color_content_nav: Color::from_argb(255, 74, 79, 107),
    color_content_muted: Color::from_argb(255, 86, 91, 120),
    color_content_subtle: Color::from_argb(255, 140, 146, 170),
    color_content_faint: Color::from_argb(255, 162, 167, 189),
    color_border: Color::from_argb(18, 20, 22, 42),
    color_border_strong: Color::from_argb(23, 20, 22, 42),
    color_border_muted: Color::from_argb(255, 95, 100, 120),
    color_on_accent: Color::from_argb(255, 255, 255, 255),
    control_pressed_alpha: 0.9,
    control_container_radius: px(10.0),
    control_tiny_size: px(24.0),
    control_small_size: px(26.0),
    control_medium_size: px(28.0),
    control_large_size: px(32.0),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(26, 0, 0, 0) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(31, 0, 0, 0) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(12.0), spread: px(0.0), color: Color::from_argb(36, 0, 0, 0) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(8.0), blur: px(24.0), spread: px(0.0), color: Color::from_argb(51, 0, 0, 0) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(30.0), spread: px(0.0), color: Color::from_argb(56, 0, 0, 0) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(12.0), blur: px(32.0), spread: px(0.0), color: Color::from_argb(61, 0, 0, 0) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(56.0), spread: px(0.0), color: Color::from_argb(71, 0, 0, 0) }],
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
    headline_small: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(20.0), weight: FontWeight::W600, line_height: px(26.0) },
    headline_medium: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(24.0), weight: FontWeight::W600, line_height: px(30.0) },
    headline_large: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(28.0), weight: FontWeight::W600, line_height: px(34.0) },
    title_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::W600, line_height: px(20.0) },
    title_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(15.0), weight: FontWeight::W600, line_height: px(22.0) },
    title_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(17.0), weight: FontWeight::W600, line_height: px(22.0) },
    body_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::W400, line_height: px(18.0) },
    body_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::W400, line_height: px(20.0) },
    body_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(15.0), weight: FontWeight::W400, line_height: px(22.0) },
    label_quiet: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::W500, line_height: px(12.0) },
    label_strong: TypeStyle { family: &["-apple-system", "SF Pro Display", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::W700, line_height: px(12.0) },
    label_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(11.0), weight: FontWeight::W600, line_height: px(11.0) },
    label_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::W600, line_height: px(12.0) },
    label_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::W600, line_height: px(13.0) },
    caption_small: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(11.0), weight: FontWeight::W400, line_height: px(14.0) },
    caption_medium: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(12.0), weight: FontWeight::W400, line_height: px(18.0) },
    caption_large: TypeStyle { family: &["-apple-system", "SF Pro Text", "system-ui", "PingFang SC", "sans-serif"], size: px(13.0), weight: FontWeight::W400, line_height: px(20.0) },
    wash_surface: 0.06,
    wash_edge: 0.2,
    checkbox_radius: px(5.0),
    dialog_width: px(440.0),
    dialog_scrim_alpha: 0.25,
    drawer_size: px(352.0),
    menu_min_width: px(176.0),
    menu_item_padding: px(7.0),
    preferences_width: px(480.0),
    preview_card_width: px(320.0),
    progress_gradient_from: colors::BRAND.shade(600),
    progress_gradient_to: Color::from_argb(255, 160, 140, 255),
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
    pub color_canvas: Color,
    pub color_surface: Color,
    pub color_surface_muted: Color,
    pub color_surface_sunken: Color,
    pub color_surface_subtle: Color,
    pub color_surface_inset: Color,
    pub color_surface_raised: Color,
    pub color_surface_overlay: Color,
    pub color_surface_chrome: Color,
    pub color_surface_column: Color,
    pub color_content: Color,
    pub color_content_secondary: Color,
    pub color_content_nav: Color,
    pub color_content_muted: Color,
    pub color_content_subtle: Color,
    pub color_content_faint: Color,
    pub color_border: Color,
    pub color_border_strong: Color,
    pub color_border_muted: Color,
    pub color_on_accent: Color,
    pub control_pressed_alpha: f64,
    pub control_container_radius: f64,
    pub control_tiny_size: f64,
    pub control_small_size: f64,
    pub control_medium_size: f64,
    pub control_large_size: f64,
    pub shadow_2xs: &'static [TokenShadow],
    pub shadow_xs: &'static [TokenShadow],
    pub shadow_sm: &'static [TokenShadow],
    pub shadow_md: &'static [TokenShadow],
    pub shadow_lg: &'static [TokenShadow],
    pub shadow_xl: &'static [TokenShadow],
    pub shadow_2xl: &'static [TokenShadow],
    pub focus_width: f64,
    pub focus_offset: f64,
    pub focus_glow_shade: u16,
    pub focus_glow_alpha: f64,
    pub focus_ring_shade: u16,
    pub focus_ring_alpha: f64,
    pub frame_window_radius: f64,
    pub frame_popover_radius: f64,
    pub frame_titlebar_size: f64,
    pub frame_sidebar_width: f64,
    pub frame_sidebar_icon_width: f64,
    pub frame_rail_width: f64,
    pub frame_aside_width: f64,
    pub frame_nav_gap: f64,
    pub motion_duration: f64,
    pub motion_easing: BezierCurve,
    pub radius_none: f64,
    pub radius_tiny: f64,
    pub radius_small: f64,
    pub radius_medium: f64,
    pub radius_large: f64,
    pub radius_big: f64,
    pub radius_full: f64,
    pub spacing_px: f64,
    pub spacing_0: f64,
    pub spacing_0_5: f64,
    pub spacing_1: f64,
    pub spacing_1_5: f64,
    pub spacing_2: f64,
    pub spacing_2_5: f64,
    pub spacing_3: f64,
    pub spacing_3_5: f64,
    pub spacing_4: f64,
    pub spacing_5: f64,
    pub spacing_6: f64,
    pub spacing_7: f64,
    pub spacing_8: f64,
    pub spacing_9: f64,
    pub spacing_10: f64,
    pub spacing_11: f64,
    pub spacing_12: f64,
    pub spacing_14: f64,
    pub spacing_16: f64,
    pub spacing_20: f64,
    pub stroke_hairline: f64,
    pub stroke_control: f64,
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
    pub wash_surface: f64,
    pub wash_edge: f64,
    pub checkbox_radius: f64,
    pub dialog_width: f64,
    pub dialog_scrim_alpha: f64,
    pub drawer_size: f64,
    pub menu_min_width: f64,
    pub menu_item_padding: f64,
    pub preferences_width: f64,
    pub preview_card_width: f64,
    pub progress_gradient_from: Color,
    pub progress_gradient_to: Color,
    pub segmented_control_inset: f64,
    pub shortcut_recorder_width: f64,
    pub switch_medium_width: f64,
    pub switch_medium_height: f64,
    pub switch_medium_thumb: f64,
    pub toast_max_width: f64,
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
    pub control_tiny_gap: f64,
    pub control_tiny_padding_block: f64,
    pub control_tiny_padding_inline: f64,
    pub control_tiny_radius: f64,
    pub control_tiny_content: TypeStyle,
    pub control_small_gap: f64,
    pub control_small_padding_block: f64,
    pub control_small_padding_inline: f64,
    pub control_small_radius: f64,
    pub control_small_content: TypeStyle,
    pub control_medium_gap: f64,
    pub control_medium_padding_block: f64,
    pub control_medium_padding_inline: f64,
    pub control_medium_radius: f64,
    pub control_medium_content: TypeStyle,
    pub control_large_gap: f64,
    pub control_large_padding_block: f64,
    pub control_large_padding_inline: f64,
    pub control_large_radius: f64,
    pub control_large_content: TypeStyle,
    pub checkbox_thickness: f64,
    pub checkbox_small_box: f64,
    pub checkbox_medium_box: f64,
    pub checkbox_large_box: f64,
    pub dialog_scrim_color: Color,
    pub preferences_row_padding: f64,
    pub preferences_row_gap: f64,
    pub preferences_heading_gap: f64,
    pub preferences_footer_gap: f64,
    pub preferences_title_gap: f64,
    pub preferences_section_gap: f64,
    pub preferences_group_gap: f64,
    pub radio_thickness: f64,
    pub radio_small_box: f64,
    pub radio_small_dot: f64,
    pub radio_medium_box: f64,
    pub radio_medium_dot: f64,
    pub radio_large_box: f64,
    pub radio_large_dot: f64,
    pub slider_small_track: f64,
    pub slider_small_thumb: f64,
    pub slider_medium_track: f64,
    pub slider_medium_thumb: f64,
    pub slider_large_track: f64,
    pub slider_large_thumb: f64,
    pub switch_small_width: f64,
    pub switch_small_height: f64,
    pub switch_small_thumb: f64,
    pub switch_large_width: f64,
    pub switch_large_height: f64,
    pub switch_large_thumb: f64,
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
        let control_container_radius = seed.control_container_radius;
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
        let drawer_size = seed.drawer_size;
        let menu_min_width = seed.menu_min_width;
        let menu_item_padding = seed.menu_item_padding;
        let preferences_width = seed.preferences_width;
        let preview_card_width = seed.preview_card_width;
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
        let preferences_row_padding = spacing_1_5;
        let preferences_row_gap = spacing_1;
        let preferences_heading_gap = spacing_2;
        let preferences_footer_gap = spacing_1;
        let preferences_title_gap = spacing_4;
        let preferences_section_gap = spacing_6;
        let preferences_group_gap = spacing_10;
        let radio_thickness = stroke_control;
        let radio_small_box = spacing_3_5;
        let radio_small_dot = spacing_1_5;
        let radio_medium_box = spacing_4;
        let radio_medium_dot = spacing_2;
        let radio_large_box = spacing_5;
        let radio_large_dot = spacing_2_5;
        let slider_small_track = spacing_1;
        let slider_small_thumb = spacing_3_5;
        let slider_medium_track = spacing_1_5;
        let slider_medium_thumb = spacing_4;
        let slider_large_track = spacing_2;
        let slider_large_thumb = spacing_5;
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
            control_container_radius,
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
            drawer_size,
            menu_min_width,
            menu_item_padding,
            preferences_width,
            preview_card_width,
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
            preferences_row_padding,
            preferences_row_gap,
            preferences_heading_gap,
            preferences_footer_gap,
            preferences_title_gap,
            preferences_section_gap,
            preferences_group_gap,
            radio_thickness,
            radio_small_box,
            radio_small_dot,
            radio_medium_box,
            radio_medium_dot,
            radio_large_box,
            radio_large_dot,
            slider_small_track,
            slider_small_thumb,
            slider_medium_track,
            slider_medium_thumb,
            slider_large_track,
            slider_large_thumb,
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
    color_canvas: Color::from_argb(255, 230, 234, 222),
    color_surface: Color::from_argb(255, 251, 250, 247),
    color_surface_muted: Color::from_argb(255, 255, 255, 255),
    color_surface_sunken: Color::from_argb(255, 229, 227, 219),
    color_surface_subtle: Color::from_argb(15, 17, 28, 46),
    color_surface_inset: Color::from_argb(255, 240, 239, 233),
    color_surface_raised: Color::from_argb(255, 255, 255, 255),
    color_surface_overlay: Color::from_argb(255, 251, 250, 247),
    color_surface_chrome: Color::from_argb(255, 255, 255, 255),
    color_surface_column: Color::from_argb(255, 244, 243, 238),
    color_content: Color::from_argb(255, 17, 28, 46),
    color_content_secondary: Color::from_argb(199, 17, 28, 46),
    color_content_nav: Color::from_argb(179, 17, 28, 46),
    color_content_muted: Color::from_argb(168, 17, 28, 46),
    color_content_subtle: Color::from_argb(115, 17, 28, 46),
    color_content_faint: Color::from_argb(97, 17, 28, 46),
    color_border: Color::from_argb(20, 17, 28, 46),
    color_border_strong: Color::from_argb(23, 17, 28, 46),
    color_border_muted: Color::from_argb(64, 17, 28, 46),
    color_on_accent: Color::from_argb(255, 214, 255, 63),
    control_container_radius: px(14.0),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(31, 17, 28, 46) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(36, 17, 28, 46) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(12.0), spread: px(0.0), color: Color::from_argb(36, 17, 28, 46) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(8.0), blur: px(24.0), spread: px(0.0), color: Color::from_argb(46, 17, 28, 46) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(30.0), spread: px(0.0), color: Color::from_argb(46, 17, 28, 46) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(12.0), blur: px(32.0), spread: px(0.0), color: Color::from_argb(51, 17, 28, 46) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(56.0), spread: px(0.0), color: Color::from_argb(61, 17, 28, 46) }],
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
    color_canvas: Color::from_argb(255, 6, 11, 18),
    color_surface: Color::from_argb(255, 12, 20, 30),
    color_surface_muted: Color::from_argb(255, 20, 29, 41),
    color_surface_sunken: Color::from_argb(255, 28, 39, 52),
    color_surface_subtle: Color::from_argb(18, 242, 244, 239),
    color_surface_inset: Color::from_argb(255, 22, 32, 44),
    color_surface_raised: Color::from_argb(255, 42, 54, 68),
    color_surface_overlay: Color::from_argb(255, 17, 26, 38),
    color_surface_chrome: Color::from_argb(255, 17, 26, 38),
    color_surface_column: Color::from_argb(255, 10, 17, 26),
    color_content: Color::from_argb(255, 242, 244, 239),
    color_content_secondary: Color::from_argb(199, 242, 244, 239),
    color_content_nav: Color::from_argb(179, 242, 244, 239),
    color_content_muted: Color::from_argb(168, 242, 244, 239),
    color_content_subtle: Color::from_argb(115, 242, 244, 239),
    color_content_faint: Color::from_argb(97, 242, 244, 239),
    color_border: Color::from_argb(20, 242, 244, 239),
    color_border_strong: Color::from_argb(23, 242, 244, 239),
    color_border_muted: Color::from_argb(255, 53, 65, 79),
    color_on_accent: Color::from_argb(255, 17, 28, 46),
    control_container_radius: px(14.0),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(102, 0, 0, 0) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Color::from_argb(140, 0, 0, 0) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Color::from_argb(148, 0, 0, 0) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Color::from_argb(153, 0, 0, 0) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Color::from_argb(179, 0, 0, 0) }],
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
    color_canvas: Color::from_argb(255, 229, 237, 240),
    color_surface: Color::from_argb(255, 255, 255, 255),
    color_surface_muted: Color::from_argb(255, 242, 246, 248),
    color_surface_sunken: Color::from_argb(255, 219, 230, 234),
    color_surface_subtle: Color::from_argb(15, 14, 31, 38),
    color_surface_inset: Color::from_argb(255, 236, 242, 244),
    color_surface_raised: Color::from_argb(255, 255, 255, 255),
    color_surface_overlay: Color::from_argb(255, 255, 255, 255),
    color_surface_chrome: Color::from_argb(255, 242, 246, 248),
    color_surface_column: Color::from_argb(255, 242, 246, 248),
    color_content: Color::from_argb(255, 14, 31, 38),
    color_content_secondary: Color::from_argb(255, 47, 76, 88),
    color_content_nav: Color::from_argb(255, 58, 91, 104),
    color_content_muted: Color::from_argb(255, 69, 101, 115),
    color_content_subtle: Color::from_argb(255, 119, 148, 160),
    color_content_faint: Color::from_argb(255, 154, 177, 186),
    color_border: Color::from_argb(33, 14, 31, 38),
    color_border_strong: Color::from_argb(43, 14, 31, 38),
    color_border_muted: Color::from_argb(255, 69, 101, 115),
    color_on_accent: Color::from_argb(255, 255, 255, 255),
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
    color_canvas: Color::from_argb(255, 7, 13, 17),
    color_surface: Color::from_argb(255, 13, 21, 26),
    color_surface_muted: Color::from_argb(255, 17, 26, 32),
    color_surface_sunken: Color::from_argb(255, 35, 51, 61),
    color_surface_subtle: Color::from_argb(18, 255, 255, 255),
    color_surface_inset: Color::from_argb(255, 24, 38, 46),
    color_surface_raised: Color::from_argb(255, 55, 75, 87),
    color_surface_overlay: Color::from_argb(255, 17, 26, 32),
    color_surface_chrome: Color::from_argb(255, 17, 26, 32),
    color_surface_column: Color::from_argb(255, 17, 26, 32),
    color_content: Color::from_argb(255, 234, 243, 247),
    color_content_secondary: Color::from_argb(255, 188, 208, 217),
    color_content_nav: Color::from_argb(255, 147, 170, 181),
    color_content_muted: Color::from_argb(255, 147, 170, 181),
    color_content_subtle: Color::from_argb(255, 127, 153, 164),
    color_content_faint: Color::from_argb(255, 98, 128, 141),
    color_border: Color::from_argb(41, 255, 255, 255),
    color_border_strong: Color::from_argb(51, 255, 255, 255),
    color_border_muted: Color::from_argb(255, 50, 69, 79),
    color_on_accent: Color::from_argb(255, 4, 23, 29),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(102, 0, 0, 0) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Color::from_argb(140, 0, 0, 0) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Color::from_argb(148, 0, 0, 0) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Color::from_argb(153, 0, 0, 0) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Color::from_argb(179, 0, 0, 0) }],
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
    color_canvas: Color::from_argb(255, 232, 232, 236),
    color_surface: Color::from_argb(255, 255, 255, 255),
    color_surface_muted: Color::from_argb(255, 246, 246, 248),
    color_surface_sunken: Color::from_argb(255, 226, 226, 231),
    color_surface_subtle: Color::from_argb(15, 23, 23, 27),
    color_surface_inset: Color::from_argb(255, 239, 239, 242),
    color_surface_raised: Color::from_argb(255, 255, 255, 255),
    color_surface_overlay: Color::from_argb(255, 255, 255, 255),
    color_surface_chrome: Color::from_argb(255, 246, 246, 248),
    color_surface_column: Color::from_argb(255, 246, 246, 248),
    color_content: Color::from_argb(255, 23, 23, 27),
    color_content_secondary: Color::from_argb(255, 61, 61, 69),
    color_content_nav: Color::from_argb(255, 76, 76, 86),
    color_content_muted: Color::from_argb(255, 87, 87, 95),
    color_content_subtle: Color::from_argb(255, 138, 138, 147),
    color_content_faint: Color::from_argb(255, 163, 163, 171),
    color_border: Color::from_argb(33, 23, 23, 27),
    color_border_strong: Color::from_argb(43, 23, 23, 27),
    color_border_muted: Color::from_argb(255, 87, 87, 95),
    color_on_accent: Color::from_argb(255, 250, 250, 250),
    control_container_radius: px(8.0),
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
    color_canvas: Color::from_argb(255, 8, 8, 10),
    color_surface: Color::from_argb(255, 15, 15, 18),
    color_surface_muted: Color::from_argb(255, 20, 20, 23),
    color_surface_sunken: Color::from_argb(255, 39, 39, 44),
    color_surface_subtle: Color::from_argb(18, 255, 255, 255),
    color_surface_inset: Color::from_argb(255, 28, 28, 33),
    color_surface_raised: Color::from_argb(255, 59, 59, 67),
    color_surface_overlay: Color::from_argb(255, 20, 20, 23),
    color_surface_chrome: Color::from_argb(255, 20, 20, 23),
    color_surface_column: Color::from_argb(255, 20, 20, 23),
    color_content: Color::from_argb(255, 244, 244, 246),
    color_content_secondary: Color::from_argb(255, 201, 201, 208),
    color_content_nav: Color::from_argb(255, 157, 157, 166),
    color_content_muted: Color::from_argb(255, 157, 157, 166),
    color_content_subtle: Color::from_argb(255, 138, 138, 147),
    color_content_faint: Color::from_argb(255, 106, 106, 115),
    color_border: Color::from_argb(41, 255, 255, 255),
    color_border_strong: Color::from_argb(51, 255, 255, 255),
    color_border_muted: Color::from_argb(255, 56, 56, 63),
    color_on_accent: Color::from_argb(255, 19, 19, 22),
    control_container_radius: px(8.0),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(102, 0, 0, 0) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Color::from_argb(140, 0, 0, 0) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Color::from_argb(148, 0, 0, 0) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Color::from_argb(153, 0, 0, 0) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Color::from_argb(179, 0, 0, 0) }],
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
    color_canvas: Color::from_argb(255, 234, 226, 212),
    color_surface: Color::from_argb(255, 255, 253, 249),
    color_surface_muted: Color::from_argb(255, 246, 241, 232),
    color_surface_sunken: Color::from_argb(255, 230, 221, 205),
    color_surface_subtle: Color::from_argb(15, 36, 26, 17),
    color_surface_inset: Color::from_argb(255, 242, 235, 223),
    color_surface_raised: Color::from_argb(255, 255, 253, 249),
    color_surface_overlay: Color::from_argb(255, 255, 253, 249),
    color_surface_chrome: Color::from_argb(255, 246, 241, 232),
    color_surface_column: Color::from_argb(255, 246, 241, 232),
    color_content: Color::from_argb(255, 36, 26, 17),
    color_content_secondary: Color::from_argb(255, 75, 58, 43),
    color_content_nav: Color::from_argb(255, 91, 72, 57),
    color_content_muted: Color::from_argb(255, 106, 87, 71),
    color_content_subtle: Color::from_argb(255, 150, 134, 122),
    color_content_faint: Color::from_argb(255, 176, 162, 148),
    color_border: Color::from_argb(33, 36, 26, 17),
    color_border_strong: Color::from_argb(43, 36, 26, 17),
    color_border_muted: Color::from_argb(255, 106, 87, 71),
    color_on_accent: Color::from_argb(255, 255, 250, 243),
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
    color_canvas: Color::from_argb(255, 12, 9, 6),
    color_surface: Color::from_argb(255, 20, 16, 11),
    color_surface_muted: Color::from_argb(255, 25, 20, 16),
    color_surface_sunken: Color::from_argb(255, 47, 39, 30),
    color_surface_subtle: Color::from_argb(18, 255, 255, 255),
    color_surface_inset: Color::from_argb(255, 35, 29, 22),
    color_surface_raised: Color::from_argb(255, 69, 58, 45),
    color_surface_overlay: Color::from_argb(255, 25, 20, 16),
    color_surface_chrome: Color::from_argb(255, 25, 20, 16),
    color_surface_column: Color::from_argb(255, 25, 20, 16),
    color_content: Color::from_argb(255, 246, 239, 228),
    color_content_secondary: Color::from_argb(255, 213, 198, 180),
    color_content_nav: Color::from_argb(255, 171, 152, 134),
    color_content_muted: Color::from_argb(255, 171, 152, 134),
    color_content_subtle: Color::from_argb(255, 148, 128, 109),
    color_content_faint: Color::from_argb(255, 113, 96, 79),
    color_border: Color::from_argb(41, 255, 255, 255),
    color_border_strong: Color::from_argb(51, 255, 255, 255),
    color_border_muted: Color::from_argb(255, 68, 58, 45),
    color_on_accent: Color::from_argb(255, 36, 18, 6),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(102, 0, 0, 0) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Color::from_argb(140, 0, 0, 0) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Color::from_argb(148, 0, 0, 0) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Color::from_argb(153, 0, 0, 0) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Color::from_argb(179, 0, 0, 0) }],
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
    color_canvas: Color::from_argb(255, 231, 231, 239),
    color_surface: Color::from_argb(255, 255, 255, 255),
    color_surface_muted: Color::from_argb(255, 246, 246, 250),
    color_surface_sunken: Color::from_argb(255, 226, 226, 236),
    color_surface_subtle: Color::from_argb(15, 35, 36, 46),
    color_surface_inset: Color::from_argb(255, 238, 238, 245),
    color_surface_raised: Color::from_argb(255, 255, 255, 255),
    color_surface_overlay: Color::from_argb(255, 255, 255, 255),
    color_surface_chrome: Color::from_argb(255, 246, 246, 250),
    color_surface_column: Color::from_argb(255, 246, 246, 250),
    color_content: Color::from_argb(255, 35, 36, 46),
    color_content_secondary: Color::from_argb(255, 56, 58, 73),
    color_content_nav: Color::from_argb(255, 76, 78, 97),
    color_content_muted: Color::from_argb(255, 99, 102, 125),
    color_content_subtle: Color::from_argb(255, 133, 136, 158),
    color_content_faint: Color::from_argb(255, 173, 175, 193),
    color_border: Color::from_argb(33, 35, 36, 46),
    color_border_strong: Color::from_argb(43, 35, 36, 46),
    color_border_muted: Color::from_argb(255, 216, 217, 228),
    color_on_accent: Color::from_argb(255, 255, 255, 255),
    control_container_radius: px(8.0),
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
    color_canvas: Color::from_argb(255, 16, 17, 32),
    color_surface: Color::from_argb(255, 22, 24, 38),
    color_surface_muted: Color::from_argb(255, 35, 37, 50),
    color_surface_sunken: Color::from_argb(255, 47, 49, 60),
    color_surface_subtle: Color::from_argb(18, 233, 233, 237),
    color_surface_inset: Color::from_argb(255, 41, 43, 49),
    color_surface_raised: Color::from_argb(255, 63, 66, 77),
    color_surface_overlay: Color::from_argb(255, 35, 37, 50),
    color_surface_chrome: Color::from_argb(255, 28, 30, 44),
    color_surface_column: Color::from_argb(255, 35, 37, 50),
    color_content: Color::from_argb(255, 233, 233, 237),
    color_content_secondary: Color::from_argb(255, 207, 211, 229),
    color_content_nav: Color::from_argb(255, 178, 182, 202),
    color_content_muted: Color::from_argb(255, 147, 151, 171),
    color_content_subtle: Color::from_argb(255, 117, 121, 140),
    color_content_faint: Color::from_argb(255, 89, 93, 108),
    color_border: Color::from_argb(41, 233, 233, 237),
    color_border_strong: Color::from_argb(51, 233, 233, 237),
    color_border_muted: Color::from_argb(255, 63, 66, 77),
    color_on_accent: Color::from_argb(255, 22, 24, 38),
    control_container_radius: px(8.0),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(102, 0, 0, 0) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Color::from_argb(140, 0, 0, 0) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Color::from_argb(148, 0, 0, 0) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Color::from_argb(153, 0, 0, 0) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Color::from_argb(179, 0, 0, 0) }],
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
    color_canvas: Color::from_argb(255, 10, 11, 16),
    color_surface: Color::from_argb(255, 13, 15, 20),
    color_surface_muted: Color::from_argb(255, 18, 20, 26),
    color_surface_sunken: Color::from_argb(255, 38, 42, 52),
    color_surface_subtle: Color::from_argb(18, 255, 255, 255),
    color_surface_inset: Color::from_argb(255, 27, 30, 38),
    color_surface_raised: Color::from_argb(255, 58, 63, 77),
    color_surface_overlay: Color::from_argb(255, 18, 20, 26),
    color_surface_chrome: Color::from_argb(255, 18, 20, 26),
    color_surface_column: Color::from_argb(255, 10, 12, 17),
    color_content: Color::from_argb(255, 242, 243, 250),
    color_content_secondary: Color::from_argb(255, 195, 200, 220),
    color_content_nav: Color::from_argb(255, 154, 161, 187),
    color_content_muted: Color::from_argb(255, 154, 161, 187),
    color_content_subtle: Color::from_argb(255, 139, 147, 176),
    color_content_faint: Color::from_argb(255, 106, 112, 144),
    color_border: Color::from_argb(15, 255, 255, 255),
    color_border_strong: Color::from_argb(23, 255, 255, 255),
    color_border_muted: Color::from_argb(255, 54, 60, 77),
    shadow_2xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(2.0), spread: px(0.0), color: Color::from_argb(102, 0, 0, 0) }],
    shadow_xs: &[TokenShadow { x: px(0.0), y: px(1.0), blur: px(3.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_sm: &[TokenShadow { x: px(0.0), y: px(4.0), blur: px(14.0), spread: px(0.0), color: Color::from_argb(115, 0, 0, 0) }],
    shadow_md: &[TokenShadow { x: px(0.0), y: px(10.0), blur: px(28.0), spread: px(0.0), color: Color::from_argb(140, 0, 0, 0) }],
    shadow_lg: &[TokenShadow { x: px(0.0), y: px(14.0), blur: px(36.0), spread: px(0.0), color: Color::from_argb(148, 0, 0, 0) }],
    shadow_xl: &[TokenShadow { x: px(0.0), y: px(16.0), blur: px(40.0), spread: px(0.0), color: Color::from_argb(153, 0, 0, 0) }],
    shadow_2xl: &[TokenShadow { x: px(0.0), y: px(24.0), blur: px(64.0), spread: px(0.0), color: Color::from_argb(179, 0, 0, 0) }],
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
