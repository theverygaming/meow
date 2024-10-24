import fastapi
from .. import registry


router = registry.register_router(fastapi.APIRouter())


@router.get("/test")
async def create_item(request: fastapi.Request):
    with request.state.env() as env:
        env["meow"].create({})
        return {"meow": ":3", "test": str(env)}
