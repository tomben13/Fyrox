use std::{
    cell::{Cell, RefCell},
    rc::Weak,
};

use fyrox_core::color::Color;

use crate::{
    error::FrameworkError,
    gpu_texture::{
        Coordinate, GpuTextureDescriptor, GpuTextureKind, GpuTextureTrait, MagnificationFilter,
        MinificationFilter, PixelKind, WrapMode,
    },
};

use super::{
    border_color_to_wgpu, mag_filter_to_wgpu, min_filter_to_wgpu, pixel_kind_to_wgpu,
    texture_kind_to_wgpu, wrap_mode_to_wgpu,
};

use super::server::WgpuGraphicsServer;

pub struct WgpuTexture {
    state: Weak<WgpuGraphicsServer>,
    pub texture: RefCell<wgpu::Texture>,
    sampler: RefCell<wgpu::Sampler>,
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
    border_color: Cell<Option<Color>>,
}

impl WgpuTexture {
    pub fn new(
        server: &WgpuGraphicsServer,
        desc: GpuTextureDescriptor,
    ) -> Self {
        let (dimension, size) = texture_kind_to_wgpu(desc.kind);

        let texture_desc = wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: desc.mip_count as u32,
            sample_count: 1,
            dimension,
            format: pixel_kind_to_wgpu(desc.pixel_kind),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture = server.device.create_texture(&texture_desc).into();

        let (min_filter, mipmap_filter) = min_filter_to_wgpu(desc.min_filter);

        let sampler = server
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: wrap_mode_to_wgpu(desc.s_wrap_mode),
                address_mode_v: wrap_mode_to_wgpu(desc.t_wrap_mode),
                address_mode_w: wrap_mode_to_wgpu(desc.r_wrap_mode),
                mag_filter: mag_filter_to_wgpu(desc.mag_filter),
                min_filter,
                mipmap_filter,
                lod_min_clamp: desc.min_lod,
                lod_max_clamp: desc.max_lod,
                compare: None,
                anisotropy_clamp: desc.anisotropy as u16,
                border_color: None,
            })
            .into();

        Self {
            state: server.weak(),
            texture,
            sampler,
            kind: desc.kind.into(),
            min_filter: desc.min_filter.into(),
            mag_filter: desc.mag_filter.into(),
            s_wrap_mode: desc.s_wrap_mode.into(),
            t_wrap_mode: desc.t_wrap_mode.into(),
            r_wrap_mode: desc.r_wrap_mode.into(),
            anisotropy: desc.anisotropy.into(),
            pixel_kind: desc.pixel_kind.into(),
            base_level: desc.base_level.into(),
            max_level: desc.max_level.into(),
            min_lod: desc.min_lod.into(),
            max_lod: desc.max_lod.into(),
            lod_bias: desc.lod_bias.into(),
            border_color: None.into(),
        }
    }

    fn update_sampler(&self) {
        let server = self.state.upgrade().unwrap();

        let (min_filter, mipmap_filter) = min_filter_to_wgpu(self.min_filter.get());

        let sampler = server.device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wrap_mode_to_wgpu(self.s_wrap_mode.get()),
            address_mode_v: wrap_mode_to_wgpu(self.t_wrap_mode.get()),
            address_mode_w: wrap_mode_to_wgpu(self.r_wrap_mode.get()),
            mag_filter: mag_filter_to_wgpu(self.mag_filter.get()),
            min_filter,
            mipmap_filter,
            lod_min_clamp: self.min_lod.get(),
            lod_max_clamp: self.max_lod.get(),
            compare: None,
            anisotropy_clamp: self.anisotropy.get() as u16,
            border_color: self
                .border_color
                .get()
                .map(|color| border_color_to_wgpu(color)),
        });

        *self.sampler.borrow_mut() = sampler;
    }
}

impl GpuTextureTrait for WgpuTexture {
    fn set_anisotropy(&self, anisotropy: f32) {
        if self.anisotropy.get() == anisotropy {
            return;
        }
        self.anisotropy.set(anisotropy);
        self.update_sampler();
    }

    fn anisotropy(&self) -> f32 {
        self.anisotropy.get()
    }

    fn set_minification_filter(&self, min_filter: MinificationFilter) {
        if self.min_filter.get() == min_filter {
            return;
        }
        self.min_filter.set(min_filter);
        self.update_sampler();
    }

    fn minification_filter(&self) -> MinificationFilter {
        self.min_filter.get()
    }

    fn set_magnification_filter(&self, mag_filter: MagnificationFilter) {
        if self.mag_filter.get() == mag_filter {
            return;
        }
        self.mag_filter.set(mag_filter);
        self.update_sampler();
    }

    fn magnification_filter(&self) -> MagnificationFilter {
        self.mag_filter.get()
    }

    fn set_wrap(&self, coordinate: Coordinate, wrap: WrapMode) {
        match coordinate {
            Coordinate::S => {
                if self.s_wrap_mode.get() == wrap {
                    return;
                }
                self.s_wrap_mode.set(wrap);
            }
            Coordinate::T => {
                if self.t_wrap_mode.get() == wrap {
                    return;
                }
                self.t_wrap_mode.set(wrap);
            }
            Coordinate::R => {
                if self.r_wrap_mode.get() == wrap {
                    return;
                }
                self.r_wrap_mode.set(wrap);
            }
        }
        self.update_sampler();
    }

    fn wrap_mode(&self, coordinate: Coordinate) -> WrapMode {
        match coordinate {
            Coordinate::S => self.s_wrap_mode.get(),
            Coordinate::T => self.t_wrap_mode.get(),
            Coordinate::R => self.r_wrap_mode.get(),
        }
    }

    fn set_border_color(&self, color: Color) {
        if self.border_color.get() == Some(color) {
            return;
        }
        self.border_color.set(Some(color));
        self.update_sampler();
    }

    fn set_data(
        &self,
        kind: GpuTextureKind,
        pixel_kind: PixelKind,
        mip_count: usize,
        data: Option<&[u8]>,
    ) -> Result<(), FrameworkError> {
        let server = self.state.upgrade().unwrap();

        let (dimension, size) = texture_kind_to_wgpu(kind);
        let format = pixel_kind_to_wgpu(pixel_kind);

        if dimension != self.texture.borrow().dimension()
            || size != self.texture.borrow().size()
            || format != self.texture.borrow().format()
            || mip_count as u32 != self.texture.borrow().mip_level_count()
        {
            let texture_desc = wgpu::TextureDescriptor {
                label: None,
                size,
                mip_level_count: mip_count as u32,
                sample_count: 1,
                dimension,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            };

            let texture = server.device.create_texture(&texture_desc);

            *self.texture.borrow_mut() = texture;
        }

        if let Some(data) = data {
            server.queue.write_texture(
                self.texture.borrow().as_image_copy(),
                data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(format.block_copy_size(None).unwrap() * (size.physical_size(format).width / format.block_dimensions().0)),
                    rows_per_image: Some(size.physical_size(format).height / format.block_dimensions().1),
                },
                size,
            );
        }

        Ok(())
    }

    fn get_image(&self, level: usize) -> Vec<u8> {
        panic!("Unsupported");
    }

    fn read_pixels(&self) -> Vec<u8> {
        panic!("Unsupported");
    }

    fn kind(&self) -> GpuTextureKind {
        self.kind.get()
    }

    fn pixel_kind(&self) -> PixelKind {
        self.pixel_kind.get()
    }

    fn set_base_level(&self, level: usize) {
        self.base_level.set(level);
    }

    fn base_level(&self) -> usize {
        self.base_level.get()
    }

    fn set_max_level(&self, level: usize) {
        self.max_level.set(level);
    }

    fn max_level(&self) -> usize {
        self.max_level.get()
    }

    fn set_min_lod(&self, min_lod: f32) {
        if self.min_lod.get() == min_lod {
            return;
        }
        self.min_lod.set(min_lod);
        self.update_sampler();
    }

    fn min_lod(&self) -> f32 {
        self.min_lod.get()
    }

    fn set_max_lod(&self, max_lod: f32) {
        if self.max_lod.get() == max_lod {
            return;
        }
        self.max_lod.set(max_lod);
        self.update_sampler();
    }

    fn max_lod(&self) -> f32 {
        self.max_lod.get()
    }

    fn set_lod_bias(&self, bias: f32) {
        if self.lod_bias.get() == bias {
            return;
        }
        self.lod_bias.set(bias);
    }

    fn lod_bias(&self) -> f32 {
        self.lod_bias.get()
    }
}
