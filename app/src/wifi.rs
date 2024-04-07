#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::get_ssid;
#[cfg(windows)]
pub use windows::get_signal;
#[cfg(unix)]
mod linux;
#[cfg(unix)]
pub use linux::get_ssid;
#[cfg(unix)]
pub use linux::get_signal;