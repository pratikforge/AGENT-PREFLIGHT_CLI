import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = "Summarize the repository";
void query({ prompt });
