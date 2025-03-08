#![no_std]
#![no_main]

use panic_halt as _;
use syterkit::{entry, mctl, println, Clocks, Peripherals};

#[entry]
fn main(p: Peripherals, _c: Clocks) {
    println!("DDR Init");
    let dram = mctl::init(&p.ccu, &p.phy);
    println!("{}M 🐏", dram.len() / (1024 * 1024));
}
