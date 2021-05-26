use clokwerk::{Scheduler, TimeUnits};
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Deserialize, Debug)]
struct Config {
    websites_to_check: String,
}

fn check_websites() -> Result<(), Box<dyn Error>> {
    // config is pulled everytime so websites to check can be changed without restarting the app
    let config_result = envy::prefixed("MIRADOR_").from_env::<Config>();
    if let Err(err) = config_result {
        return Err(Box::new(err));
    }

    let config = config_result.unwrap();
    println!("{:#?}", config);

    let http_client = reqwest::blocking::Client::new();
    let websites_to_check: Vec<&str> = config.websites_to_check.split(' ').collect();

    for website in websites_to_check {
        let http_request = http_client.get(website);

        let start = Instant::now();
        let http_response = http_request.send();
        let duration = start.elapsed();
        println!("Time elapsed to get response is: {:?}", duration);

        match http_response {
            Ok(response) => {
                let headers = response.headers();
                println!("headers: {:#?}", headers);

                // let content = http_response.json::<HashMap<String, String>>();
                // println!("content: {:#?}", content);
            }
            Err(err) => {
                eprintln!("Problem reaching website '{}', error: '{}'", website, err);
            }
        }
    }

    Ok(())
}

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.every(10.seconds()).run(|| {
        match check_websites() {
            Ok(_) => println!("All good!"),
            Err(err) => eprintln!("Error: {}", err),
        };
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
