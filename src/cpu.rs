use core::arch::asm;

#[inline(always)]
pub unsafe fn get_core_id() -> u64 {
	let mut core_id: u64;
	asm!("mrs {core_id}, MPIDR_EL1", core_id = out(reg) core_id);
	core_id & 0b11
}

#[inline(always)]
pub unsafe fn set_stack_pointer(sp: u64) {
	asm!("mov sp, {__stack_start}", __stack_start = in(reg) sp);
}

#[inline(always)]
pub fn nop() {
	unsafe { asm!("nop"); }
}

#[inline(always)]
pub fn delay(mut cycles: u64) {
	while cycles > 0 {
		cycles-=1;
		unsafe { asm!(""); }
	}
}