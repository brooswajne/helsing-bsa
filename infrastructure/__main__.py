from pathlib import Path

import pulumi

from src.routes import create_routes_from_rust_project
from src.api import create_serverless_api

API_PROJECT_ROOT = Path(__file__).parent.joinpath("../api/").resolve()

routes = create_routes_from_rust_project(API_PROJECT_ROOT)
api = create_serverless_api("API", list(routes))

pulumi.export("API Gateway URL", api.url)
