use core::arch::{ asm, global_asm };

extern "Rust" {
	pub fn main() -> !;
}

static BOOT_CORE_ID: u64 = 0;
static CORE_ID_MASK: u64 = 0b11;

global_asm!(include_str!("boot.s"), sym CORE_ID_MASK, sym BOOT_CORE_ID, sym main);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { 
		unsafe {
			asm!("wfe");
		}
	}
}