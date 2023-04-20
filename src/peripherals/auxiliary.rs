use core::arch::asm;

use crate::mmio::MMIO;

use super::gpio;

/** Auxiliary base address */
pub const AUX_BASE_ADDRESS: usize = super::BASE_ADDRESS + 0x215000;

/** Auxiliary Interrupt status */
pub const AUX_IRQL: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x00);
/** Auxiliary enables */
pub const AUX_ENBL: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x04);

/** Mini Uart I/O Data */
pub const AUX_MU_IO: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x40);
/** Mini Uart Interrupt Enable  */
pub const AUX_MU_IER: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x44);
/** Mini Uart Interrupt Identify  */
pub const AUX_MU_IIR: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x48);
/** Mini Uart Line Control */
pub const AUX_MU_LCR: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x4C);
/** Mini Uart Modem Control */
pub const AUX_MU_MCR: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x50);
/** Mini Uart Line Status */
pub const AUX_MU_LSR: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x54);
/** Mini Uart Modem Status */
pub const AUX_MU_MSR: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x58);
/** Mini Uart Scratch */
pub const AUX_MU_SCR: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x5C);
/** Mini Uart Extra Control */
pub const AUX_MU_CNTL: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x60);
/** Mini Uart Extra Status */
pub const AUX_MU_STAT: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x64);
/** Mini Uart Baudrate */
pub const AUX_MU_BAUD: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x68);

/** SPI 1 Control register 0 */
pub const AUX_SPI0_CNTL0: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x80);
/** SPI 1 Control register 1 */
pub const AUX_SPI0_CNTL1: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x84);
/** SPI 1 Status */
pub const AUX_SPI0_STAT: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x88);
/** SPI 1 Data */
pub const AUX_SPI0_IO: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x90);
/** SPI 1 Peek */
pub const AUX_SPI0_PEEK: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0x94);

/** SPI 2 Control register 0 */
pub const AUX_SPI1_CNTL0: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0xC0);
/** SPI 2 Control register 1 */
pub const AUX_SPI1_CNTL1L: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0xC4);
/** SPI 2 Status */
pub const AUX_SPI1_STAT: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0xC8);
/** SPI 2 Data */
pub const AUX_SPI1_IO: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0xD0);
/** SPI 2 Peek  */
pub const AUX_SPI1_PEEK: MMIO<u32> = MMIO::new(AUX_BASE_ADDRESS + 0xD4);

pub const SYS_CLOCK_FREQ: usize = 250_000_000;
pub const TXD: u32 = 14;
pub const RXD: u32 = 15;

pub fn mini_uart_init(baudrate: usize) {

	gpio::gpio_select_function(TXD, gpio::GpioFn::ALT5);
	gpio::gpio_select_function(RXD, gpio::GpioFn::ALT5);

	gpio::gpio_enable(TXD);
	gpio::gpio_enable(RXD);

	AUX_ENBL.set(1);
	AUX_MU_CNTL.set(0);
	AUX_MU_IER.set(0);
	AUX_MU_LCR.set(3);
	AUX_MU_MCR.set(0);
	AUX_MU_BAUD.set((SYS_CLOCK_FREQ / baudrate / 8 - 1) as u32);
	AUX_MU_CNTL.set(3);

	mini_uart_clear();
	mini_uart_send('\n');
	mini_uart_send('\n');
}

pub fn mini_uart_send(c: char) {
 	while AUX_MU_LSR.get() & 0x20 == 0 {
		unsafe { asm!("nop") }
	}
	
	AUX_MU_IO.set(c as u32);
}

pub fn mini_uart_receive() -> char {
	while AUX_MU_LSR.get() & 0x01 == 0 {
		unsafe { asm!("nop") }
	}
	
	unsafe {
		char::from_u32_unchecked(AUX_MU_IO.get() & 0xFF)
	}
}

pub fn mini_uart_clear() {
	while AUX_MU_LSR.get() & 0x01 == 1 {
		AUX_MU_IO.get();
	}
} 

pub fn mini_uart_send_string(str: &str) {
	for c in str.chars() {
		mini_uart_send(c)
	}
}

pub fn logn<const N: usize>(num: usize) -> u32 {
	let mut res: u32 = 0;
	let mut pow: usize = N;
	while num >= pow { 
		res+=1; 

		if res == 19 {
			break;
		}

		pow *= N;
	}
	res
}

pub fn mini_uart_send_num(mut num: usize) {
	const C0: u32 = '0' as u32;
	const N: usize = 10;
	let mut power = logn::<N>(num);

	while power > 0 {
		let n: u32 = (num / N.pow(power)) as u32;
		
		mini_uart_send(unsafe {
			char::from_u32_unchecked((C0 + n) & 255)
		});

		num -= N.pow(power) * n as usize;
		power -= 1;
	}

	mini_uart_send(unsafe {
		char::from_u32_unchecked((C0 + num as u32) & 255)
	});
}