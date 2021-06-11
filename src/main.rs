use chrono::{NaiveDateTime};
use clap::{Arg, App};
mod database;
mod download;
mod display;
mod display_plotters;
use crate::database::CandleRecord;
use crate::database::create_table;
use crate::database::save;
use crate::database::get_candles;
use crate::download::download;
use crate::display::display;
use crate::display_plotters::display_plotters;

#[derive(Debug)]
struct Candle {
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    value: f64,
    volume: f64,
    begin: NaiveDateTime,
    end: NaiveDateTime,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .about("
        Download and save to sqlite database:
        cargo run --release -- -s VTBX,VTBB,VTBE,VTBA,VTBG,FXWO -i 24 -d 2020-01-01 -a d
        Show:
        cargo run --release -- -s VTBX,VTBB,VTBE,VTBA,VTBG,FXWO -i 24 -d 2020-01-01 -a s
        ")
        .arg(Arg::with_name("secname")
                 .short("s")
                 .long("secname")
                 .takes_value(true)
                 .help("Security (ticker) name, e.g. AFLT"))
        .arg(Arg::with_name("datestart")
                 .short("d")
                 .long("datestart")
                 .takes_value(true)
                 .help("Start date, format YYYY-MM-DD"))
        .arg(Arg::with_name("interval")
                 .short("i")
                 .long("interval")
                 .takes_value(true)
                 .help("Interval in minutes (default 60), use 24 for daily data"))
        .arg(Arg::with_name("action")
                 .short("a")
                 .long("action")
                 .takes_value(true)
                 .help("Action: 'd'ownload, 's'how"))
        .get_matches();

    let action = matches.value_of("action").unwrap_or("s");
    let secname = matches.value_of("secname").unwrap_or("AFLT");
    println!("secname passed is: {}", secname);
    let datestart = matches.value_of("datestart").unwrap_or("2020-01-01");
    println!("datestart passed is: {}", datestart);

    let interval_str = matches.value_of("interval");
    let interval = match interval_str {
        None => {
            println!("No interval given.");
            60
        },
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => {
                    println!("Interval parsed as number {}.", n);
                    n
                },
                Err(_) => {
                    println!("That's not a number! {}", s);
                    60
                },
            }
        }
    };
    println!("Interval: {}", interval);
    if (action == "d") {
        for s in secname.split(",") {
            let candle_records = download(s.to_string(), interval, datestart.to_string())?;
            println!("Downloaded {} records for ticker {}", candle_records.len(), s);
            save(candle_records);
        }
    } else if (action == "s") {
        for s in secname.split(",") {
            let candle_records = get_candles(s.to_string(), interval, datestart.to_string())?;
            display_plotters(&candle_records);
        }
    } else {
        println!("No action specified");
    }
    Ok(())   
}