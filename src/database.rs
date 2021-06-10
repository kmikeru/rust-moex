use chrono::{NaiveDateTime};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct CandleRecord {
    pub secname: String,
    pub timeframe: i32,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub value: f64,
    pub volume: f64,
    pub begin: NaiveDateTime,
    pub end: NaiveDateTime,
}

pub fn create_table() -> Result<(), rusqlite::Error> {
    let mut conn = Connection::open("test.db")?;
    let sql = "CREATE TABLE if not exists candles(secname varchar(32) not null, timeframe int not null, open real, close real, high real, low real, value real, volume real, begin text not null, end text not null, primary key (secname, timeframe,  begin))";
    conn.execute(sql, [])?;
    conn.close();
    Ok(())
}

pub fn save(candle_records: Vec<CandleRecord>) -> Result<()> {
    let mut conn = Connection::open("test.db")?;
    let tx = conn.transaction()?;
    for c in candle_records.iter() {
        tx.execute(
            "INSERT or ignore INTO candles VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![c.secname, c.timeframe, c.open, c.close, c.high, c.low, c.value, c.volume, c.begin, c.end],
        )?;
    }
    tx.commit()?;
    conn.close();
    Ok(())
}

pub fn get_candles(secname: String, interval: i32, datestart: String) -> Result<Vec<CandleRecord>, rusqlite::Error> {
    let mut conn = Connection::open("test.db")?;
    let mut stmt = conn.prepare("SELECT secname, timeframe, open, close, high, low, value, volume, begin, end from candles where secname = ? and timeframe = ? and begin>?")?;
    let candles_iter = stmt.query_map([secname, interval.to_string(), datestart], |row| {
        Ok(CandleRecord {
            secname: row.get(0)?,
            timeframe: row.get(1)?,
            open: row.get(2)?,
            close: row.get(3)?,
            high: row.get(4)?,
            low: row.get(5)?,
            value: row.get(6)?,
            volume: row.get(7)?,
            begin: row.get(8)?,
            end: row.get(9)?
        })
    })?;
    candles_iter.collect()
}