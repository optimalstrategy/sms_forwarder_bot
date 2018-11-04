#!/bin/sh
cd /sms_forwarder_bot
python manage.py migrate
if [ "$DEBUG" = "False" ]; then
    python bot/setwebhook.py
fi
exec "$@"