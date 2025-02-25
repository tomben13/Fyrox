use crate::{
    error::FrameworkError,
    gpu_program::{GpuProgramTrait, ShaderResourceDefinition, ShaderResourceKind},
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
        let bind_group_layouts: Vec<wgpu::BindGroupLayout> = resources
            .into_iter()
            .map(|resource| {
                let bind_group_layout =
                    server
                        .device
                        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            label: Some(&(resource.name.to_mutable() + " BindGroupLayout")),
                            entries: match resource.kind {
                                ShaderResourceKind::Texture { .. } => &[
                                    wgpu::BindGroupLayoutEntry {
                                        binding: 0,
                                        visibility: wgpu::ShaderStages::FRAGMENT,
                                        ty: wgpu::BindingType::Texture {
                                            sample_type: wgpu::TextureSampleType::Float {
                                                filterable: true,
                                            },
                                            view_dimension: wgpu::TextureViewDimension::D2,
                                            multisampled: false,
                                        },
                                        count: None,
                                    },
                                    wgpu::BindGroupLayoutEntry {
                                        binding: 1,
                                        visibility: wgpu::ShaderStages::FRAGMENT,
                                        ty: wgpu::BindingType::Sampler(
                                            wgpu::SamplerBindingType::Filtering,
                                        ),
                                        count: None,
                                    },
                                ],
                                ShaderResourceKind::PropertyGroup(ref properties) => {
                                    let entries: Vec<wgpu::BindGroupLayoutEntry> = properties.iter().enumerate().map(|(i, property)| {
                                        wgpu::BindGroupLayoutEntry {
                                            binding: i as u32,
                                            visibility: ,
                                            ty: todo!(),
                                            count: todo!(),
                                        }
                                }).collect();
                                    wgpu::BindGroupLayoutEntry {
                                    },
                                ],
                            },
                        });

                bind_group_layout
            })
            .collect();

        let layout = server
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: todo!(),
                push_constant_ranges: todo!(),
            });
        let pipeline = server
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: todo!(),
                vertex: todo!(),
                primitive: todo!(),
                depth_stencil: todo!(),
                multisample: todo!(),
                fragment: todo!(),
                multiview: todo!(),
                cache: todo!(),
            });
    }
}

impl GpuProgramTrait for WgpuProgram {}
