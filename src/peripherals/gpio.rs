use crate::{mmio::{MMIO, MMIOSet}, cpu::delay};

/** General purpose I/O nbase address */
const GPIO_BASE_ADDRESS: usize = super::BASE_ADDRESS + 0x200000;

/** GPIO Function Select 0-5 32 R/W */
pub const GPFSL: MMIOSet<u32, 6> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x00);
/** GPIO Pin Output Set 0-1 32 W */
pub const GPSET: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x1C);
/** GPIO Pin Output Clear 0-1 32 W */
pub const GPCLR: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x28);
/** GPIO Pin Level 0-1 32 R */
pub const GPLEV: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x34);
/** GPIO Pin Event Detect Status 0-1 32 R/W */
pub const GPEDS: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x40);
/** GPIO Pin Rising Edge Detect Enable 0-1 32 R/W */
pub const GPREN: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x4C);
/** GPIO Pin Falling Edge Detect Enable 0-1 32 R/W */
pub const GPFEN: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x58);
/** GPIO Pin High Detect Enable 0-1 32 R/W */
pub const GPHEN: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x64);
/** GPIO Pin Low Detect Enable 0-1 32 R/W */
pub const GPLEN: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x70);
/** GPIO Pin Async Rising Edge Detect Enable 0-1 32 R/W */
pub const GPAREN: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x7C);
/** GPIO Pin Async Falling Edge Detect Enable 0-1 32 R/W */
pub const GPAFEN: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x88);
/** GPIO Pin Pull-up/down Enable 0-1 32 R/W */
pub const GPPUD: MMIO<u32> = MMIO::new(GPIO_BASE_ADDRESS + 0x94);
/** GPIO Pin Pull-up/down Enable Clock 0-1 32 R/W */
pub const GPPUDCLK: MMIOSet<u32, 2> = MMIOSet::new(GPIO_BASE_ADDRESS + 0x98);

#[derive(Clone, Copy)]
pub enum GpioFn {
	INPUT  = 0b000,
	OUTPUT = 0b001,
	ALT0   = 0b100,
	ALT1   = 0b101,
	ALT2   = 0b110,
	ALT3   = 0b111,
	ALT4   = 0b011,
	ALT5   = 0b010
}

impl GpioFn {
	pub fn value(&self, pin: u32) -> u32 {
		(*self as u32) << (pin % 10 * 3)
	}
}

pub fn gpio_select_function(pin: u32, func: GpioFn) {
	let value = func.value(pin);
	let index = pin as usize / 10;
	let old_value = GPFSL.get(index);
	GPFSL.set(index, old_value ^ value | value);
}

pub fn gpio_output_set(pin: u32) {
	let value = 1 << (pin % 32);
	let index = pin as usize / 32;
	GPSET.set(index, value);
}

pub fn gpio_output_clear(pin: u32) {
	let value = 1 << (pin % 32);
	let index = pin as usize / 32;
	GPCLR.set(index, value);
}

pub fn gpio_enable(pin: u32) {
	GPPUD.set(0);
	delay(150);
	GPPUDCLK.set(pin as usize / 32, 1 << (pin % 32));
	delay(150);
	GPPUD.set(0);
	GPPUDCLK.set(pin as usize / 32, 0);
}