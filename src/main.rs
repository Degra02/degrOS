#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(degrOS::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(non_snake_case)]

use core::panic::PanicInfo;
use degrOS::serial_println;
pub mod utils;
pub mod vga_buffer;

use bootloader::{entry_point, BootInfo};
use x86_64::{structures::paging::PageTable, VirtAddr};

entry_point!(kernel_main);

/// First function called at OS startup
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    degrOS::init();

    utils::startup_message();

    // print_all_levels_tables(boot_info.physical_memory_offset);

    #[cfg(test)]
    test_main();

    println!("Startup complete!");
    degrOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use degrOS::{exit_qemu, QemuExitCode};

    serial_println!("[failed]\n");
    serial_println!("Error {}", _info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    degrOS::hlt_loop();
}

fn _print_all_levels_tables(physical_memory_offset: u64) {
    use degrOS::memory::active_level_4_table;

    let phys_mem_offset = VirtAddr::new(physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            // get the physical address from the entry and convert it to virtual
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // print non-empty enties of the level_3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("L3 Entry {}: {:?}", i, entry);

                    let phys = entry.frame().unwrap().start_address();
                    let virt = phys.as_u64() + physical_memory_offset;
                    let ptr = VirtAddr::new(virt).as_mut_ptr();
                    let l2_table: &PageTable = unsafe { &*ptr };

                    // print non-empty enties of the level_2 table
                    for (i, entry) in l2_table.iter().enumerate() {
                        if !entry.is_unused() {
                            println!("  L2 Entry {}: {:?}", i, entry);
                            let phys = entry.frame().unwrap().start_address();
                            let virt = phys.as_u64() + physical_memory_offset;
                            let ptr = VirtAddr::new(virt).as_mut_ptr();
                            let l1_table: &PageTable = unsafe { &*ptr };

                            // print non-empty enties of the level_1 table
                            for (i, entry) in l1_table.iter().enumerate() {
                                if !entry.is_unused() {
                                    println!("   L1 Entry {}: {:?}", i, entry);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
