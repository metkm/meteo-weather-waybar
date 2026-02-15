use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeather {
    temperature: f32,
    is_day: u8
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    current_weather: CurrentWeather,
}

#[derive(Serialize, Deserialize)]
struct Output {
    alt: String,
    text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = match std::env::args().nth(1) {
        Some(url) => url,
        None => {
            "No CLI URL provided".into()
        }
    };

    let res = reqwest::blocking::get(url)?
        .json::<Response>()?;

    let alt = match res.current_weather.is_day {
        0 => "night",
        1 => "day",
        _ => ""
    };

    let out = Output {
        text: (res.current_weather.temperature as u8).to_string(),
        alt: alt.to_string(),
    };

    print!("{}", serde_json::to_string(&out)?);

    Ok(())
}
