from agents import Agent


specialist = Agent(name="Specialist")
manager = Agent(
    name="Manager",
    tools=[
        specialist.as_tool(
            tool_name="review_request",
            tool_description="Review the request.",
            needs_approval=True,
        )
    ],
)
