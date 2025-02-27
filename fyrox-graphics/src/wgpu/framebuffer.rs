use std::{cell::Cell, rc::Weak};

use fyrox_core::{color::Color, math::Rect};

use crate::{
    error::FrameworkError,
    framebuffer::{
        Attachment, DrawCallStatistics, GpuFrameBuffer, GpuFrameBufferTrait, ResourceBindGroup,
    },
    geometry_buffer::{GpuGeometryBuffer, GpuGeometryBufferTrait},
    gpu_program::GpuProgram,
    gpu_texture::CubeMapFace,
    DrawParameters, ElementRange,
};

use super::{
    geometry_buffer::WgpuGeometryBuffer, program::WgpuProgram, server::WgpuGraphicsServer,
    texture::WgpuTexture,
};

struct WgpuFrameBuffer {
    state: Weak<WgpuGraphicsServer>,
    // pipeline: wgpu::RenderPipeline,
    color_attachments: Vec<Attachment>,
    depth_attachment: Option<Attachment>,
    clear_color: Cell<Option<Color>>,
    clear_depth: Cell<Option<f32>>,
    clear_stencil: Cell<Option<i32>>,
}

impl WgpuFrameBuffer {
    pub fn new(
        server: &WgpuGraphicsServer,
        depth_attachment: Option<Attachment>,
        color_attachments: Vec<Attachment>,
        program: &WgpuProgram,
    ) -> Self {
        Self {
            state: server.weak(),
            pipeline,
            color_attachments,
            depth_attachment,
            clear_color: None.into(),
            clear_depth: None.into(),
            clear_stencil: None.into(),
        }
    }
}

impl GpuFrameBufferTrait for WgpuFrameBuffer {
    fn color_attachments(&self) -> &[Attachment] {
        &self.color_attachments
    }

    fn depth_attachment(&self) -> Option<&Attachment> {
        self.depth_attachment.as_ref()
    }

    fn set_cubemap_face(&self, attachment_index: usize, face: CubeMapFace) {
        todo!()
    }

    fn blit_to(
        &self,
        dest: &GpuFrameBuffer,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        copy_color: bool,
        copy_depth: bool,
        copy_stencil: bool,
    ) {
        todo!()
    }

    fn clear(
        &self,
        _viewport: Rect<i32>,
        color: Option<Color>,
        depth: Option<f32>,
        stencil: Option<i32>,
    ) {
        self.clear_color.set(color);
        self.clear_depth.set(depth);
        self.clear_stencil.set(stencil);
    }

    fn draw(
        &self,
        geometry: &GpuGeometryBuffer,
        viewport: Rect<i32>,
        program: &GpuProgram,
        params: &DrawParameters,
        resources: &[ResourceBindGroup],
        element_range: ElementRange,
    ) -> Result<DrawCallStatistics, FrameworkError> {
        let server = self.state.upgrade().unwrap();

        let color_targets: Vec<Option<wgpu::ColorTargetState>> = self
            .color_attachments
            .iter()
            .map(|color_attachment| {
                let texture = color_attachment
                    .texture
                    .as_any()
                    .downcast_ref::<WgpuTexture>()
                    .unwrap();

                Some(wgpu::ColorTargetState {
                    format: texture.texture.borrow().format(),
                    blend: Default::default(),
                    write_mask: Default::default(),
                })
            })
            .collect();

        let program = program.as_any().downcast_ref::<WgpuProgram>().unwrap();

        let pipeline = server
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&program.layout),
                vertex: wgpu::VertexState {
                    module: &program.shader,
                    entry_point: None,
                    compilation_options: Default::default(),
                    buffers: todo!(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &program.shader,
                    entry_point: None,
                    compilation_options: Default::default(),
                    targets: &color_targets,
                }),
                primitive: wgpu::PrimitiveState {
                    topology: todo!(),
                    strip_index_format: todo!(),
                    front_face: todo!(),
                    cull_mode: todo!(),
                    unclipped_depth: todo!(),
                    polygon_mode: todo!(),
                    conservative: todo!(),
                },
                depth_stencil: self.depth_attachment.map(|depth_attachment| {
                    let format = depth_attachment
                        .texture
                        .as_any()
                        .downcast_ref::<WgpuTexture>()
                        .unwrap()
                        .texture
                        .borrow()
                        .format();

                    wgpu::DepthStencilState {
                        format,
                        depth_write_enabled: todo!(),
                        depth_compare: todo!(),
                        stencil: Default::default(),
                        bias: Default::default(),
                    }
                }),
                multisample: Default::default(),
                multiview: Default::default(),
                cache: Default::default(),
            });

        let mut encoder = server
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let views_iter: Vec<wgpu::TextureView> = self
            .color_attachments
            .iter()
            .map(|attachment| {
                let texture = attachment
                    .texture
                    .as_any()
                    .downcast_ref::<WgpuTexture>()
                    .unwrap();

                texture
                    .texture
                    .borrow()
                    .create_view(&wgpu::TextureViewDescriptor::default())
            })
            .collect();

        let color_attachments = views_iter
            .iter()
            .map(|view| {
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if let Some(color) = self.clear_color.get() {
                            let color = color.as_frgba();
                            let color = wgpu::Color {
                                r: color.x as f64,
                                g: color.y as f64,
                                b: color.z as f64,
                                a: color.w as f64,
                            };
                            wgpu::LoadOp::Clear(color)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })
            })
            .collect::<Vec<Option<wgpu::RenderPassColorAttachment>>>();

        let view = self.depth_attachment.as_ref().map(|attachment| {
            let texture = attachment
                .texture
                .as_any()
                .downcast_ref::<WgpuTexture>()
                .unwrap();

            texture
                .texture
                .borrow()
                .create_view(&wgpu::TextureViewDescriptor::default())
        });

        let depth_stencil_attachment =
            view.as_ref()
                .map(|view| wgpu::RenderPassDepthStencilAttachment {
                    view,
                    depth_ops: Some(wgpu::Operations {
                        load: if let Some(depth) = self.clear_depth.get() {
                            wgpu::LoadOp::Clear(depth)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: if let Some(stencil) = self.clear_stencil.get() {
                            wgpu::LoadOp::Clear(stencil as u32)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    }),
                });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let program = program.as_any().downcast_ref::<WgpuProgram>().unwrap();

        //render_pass.set_pipeline(todo!());

        for resource in resources {
            server.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: todo!(),
                entries: todo!(),
            });
        }
        //render_pass.set_bind_group(todo!());

        let geometry = geometry
            .as_any()
            .downcast_ref::<WgpuGeometryBuffer>()
            .unwrap();

        for (i, vertex_buffer) in geometry.vertex_buffers.iter().enumerate() {
            render_pass.set_vertex_buffer(i as u32, vertex_buffer.buffer.slice(..));
        }
        render_pass.set_index_buffer(
            geometry.index_buffer.buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        let range = match element_range {
            ElementRange::Full => 0..geometry.element_count() as u32,
            ElementRange::Specific { offset, count } => offset as u32..offset as u32 + count as u32,
        };
        render_pass.draw_indexed(range, 0, 0..1);
    }

    fn draw_instances(
        &self,
        count: usize,
        geometry: &GpuGeometryBuffer,
        viewport: Rect<i32>,
        program: &GpuProgram,
        params: &DrawParameters,
        resources: &[ResourceBindGroup],
    ) -> DrawCallStatistics {
        todo!()
    }
}
