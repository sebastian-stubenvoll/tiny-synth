#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;


//BSP alias
use pimoroni_tiny2040 as bsp;

use bsp::hal::{
    clocks::{Clock, ClockSource, ClocksManager},
    pac,
    pll::{common_configs::PLL_USB_48MHZ, setup_pll_blocking, PLLConfig},
    sio::Sio,
    xosc::setup_xosc_blocking
};

use fugit::{Rate, RateExtU32};

const XTAL_FREQ_HZ: u32 = 12_000_000u32;




#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    // Set up overclocking

    let mut clocks = ClocksManager::new(pac.CLOCKS);
    let xosc = setup_xosc_blocking(pac.XOSC, XTAL_FREQ_HZ.Hz())
        .ok()
        .unwrap();

    // Init custom PLL_SYS struct
    // Clock will run at vco_freq / ( refdiv * post_div1 * post_div2 )
    // i.e. 1500e+6 / (1 * 3 * 2) = 250e+6
    let pll_sys_freq: PLLConfig = PLLConfig {
        vco_freq: 1500.MHz(),
        refdiv: 1,
        post_div1: 3,
        post_div2: 2,
    };

    // Setup the PLLs
    let pll_sys = setup_pll_blocking(
        pac.PLL_SYS,
        xosc.operating_frequency().into(),
        pll_sys_freq,
        &mut clocks,
        &mut pac.RESETS,
    )
        .ok()
        .unwrap();

    let pll_usb = setup_pll_blocking(
        pac.PLL_USB,
        xosc.operating_frequency().into(),
        PLL_USB_48MHZ,
        &mut clocks,
        &mut pac.RESETS,
    )
        .ok()
        .unwrap();

    // Start clocks
    clocks.reference_clock.configure_clock(&xosc, xosc.get_freq()).ok();
    clocks.system_clock.configure_clock(&pll_sys, pll_sys.get_freq()).ok();
    clocks.usb_clock.configure_clock(&pll_usb, pll_usb.get_freq()).ok();
    clocks.adc_clock.configure_clock(&pll_usb, pll_usb.get_freq()).ok();
    clocks.rtc_clock.configure_clock(&pll_usb, 46875u32.Hz()).ok();
    clocks.peripheral_clock.configure_clock(&clocks.system_clock, clocks.system_clock.freq()).ok();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Set the pins to their default state
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_green = pins.led_green.into_push_pull_output();
    let mut led_red = pins.led_red.into_push_pull_output();
    let mut led_blue = pins.led_blue.into_push_pull_output();
    led_green.set_high().unwrap();
    led_red.set_high().unwrap();
    led_blue.set_high().unwrap();


    loop {
        led_green.set_low().unwrap();
        delay.delay_ms(100);
        led_green.set_high().unwrap();
        delay.delay_ms(400);
    }
}

// End of file
