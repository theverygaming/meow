import logging
import threading
import contextlib
import sqlite3
import sillyorm
import fastapi
from . import registry


def run_app():
    logging.basicConfig(
        format="%(asctime)s %(levelname)s %(name)s: %(message)s", level=logging.INFO
    )

    env = sillyorm.Environment(
        sillyorm.dbms.sqlite.SQLiteConnection("test.db", check_same_thread=False).cursor()
    )
    env_lock = threading.Lock()

    for model in registry.models:
        env.register_model(model)

    env.init_tables()

    app = fastapi.FastAPI()

    @contextlib.contextmanager
    def env_contextmanager():
        env_lock.acquire()
        try:
            yield env
        finally:
            env_lock.release()

    @app.middleware("http")
    async def add_process_time_header(request: fastapi.Request, call_next):
        request.state.env = env_contextmanager
        response = await call_next(request)
        return response

    for router in registry.routers:
        app.include_router(router)

    return app
