# Weather CLI App

![Weather CLI App](https://newsonair.gov.in/writereaddata/News_Pictures/MIS/2022/Feb/NPIC-202221784443.jpg)

## Description

This is a simple CLI app that uses the [OpenWeatherMap API](https://openweathermap.org/api) and the [WeatherApi API](https://www.weatherapi.com/) to get the weather for a given location.

## How to use

1. Clone the repo
2. Run `cargo run get --location <Location-name>` in the root directory
3. Change the datasource provider by: `cargo run configure --provider <Provider-name>`

If you're confused in some command you can always run `cargo run --help` to get the list of available commands.

## Providers

Currently, the app supports two providers:
1. OpenWeather (default)
2. WeatherApi

> Note, that historical data in both providers is available only for paid accounts.

## Env variables

The app uses the following env variables:
```dotenv
# OpenWeather API key
OPEN_WEATHER_API_KEY='...'
# WeatherApi API key
WEATHER_API_API_KEY='...'
# Path to the config file
CONFIG_PATH='...'
```

## Config file

Config file is generated automatically. Only you is to provide the path where you'd like to store it.
