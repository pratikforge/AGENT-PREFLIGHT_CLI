from agents import function_tool


@function_tool(needs_approval=True)
def delete_user() -> None:
    pass
