use super::ortho::{init_ortho, OrthoResources};
use super::vertex::Vertex;
use std::borrow::Cow;

pub struct RectResources {
    pub pipeline: wgpu::RenderPipeline,
    pub ortho: OrthoResources,
}

pub fn make_rect_pipeline(
    gpu_device: &wgpu::Device,
    swap_chain_descr: &wgpu::SwapChainDescriptor,
) -> RectResources {
    let ortho = init_ortho(swap_chain_descr.width, swap_chain_descr.height, gpu_device);

    let pipeline_layout = gpu_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&ortho.bind_group_layout],
        push_constant_ranges: &[],
        label: Some("Rectangle pipeline layout"),
    });
    let pipeline = create_render_pipeline(
        &gpu_device,
        &pipeline_layout,
        swap_chain_descr.format,
        &wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/shader.wgsl"))),
            flags: wgpu::ShaderFlags::all(),
        },
    );

    RectResources { pipeline, ortho }
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    shader_module_desc: &wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader_module_desc);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::DESC],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: color_format,
                color_blend: wgpu::BlendState {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                },
                alpha_blend: wgpu::BlendState::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
    })
}
