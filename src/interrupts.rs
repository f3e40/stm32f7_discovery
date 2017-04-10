//! Interrupts

#![feature(alloc)]
extern crate alloc;

#[no_mangle]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 97] = [Some(handle_interrupt); 97];

pub static mut HANDLE_INT: Option<alloc::boxed::Box<FnMut() -> ()>> = None;
pub static mut INTS : [bool; 97] = [false; 97];


unsafe extern "C" fn handle_interrupt () {
	let iabr_addr = 0xE000E300 as *const [u32; 3];
	let iabr : [u32; 3] = ::core::ptr::read_volatile(iabr_addr);
	let mut irqs: [bool; 97] = [false; 97];
	for i in 0usize..96 {
		let x = i / 32;
		let y = i % 32;
		let irqi_triggered =((iabr[x] >> y) & 1) == 1;
		INTS[i] |= irqi_triggered;
		irqs[i] = irqi_triggered;
	}

	// USB Interrupt (handle externally)
	if irqs[74..78].iter().any(|b| *b) {
		if let Some(ref mut f) = HANDLE_INT {
			f() 
		}	
	}
}

extern crate embedded_stm32f7;
pub fn enable_interrupt(irq: u8, nvic: &mut embedded_stm32f7::nvic::Nvic) {
	let iser_no = irq / 32u8;
	let mask = 1 << (irq % 32u8);
	match iser_no {
		0 => nvic.iser0.update(|r| {let old = r.setena(); r.set_setena(old | mask);}),
		1 => nvic.iser1.update(|r| {let old = r.setena(); r.set_setena(old | mask);}),
		2 => nvic.iser2.update(|r| {let old = r.setena(); r.set_setena(old | mask);}),
		_ => ()
	}
	// TODO
	nvic.ipr19.update(|r| r.set_ipr_n1(1)); // set priority of irq77
}
