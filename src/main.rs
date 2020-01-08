mod error;
mod generator;
mod helper;
mod loader;
mod rules;

use std::env;
use std::path::Path;

use generator::Generator;

use log::{debug, info};

use env_logger::Env;
use failure::{Error, Fail};

type Result<T> = std::result::Result<T, Error>;

fn hag_exit(err: failure::Error) -> ! {
    for cause in Fail::iter_chain(err.as_fail()) {
        println!("{}: {}", cause.name().unwrap_or("Error"), cause);
    }

    std::process::exit(1)
}

fn hag_run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    debug!("{:?}", args);

    let root = Path::new(args.get(1).expect("Missing directory"));
    info!("Root path: {:?}", root);

    let mut generator = Generator::new(root);
    Ok(generator.run()?)
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    match hag_run() {
        Err(e) => hag_exit(e),
        Ok(()) => {}
    }
}
