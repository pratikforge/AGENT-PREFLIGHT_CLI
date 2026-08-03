from google.adk.tools.function_tool import FunctionTool


def delete_user() -> None:
    pass


tool = FunctionTool(delete_user, require_confirmation=False)
