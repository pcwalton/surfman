//! A surface abstraction that can switch between hardware and software rendering.

use crate::gl::types::{GLenum, GLuint};
use crate::platform::default::surface::{NativeWidget as HWNativeWidget, Surface as HWSurface, SurfaceTexture as HWSurfaceTexture};
use crate::platform::generic::osmesa::surface::NativeWidget as SWNativeWidget;
use crate::platform::generic::osmesa::surface::Surface as SWSurface;
use crate::platform::generic::osmesa::surface::SurfaceTexture as SWSurfaceTexture;
use crate::{Error, SurfaceID, SurfaceType};
use super::context::Context;
use super::context::ContextDescriptor;
use super::device::Device;

use euclid::default::Size2D;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum Surface {
    Hardware(HWSurface),
    Software(SWSurface),
}

impl Surface {
    pub fn hardware(self) -> Result<HWSurface, Error> {
        match self {
	    Surface::Hardware(surface) => Ok(surface),
	    Surface::Software(_) => Err(Error::IncompatibleSurface),
	}
    }

    pub fn software(self) -> Result<SWSurface, Error> {
        match self {
	    Surface::Software(surface) => Ok(surface),
	    Surface::Hardware(_) => Err(Error::IncompatibleSurface),
	}
    }
}

pub enum SurfaceTexture {
    Hardware(HWSurfaceTexture),
    Software(SWSurfaceTexture),
}

impl SurfaceTexture {
    pub fn hardware(self) -> Result<HWSurfaceTexture, Error> {
        match self {
	    SurfaceTexture::Hardware(surface) => Ok(surface),
	    SurfaceTexture::Software(_) => Err(Error::IncompatibleSurfaceTexture),
	}
    }

    pub fn software(self) -> Result<SWSurfaceTexture, Error> {
        match self {
	    SurfaceTexture::Software(surface) => Ok(surface),
	    SurfaceTexture::Hardware(_) => Err(Error::IncompatibleSurfaceTexture),
	}
    }
}

pub type NativeWidget = HWNativeWidget;

impl Device {
    pub fn create_surface(&mut self, context: &Context, surface_type: &SurfaceType<NativeWidget>)
                          -> Result<Surface, Error> {
        match *self {
            Device::Hardware(ref mut device) => {
                device.create_surface(context.hardware_ref()?, surface_type).map(Surface::Hardware)
            }
            Device::Software(ref mut device) => {
                device.create_surface(context.software_ref()?, &surface_type.into_software()?).map(Surface::Software)
            }
        }
    }

    pub fn create_surface_texture(&self, context: &mut Context, surface: Surface)
                                  -> Result<SurfaceTexture, Error> {
        match *self {
            Device::Hardware(ref device) => {
                device.create_surface_texture(context.hardware_mut()?, surface.hardware()?).map(SurfaceTexture::Hardware)
            }
            Device::Software(ref device) => {
                device.create_surface_texture(context.software_mut()?, surface.software()?).map(SurfaceTexture::Software)
            }
        }
    }

    pub fn destroy_surface(&self, context: &mut Context, surface: Surface) -> Result<(), Error> {
        match *self {
            Device::Hardware(ref device) => {
                device.destroy_surface(context.hardware_mut()?, surface.hardware()?)
            }
            Device::Software(ref device) => {
                device.destroy_surface(context.software_mut()?, surface.software()?)
            }
        }
    }

    pub fn destroy_surface_texture(&self, context: &mut Context, surface_texture: SurfaceTexture)
                                   -> Result<Surface, Error> {
        match *self {
            Device::Hardware(ref device) => {
                device.destroy_surface_texture(context.hardware_mut()?, surface_texture.hardware()?).map(Surface::Hardware)
            }
            Device::Software(ref device) => {
                device.destroy_surface_texture(context.software_mut()?, surface_texture.software()?).map(Surface::Software)
            }
        }
    }

    #[inline]
    pub fn surface_gl_texture_target(&self) -> GLenum {
        match *self {
            Device::Hardware(ref device) => device.surface_gl_texture_target(),
            Device::Software(ref device) => device.surface_gl_texture_target(),
        }
    }

    #[inline]
    pub fn lock_surface_data<'s>(&self, surface: &'s mut Surface)
                                 -> Result<SurfaceDataGuard<'s>, Error> {
        Err(Error::Unimplemented)
    }
}

impl Surface {
    #[inline]
    pub fn size(&self) -> Size2D<i32> {
        match *self {
            Surface::Hardware(ref surface) => surface.size(),
            Surface::Software(ref surface) => surface.size(),
        }
    }

    #[inline]
    pub fn id(&self) -> SurfaceID {
        match *self {
            Surface::Hardware(ref surface) => surface.id(),
            Surface::Software(ref surface) => surface.id(),
        }
    }
}

impl SurfaceTexture {
    #[inline]
    pub fn gl_texture(&self) -> GLuint {
        match *self {
            SurfaceTexture::Hardware(ref surface_texture) => surface_texture.gl_texture(),
            SurfaceTexture::Software(ref surface_texture) => surface_texture.gl_texture(),
        }
    }
}

impl SurfaceType<NativeWidget> {
    pub fn into_software(&self) -> Result<SurfaceType<SWNativeWidget>, Error> {
        match *self {
	    SurfaceType::Generic { ref size } => Ok(SurfaceType::Generic { size: size.clone() }),
	    SurfaceType::Widget { .. } => Err(Error::InvalidNativeWidget),
	}
    }
}

pub struct SurfaceDataGuard<'a> {
    phantom: PhantomData<&'a ()>,
}
