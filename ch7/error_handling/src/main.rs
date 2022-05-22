fn main() {
    fn pirate_share(total: u64, crew_size: usize) -> u64 {
        let half = total / 2;
        half / crew_size as u64
    }

    // Rust doesn't have exceptions. Instead, functions that can fail have a
    // return type that says so. For example:
    fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error> {}

    // The most thorough way of dealing with a Result is to use a
    // match expression
    match get_weather(hometowm) {
        Ok(report) => {
            display_weather(hometown, &report);
        }
        Err(err) => {
            println!("error querying the weather: {}", err);
            schedule_weather_retry();
        }
    }
}
