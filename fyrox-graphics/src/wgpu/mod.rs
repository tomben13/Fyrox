use crate::gpu_texture::PixelKind;

pub mod buffer;
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
