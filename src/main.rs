#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use reqwest;
use reqwest::{Response, Result};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use serde_json::{Value, Map};

#[tokio::main]
async fn main() -> Result<()> {

    // println!("{}", body);
    let resp = get_weather_results().await;
    let parsed = read_json(&resp).as_object().unwrap().to_owned();
    let hourly_parsed = parsed["hourly"].as_object().unwrap().to_owned();
    println!("{:?}", hourly_parsed);
    Ok(())
}

async fn get_weather_results() -> String {

    let endpoint = "https://api.open-meteo.com/v1/forecast".to_string();

    let url = endpoint + "?latitude=52.52&longitude=13.41&hourly=temperature_2m";

    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    return body;
}

fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}


struct WeatherData<T > {
    elevation: f64,
    hourly: HashMap<String, T>
}

struct WPoller<'a, T> {
    senders: Vec<&'a dyn Sender<'a, T>>,
}

impl<'a, T> WPoller<'a, T> {
    fn start(&self) {
        println!("Starting the WPoller");

        loop {

            thread::sleep(Duration::from_secs(3));
        }
    }
}


/// Errors
enum WPollerError {
    ErrorWhileSending,
}

trait Sender<'a, T> {
    fn send(&self, weather_data: &'a WeatherData<T>) -> bool;
}

struct EmailSender<'a> {
    email: &'a str,
}

impl <'a, T> Sender<'a, T> for EmailSender<'a> {
    fn send(&self, weather_data: &'a WeatherData<T>) -> bool {
        println!("Sending weather data to {}", self.email); 
        return false;
    }
}

struct SMSSender<'a> {
    phone: &'a str,
}

impl <'a, T> Sender<'a, T> for SMSSender<'a> {
    fn send(&self, weather_data: &'a WeatherData<T>) -> bool {
        println!("Sending weather data to phone: {}", self.phone);

        return false;
    }
}
