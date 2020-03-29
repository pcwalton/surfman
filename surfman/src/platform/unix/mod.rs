// surfman/surfman/src/platform/unix/mod.rs
//
//! Backends specific to Unix-like systems, particularly Linux.

// The default when x11 is enabled
#[cfg(all(feature = "sm-x11", unix, not(any(target_os = "macos", target_os = "android"))))]
pub mod default;

// The default when x11 is not enabled
#[cfg(not(all(feature = "sm-x11", unix, not(any(target_os = "macos", target_os = "android")))))]
pub use wayland as default;

#[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
pub mod generic;

#[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
pub mod wayland;
#[cfg(all(feature = "sm-x11", unix, not(any(target_os = "macos", target_os = "android"))))]
pub mod x11;
