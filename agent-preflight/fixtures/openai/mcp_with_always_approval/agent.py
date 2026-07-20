from agents.mcp import MCPServerStdio


server = MCPServerStdio(
    name="Restricted server",
    params={"command": "example"},
    require_approval="always",
)
