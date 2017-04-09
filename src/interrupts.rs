//! Interrupts

#[no_mangle]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 97] = [Some(handle_interrupt); 97];

pub static mut HANDLE_INT: Option<fn()> = None;
pub static mut INTS : [bool; 97] = [false; 97];


unsafe extern "C" fn handle_interrupt () {
	let a = 5;
	let iabr_addr = 0xE000E300 as *const [u32; 3];
	let iabr : [u32; 3] = ::core::ptr::read_volatile(iabr_addr);
	for i in 0usize..96 {
		let x = i / 32;
		let y = i % 32;
		INTS[i] |= ((iabr[x] >> y) & 1) == 1;
	}

	if let Some(f) = HANDLE_INT {
		f() 
	}	

	let gintsts_addr = 0x40040014 as *mut u32;
	let gintsts = ::core::ptr::read_volatile(gintsts_addr);
	::core::ptr::write_volatile(gintsts_addr, gintsts);
}
