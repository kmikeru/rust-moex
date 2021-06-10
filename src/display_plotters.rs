use chrono::offset::{Local, TimeZone};
use chrono::{Date, Duration, DateTime};
use plotters::prelude::*;

use std::path::Path;
use crate::CandleRecord;

const OUT_FILE_NAME: &'static str = "stock.png";
pub fn display_plotters(candles: &Vec<CandleRecord>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let to_date : DateTime<Local> = Local.from_local_datetime(&candles.last().unwrap().begin).unwrap();
    let from_date : DateTime<Local> = Local.from_local_datetime(&candles[0].begin).unwrap();
    println!("{:#?}", (to_date, from_date));
    let minimum = candles.iter().fold(f64::INFINITY, |acc, x| acc.min(x.low)) as f32;
    let maximum = candles.iter().fold(f64::NEG_INFINITY, |acc, x| acc.max(x.high)) as f32;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(candles[0].secname.to_string() + " Stock Price", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, minimum..maximum)?;

    chart.configure_mesh().light_line_style(&WHITE).draw()?;

    chart.draw_series(
        candles.iter().map(|x| CandleStick::new(Local.from_local_datetime(&x.begin).unwrap(), x.open as f32, x.high  as f32, x.low  as f32, x.close  as f32, &GREEN, &RED, 15))
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
