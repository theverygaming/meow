models = []


def register_model(cls):
    models.append(cls)
    return cls


routers = []


def register_router(router):
    routers.append(router)
    return router
