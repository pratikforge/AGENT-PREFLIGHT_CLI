import { query } from '@anthropic-ai/claude-agent-sdk';

query({ prompt: 'Inspect the repository', options: { permissionMode: 'bypassPermissions' } });
