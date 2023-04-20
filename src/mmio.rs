use core::marker::PhantomData;

pub struct MMIO<T>(usize, PhantomData<T>);

impl<T> MMIO<T> {
	pub const fn new(address: usize) -> Self {
		Self(address, PhantomData::<T>)
	}

	pub fn set(&self, value: T) {
		let ptr = self.0 as *mut T;

		unsafe { 
			core::ptr::write_volatile(ptr, value) 
		}
	}

	pub fn get(&self) -> T {
		let ptr = self.0 as *const T;

		unsafe { 
			core::ptr::read_volatile(ptr) 
		}
	}
}

pub struct MMIOSet<T, const SIZE: usize>(usize, PhantomData<T>);

impl<T, const SIZE: usize> MMIOSet<T, SIZE> {
	pub const fn new(address: usize) -> Self {
		Self(address, PhantomData::<T>)
	}

	fn addr(&self, index: usize) -> usize {
		self.0 + (index * core::mem::size_of::<T>())  
	}

	pub fn set(&self, index: usize, value: T) {
		let address = self.addr(index);
		let ptr = address as *mut T;

		unsafe { 
			core::ptr::write_volatile(ptr, value) 
		}
	}

	pub fn get(&self, index: usize) -> T {
		let address = self.addr(index);
		let ptr = address as *const T;

		unsafe { 
			core::ptr::read_volatile(ptr) 
		}
	}
}