use bme280::BME280;
use chrono::prelude::*;
use clap::{App, Arg};
use colored::*;
use linux_embedded_hal::{Delay, I2cdev};
use std::error::Error;
use std::{thread, time};
use termion::{clear, cursor};

// Main Variables
const I2C: &str = "/dev/i2c-1"; // Path to i2C bus

// Main stuff
fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("RPi BME280 Logger")
        .version("1.2.0")
        .author("Ferrah Aiko Wolf <ferrahwolfeh@protonmail.com>")
        .about("Logs data from a BME280 sensor connected to I2C Bus 1")
        .arg(
            Arg::with_name("config")
                .short("i")
                .long("interval")
                .value_name("Minutes")
                .help("Set a custom polling interval in minutes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filename")
                .help("Sets the output file to write sensor data")
                .index(1),
        )
        .get_matches();

    let interval: u16 = matches
        .value_of("interval")
        .unwrap_or("1")
        .parse::<u16>()
        .unwrap();

    // First initialize the sensor
    let mut bme280 = BME280::new_primary(I2cdev::new(I2C).unwrap(), Delay);
    bme280.init().unwrap();
    let mut wtr = csv::Writer::from_path(matches.value_of("filename").unwrap_or("./sensor.csv"))?;
    wtr.write_record(&["Hour", "Temp", "Hum", "Pressure"])?;

    // Clear screen
    print!("{}{}", clear::All, cursor::Goto(1, 1));

    let mut trigger: u16 = (interval * 60) + 1;
    let mut lastwrt = String::from("Never");

    // Print sensor data to screen every second forever
    loop {
        let now = Local::now();

        let measurements = bme280.measure().unwrap();
        let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let hum = format!("{:.2}", measurements.humidity);
        let temp = format!("{:.2}", measurements.temperature);
        let press = format!("{:.7}", (measurements.pressure / 1000 as f32));

        println!("{}", time.white().bold());
        println!(
            "Temperature = {}{}
            Humidity = {}{}
            Pressure = {} {}",
            temp.bright_green().bold(),
            "°C".bright_green().bold(),
            hum.bright_green().bold(),
            "%".bright_green().bold(),
            press.bright_green().bold(),
            "kPa".bright_green().bold()
        );

        if trigger == (interval * 60) || trigger == (interval * 60) + 1 {
            println!(
                "{}{}",
                "Wrote to csv at ".white().bold(),
                time.bright_green().bold()
            );
            wtr.serialize((&time, temp, hum, press))?;
            wtr.flush()?;
            trigger = 0;
            lastwrt = time;
        }

        println!(
            "{} {}
            {}{}",
            (60 - trigger),
            "seconds to next write".white().bold(),
            "\nLast csv write: ".white().bold(),
            lastwrt.bright_yellow().bold()
        );

        trigger += 1;

        thread::sleep(time::Duration::from_secs(1));

        print!("{}{}", clear::All, cursor::Goto(1, 1));
    }
}
