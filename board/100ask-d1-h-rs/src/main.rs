#![no_std]
#![no_main]

use panic_halt as _;
use syterkit::{clock_dump, entry, println, show_banner, Clocks, Peripherals};

#[entry] // This macro would initialize system clocks.
fn main(p: Peripherals, _c: Clocks) {
    // Display the bootloader banner.
    show_banner();

    // Initialize the DRAM.
    let dram_size = syterkit::mctl::init(&p.ccu);
    println!("DRAM size: {}M 🐏", dram_size);

    // Dump information about the system clocks.
    clock_dump(&p.ccu);
}
