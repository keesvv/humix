#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

#[cfg(target_arch = "wasm32")]
mod wasm32;

#[cfg(target_arch = "wasm32")]
pub use self::wasm32::*;
