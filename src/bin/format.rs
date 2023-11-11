#![no_main]
#![no_std]

use defmt::Format;
use test_rs_defmt as _; // global logger + panicking-behavior + memory layout // <- derive attribute

#[derive(Format)]
struct S1<T> {
    x: u8,
    y: T,
}

#[derive(Format)]
struct S2 {
    z: u8,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let s = S1 {
        x: 42,
        y: S2 { z: 43 },
    };
    defmt::println!("s={:?}", s);
    let mut x = 42;
    defmt::println!("x={=u8}", x);
    x /= 7;
    defmt::println!("x={=u8}", x);
    x /= 7;
    defmt::println!("x={=u8}", x);

    test_rs_defmt::exit()
}
