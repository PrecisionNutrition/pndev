// taken from https://github.com/rust-cli/clap-log-flag/blob/master/src/lib.rs
// adapted to work with modern crates
use failure::Error;
use log::Level;

use structopt::StructOpt;

/// Add log functionality to Structopt.
#[derive(StructOpt, Debug)]
pub struct Log {
    /// Enable pretty printing
    #[structopt(short = "P", long = "pretty")]
    pretty: bool,
}

use env_logger::Builder as LoggerBuilder;

fn init_builder(_pretty: bool) -> LoggerBuilder {
    LoggerBuilder::new()
}

impl Log {
    /// Initialize `env_logger` and set the log level for all packages. No
    /// additional filtering is applied.
    pub fn log_all(&self, level: Option<Level>) -> Result<(), Error> {
        let level_filter = match level {
            Some(level) => level.to_level_filter(),
            None => return Ok(()),
        };

        init_builder(self.pretty)
            .filter(None, level_filter)
            .try_init()?;
        Ok(())
    }
}
