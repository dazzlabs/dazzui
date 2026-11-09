// Doing `if let` gives you nice scoping with passes/encoders
#![allow(irrefutable_let_patterns)]

use super::{BladeAtlas, BladeContext};
use crate::{
    Background, Bounds, DevicePixels, GpuSpecs, MonochromeSprite, Path, Point, PolychromeSprite,
    PrimitiveBatch, Quad, ScaledPixels, Scene, Shadow, Size, Underline,
};
use blade_graphics as gpu;
use blade_util::{BufferBelt, BufferBeltDescriptor};
use bytemuck::{Pod, Zeroable};
#[cfg(target_os = "macos")]
use media::core_video::CVMetalTextureCache;
use smallvec::SmallVec;
use std::sync::Arc;

const MAX_FRAME_TIME_MS: u32 = 10000;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GlobalParams {
    viewport_size: [f32; 2],
    premultiplied_alpha: u32,
    pad: u32,
}

//Note: we can't use `Bounds` directly here because
// it doesn't implement Pod + Zeroable
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct PodBounds {
    origin: [f32; 2],
    size: [f32; 2],
}

impl From<Bounds<ScaledPixels>> for PodBounds {
    fn from(bounds: Bounds<ScaledPixels>) -> Self {
        Self {
            origin: [bounds.origin.x.0, bounds.origin.y.0],
            size: [bounds.size.width.0, bounds.size.height.0],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct SurfaceParams {
    bounds: PodBounds,
    content_mask: PodBounds,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CompositeParams {
    bounds: PodBounds,
    content_mask: PodBounds,
    corner_radii: [f32; 4],
    corner_radii_y: [f32; 4],
    opacity: f32,
    pad: [f32; 3],
    transform_row0: [f32; 4],
    transform_row1: [f32; 4],
    source_origin: [f32; 2],
    source_pad: [f32; 2],
}

#[derive(blade_macros::ShaderData)]
struct ShaderCompositeData {
    globals: GlobalParams,
    composite: CompositeParams,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
}

#[derive(blade_macros::ShaderData)]
struct ShaderSolidCompositeData {
    globals: GlobalParams,
    b_solid_rrect_vertices: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderGraphiteCompositeData {
    globals: GlobalParams,
    composite: CompositeParams,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
    b_graphite_rrect_vertices: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderClipData {
    globals: GlobalParams,
    b_clip_vertices: gpu::BufferPiece,
}

#[derive(Clone, Copy)]
struct ClipTarget {
    color: gpu::Texture,
    color_view: gpu::TextureView,
    stencil: gpu::Texture,
    stencil_view: gpu::TextureView,
    size: gpu::Extent,
}

enum ClipDraw<'a> {
    Original(PrimitiveBatch<'a>),
    Folded(crate::Composite),
}

#[derive(Clone, Copy)]
enum ScenePipelineMode {
    Root,
    Premultiplied,
    ClipTest,
    ClipBypass,
}

macro_rules! scene_pipelines {
    ($renderer:expr, $mode:expr) => {
        match $mode {
            ScenePipelineMode::Root => &$renderer.pipelines,
            ScenePipelineMode::Premultiplied => &$renderer.offscreen_pipelines,
            ScenePipelineMode::ClipTest => &$renderer.clip_pipelines.as_ref().unwrap().test,
            ScenePipelineMode::ClipBypass => &$renderer.clip_pipelines.as_ref().unwrap().bypass,
        }
    };
}

struct CompositeTarget {
    texture: gpu::Texture,
    view: gpu::TextureView,
    size: gpu::Extent,
}

#[derive(blade_macros::ShaderData)]
struct ShaderQuadsData {
    globals: GlobalParams,
    b_quads: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderShadowsData {
    globals: GlobalParams,
    b_shadows: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderPathRasterizationData {
    globals: GlobalParams,
    b_path_vertices: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderPathsData {
    globals: GlobalParams,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
    b_path_sprites: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderUnderlinesData {
    globals: GlobalParams,
    b_underlines: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderMonoSpritesData {
    globals: GlobalParams,
    gamma_ratios: [f32; 4],
    grayscale_enhanced_contrast: f32,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
    b_mono_sprites: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderPolySpritesData {
    globals: GlobalParams,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
    b_poly_sprites: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderSurfacesData {
    globals: GlobalParams,
    surface_locals: SurfaceParams,
    t_y: gpu::TextureView,
    t_cb_cr: gpu::TextureView,
    s_surface: gpu::Sampler,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
struct PathSprite {
    bounds: Bounds<ScaledPixels>,
}

#[derive(Clone, Debug)]
#[repr(C)]
struct PathRasterizationVertex {
    xy_position: Point<ScaledPixels>,
    st_position: Point<f32>,
    color: Background,
    bounds: Bounds<ScaledPixels>,
}

struct BladePipelines {
    graphite_composite: gpu::RenderPipeline,
    solid_composite: gpu::RenderPipeline,
    composite: gpu::RenderPipeline,
    quads: gpu::RenderPipeline,
    shadows: gpu::RenderPipeline,
    path_rasterization: gpu::RenderPipeline,
    paths: gpu::RenderPipeline,
    underlines: gpu::RenderPipeline,
    mono_sprites: gpu::RenderPipeline,
    poly_sprites: gpu::RenderPipeline,
    surfaces: gpu::RenderPipeline,
}

impl BladePipelines {
    fn new(gpu: &gpu::Context, surface_info: gpu::SurfaceInfo, path_sample_count: u32) -> Self {
        Self::with_clip(gpu, surface_info, path_sample_count, None)
    }

    fn with_clip(
        gpu: &gpu::Context,
        surface_info: gpu::SurfaceInfo,
        path_sample_count: u32,
        test_clip: Option<bool>,
    ) -> Self {
        use gpu::ShaderData as _;
        let depth_stencil = test_clip.map(|test| {
            clip_depth_stencil(if test {
                ClipStencilMode::Test
            } else {
                ClipStencilMode::Bypass
            })
        });
        let multisample_state = gpu::MultisampleState {
            sample_count: if test_clip.is_some() { 4 } else { 1 },
            ..Default::default()
        };

        log::info!(
            "Initializing Blade pipelines for surface {:?}",
            surface_info
        );
        let shader = gpu.create_shader(gpu::ShaderDesc {
            source: include_str!("shaders.wgsl"),
        });
        shader.check_struct_size::<GlobalParams>();
        shader.check_struct_size::<SurfaceParams>();
        shader.check_struct_size::<CompositeParams>();
        shader.check_struct_size::<crate::scene::GraphiteRRectVertex>();
        shader.check_struct_size::<Quad>();
        shader.check_struct_size::<crate::scene::SolidRRectVertex>();
        shader.check_struct_size::<Shadow>();
        shader.check_struct_size::<PathRasterizationVertex>();
        shader.check_struct_size::<PathSprite>();
        shader.check_struct_size::<Underline>();
        shader.check_struct_size::<MonochromeSprite>();
        shader.check_struct_size::<PolychromeSprite>();

        // See https://apoorvaj.io/alpha-compositing-opengl-blending-and-premultiplied-alpha/
        let blend_mode = match surface_info.alpha {
            gpu::AlphaMode::Ignored => gpu::BlendState::ALPHA_BLENDING,
            gpu::AlphaMode::PreMultiplied => gpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
            gpu::AlphaMode::PostMultiplied => gpu::BlendState::ALPHA_BLENDING,
        };
        let color_targets = &[gpu::ColorTargetState {
            format: surface_info.format,
            blend: Some(blend_mode),
            write_mask: gpu::ColorWrites::default(),
        }];

        Self {
            graphite_composite: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "graphite composite",
                data_layouts: &[&ShaderGraphiteCompositeData::layout()],
                vertex: shader.at("vs_graphite_composite"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_graphite_composite")),
                color_targets: &[gpu::ColorTargetState {
                    format: surface_info.format,
                    blend: Some(gpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: gpu::ColorWrites::default(),
                }],
                multisample_state,
            }),
            solid_composite: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "solid composite",
                data_layouts: &[&ShaderSolidCompositeData::layout()],
                vertex: shader.at("vs_solid_composite"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_solid_composite")),
                color_targets: &[gpu::ColorTargetState {
                    format: surface_info.format,
                    blend: Some(gpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: gpu::ColorWrites::default(),
                }],
                multisample_state,
            }),
            composite: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "composite",
                data_layouts: &[&ShaderCompositeData::layout()],
                vertex: shader.at("vs_composite"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_composite")),
                color_targets: &[gpu::ColorTargetState {
                    format: surface_info.format,
                    blend: Some(gpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: gpu::ColorWrites::default(),
                }],
                multisample_state,
            }),
            quads: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "quads",
                data_layouts: &[&ShaderQuadsData::layout()],
                vertex: shader.at("vs_quad"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_quad")),
                color_targets,
                multisample_state,
            }),
            shadows: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "shadows",
                data_layouts: &[&ShaderShadowsData::layout()],
                vertex: shader.at("vs_shadow"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_shadow")),
                color_targets,
                multisample_state,
            }),
            path_rasterization: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "path_rasterization",
                data_layouts: &[&ShaderPathRasterizationData::layout()],
                vertex: shader.at("vs_path_rasterization"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_path_rasterization")),
                // The original implementation was using ADDITIVE blende mode,
                // I don't know why
                // color_targets: &[gpu::ColorTargetState {
                //     format: PATH_TEXTURE_FORMAT,
                //     blend: Some(gpu::BlendState::ADDITIVE),
                //     write_mask: gpu::ColorWrites::default(),
                // }],
                color_targets: &[gpu::ColorTargetState {
                    format: surface_info.format,
                    blend: Some(gpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: gpu::ColorWrites::default(),
                }],
                multisample_state: gpu::MultisampleState {
                    sample_count: path_sample_count,
                    ..Default::default()
                },
            }),
            paths: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "paths",
                data_layouts: &[&ShaderPathsData::layout()],
                vertex: shader.at("vs_path"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_path")),
                color_targets: &[gpu::ColorTargetState {
                    format: surface_info.format,
                    blend: Some(gpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: gpu::ColorWrites::default(),
                }],
                multisample_state,
            }),
            underlines: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "underlines",
                data_layouts: &[&ShaderUnderlinesData::layout()],
                vertex: shader.at("vs_underline"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_underline")),
                color_targets,
                multisample_state,
            }),
            mono_sprites: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "mono-sprites",
                data_layouts: &[&ShaderMonoSpritesData::layout()],
                vertex: shader.at("vs_mono_sprite"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_mono_sprite")),
                color_targets,
                multisample_state,
            }),
            poly_sprites: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "poly-sprites",
                data_layouts: &[&ShaderPolySpritesData::layout()],
                vertex: shader.at("vs_poly_sprite"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_poly_sprite")),
                color_targets,
                multisample_state,
            }),
            surfaces: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "surfaces",
                data_layouts: &[&ShaderSurfacesData::layout()],
                vertex: shader.at("vs_surface"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: depth_stencil.clone(),
                fragment: Some(shader.at("fs_surface")),
                color_targets,
                multisample_state,
            }),
        }
    }

    fn destroy(&mut self, gpu: &gpu::Context) {
        gpu.destroy_render_pipeline(&mut self.composite);
        gpu.destroy_render_pipeline(&mut self.graphite_composite);
        gpu.destroy_render_pipeline(&mut self.solid_composite);
        gpu.destroy_render_pipeline(&mut self.quads);
        gpu.destroy_render_pipeline(&mut self.shadows);
        gpu.destroy_render_pipeline(&mut self.path_rasterization);
        gpu.destroy_render_pipeline(&mut self.paths);
        gpu.destroy_render_pipeline(&mut self.underlines);
        gpu.destroy_render_pipeline(&mut self.mono_sprites);
        gpu.destroy_render_pipeline(&mut self.poly_sprites);
        gpu.destroy_render_pipeline(&mut self.surfaces);
    }
}

enum ClipStencilMode {
    Write,
    Test,
    Bypass,
}

fn clip_depth_stencil(mode: ClipStencilMode) -> gpu::DepthStencilState {
    let face = gpu::StencilFaceState {
        compare: if matches!(mode, ClipStencilMode::Test) {
            gpu::CompareFunction::Equal
        } else {
            gpu::CompareFunction::Always
        },
        pass_op: if matches!(mode, ClipStencilMode::Write) {
            gpu::StencilOperation::Replace
        } else {
            gpu::StencilOperation::Keep
        },
        ..Default::default()
    };
    gpu::DepthStencilState {
        format: gpu::TextureFormat::Depth32FloatStencil8Uint,
        depth_write_enabled: false,
        depth_compare: gpu::CompareFunction::Always,
        stencil: gpu::StencilState {
            front: face,
            back: face,
            read_mask: 255,
            write_mask: if matches!(mode, ClipStencilMode::Write) {
                255
            } else {
                0
            },
        },
        bias: Default::default(),
    }
}

struct BladeClipPipelines {
    test: BladePipelines,
    bypass: BladePipelines,
    mask: gpu::RenderPipeline,
}

impl BladeClipPipelines {
    fn new(gpu: &gpu::Context, info: gpu::SurfaceInfo, path_samples: u32) -> Self {
        use gpu::ShaderData as _;
        let info = gpu::SurfaceInfo {
            alpha: gpu::AlphaMode::PreMultiplied,
            ..info
        };
        let shader = gpu.create_shader(gpu::ShaderDesc {
            source: include_str!("shaders.wgsl"),
        });
        let mask = gpu.create_render_pipeline(gpu::RenderPipelineDesc {
            name: "conic clip stencil",
            data_layouts: &[&ShaderClipData::layout()],
            vertex: shader.at("vs_clip"),
            vertex_fetches: &[],
            primitive: gpu::PrimitiveState {
                topology: gpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: Some(clip_depth_stencil(ClipStencilMode::Write)),
            fragment: Some(shader.at("fs_clip")),
            color_targets: &[gpu::ColorTargetState {
                format: info.format,
                blend: None,
                write_mask: gpu::ColorWrites::empty(),
            }],
            multisample_state: gpu::MultisampleState {
                sample_count: 4,
                ..Default::default()
            },
        });
        Self {
            test: BladePipelines::with_clip(gpu, info, path_samples, Some(true)),
            bypass: BladePipelines::with_clip(gpu, info, path_samples, Some(false)),
            mask,
        }
    }

    fn destroy(&mut self, gpu: &gpu::Context) {
        self.test.destroy(gpu);
        self.bypass.destroy(gpu);
        gpu.destroy_render_pipeline(&mut self.mask);
    }
}

fn graphite_clip_pass(composite: &crate::Composite) -> bool {
    #[cfg(target_os = "macos")]
    {
        return composite.requires_graphite_clip_pass();
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = composite;
        false
    }
}

fn graphite_clip_samples(composite: Option<&crate::Composite>) -> bool {
    #[cfg(target_os = "macos")]
    {
        return composite.is_some_and(|clip| !clip.can_fold_all_clip_draws());
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = composite;
        false
    }
}

fn graphite_clip_vertices(composite: &crate::Composite) -> Vec<[f32; 2]> {
    #[cfg(target_os = "macos")]
    {
        return composite
            .graphite_clip_mesh()
            .into_iter()
            .map(|p| [p.x.0, p.y.0])
            .collect();
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = composite;
        unreachable!("Graphite clip pass is macOS-only")
    }
}

fn needs_graphite_clipping(scene: &Scene) -> bool {
    scene
        .composites
        .iter()
        .any(|c| graphite_clip_pass(c) || needs_graphite_clipping(&c.scene))
}

fn prepare_clip_draws<'a>(
    batch: PrimitiveBatch<'a>,
    clip: Option<&crate::Composite>,
) -> SmallVec<[ClipDraw<'a>; 1]> {
    let mut result = SmallVec::new();
    #[cfg(target_os = "macos")]
    if let Some(clip) = clip {
        match batch {
            PrimitiveBatch::Quads(quads) => {
                let mut start = 0;
                for (i, q) in quads.iter().enumerate() {
                    let folded = (q.corner_radii == Default::default()
                        && q.border_widths == Default::default())
                    .then(|| {
                        clip.fold_clip_into(
                            q.bounds.intersect(&q.content_mask.bounds),
                            q.clone().into(),
                        )
                    })
                    .flatten();
                    if let Some(folded) = folded {
                        if i > start {
                            result
                                .push(ClipDraw::Original(PrimitiveBatch::Quads(&quads[start..i])));
                        }
                        result.push(ClipDraw::Folded(folded));
                        start = i + 1;
                    }
                }
                if start < quads.len() {
                    result.push(ClipDraw::Original(PrimitiveBatch::Quads(&quads[start..])));
                }
                return result;
            }
            PrimitiveBatch::PolychromeSprites {
                texture_id,
                sprites,
            } => {
                let mut start = 0;
                for (i, s) in sprites.iter().enumerate() {
                    let folded = (s.corner_radii == Default::default())
                        .then(|| {
                            clip.fold_clip_into(
                                s.bounds.intersect(&s.content_mask.bounds),
                                s.clone().into(),
                            )
                        })
                        .flatten();
                    if let Some(folded) = folded {
                        if i > start {
                            result.push(ClipDraw::Original(PrimitiveBatch::PolychromeSprites {
                                texture_id,
                                sprites: &sprites[start..i],
                            }));
                        }
                        result.push(ClipDraw::Folded(folded));
                        start = i + 1;
                    }
                }
                if start < sprites.len() {
                    result.push(ClipDraw::Original(PrimitiveBatch::PolychromeSprites {
                        texture_id,
                        sprites: &sprites[start..],
                    }));
                }
                return result;
            }
            PrimitiveBatch::Composite(c)
                if c.corner_radii == Default::default()
                    && c.corner_radii_y == Default::default()
                    && c.transformation.rotation_scale[0][1] == 0.
                    && c.transformation.rotation_scale[1][0] == 0. =>
            {
                if let Some(folded) = clip.fold_clip_into(
                    c.painted_bounds.intersect(&c.content_mask.bounds),
                    c.clone().into(),
                ) {
                    result.push(ClipDraw::Folded(folded));
                    return result;
                }
            }
            _ => {}
        }
    }
    #[cfg(not(target_os = "macos"))]
    let _ = clip;
    result.push(ClipDraw::Original(batch));
    result
}

pub struct BladeSurfaceConfig {
    pub size: gpu::Extent,
    pub transparent: bool,
}

//Note: we could see some of these fields moved into `BladeContext`
// so that they are shared between windows. E.g. `pipelines`.
// But that is complicated by the fact that pipelines depend on
// the format and alpha mode.
pub struct BladeRenderer {
    gpu: Arc<gpu::Context>,
    surface: Option<gpu::Surface>,
    surface_info: gpu::SurfaceInfo,
    surface_config: gpu::SurfaceConfig,
    command_encoder: gpu::CommandEncoder,
    last_sync_point: Option<gpu::SyncPoint>,
    pipelines: BladePipelines,
    offscreen_pipelines: BladePipelines,
    composite_targets: Vec<CompositeTarget>,
    clip_targets: Vec<Option<ClipTarget>>,
    clip_pipelines: Option<BladeClipPipelines>,
    instance_belt: BufferBelt,
    atlas: Arc<BladeAtlas>,
    atlas_sampler: gpu::Sampler,
    #[cfg(target_os = "macos")]
    core_video_texture_cache: CVMetalTextureCache,
    intermediate_size: gpu::Extent,
    path_intermediate_texture: gpu::Texture,
    path_intermediate_texture_view: gpu::TextureView,
    path_intermediate_msaa_texture: Option<gpu::Texture>,
    path_intermediate_msaa_texture_view: Option<gpu::TextureView>,
    rendering_parameters: RenderingParameters,
}

impl BladeRenderer {
    pub fn new<I: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        context: &BladeContext,
        window: &I,
        config: BladeSurfaceConfig,
    ) -> anyhow::Result<Self> {
        let surface_config = gpu::SurfaceConfig {
            size: config.size,
            usage: gpu::TextureUsage::TARGET,
            display_sync: gpu::DisplaySync::Recent,
            color_space: gpu::ColorSpace::Srgb,
            allow_exclusive_full_screen: false,
            transparent: config.transparent,
        };
        let surface = context
            .gpu
            .create_surface_configured(window, surface_config)
            .map_err(|err| anyhow::anyhow!("Failed to create surface: {err:?}"))?;

        let info = surface.info();
        Ok(Self::with_surface(
            context,
            Some(surface),
            info,
            surface_config,
        ))
    }

    fn with_surface(
        context: &BladeContext,
        surface: Option<gpu::Surface>,
        surface_info: gpu::SurfaceInfo,
        surface_config: gpu::SurfaceConfig,
    ) -> Self {
        let command_encoder = context.gpu.create_command_encoder(gpu::CommandEncoderDesc {
            name: "main",
            buffer_count: 2,
        });
        let rendering_parameters = RenderingParameters::from_env(context);
        let pipelines = BladePipelines::new(
            &context.gpu,
            surface_info,
            rendering_parameters.path_sample_count,
        );
        let instance_belt = BufferBelt::new(BufferBeltDescriptor {
            memory: gpu::Memory::Shared,
            min_chunk_size: 0x1000,
            alignment: 0x40, // Vulkan `minStorageBufferOffsetAlignment` on Intel Xe
        });
        let atlas = Arc::new(BladeAtlas::new(&context.gpu));
        let atlas_sampler = context.gpu.create_sampler(gpu::SamplerDesc {
            name: "path rasterization sampler",
            mag_filter: gpu::FilterMode::Linear,
            min_filter: gpu::FilterMode::Linear,
            ..Default::default()
        });

        let (path_intermediate_texture, path_intermediate_texture_view) =
            create_path_intermediate_texture(
                &context.gpu,
                surface_info.format,
                surface_config.size.width,
                surface_config.size.height,
            );
        let (path_intermediate_msaa_texture, path_intermediate_msaa_texture_view) =
            create_msaa_texture_if_needed(
                &context.gpu,
                surface_info.format,
                surface_config.size.width,
                surface_config.size.height,
                rendering_parameters.path_sample_count,
            )
            .unzip();

        #[cfg(target_os = "macos")]
        let core_video_texture_cache = unsafe {
            CVMetalTextureCache::new(
                objc2::rc::Retained::as_ptr(&context.gpu.metal_device()) as *mut _
            )
            .unwrap()
        };

        let offscreen_pipelines = BladePipelines::new(
            &context.gpu,
            gpu::SurfaceInfo {
                alpha: gpu::AlphaMode::PreMultiplied,
                ..surface_info
            },
            rendering_parameters.path_sample_count,
        );
        Self {
            gpu: Arc::clone(&context.gpu),
            surface,
            surface_info,
            surface_config,
            offscreen_pipelines,
            composite_targets: Vec::new(),
            clip_targets: Vec::new(),
            clip_pipelines: None,
            command_encoder,
            last_sync_point: None,
            pipelines,
            instance_belt,
            atlas,
            atlas_sampler,
            #[cfg(target_os = "macos")]
            core_video_texture_cache,
            intermediate_size: surface_config.size,
            path_intermediate_texture,
            path_intermediate_texture_view,
            path_intermediate_msaa_texture,
            path_intermediate_msaa_texture_view,
            rendering_parameters,
        }
    }

    fn wait_for_gpu(&mut self) {
        if let Some(last_sp) = self.last_sync_point.take()
            && !self.gpu.wait_for(&last_sp, MAX_FRAME_TIME_MS)
        {
            log::error!("GPU hung");
            #[cfg(target_os = "linux")]
            if self.gpu.device_information().driver_name == "radv" {
                log::error!(
                    "there's a known bug with amdgpu/radv, try setting ZED_PATH_SAMPLE_COUNT=0 as a workaround"
                );
                log::error!(
                    "if that helps you're running into https://github.com/zed-industries/zed/issues/26143"
                );
            }
            log::error!(
                "your device information is: {:?}",
                self.gpu.device_information()
            );
            while !self.gpu.wait_for(&last_sp, MAX_FRAME_TIME_MS) {}
        }
    }

    pub fn update_drawable_size(&mut self, size: Size<DevicePixels>) {
        self.update_drawable_size_impl(size, false);
    }

    /// Like `update_drawable_size` but skips the check that the size has changed. This is useful in
    /// cases like restoring a window from minimization where the size is the same but the
    /// renderer's swap chain needs to be recreated.
    #[cfg_attr(
        any(target_os = "macos", target_os = "linux", target_os = "freebsd"),
        allow(dead_code)
    )]
    pub fn update_drawable_size_even_if_unchanged(&mut self, size: Size<DevicePixels>) {
        self.update_drawable_size_impl(size, true);
    }

    fn update_drawable_size_impl(&mut self, size: Size<DevicePixels>, always_resize: bool) {
        let gpu_size = gpu::Extent {
            width: size.width.0 as u32,
            height: size.height.0 as u32,
            depth: 1,
        };

        if always_resize || gpu_size != self.surface_config.size {
            self.wait_for_gpu();
            self.surface_config.size = gpu_size;
            if let Some(surface) = &mut self.surface {
                self.gpu.reconfigure_surface(surface, self.surface_config);
                self.surface_info = surface.info();
            }
            self.update_intermediate_textures(gpu_size);
        }
    }

    fn update_intermediate_textures(&mut self, gpu_size: gpu::Extent) {
        self.intermediate_size = gpu_size;
        self.gpu.destroy_texture(self.path_intermediate_texture);
        self.gpu
            .destroy_texture_view(self.path_intermediate_texture_view);
        if let Some(msaa_texture) = self.path_intermediate_msaa_texture {
            self.gpu.destroy_texture(msaa_texture);
        }
        if let Some(msaa_view) = self.path_intermediate_msaa_texture_view {
            self.gpu.destroy_texture_view(msaa_view);
        }
        let (path_intermediate_texture, path_intermediate_texture_view) =
            create_path_intermediate_texture(
                &self.gpu,
                self.surface_info.format,
                gpu_size.width,
                gpu_size.height,
            );
        self.path_intermediate_texture = path_intermediate_texture;
        self.path_intermediate_texture_view = path_intermediate_texture_view;
        let (path_intermediate_msaa_texture, path_intermediate_msaa_texture_view) =
            create_msaa_texture_if_needed(
                &self.gpu,
                self.surface_info.format,
                gpu_size.width,
                gpu_size.height,
                self.rendering_parameters.path_sample_count,
            )
            .unzip();
        self.path_intermediate_msaa_texture = path_intermediate_msaa_texture;
        self.path_intermediate_msaa_texture_view = path_intermediate_msaa_texture_view;
    }

    fn prepare_scene(&mut self, scene: &Scene) {
        if cfg!(target_os = "macos")
            && self.clip_pipelines.is_none()
            && (self.gpu.capabilities().sample_count_mask & 4) != 0
            && needs_graphite_clipping(scene)
        {
            self.clip_pipelines = Some(BladeClipPipelines::new(
                &self.gpu,
                self.surface_info,
                self.rendering_parameters.path_sample_count,
            ));
        }
        let size = scene.intermediate_size(crate::size(
            DevicePixels(self.surface_config.size.width as i32),
            DevicePixels(self.surface_config.size.height as i32),
        ));
        let extent = gpu::Extent {
            width: size.width.0 as u32,
            height: size.height.0 as u32,
            depth: 1,
        };
        if extent != self.intermediate_size {
            self.wait_for_gpu();
            self.clear_composite_targets();
            self.update_intermediate_textures(extent);
        }
    }

    pub fn update_transparency(&mut self, transparent: bool) {
        if transparent != self.surface_config.transparent {
            self.wait_for_gpu();
            self.surface_config.transparent = transparent;
            if let Some(surface) = &mut self.surface {
                self.gpu.reconfigure_surface(surface, self.surface_config);
                self.surface_info = surface.info();
            }
            self.clear_composite_targets();
            if let Some(mut pipelines) = self.clip_pipelines.take() {
                pipelines.destroy(&self.gpu);
            }
            self.offscreen_pipelines.destroy(&self.gpu);
            self.offscreen_pipelines = BladePipelines::new(
                &self.gpu,
                gpu::SurfaceInfo {
                    alpha: gpu::AlphaMode::PreMultiplied,
                    ..self.surface_info
                },
                self.rendering_parameters.path_sample_count,
            );
            self.pipelines.destroy(&self.gpu);
            self.pipelines = BladePipelines::new(
                &self.gpu,
                self.surface_info,
                self.rendering_parameters.path_sample_count,
            );
        }
    }

    #[cfg_attr(
        any(target_os = "macos", feature = "wayland", target_os = "windows"),
        allow(dead_code)
    )]
    pub fn viewport_size(&self) -> gpu::Extent {
        self.surface_config.size
    }

    pub fn sprite_atlas(&self) -> &Arc<BladeAtlas> {
        &self.atlas
    }

    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub fn gpu_specs(&self) -> GpuSpecs {
        let info = self.gpu.device_information();

        GpuSpecs {
            is_software_emulated: info.is_software_emulated,
            device_name: info.device_name.clone(),
            driver_name: info.driver_name.clone(),
            driver_info: info.driver_info.clone(),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn layer(&self) -> metal::MetalLayer {
        unsafe { foreign_types::ForeignType::from_ptr(self.layer_ptr()) }
    }

    #[cfg(target_os = "macos")]
    pub fn layer_ptr(&self) -> *mut metal::CAMetalLayer {
        objc2::rc::Retained::as_ptr(&self.surface.as_ref().unwrap().metal_layer()) as *mut _
    }

    #[profiling::function]
    fn draw_paths_to_intermediate(
        &mut self,
        paths: &[Path<ScaledPixels>],
        width: f32,
        height: f32,
    ) {
        self.command_encoder
            .init_texture(self.path_intermediate_texture);
        if let Some(msaa_texture) = self.path_intermediate_msaa_texture {
            self.command_encoder.init_texture(msaa_texture);
        }

        let target = if let Some(msaa_view) = self.path_intermediate_msaa_texture_view {
            gpu::RenderTarget {
                view: msaa_view,
                init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                finish_op: gpu::FinishOp::ResolveTo(self.path_intermediate_texture_view),
            }
        } else {
            gpu::RenderTarget {
                view: self.path_intermediate_texture_view,
                init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                finish_op: gpu::FinishOp::Store,
            }
        };
        if let mut pass = self.command_encoder.render(
            "rasterize paths",
            gpu::RenderTargetSet {
                colors: &[target],
                depth_stencil: None,
            },
        ) {
            let globals = GlobalParams {
                viewport_size: [width, height],
                premultiplied_alpha: 0,
                pad: 0,
            };
            let mut encoder = pass.with(&self.pipelines.path_rasterization);

            let mut vertices = Vec::new();
            for path in paths {
                vertices.extend(path.vertices.iter().map(|v| PathRasterizationVertex {
                    xy_position: v.xy_position,
                    st_position: v.st_position,
                    color: path.color,
                    bounds: path.clipped_bounds(),
                }));
            }
            let vertex_buf = unsafe { self.instance_belt.alloc_typed(&vertices, &self.gpu) };
            encoder.bind(
                0,
                &ShaderPathRasterizationData {
                    globals,
                    b_path_vertices: vertex_buf,
                },
            );
            encoder.draw(0, vertices.len() as u32, 0, 1);
        }
    }

    pub fn destroy(&mut self) {
        self.wait_for_gpu();
        self.atlas.destroy();
        self.gpu.destroy_sampler(self.atlas_sampler);
        self.instance_belt.destroy(&self.gpu);
        self.gpu.destroy_command_encoder(&mut self.command_encoder);
        if let Some(mut pipelines) = self.clip_pipelines.take() {
            pipelines.destroy(&self.gpu);
        }
        self.pipelines.destroy(&self.gpu);
        self.offscreen_pipelines.destroy(&self.gpu);
        self.clear_composite_targets();
        if let Some(surface) = &mut self.surface {
            self.gpu.destroy_surface(surface);
        }
        self.gpu.destroy_texture(self.path_intermediate_texture);
        self.gpu
            .destroy_texture_view(self.path_intermediate_texture_view);
        if let Some(msaa_texture) = self.path_intermediate_msaa_texture {
            self.gpu.destroy_texture(msaa_texture);
        }
        if let Some(msaa_view) = self.path_intermediate_msaa_texture_view {
            self.gpu.destroy_texture_view(msaa_view);
        }
    }

    /// Render the same scene encoder to an app-owned texture, without
    /// compositor overlays or a screen-capture permission.
    pub fn capture_scene(&mut self, scene: &Scene) -> anyhow::Result<image::RgbaImage> {
        let size = self.surface_config.size;
        anyhow::ensure!(size.width > 0 && size.height > 0, "empty scene capture");
        anyhow::ensure!(
            matches!(
                self.surface_info.format,
                gpu::TextureFormat::Rgba8Unorm | gpu::TextureFormat::Bgra8Unorm
            ),
            "unsupported capture format: {:?}",
            self.surface_info.format
        );
        self.wait_for_gpu();
        let (target, view) = create_path_intermediate_texture(
            &self.gpu,
            self.surface_info.format,
            size.width,
            size.height,
        );
        let stride = (size.width * 4 + 255) & !255;
        let readback = self.gpu.create_buffer(gpu::BufferDesc {
            name: "scene capture readback",
            size: u64::from(stride) * u64::from(size.height),
            memory: gpu::Memory::Shared,
        });
        self.prepare_scene(scene);
        self.command_encoder.start();
        self.atlas.before_frame(&mut self.command_encoder);
        self.command_encoder.init_texture(target);
        self.encode_scene(
            scene,
            view,
            self.surface_info.alpha == gpu::AlphaMode::PreMultiplied,
            0,
            self.surface_config.size,
            None,
        );
        self.command_encoder
            .transfer("read scene capture")
            .copy_texture_to_buffer(target.into(), readback.into(), stride, size);
        let sync = self.gpu.submit(&mut self.command_encoder);
        self.instance_belt.flush(&sync);
        self.atlas.after_frame(&sync);
        let complete = self.gpu.wait_for(&sync, MAX_FRAME_TIME_MS);
        // Keep every referenced resource alive until a submitted readback
        // completes, even when the diagnostic timeout is exceeded.
        if !complete {
            log::error!("scene capture exceeded GPU timeout");
            while !self.gpu.wait_for(&sync, MAX_FRAME_TIME_MS) {}
        }
        let bytes = unsafe {
            std::slice::from_raw_parts(readback.data(), stride as usize * size.height as usize)
        };
        let bgra = self.surface_info.format == gpu::TextureFormat::Bgra8Unorm;
        let mut pixels = Vec::with_capacity((size.width * size.height * 4) as usize);
        for row in bytes.chunks_exact(stride as usize) {
            for source in row[..size.width as usize * 4].chunks_exact(4) {
                let mut pixel = [source[0], source[1], source[2], source[3]];
                if bgra {
                    pixel.swap(0, 2);
                }
                let alpha = u32::from(pixel[3]);
                for channel in &mut pixel[..3] {
                    *channel = if alpha == 0 {
                        0
                    } else {
                        ((u32::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8
                    };
                }
                pixels.extend_from_slice(&pixel);
            }
        }
        self.gpu.destroy_buffer(readback);
        self.gpu.destroy_texture_view(view);
        self.gpu.destroy_texture(target);
        anyhow::ensure!(complete, "scene capture GPU timeout");
        image::RgbaImage::from_raw(size.width, size.height, pixels)
            .ok_or_else(|| anyhow::anyhow!("invalid scene capture dimensions"))
    }

    fn clear_composite_targets(&mut self) {
        for target in self.clip_targets.drain(..).flatten() {
            self.gpu.destroy_texture_view(target.color_view);
            self.gpu.destroy_texture(target.color);
            self.gpu.destroy_texture_view(target.stencil_view);
            self.gpu.destroy_texture(target.stencil);
        }
        for target in self.composite_targets.drain(..) {
            self.gpu.destroy_texture_view(target.view);
            self.gpu.destroy_texture(target.texture);
        }
    }

    fn composite_target(&mut self, depth: usize) -> gpu::TextureView {
        let size = self.intermediate_size;
        if self
            .composite_targets
            .get(depth)
            .is_some_and(|target| target.size != size)
        {
            self.wait_for_gpu();
            self.clear_composite_targets();
        }
        if depth == self.composite_targets.len() {
            let (texture, view) = create_path_intermediate_texture(
                &self.gpu,
                self.surface_info.format,
                size.width,
                size.height,
            );
            // Initialize only a newly allocated image. Reused images stay
            // in GENERAL layout; render-pass barriers order sibling reads
            // before the next clear without discarding them prematurely.
            self.command_encoder.init_texture(texture);
            self.composite_targets.push(CompositeTarget {
                texture,
                view,
                size,
            });
        }
        self.composite_targets[depth].view
    }

    fn clip_target(&mut self, depth: usize) -> ClipTarget {
        let size = self.intermediate_size;
        if self.clip_targets.len() <= depth {
            self.clip_targets.resize_with(depth + 1, || None);
        }
        if let Some(target) = self.clip_targets[depth] {
            assert_eq!(
                target.size, size,
                "clip targets must be resized before recording the frame"
            );
            return target;
        }
        let (color, color_view) = create_msaa_texture_if_needed(
            &self.gpu,
            self.surface_info.format,
            size.width,
            size.height,
            4,
        )
        .unwrap();
        let (stencil, stencil_view) = create_msaa_texture_if_needed(
            &self.gpu,
            gpu::TextureFormat::Depth32FloatStencil8Uint,
            size.width,
            size.height,
            4,
        )
        .unwrap();
        self.command_encoder.init_texture(color);
        self.command_encoder.init_texture(stencil);
        let target = ClipTarget {
            color,
            color_view,
            stencil,
            stencil_view,
            size,
        };
        self.clip_targets[depth] = Some(target);
        target
    }

    fn encode_scene(
        &mut self,
        scene: &Scene,
        target: gpu::TextureView,
        premultiplied: bool,
        depth: usize,
        viewport: gpu::Extent,
        clip: Option<&crate::Composite>,
    ) {
        let globals = GlobalParams {
            viewport_size: [viewport.width as f32, viewport.height as f32],
            premultiplied_alpha: u32::from(premultiplied),
            pad: 0,
        };

        let multisampled = graphite_clip_samples(clip) && self.clip_pipelines.is_some();
        let clip_target = multisampled.then(|| self.clip_target(depth));
        let color_view = clip_target.map_or(target, |t| t.color_view);
        let clip_mesh = clip.filter(|_| multisampled).map(graphite_clip_vertices);
        let clip_buffer = clip_mesh
            .as_ref()
            .map(|v| unsafe { self.instance_belt.alloc_typed(v, &self.gpu) });
        let mut pass = self.command_encoder.render(
            "main",
            gpu::RenderTargetSet {
                colors: &[gpu::RenderTarget {
                    view: color_view,
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                    finish_op: gpu::FinishOp::Store,
                }],
                depth_stencil: clip_target.map(|t| gpu::RenderTarget {
                    view: t.stencil_view,
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                    finish_op: gpu::FinishOp::Store,
                }),
            },
        );

        pass.set_stencil_reference(1);
        if let Some(buffer) = clip_buffer {
            let mut encoder = pass.with(&self.clip_pipelines.as_ref().unwrap().mask);
            encoder.bind(
                0,
                &ShaderClipData {
                    globals,
                    b_clip_vertices: buffer,
                },
            );
            encoder.draw(0, clip_mesh.as_ref().unwrap().len() as u32, 0, 1);
        }
        profiling::scope!("render pass");
        for draw in scene
            .batches()
            .flat_map(|batch| prepare_clip_draws(batch, clip))
        {
            let (batch, bypass) = match draw {
                ClipDraw::Original(batch) => (batch, false),
                ClipDraw::Folded(ref composite) => (PrimitiveBatch::Composite(composite), true),
            };
            let mode = if multisampled {
                if bypass {
                    ScenePipelineMode::ClipBypass
                } else {
                    ScenePipelineMode::ClipTest
                }
            } else if premultiplied {
                ScenePipelineMode::Premultiplied
            } else {
                ScenePipelineMode::Root
            };
            match batch {
                PrimitiveBatch::Quads(quads) => {
                    let instance_buf = unsafe { self.instance_belt.alloc_typed(quads, &self.gpu) };
                    let mut encoder = pass.with(&scene_pipelines!(self, mode).quads);
                    encoder.bind(
                        0,
                        &ShaderQuadsData {
                            globals,
                            b_quads: instance_buf,
                        },
                    );
                    encoder.draw(0, 4, 0, quads.len() as u32);
                }
                PrimitiveBatch::Shadows(shadows) => {
                    let instance_buf =
                        unsafe { self.instance_belt.alloc_typed(shadows, &self.gpu) };
                    let mut encoder = pass.with(&scene_pipelines!(self, mode).shadows);
                    encoder.bind(
                        0,
                        &ShaderShadowsData {
                            globals,
                            b_shadows: instance_buf,
                        },
                    );
                    encoder.draw(0, 4, 0, shadows.len() as u32);
                }
                PrimitiveBatch::Paths(paths) => {
                    let Some(first_path) = paths.first() else {
                        continue;
                    };
                    drop(pass);
                    self.draw_paths_to_intermediate(
                        paths,
                        self.intermediate_size.width as f32,
                        self.intermediate_size.height as f32,
                    );
                    pass = self.command_encoder.render(
                        "main",
                        gpu::RenderTargetSet {
                            colors: &[gpu::RenderTarget {
                                view: color_view,
                                init_op: gpu::InitOp::Load,
                                finish_op: gpu::FinishOp::Store,
                            }],
                            depth_stencil: clip_target.map(|t| gpu::RenderTarget {
                                view: t.stencil_view,
                                init_op: gpu::InitOp::Load,
                                finish_op: gpu::FinishOp::Store,
                            }),
                        },
                    );
                    pass.set_stencil_reference(1);
                    let mut encoder = pass.with(&scene_pipelines!(self, mode).paths);
                    // When copying paths from the intermediate texture to the drawable,
                    // each pixel must only be copied once, in case of transparent paths.
                    //
                    // If all paths have the same draw order, then their bounds are all
                    // disjoint, so we can copy each path's bounds individually. If this
                    // batch combines different draw orders, we perform a single copy
                    // for a minimal spanning rect.
                    let sprites = if paths.last().unwrap().order == first_path.order {
                        paths
                            .iter()
                            .map(|path| PathSprite {
                                bounds: path.clipped_bounds(),
                            })
                            .collect()
                    } else {
                        let mut bounds = first_path.clipped_bounds();
                        for path in paths.iter().skip(1) {
                            bounds = bounds.union(&path.clipped_bounds());
                        }
                        vec![PathSprite { bounds }]
                    };
                    let instance_buf =
                        unsafe { self.instance_belt.alloc_typed(&sprites, &self.gpu) };
                    encoder.bind(
                        0,
                        &ShaderPathsData {
                            globals,
                            t_sprite: self.path_intermediate_texture_view,
                            s_sprite: self.atlas_sampler,
                            b_path_sprites: instance_buf,
                        },
                    );
                    encoder.draw(0, 4, 0, sprites.len() as u32);
                }
                PrimitiveBatch::Underlines(underlines) => {
                    let instance_buf =
                        unsafe { self.instance_belt.alloc_typed(underlines, &self.gpu) };
                    let mut encoder = pass.with(&scene_pipelines!(self, mode).underlines);
                    encoder.bind(
                        0,
                        &ShaderUnderlinesData {
                            globals,
                            b_underlines: instance_buf,
                        },
                    );
                    encoder.draw(0, 4, 0, underlines.len() as u32);
                }
                PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    sprites,
                } => {
                    let tex_info = self.atlas.get_texture_info(texture_id);
                    let instance_buf =
                        unsafe { self.instance_belt.alloc_typed(sprites, &self.gpu) };
                    let mut encoder = pass.with(&scene_pipelines!(self, mode).mono_sprites);
                    encoder.bind(
                        0,
                        &ShaderMonoSpritesData {
                            globals,
                            gamma_ratios: self.rendering_parameters.gamma_ratios,
                            grayscale_enhanced_contrast: self
                                .rendering_parameters
                                .grayscale_enhanced_contrast,
                            t_sprite: tex_info.raw_view,
                            s_sprite: self.atlas_sampler,
                            b_mono_sprites: instance_buf,
                        },
                    );
                    encoder.draw(0, 4, 0, sprites.len() as u32);
                }
                PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites,
                } => {
                    let tex_info = self.atlas.get_texture_info(texture_id);
                    let instance_buf =
                        unsafe { self.instance_belt.alloc_typed(sprites, &self.gpu) };
                    let mut encoder = pass.with(&scene_pipelines!(self, mode).poly_sprites);
                    encoder.bind(
                        0,
                        &ShaderPolySpritesData {
                            globals,
                            t_sprite: tex_info.raw_view,
                            s_sprite: self.atlas_sampler,
                            b_poly_sprites: instance_buf,
                        },
                    );
                    encoder.draw(0, 4, 0, sprites.len() as u32);
                }
                PrimitiveBatch::Composite(composite) => {
                    if let Some(vertices) = composite.solid_rrect_mesh() {
                        let buffer =
                            unsafe { self.instance_belt.alloc_typed(&vertices, &self.gpu) };
                        let mut encoder = pass.with(&scene_pipelines!(self, mode).solid_composite);
                        encoder.bind(
                            0,
                            &ShaderSolidCompositeData {
                                globals,
                                b_solid_rrect_vertices: buffer,
                            },
                        );
                        encoder.draw(0, vertices.len() as u32, 0, 1);
                        continue;
                    }
                    drop(pass);
                    let view = self.composite_target(depth);
                    let clip_applied =
                        self.clip_pipelines.is_some() && graphite_clip_pass(composite);
                    self.encode_scene(
                        &composite.scene,
                        view,
                        true,
                        depth + 1,
                        self.intermediate_size,
                        clip_applied.then_some(composite),
                    );
                    pass = self.command_encoder.render(
                        "resume composite parent",
                        gpu::RenderTargetSet {
                            colors: &[gpu::RenderTarget {
                                view: color_view,
                                init_op: gpu::InitOp::Load,
                                finish_op: gpu::FinishOp::Store,
                            }],
                            depth_stencil: clip_target.map(|t| gpu::RenderTarget {
                                view: t.stencil_view,
                                init_op: gpu::InitOp::Load,
                                finish_op: gpu::FinishOp::Store,
                            }),
                        },
                    );
                    pass.set_stencil_reference(1);
                    let data = ShaderCompositeData {
                        globals,
                        composite: CompositeParams {
                            bounds: composite.bounds.into(),
                            content_mask: composite.content_mask.bounds.into(),
                            corner_radii: if clip_applied {
                                [0.; 4]
                            } else {
                                [
                                    composite.corner_radii.top_left.0,
                                    composite.corner_radii.top_right.0,
                                    composite.corner_radii.bottom_right.0,
                                    composite.corner_radii.bottom_left.0,
                                ]
                            },
                            corner_radii_y: if clip_applied {
                                [0.; 4]
                            } else {
                                [
                                    composite.corner_radii_y.top_left.0,
                                    composite.corner_radii_y.top_right.0,
                                    composite.corner_radii_y.bottom_right.0,
                                    composite.corner_radii_y.bottom_left.0,
                                ]
                            },
                            source_origin: [
                                composite.source_origin.x.0,
                                composite.source_origin.y.0,
                            ],
                            source_pad: [0.; 2],
                            opacity: composite.opacity,
                            pad: [0.; 3],
                            transform_row0: [
                                composite.transformation.rotation_scale[0][0],
                                composite.transformation.rotation_scale[0][1],
                                composite.transformation.translation[0],
                                0.,
                            ],
                            transform_row1: [
                                composite.transformation.rotation_scale[1][0],
                                composite.transformation.rotation_scale[1][1],
                                composite.transformation.translation[1],
                                0.,
                            ],
                        },
                        t_sprite: view,
                        s_sprite: self.atlas_sampler,
                    };
                    if let Some(vertices) = (!clip_applied)
                        .then(|| composite.graphite_rrect_mesh())
                        .flatten()
                    {
                        let buffer =
                            unsafe { self.instance_belt.alloc_typed(&vertices, &self.gpu) };
                        let mut encoder =
                            pass.with(&scene_pipelines!(self, mode).graphite_composite);
                        encoder.bind(
                            0,
                            &ShaderGraphiteCompositeData {
                                globals: data.globals,
                                composite: data.composite,
                                t_sprite: data.t_sprite,
                                s_sprite: data.s_sprite,
                                b_graphite_rrect_vertices: buffer,
                            },
                        );
                        encoder.draw(0, vertices.len() as u32, 0, 1);
                    } else {
                        let mut encoder = pass.with(&scene_pipelines!(self, mode).composite);
                        encoder.bind(0, &data);
                        encoder.draw(0, 4, 0, 1);
                    }
                }
                PrimitiveBatch::Surfaces(surfaces) => {
                    let mut _encoder = pass.with(&scene_pipelines!(self, mode).surfaces);

                    for surface in surfaces {
                        #[cfg(not(target_os = "macos"))]
                        {
                            let _ = surface;
                            continue;
                        };

                        #[cfg(target_os = "macos")]
                        {
                            let (t_y, t_cb_cr) = unsafe {
                                use core_foundation::base::TCFType as _;
                                use std::ptr;

                                assert_eq!(
                                        surface.image_buffer.get_pixel_format(),
                                        core_video::pixel_buffer::kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
                                    );

                                let y_texture = self
                                    .core_video_texture_cache
                                    .create_texture_from_image(
                                        surface.image_buffer.as_concrete_TypeRef(),
                                        ptr::null(),
                                        metal::MTLPixelFormat::R8Unorm,
                                        surface.image_buffer.get_width_of_plane(0),
                                        surface.image_buffer.get_height_of_plane(0),
                                        0,
                                    )
                                    .unwrap();
                                let cb_cr_texture = self
                                    .core_video_texture_cache
                                    .create_texture_from_image(
                                        surface.image_buffer.as_concrete_TypeRef(),
                                        ptr::null(),
                                        metal::MTLPixelFormat::RG8Unorm,
                                        surface.image_buffer.get_width_of_plane(1),
                                        surface.image_buffer.get_height_of_plane(1),
                                        1,
                                    )
                                    .unwrap();
                                (
                                    gpu::TextureView::from_metal_texture(
                                        &objc2::rc::Retained::retain(
                                            foreign_types::ForeignTypeRef::as_ptr(
                                                y_texture.as_texture_ref(),
                                            )
                                                as *mut objc2::runtime::ProtocolObject<
                                                    dyn objc2_metal::MTLTexture,
                                                >,
                                        )
                                        .unwrap(),
                                        gpu::TexelAspects::COLOR,
                                    ),
                                    gpu::TextureView::from_metal_texture(
                                        &objc2::rc::Retained::retain(
                                            foreign_types::ForeignTypeRef::as_ptr(
                                                cb_cr_texture.as_texture_ref(),
                                            )
                                                as *mut objc2::runtime::ProtocolObject<
                                                    dyn objc2_metal::MTLTexture,
                                                >,
                                        )
                                        .unwrap(),
                                        gpu::TexelAspects::COLOR,
                                    ),
                                )
                            };

                            _encoder.bind(
                                0,
                                &ShaderSurfacesData {
                                    globals,
                                    surface_locals: SurfaceParams {
                                        bounds: surface.bounds.into(),
                                        content_mask: surface.content_mask.bounds.into(),
                                    },
                                    t_y,
                                    t_cb_cr,
                                    s_surface: self.atlas_sampler,
                                },
                            );

                            _encoder.draw(0, 4, 0, 1);
                        }
                    }
                }
            }
        }
        drop(pass);
        if let Some(t) = clip_target {
            let resolve = self.command_encoder.render(
                "resolve conic clip",
                gpu::RenderTargetSet {
                    colors: &[gpu::RenderTarget {
                        view: t.color_view,
                        init_op: gpu::InitOp::Load,
                        finish_op: gpu::FinishOp::ResolveTo(target),
                    }],
                    depth_stencil: None,
                },
            );
            drop(resolve);
        }
    }

    pub fn draw(&mut self, scene: &Scene) {
        self.prepare_scene(scene);
        self.command_encoder.start();
        self.atlas.before_frame(&mut self.command_encoder);

        let frame = {
            profiling::scope!("acquire frame");
            self.surface
                .as_mut()
                .expect("onscreen draw requires a surface")
                .acquire_frame()
        };
        self.command_encoder.init_texture(frame.texture());

        self.encode_scene(
            scene,
            frame.texture_view(),
            self.surface_info.alpha == gpu::AlphaMode::PreMultiplied,
            0,
            self.surface_config.size,
            None,
        );

        self.command_encoder.present(frame);
        let sync_point = self.gpu.submit(&mut self.command_encoder);

        profiling::scope!("finish");
        self.instance_belt.flush(&sync_point);
        self.atlas.after_frame(&sync_point);

        self.wait_for_gpu();
        self.last_sync_point = Some(sync_point);
    }
}

fn create_path_intermediate_texture(
    gpu: &gpu::Context,
    format: gpu::TextureFormat,
    width: u32,
    height: u32,
) -> (gpu::Texture, gpu::TextureView) {
    let texture = gpu.create_texture(gpu::TextureDesc {
        name: "path intermediate",
        format,
        size: gpu::Extent {
            width,
            height,
            depth: 1,
        },
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: gpu::TextureDimension::D2,
        usage: gpu::TextureUsage::COPY | gpu::TextureUsage::RESOURCE | gpu::TextureUsage::TARGET,
        external: None,
    });
    let texture_view = gpu.create_texture_view(
        texture,
        gpu::TextureViewDesc {
            name: "path intermediate view",
            format,
            dimension: gpu::ViewDimension::D2,
            subresources: &Default::default(),
        },
    );
    (texture, texture_view)
}

fn create_msaa_texture_if_needed(
    gpu: &gpu::Context,
    format: gpu::TextureFormat,
    width: u32,
    height: u32,
    sample_count: u32,
) -> Option<(gpu::Texture, gpu::TextureView)> {
    if sample_count <= 1 {
        return None;
    }
    let texture_msaa = gpu.create_texture(gpu::TextureDesc {
        name: "path intermediate msaa",
        format,
        size: gpu::Extent {
            width,
            height,
            depth: 1,
        },
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count,
        dimension: gpu::TextureDimension::D2,
        usage: gpu::TextureUsage::TARGET,
        external: None,
    });
    let texture_view_msaa = gpu.create_texture_view(
        texture_msaa,
        gpu::TextureViewDesc {
            name: "path intermediate msaa view",
            format,
            dimension: gpu::ViewDimension::D2,
            subresources: &Default::default(),
        },
    );

    Some((texture_msaa, texture_view_msaa))
}

/// A set of parameters that can be set using a corresponding environment variable.
struct RenderingParameters {
    // Env var: ZED_PATH_SAMPLE_COUNT
    // workaround for https://github.com/zed-industries/zed/issues/26143
    path_sample_count: u32,

    // Env var: ZED_FONTS_GAMMA
    // Allowed range [1.0, 2.2], other values are clipped
    // Default: 1.8
    gamma_ratios: [f32; 4],
    // Env var: ZED_FONTS_GRAYSCALE_ENHANCED_CONTRAST
    // Allowed range: [0.0, ..), other values are clipped
    // Default: 1.0
    grayscale_enhanced_contrast: f32,
}

impl RenderingParameters {
    fn from_env(context: &BladeContext) -> Self {
        use std::env;

        let path_sample_count = env::var("ZED_PATH_SAMPLE_COUNT")
            .ok()
            .and_then(|v| v.parse().ok())
            .or_else(|| {
                [4, 2, 1]
                    .into_iter()
                    .find(|&n| (context.gpu.capabilities().sample_count_mask & n) != 0)
            })
            .unwrap_or(1);
        let gamma = env::var("ZED_FONTS_GAMMA")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.8_f32)
            .clamp(1.0, 2.2);
        let gamma_ratios = Self::get_gamma_ratios(gamma);
        let grayscale_enhanced_contrast = env::var("ZED_FONTS_GRAYSCALE_ENHANCED_CONTRAST")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.0_f32)
            .max(0.0);

        Self {
            path_sample_count,
            gamma_ratios,
            grayscale_enhanced_contrast,
        }
    }

    // Gamma ratios for brightening/darkening edges for better contrast
    // https://github.com/microsoft/terminal/blob/1283c0f5b99a2961673249fa77c6b986efb5086c/src/renderer/atlas/dwrite.cpp#L50
    fn get_gamma_ratios(gamma: f32) -> [f32; 4] {
        const GAMMA_INCORRECT_TARGET_RATIOS: [[f32; 4]; 13] = [
            [0.0000 / 4.0, 0.0000 / 4.0, 0.0000 / 4.0, 0.0000 / 4.0], // gamma = 1.0
            [0.0166 / 4.0, -0.0807 / 4.0, 0.2227 / 4.0, -0.0751 / 4.0], // gamma = 1.1
            [0.0350 / 4.0, -0.1760 / 4.0, 0.4325 / 4.0, -0.1370 / 4.0], // gamma = 1.2
            [0.0543 / 4.0, -0.2821 / 4.0, 0.6302 / 4.0, -0.1876 / 4.0], // gamma = 1.3
            [0.0739 / 4.0, -0.3963 / 4.0, 0.8167 / 4.0, -0.2287 / 4.0], // gamma = 1.4
            [0.0933 / 4.0, -0.5161 / 4.0, 0.9926 / 4.0, -0.2616 / 4.0], // gamma = 1.5
            [0.1121 / 4.0, -0.6395 / 4.0, 1.1588 / 4.0, -0.2877 / 4.0], // gamma = 1.6
            [0.1300 / 4.0, -0.7649 / 4.0, 1.3159 / 4.0, -0.3080 / 4.0], // gamma = 1.7
            [0.1469 / 4.0, -0.8911 / 4.0, 1.4644 / 4.0, -0.3234 / 4.0], // gamma = 1.8
            [0.1627 / 4.0, -1.0170 / 4.0, 1.6051 / 4.0, -0.3347 / 4.0], // gamma = 1.9
            [0.1773 / 4.0, -1.1420 / 4.0, 1.7385 / 4.0, -0.3426 / 4.0], // gamma = 2.0
            [0.1908 / 4.0, -1.2652 / 4.0, 1.8650 / 4.0, -0.3476 / 4.0], // gamma = 2.1
            [0.2031 / 4.0, -1.3864 / 4.0, 1.9851 / 4.0, -0.3501 / 4.0], // gamma = 2.2
        ];

        const NORM13: f32 = ((0x10000 as f64) / (255.0 * 255.0) * 4.0) as f32;
        const NORM24: f32 = ((0x100 as f64) / (255.0) * 4.0) as f32;

        let index = ((gamma * 10.0).round() as usize).clamp(10, 22) - 10;
        let ratios = GAMMA_INCORRECT_TARGET_RATIOS[index];

        [
            ratios[0] * NORM13,
            ratios[1] * NORM24,
            ratios[2] * NORM13,
            ratios[3] * NORM24,
        ]
    }
}

#[cfg(test)]
#[path = "../renderer_tests.rs"]
mod gpu_tests;
