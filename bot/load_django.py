import os
import sys

from django import setup
from django.apps.registry import apps


def load_django(
    app_settings: str = "forwarder.settings", project_directory: str = None
):
    """
    A function that loads django apps without starting the server.

    :param app_settings: project settings file
    :param project_directory: path to project directory
    """
    if project_directory is None:
        project_directory = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    if not apps.ready:
        if app_settings is None:
            raise ValueError(
                "Application settings must be provided when django isn't loaded!"
            )
        sys.path.append(os.path.abspath(project_directory))
        os.environ["DJANGO_SETTINGS_MODULE"] = app_settings
        setup()
