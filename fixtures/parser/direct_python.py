from agents import function_tool


@function_tool(needs_approval=True)
def send_email() -> None:
    pass
