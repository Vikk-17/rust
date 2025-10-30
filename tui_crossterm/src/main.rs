// use std::{
//     io,
//     time::{Instant, Duration}
// };
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use ratatui::{
//     buffer::Buffer,
//     layout::Rect,
// };
use color_eyre::Result;


fn main() -> Result<()> {
    // color_eyre::install()?;
    // ratatui::run(|terminal| App::new().run(terminal))
    Ok(())
}

struct App {
    singnal1: SinSignal,
    data1: Vec<(f64, f64)>,
    singnal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

#[derive(Clone)]
struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    const fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self { 
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    
    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

impl App {
    fn new() -> Self {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();

        Self {
            [..]
        }
    }
}
