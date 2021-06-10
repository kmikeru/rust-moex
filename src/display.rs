use criterion_plot::prelude::*;
use std::path::Path;
use crate::CandleRecord;

pub fn display(candles: &Vec<CandleRecord>) {
    let xs = candles.clone().iter().enumerate().map(|(i, j)| i).collect::<Vec<_>>();
    let bh = candles.clone().iter().map(|i| i.close).collect::<Vec<_>>();
    let bm = candles.clone().iter().map(|i| i.open).collect::<Vec<_>>();
    let wm = candles.clone().iter().map(|i| i.low).collect::<Vec<_>>();
    let wh = candles.clone().iter().map(|i| i.high).collect::<Vec<_>>();

Figure::new()
    .set(BoxWidth(0.2))
    .set(Output(Path::new("curve.jpg")))
    .set(Size(1280, 720))
    .configure(Axis::BottomX, |a| a.set(Range::Limits(0., 60.)))
    .plot(Candlesticks {
              x: xs.clone(),
              whisker_min: &wm,
              box_min: &bm,
              box_high: &bh,
              whisker_high: &wh,
          },
          |cs| {
              cs.set(Color::Rgb(86, 180, 233))
                .set(Label("Quartiles"))
                .set(LineWidth(2.))
          })
    .draw()
    .ok();
}
