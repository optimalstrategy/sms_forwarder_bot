from django.apps import apps
from django.db import migrations, models
from django.db.models import F


def copy_field(apps, schema_editor):
    db_alias = schema_editor.connection.alias
    TgUser = apps.get_model("bot", "TgUser")
    TgUser.objects.using(db_alias).all().update(codes=F("code"))


class Migration(migrations.Migration):
    dependencies = [
        ("bot", "0001_initial"),
    ]

    operations = [
        migrations.RunPython(copy_field),
    ]


class Migration(migrations.Migration):
    dependencies = [
        ("bot", "0001_initial"),
    ]

    operations = [
        migrations.AddField(
            model_name="tguser",
            name="codes",
            field=models.CharField(default=None, max_length=36),
            preserve_default=False,
        ),
        migrations.RunPython(copy_field),
        migrations.RemoveField(
            model_name="tguser",
            name="code",
        ),
    ]
