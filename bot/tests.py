from unittest.mock import patch, MagicMock
from django.test import TestCase, Client

from bot.models import TgUser


class TestAPIMethods(TestCase):
    def setUp(self):
        self.username = "test_user"
        self.code = "testcode"
        self.client = Client()
        self.u = TgUser.create(123_456_789, self.code, self.username)

    def test_check_user__user_exists(self):
        response = self.client.get(f"/check_user?username={self.username}")
        self.assertEqual(response.status_code, 400)

    def test_check_user__user_exists_and_code_is_valid(self):
        response = self.client.get(
            f"/check_user?username={self.username}&code={self.code}"
        )
        self.assertEqual(response.status_code, 200)

    def test_check_user__username_is_required(self):
        response = self.client.get("/check_user")
        self.assertEqual(response.status_code, 400)

    def test_check_user__bad_code(self):
        response = self.client.get(
            f"/check_user?username={self.username}&code=random_code"
        )
        self.assertEqual(response.status_code, 400)

    def test_check_user__bad_user(self):
        response = self.client.get(f"/check_user?username=random_user")
        self.assertEqual(response.status_code, 404)

    @patch("bot.views.bot_instance")
    def test_forward_sms__code_is_required(self, mock: MagicMock):
        response = self.client.get(f"/forward?username={self.username}")
        self.assertEqual(response.status_code, 400)

    @patch("bot.views.bot_instance")
    def test_forward_sms__username_is_required(self, mock: MagicMock):
        response = self.client.get(f"/forward?code={self.code}")
        self.assertEqual(response.status_code, 400)

    @patch("bot.views.bot_instance")
    def test_forward_sms__bad_user(self, mock: MagicMock):
        response = self.client.get(f"/forward?code={self.code}&username=random_user")
        self.assertEqual(response.status_code, 404)

    @patch("bot.views.bot_instance")
    def test_forward_sms__bad_code(self, mock: MagicMock):
        response = self.client.get(
            f"/forward?code=random_code&username={self.username}"
        )
        self.assertEqual(response.status_code, 401)

    @patch("bot.views.bot_instance")
    def test_forward_sms__post_requests_are_supported(self, mock: MagicMock):
        response = self.client.post(
            f"/forward?code={self.code}&username={self.username}",
            data={"address": "test123", "name": "abc"},
        )
        self.assertEqual(response.status_code, 200)
