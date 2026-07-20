from claude_agent_sdk import ClaudeAgentOptions, query


async def run_agent() -> None:
    async for _message in query(
        prompt="Inspect the repository",
        options=ClaudeAgentOptions(
            permission_mode="dontAsk",
            allowed_tools=["Read"],
        ),
    ):
        pass
