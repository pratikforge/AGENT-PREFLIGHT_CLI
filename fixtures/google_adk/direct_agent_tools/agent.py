from google.adk.agents import Agent


def update_discount_status() -> None:
    pass


root_agent = Agent(
    name="discount_helper",
    tools=[update_discount_status],
)
