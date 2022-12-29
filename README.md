# Miradors
This app is just a very simple tool allowing you to monitor is your websites are up and send you an email if not.

# Run it locally
## Via ENV
```
$ MIRADORS_CHECK_INTERVAL_IN_SECONDS=30 \
MIRADORS_WEBSITES_TO_CHECK="https://google.com https://google.fr" \
MIRADORS_EMAIL_SERVICE_SENDER_EMAIL=miradors@example.sh \
MIRADORS_EMAIL_SERVICE_SENDER_DISPLAYED_NAME=Miradors \
MIRADORS_EMAIL_SERVICE_DOMAIN=example.sh \
MIRADORS_EMAIL_SERVICE_API_KEY=[MAILGUN_API_KEY] \
MIRADORS_EMAIL_SERVICE_RECIPIENT_EMAIL=your-email@one.com \
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/miradors`
2022-12-29T20:29:31.736859Z  INFO miradors: https://google.fr: [TIME] 141.880138ms
2022-12-29T20:29:31.736969Z  INFO miradors: https://google.fr: [OK]
2022-12-29T20:29:31.917753Z  INFO miradors: https://google.com: [TIME] 180.662528ms
2022-12-29T20:29:31.917803Z  INFO miradors: https://google.com: [OK]
2022-12-29T20:29:31.918911Z  INFO miradors: All good!
2022-12-29T20:30:02.044902Z  INFO miradors: https://google.fr: [TIME] 120.376224ms
2022-12-29T20:30:02.044937Z  INFO miradors: https://google.fr: [OK]
2022-12-29T20:30:02.197198Z  INFO miradors: https://google.com: [TIME] 152.082385ms
2022-12-29T20:30:02.197251Z  INFO miradors: https://google.com: [OK]
2022-12-29T20:30:02.198014Z  INFO miradors: All good!
2022-12-29T20:30:32.318437Z  INFO miradors: https://google.fr: [TIME] 116.46398ms
2022-12-29T20:30:32.318483Z  INFO miradors: https://google.fr: [OK]
2022-12-29T20:30:32.462661Z  INFO miradors: https://google.com: [TIME] 143.998689ms
2022-12-29T20:30:32.462694Z  INFO miradors: https://google.com: [OK]
2022-12-29T20:30:32.463398Z  INFO miradors: All good!
2022-12-29T20:31:02.587308Z  INFO miradors: https://google.fr: [TIME] 118.382081ms
2022-12-29T20:31:02.587339Z  INFO miradors: https://google.fr: [OK]
2022-12-29T20:31:02.749481Z  INFO miradors: https://google.com: [TIME] 162.043646ms
2022-12-29T20:31:02.749530Z  INFO miradors: https://google.com: [OK]
2022-12-29T20:31:02.750230Z  INFO miradors: All good!

```

## Via config file
```
$ cat config.json
{
     "check_interval_in_seconds": 30,
     "websites_to_check": "https://google.com https://google.fr",
     "email_service_config": {
         "sender_email": "miradors@example.sh",
         "sender_displayed_name": "Miradors",
         "domain": "example.sh",
         "api_key": "[MAILGUN_API_KEY]",
         "recipient_email": "your-email@one.com"
     }
}

$ MIRADORS_CONFIG_FILE=config.json cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/miradors`
2022-12-29T20:19:52.072370Z  INFO miradors: https://google.com: [TIME] 201.387487ms
2022-12-29T20:19:52.072466Z  INFO miradors: https://google.com: [OK]
2022-12-29T20:19:52.073327Z  INFO miradors: All good!
2022-12-29T20:20:22.307127Z  INFO miradors: https://google.com: [TIME] 228.32006ms
2022-12-29T20:20:22.307194Z  INFO miradors: https://google.com: [OK]
2022-12-29T20:20:22.308085Z  INFO miradors: All good!

```

# Using Docker image
## Via ENV
## Via config file
