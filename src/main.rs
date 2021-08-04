use bme280::BME280;
use chrono::prelude::*;
use colored::*;
use linux_embedded_hal::{Delay, I2cdev};
use std::error::Error;
use std::{thread, time};
use termion::{clear, cursor};

// Main Variables
const I2C: &str = "/dev/i2c-1";
const FILEPATH: &str = "./sensor.csv";
const INTERVAL: u16 = 60; // In seconds

fn main() -> Result<(), Box<dyn Error>> {
    // First initialize the sensor
    let mut bme280 = BME280::new_primary(I2cdev::new(I2C).unwrap(), Delay);
    bme280.init().unwrap();
    let mut wtr = csv::Writer::from_path(FILEPATH)?;
    wtr.write_record(&["Dia", "Hora", "Temperatura", "Humidade", "Pressão"])?;

    // Clear screen
    print!("{}{}", clear::All, cursor::Goto(1, 1));

    let mut trigger: u16 = INTERVAL + 1;
    // Print sensor data to screen every second forever
    loop {
        let measurements = bme280.measure().unwrap();

        let now = Local::now();

        let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let day = format!("{:02}/{:02}/{:04}", now.day(), now.month(), now.year());
        let humidity = format!("{:.2}", measurements.humidity);
        let temp = format!("{:.2}", measurements.temperature);
        let _press = format!("{:.7}", (measurements.pressure / 1000 as f32));

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
            "kPa".bright_green().bold()
        );

        if trigger == INTERVAL || trigger == INTERVAL + 1 {
            wtr.serialize((day.to_string(), time.to_string(), temp, humidity, _press))?;
            wtr.flush()?;
            println!(
                "{}{}",
                "Wrote to csv at ".white().bold(),
                time.bright_green().bold()
            );
            trigger = 0;
        }

        trigger += 1;
        println!("Trigger: {}", trigger);

        thread::sleep(time::Duration::from_secs(1));

        print!("{}{}", clear::All, cursor::Goto(1, 1));
    }
}
