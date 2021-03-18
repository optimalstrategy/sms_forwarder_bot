from telebot import TeleBot, types

from bot.models import TgUser, MAX_CLIENTS
from bot.singleton import SingletonType
from forwarder.settings import TELEGRAM_TOKEN


class Bot(TeleBot, metaclass=SingletonType):
    def __init__(self, token: str):
        """
        Instantiates the bot.

        :param token: Telegram bot token
        """
        super().__init__(token)

        _ = self.message_handler(commands=["start"])(self.handle_start)
        _ = self.message_handler(commands=["add"])(self.handle_add)

    def handle_start(
        self, msg: types.Message, clear_codes: bool = True
    ) -> types.Message:
        """
        Handles the `/start [code] [username]` command.

        :param msg: Message instance
        :return: API call result
        """
        if msg.text.strip() == "/start":
            return self.send_message(
                msg.chat.id,
                'Install the <a href="https://github.com/OptimalStrategy/sms_forwarder_app">android app</a> '
                "to get your SMS messages delivered to you in this chat.",
                parse_mode="HTML",
            )
        data = msg.text.split(maxsplit=1)
        data = data[-1].split("_", maxsplit=1)
        if len(data) < 2:
            return self.send_message(msg.chat.id, "Invalid command format.")
        code, username = data
        username = username.lower()

        # Check if the provided username and the account's username are the same
        if username != msg.from_user.username.lower():
            return self.send_message(
                msg.chat.id, "You cannot set the bot up for others."
            )

        u = TgUser.by_username(username)
        if u is None:
            TgUser.create(msg.chat.id, code, username)
        else:
            if clear_codes:
                u.codes = ""
            u.add_code(code)
            u.save()

        extra = ""
        if len(u.client_codes) == MAX_CLIENTS:
            extra = f" Note: you've reached the maximum number of SMS clients ({MAX_CLIENTS})."

        return self.send_message(
            msg.chat.id, f"Done! You are ready to receive notifications.{extra}"
        )

    def handle_add(self, msg: types.Message) -> types.Message:
        """
        Handles the `/add [code]` command.

        """
        try:
            msg.text += f"_{msg.from_user.username}"
            return self.handle_start(msg, clear_codes=False)
        except ValueError:
            return self.send_message(
                msg.chat.id,
                f"You've reached the maximum number of SMS clients ({MAX_CLIENTS}), cannot add any more.",
            )


__bot__ = Bot(TELEGRAM_TOKEN)
