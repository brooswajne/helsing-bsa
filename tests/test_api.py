"""
Some example end-to-end tests for our basic API.
"""

from contextlib import contextmanager
from os import getenv
from pathlib import Path
from typing import Generator
from urllib import request, parse

from pulumi.automation import (
    CommandError,
    ConfigValue,
    LocalWorkspace,
    Stack,
)
import pytest


TEST_STACK_NAME = "Test"

PULUMI_PROJECT_ROOT = Path(__file__).parent.joinpath("../infrastructure/").resolve()


@contextmanager
def create_stack(project_root: Path, stack_name: str) -> Generator[Stack, None, None]:
    """
    Creates a pulumi stack for the given project, which will automatically be cleaned
    up when the context is exited.
    """

    workspace = LocalWorkspace(project_root.as_posix())
    stack = Stack.create(stack_name, workspace)

    try:
        yield stack
    finally:
        try:
            # Destroying the stack will fail if there's a deployment currently
            # in progress, so abort it (the test has ended anyways):
            stack.cancel()
        except CommandError:
            # This will be raised if there was nothing to cancel, which is
            # usually the case, so we don't want to bubble up the error.
            #
            # If a deployment **was** indeed in progress, and we failed to
            # cancel it, then the `stack.destroy()` call will fail anyways
            # so there's nothing more we need to do here.
            pass
        stack.destroy()
        workspace.remove_stack(stack.name)



@pytest.fixture(scope = "module")
def api_url() -> Generator[str, None, None]:
    with create_stack(PULUMI_PROJECT_ROOT, TEST_STACK_NAME) as stack:
        aws_region = getenv("AWS_REGION")
        if aws_region is None:
            raise Exception("An AWS region to deploy test stacks to must be specified"
                " using the AWS_REGION environment variable")
        stack.set_config("aws:region", ConfigValue(aws_region))
        stack.up()
        
        outputs = stack.outputs()
        api_url = outputs["API Gateway URL"].value
        yield api_url


def test_root_route(api_url: str):
    with request.urlopen(f"{api_url}") as response:
        assert response.status == 200
        response = response.read().decode("utf-8")
        assert response.startswith("API version:")


def test_hello_world_route(api_url: str):
    with request.urlopen(f"{api_url}/hello") as response:
        assert response.status == 200
        assert response.read().decode("utf-8") == "Hello, world!"


def test_hello_user_route(api_url: str):
    with request.urlopen(f"{api_url}/hello/person") as response:
        assert response.status == 200
        assert response.read().decode("utf-8") == "Hello, person!"

    person = "person whose name will have to be encoded! (due to reserved characters)"
    with request.urlopen(f"{api_url}/hello/{parse.quote(person)}") as response:
        assert response.status == 200
        assert response.read().decode("utf-8") == f"Hello, {person}!"
