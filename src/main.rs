use bme280::BME280;
use chrono::prelude::*;
use colored::*;
use linux_embedded_hal::{Delay, I2cdev};
use std::error::Error;
use std::{thread, time};
use termion::{clear, cursor};

// Main Variables
const I2C: &str = "/dev/i2c-1";        // Path to i2C bus
const FILEPATH: &str = "./sensor.csv"; // Path to csv log file
const INTERVAL: u16 = 1;               // In minutes

struct Info {
    day: String,
    hour: String,
    temp: String,
    hum: String,
    press: String
}


// Main stuff
fn main() -> Result<(), Box<dyn Error>> {


    // First initialize the sensor
    let mut bme280 = BME280::new_primary(I2cdev::new(I2C).unwrap(), Delay);
    bme280.init().unwrap();
    let mut wtr = csv::Writer::from_path(FILEPATH)?;
    wtr.write_record(&["Day", "Hour", "Temp", "Hum", "Pressure"])?;

    // Clear screen
    print!("{}{}", clear::All, cursor::Goto(1, 1));

    let mut trigger: u16 = (INTERVAL * 60) + 1;
    let mut lastwrt = String::from("Never");
    // Print sensor data to screen every second forever
    loop {
        println!("{} - {}", poll().day.white().bold(), poll().hour.white().bold());
        println!(
            "Temperature = {}{}",
            poll().temp.bright_green().bold(),
            "°C".bright_green().bold()
        );
        println!(
            "Humidity = {}{}",
            poll().hum.bright_green().bold(),
            "%".bright_green().bold()
        );
        println!(
            "Pressure = {} {}",
            poll().press.bright_green().bold(),
            "kPa".bright_green().bold()
        );

        if trigger == INTERVAL || trigger == INTERVAL + 1 {
            wtr.serialize((poll().day, poll().hour, poll().temp, poll().hum, poll().press))?;
            wtr.flush()?;
            println!(
                "{}{}",
                "Wrote to csv at ".white().bold(),
                poll().hour.bright_green().bold()
            );
            trigger = 0;
            lastwrt = poll().hour;
        }

        trigger += 1;
        println!("Trigger: {}", trigger);
        println!("{}{}", "\nLast csv write: ".white().bold(), lastwrt.bright_yellow().bold());

        thread::sleep(time::Duration::from_secs(1));

        print!("{}{}", clear::All, cursor::Goto(1, 1));
    }
}

fn poll() -> Info{
    let mut bme280 = BME280::new_primary(I2cdev::new(I2C).unwrap(), Delay);
    let now = Local::now();
    let measurements = bme280.measure().unwrap();
    let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
    let day = format!("{:02}/{:02}/{:04}", now.day(), now.month(), now.year());
    let hum = format!("{:.2}", measurements.humidity);
    let temp = format!("{:.2}", measurements.temperature);
    let press = format!("{:.7}", (measurements.pressure / 1000 as f32));
    return Info {
        day,
        hour: time,
        temp,
        hum,
        press
    };
}


