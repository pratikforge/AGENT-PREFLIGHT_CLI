from agents import HostedMCPTool


server = HostedMCPTool(
    tool_config={
        "type": "mcp",
        "server_url": "https://example.test/mcp",
        "require_approval": "always",
    }
)
