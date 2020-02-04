# SMS forwarder bot
A bot/broker that forwards SMS messages to telegram. Designed to work with the [app](https://github.com/OptimalStrategy/sms_forwarder_app).
<br>The deployed bot is running at https://forwarder.whatever.team and is accessible via https://t.me/smsforwarderrobot.

## Development
1. Clone the repostiory
2. Add your telegram token to configs/development.env
3. Run docker-compose:
```bash
# docker-compose -f docker-compose-dev.yml up
```
The container will be exposed at localhost:6336.

## Production
1. Clone the repository
2. Fill in the missing ENV variables in configs/production.env
3. Set up an HTTPS proxy or put your certificate file in the project directory (Webhooks require HTTPS to work)
4. Run docker-compose in production mode:
```bash
# docker-compose up
```
Th econtainer will be exposed at localhost:6336.

## TODO
- copy-paste deployment
