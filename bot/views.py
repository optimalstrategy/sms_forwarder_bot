import json

from django.http import HttpRequest, HttpResponse, RawPostDataException
from django.views.decorators.csrf import csrf_exempt
from telebot.types import Update

from bot.tgbot import __bot__ as bot_instance
from bot.models import TgUser


@csrf_exempt
def web_hook_view(request: HttpRequest, *args, **kwargs) -> HttpResponse:
    """
    Accepts and processes updates from Telegram.

    @endpoint /{TOKEN}
    """
    # Return 403 if the request is not in JSON
    if request.content_type != "application/json":
        return HttpResponse(status=403)

    json_string = json.loads(request.body)
    update = Update.de_json(json_string)
    bot_instance.process_new_updates([update])
    return HttpResponse(status=200)


@csrf_exempt
def forward_sms(request: HttpRequest) -> HttpResponse:
    """
    Forwards an SMS message to a user.

    @endpoint /forward
    @param    ?code=<code>
    @param    &username=<username>
    @params   [&{address, body, date}]
    """
    params = dict(request.GET)
    params.update(request.POST or {})
    try:
        params.update(json.loads(request.body))
    except (json.JSONDecodeError, RawPostDataException):
        pass

    # Check for the required parameters
    if "code" not in params:
        return HttpResponse(b"The `code` param must be provided.", status=400)
    if "username" not in params:
        return HttpResponse(b"The `username` param must be provided.", status=400)
    params = {k: p[0] if isinstance(p, list) else p for k, p in params.items()}
    code = params.get("code")
    username = params.get("username")
    address, body, date = [params.get(f, "<>") for f in ("address", "body", "date")]

    user = TgUser.by_username(username)
    if user is None:
        return HttpResponse(b"Such user does not exist.", status=404)
    if user.code != code:
        return HttpResponse(b"Bad code.", status=401)

    bot_instance.send_message(
        user.telegram_id, f"New SMS message from '{address}':\n{body}\n\nDate: {date}."
    )

    return HttpResponse(status=200)


def check_user(request: HttpRequest) -> HttpResponse:
    """
    Returns 200 if the user exists.

    @endpoint /check_user
    @param    ?username=<username>
    @param    [& code]
    """
    if "username" not in request.GET:
        return HttpResponse(b"The `username` param must be provided.", status=400)
    user = TgUser.by_username(request.GET.get("username"))
    if user is None:
        return HttpResponse(b"Such user does not exist.", status=404)
    if request.GET.get("code", -1) != user.code:
        return HttpResponse(b"Bad code.", status=400)
    return HttpResponse(b"OK", status=200)


def is_alive(request: HttpResponse) -> HttpResponse:
    """
    Returns code 200 if the app is alive.

    @endpoint /-/__heartbeat__
    """
    return HttpResponse(b"Alive", status=200)
