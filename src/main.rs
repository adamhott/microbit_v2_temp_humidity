#![no_main]
#![no_std]

use core::*;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use microbit::hal::gpio::{p0::Parts as P0Parts, p1::Parts as P1Parts};
use microbit::hal::twim::{Twim, Pins as TwimPins};
use microbit::hal::pac::Peripherals;

use sht31::SHT31;
use sht31::prelude::*;
use sht31::error::SHTError::InvalidStatusChecksumError;
use sht31::error::SHTError::InvalidTemperatureChecksumError;
use sht31::error::SHTError::InvalidHumidityChecksumError;
use sht31::error::SHTError::ReadingTimeoutError;
use sht31::error::SHTError::WriteReadI2CError;
use sht31::error::SHTError::WriteI2CError;
use sht31::error::SHTError::PlaceholderError;


#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Program Starting!");

    let peripherals = Peripherals::take().expect("Couldn't initialize peripherals.");

    let cortex_peripherials = cortex_m::Peripherals::take().expect("Failed to get Cortex-M peripherals");
    
    let gpio_p0 = P0Parts::new(peripherals.P0);
    let gpio_p1 = P1Parts::new(peripherals.P1);

    rprintln!("Peripherals taken correctly");

    // Setup i2C Pins
    // Pinout reference: https://tech.microbit.org/hardware/edgeconnector/#pins-and-signals
    let i2c_pins = TwimPins {
        //Switched Pins for Test
        scl: gpio_p0.p0_26.into_floating_input().degrade(), // SCL pin
        sda: gpio_p1.p1_00.into_floating_input().degrade(), // SDA pin
    };

    let i2c = Twim::new(peripherals.TWIM0, i2c_pins, microbit::hal::twim::Frequency::K100);

    //Set delay - 64 is Hertz rate
    let delay = cortex_m::delay::Delay::new(cortex_peripherials.SYST, 10_000_000);

    // In periodic mode, the sensor keeps updating the reading
    // without needing to measure
    let mut sensor = SHT31::new(i2c, delay)
        .with_mode(Periodic::new().with_mps(MPS::Half))
        .with_accuracy(Accuracy::High)
        .with_unit(TemperatureUnit::Fahrenheit);
    
    rprintln!("Sensor Defined");

    // Trigger the measure before running your loop to initialize the periodic mode
    match sensor.measure() {
        Ok(_) => rprintln!("Measurement triggered successfully"),
        Err(e) => rprintln!("Failed to trigger measure: {:?}", e),
    }
    
    rprintln!("Measure triggered");


    loop {

        //Add delay between loop iterations
        cortex_m::asm::delay(32_000_000);

        // Read temperature and humidity
        match sensor.read() {

            //Reading is successful
            Ok(reading) => {

                let temperature = reading.temperature;
                let humidity = reading.humidity;

                // Output temperature and humidity to terminal
                rprintln!("Temperature : {:?}", temperature);
                rprintln!("Humiditiy: {:?}", humidity);


                // Check if humidity is a negative number
                if humidity < 0.0 {

                    rprintln!("Error: Negative relative humidity, this is impossible.");

                // Check if humidity starts with a decimal point in it
                } else if humidity < 1.0 && humidity >= 0.0 {

                    rprintln!("Error: Starting decimal point, this is Mars.");

                } else  {

                    rprintln!("Data Collection Cycle Complete");

                }
            },  
            
            // Check for sensor read error
            Err(e) => {
                match e {
                    sht31::error::SHTError::InvalidStatusChecksumError { bytes_start, bytes_end, expected_checksum, calculated_checksum } => {
                        rprintln!("Invalid Status Checksum Error!");
                        },
                    sht31::error::SHTError::InvalidTemperatureChecksumError { bytes_start, bytes_end, expected_checksum, calculated_checksum } => {
                        rprintln!("Invalid Temperature Checksum Error!");
                        },
                    sht31::error::SHTError::InvalidHumidityChecksumError { bytes_start, bytes_end, expected_checksum, calculated_checksum } => {
                        rprintln!("Invalid Humidity Checksum Error!");
                        },
                    sht31::error::SHTError::ReadingTimeoutError => {
                        rprintln!("Reading Timeout Error!");
                        },
                    sht31::error::SHTError::WriteReadI2CError => {
                        rprintln!("Write Read I2C Error!");
                        },
                    sht31::error::SHTError::WriteI2CError => {
                        rprintln!("Write I2C Error!");
                        },
                    sht31::error::SHTError::PlaceholderError => {
                        rprintln!("Placeholder Error!");
                        },
                }
            }

        }   
        
    }
}