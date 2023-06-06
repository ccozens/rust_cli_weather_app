use dotenv;
use clap::{Parser};
use reqwest;
use serde::Deserialize;

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
#[derive(Deserialize, Debug)]

struct Coord {
    lon: f32,
    lat: f32,
}
#[derive(Deserialize, Debug)]

struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Deserialize, Debug)]

struct CurrentWeatherMain {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    // pressure: u32,
    humidity: u32,
    // sea_level: u32,
    // grnd_level: u32,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    coord: Coord,
    weather: Weather,
    base: String,
    main: CurrentWeatherMain,
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
    
    let args = Args::parse();

let method = match args.days {
    0 => "weather",
    _ => "forecast",
};
let count = args.days * 8;



    let url: String = format!("https://api.openweathermap.org/data/2.5/forecast?lat={LAT}&lon={LON}&appid={api_key}&units=metric&cnt={count}");

    let body: CurrentWeather = reqwest::blocking::get(url)?
    .json()?;

    println!("body = {:?}", body);
    

    // println!("{}", args.days);
    Ok(())
}
