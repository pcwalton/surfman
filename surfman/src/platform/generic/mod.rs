// surfman/src/platform/generic/mod.rs

#[cfg(any(target_os = "android", all(target_os = "windows", feature = "sm-angle")))]
pub(crate) mod egl;

#[cfg(feature = "sm-osmesa")]
pub mod osmesa;

#[cfg(feature = "sm-universal")]
pub mod universal;
