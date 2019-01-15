from django.urls import path

from bot.views import *
from forwarder.settings import TELEGRAM_TOKEN

urlpatterns = [
    path(f"{TELEGRAM_TOKEN}", web_hook_view, name="webhook"),
    path("forward", forward_sms, name="forward"),
    path("check_user", check_user, name="check_user"),
    path("-/__heartbeat__", is_alive, name="__heartbeat__"),
]
