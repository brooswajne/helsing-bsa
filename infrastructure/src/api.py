from typing import Sequence

from pulumi_aws_apigateway import RestAPI, RouteArgs

from src.naming import namespace

def create_serverless_api(name: str, routes: Sequence[RouteArgs]) -> RestAPI:
    return RestAPI(
        resource_name = namespace(name),
        routes = routes,
    )
