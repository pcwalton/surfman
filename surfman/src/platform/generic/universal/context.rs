//! A context abstraction that can switch between hardware and software rendering.

use crate::gl::types::GLuint;
use crate::platform::default::context::Context as HWContext;
use crate::platform::default::context::ContextDescriptor as HWContextDescriptor;
use crate::platform::default::device::Device as HWDevice;
use crate::platform::generic::osmesa::context::Context as SWContext;
use crate::platform::generic::osmesa::context::ContextDescriptor as SWContextDescriptor;
use crate::platform::generic::osmesa::device::Device as SWDevice;
use crate::{ContextAttributes, Error, SurfaceID, SurfaceType};
use super::device::Device;
use super::surface::NativeWidget;
use super::surface::Surface;

use euclid::default::Size2D;
use std::os::raw::c_void;

pub enum Context {
    Hardware(HWContext),
    Software(SWContext),
}

impl Context {
    pub fn hardware_ref(&self) -> Result<&HWContext, Error> {
        match *self {
	    Context::Hardware(ref context) => Ok(context),
	    Context::Software(_) => Err(Error::IncompatibleContext),
	}
    }

    pub fn software_ref(&self) -> Result<&SWContext, Error> {
        match *self {
	    Context::Software(ref context) => Ok(context),
	    Context::Hardware(_) => Err(Error::IncompatibleContext),
	}
    }

    pub fn hardware_mut(&mut self) -> Result<&mut HWContext, Error> {
        match *self {
	    Context::Hardware(ref mut context) => Ok(context),
	    Context::Software(_) => Err(Error::IncompatibleContext),
	}
    }

    pub fn software_mut(&mut self) -> Result<&mut SWContext, Error> {
        match *self {
	    Context::Software(ref mut context) => Ok(context),
	    Context::Hardware(_) => Err(Error::IncompatibleContext),
	}
    }
}

#[derive(Clone)]
pub enum ContextDescriptor {
    Hardware(HWContextDescriptor),
    Software(SWContextDescriptor),
}

impl ContextDescriptor {
    pub fn hardware(&self) -> Result<&HWContextDescriptor, Error> {
        match *self {
	    ContextDescriptor::Hardware(ref descriptor) => Ok(descriptor),
	    ContextDescriptor::Software(_) => Err(Error::IncompatibleContext),
	}
    }

    pub fn software(&self) -> Result<&SWContextDescriptor, Error> {
        match *self {
	    ContextDescriptor::Software(ref descriptor) => Ok(descriptor),
	    ContextDescriptor::Hardware(_) => Err(Error::IncompatibleContext),
	}
    }
}

impl Device {
    pub fn create_context_descriptor(&self, attributes: &ContextAttributes)
                                     -> Result<ContextDescriptor, Error> {
        match *self {
            Device::Hardware(ref device) => {
                device.create_context_descriptor(attributes).map(ContextDescriptor::Hardware)
            }
            Device::Software(ref device) => {
                device.create_context_descriptor(attributes).map(ContextDescriptor::Software)
            }
        }
    }

    /// Opens the device and context corresponding to the current hardware context.
    pub unsafe fn from_current_hardware_context() -> Result<(Device, Context), Error> {
        HWDevice::from_current_context().map(|(device, context)| {
            (Device::Hardware(device), Context::Hardware(context))
        })
    }

    /// Opens the device and context corresponding to the current software context.
    pub unsafe fn from_current_software_context() -> Result<(Device, Context), Error> {
        SWDevice::from_current_context().map(|(device, context)| {
            (Device::Software(device), Context::Software(context))
        })
    }

    pub fn create_context(&mut self, descriptor: &ContextDescriptor, surface_type: &SurfaceType<NativeWidget>)
                          -> Result<Context, Error> {
        match *self  {
            Device::Hardware(ref mut device) => {
                device.create_context(descriptor.hardware()?, surface_type).map(Context::Hardware)
            }
            Device::Software(ref mut device) => {
                device.create_context(descriptor.software()?, &surface_type.into_software()?).map(Context::Software)
            }
        }
    }

    pub fn destroy_context(&self, context: &mut Context) -> Result<(), Error> {
        match *self {
            Device::Hardware(ref device) => {
                 device.destroy_context(context.hardware_mut()?)
            }
            Device::Software(ref device) => {
                 device.destroy_context(context.software_mut()?)
            }
        }
    }

    pub fn context_descriptor(&self, context: &Context) -> ContextDescriptor {
        match *self {
            Device::Hardware(ref device) => {
                 ContextDescriptor::Hardware(device.context_descriptor(context.hardware_ref().unwrap()))
            }
            Device::Software(ref device) => {
                 ContextDescriptor::Software(device.context_descriptor(context.software_ref().unwrap()))
            }
        }
    }

    pub fn make_context_current(&self, context: &Context) -> Result<(), Error> {
        match *self {
            Device::Hardware(ref device) => {
                 device.make_context_current(context.hardware_ref()?)
            }
            Device::Software(ref device) => {
                 device.make_context_current(context.software_ref()?)
            }
        }
    }

    pub fn make_no_context_current(&self) -> Result<(), Error> {
        match self {
            &Device::Hardware(ref device) => {
                device.make_no_context_current()
            }
            &Device::Software(ref device) => {
                device.make_no_context_current()
            }
        }
    }

    pub fn replace_context_surface(&self, context: &mut Context, new_surface: Surface)
                                   -> Result<Surface, Error> {
        match *self {
            Device::Hardware(ref device) => {
                 device.replace_context_surface(context.hardware_mut()?, new_surface.hardware()?).map(Surface::Hardware)
            }
            Device::Software(ref device) => {
                 device.replace_context_surface(context.software_mut()?, new_surface.software()?).map(Surface::Software)
            }
        }
    }

    pub fn context_surface_framebuffer_object(&self, context: &Context) -> Result<GLuint, Error> {
        match *self {
            Device::Hardware(ref device) => {
                 device.context_surface_framebuffer_object(context.hardware_ref()?)
            }
            Device::Software(ref device) => {
                 device.context_surface_framebuffer_object(context.software_ref()?)
            }
        }
    }

    pub fn context_surface_size(&self, context: &Context) -> Result<Size2D<i32>, Error> {
        match *self {
            Device::Hardware(ref device) => {
                 device.context_surface_size(context.hardware_ref()?)
            }
            Device::Software(ref device) => {
                 device.context_surface_size(context.software_ref()?)
            }
        }
    }

    pub fn context_surface_id(&self, context: &Context) -> Result<SurfaceID, Error> {
        match *self {
            Device::Hardware(ref device) => {
                 device.context_surface_id(context.hardware_ref()?)
            }
            Device::Software(ref device) => {
                 device.context_surface_id(context.software_ref()?)
            }
        }
    }

    pub fn context_descriptor_attributes(&self, context_descriptor: &ContextDescriptor)
                                         -> ContextAttributes {
        match *self {
            Device::Hardware(ref device) => {
                 device.context_descriptor_attributes(context_descriptor.hardware().unwrap())
            }
            Device::Software(ref device) => {
                 device.context_descriptor_attributes(context_descriptor.software().unwrap())
            }
        }
    }

    pub fn get_proc_address(&self, context: &Context, symbol_name: &str) -> *const c_void {
        match *self {
            Device::Hardware(ref device) => {
                 device.get_proc_address(context.hardware_ref().unwrap(), symbol_name)
            }
            Device::Software(ref device) => {
                 device.get_proc_address(context.software_ref().unwrap(), symbol_name)
            }
        }
    }
}
