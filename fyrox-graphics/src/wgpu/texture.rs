use std::{cell::Cell, rc::Weak};

use fyrox_core::color::Color;

use crate::{
    error::FrameworkError,
    gpu_texture::{
        Coordinate, GpuTextureDescriptor, GpuTextureKind, GpuTextureTrait, MagnificationFilter,
        MinificationFilter, PixelKind, WrapMode,
    },
    wgpu::pixel_kind_to_wgpu,
};

use super::server::WgpuGraphicsServer;

pub struct WgpuTexture {
    state: Weak<WgpuGraphicsServer>,
    texture: wgpu::Texture,
    kind: Cell<GpuTextureKind>,
    min_filter: Cell<MinificationFilter>,
    mag_filter: Cell<MagnificationFilter>,
    s_wrap_mode: Cell<WrapMode>,
    t_wrap_mode: Cell<WrapMode>,
    r_wrap_mode: Cell<WrapMode>,
    anisotropy: Cell<f32>,
    pixel_kind: Cell<PixelKind>,
    base_level: Cell<usize>,
    max_level: Cell<usize>,
    min_lod: Cell<f32>,
    max_lod: Cell<f32>,
    lod_bias: Cell<f32>,
}

impl WgpuTexture {
    pub fn new(
        server: &WgpuGraphicsServer,
        desc: GpuTextureDescriptor,
    ) -> Result<Self, FrameworkError> {
        let size = match desc.kind {
            GpuTextureKind::Line { length } => wgpu::Extent3d {
                width: length as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
            GpuTextureKind::Rectangle { width, height } => wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            GpuTextureKind::Cube { width, height } => wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            GpuTextureKind::Volume {
                width,
                height,
                depth,
            } => wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: depth as u32,
            },
        };

        let texture = server.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: desc.mip_count as u32,
            sample_count: 1,
            dimension: match desc.kind {
                GpuTextureKind::Line { .. } => wgpu::TextureDimension::D1,
                GpuTextureKind::Rectangle { .. } => wgpu::TextureDimension::D2,
                GpuTextureKind::Cube { .. } => wgpu::TextureDimension::D2,
                GpuTextureKind::Volume { .. } => wgpu::TextureDimension::D3,
            },
            format: pixel_kind_to_wgpu(desc.pixel_kind),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Ok(Self { texture })
    }
}

impl GpuTextureTrait for WgpuTexture {
    fn set_anisotropy(&self, anisotropy: f32) {
        todo!()
    }

    fn anisotropy(&self) -> f32 {
        todo!()
    }

    fn set_minification_filter(&self, min_filter: MinificationFilter) {
        todo!()
    }

    fn minification_filter(&self) -> MinificationFilter {
        todo!()
    }

    fn set_magnification_filter(&self, mag_filter: MagnificationFilter) {
        todo!()
    }

    fn magnification_filter(&self) -> MagnificationFilter {
        todo!()
    }

    fn set_wrap(&self, coordinate: Coordinate, wrap: WrapMode) {
        todo!()
    }

    fn wrap_mode(&self, coordinate: Coordinate) -> WrapMode {
        todo!()
    }

    fn set_border_color(&self, color: Color) {
        todo!()
    }

    fn set_data(
        &self,
        kind: GpuTextureKind,
        pixel_kind: PixelKind,
        mip_count: usize,
        data: Option<&[u8]>,
    ) -> Result<(), FrameworkError> {
        todo!()
    }

    fn get_image(&self, level: usize) -> Vec<u8> {
        todo!()
    }

    fn read_pixels(&self) -> Vec<u8> {
        todo!()
    }

    fn kind(&self) -> GpuTextureKind {
        todo!()
    }

    fn pixel_kind(&self) -> PixelKind {
        todo!()
    }

    fn set_base_level(&self, level: usize) {
        todo!()
    }

    fn base_level(&self) -> usize {
        todo!()
    }

    fn set_max_level(&self, level: usize) {
        todo!()
    }

    fn max_level(&self) -> usize {
        todo!()
    }

    fn set_min_lod(&self, min_lod: f32) {
        todo!()
    }

    fn min_lod(&self) -> f32 {
        todo!()
    }

    fn set_max_lod(&self, max_lod: f32) {
        todo!()
    }

    fn max_lod(&self) -> f32 {
        todo!()
    }

    fn set_lod_bias(&self, bias: f32) {
        todo!()
    }

    fn lod_bias(&self) -> f32 {
        todo!()
    }
}
