from agents import function_tool

requires_approval = configuration.requires_approval


@function_tool(needs_approval=requires_approval)
def delete_user() -> None:
    pass
