//! An adapter abstraction that can choose between hardware and software rendering.

use crate::Error;
use crate::platform::hardware::adapter::Adapter as HWAdapter;
use crate::platform::software::adapter::Adapter as SWAdapter;

#[derive(Clone, Debug)]
pub enum Adapter {
    Hardware(HWAdapter),
    Software(SWAdapter),
}

impl Adapter {
    /// Returns the "best" adapter on this system.
    pub fn default() -> Result<Adapter, Error> {
        match Adapter::hardware() {
            Ok(adapter) => Ok(adapter),
            Err(_) => Adapter::software(),
        }
    }

    /// Returns the "best" hardware adapter on this system.
    #[inline]
    pub fn hardware() -> Result<Adapter, Error> {
        HWAdapter::default().map(Adapter::Hardware)
    }

    /// Returns the "best" software adapter on this system.
    #[inline]
    pub fn software() -> Result<Adapter, Error> {
        SWAdapter::default().map(Adapter::Software)
    }
}
