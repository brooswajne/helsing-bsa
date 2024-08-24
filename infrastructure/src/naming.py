import pulumi

def namespace(resource_name: str) -> str:
    """
    Given a "plain" resource name, returns a namespaced version of that resource
    name which follows the following naming convention:

    `<ProjectName>-<EnvironmentName>-<ResourceName>`

    This naming convention ensures that it's immediately obvious from a resource's
    name which project / stack it belongs to.
    """

    project_name = pulumi.get_project()
    environment_name = pulumi.get_stack()

    return f"{project_name}-{environment_name}-{resource_name}"
