extern crate updater;
extern crate version;

use std::env;

// LOGGER  //////////////////////////////////////////
extern crate log;

use log::{Record, Level, Metadata};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

use log::{SetLoggerError, LevelFilter};

static LOGGER: SimpleLogger = SimpleLogger;

fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}

// LOGGER  //////////////////////////////////////////

fn main() {
  init();
  env::set_var("OUTPUT_DEBUG_ENABLED","true"); // to show debugging text

  match updater::has_updates(&version::version::Version::new(0,1,0),"https://github.com/snsvrno/lpsettings-rs") {
    Ok(found) => println!("update found: {}",found),
    Err(error) => println!("error: {}",error)
  }
}