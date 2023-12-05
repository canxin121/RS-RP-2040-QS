#![no_std]
#![no_main]

use defmt;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use fugit::RateExtU32;
use panic_probe as _;
use rp2040_hal::{
    clocks::init_clocks_and_plls,
    entry, pac,
    sio::Sio,
    uart::{DataBits, StopBits, UartConfig, UartPeripheral},
    watchdog::Watchdog,
    Clock,
};
use rp_pico as bsp;

const EXTERNAL_XTAL_FREQ_HZ: u32 = 12_000_000u32;
#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    // External high-speed crystal on the pico board is 12Mhz
    let clocks = init_clocks_and_plls(
        EXTERNAL_XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let uart = UartPeripheral::new(
        pac.UART0,
        (pins.gpio0.into_function(), pins.gpio1.into_function()),
        &mut pac.RESETS,
    )
    .enable(
        UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();
    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        defmt::println!("on!");
        uart.write_full_blocking(b"on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        defmt::println!("off!");
        uart.write_full_blocking(b"off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
