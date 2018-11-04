"""
A script to set bot's webhook up.
See PyTelegramBotApi docs on webhooks and
https://github.com/eternnoir/pyTelegramBotAPI/blob/master/examples/webhook_examples/webhook_flask_echo_bot.py
"""
from bot.load_django import load_django
load_django()

from bot.tgbot import bot
from forwarder.settings import HAS_WEBHOOK_CERT, WEBHOOK_URL, WEBHOOK_CERT_PATH


if __name__ == "__main__":
    bot.remove_webhook()
    cert = open(WEBHOOK_CERT_PATH) if HAS_WEBHOOK_CERT else None
    bot.set_webhook(WEBHOOK_URL, certificate=cert)
