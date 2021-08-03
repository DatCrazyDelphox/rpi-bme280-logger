use bme280::BME280;
use chrono::prelude::*;
use colored::*;
use linux_embedded_hal::{Delay, I2cdev};
use std::error::Error;
use std::thread;
use std::time;
use termion::*;


fn main() -> Result<(), Box<dyn Error>> {

    // First initialize the sensor and CSV writer

    let mut wtr = csv::Writer::from_path("sensor.csv")?;

    // Write CSV Header (also clears csv file if it exists)
    wtr.write_record(&["Dia", "Hora", "Temperatura", "Humidade", "Pressão"])?;

    // Clear screen
    clear::All;




    thread::spawn(|| {
        let mut bme280 = BME280::new_primary(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
        bme280.init().unwrap();
        loop {

            let measurements = bme280.measure().unwrap();

            let now = Local::now();

            let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
            let day = format!("{:02}/{:02}/{:04}", now.day(), now.month(), now.year());
            let humidity = format!("{:.2}", measurements.humidity);
            let temp = format!("{:.2}", measurements.temperature);
            let _press = format!("{:.2}", (measurements.pressure / 1000 as f32));

            println!("{} - {}", day.white().bold(), time.white().bold());
            println!(
                "Temperatura = {}{}",
                temp.bright_green().bold(),
                "°C".bright_green().bold()
            );
            println!(
                "Humidade = {}{}",
                humidity.bright_green().bold(),
                "%".bright_green().bold()
            );
            println!(
                "Pressão = {} {}",
                _press.bright_green().bold(),
                "KPa".bright_green().bold()
            );
        }
    });



    // Begin infinitely writing sensor data to a CSV
    loop {
        let measurements = bme280.measure().unwrap();

        let now = Local::now();

        let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let day = format!("{:02}/{:02}/{:04}", now.day(), now.month(), now.year());
        let humidity = format!("{:.2}", measurements.humidity);
        let temp = format!("{:.2}", measurements.temperature);
        let _press = format!("{:.2}", (measurements.pressure / 1000 as f32));

        println!("{} - {}", day.white().bold(), time.white().bold());
        println!(
            "Temperatura = {}{}",
            temp.bright_green().bold(),
            "°C".bright_green().bold()
        );
        println!(
            "Humidade = {}{}",
            humidity.bright_green().bold(),
            "%".bright_green().bold()
        );
        println!(
            "Pressão = {} {}",
            _press.bright_green().bold(),
            "KPa".bright_green().bold()
        );
        wtr.serialize((day.to_string(), time.to_string(), temp, humidity, _press))?;
        wtr.flush()?;
        thread::sleep(time::Duration::from_secs(60));
        clear::All;
    }
}
