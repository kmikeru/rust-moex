use std::collections::HashMap;
use chrono::{NaiveDateTime};
use crate::CandleRecord;

pub fn download(secname: String, interval: i32, datestart: String) -> Result<Vec<CandleRecord>, Box<dyn std::error::Error>> {
    let url = format!("https://iss.moex.com/iss/engines/stock/markets/shares/securities/{}/candles.json?interval={}&from={}", secname, interval, datestart);
    println!("{:#?}", url);
    let resp = reqwest::blocking::get(url)?
        .json::<HashMap<String, serde_json::Value>>()?;

    let resp_iter = resp["candles"]["data"].as_array().unwrap().iter();
    let candle_records: Vec<CandleRecord> = resp_iter.map(|x| 
        CandleRecord{
            secname: secname.to_string(),
            timeframe: interval,
            open: x[0].as_f64().unwrap(), 
            close: x[1].as_f64().unwrap(),
            high: x[2].as_f64().unwrap(),
            low: x[3].as_f64().unwrap(),
            value: x[4].as_f64().unwrap(),
            volume: x[5].as_f64().unwrap(),
            begin: NaiveDateTime::parse_from_str(x[6].as_str().unwrap(), "%Y-%m-%d %H:%M:%S").unwrap(),
            end: NaiveDateTime::parse_from_str(x[7].as_str().unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()
        }
    ).collect();
    return Ok(candle_records);
}