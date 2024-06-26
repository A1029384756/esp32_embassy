#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp32c3_hal::gpio::{AnyPin, Output, PushPull};
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, IO};
use esp_backtrace as _;
use esp_println::print;

#[embassy_executor::task]
async fn one_second_task(mut out: AnyPin<Output<PushPull>>) {
    let mut count = 0;
    loop {
        let _ = out.toggle();
        esp_println::println!("Pin state: {}", out.is_set_high().unwrap());
        esp_println::println!("Spawn Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let pin_10 = io.pins.gpio10.into_push_pull_output().degrade();

    embassy::init(
        &clocks,
        esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    spawner.spawn(one_second_task(pin_10)).unwrap();

    // This line is for Wokwi only so that the console output is formatted correctly
    print!("\x1b[20h");

    let mut count = 0;
    loop {
        esp_println::println!("Main Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(5_000)).await;
    }
}
