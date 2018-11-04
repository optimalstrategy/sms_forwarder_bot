#!/bin/sh
cd /sms_forwarder_bot
python manage.py migrate
python bot/setwebhook.py
exec "$@"