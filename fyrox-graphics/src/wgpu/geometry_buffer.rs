use std::{cell::Cell, rc::Weak};

use fyrox_core::{array_as_u8_slice, math::TriangleDefinition};

use crate::{
    buffer::{BufferKind, GpuBufferTrait},
    error::FrameworkError,
    geometry_buffer::{ElementsDescriptor, GeometryBufferDescriptor, GpuGeometryBufferTrait},
};

use super::{buffer::WgpuBuffer, server::WgpuGraphicsServer};

pub(crate) struct WgpuGeometryBuffer {
    state: Weak<WgpuGraphicsServer>,
    pub(crate) vertex_buffers: Vec<WgpuBuffer>,
    pub(crate) index_buffer: WgpuBuffer,
    element_count: Cell<usize>,
    pub(crate) desc: GeometryBufferDescriptor,
}

impl WgpuGeometryBuffer {
    pub(crate) fn new(
        server: &WgpuGraphicsServer,
        desc: GeometryBufferDescriptor,
    ) -> Result<Self, FrameworkError> {
        let (element_count, data) = match desc.elements {
            ElementsDescriptor::Triangles(triangles) => {
                (triangles.len(), array_as_u8_slice(triangles))
            }
            ElementsDescriptor::Lines(lines) => (lines.len(), array_as_u8_slice(lines)),
            ElementsDescriptor::Points(points) => (points.len(), array_as_u8_slice(points)),
        };

        let index_buffer = WgpuBuffer::new(server, data.len(), BufferKind::Index, desc.usage);

        index_buffer.write_data(data)?;

        let vertex_buffers = desc
            .buffers
            .into_iter()
            .map(|buffer_desc| {
                WgpuBuffer::new(
                    server,
                    buffer_desc.data.bytes.unwrap().len(),
                    BufferKind::Vertex,
                    buffer_desc.usage,
                )
            })
            .collect();

        Ok(Self {
            state: server.weak(),
            vertex_buffers,
            index_buffer,
            element_count: element_count.into(),
            desc,
        })
    }
}

impl GpuGeometryBufferTrait for WgpuGeometryBuffer {
    fn set_buffer_data(&self, buffer: usize, data: &[u8]) {
        self.vertex_buffers[buffer].write_data(data).unwrap();
    }

    fn element_count(&self) -> usize {
        self.element_count.get()
    }

    fn set_triangles(&self, triangles: &[TriangleDefinition]) {
        self.element_count.set(triangles.len());
        self.index_buffer
            .write_data(array_as_u8_slice(triangles))
            .unwrap();
    }

    fn set_lines(&self, lines: &[[u32; 2]]) {
        self.element_count.set(lines.len());
        self.index_buffer
            .write_data(array_as_u8_slice(lines))
            .unwrap();
    }

    fn set_points(&self, points: &[u32]) {
        self.element_count.set(points.len());
        self.index_buffer
            .write_data(array_as_u8_slice(points))
            .unwrap();
    }
}
