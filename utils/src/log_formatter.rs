#![cfg(feature = "log_formatter")]

use {env_logger::fmt::Formatter, log::Record, std::io::Result, std::io::Write};

pub fn log_formatter(buf: &mut Formatter, record: &Record) -> Result<()> {
    let color_code = match record.level() {
        log::Level::Error => 1, // red
        log::Level::Warn => 3,  // yellow
        log::Level::Info => 4,  // blue
        log::Level::Debug => 2, // green
        log::Level::Trace => 7, // white
    };
    let space = match record.level().as_str().len() {
        4 => " ",
        _ => "",
    };
    let level = record.level().as_str();
    let target = record.target();
    let timestamp = buf.timestamp();
    let body = record.args();

    writeln!(
        buf,
        "[\x1b[3{}m{}\x1b[0m]{} {} {} - {}",
        color_code, level, space, timestamp, target, body
    )
}
