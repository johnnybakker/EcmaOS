#![no_std]

pub mod mmio;
pub mod cpu;
pub mod peripherals;
mod boot;

pub mod prelude {
	pub use crate::cpu;
	pub use crate::peripherals::*;
}