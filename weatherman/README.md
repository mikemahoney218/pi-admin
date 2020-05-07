# Weatherman Rust utility

Package for pulling forecast data from the US Weather Service and outputting
in InfluxDB line protocol, designed to be run by Telegraf's `exec` input. 
Currently hardcoded to Boston area, change the URL in `.get` to pull forecast
for your region.
