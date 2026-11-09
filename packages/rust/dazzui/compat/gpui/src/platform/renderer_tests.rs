//! GPU readbacks through the production Metal, Blade and DirectX scene encoders.
//! No platform window, TestAppContext or screen capture is used.
use super::*;
use crate::{ContentMask, Corners, Hsla, TransformationMatrix, point, rgb, size};
// Windows' parent module also imports GPUI's context-taking `test` macro.
use core::prelude::v1::test;

#[derive(Clone, Copy)]
enum CaptureFormat {
    Bgra,
    #[cfg(any(
        not(any(target_os = "macos", target_os = "windows")),
        feature = "macos-blade"
    ))]
    Rgba,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurfaceAlpha {
    PreMultiplied,
    #[cfg(any(
        not(any(target_os = "macos", target_os = "windows")),
        feature = "macos-blade"
    ))]
    Ignored,
    #[cfg(any(
        not(any(target_os = "macos", target_os = "windows")),
        feature = "macos-blade"
    ))]
    PostMultiplied,
}
fn alpha_modes() -> impl Iterator<Item = SurfaceAlpha> {
    #[cfg(any(
        target_os = "windows",
        all(target_os = "macos", not(feature = "macos-blade"))
    ))]
    return [SurfaceAlpha::PreMultiplied].into_iter();
    #[cfg(any(
        not(any(target_os = "macos", target_os = "windows")),
        feature = "macos-blade"
    ))]
    return [
        SurfaceAlpha::Ignored,
        SurfaceAlpha::PostMultiplied,
        SurfaceAlpha::PreMultiplied,
    ]
    .into_iter();
}

#[cfg(all(target_os = "macos", not(feature = "macos-blade")))]
type TestRenderer = MetalRenderer;
#[cfg(any(
    not(any(target_os = "macos", target_os = "windows")),
    feature = "macos-blade"
))]
type TestRenderer = BladeRenderer;
#[cfg(target_os = "windows")]
type TestRenderer = DirectXRenderer;

#[cfg(all(target_os = "macos", not(feature = "macos-blade")))]
struct AutoreleasePool(cocoa::base::id);
#[cfg(all(target_os = "macos", not(feature = "macos-blade")))]
impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        use cocoa::foundation::NSAutoreleasePool;
        unsafe {
            self.0.drain();
        }
    }
}

struct Fixture {
    renderer: TestRenderer,
    #[cfg(all(target_os = "macos", not(feature = "macos-blade")))]
    _pool: AutoreleasePool,
}
impl Fixture {
    fn new() -> Self {
        #[cfg(any(
            target_os = "windows",
            all(target_os = "macos", not(feature = "macos-blade"))
        ))]
        return Self::with_format(CaptureFormat::Bgra);
        #[cfg(any(
            not(any(target_os = "macos", target_os = "windows")),
            feature = "macos-blade"
        ))]
        return Self::with_format(CaptureFormat::Rgba);
    }
    fn with_format(format: CaptureFormat) -> Self {
        Self::with_alpha(format, SurfaceAlpha::PreMultiplied)
    }
    fn with_alpha(format: CaptureFormat, alpha: SurfaceAlpha) -> Self {
        if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
            std::fs::create_dir_all(out).unwrap();
        }
        #[cfg(target_os = "windows")]
        {
            let _ = format;
            assert_eq!(alpha, SurfaceAlpha::PreMultiplied);
            let native = DirectXDevices::new().expect("a native Direct3D device is required");
            let devices = DirectXRendererDevices::new(&native, true).unwrap();
            let desc = unsafe { devices.adapter.GetDesc1() }.unwrap();
            assert_eq!(
                desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32,
                0,
                "the DirectX GPU probe requires a hardware adapter"
            );
            eprintln!(
                "DirectX GPU: {}",
                String::from_utf16_lossy(&desc.Description).trim_end_matches('\0')
            );
            // SSH session 0 has no presentation swap chain. Use the same
            // encoders with a plain render target and staging readback.
            let resources = DirectXResources::offscreen(&devices, 64, 64).unwrap();
            let atlas = Arc::new(DirectXAtlas::new(&devices.device, &devices.device_context));
            let globals = DirectXGlobalElements::new(&devices.device).unwrap();
            let pipelines = DirectXRenderPipelines::new(&devices.device).unwrap();
            Self {
                renderer: DirectXRenderer {
                    hwnd: HWND::default(),
                    atlas,
                    devices,
                    resources,
                    globals,
                    pipelines,
                    direct_composition: None,
                    font_info: DirectXRenderer::get_font_info(),
                    composite_targets: Vec::new(),
                },
            }
        }
        #[cfg(all(target_os = "macos", not(feature = "macos-blade")))]
        {
            use cocoa::foundation::NSAutoreleasePool;
            let _ = format; // Metal's production pipelines use BGRA8Unorm.
            assert_eq!(alpha, SurfaceAlpha::PreMultiplied);
            let pool = AutoreleasePool(unsafe { NSAutoreleasePool::new(cocoa::base::nil) });
            let mut renderer =
                MetalRenderer::new(Arc::new(Mutex::new(InstanceBufferPool::default())));
            eprintln!("Metal GPU: {}", renderer.device.name());
            renderer.update_drawable_size(size(DevicePixels(64), DevicePixels(64)));
            Self {
                renderer,
                _pool: pool,
            }
        }
        #[cfg(any(
            not(any(target_os = "macos", target_os = "windows")),
            feature = "macos-blade"
        ))]
        {
            let gpu = Arc::new(
                unsafe {
                    gpu::Context::init(gpu::ContextDesc {
                        presentation: false,
                        validation: true,
                        ..Default::default()
                    })
                }
                .expect("a native graphics device is required for this GPU test"),
            );
            eprintln!("GPU: {:?}", gpu.device_information());
            let context = BladeContext { gpu };
            let format = match format {
                CaptureFormat::Rgba => gpu::TextureFormat::Rgba8Unorm,
                CaptureFormat::Bgra => gpu::TextureFormat::Bgra8Unorm,
            };
            let alpha = match alpha {
                SurfaceAlpha::PreMultiplied => gpu::AlphaMode::PreMultiplied,
                SurfaceAlpha::Ignored => gpu::AlphaMode::Ignored,
                SurfaceAlpha::PostMultiplied => gpu::AlphaMode::PostMultiplied,
            };
            Self {
                renderer: BladeRenderer::with_surface(
                    &context,
                    None,
                    gpu::SurfaceInfo { format, alpha },
                    gpu::SurfaceConfig {
                        size: gpu::Extent {
                            width: 64,
                            height: 64,
                            depth: 1,
                        },
                        usage: gpu::TextureUsage::TARGET,
                        display_sync: gpu::DisplaySync::Recent,
                        color_space: gpu::ColorSpace::Srgb,
                        transparent: true,
                        allow_exclusive_full_screen: false,
                    },
                ),
            }
        }
    }
    fn atlas(&self) -> &dyn crate::PlatformAtlas {
        #[cfg(all(target_os = "macos", not(feature = "macos-blade")))]
        return self.renderer.sprite_atlas.as_ref();
        #[cfg(any(not(target_os = "macos"), feature = "macos-blade"))]
        return self.renderer.atlas.as_ref();
    }
    fn resize(&mut self, size: Size<DevicePixels>) {
        #[cfg(not(target_os = "windows"))]
        self.renderer.update_drawable_size(size);
        #[cfg(target_os = "windows")]
        self.renderer.resize(size).unwrap();
    }
    fn capture(&mut self, mut scene: Scene) -> image::RgbaImage {
        scene.finish();
        self.renderer.capture_scene(&scene).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        #[cfg(not(target_os = "windows"))]
        self.renderer.destroy();
    }
}

fn rect(x: f32, y: f32, w: f32, h: f32) -> Bounds<ScaledPixels> {
    Bounds::new(
        point(ScaledPixels(x), ScaledPixels(y)),
        size(ScaledPixels(w), ScaledPixels(h)),
    )
}
fn quad(bounds: Bounds<ScaledPixels>, color: u32) -> Quad {
    Quad {
        bounds,
        content_mask: ContentMask {
            bounds: rect(0., 0., 128., 128.),
        },
        background: rgb(color).into(),
        ..Default::default()
    }
}
fn group(scene: &mut Scene, opacity: f32) {
    scene.push_composite(
        rect(0., 0., 64., 64.),
        ContentMask {
            bounds: rect(0., 0., 64., 64.),
        },
        Corners::default(),
        opacity,
        TransformationMatrix::unit(),
    );
}
fn pixel(image: &image::RgbaImage, x: u32, y: u32, expected: [u8; 4]) {
    let actual = image.get_pixel(x, y).0;
    assert!(
        actual.iter().zip(expected).all(|(a, b)| a.abs_diff(b) <= 1),
        "({x},{y}): {actual:?} != {expected:?}"
    );
}

#[test]
fn gpu_bgra_surface_readback_preserves_color_channels_and_straight_alpha() {
    let mut fixture = Fixture::with_format(CaptureFormat::Bgra);
    let mut scene = Scene::default();
    group(&mut scene, 0.5);
    scene.insert_primitive(quad(rect(4., 8., 32., 16.), 0x407fc0));
    scene.pop_composite();
    let image = fixture.capture(scene);
    pixel(&image, 10, 16, [64, 127, 192, 128]);
    pixel(&image, 60, 16, [0, 0, 0, 0]);
}

#[test]
fn gpu_translucent_overlaps_keep_source_over_alpha_inside_a_group() {
    for group_alpha in [1., 0.5] {
        let mut fixture = Fixture::new();
        let mut scene = Scene::default();
        group(&mut scene, group_alpha);
        let mut color: Hsla = rgb(0xff0000).into();
        color.a = 0.5;
        let mut first = quad(rect(4., 4., 28., 28.), 0xff0000);
        first.background = color.into();
        let mut second = first.clone();
        second.bounds.origin.x = ScaledPixels(20.);
        scene.insert_primitive(first);
        scene.insert_primitive(second);
        scene.pop_composite();
        let image = fixture.capture(scene);
        if group_alpha == 0.5 {
            if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
                image.save(format!("{out}/alpha-overlap.png")).unwrap();
            }
        }
        pixel(
            &image,
            10,
            10,
            [255, 0, 0, (128. * group_alpha).round() as u8],
        );
        pixel(
            &image,
            24,
            10,
            [255, 0, 0, (192. * group_alpha).round() as u8],
        );
        pixel(
            &image,
            40,
            10,
            [255, 0, 0, (128. * group_alpha).round() as u8],
        );
    }
}

#[test]
fn gpu_group_opacity_does_not_accumulate_at_overlapping_children() {
    let mut fixture = Fixture::new();
    let mut scene = Scene::default();
    group(&mut scene, 0.5);
    scene.insert_primitive(quad(rect(4., 8., 32., 16.), 0xff0000));
    scene.insert_primitive(quad(rect(20., 8., 32., 16.), 0x0000ff));
    scene.pop_composite();
    let image = fixture.capture(scene);
    pixel(&image, 10, 16, [255, 0, 0, 128]);
    pixel(&image, 25, 16, [0, 0, 255, 128]);
    pixel(&image, 60, 16, [0, 0, 0, 0]);
    if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
        image.save(format!("{out}/opacity.png")).unwrap();
    }
}

#[test]
fn gpu_nested_rounded_overflow_clips_the_composed_subtree() {
    let mut fixture = Fixture::new();
    let mut scene = Scene::default();
    group(&mut scene, 0.5);
    scene.push_composite(
        rect(8., 8., 32., 32.),
        ContentMask {
            bounds: rect(0., 0., 64., 64.),
        },
        Corners::all(ScaledPixels(10.)),
        0.5,
        TransformationMatrix::unit(),
    );
    scene.insert_primitive(quad(rect(0., 0., 64., 64.), 0x00ff00));
    scene.pop_composite();
    scene.pop_composite();
    let image = fixture.capture(scene);
    pixel(&image, 8, 8, [0, 0, 0, 0]);
    pixel(&image, 24, 24, [0, 255, 0, 64]);
    pixel(&image, 45, 24, [0, 0, 0, 0]);
    if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
        image.save(format!("{out}/rounded.png")).unwrap();
    }
}

#[test]
fn gpu_scaled_group_uses_the_parent_clip_in_destination_coordinates() {
    let mut fixture = Fixture::new();
    let mut scene = Scene::default();
    scene.push_composite(
        rect(0., 0., 64., 64.),
        ContentMask {
            bounds: rect(28., 0., 36., 64.),
        },
        Corners::default(),
        1.,
        TransformationMatrix {
            rotation_scale: [[0.5, 0.], [0., 0.5]],
            translation: [16., 16.],
        },
    );
    scene.insert_primitive(quad(rect(16., 16., 32., 32.), 0xff0000));
    scene.pop_composite();
    let image = fixture.capture(scene);
    pixel(&image, 25, 32, [0, 0, 0, 0]);
    pixel(&image, 32, 32, [255, 0, 0, 255]);
    pixel(&image, 41, 32, [0, 0, 0, 0]);
    if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
        image.save(format!("{out}/scale.png")).unwrap();
    }
}

#[test]
fn gpu_padding_edge_uses_elliptical_corners_and_transparent_border_geometry() {
    use crate::{Edges, Hsla, Overflow, Style, px};
    let mut fixture = Fixture::new();
    let bounds = rect(8., 8., 48., 48.).map(|v| px(v.0));
    let parent = rect(0., 0., 64., 64.).map(|v| px(v.0));
    let cases: &[(f32, f32, &str, &[(u32, u32)], &[(u32, u32)])] = &[
        (
            2.,
            10.,
            "padding-edge",
            &[(9, 30), (47, 30), (28, 10), (28, 49), (10, 14), (11, 47)],
            &[(28, 28), (15, 20), (41, 20), (16, 43), (41, 43)],
        ),
        // An opposite border leaves less room than the original inner arc;
        // the browser constrains every remaining ellipse axis together.
        (
            38.,
            0.,
            "padding-edge-wide-border",
            &[(44, 24)],
            &[(48, 24), (54, 24), (48, 36), (54, 16)],
        ),
    ];
    for &(left, right, id, clear, solid) in cases {
        let style = Style {
            overflow: point(Overflow::Hidden, Overflow::Hidden),
            border_color: Some(Hsla::transparent_black()),
            border_widths: Edges {
                top: px(4.).into(),
                right: px(right).into(),
                bottom: px(8.).into(),
                left: px(left).into(),
            },
            corner_radii: Corners::all(px(16.).into()),
            ..Default::default()
        };
        let mask = style
            .overflow_mask_with_parent(bounds, px(16.), parent)
            .unwrap()
            .scale(1.);
        let radii = style.paint_overflow_corner_radii(bounds, px(16.));
        let mut scene = Scene::default();
        scene.push_elliptical_composite(
            mask.bounds,
            mask.clone(),
            radii.map(|r| r.width.scale(1.)),
            radii.map(|r| r.height.scale(1.)),
            1.,
            TransformationMatrix::unit(),
        );
        scene.insert_primitive(quad(rect(8., 8., 48., 48.), 0x00ff00));
        scene.pop_composite();
        scene.finish();
        let image = fixture.renderer.capture_scene(&scene).unwrap();
        let mut replayed = Scene::default();
        replayed.replay(0..scene.len(), &scene);
        assert_eq!(
            image,
            fixture.capture(replayed),
            "cached paint retains both ellipse axes"
        );
        for &(x, y) in clear {
            pixel(&image, x, y, [0, 0, 0, 0]);
        }
        for &(x, y) in solid {
            pixel(&image, x, y, [0, 255, 0, 255]);
        }
        if id == "padding-edge-wide-border" {
            let alpha = if cfg!(target_os = "macos") {
                [194, 200]
            } else {
                [236, 217]
            };
            pixel(&image, 48, 12, [0, 255, 0, alpha[0]]);
            pixel(&image, 52, 46, [0, 255, 0, alpha[1]]);
        }
        if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
            image.save(format!("{out}/{id}.png")).unwrap();
        }
    }
}

#[test]
fn gpu_single_axis_clip_preserves_visible_axis_and_square_corners() {
    use crate::{Edges, Overflow, Style, px};
    let mut fixture = Fixture::new();
    let style = Style {
        overflow: point(Overflow::Clip, Overflow::Visible),
        border_widths: Edges::all(px(4.).into()),
        corner_radii: Corners::all(px(16.).into()),
        ..Default::default()
    };
    let bounds = rect(16., 16., 32., 32.).map(|v| px(v.0));
    let parent = rect(0., 0., 64., 64.).map(|v| px(v.0));
    let mask = style
        .overflow_mask_with_parent(bounds, px(16.), parent)
        .unwrap()
        .scale(1.);
    let mut primitive = quad(rect(0., 0., 64., 64.), 0x0000ff);
    primitive.content_mask = mask;
    let mut scene = Scene::default();
    scene.insert_primitive(primitive);
    let image = fixture.capture(scene);
    for (x, y) in [(20, 4), (22, 17), (32, 60)] {
        pixel(&image, x, y, [0, 0, 255, 255]);
    }
    for (x, y) in [(19, 32), (44, 32)] {
        pixel(&image, x, y, [0, 0, 0, 0]);
    }
    if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
        image.save(format!("{out}/axis-clip.png")).unwrap();
    }
}

#[test]
fn gpu_scaled_group_keeps_source_pixels_outside_the_window() {
    let mut fixture = Fixture::new();
    let mut scene = Scene::default();
    let source = rect(-32., -32., 128., 128.);
    scene.push_composite(
        source,
        ContentMask {
            bounds: rect(0., 0., 64., 64.),
        },
        Corners::default(),
        1.,
        TransformationMatrix {
            rotation_scale: [[0.5, 0.], [0., 0.5]],
            translation: [16., 16.],
        },
    );
    scene.push_composite(
        source,
        ContentMask { bounds: source },
        Corners::default(),
        0.5,
        TransformationMatrix::unit(),
    );
    for (bounds, color) in [
        (rect(-16., 24., 16., 16.), 0xff0000),
        (rect(64., 24., 16., 16.), 0x0000ff),
        (rect(24., -16., 16., 16.), 0x00ff00),
        (rect(24., 64., 16., 16.), 0xffff00),
    ] {
        let mut primitive = quad(bounds, color);
        primitive.content_mask.bounds = source;
        scene.insert_primitive(primitive);
    }
    scene.pop_composite();
    scene.pop_composite();
    let image = fixture.capture(scene);
    if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
        image.save(format!("{out}/offscreen-scale.png")).unwrap();
    }
    pixel(&image, 12, 32, [255, 0, 0, 128]);
    pixel(&image, 52, 32, [0, 0, 255, 128]);
    pixel(&image, 32, 12, [0, 255, 0, 128]);
    pixel(&image, 32, 52, [255, 255, 0, 128]);
    pixel(&image, 32, 32, [0, 0, 0, 0]);
}

#[test]
fn gpu_cached_group_targets_clear_between_siblings_frames_and_viewport_sizes() {
    let mut fixture = Fixture::new();
    let mut scene = Scene::default();
    group(&mut scene, 0.5);
    scene.insert_primitive(quad(rect(0., 0., 16., 16.), 0xff0000));
    scene.pop_composite();
    group(&mut scene, 0.5);
    scene.insert_primitive(quad(rect(32., 0., 16., 16.), 0x0000ff));
    scene.pop_composite();
    let image = fixture.capture(scene);
    pixel(&image, 8, 8, [255, 0, 0, 128]);
    pixel(&image, 40, 8, [0, 0, 255, 128]);
    fixture.resize(crate::size(DevicePixels(96), DevicePixels(80)));
    let mut scene = Scene::default();
    group(&mut scene, 1.);
    scene.insert_primitive(quad(rect(32., 32., 16., 16.), 0x00ff00));
    scene.pop_composite();
    let image = fixture.capture(scene);
    assert_eq!(image.dimensions(), (96, 80));
    pixel(&image, 8, 8, [0, 0, 0, 0]);
    pixel(&image, 40, 40, [0, 255, 0, 255]);
}

#[test]
fn gpu_group_samples_uploaded_masks_color_images_and_vector_paths_once() {
    use crate::{AtlasKey, FontId, GlyphId, RenderGlyphParams, px, rgba};
    use std::borrow::Cow;
    let mut fixture = Fixture::new();
    let key = |emoji| {
        AtlasKey::Glyph(RenderGlyphParams {
            font_id: FontId(0),
            glyph_id: GlyphId(0),
            font_size: px(12.),
            subpixel_variant: point(0, 0),
            scale_factor: 1.,
            is_emoji: emoji,
        })
    };
    let mono = fixture
        .atlas()
        .get_or_insert_with(&key(false), &mut || {
            Ok(Some((
                size(DevicePixels(4), DevicePixels(4)),
                Cow::Owned(vec![255; 16]),
            )))
        })
        .unwrap()
        .unwrap();
    let poly = fixture
        .atlas()
        .get_or_insert_with(&key(true), &mut || {
            Ok(Some((
                size(DevicePixels(4), DevicePixels(4)),
                Cow::Owned([0, 255, 0, 255].repeat(16)),
            )))
        })
        .unwrap()
        .unwrap();
    let mask = ContentMask {
        bounds: rect(0., 0., 64., 64.),
    };
    for source in [
        rect(0., 0., 64., 64.),
        rect(-32., -32., 128., 128.),
        rect(0., 0., 64., 64.),
    ] {
        let mut scene = Scene::default();
        scene.push_composite(
            source,
            ContentMask {
                bounds: rect(0., 0., 64., 64.),
            },
            Corners::default(),
            0.5,
            TransformationMatrix::unit(),
        );
        scene.insert_primitive(MonochromeSprite {
            order: 0,
            pad: 0,
            bounds: rect(4., 4., 12., 12.),
            content_mask: mask.clone(),
            color: rgb(0xff0000).into(),
            tile: mono.clone(),
            transformation: TransformationMatrix::unit(),
        });
        scene.insert_primitive(PolychromeSprite {
            order: 0,
            pad: 0,
            grayscale: false,
            opacity: 0.5,
            bounds: rect(20., 4., 12., 12.),
            content_mask: mask.clone(),
            corner_radii: Corners::default(),
            tile: poly.clone(),
        });
        let mut path = crate::PathBuilder::fill();
        path.move_to(point(px(36.), px(4.)));
        path.line_to(point(px(48.), px(4.)));
        path.line_to(point(px(48.), px(16.)));
        path.line_to(point(px(36.), px(16.)));
        path.close();
        let mut path = path.build().unwrap().scale(1.);
        path.color = rgba(0x0000ff80).into();
        path.content_mask = mask.clone();
        scene.insert_primitive(path.clone());
        scene.pop_composite();
        // Root paths use the same intermediate texture as enlarged layers,
        // but retain the root viewport's projection and sample coordinates.
        path.bounds.origin.y += ScaledPixels(28.);
        for vertex in &mut path.vertices {
            vertex.xy_position.y += ScaledPixels(28.);
        }
        scene.insert_primitive(path);
        let image = fixture.capture(scene);
        if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
            image.save(format!("{out}/primitives.png")).unwrap();
        }
        pixel(&image, 10, 10, [255, 0, 0, 128]);
        pixel(&image, 26, 10, [0, 255, 0, 64]);
        pixel(&image, 42, 10, [0, 0, 255, 64]);
        pixel(&image, 42, 38, [0, 0, 255, 128]);
        for y in 4..16 {
            for x in 4..16 {
                pixel(&image, x, y, [255, 0, 0, 128]);
                pixel(&image, x + 16, y, [0, 255, 0, 64]);
            }
        }
    }
}

#[test]
fn gpu_transparent_image_filtering_preserves_color_and_clamps_to_its_tile() {
    use crate::{AtlasKey, FontId, GlyphId, RenderGlyphParams, px};
    use std::borrow::Cow;
    for alpha in alpha_modes() {
        let mut fixture = Fixture::with_alpha(CaptureFormat::Bgra, alpha);
        let tile = fixture
            .atlas()
            .get_or_insert_with(
                &AtlasKey::Glyph(RenderGlyphParams {
                    font_id: FontId(0),
                    glyph_id: GlyphId(0),
                    font_size: px(12.),
                    subpixel_variant: point(0, 0),
                    scale_factor: 1.,
                    is_emoji: true,
                }),
                &mut || {
                    Ok(Some((
                        size(DevicePixels(2), DevicePixels(1)),
                        // Opaque red beside transparent blue, in straight BGRA.
                        Cow::Owned(vec![0, 0, 255, 255, 255, 0, 0, 0]),
                    )))
                },
            )
            .unwrap()
            .unwrap();
        let mut scene = Scene::default();
        for (y, isolated) in [(4., false), (20., true)] {
            if isolated {
                group(&mut scene, 0.5);
            }
            scene.insert_primitive(PolychromeSprite {
                order: 0,
                pad: 0,
                grayscale: false,
                opacity: if isolated { 1. } else { 0.5 },
                bounds: rect(4., y, 16., 8.),
                content_mask: ContentMask {
                    bounds: rect(0., 0., 64., 64.),
                },
                corner_radii: Corners::default(),
                tile: tile.clone(),
            });
            if isolated {
                scene.pop_composite();
            }
        }
        let image = fixture.capture(scene);
        if alpha == SurfaceAlpha::PreMultiplied {
            if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
                image.save(format!("{out}/alpha-image.png")).unwrap();
            }
        }
        for y in [7, 23] {
            pixel(&image, 4, y, [255, 0, 0, 128]);
            pixel(&image, 11, y, [255, 0, 0, 72]);
            pixel(&image, 12, y, [255, 0, 0, 56]);
            pixel(&image, 19, y, [0, 0, 0, 0]);
        }
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "windows"
))]
#[test]
fn gpu_translucent_color_font_keeps_its_ink_through_the_atlas() {
    use crate::{AtlasKey, PlatformTextSystem, RenderGlyphParams, font, px};
    use std::borrow::Cow;
    let mut fixture = Fixture::new();
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let system = crate::platform::linux::CosmicTextSystem::new();
    #[cfg(target_os = "macos")]
    let system = crate::platform::mac::MacTextSystem::new();
    #[cfg(target_os = "windows")]
    let system = crate::platform::windows::DirectWriteTextSystem::new(&DirectXDevices {
        adapter: fixture.renderer.devices.adapter.clone(),
        dxgi_factory: fixture.renderer.devices.dxgi_factory.clone(),
        device: fixture.renderer.devices.device.clone(),
        device_context: fixture.renderer.devices.device_context.clone(),
    })
    .unwrap();
    system
        .add_fonts(vec![Cow::Borrowed(include_bytes!(
            "../../fixtures/colr-alpha.ttf"
        ))])
        .unwrap();
    let id = system.font_id(&font("DazzUI Alpha Probe")).unwrap();
    let params = RenderGlyphParams {
        font_id: id,
        glyph_id: system.glyph_for_char(id, 'A').unwrap(),
        font_size: px(24.),
        subpixel_variant: point(0, 0),
        scale_factor: 1.,
        is_emoji: true,
    };
    let bounds = system.glyph_raster_bounds(&params).unwrap();
    let (bitmap_size, bytes) = system.rasterize_glyph(&params, bounds).unwrap();
    let tile = fixture
        .atlas()
        .get_or_insert_with(&AtlasKey::Glyph(params), &mut || {
            Ok(Some((bitmap_size, Cow::Borrowed(&bytes))))
        })
        .unwrap()
        .unwrap();
    let mut scene = Scene::default();
    group(&mut scene, 0.5);
    scene.insert_primitive(PolychromeSprite {
        order: 0,
        pad: 0,
        grayscale: false,
        opacity: 1.,
        bounds: rect(
            4.,
            4.,
            bitmap_size.width.0 as f32,
            bitmap_size.height.0 as f32,
        ),
        content_mask: ContentMask {
            bounds: rect(0., 0., 64., 64.),
        },
        corner_radii: Corners::default(),
        tile,
    });
    scene.pop_composite();
    let image = fixture.capture(scene);
    pixel(&image, 16, 16, [255, 0, 0, 64]);
    pixel(&image, 40, 40, [0, 0, 0, 0]);
    if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
        image.save(format!("{out}/color-font.png")).unwrap();
    }
}

#[test]
fn gpu_text_decorations_apply_ink_alpha_once_before_group_opacity() {
    use crate::Hsla;
    for mode in alpha_modes() {
        let mut fixture = Fixture::with_alpha(CaptureFormat::Bgra, mode);
        for wavy in [false, true] {
            let height = if wavy { 6. } else { 2. };
            for ink_alpha in [0., 0.25, 0.5, 0.75, 1.] {
                for group_alpha in [1., 0.5] {
                    let mut scene = Scene::default();
                    for (y, alpha, isolated) in [(4., 1., false), (24., ink_alpha, true)] {
                        if isolated && group_alpha < 1. {
                            group(&mut scene, group_alpha);
                        }
                        let mut color: Hsla = rgb(0xff0000).into();
                        color.a = alpha;
                        scene.insert_primitive(Underline {
                            order: 0,
                            pad: 0,
                            bounds: rect(4., y, 48., height),
                            content_mask: ContentMask {
                                bounds: rect(10., 0., 30., 64.),
                            },
                            color,
                            thickness: ScaledPixels(2.),
                            wavy: u32::from(wavy),
                        });
                        if isolated && group_alpha < 1. {
                            scene.pop_composite();
                        }
                    }
                    let image = fixture.capture(scene);
                    let mut partial_coverage = 0;
                    for y in 0..height as u32 {
                        for x in 0..64 {
                            let control = image.get_pixel(x, y + 4).0;
                            let actual = image.get_pixel(x, y + 24).0;
                            let expected_alpha =
                                (f32::from(control[3]) * ink_alpha * group_alpha).round() as u8;
                            assert!(
                                actual[3].abs_diff(expected_alpha) <= 1,
                                "{mode:?}, wavy={wavy}, ink={ink_alpha}, group={group_alpha}, ({x},{y}): {actual:?}, expected alpha {expected_alpha}"
                            );
                            if actual[3] > 0 {
                                assert_eq!(&actual[..3], &[255, 0, 0]);
                            }
                            if control[3] > 0 && control[3] < 255 {
                                partial_coverage += 1;
                            }
                        }
                    }
                    if wavy {
                        assert!(partial_coverage > 0, "exercise actual curve coverage");
                    }
                    if mode == SurfaceAlpha::PreMultiplied && ink_alpha == 0.5 && group_alpha == 0.5
                    {
                        if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
                            let name = if wavy {
                                "wavy-decorations"
                            } else {
                                "decorations"
                            };
                            image.save(format!("{out}/{name}.png")).unwrap();
                        }
                    }
                }
            }
        }
    }
}

/// CSS cases are shared with example/tool/overflow-raster.mjs. The browser
/// independently resolves the clips and compares every alpha pixel at 1x/2x.
#[test]
fn gpu_overflow_raster_matrix_keeps_solid_gradient_image_and_split_paths() {
    use crate::{
        AtlasKey, Edges, FontId, GlyphId, Overflow, RenderGlyphParams, Style, linear_color_stop,
        linear_gradient, px,
    };
    use std::borrow::Cow;
    #[derive(serde::Deserialize)]
    struct Case {
        id: String,
        r: [f32; 4],
        b: [f32; 4],
        x: Option<f32>,
        y: Option<f32>,
        w: Option<f32>,
        h: Option<f32>,
        paint: Option<String>,
        scale: Option<f32>,
        empty: Option<bool>,
        center_alpha: Option<u8>,
    }
    let cases: Vec<Case> =
        serde_json::from_str(include_str!("../../fixtures/overflow-raster.json")).unwrap();
    let mut fixture = Fixture::new();
    let tile = fixture
        .atlas()
        .get_or_insert_with(
            &AtlasKey::Glyph(RenderGlyphParams {
                font_id: FontId(0),
                glyph_id: GlyphId(0),
                font_size: px(12.),
                subpixel_variant: point(0, 0),
                scale_factor: 1.,
                is_emoji: true,
            }),
            &mut || {
                Ok(Some((
                    size(DevicePixels(1), DevicePixels(1)),
                    Cow::Owned(vec![0, 255, 0, 255]),
                )))
            },
        )
        .unwrap()
        .unwrap();
    for dpr in [1., 2.] {
        fixture.resize(size(
            DevicePixels((64. * dpr) as i32),
            DevicePixels((64. * dpr) as i32),
        ));
        for c in &cases {
            let bounds = rect(
                c.x.unwrap_or(8.),
                c.y.unwrap_or(8.),
                c.w.unwrap_or(48.),
                c.h.unwrap_or(48.),
            )
            .map(|v| px(v.0));
            let style = Style {
                overflow: point(Overflow::Hidden, Overflow::Hidden),
                border_widths: Edges {
                    top: px(c.b[0]).into(),
                    right: px(c.b[1]).into(),
                    bottom: px(c.b[2]).into(),
                    left: px(c.b[3]).into(),
                },
                corner_radii: Corners {
                    top_left: px(c.r[0]).into(),
                    top_right: px(c.r[1]).into(),
                    bottom_right: px(c.r[2]).into(),
                    bottom_left: px(c.r[3]).into(),
                },
                ..Default::default()
            };
            let mask = style
                .paint_overflow_mask(bounds, px(16.), rect(0., 0., 64., 64.).map(|v| px(v.0)))
                .unwrap()
                .scale(dpr);
            let radii = style.paint_overflow_corner_radii(bounds, px(16.));
            let scale = c.scale.unwrap_or(1.);
            let transformation = TransformationMatrix {
                rotation_scale: [[scale, 0.], [0., scale]],
                translation: [
                    bounds.origin.x.0 * dpr * (1. - scale),
                    bounds.origin.y.0 * dpr * (1. - scale),
                ],
            };
            let mut scene = Scene::default();
            scene.push_elliptical_composite(
                mask.bounds,
                ContentMask {
                    bounds: rect(0., 0., 64. * dpr, 64. * dpr),
                },
                radii.map(|r| r.width.scale(dpr)),
                radii.map(|r| r.height.scale(dpr)),
                1.,
                transformation,
            );
            let full = Style::pixel_snapped_bounds(bounds).scale(dpr);
            match c.paint.as_deref() {
                Some("gradient") => {
                    let mut q = quad(full, 0);
                    q.background = linear_gradient(
                        180.,
                        linear_color_stop(rgb(0xff0000), 0.),
                        linear_color_stop(rgb(0x0000ff), 1.),
                    );
                    scene.insert_primitive(q);
                }
                Some("image" | "image-overlay") => {
                    scene.insert_primitive(PolychromeSprite {
                        order: 0,
                        pad: 0,
                        grayscale: false,
                        bounds: full,
                        content_mask: ContentMask {
                            bounds: rect(0., 0., 128., 128.),
                        },
                        corner_radii: Corners::default(),
                        opacity: 1.,
                        tile: tile.clone(),
                    });
                    if c.paint.as_deref() == Some("image-overlay") {
                        let b = mask.bounds;
                        let mut q = quad(
                            rect(
                                b.left().0 + b.size.width.0 * 0.3,
                                b.top().0,
                                b.size.width.0 * 0.7,
                                b.size.height.0,
                            ),
                            0x0000ff,
                        );
                        q.background = Hsla {
                            a: 0.5,
                            ..rgb(0x0000ff).into()
                        }
                        .into();
                        scene.insert_primitive(q);
                    }
                }
                Some("overlap" | "nested-overlap") => {
                    let b = mask.bounds;
                    let nested = c.paint.as_deref() == Some("nested-overlap");
                    if nested {
                        scene.push_composite(
                            b,
                            ContentMask { bounds: b },
                            Corners::default(),
                            0.5,
                            TransformationMatrix::unit(),
                        );
                    }
                    for (index, color) in [0xff0000, 0x0000ff].into_iter().enumerate() {
                        let mut q = quad(
                            rect(
                                b.left().0 + index as f32 * b.size.width.0 * 0.3,
                                b.top().0,
                                b.size.width.0 * 0.7,
                                b.size.height.0,
                            ),
                            color,
                        );
                        q.background = Hsla {
                            a: 0.5,
                            ..rgb(color).into()
                        }
                        .into();
                        scene.insert_primitive(q);
                    }
                    if nested {
                        scene.pop_composite();
                    }
                }
                Some("interleaved") => {
                    let b = mask.bounds;
                    let red = rect(b.left().0, b.top().0, b.size.width.0 * 0.7, b.size.height.0);
                    let blue = rect(
                        b.left().0 + b.size.width.0 * 0.3,
                        b.top().0,
                        b.size.width.0 * 0.7,
                        b.size.height.0,
                    );
                    let green = rect(
                        b.left().0,
                        b.top().0 + b.size.height.0 * 0.5,
                        b.size.width.0 * 0.7,
                        b.size.height.0 * 0.5,
                    );
                    let mut q = quad(red, 0xff0000);
                    q.background = Hsla {
                        a: 0.5,
                        ..rgb(0xff0000).into()
                    }
                    .into();
                    scene.insert_primitive(q);
                    scene.push_composite(
                        blue,
                        ContentMask { bounds: blue },
                        Corners::default(),
                        0.5,
                        TransformationMatrix::unit(),
                    );
                    scene.insert_primitive(quad(blue, 0x0000ff));
                    scene.pop_composite();
                    let mut q = quad(green, 0x00ff00);
                    q.background = Hsla {
                        a: 0.25,
                        ..rgb(0x00ff00).into()
                    }
                    .into();
                    scene.insert_primitive(q);
                }
                Some("split") => {
                    let b = mask.bounds;
                    for (index, color) in [0xff0000, 0x0000ff].into_iter().enumerate() {
                        scene.insert_primitive(quad(
                            rect(
                                b.origin.x.0 + index as f32 * b.size.width.0 / 2.,
                                b.origin.y.0,
                                b.size.width.0 / 2.,
                                b.size.height.0,
                            ),
                            color,
                        ));
                    }
                }
                None => scene.insert_primitive(quad(full, 0x00ff00)),
                Some(paint) => panic!("unknown paint {paint}"),
            }
            scene.pop_composite();
            let image = fixture.capture(scene);
            if cfg!(target_os = "macos") {
                // Independent hardware Chromium values; the old circle SDF
                // and rectangular varying interpolation miss these pixels.
                match (c.id.as_str(), dpr as u32) {
                    ("circle", 1) => pixel(&image, 17, 8, [0, 255, 0, 255]),
                    ("tiny-circle", 2) => pixel(&image, 16, 16, [0, 255, 0, 218]),
                    ("small-ellipse", 2) => pixel(&image, 16, 18, [0, 255, 0, 20]),
                    ("eccentric", 2) => pixel(&image, 20, 46, [0, 255, 0, 7]),
                    _ => {}
                }
            }
            if c.empty == Some(true) {
                assert!(
                    image.pixels().all(|p| p.0[3] == 0),
                    "{} must match the browser's empty paint",
                    c.id
                );
            }
            let visible = mask.bounds.intersect(&rect(0., 0., 64. * dpr, 64. * dpr));
            let center = transformation.apply(visible.center().map(|p| px(p.0)));
            let center_alpha = image.get_pixel(center.x.0 as u32, center.y.0 as u32).0[3];
            assert!(
                if let Some(expected) = c.center_alpha {
                    center_alpha.abs_diff(expected) <= 1
                } else if mask.bounds.is_empty() {
                    center_alpha == 0
                } else if mask.bounds.size.width.0 * scale < 1. {
                    center_alpha > 0
                } else {
                    center_alpha >= 254
                },
                "{} must paint its center",
                c.id
            );
            if c.x.unwrap_or(8.) >= 1. && c.y.unwrap_or(8.) >= 1. {
                pixel(&image, 0, 0, [0, 0, 0, 0]);
            }
            if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
                image
                    .save(format!("{out}/{}-{}.png", c.id, dpr as u32))
                    .unwrap();
            }
        }
    }
}

#[test]
fn gpu_rrect_fringe_respects_ancestor_clip_and_following_paint() {
    let mut fixture = Fixture::new();
    for clipped in [false, true] {
        let mut scene = Scene::default();
        let mask = ContentMask {
            bounds: if clipped {
                rect(0., 8., 64., 56.)
            } else {
                rect(0., 0., 64., 64.)
            },
        };
        scene.push_composite(
            rect(8., 8., 32., 32.),
            mask,
            Corners::all(ScaledPixels(16.)),
            0.5,
            TransformationMatrix::unit(),
        );
        scene.insert_primitive(quad(rect(0., 0., 64., 64.), 0x00ff00));
        scene.pop_composite();
        let mut replayed = Scene::default();
        replayed.replay(0..scene.len(), &scene);
        let image = fixture.capture(replayed);
        if clipped || cfg!(target_os = "macos") {
            pixel(&image, 23, 7, [0, 0, 0, 0]);
        } else {
            pixel(&image, 23, 7, [0, 255, 0, 6]);
        }
        pixel(&image, 24, 24, [0, 255, 0, 128]);
        // Ends before the logical rrect, but covers its AA fringe. It must
        // retain its later paint order instead of batching before the rrect.
        scene.insert_primitive(quad(rect(0., 0., 64., 7.9), 0xff0000));
        pixel(&fixture.capture(scene), 23, 7, [255, 0, 0, 255]);
    }
}

/// Render real Div output, including its child content masks, rather than
/// relying only on manually assembled renderer primitives.
#[gpui::test]
fn gpu_styled_div_pixel_snapping_uses_the_actual_layout_and_paint_path(
    cx: &mut crate::TestAppContext,
) {
    use crate::{AvailableSpace, IntoElement, ParentElement, Styled, canvas, div, px};
    use std::{cell::RefCell, rc::Rc};
    let fixture = Rc::new(RefCell::new(Fixture::new()));
    let cx = cx.add_empty_window();
    cx.simulate_resize(size(px(64.), px(64.)));
    #[derive(serde::Deserialize)]
    struct Case {
        id: String,
        x: Option<f32>,
        y: Option<f32>,
        w: Option<f32>,
        h: Option<f32>,
        r: [f32; 4],
        b: [f32; 4],
        styled: Option<bool>,
        paint: Option<String>,
        scale: Option<f32>,
        empty: Option<bool>,
    }
    let cases: Vec<Case> =
        serde_json::from_str(include_str!("../../fixtures/overflow-raster.json")).unwrap();
    for c in cases.into_iter().filter(|c| c.styled == Some(true)) {
        assert!(
            c.paint.is_none() && c.scale.unwrap_or(1.) == 1.,
            "the styled probe requires a solid, untransformed fixture"
        );
        let empty = c.empty == Some(true);
        let (id, x, y, width, height, radius, border) = (
            c.id,
            c.x.unwrap_or(8.),
            c.y.unwrap_or(8.),
            c.w.unwrap_or(48.),
            c.h.unwrap_or(48.),
            c.r,
            c.b,
        );
        let fixture = fixture.clone();
        cx.draw(Point::default(), size(px(64.), px(64.)), move |_, _| {
            canvas(
                move |_, window, cx| {
                    let mut element = div()
                        .size_full()
                        .child(
                            div()
                                .absolute()
                                .left(px(x))
                                .top(px(y))
                                .w(px(width))
                                .h(px(height))
                                .rounded_tl(px(radius[0]))
                                .rounded_tr(px(radius[1]))
                                .rounded_br(px(radius[2]))
                                .rounded_bl(px(radius[3]))
                                .overflow_hidden()
                                .border_t(px(border[0]))
                                .border_r(px(border[1]))
                                .border_b(px(border[2]))
                                .border_l(px(border[3]))
                                .child(
                                    div()
                                        .absolute()
                                        .left(px(-border[3]))
                                        .top(px(-border[0]))
                                        .w(px(width))
                                        .h(px(height))
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
                move |_, mut element, window, cx| {
                    element.paint(window, cx);
                    let mut scene = Scene::default();
                    scene.replay(0..window.next_frame.scene.len(), &window.next_frame.scene);
                    let scale = window.scale_factor();
                    let mut fixture = RefCell::borrow_mut(fixture.as_ref());
                    fixture.resize(size(
                        DevicePixels((64. * scale) as i32),
                        DevicePixels((64. * scale) as i32),
                    ));
                    let image = fixture.capture(scene);
                    if empty {
                        assert!(image.pixels().all(|p| p.0[3] == 0));
                    } else {
                        let (x, y) = if id == "fractional-ellipse" {
                            (32., 33.)
                        } else {
                            (8., 10.)
                        };
                        pixel(
                            &image,
                            (x * scale) as u32,
                            (y * scale) as u32,
                            [0, 255, 0, 255],
                        );
                    }
                    if let Ok(out) = std::env::var("DAZZUI_GPU_PROBE_OUT") {
                        image
                            .save(format!("{out}/styled-{id}-{}.png", scale as u32))
                            .unwrap();
                    }
                },
            )
            .size_full()
        });
    }
}
