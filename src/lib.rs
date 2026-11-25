#![forbid(unsafe_code)]

#[cfg(not(target_os = "linux"))]
compile_error!("SysGhost requires Linux");

pub mod events;
pub mod models;
pub mod monitor;
