#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

use reqwest::{blocking, Error};
use serde_json::{json, value};
use std::string::String;
use geojson::GeoJson;
use reqwest::header::USER_AGENT;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct Period {
    number: String,
    name: String,
    start_time: String,
    end_time: String,
    is_daytime: String,
    temperature: String,
    temperature_unit: String,
    temperature_trend: String,
    wind_speed: String,
    wind_direction: String,
    icon: String,
    short_forecast: String,
    detailed_forecast: String
}

fn main() -> Result<(), Error> {
    let client = blocking::Client::new();
    let res = client
        .get("https://api.weather.gov/gridpoints/BOX/70,76/forecast")
        .header(USER_AGENT, "weatherman-rs")
        .send()?;

    let request = res.text()?;

    let geo_req = request.parse::<GeoJson>().unwrap();

    let parse_req: value::Value = json!(geo_req);
    let timestamp = SystemTime::now()
                               .duration_since(UNIX_EPOCH)
                               .unwrap()
                               .as_nanos();

    for i in 0..parse_req["properties"]["periods"]
                .as_array()
                .unwrap()
                .len() {
        let period = &parse_req["properties"]["periods"][i];
        let experimental_period = Period {
            number: period["number"].to_string(),
            name: period["name"].to_string(),
            start_time: period["startTime"].to_string(),
            end_time: period["endTime"].to_string(),
            is_daytime: period["isDaytime"].to_string(),
            temperature: period["temperature"].to_string(),
            temperature_unit: period["temperatureUnit"].to_string(),
            temperature_trend: period["temperatureTrend"].to_string(),
            wind_speed: period["windSpeed"].to_string(),
            wind_direction: period["windDirection"].to_string(),
            icon: period["icon"].to_string(),
            short_forecast: period["shortForecast"].to_string(),
            detailed_forecast: period["detailedForecast"].to_string()
          };
          println!("weatherForecast,number={},isDaytime={},tempUnit={} name={},startTime={},endTime={},temperature={},tempTrend={},windSpeed={},windDirection={},icon={},shortForecast={},detailedForecast={} {}", 
                   experimental_period.number,
                   experimental_period.is_daytime,
                   experimental_period.temperature_unit,
                   experimental_period.name,
                   experimental_period.start_time,
                   experimental_period.end_time,
                   experimental_period.temperature,
                   experimental_period.temperature_trend,
                   experimental_period.wind_speed,
                   experimental_period.wind_direction,
                   experimental_period.icon,
                   experimental_period.short_forecast,
                   experimental_period.detailed_forecast,
                   timestamp
                )
      };

    Ok(())

}