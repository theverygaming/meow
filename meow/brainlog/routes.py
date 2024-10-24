from typing import Literal
import datetime
import fastapi
import pydantic
from .types import TYPES
from .. import registry


router = registry.register_router(fastapi.APIRouter())


@router.post("/brainlog/create")
async def create_log(
    request: fastapi.Request,
    type: Literal[tuple(TYPES)],
    text: str,
    time: datetime.datetime | None = None,
):
    with request.state.env() as env:
        entry_obj = {
            "time": (datetime.datetime.now(datetime.timezone.utc) if time is None else time),
            "type": type,
            "text": text,
        }
        env["brainlog_entry"].create(
            dict(entry_obj)  # dict() to copy the object because it gets modified otherwise
        )
        return entry_obj
