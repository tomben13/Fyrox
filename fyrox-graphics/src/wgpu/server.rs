use std::{
    cell::OnceCell,
    rc::{Rc, Weak},
    sync::Arc,
};

use fyrox_core::futures::executor::block_on;
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

use crate::{
    buffer::{BufferKind, BufferUsage, GpuBuffer},
    error::FrameworkError,
    server::{GraphicsServer, SharedGraphicsServer},
};

use super::buffer::WgpuBuffer;

pub struct WgpuGraphicsServer {
    surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    this: OnceCell<Weak<WgpuGraphicsServer>>,
}

impl WgpuGraphicsServer {
    pub fn new(
        window_target: &EventLoopWindowTarget<()>,
        window_builder: WindowBuilder,
    ) -> Result<(Arc<Window>, SharedGraphicsServer), FrameworkError> {
        let window = Arc::new(window_builder.build(window_target).unwrap());

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                ..Default::default()
            },
            None,
        ))
        .unwrap();

        let state = Self {
            surface,
            device,
            queue,
            this: Default::default(),
        };

        let shared = Rc::new(state);

        shared.this.set(Rc::downgrade(&shared));

        Ok((window, shared))
    }

    pub fn weak(&self) -> Weak<WgpuGraphicsServer> {
        self.this.get().unwrap().clone()
    }
}

impl GraphicsServer for WgpuGraphicsServer {
    fn create_buffer(
        &self,
        size: usize,
        buffer_kind: BufferKind,
        buffer_usage: BufferUsage,
    ) -> Result<GpuBuffer, FrameworkError> {
        Ok(GpuBuffer(Rc::new(WgpuBuffer::new(
            self,
            size,
            buffer_kind,
            buffer_usage,
        ))))
    }

    fn create_texture(
        &self,
        desc: crate::gpu_texture::GpuTextureDescriptor,
    ) -> Result<crate::gpu_texture::GpuTexture, crate::error::FrameworkError> {
        todo!()
    }

    fn create_frame_buffer(
        &self,
        depth_attachment: Option<crate::framebuffer::Attachment>,
        color_attachments: Vec<crate::framebuffer::Attachment>,
    ) -> Result<crate::framebuffer::GpuFrameBuffer, crate::error::FrameworkError> {
        todo!()
    }

    fn back_buffer(&self) -> crate::framebuffer::GpuFrameBuffer {
        todo!()
    }

    fn create_query(&self) -> Result<crate::query::GpuQuery, crate::error::FrameworkError> {
        todo!()
    }

    fn create_program(
        &self,
        name: &str,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<crate::gpu_program::GpuProgram, crate::error::FrameworkError> {
        todo!()
    }

    fn create_program_with_properties(
        &self,
        name: &str,
        vertex_source: &str,
        fragment_source: &str,
        properties: &[crate::gpu_program::ShaderResourceDefinition],
    ) -> Result<crate::gpu_program::GpuProgram, crate::error::FrameworkError> {
        todo!()
    }

    fn create_async_read_buffer(
        &self,
        pixel_size: usize,
        pixel_count: usize,
    ) -> Result<crate::read_buffer::GpuAsyncReadBuffer, crate::error::FrameworkError> {
        todo!()
    }

    fn create_geometry_buffer(
        &self,
        desc: crate::geometry_buffer::GeometryBufferDescriptor,
    ) -> Result<crate::geometry_buffer::GpuGeometryBuffer, crate::error::FrameworkError> {
        todo!()
    }

    fn weak(self: std::rc::Rc<Self>) -> std::rc::Weak<dyn GraphicsServer> {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }

    fn finish(&self) {
        todo!()
    }

    fn invalidate_resource_bindings_cache(&self) {
        todo!()
    }

    fn pipeline_statistics(&self) -> crate::stats::PipelineStatistics {
        todo!()
    }

    fn swap_buffers(&self) -> Result<(), crate::error::FrameworkError> {
        todo!()
    }

    fn set_frame_size(&self, new_size: (u32, u32)) {
        todo!()
    }

    fn capabilities(&self) -> crate::server::ServerCapabilities {
        todo!()
    }

    fn set_polygon_fill_mode(
        &self,
        polygon_face: crate::PolygonFace,
        polygon_fill_mode: crate::PolygonFillMode,
    ) {
        todo!()
    }
}
