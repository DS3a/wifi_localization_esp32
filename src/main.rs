use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::fmt::Write;

use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::wifi::WifiDriver;
// use esp_idf_hal::serial;
use esp_idf_hal::{prelude::*, uart, gpio};
use esp_idf_hal::gpio::InputPin;

fn get_dist(power: f64, power_1m: f64) -> f64 {
    let log_dist_div_20 = (-power - power_1m) / 20.0f64;
    let distance = f64::powf(10.0, log_dist_div_20);
    return distance;
}


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let config = uart::config::Config::default().baudrate(Hertz(115_200));

    let mut uart0: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart0,
        pins.gpio43,
        pins.gpio44,
        Some(pins.gpio16),
        Some(pins.gpio15),
        &config
    ).unwrap();

    let ching_chong_measurepower: f64 = 37.0;
    let galaxy_measurepower: f64 = 43.0;
    let pixel_measurepoer: f64 = 44.0;

    let wifi_modem: Modem = unsafe { Modem::new() };
    if let Ok(system_event_loop) = EspSystemEventLoop::take() {
        if let Ok(mut driver) = WifiDriver::new(wifi_modem, system_event_loop, None) {
            println!("Driver Initialized!");
            let sta_config = Configuration::Client(ClientConfiguration::default());
            driver.set_configuration(&sta_config);
            driver.disconnect();

            loop {
                let mut pixel_dist: f64 = 0f64;
                let mut galaxy_dist: f64 = 0f64;
                let mut chong_dist: f64 = 0f64;
                if let Ok(aps) = driver.scan() {
                    println!("Found {} access points", aps.len());
                    for ap in aps {
                        if ap.ssid == "Normal" {
                            println!("The strength of Normal is : {}", ap.signal_strength);
                            println!(
                                "The distance of Normal is : {}",
                                get_dist(ap.signal_strength as f64, ching_chong_measurepower)
                            );
                        } else if ap.ssid == "Galaxys20" {
                            println!("The strength of Galaxys20 is : {}", ap.signal_strength);
                            println!(
                                "The distance of Galaxys20 is : {}",
                                get_dist(ap.signal_strength as f64, galaxy_measurepower)
                            );
                        }
                        if ap.ssid == "Pixel_2904" {
                            println!("The strength of Pixel_2904 is : {}", ap.signal_strength);
                            println!(
                                "The distance of Pixel_2904 is : {}",
                                get_dist(ap.signal_strength as f64, galaxy_measurepower)
                            );
                        }
                    }
                }
            }
        } else {
            println!("Failed to initialize driver");
        }
    } else {
        println!("Failed to take system loop");
    }
}
