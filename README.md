# SMS forwarder bot
A bot + broker that forwards SMS messages to telegram.

## Development
1. Clone repostiory
2. Add your telegram token to configs/development.env
3. Run docker-compose:
```
# docker-compose -f docker-compose-dev.yml up
```
Container will be exposeda at localhost:6336.

## Production
1. Clone repository
2. Fill missing ENV variables in configs/production.env
3. Set up HTTPS proxy or put your certificate file in the project directory (Webhooks require HTTPS to work)
4. Run docker-compose in productions mode:
```bash
# docker-compose up
```
Container will be exposed at localhost:6336.
