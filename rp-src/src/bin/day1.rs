#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::{iter::zip, ptr::addr_of_mut};
use defmt::{info, warn};
use embedded_alloc::LlffHeap as Heap;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use panic_halt;
use rp235x_hal::{self as hal};
use usb_device::bus::UsbBusAllocator;

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

const INPUT: &'static str = include_str!("../../../inputs/day1.txt");

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[hal::entry]
fn main() -> ! {
    // init allocator
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 128;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }
    let mut pac = hal::pac::Peripherals::take().unwrap();

    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();

    warn!("part 1:");
    let (mut left, mut right) = parse_input();
    // sort numbers
    left.sort();
    right.sort();
    // iter over both, summing the differences
    let answer = zip(left, right).fold(0, |acc, (left, right)| acc + (left - right).abs());
    warn!("{}", answer);

    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(100);
        led_pin.set_low().unwrap();
        timer.delay_ms(100);
    }
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let lines = INPUT.lines();
    let mut ret = (Vec::new(), Vec::new());
    for line in lines {
        let mut inner = line.split_whitespace();
        ret.0.push(inner.next().unwrap().parse().unwrap());
        ret.1.push(inner.next().unwrap().parse().unwrap());
    }

    ret
}

#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 4] = [
    hal::binary_info::rp_program_name!(c"2024-day1"),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"AOC 2024 Day1"),
    hal::binary_info::rp_program_build_attribute!(),
];
