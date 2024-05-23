use linux_embedded_hal::{Delay, I2cdev};
use bosch_bme680::{Bme680, DeviceAddress, Configuration};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Reading sensor data for temperature, humidity, pressure and gas resistance");

    // Initialize the I2C bus
    let i2c_path = "/dev/i2c-1";
    let i2c = I2cdev::new(i2c_path).expect("Failed to open I2C device");

    let delay = Delay;
    let config = Configuration::default();

    // DeviceAddress::Primary represents 0x76, Secondary is 0x77. On my Pi I have to use Secondary.
    let mut bme = match Bme680::new(i2c, DeviceAddress::Secondary, delay, &config, 20) {
        Ok(sensor) => sensor,
        Err(e) => {
            eprintln!("Failed to initialize BME680: {:?}", e);
            return;
        }
    };

    // wait for sensor to stabilize
    thread::sleep(Duration::from_millis(500));

    loop {
        // Measure every two seconds
        thread::sleep(Duration::from_secs(2));
        
        // Perform a measurement
        match bme.measure() {
            Ok(values) => {
                let temperature = values.temperature;
                let humidity = values.humidity;
                let pressure = values.pressure;
                let gas_resistance = values.gas_resistance;

                // Print the sensor values
                println!("Temperature: {:.2} °C", temperature);
                println!("Humidity: {:.2} %", humidity);
                println!("Pressure: {:.2} hPa", pressure / 100.0); // Convert Pa to hPa
                if let Some(gas) = gas_resistance {
                    println!("Gas Resistance: {:.2} Ω", gas);
                } else {
                    println!("Gas Resistance: Not available");
                }
            }
            Err(e) => eprintln!("Failed to measure sensor values: {:?}", e),
        }
    }
}
