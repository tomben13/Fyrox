use fyrox_core::log::Log;

use crate::{
    error::FrameworkError,
    gpu_program::{
        GpuProgramTrait, ShaderPropertyKind, ShaderResourceDefinition, ShaderResourceKind,
    },
};

use super::server::WgpuGraphicsServer;

pub(crate) struct WgpuProgram {
    pipeline: wgpu::RenderPipeline,
}

impl WgpuProgram {
    pub(crate) fn from_source_and_resources(
        server: &WgpuGraphicsServer,
        program_name: &str,
        vertex_source: &str,
        fragment_source: &str,
        resources: &[ShaderResourceDefinition],
    ) -> Result<WgpuProgram, FrameworkError> {
        let mut texture_entries = Vec::new();
        let mut property_group_entries = Vec::new();
        for resource in resources {
            match resource.kind {
                ShaderResourceKind::Texture { kind, fallback } => {
                    let texture_entry = wgpu::BindGroupLayoutEntry {
                        binding: 2 * resource.binding as u32,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    };

                    let sampler_entry = wgpu::BindGroupLayoutEntry {
                        binding: 2 * resource.binding as u32 + 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    };

                    texture_entries.push(texture_entry);
                    texture_entries.push(sampler_entry);
                }
                ShaderResourceKind::PropertyGroup(_) => {
                    let property_group_entry = wgpu::BindGroupLayoutEntry {
                        binding: resource.binding as u32,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    };

                    property_group_entries.push(property_group_entry);
                }
            }
        }

        let texture_bind_group_layout =
            server
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &texture_entries,
                });

        let property_group_bind_group_layout =
            server
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &property_group_entries,
                });

        let layout = server
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &property_group_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let pipeline = server
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: None,
                    compilation_options: Default::default(),
                    buffers: todo!(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: None,
                    compilation_options: Default::default(),
                    targets: todo!(),
                }),
                primitive: todo!(),
                depth_stencil: todo!(),
                multisample: todo!(),
                multiview: todo!(),
                cache: todo!(),
            });

        Ok(Self { pipeline })
    }
}

impl GpuProgramTrait for WgpuProgram {}
