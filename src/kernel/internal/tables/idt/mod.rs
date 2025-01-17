use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

mod hardware_interrupts;
mod idt_handlers;

use idt_handlers::*;
use hardware_interrupts::*;
pub const PIC_LOCATION: u8 = 0x20;
pub const PIC2_LOCATION: u8 = PIC_LOCATION + 8;


pub(self) static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_LOCATION, PIC2_LOCATION) });
pub(self) static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
    unsafe {
        let mut idt = &mut IDT;
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::kernel::internal::DOUBLE_FAULT_IST_INDEX);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt[InterruptIndex::Timer as u8 as usize].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as u8 as usize].set_handler_fn(keyboard_interrupt_handler);
        IDT.load(); 
        PICS.lock().initialize();
        x86_64::instructions::interrupts::enable();
    }

}