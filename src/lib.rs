#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
  gdt::init();
  interrupts::init_idt();
  interrupts::init_pics();
  x86_64::instructions::interrupts::enable();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
  exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &::core::panic::PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);
  exit_qemu(QemuExitCode::Failed);
  hlt_loop();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
  use x86_64::instructions::port::Port;
  unsafe {
    let mut port = Port::new(0xf4);
    port.write(exit_code as u32);
  }
}

pub fn hlt_loop() -> ! {
  loop {
    x86_64::instructions::hlt();
  }
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
  init();
  test_main();
  hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
  test_panic_handler(info)
}
