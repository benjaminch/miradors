use clokwerk::{Scheduler, TimeUnits};
use mailgun_rs::{EmailAddress, Mailgun, Message};
use serde::Deserialize;
use serde_with::with_prefix;
use std::collections::HashMap;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

with_prefix!(email_service "email_service_");

#[derive(Deserialize, Debug)]
struct Config {
    websites_to_check: String,
    #[serde(flatten, with = "email_service")]
    email_service_config: EmailServiceConfig,
}

#[derive(Deserialize, Debug)]
struct EmailServiceConfig {
    sender_email: String,
    sender_displayed_name: String,
    domain: String,
    api_key: String,
    recipient_email: String,
}

fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_result = envy::prefixed("MIRADORS_").from_env::<Config>();
    if let Err(err) = config_result {
        return Err(Box::new(err));
    }

    Ok(config_result.unwrap())
}

fn report_issue(
    email_service_config: EmailServiceConfig,
    errored_websites: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    // Send an email
    let message = Message {
        to: vec![EmailAddress::address(&email_service_config.recipient_email)],
        subject: String::from("Miradors: Some monitored websites are not reachable!"),
        html: errored_websites
			.keys()
			.map(|s| &**s)
			.collect::<Vec<_>>()
			.join("<br/>"),
        ..Default::default()
    };

    let client = Mailgun {
        api_key: String::from(email_service_config.api_key),
        domain: String::from(email_service_config.domain),
        message: message,
    };

    let sender = EmailAddress::name_address(
		email_service_config.sender_displayed_name.as_str(),
		email_service_config.sender_email.as_str(),
	);

    match client.send(&sender) {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}

fn check_websites() -> Result<(), Box<dyn Error>> {
    // config is pulled everytime so websites to check can be changed without restarting the app
    let config = get_config()?;

    let http_client = reqwest::blocking::Client::new();
    let websites_to_check: Vec<String> = config.websites_to_check
		.split(' ')
		.map(|s| s.to_string())
		.collect();
    let mut errored_websites: HashMap<String, String> = HashMap::new();

    for website in websites_to_check.iter() {
        let http_request = http_client.get(website);
        let start = Instant::now();
        let http_response = http_request.send();
        let duration = start.elapsed();

        println!("{}: [TIME] {:?}", website, duration);

        match http_response {
            Ok(_response) => {
                println!("{}: [OK]", website);
            }
            Err(err) => {
                eprintln!("{}: [ERROR] '{}'", website, err);
                errored_websites.insert(website.to_string(), err.to_string());
            }
        }
    }

    if errored_websites.len() > 0 {
        return report_issue(config.email_service_config, &errored_websites);
    }

    Ok(())
}

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.every(10.minutes()).run(|| {
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
