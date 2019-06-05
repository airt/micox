#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(micox::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use micox::allocator::HEAP_SIZE;
use micox::{serial_print, serial_println};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
  use micox::allocator;
  use micox::memory::{self, BootInfoFrameAllocator};
  use x86_64::VirtAddr;

  micox::init();
  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
  allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

  test_main();
  loop {}
}

#[test_case]
fn simple_allocation() {
  serial_print!("simple_allocation... ");
  let heap_value_1 = Box::new(41);
  let heap_value_2 = Box::new(13);
  assert_eq!(*heap_value_1, 41);
  assert_eq!(*heap_value_2, 13);
  serial_println!("[ok]");
}

#[test_case]
fn large_vec() {
  serial_print!("large_vec... ");
  let n = 1000;
  let mut vec = Vec::new();
  for i in 0..n {
    vec.push(i);
  }
  assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
  serial_println!("[ok]");
}

#[test_case]
fn many_boxes() {
  serial_print!("many_boxes... ");
  for i in 0..HEAP_SIZE {
    let x = Box::new(i);
    assert_eq!(*x, i);
  }
  serial_println!("[ok]");
}

#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
  micox::test_panic_handler(info)
}
