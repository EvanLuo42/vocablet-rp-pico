#![no_std]
#![no_main]

use bsp::{
    entry, 
    hal::{
        gpio::FunctionSpi,
        spi::Spi,
        clocks::{init_clocks_and_plls, Clock},
        pac::{
            Peripherals,
            CorePeripherals
        },
        sio::Sio,
        watchdog::Watchdog, fugit::RateExtU32,
    },
    Pins,
};

use defmt::*;
use defmt_rtt as _;
use embedded_hal::{spi::MODE_0, digital::v2::OutputPin};
use panic_probe as _;

use cortex_m::delay::Delay;

use epd_waveshare::{
    epd7in5_v2::{
        Epd7in5 as Epd,
        Display7in5 as EpdDisplay
    },
    prelude::WaveshareDisplay, 
    color::Color, 
    graphics::Display
};

use rp_pico as bsp;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let spi_sclk = pins.gpio10.into_function::<FunctionSpi>();
    let spi_mosi = pins.gpio11.into_function::<FunctionSpi>();
    let spi = Spi::<_, _, _, 8>::new(pac.SPI1, (spi_mosi, spi_sclk));
    let mut spi = spi.init(
        &mut pac.RESETS, 
        clocks.peripheral_clock.freq(), 
        4_000_000u32.Hz(), 
        MODE_0
    );

    let mut cs = pins.gpio9.into_push_pull_output();
    cs.set_high().unwrap();
    let busy = pins.gpio13.into_pull_up_input();
    let dc = pins.gpio8.into_push_pull_output();
    let rst = pins.gpio12.into_push_pull_output();

    let mut epd = Epd::new(
        &mut spi,
        cs,
        busy,
        dc,
        rst,
        &mut delay
    ).unwrap();

    let mut display = EpdDisplay::default();

    epd.wake_up(&mut spi, &mut delay).unwrap();
    epd.clear_frame(&mut spi, &mut delay).unwrap();
    
    display.clear_buffer(Color::White);

    loop {
        info!("");
    }
}