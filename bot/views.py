import json

from django.http import HttpRequest, HttpResponse
from django.views.decorators.csrf import csrf_exempt
from telebot.types import Update

from bot.tgbot import bot as bot_instance
from bot.models import TgUser


@csrf_exempt
def web_hook_view(request: HttpRequest, *args, **kwargs) -> HttpResponse:
    """
    Accepts and processes updates from Telegram.

    @endpoint /{TOKEN}
    """
    # Return 403 if request is invalid
    if request.content_type != 'application/json':
        return HttpResponse(status=403)

    # Decode json from request body and process updates
    json_string = json.loads(request.body)
    update = Update.de_json(json_string)
    bot_instance.process_new_updates([update])
    return HttpResponse(status=200)


def forward_sms(request: HttpRequest) -> HttpResponse:
    """
    Forwards SMS to a user.

    @endpoint /forward
    @param    ?code=<code>
    @param    &username=<username>
    @params   [&{address, body, date}]
    """
    # Check for required params
    if 'code' not in request.GET:
        return HttpResponse(b"The `code` param must be provided.", status=400)
    if 'username' not in request.GET:
        return HttpResponse(b"The `username` param must be provided.", status=400)
    code = request.GET.get("code")
    username = request.GET.get("username")
    address, body, date = [request.GET.get(f, "<>") for f in ('address', 'body', 'date')]

    # Get user instance and compare codes
    user = TgUser.by_username(username)
    if user is None:
        return HttpResponse(b"Such user does not exist.", status=404)
    if user.code != code:
        return HttpResponse(b"Bad code.", status=401)

    # Forward SMS message
    bot_instance.send_message(
        user.telegram_id,
        f"New SMS message from \'{address}\':\n{body}\n\nDate: {date}."
    )

    return HttpResponse(status=200)


def check_user(request: HttpRequest) -> HttpResponse:
    """
    Returns 200 if user exists.


    @endpoint /check_user
    @param    ?username=<username>
    @param    [& code]
    """
    # Check for required param
    if 'username' not in request.GET:
        return HttpResponse(b"The `username` param must be provided.", status=400)
    user = TgUser.by_username(request.GET.get("username"))
    if user is None:
        return HttpResponse(b"Such user does not exist.", status=404)
    if 'code' in request.GET and request.GET['code'] != user.code:
        return HttpResponse(b"Bad code.", status=400)
    return HttpResponse(b"OK", status=200)
