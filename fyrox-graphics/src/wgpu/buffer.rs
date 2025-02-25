use std::rc::Weak;

use crate::{
    buffer::{BufferKind, BufferUsage, GpuBufferTrait},
    error::FrameworkError,
};

use super::server::WgpuGraphicsServer;

pub struct WgpuBuffer {
    state: Weak<WgpuGraphicsServer>,
    pub(crate) buffer: wgpu::Buffer,
    size: usize,
    kind: BufferKind,
    usage: BufferUsage,
}

impl WgpuBuffer {
    pub fn new(
        server: &WgpuGraphicsServer,
        size: usize,
        kind: BufferKind,
        usage: BufferUsage,
    ) -> Self {
        let buffer = server.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size as u64,
            usage: Self::get_wgpu_buffer_usages(kind, usage),
            mapped_at_creation: false,
        });

        Self {
            state: server.weak(),
            buffer,
            size,
            kind,
            usage,
        }
    }

    fn get_wgpu_buffer_usages(kind: BufferKind, usage: BufferUsage) -> wgpu::BufferUsages {
        let mut wgpu_usages = match kind {
            BufferKind::Vertex => wgpu::BufferUsages::VERTEX,
            BufferKind::Index => wgpu::BufferUsages::INDEX,
            BufferKind::Uniform => wgpu::BufferUsages::UNIFORM,
            BufferKind::PixelRead => wgpu::BufferUsages::MAP_READ,
            BufferKind::PixelWrite => wgpu::BufferUsages::MAP_WRITE,
        };

        wgpu_usages |= match usage {
            BufferUsage::StreamCopy | BufferUsage::DynamicCopy | BufferUsage::StaticCopy => {
                wgpu::BufferUsages::COPY_DST
            }

            BufferUsage::StreamRead | BufferUsage::DynamicRead | BufferUsage::StaticRead => {
                wgpu::BufferUsages::MAP_READ
            }

            BufferUsage::StreamDraw | BufferUsage::DynamicDraw | BufferUsage::StaticDraw => {
                wgpu::BufferUsages::MAP_WRITE
            }
        };

        wgpu_usages
    }
}

impl GpuBufferTrait for WgpuBuffer {
    fn usage(&self) -> BufferUsage {
        self.usage
    }

    fn kind(&self) -> BufferKind {
        self.kind
    }

    fn size(&self) -> usize {
        self.size
    }

    fn write_data(&self, data: &[u8]) -> Result<(), FrameworkError> {
        let Some(server) = self.state.upgrade() else {
            return Err(FrameworkError::GraphicsServerUnavailable);
        };

        if !data.len() <= self.size() {
            return Err(FrameworkError::Custom(String::from("Buffer too small")));
        }

        server.queue.write_buffer(&self.buffer, 0, data);

        Ok(())
    }

    fn read_data(&self, data: &mut [u8]) -> Result<(), FrameworkError> {
        todo!()
    }
}
