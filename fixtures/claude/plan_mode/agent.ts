import { query } from "@anthropic-ai/claude-agent-sdk";


void query({
  prompt: "Inspect this repository and prepare a plan.",
  options: { permissionMode: "plan" },
});
