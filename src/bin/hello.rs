#![no_main]
#![no_std]

use test_rs_defmt as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    test_rs_defmt::exit()
}
