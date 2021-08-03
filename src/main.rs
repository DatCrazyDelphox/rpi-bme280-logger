use bme280::BME280;
use chrono::prelude::*;
use colored::*;
use linux_embedded_hal::{Delay, I2cdev};
use std::error::Error;
use std::{thread, time, process};
use termion::{clear, cursor};

fn csvwriter() -> Result<(), Box<dyn Error>> {
        let mut bme280 = BME280::new_primary(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
        let mut wtr = csv::Writer::from_path("sensor.csv")?;
        bme280.init().unwrap();
        wtr.write_record(&["Dia", "Hora", "Temperatura", "Humidade", "Pressão"])?;
        loop {
        let measurements = bme280.measure().unwrap();

            let now = Local::now();

            let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
            let day = format!("{:02}/{:02}/{:04}", now.day(), now.month(), now.year());
            let humidity = format!("{:.2}", measurements.humidity);
            let temp = format!("{:.2}", measurements.temperature);
            let _press = format!("{:.2}", (measurements.pressure / 1000 as f32));
            wtr.serialize((day.to_string(), time.to_string(), temp, humidity, _press))?;
            wtr.flush()?;
            thread::sleep(time::Duration::from_secs(60));
        }
}

fn main() {

    // First initialize the sensor and CSV writer
    let mut bme280 = BME280::new_primary(I2cdev::new("/dev/i2c-1").unwrap(), Delay);
    bme280.init().unwrap();
    thread::spawn(|| {
            if let Err(err) = csvwriter() {
        println!("{}", err);
        process::exit(1);
        }
    });
    print!("{}{}", clear::All, cursor::Goto(1, 1));
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
        thread::sleep(time::Duration::from_secs(1));
        print!("{}", clear::All);
    };
}
