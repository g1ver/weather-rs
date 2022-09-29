# weather-rs
## Description
OpenWeatherMap script for polybar

## Install
```
$ git clone https://github.com/g1ver/weather-rs.git
$ cargo build --release 
$ mv release/weather-rs ~/.config/polybar/scripts/weather/
$ mv example/config.toml ~/.config/polybar/scripts/weather/
```

## Usage
`Usage: ./weather-rs config_path`
## Example config.toml
Grab an API key from [OpenWeatherMap](https://openweathermap.org/).

Find desired city and grab the id from URL.
```toml
city = '5368361'
api_key = 'd2VhdGhlci1ycyBpcyBjb29s'
units = 'Imperial'
unit_key = 'F'
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
