use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    city: String,
    api_key: String,
    units: String,
    unit_key: String,
    lang: String,
}

#[derive(Deserialize, Debug)]
struct Message {
    weather: Vec<Weather>,
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String, 
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64, 
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./weather-rs config_path");
        std::process::exit(1);
    }

    let config_path = &args[1];

    let config_string = std::fs::read_to_string(config_path)
        .expect("Issue getting TOML config");

    let config: Config = toml::from_str(&config_string)
        .expect("Invalid TOML config");

    let url = "http://api.openweathermap.org/data/2.5/weather";

    let response = ureq::get(&url)
        .query("id", &config.city)
        .query("lang", &config.lang)
        .query("appid", &config.api_key)
        .query("units", &config.units)
        .call();

    match response {
        Ok(res) => {
            let m: Message = res.into_json().unwrap();
            println!("{}, {} °{}", 
                     capitalize(&m.weather[0].description), 
                     *&m.main.temp as usize, 
                     &config.unit_key);
        },
        Err(ureq::Error::Status(code, res)) => { 
            println!("HTTP Error: {}, {}", code, res.status_text());
        }
        Err(e) => {
            println!("IO/Transport Error: {}", e);
        }
    };
}

fn capitalize(s: &String) -> String {
    let mut c: Vec<char> = s.chars().collect();
    c[0] = c[0].to_ascii_uppercase();
    c.iter().cloned().collect::<String>()
}
