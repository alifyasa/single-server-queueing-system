use log::LevelFilter;
use std::io::Write;

pub struct Logger {}

impl Logger {
    pub fn init(level_filter: LevelFilter) {
        env_logger::Builder::new()
            .format(|buf, record| {
                let mut level_style = buf.style();
                level_style.set_bold(true).set_color(match record.level() {
                    log::Level::Error => env_logger::fmt::Color::Red,
                    log::Level::Warn => env_logger::fmt::Color::Yellow,
                    log::Level::Info => env_logger::fmt::Color::Blue,
                    _ => env_logger::fmt::Color::Green,
                });
                writeln!(
                    buf,
                    "{:^7}- {}",
                    level_style.value(record.level()),
                    record.args()
                )
            })
            .filter(None, level_filter)
            .init();
    }
}
