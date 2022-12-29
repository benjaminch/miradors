use mailgun_rs::{EmailAddress, Mailgun, Message};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Deserialize, Debug)]
struct Config {
    websites_to_check: String,
    check_interval_in_seconds: u64,
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
    // try to load config from file
    if let Ok(config_file_path) = env::var("MIRADORS_CONFIG_FILE") {
        let file = File::open(config_file_path).map_err(Box::new)?;
        let reader = BufReader::new(file);

        // config has been loaded from file, ignoring env vars
        let config: Config = serde_json::from_reader(reader)?;

        return Ok(config);
    }

    // config file was not set, try to load configuration from env
    Ok(Config {
        websites_to_check: env::var("MIRADORS_WEBSITES_TO_CHECK")?,
        check_interval_in_seconds: env::var("MIRADORS_CHECK_INTERVAL_IN_SECONDS")?.parse::<u64>()?,
        email_service_config: EmailServiceConfig {
            sender_email: env::var("MIRADORS_EMAIL_SERVICE_SENDER_EMAIL")?,
            sender_displayed_name: env::var("MIRADORS_EMAIL_SERVICE_SENDER_DISPLAYED_NAME")?,
            domain: env::var("MIRADORS_EMAIL_SERVICE_DOMAIN")?,
            api_key: env::var("MIRADORS_EMAIL_SERVICE_API_KEY")?,
            recipient_email: env::var("MIRADORS_EMAIL_SERVICE_RECIPIENT_EMAIL")?,
        },
    })
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
        api_key: email_service_config.api_key,
        domain: email_service_config.domain,
        message,
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

fn check_websites(websites_to_check: Vec<String>) -> Result<(), Box<dyn Error>> {
    // config is pulled everytime so websites to check can be changed without restarting the app
    let config = get_config()?;
    let http_client = reqwest::blocking::Client::new();
    let mut errored_websites: HashMap<String, String> = HashMap::new();

    for website in websites_to_check.iter() {
        let http_request = http_client.get(website);
        let start = Instant::now();
        let http_response = http_request.send();
        let duration = start.elapsed();

        info!("{}: [TIME] {:?}", website, duration);

        match http_response {
            Ok(_response) => {
                info!("{}: [OK]", website);
            }
            Err(err) => {
                error!("{}: [ERROR] '{}'", website, err);
                errored_websites.insert(website.to_string(), err.to_string());
            }
        }
    }

    if !errored_websites.is_empty() {
        return report_issue(config.email_service_config, &errored_websites);
    }

    Ok(())
}

fn main() {
    // init logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    loop {
        // load config everytime to be sure to have fresh config in case of changes
        // allowing not to restart app in case of config changes
        let config = match get_config() {
            Ok(c) => c,
            Err(e) => {
                error!("Error getting configuration: {}", e);
                return;
            }
        };

        match check_websites(
            config
                .websites_to_check
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        ) {
            Ok(_) => info!("All good!"),
            Err(err) => error!("Error: {}", err),
        };

        thread::sleep(Duration::from_secs(config.check_interval_in_seconds));
    }
}
