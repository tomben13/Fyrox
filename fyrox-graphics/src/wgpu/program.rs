use fyrox_core::log::Log;

use crate::{
    error::FrameworkError,
    gpu_program::{
        GpuProgramTrait, ShaderPropertyKind, ShaderResourceDefinition, ShaderResourceKind,
    },
};

use super::server::WgpuGraphicsServer;

pub(crate) struct WgpuProgram {
    pub(crate) shader: wgpu::ShaderModule,
    pub(crate) layout: wgpu::PipelineLayout,
}

impl WgpuProgram {
    pub(crate) fn from_source_and_resources(
        server: &WgpuGraphicsServer,
        program_name: &str,
        vertex_source: &str,
        fragment_source: &str,
        resources: &[ShaderResourceDefinition],
    ) -> Result<WgpuProgram, FrameworkError> {
        let shader = server
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(
                    (vertex_source.to_owned() + fragment_source).into(),
                ),
            });

        let mut texture_entries = Vec::new();
        let mut property_group_entries = Vec::new();
        for resource in resources {
            match resource.kind {
                ShaderResourceKind::Texture { .. } => {
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

        Ok(Self { shader, layout })
    }
}

impl GpuProgramTrait for WgpuProgram {}
