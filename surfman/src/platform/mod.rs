//! Platform-specific backends.

pub mod generic;
#[cfg(feature = "sm-software")]
pub use generic::osmesa as software;

#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "android")]
pub use android as hardware;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos as hardware;

#[cfg(unix)]
pub mod unix;
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]
pub use unix::x11 as hardware;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(all(target_os = "windows", feature = "sm-angle"))]
pub use windows::angle as hardware;
#[cfg(all(target_os = "windows", not(feature = "sm-angle")))]
pub use windows::wgl as hardware;

#[cfg(feature = "sm-software-default")]
pub use software as default;
#[cfg(feature = "sm-universal-default")]
pub use generic::universal as default;
#[cfg(all(not(feature = "sm-universal-default"), not(feature = "sm-software-default")))]
pub use hardware as default;
