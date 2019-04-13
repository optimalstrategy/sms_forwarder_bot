"""
A script to set bot's webhook up.
See PyTelegramBotApi docs on webhooks and
https://github.com/eternnoir/pyTelegramBotAPI/blob/master/examples/webhook_examples/webhook_flask_echo_bot.py
"""
import sys

sys.path.extend(["..", "."])

from bot.load_django import load_django

load_django()

from bot.tgbot import __bot__
from forwarder.settings import HAS_WEBHOOK_CERT, WEBHOOK_URL, WEBHOOK_CERT_PATH, DEBUG


if __name__ == "__main__":
    if not DEBUG:
        print("Removing old webhook...")
        __bot__.remove_webhook()
        cert = open(WEBHOOK_CERT_PATH) if HAS_WEBHOOK_CERT else None
        print("Setting new webhook...")
        __bot__.set_webhook(WEBHOOK_URL, certificate=cert)
        print("Set new webhook.")
    else:
        print("Skipping webhook in development mode")
