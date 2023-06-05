use clap::{Parser};
use reqwest;


const LAT: f32 = 52.636709;
const LONG: f32 = -1.13557;

#[derive(Parser)]
#[command(name="weather")]
#[command(about = "A command line weather app written in Rust", long_about = None)]

struct Args {
    // number of days for the forecase
    #[arg(short, default_value_t = 0)]
    days: u8,
}


fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let body = reqwest::blocking::get("https://www.rust-lang.org")?
    .text()?;

    println!("body = {:?}", body);

    // println!("{}", args.days);
    Ok(())
}
