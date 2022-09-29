# weather-rs
## Description
OpenWeatherMap script for polybar

## Install
```
$ git clone https://github.com/g1ver/weather-rs.git
$ cargo build --release --target-dir ~/.config/polybar/scripts/weather/
```

## Usage
`Usage: ./weather-rs config_path`
## config.toml
Grab an API key from [OpenWeatherMap](https://openweathermap.org/).

Find desired city and grab the id from URL.
```
city = ''
api_key = ''
units = 'Metric | Imperial'
unit_key = 'C | F'
lang = 'en'
```
## Polybar Config
```
[module/weather]
type = custom/script
interval = 20
format = <label>
format-prefix = " "
exec = ~/.config/polybar/scripts/weather/release/weather-rs ~/.config/polybar/scripts/weather/config.toml
tail = true
```
