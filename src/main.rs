use dotenv;
use clap::{Parser};
use reqwest;

const LAT: f32 = 52.636709;
const LON: f32 = -1.13557;

#[derive(Parser)]
#[command(name="weather")]
#[command(about = "A command line weather app written in Rust", long_about = None)]

struct Args {
    // number of days for the forecase
    #[arg(short, default_value_t = 0)]
    days: u8,
}


fn main() -> Result<(), reqwest::Error> {
    //dotenv at start of program
    dotenv::dotenv().unwrap(); // !!

    // let api_key: String = dotenv::var("OPEN_WEATHER_API_KEY")?;
    
    let mut api_key: Option<String> = None;
    for (key, value) in std::env::vars() {
        if key != "OPEN_WEATHER_API_KEY" {
            continue;
        }
        api_key = Some(value);
    }
    if api_key.is_none() {
        panic!("need API key");
    }
    
    let api_key: String = api_key.unwrap();
    println!("api_key = {:?}", api_key);
    let args = Args::parse();

    let url: String = format!("https://api.openweathermap.org/data/2.5/forecast?lat={LAT}&lon={LON}&appid={api_key}&units=metric");

    let body = reqwest::blocking::get(url)?
    .text()?;

    println!("body = {:?}", body);

    // println!("{}", args.days);
    Ok(())
}
