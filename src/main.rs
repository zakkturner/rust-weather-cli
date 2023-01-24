use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;
use std::io;


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
    sea_level: Option<i32>,
    grnd_level: Option<i32> 
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
    gust: Option<f64>
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

// News
#[derive(Serialize, Deserialize, Debug)]
struct News{
    status: String,
    total_results: Option<i32>,
    articles: Option<Article>
}
#[derive(Serialize, Deserialize, Debug)]
struct Article{
    source: Source,
    author: String,
    title: String,
    description: String,
    url: String,
    url_to_image: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Source{
    id: i32,
    name: String
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


impl News {
    async fn get(city: &String, country_code: &String) -> Result<Vec<Self>, ExitFailure>{
        println!("https://newsapi.org/v2/everything?q='{} {} news'&apiKey=4118cd277f604aecbc6fb2db52329874
        ", city , country_code);
        let url = format!("https://newsapi.org/v2/everything?q='{} {} news'&apiKey=4118cd277f604aecbc6fb2db52329874
        ", city , country_code);
       println!("{}", url);
       let url = Url::parse(&*url)?;
       
        let resp = reqwest::get(url).await?;

        let articles: Vec<News> = resp.json().await?;

        println!("{:?}", articles);
        Ok(articles)
    //     if resp.status() != reqwest::StatusCode::OK {
    //         eprintln!("didn't get OK status: {}", resp.status());
    //    } else {
    //         let foo_bar = resp.json().await?;
    //         println!("{}", foo_bar.a_value);
    //    }
        // Ok(resp)
        // Err(e) => eprintln!("{}", e);
    }
}
#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;
    let mut news_input = String::new();
    println!("The current weather in {}, {}, is {} :
    Temperature: {} F,
    High: {} F,
    Low: {} F,
    Feels like: {} F,
    Humidity: {}%", args.city, args.country_code, response.weather.details.description, response.main.temp, response.main.temp_max, response.main.temp_min, response.main.feels_like, response.main.humidity);
    println!("Would you like to get local news for {}, {}?", &args.city, &args.country_code);
    io::stdin().read_line(&mut news_input)?;
    if  news_input.trim() == "y"{
        let news_resp = News::get(&args.city, &args.country_code).await?;
        println!("Here are your news articles {:?}", news_resp);

    }else{
        print!("Thank you")
    }

    Ok(())

}
