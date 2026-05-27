#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_hal::gpio::{Level, Output, OutputConfig};


esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn blink(mut led: Output<'static>){
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await; // should blink 2x per second
    }
}

#[embassy_executor::task]
async fn servo_task(mut servo_pin: Output<'static>) {
    let pulse_min = 1000;      // min servo angle 
    let pulse_max = 2000;      // max servo angle
    let step_size = 15;      // rotation size
    let delay_ms = 10;         // update speed in milliseconds
    
    loop {
        // forward sweep
        for pulse in (pulse_min..=pulse_max).step_by(step_size) {
            servo_pin.set_high();
            Timer::after(Duration::from_micros(pulse)).await;
            servo_pin.set_low();
            Timer::after(Duration::from_millis(delay_ms)).await;
        }
        
        // back sweep
        for pulse in (pulse_min..=pulse_max).rev().step_by(step_size) {
            servo_pin.set_high();
            Timer::after(Duration::from_micros(pulse)).await;
            servo_pin.set_low();
            Timer::after(Duration::from_millis(delay_ms)).await;
        }
    }
}


#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c6 -o unstable-hal -o embassy -o esp-backtrace -o defmt -o vscode

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    let led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
    let servo_pin = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    info!("Embassy initialized");

    let led_token = blink(led).unwrap();
    let servo_token = servo_task(servo_pin).unwrap();

    spawner.spawn(led_token);
    spawner.spawn(servo_token); 

    loop {        
        Timer::after(Duration::from_secs(1)).await;
    }

}
