from google.adk.tools.function_tool import FunctionTool

requires_confirmation = settings.requires_confirmation
tool = FunctionTool(delete_user, require_confirmation=requires_confirmation)
