from typing import Optional

from django.core.exceptions import ObjectDoesNotExist
from django.db import models


class TgUser(models.Model):
    code = models.CharField(max_length=8)
    username = models.CharField(max_length=50, unique=True)
    telegram_id = models.IntegerField(unique=True, primary_key=True)

    @classmethod
    def by_username(cls, username: str) -> Optional["TgUser"]:
        """
        Returns user with given username or None.

        :param username: username
        :return: user instance or None
        """
        try:
            return TgUser.objects.get(username=username)
        except ObjectDoesNotExist:
            return None

    @classmethod
    def create(cls, telegram_id: int, code: str, username: str):
        """
        Creates new TgUser.

        :param telegram_id: chat id
        :param code: client code
        :param username: user's @handle
        :return: TgUser instance
        """
        t = TgUser(code=code, username=username, telegram_id=telegram_id)
        t.save()
        return t
