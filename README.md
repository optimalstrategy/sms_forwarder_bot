# SMS forwarder bot
A bot/broker that forwards SMS messages to telegram. Intended to work with the [app](https://github.com/optimalstrategy/sms_forwarder_app).
<br>The deployed bot is running at https://forwarder.whatever.team and is accessible via https://t.me/smsforwarderrobot.

## Development
1. Copy `.env.example` and add your telegram token. If you don't want to set up a webhook with something like ngrok, set `FWD_LONG_POLLING` to `true`.
2. Spin up a postgres container with the command below.

```bash
$ docker-compose -f docker-compose-dev.yml up -d
```

3. Run the app with with `just dev` or `cargo run` (you have to manually set the env variables).

## Production
1. Copy `.env.example` and add your telegram token and database password. Set `DB_HOST=db`. Set `FWD_USE_LONG_POLLING=false` to use webhooks. Note that you _can_ use long polling in production if you want, but it's not recommended.
2. If you care about rate limiting, set `FWD_IP_SOURCE` to an appropriate value for your hosting service instead of the default `ConnectInfo`. For example, if you use an exposed nginx, you can set it to `X-Real-IP` after binding `X-Real-IP` to `$remote_addr` in the nginx config.
3. Run docker-compose to build and start the app:

```bash
$ docker-compose -f docker-compose.yml up -d
```

The container will be exposed at `127.0.0.1:6336` by default.


## Python Implementation
Archived in the [`python`](https://github.com/optimalstrategy/sms_forwarder_bot/tree/python) branch. Migrating an existing deployment to rust should be seamless, just correctly convert `configs/production.env` to `.env`.
