use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;


#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    name: String,
    cod: i32
}

#[derive(Serialize, Deserialize, Debug)]

struct Coord{
    lon: f64,
    lat: f64
}
#[derive(Serialize, Deserialize, Debug)]

struct Weather{
    details: Details
}

#[derive(Serialize, Deserialize, Debug)]

struct Details{
    id: i32,
    main: String,
    description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Debug)]

struct Temps {
   temp: f64,
   feels_like: f64,
   temp_min: f64,
   temp_max: f64,
    pressure: i32,
    humidity: i32,
    // sea_level: i32,
    // grnd_level: i32 
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
    gust: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds{
    all: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys{
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32


}

impl Forecast{
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure>{
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid=e60d17917503a572e0d4e1819f889c49&units=imperial
        ",city, country_code);

       
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;
    println!("The current weather in {}, {}, is {} :
    Temperature: {} F,
    High: {} F,
    Low: {} F,
    Feels like: {} F,
    Humidity: {}%", args.city, args.country_code, response.weather.details.description, response.main.temp, response.main.temp_max, response.main.temp_min, response.main.feels_like, response.main.humidity);
    Ok(())

}
