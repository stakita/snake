// More details here: https://github.com/estk/log4rs/blob/master/examples/log_to_file.rs

use log::{debug, error, info, trace, warn, SetLoggerError};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
};

fn main() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Trace;
    let file_path = "./foo.log";

    let logfile = FileAppender::builder().build(file_path).unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(level))
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    error!("Goes to file 1");
    warn!("Goes to file 2");
    info!("Goes to file 3");
    debug!("Goes to file 4");
    trace!("Goes to file 5");

    Ok(())
}
