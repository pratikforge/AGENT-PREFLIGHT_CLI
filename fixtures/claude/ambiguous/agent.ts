import { query } from '@anthropic-ai/claude-agent-sdk';

const mode = getPermissionMode();
query({ permissionMode: mode });
