use structopt::StructOpt;

use std::time::{Instant};


#[macro_use]
extern crate log;
extern crate env_logger;

extern crate reqwest;


#[derive(StructOpt)]
struct Cli {
	domain: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Cli::from_args();

    // start timer
    // info!("Starting timer")
    let start = Instant::now();

    // make request
    info!("Making request for {}", args.domain);
    let res = reqwest::get(&args.domain)?;

    // calculate duration
    // info!("Finishing Timer")
    let duration = start.elapsed();

    println!("url={} status='{}' duration={}ms", args.domain, res.status(), duration.as_millis());

    Ok(())
}
