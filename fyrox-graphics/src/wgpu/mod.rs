use fyrox_core::color::Color;

use crate::gpu_texture::{
    GpuTextureKind, MagnificationFilter, MinificationFilter, PixelKind, WrapMode,
};

pub mod buffer;
pub mod framebuffer;
pub mod geometry_buffer;
pub mod program;
pub mod server;
pub mod texture;

fn pixel_kind_to_wgpu(pixel_kind: PixelKind) -> wgpu::TextureFormat {
    match pixel_kind {
        PixelKind::R32F => wgpu::TextureFormat::R32Float,
        PixelKind::R32UI => wgpu::TextureFormat::R32Uint,
        PixelKind::R16F => wgpu::TextureFormat::R16Float,
        PixelKind::D32F => wgpu::TextureFormat::Depth32Float,
        PixelKind::D16 => wgpu::TextureFormat::Depth16Unorm,
        PixelKind::D24S8 => wgpu::TextureFormat::Depth24PlusStencil8,
        PixelKind::RGBA8 => wgpu::TextureFormat::Rgba8Unorm,
        PixelKind::SRGBA8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        PixelKind::RGB8 => todo!(),
        PixelKind::SRGB8 => todo!(),
        PixelKind::BGRA8 => wgpu::TextureFormat::Bgra8Unorm,
        PixelKind::BGR8 => todo!(),
        PixelKind::RG8 => wgpu::TextureFormat::Rg8Unorm,
        PixelKind::LA8 => todo!(),
        PixelKind::LA16 => todo!(),
        PixelKind::RG16 => wgpu::TextureFormat::Rg16Unorm,
        PixelKind::R8 => wgpu::TextureFormat::R8Unorm,
        PixelKind::L8 => todo!(),
        PixelKind::L16 => todo!(),
        PixelKind::R8UI => wgpu::TextureFormat::R8Uint,
        PixelKind::R16 => wgpu::TextureFormat::R16Unorm,
        PixelKind::RGB16 => todo!(),
        PixelKind::RGBA16 => wgpu::TextureFormat::Rgba16Unorm,
        PixelKind::DXT1RGB => todo!(),
        PixelKind::DXT1RGBA => todo!(),
        PixelKind::DXT3RGBA => todo!(),
        PixelKind::DXT5RGBA => todo!(),
        PixelKind::RGB32F => todo!(),
        PixelKind::RGBA32F => wgpu::TextureFormat::Rgba32Float,
        PixelKind::RGB16F => todo!(),
        PixelKind::RGBA16F => wgpu::TextureFormat::Rgba16Float,
        PixelKind::R8RGTC => todo!(),
        PixelKind::RG8RGTC => todo!(),
        PixelKind::R11G11B10F => wgpu::TextureFormat::Rg11b10Ufloat,
        PixelKind::RGB10A2 => wgpu::TextureFormat::Rgb10a2Unorm,
    }
}

fn wrap_mode_to_wgpu(wrap_mode: WrapMode) -> wgpu::AddressMode {
    match wrap_mode {
        WrapMode::Repeat => wgpu::AddressMode::Repeat,
        WrapMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
        WrapMode::ClampToBorder => wgpu::AddressMode::ClampToBorder,
        WrapMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
        WrapMode::MirrorClampToEdge => panic!("Unsupported by wgpu"),
    }
}

fn min_filter_to_wgpu(min_filter: MinificationFilter) -> (wgpu::FilterMode, wgpu::FilterMode) {
    match min_filter {
        MinificationFilter::Nearest => (wgpu::FilterMode::Nearest, Default::default()),
        MinificationFilter::NearestMipMapNearest => {
            (wgpu::FilterMode::Nearest, wgpu::FilterMode::Nearest)
        }
        MinificationFilter::NearestMipMapLinear => {
            (wgpu::FilterMode::Nearest, wgpu::FilterMode::Linear)
        }
        MinificationFilter::Linear => (wgpu::FilterMode::Linear, Default::default()),
        MinificationFilter::LinearMipMapNearest => {
            (wgpu::FilterMode::Linear, wgpu::FilterMode::Nearest)
        }
        MinificationFilter::LinearMipMapLinear => {
            (wgpu::FilterMode::Linear, wgpu::FilterMode::Linear)
        }
    }
}

fn mag_filter_to_wgpu(mag_filter: MagnificationFilter) -> wgpu::FilterMode {
    match mag_filter {
        MagnificationFilter::Nearest => wgpu::FilterMode::Nearest,
        MagnificationFilter::Linear => wgpu::FilterMode::Linear,
    }
}

fn border_color_to_wgpu(color: Color) -> wgpu::SamplerBorderColor {
    match color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        } => wgpu::SamplerBorderColor::TransparentBlack,
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        } => wgpu::SamplerBorderColor::OpaqueBlack,
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        } => wgpu::SamplerBorderColor::OpaqueWhite,
        _ => panic!("Unsupported by wgpu"),
    }
}

fn texture_kind_to_wgpu(kind: GpuTextureKind) -> (wgpu::TextureDimension, wgpu::Extent3d) {
    match kind {
        GpuTextureKind::Line { length } => (
            wgpu::TextureDimension::D1,
            wgpu::Extent3d {
                width: length as u32,
                height: 1,
                depth_or_array_layers: 1,
            },
        ),
        GpuTextureKind::Rectangle { width, height } => (
            wgpu::TextureDimension::D2,
            wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
        ),
        GpuTextureKind::Cube { width, height } => (
            wgpu::TextureDimension::D2,
            wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
        ),
        GpuTextureKind::Volume {
            width,
            height,
            depth,
        } => (
            wgpu::TextureDimension::D3,
            wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: depth as u32,
            },
        ),
    }
}
