//! A device abstraction that can switch between hardware and software rendering.

use crate::{Error, GLApi};
use crate::platform::hardware::device::Device as HWDevice;
use crate::platform::software::device::Device as SWDevice;
use super::adapter::Adapter;

pub enum Device {
    Hardware(HWDevice),
    Software(SWDevice),
}

impl Device {
    #[inline]
    pub fn new(adapter: &Adapter) -> Result<Device, Error> {
        match *adapter {
            Adapter::Hardware(ref adapter) => HWDevice::new(adapter).map(Device::Hardware),
            Adapter::Software(ref adapter) => SWDevice::new(adapter).map(Device::Software),
        }
    }

    #[inline]
    pub fn adapter(&self) -> Adapter {
        match *self {
            Device::Hardware(ref device) => Adapter::Hardware(device.adapter()),
            Device::Software(ref device) => Adapter::Software(device.adapter()),
        }
    }

    // FIXME(pcwalton): This should take `self`!
    #[inline]
    pub fn gl_api() -> GLApi {
        GLApi::GL
    }
}
