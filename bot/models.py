from typing import Optional, Set

from django.core.exceptions import ObjectDoesNotExist
from django.db import models

MAX_CLIENTS = 4
CODE_LENGTH = 8


class TgUser(models.Model):
    codes = models.CharField(max_length=(CODE_LENGTH + len(",")) * MAX_CLIENTS)
    username = models.CharField(max_length=50, unique=True)
    telegram_id = models.IntegerField(unique=True, primary_key=True)

    def add_code(self, code: str) -> None:
        """
        Adds the given client code to the set of codes that may be used to message this user.

        :param code: a client code
        """
        codes = self.client_codes
        if len(codes) == MAX_CLIENTS:
            raise ValueError(f"Too many clients ({MAX_CLIENTS} max)")
        codes.add(code)
        self.codes = ",".join(codes)

    @property
    def client_codes(self) -> Set[str]:
        """
        Returns the set of the codes belonging to the clients that may message this user.

        :return: set of client codes
        """
        return set(filter(None, self.codes.split(",")))

    @classmethod
    def by_username(cls, username: str) -> Optional["TgUser"]:
        """
        Returns the user with the given username or None.

        :param username: username
        :return: user instance or None
        """
        try:
            return TgUser.objects.get(username=username.lower())
        except ObjectDoesNotExist:
            return None

    @classmethod
    def create(cls, telegram_id: int, code: str, username: str) -> "TgUser":
        """
        Creates a new TgUser.

        :param telegram_id: chat id
        :param code: client code
        :param username: user's @handle
        :return: TgUser instance
        """
        t = TgUser(codes=code, username=username.lower(), telegram_id=telegram_id)
        t.save()
        return t
