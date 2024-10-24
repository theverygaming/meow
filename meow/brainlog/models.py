# based on Arya's poc_4 script :3
from typing import Any
import datetime
import sillyorm
from .types import TYPES
from .. import registry


class DatetimeForcedUTCField(sillyorm.fields.Datetime):
    def _convert_type_set(self, value: Any) -> Any:
        if not value.tzinfo == datetime.timezone.utc:
            raise Exception("non-UTC timezone!")
        value = value.replace(tzinfo=None)
        val = super()._convert_type_set(value)
        return val

    def _convert_type_get(self, value: Any) -> Any:
        val = super()._convert_type_get(value)
        return val.replace(tzinfo=datetime.timezone.utc)


@registry.register_model
class BrainLogEntry(sillyorm.model.Model):
    _name = "brainlog_entry"

    time = DatetimeForcedUTCField()
    type = sillyorm.fields.Selection(TYPES)
    text = sillyorm.fields.String()
