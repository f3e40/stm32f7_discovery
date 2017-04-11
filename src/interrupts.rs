//! Interrupts

extern crate alloc;

#[no_mangle]
pub static INTERRUPTS: [unsafe extern "C" fn(); 97] = [handle_interrupt; 97];

pub static mut INTS : [bool; 97] = [false; 97];
static mut ISRS: [Option<unsafe fn(u8)>; 97] = [None; 97];


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
	
	for (irq, _) in irqs.iter().enumerate().filter(|&(_, &b)| b) {
		match ISRS[irq] {
			Some(ref mut f) => f(irq as u8),
			None => ()
		}
	}
}

extern crate embedded_stm32f7;
use self::embedded_stm32f7::nvic::Nvic;
pub fn enable_interrupt(irq: u8, prio: u8, isr: Option<unsafe fn(u8)>, nvic: &mut Nvic) {
	let iser_no = irq / 32u8;
	let mask = 1 << (irq % 32u8);
	match iser_no {
		0 => nvic.iser0.update(|r| {let old = r.setena(); r.set_setena(old | mask);}),
		1 => nvic.iser1.update(|r| {let old = r.setena(); r.set_setena(old | mask);}),
		2 => nvic.iser2.update(|r| {let old = r.setena(); r.set_setena(old | mask);}),
		_ => ()
	}
	// TODO
	if irq == 77 {
		nvic.ipr19.update(|r| r.set_ipr_n1(prio)); // set priority of irq77
	}

	unsafe { ISRS[irq as usize] = isr; }
}
