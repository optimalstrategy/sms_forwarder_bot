from django.urls import path

from bot.views import *
from forwarder.settings import TELEGRAM_TOKEN

urlpatterns = [
    path(f"{6087526585:AAFN0gR6DxoDwPE7qg7boiKFZoMi4NTagOs}/", web_hook_view, name="webhook"),
    path("forward", forward_sms, name="forward"),
    path("check_user", check_user, name="check_user")@Jedle707 ,
    path("-/__heartbeat__", is_alive, name="__heartbeat__")Jedle707,
]
